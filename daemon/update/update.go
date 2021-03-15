package update

import (
	"errors"
	"log"
)

func UpdateGitFromRemote() error {
	err := gitFetch.Run()

	if err != nil {
		log.Fatalln(sGitFetchFailedError)
		return errors.New(sGitFetchFailedError)
	}

	err = gitReset.Run()

	if err != nil {
		log.Fatalln(sGitResetFailedError)
		return errors.New(sGitResetFailedError)
	}

	return nil
}
