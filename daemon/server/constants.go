package server

const (
	// Strings
	sEndpoint            = "/refresh"
	sPort                = ":8090"
	sSignatureHeaderName = "Signature"

	// Warnings
	sSignatureValidationFailedWarn = "Signature validation failed for an event, so listeners will not be called"

	// Errors
	sBodyParseError                 = "Failed to parse body"
	sMissingSignatureError          = "Missing Signature"
	sUnexpectedHTTPServerCloseError = "Builtin HTTP server exited unexpectedly"

	// Status
	statRequestRecieved          = "Recieved restart request from application, processing..."
	statRequestValidationFailed  = "Request validation failed, restart will not be triggered"
	statRequestValidationSuccess = "Request validation successful, restarting program"
)
