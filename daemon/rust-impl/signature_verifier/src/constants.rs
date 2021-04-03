// Errors
pub const INVALID_SIGNATURE_ERROR: &str = "Invalid Signature";
pub const BAD_PAYLOAD_ERROR: &str = "Bad payload";
pub const SIGNATURE_TOO_OLD_ERROR: &str = "Signature too old";
pub const BAD_ENDPOINT_ERROR: &str = "Signed request not intended for current endpoint";

// HTTP Statuses
pub mod http_status {
    pub const BAD_REQUEST: i32 = 400;
    pub const UNAUTHORIZED: i32 = 401;
    pub const FORBIDDEN: i32 = 403;
}
