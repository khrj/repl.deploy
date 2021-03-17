package stdio

import (
	"bufio"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"os"
	"os/exec"
	"regexp"

	"github.com/KhushrajRathod/repl.deploy/signature"
)

func HandleStdio(cmd *exec.Cmd, handler func() (*exec.Cmd, error)) {
	for {
		err := scanProcessStdoutAndValidate(cmd)

		if err != nil {
			log.Println(err)
		}

		cmd, err = handler()

		if err != nil {
			log.Println(err)
		}
	}
}

func scanProcessStdoutAndValidate(cmd *exec.Cmd) error {
	cmdReader, cmdWriter, err := setupPipes(cmd)

	if err != nil {
		return err
	}

	err = cmd.Start()

	if err != nil {
		log.Println(sFailedToStartChildProcessError)
		return err
	}

	reader := bufio.NewReader(cmdReader)
	validatedChannel := make(chan bool)

	log.Println(statProgramStart)

	go func(reader *bufio.Reader) {
		scanner := bufio.NewScanner(reader)
		for scanner.Scan() {
			text := scanner.Text()

			if text == sSuccess {
				validatedChannel <- true
				close(validatedChannel)
				return
			}

			regex := regexp.MustCompile(sRegex)
			match := regex.FindStringSubmatch(string(text))

			if len(match) >= 2 {
				log.Println(statRequestRecieved)
				payload := match[1]
				inputSignature := match[2]
				validationError := signature.ValidateSignatureAndPayload(inputSignature, []byte(payload))

				if validationError != nil {
					log.Println(statRequestValidationFailed)
					json, err := json.Marshal(validationError)

					if err != nil {
						log.Println(sProblemsMarshalingJSONError)
						continue
					}

					json = append(json, []byte("\n")...)
					_, err = cmdWriter.Write(json)

					if err != nil {
						log.Println(sProblemsWritingToStdinOfSubprocessError)
					}

					continue
				}

				validatedChannel <- true

				successMessage := signature.ValidationResult{
					Body:   "OK",
					Status: 200,
				}

				json, err := json.Marshal(successMessage)

				if err != nil {
					log.Println(sProblemsMarshalingJSONError)
					continue
				}

				json = append(json, []byte("\n")...)
				_, err = cmdWriter.Write(json)

				if err != nil {
					log.Println(sProblemsWritingToStdinOfSubprocessError)
					continue
				}

				log.Println(sWrittenSuccessJSON)
			} else {
				fmt.Println(text)
			}
		}
	}(reader)

	// Read twice, once for signature valid, once for responded successfully
	<-validatedChannel
	log.Println(statRequestValidationSuccess)
	<-validatedChannel
	log.Println(sRespondedSuccessfully)
	return nil
}

func setupPipes(cmd *exec.Cmd) (io.ReadCloser, io.WriteCloser, error) {
	cmdReader, err := cmd.StdoutPipe()

	if err != nil {
		return nil, nil, err
	}

	cmdWriter, err := cmd.StdinPipe()

	if err != nil {
		return nil, nil, err
	}

	cmd.Stderr = os.Stderr
	return cmdReader, cmdWriter, nil
}
