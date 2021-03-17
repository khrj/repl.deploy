package server

import (
	"fmt"
	"io"
	"net/http"

	"github.com/KhushrajRathod/repl.deploy/signature"
	"github.com/KhushrajRathod/repl.deploy/logger"
)

func Listen(handler func() error) {
	http.HandleFunc(sEndpoint, func(w http.ResponseWriter, req *http.Request) {
		logger.Info(statRequestRecieved)
		
		body, err := io.ReadAll(req.Body)

		if err != nil {
			http.Error(w, sBodyParseError, http.StatusBadRequest)
			logger.Warn(statRequestValidationFailed)
			return
		}

		signatureHeader := req.Header.Get(sSignatureHeaderName)

		if signatureHeader == "" {
			http.Error(w, sMissingSignatureError, http.StatusUnauthorized)
			logger.Warn(statRequestValidationFailed)
			return
		}

		validationError := signature.ValidateSignatureAndPayload(signatureHeader, body)

		if validationError != nil {
			http.Error(w, validationError.Body, validationError.Status)
			logger.Warn(sSignatureValidationFailedWarn)
			logger.Warn(statRequestValidationFailed)
			return
		}

		logger.Success(statRequestValidationSuccess)

		err = handler()

		logger.Success(statProgramStart)

		if err != nil {
			fmt.Fprintf(w, "OK")
		}
	})

	err := http.ListenAndServe(sPort, nil)

	if err != nil {
		logger.FatalError(sUnexpectedHTTPServerCloseError)
	}
}
