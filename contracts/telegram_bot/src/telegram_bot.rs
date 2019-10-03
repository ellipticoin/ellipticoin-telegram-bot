use crate::error;
use ellipticoin::{call, contract_address, export, get_memory, sender, set_memory};
use wasm_rpc::error::Error;
use wasm_rpc::Value;

#[export]
mod telegram_bot {
    pub fn constructor() {}

    pub fn deposit(
        token_deployer: Vec<u8>,
        token_name: String,
        to: String,
        amount: u64,
    ) -> Result<Value, Error> {
        let token_address = [&token_deployer, token_name.as_bytes()].concat();
        let (result_code, _result_value) = call(
            token_address.clone(),
            "transfer_from",
            vec![
                Value::Bytes(sender()),
                Value::Bytes(contract_address()),
                Value::U64(amount),
            ]
            .into(),
        );

        if result_code == 0 {
            credit(token_address, to, amount);
            Ok(Value::Null)
        } else {
            Err(error::TRANSFER_FAILED)
        }
    }

    pub fn withdraw(
        token_deployer: Vec<u8>,
        token_name: String,
        from: String,
        amount: u64,
    ) -> Result<Value, Error> {
        let token_address = [&token_deployer, token_name.as_bytes()].concat();
        let balance = get_balance(token_address.clone(), from.clone());
        if balance >= amount {
            let (result_code, _result_value) = call(
                token_address.clone(),
                "transfer",
                vec![Value::Bytes(sender()), Value::U64(amount)].into(),
            );

            if result_code == 0 {
                debit(token_address, from, amount);
                Ok(Value::Null)
            } else {
                Err(error::TRANSFER_FAILED)
            }
        } else {
            Err(error::INSUFFICIENT_FUNDS)
        }
    }

    pub fn transfer(
        token_deployer: Vec<u8>,
        token_name: String,
        from: String,
        to: String,
        amount: u64,
    ) -> Result<Value, Error> {
        let token_address = [&token_deployer, token_name.as_bytes()].concat();
        let balance = get_balance(token_address.clone(), from.clone());
        if is_owner() {
            if balance >= amount {
                debit(token_address.clone(), from, amount);
                credit(token_address, to, amount);
                Ok(Value::Null)
            } else {
                Err(error::INSUFFICIENT_FUNDS)
            }
        } else {
            Err(error::PERMISSION_DENIED)
        }
    }

    fn is_owner() -> bool {
        contract_address().starts_with(&sender())
    }

    fn credit(token_address: Vec<u8>, username: String, amount: u64) {
        let balance: u64 = get_balance(token_address.clone(), username.clone());
        set_balance(token_address, username, balance + amount);
    }

    fn debit(token_address: Vec<u8>, username: String, amount: u64) {
        let balance: u64 = get_balance(token_address.clone(), username.clone());
        set_balance(token_address, username, balance - amount);
    }

    fn set_balance(token_address: Vec<u8>, username: String, amount: u64) {
        set_memory([&token_address, username.as_bytes()].concat(), amount);
    }

