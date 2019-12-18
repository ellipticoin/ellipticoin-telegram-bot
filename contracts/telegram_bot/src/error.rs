pub use wasm_rpc::error::Error;
lazy_static!{ 
    pub static ref TRANSFER_FAILED: Error = (
        1,
        "Failed to transfer tokens from the specified contract. Please approve th transfer and try again".into(),
    );
    pub static ref PERMISSION_DENIED: Error = (
        2,
        "Permission Denied".into(),
    );
    pub static ref INSUFFICIENT_FUNDS: Error = (
        3,
        "Insufficient Funds".into(),
    );
}
