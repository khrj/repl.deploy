package main

import (
	"crypto"
	"crypto/rsa"
	"crypto/sha256"
	"crypto/x509"
	"encoding/base64"
	"encoding/json"
	"encoding/pem"
	"gopkg.in/go-playground/validator.v9"
	"io"
	"log"
	"net/http"
	"os"
	"time"
)

var validate *validator.Validate

func main() {
	config := parseConfig()
	rsaPublicKey := parseKey()

	http.HandleFunc(sEndpoint, func(w http.ResponseWriter, req *http.Request) {
		body, err := io.ReadAll(req.Body)

		if err != nil {
			http.Error(w, sBodyParseError, http.StatusBadRequest)
			return
		}

		signature := req.Header.Get(sSignatureHeaderName)

		if isSignatureOK(rsaPublicKey, signature, body) {
			if isPayloadOK(w, body, config) {

			}
		} else {
			http.Error(w, sInvalidSignatureError, http.StatusForbidden)
			return
		}
	})

	err := http.ListenAndServe(sPort, nil)

	if err != nil {
		log.Fatalln(sUnexpectedHTTPServerCloseError)
		os.Exit(1)
	}
}

func parseConfig() Config {
	// Config parsing

	configData, err := os.ReadFile(sReplitDeployJsonPath)

	if err != nil {
		log.Fatalln(sMissingConfigFileError)
		os.Exit(1)
	}

	var config Config
	err = json.Unmarshal(configData, &config)

	if err != nil {
		log.Fatalln(sInvalidJSONError)
		os.Exit(1)
	}

	validate = validator.New()
	err = validate.Struct(config)

	if err != nil {
		for _, e := range err.(validator.ValidationErrors) {
			log.Fatalln(e)
		}
		os.Exit(1)
	}

	return config
}

func parseKey() *rsa.PublicKey {
	block, _ := pem.Decode([]byte(ReplDeployPublicKey))
	rsaPublicKey, err := x509.ParsePKCS1PublicKey(block.Bytes)

	if err != nil {
		panic(sPrivateKeyParseError)
	}

	return rsaPublicKey
}

func isOlderThanFifteenSeconds(ts int) bool {
	return ts < int((time.Now().UnixNano()/1000000)-15000)
}

func isPayloadOK(w http.ResponseWriter, body []byte, config Config) bool {
	var payload Payload
	err := json.Unmarshal(body, &payload)

	if err != nil {
		http.Error(w, sBadPayloadError, http.StatusBadRequest)
		return false
	}

	if isOlderThanFifteenSeconds(payload.Timestamp) {
		http.Error(w, sSignatureTooOldError, http.StatusUnauthorized)
		return false
	}

	if config.Endpoint != payload.Endpoint {
		http.Error(w, sBadEndpointError, http.StatusForbidden)
		return false
	}

	return true
}

func isSignatureOK(key *rsa.PublicKey, signature string, body []byte) bool {
	hash := sha256.Sum256(body)
	decodedSignature, err := base64.StdEncoding.DecodeString(signature)

	if err != nil {
		return false
	}

	err = rsa.VerifyPKCS1v15(key, crypto.SHA256, hash[:], decodedSignature)

	return err == nil
}
