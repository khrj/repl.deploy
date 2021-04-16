// Strings
pub const REPLIT_DEPLOY_JSON_PATH: &str = "./replit-deploy.json";
pub const SIGNATURE_HEADER_NAME: &str = "Signature";
pub const STDIN_REGEX: &str = r"repl\.deploy(\{.*})(.*)";
pub const STDIN_SUCCESS: &str = "repl.deploy-success";
pub const STDIN_RESPONDED_SUCCESSFULLY: &str = "Responded successfully";
pub const OK: &str = "OK";

// Errors
pub const PUBLIC_KEY_PARSE_ERROR: &str =
    "Failed to parse public key. This shouldn't have happened, please open a new issue at https://github.com/KhushrajRathod/repl.deploy/issues/new";
pub const MISSING_CONFIG_FILE_ERROR: &str = "Config file doesn't exist";
pub const INVALID_CONFIG_JSON_ERROR: &str = "Invalid config JSON";
pub const GIT_FETCH_FAILED_ERROR: &str = "'git fetch --all' failed";
pub const GIT_RESET_FAILED_ERROR: &str = "'git reset --hard origin/main' failed";
pub const INVALID_SIGNATURE_ERROR: &str = "Invalid Signature";
pub const BAD_PAYLOAD_ERROR: &str = "Bad payload";
pub const PAYLOAD_TOO_OLD_ERROR: &str = "Payload too old";
pub const BAD_ENDPOINT_ERROR: &str = "Signed request not intended for current endpoint";
pub const FAILED_TO_START_CHILD_PROCESS_ERROR: &str = "Failed to start child process";
pub const FAILED_TO_KILL_CHILD_PROCESS_ERROR: &str = "Failed to kill child process";
pub const PROBLEMS_SERIALIZING_JSON_ERROR: &str = "Problems serializing JSON";
pub const PROBLEMS_WRITING_TO_STDIN_OF_SUBPROCESS_ERROR: &str =
    "Problems writing to stdin of subprocess";
pub const UNKNOWN_ERROR_WHILE_PROCESSING_REQUEST: &str =
    "An unknown error occured while processing the request";

// Warnings
pub const SIGNATURE_VALIDATION_FAILED_WARN: &str =
    "Signature validation failed for an event, so listeners will not be called";
pub const GIT_FETCH_FAILED_STARTUP_WARN: &str =
    "Failed to fetch from GitHub on startup, make sure git is set up";

// Status
pub const STAT_PROGRAM_STARTED: &str = "Program has been started.";
pub const STAT_REQUEST_RECEIVED: &str = "Received restart request from application, processing...";
pub const STAT_SIGNATURE_VALIDATION_FAILED: &str =
    "Signature validation failed, restart will not be triggered";
pub const STAT_SIGNATURE_VALIDATION_SUCCESS: &str =
    "Signature validation successful, restarting program";
