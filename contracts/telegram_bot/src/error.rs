pub use wasm_rpc::error::{Error, ErrorStruct};
pub const TRANSFER_FAILED: ErrorStruct<'static> = Error {
    code: 1,
    message: "Failed to transfer tokens from the specified contract. Please approve th transfer and try again",
};
pub const PERMISSION_DENIED: ErrorStruct<'static> = Error {
    code: 2,
    message: "Permission Denied",
};