    fn get_balance(token_address: Vec<u8>, username: String) -> u64 {
        get_memory([&token_address, username.as_bytes()].concat())
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use ellipticoin::{set_contract_address, set_mock_call, set_sender, SYSTEM_ADDRESS};
    use ellipticoin_test_framework::{ALICE, BOB};

    #[test]
    fn test_deposit() {
        set_sender(ALICE.to_vec());
        set_mock_call(
            [
                SYSTEM_ADDRESS.to_vec(),
                "BaseToken".as_bytes().to_vec(),
            ].concat(),
            "transfer_from",
            &|_arguments| (0, Value::Null),
        );
        deposit(
            SYSTEM_ADDRESS.to_vec(),
            "BaseToken".to_string(),
            "alice".to_string(),
            100,
        )
        .unwrap();
        assert_eq!(
            get_memory::<_, u64>(
                [
                    SYSTEM_ADDRESS.to_vec(),
                    "BaseToken".as_bytes().to_vec(),
                    "alice".as_bytes().to_vec(),
                ]
                .concat()
            ),
            100
        );
    }

    #[test]
    fn test_withdraw() {
        set_sender(ALICE.to_vec());
        set_memory::<_, u64>(
            vec![
                SYSTEM_ADDRESS.to_vec(),
                "BaseToken".as_bytes().to_vec(),
                "alice".as_bytes().to_vec(),
            ]
            .concat(),
            100,
        );
        set_mock_call(
            [
                SYSTEM_ADDRESS.to_vec(),
                "BaseToken".as_bytes().to_vec(),
            ].concat(),
            "transfer",
            &|_arguments| (0, Value::Null),
        );
        withdraw(
            SYSTEM_ADDRESS.to_vec(),
            "BaseToken".to_string(),
            "alice".to_string(),
            50,
        )
        .unwrap();
        assert_eq!(
            get_memory::<_, u64>(
                [
                    SYSTEM_ADDRESS.to_vec(),
                    "BaseToken".as_bytes().to_vec(),
                    "alice".as_bytes().to_vec(),
                ]
                .concat()
            ),
            50
        );
    }

    #[test]
    fn test_withdraw_with_insufficient_funds() {
        set_sender(ALICE.to_vec());
        set_memory::<_, u64>(
            vec![
                SYSTEM_ADDRESS.to_vec(),
                "BaseToken".as_bytes().to_vec(),
                "alice".as_bytes().to_vec(),
            ]
            .concat(),
            50,
        );
        set_mock_call(
            [
                SYSTEM_ADDRESS.to_vec(),
                "BaseToken".as_bytes().to_vec(),
            ].concat(),
            "transfer",
            &|_arguments| (0, Value::Null),
        );
        assert!(withdraw(
            SYSTEM_ADDRESS.to_vec(),
            "BaseToken".to_string(),
            "alice".to_string(),
            51,
        )
        .is_err());
    }

    #[test]
    fn test_transfer() {
        set_sender(ALICE.to_vec());
        set_contract_address([ALICE.to_vec(), "BaseToken".as_bytes().to_vec()].concat());
        set_mock_call(
            [
                SYSTEM_ADDRESS.to_vec(),
                "BaseToken".as_bytes().to_vec(),
            ].concat(),
            "transfer_from",
            &|_arguments| (0, Value::Null),
        );
        deposit(
            SYSTEM_ADDRESS.to_vec(),
            "BaseToken".to_string(),
            "alice".to_string(),
            100,
        )
        .unwrap();
        transfer(
            SYSTEM_ADDRESS.to_vec(),
            "BaseToken".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            50,
        )
        .unwrap();
        assert_eq!(
            get_memory::<_, u64>(
                [
                    SYSTEM_ADDRESS.to_vec(),
                    "BaseToken".as_bytes().to_vec(),
                    "alice".as_bytes().to_vec(),
                ]
                .concat()
            ),
            50
        );
        assert_eq!(
            get_memory::<_, u64>(
                [
                    SYSTEM_ADDRESS.to_vec(),
                    "BaseToken".as_bytes().to_vec(),
                    "bob".as_bytes().to_vec(),
                ]
                .concat()
            ),
            50
        );
    }

    #[test]
    fn test_transfer_with_insufficient_funds() {
        set_sender(ALICE.to_vec());
        set_contract_address([ALICE.to_vec(), "BaseToken".as_bytes().to_vec()].concat());
        set_mock_call(
            [
                SYSTEM_ADDRESS.to_vec(),
                "BaseToken".as_bytes().to_vec(),
            ].concat(),
            "transfer_from",
            &|_arguments| (0, Value::Null),
        );
        deposit(
            SYSTEM_ADDRESS.to_vec(),
            "BaseToken".to_string(),
            "alice".to_string(),
            50,
        )
        .unwrap();
        assert!(transfer(
            SYSTEM_ADDRESS.to_vec(),
            "BaseToken".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            51,
        )
        .is_err());
    }

    #[test]
    fn test_transfer_is_deployer_only() {
        set_sender(ALICE.to_vec());
        set_contract_address([BOB.to_vec(), "BaseToken".as_bytes().to_vec()].concat());
        set_mock_call(
            [
                SYSTEM_ADDRESS.to_vec(),
                "BaseToken".as_bytes().to_vec(),
            ].concat(),
            "transfer_from",
            &|_arguments| (0, Value::Null),
        );
        deposit(
            SYSTEM_ADDRESS.to_vec(),
            "BaseToken".to_string(),
            "alice".to_string(),
            100,
        )
        .unwrap();
        assert!(transfer(
            SYSTEM_ADDRESS.to_vec(),
            "BaseToken".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            50
        )
        .is_err());
    }
}
