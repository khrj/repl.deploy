package server

import (
	"fmt"
	"io"
	"log"
	"net/http"

	"github.com/KhushrajRathod/repl.deploy/signature"
)

func Listen(handler func() error) {
	http.HandleFunc(sEndpoint, func(w http.ResponseWriter, req *http.Request) {
		log.Println(statRequestRecieved)
		
		body, err := io.ReadAll(req.Body)

		if err != nil {
			http.Error(w, sBodyParseError, http.StatusBadRequest)
			log.Println(statRequestValidationFailed)
			return
		}

		signatureHeader := req.Header.Get(sSignatureHeaderName)

		if signatureHeader == "" {
			http.Error(w, sMissingSignatureError, http.StatusUnauthorized)
			log.Println(statRequestValidationFailed)
			return
		}

		validationError := signature.ValidateSignatureAndPayload(signatureHeader, body)

		if validationError != nil {
			http.Error(w, validationError.Err, validationError.Status)
			log.Println(sSignatureValidationFailedWarn)
			log.Println(statRequestValidationFailed)
			return
		}

		log.Println(statRequestValidationSuccess)

		err = handler()

		log.Println(statProgramStart)

		if err != nil {
			fmt.Fprintf(w, "OK")
		}
	})

	err := http.ListenAndServe(sPort, nil)

	if err != nil {
		log.Fatalln(sUnexpectedHTTPServerCloseError)
	}
}
