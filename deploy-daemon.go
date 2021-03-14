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
	// Config parsing

	configData, err := os.ReadFile("./replit-deploy.json")

	if err != nil {
		log.Fatalln("Config file doesn't exist")
		os.Exit(1)
	}

	var config Config
	err = json.Unmarshal(configData, &config)

	if err != nil {
		log.Fatalln("Invalid config JSON")
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

	// Key parsing

	block, _ := pem.Decode([]byte(ReplDeployPublicKey))
	rsaPublicKey, err := x509.ParsePKCS1PublicKey(block.Bytes)

	if err != nil {
		panic("Couldn't parse public key, open a new issue")
	}

	http.HandleFunc("/refresh", func(w http.ResponseWriter, req *http.Request) {
		body, err := io.ReadAll(req.Body)

		if err != nil {
			http.Error(w, "Failed to parse body", http.StatusBadRequest)
			return
		}

		signature := req.Header.Get("Signature")

		if isSignatureOK(rsaPublicKey, signature, body) {

			var payload Payload
			err = json.Unmarshal(body, &payload)

			if err != nil {
				http.Error(w, "Bad payload", http.StatusBadRequest)
				return
			}

			if isOlderThanFifteenSeconds(payload.Timestamp) {
				http.Error(w, "Signature too old", http.StatusUnauthorized)
				return
			}

			if config.Endpoint != payload.Endpoint {
				http.Error(w, "Signed request not intended for current endpoint", http.StatusForbidden)
				return
			}
		} else {
			http.Error(w, "Invalid Signature", http.StatusForbidden)
			return
		}
	})

	_ = http.ListenAndServe(":8090", nil)
}

func isOlderThanFifteenSeconds(ts int) bool {
	return ts < int((time.Now().UnixNano()/1000000)-15000)
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
