package main

// Strings
const sUsage = `Usage: repl.deploy [--standalone] <command to execute your program>

Parameters: 
    --standalone Start an HTTP server to listen for refresh events
    --help Show this help message

Examples:
    repl.deploy --standalone node index.js
    repl.deploy --standalone go run .
    repl.deploy node server.js
`

// Errors
const (
	sFailedToKillChildProcessError  = "Failed to kill child process"
	sFailedToStartChildProcessError = "Failed to start child process"
	statProgramStart                = "Program has been started."
)
