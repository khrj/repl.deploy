package update

import (
	"errors"

	"github.com/KhushrajRathod/repl.deploy/logger"
)

func UpdateGitFromRemote() error {
	err := gitFetch.Run()

	if err != nil {
		logger.Error(sGitFetchFailedError)
		return errors.New(sGitFetchFailedError)
	}

	err = gitReset.Run()

	if err != nil {
		logger.Error(sGitResetFailedError)
		return errors.New(sGitResetFailedError)
	}

	return nil
}
