package update

import (
	"os/exec"

	"github.com/KhushrajRathod/repl.deploy/logger"
)

func UpdateGitFromRemote() error {
	gitFetch := exec.Command("git", "fetch", "--all")
	gitReset := exec.Command("git", "reset", "--hard", "origin/main")

	err := gitFetch.Run()

	if err != nil {
		logger.Error(sGitFetchFailedError)
		return err
	}

	err = gitReset.Run()

	if err != nil {
		logger.Error(sGitResetFailedError)
		return err
	}

	return nil
}
