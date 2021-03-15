package server

// Strings
const sEndpoint = "/refresh"
const sPort = ":8090"
const sSignatureHeaderName = "Signature"

// Warnings
const sSignatureValidationFailedWarn = "Signature validation failed for an event, so listeners will not be called"

// Errors
const sBodyParseError = "Failed to parse body"
const sMissingSignatureError = "Missing Signature"
const sUnexpectedHTTPServerCloseError = "Builtin HTTP server exited unexpectedly"
