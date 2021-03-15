package update

import (
	"os/exec"
)

// Commands
var gitFetch = exec.Command("git", "fetch", "--all")
var gitReset = exec.Command("git", "reset", "--hard", "origin/main")

// Strings
const sGitFetchFailedError = "'git fetch --all' failed"
const sGitResetFailedError = "'git reset --hard origin/main' failed"