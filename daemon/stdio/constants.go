package stdio

// Strings
const sRegex = `repl\.deploy({.*})(.*)`
const sSuccess = "repl.deploy-success"
const sRespondedSuccessfully = "Responded successfully"

// Errors
const sFailedToStartChildProcessError = "Failed to start child process"
const sProblemsMarshalingJSONError = "Problems marshaling JSON"
const sProblemsWritingToStdinOfSubprocessError = "Problems writing to stdin of subprocess"
const sWrittenSuccessJSON = "Written successful JSON response to subprocess stdin"

// Status
const statProgramStart = "Program has been started."
const statRequestRecieved = "Recieved restart request from application, processing..."
const statRequestValidationFailed = "Request validation failed, restart will not be triggered"
const statRequestValidationSuccess = "Request validation successful, restarting program"
