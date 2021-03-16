package update

import (
	"errors"
	"log"
)

func UpdateGitFromRemote() error {
	err := gitFetch.Run()

	if err != nil {
		log.Println(sGitFetchFailedError)
		return errors.New(sGitFetchFailedError)
	}

	err = gitReset.Run()

	if err != nil {
		log.Println(sGitResetFailedError)
		return errors.New(sGitResetFailedError)
	}

	return nil
}
