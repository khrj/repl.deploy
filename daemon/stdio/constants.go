package stdio

// Strings
const sRegex = `repl\.deploy({.*})(.*)`

// Errors
const sFailedToStartChildProcessError = "Failed to start child process"
const sProblemsMarshalingJSONError = "Problems marshaling JSON err"

// Status
const statProgramStart = "Program has been started."
const statRequestRecieved = "Recieved restart request from application, processing..."
const statRequestValidationFailed = "Request validation failed, restart will not be triggered"
const statRequestValidationSuccess = "Request validation successful, restarting program"