package signature

import (
	"crypto/rsa"
	"crypto/x509"
	"encoding/json"
	"encoding/pem"
	"fmt"
	"os"
	"time"

	"github.com/KhushrajRathod/repl.deploy/logger"
	"gopkg.in/go-playground/validator.v9"
)

func parseConfig() Config {
	configData, err := os.ReadFile(sReplitDeployJsonPath)

	if err != nil {
		logger.FatalError(sMissingConfigFileError)
	}

	var config Config
	err = json.Unmarshal(configData, &config)

	if err != nil {
		logger.FatalError(sInvalidJSONError)
	}

	validate := validator.New()
	err = validate.Struct(config)

	if err != nil {
		for _, e := range err.(validator.ValidationErrors) {
			logger.Error(fmt.Sprint(e))
		}
		os.Exit(1)
	}

	return config
}

func parseKey() *rsa.PublicKey {
	block, _ := pem.Decode([]byte(replDeployPublicKey))
	rsaPublicKey, err := x509.ParsePKCS1PublicKey(block.Bytes)

	if err != nil {
		panic(sPrivateKeyParseError)
	}

	return rsaPublicKey
}

func isOlderThanFifteenSeconds(ts int) bool {
	return ts < int((time.Now().UnixNano()/1000000)-15000)
}
