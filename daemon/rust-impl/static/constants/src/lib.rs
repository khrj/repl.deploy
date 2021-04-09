// Strings
pub const REPLIT_DEPLOY_JSON_PATH: &str = "./replit-deploy.json";
pub const SIGNATURE_HEADER_NAME: &str = "Signature";

// Errors
pub const PUBLIC_KEY_PARSE_ERROR: &str = 
    "Failed to parse public key. This shouldn't have happened, please open a new issue at https://github.com/KhushrajRathod/repl.deploy/issues/new";
pub const MISSING_CONFIG_FILE_ERROR: &str = "Config file doesn't exist";
pub const INVALID_JSON_ERROR: &str = "Invalid config JSON";
pub const GIT_FETCH_FAILED_ERROR: &str = "'git fetch --all' failed";
pub const GIT_RESET_FAILED_ERROR: &str = "'git reset --hard origin/main' failed";
pub const INVALID_SIGNATURE_ERROR: &str = "Invalid Signature";
pub const BAD_PAYLOAD_ERROR: &str = "Bad payload";
pub const PAYLOAD_TOO_OLD_ERROR: &str = "Payload too old";
pub const BAD_ENDPOINT_ERROR: &str = "Signed request not intended for current endpoint";

// Warnings
pub const SIGNATURE_VALIDATION_FAILED_WARN: &str =
    "Signature validation failed for an event, so listeners will not be called";

// Status
pub const STAT_REQUEST_RECEIVED: &str = "Received restart request from application, processing...";
pub const STAT_SIGNATURE_VALIDATION_FAILED: &str =
    "Signature validation failed, restart will not be triggered";
pub const STAT_SIGNATURE_VALIDATION_SUCCESS: &str =
    "Signature validation successful, restarting program";

// HTTP Statuses
pub mod http_status {
    pub const BAD_REQUEST: u16 = 400;
    pub const UNAUTHORIZED: u16 = 401;
    pub const FORBIDDEN: u16 = 403;
}