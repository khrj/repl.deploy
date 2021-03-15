package main

import (
	"flag"
	"fmt"
	"github.com/KhushrajRathod/repl.deploy/server"
	"github.com/KhushrajRathod/repl.deploy/update"
	"log"
	"os"
	"os/exec"
)

const usage = `Usage: repl.deploy [--standalone] <command to execute your program>

Parameters: 
    --standalone Start an HTTP server to listen for refresh events
    --help Show this help message

Examples:
    repl.deploy --standalone node index.js
    repl.deploy --standalone go run .
    repl.deploy node server.js
`

var cmd *exec.Cmd

func main() {
	isStandalone, args := parseArgs()

	fmt.Println("is standalone?: ", isStandalone)
	fmt.Println("to exec: ", args)

	cmd = buildCmd(args)
	err := cmd.Start()

	if err != nil {
		log.Fatalln("Failed to start child process")
		os.Exit(1)
	}

	if isStandalone {
		server.Listen(func() error {
			return updateAndRestartProcess(cmd)
		})
	} else {
        // TODO
	}
}

func updateAndRestartProcess(cmd *exec.Cmd) error {
	err := update.UpdateGitFromRemote()

	if err != nil {
		return err
	}

	err = cmd.Process.Kill()

	if err != nil {
		log.Fatalln("Failed to kill child process")
		return err
	}

	cmd = buildCmd(flag.Args())
	err = cmd.Start()

	if err != nil {
		log.Fatalln("Failed to start child process")
		return err
	}

	return nil
}

func parseArgs() (bool, []string) {
	isStandalone := flag.Bool("standalone", false, "")

	flag.Usage = func() {
		fmt.Fprintln(os.Stderr, usage)
	}

	flag.Parse()

	args := flag.Args()

	if len(args) == 0 {
		flag.Usage()
		os.Exit(1)
	}

	return *isStandalone, args
}

func buildCmd(cmdAndArgs []string) *exec.Cmd {
	cmd := cmdAndArgs[1]
	cmdArgs := cmdAndArgs[1:]

	return exec.Command(cmd, cmdArgs...)
}
