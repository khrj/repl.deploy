package stdio

const (
	// Strings
	sRegex                 = `repl\.deploy({.*})(.*)`
	sSuccess               = "repl.deploy-success"
	sRespondedSuccessfully = "Responded successfully"

	// Errors
	sFailedToStartChildProcessError          = "Failed to start child process"
	sProblemsMarshalingJSONError             = "Problems marshaling JSON"
	sProblemsWritingToStdinOfSubprocessError = "Problems writing to stdin of subprocess"
	sWrittenSuccessJSON                      = "Written successful JSON response to subprocess stdin"

	// Status
	statProgramStart             = "Program has been started."
	statRequestRecieved          = "Recieved restart request from application, processing..."
	statRequestValidationFailed  = "Request validation failed, restart will not be triggered"
	statRequestValidationSuccess = "Request validation successful, restarting program"
)
