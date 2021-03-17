package main

import (
	"flag"
	"fmt"
	"os"
	"os/exec"

	"github.com/KhushrajRathod/repl.deploy/logger"
	"github.com/KhushrajRathod/repl.deploy/server"
	"github.com/KhushrajRathod/repl.deploy/stdio"
	"github.com/KhushrajRathod/repl.deploy/update"
)

var cmd *exec.Cmd

func main() {
	isStandalone, args := parseArgs()
	cmd = buildCmd(args, isStandalone)

	if isStandalone {
		err := cmd.Start()

		if err != nil {
			logger.FatalError(sFailedToStartChildProcessError)
		}

		server.Listen(func() error {
			return updateAndRestartProcess(true)
		})
	} else {
		stdio.HandleStdio(cmd, func() (*exec.Cmd, error) {
			err := updateAndRestartProcess(false)
			return cmd, err
		})
	}
}

func updateAndRestartProcess(isStandalone bool) error {
	err := update.UpdateGitFromRemote()

	if err != nil {
		return err
	}

	err = cmd.Process.Kill()

	if err != nil {
		logger.Error(sFailedToKillChildProcessError)
		return err
	}

	if isStandalone {
		cmd = buildCmd(flag.Args(), true)
		err := cmd.Start()
		logger.Success(statProgramStart)

		if err != nil {
			logger.Error(sFailedToKillChildProcessError)
			return err
		}
	} else {
		cmd = buildCmd(flag.Args(), false)
	}

	return nil
}

func parseArgs() (bool, []string) {
	isStandalone := flag.Bool("standalone", false, "")

	flag.Usage = func() {
		fmt.Fprintln(os.Stderr, sUsage)
	}

	flag.Parse()

	args := flag.Args()

	if len(args) == 0 {
		flag.Usage()
		os.Exit(1)
	}

	return *isStandalone, args
}

func buildCmd(cmdAndArgs []string, isStandalone bool) *exec.Cmd {
	cmd := cmdAndArgs[0]
	cmdArgs := cmdAndArgs[1:]

	cmdToExec := exec.Command(cmd, cmdArgs...)

	if isStandalone {
		cmdToExec.Stdout = os.Stdout
		cmdToExec.Stderr = os.Stderr
		cmdToExec.Stdin = os.Stdin
	}

	return cmdToExec
}
