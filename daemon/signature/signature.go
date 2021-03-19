package signature

import (
	"crypto"
	"crypto/rsa"
	"crypto/sha256"
	"encoding/base64"
	"encoding/json"
	"net/http"
)

var config = parseConfig()
var rsaPublicKey = parseKey()

func ValidateSignatureAndPayload(signature string, body []byte) *ValidationResult {
	err := validateSignature(rsaPublicKey, signature, body)

	if err != nil {
		return &ValidationResult{
			Body:   sInvalidSignatureError,
			Status: http.StatusForbidden,
		}
	}

	validationError := validatePayload(body, config)

	if validationError != nil {
		return validationError
	}

	return nil
}

func validatePayload(body []byte, config Config) *ValidationResult {
	var payload Payload
	err := json.Unmarshal(body, &payload)

	if err != nil {
		return &ValidationResult{
			Body:   sBadPayloadError,
			Status: http.StatusBadRequest,
		}
	}

	if isOlderThanFifteenSeconds(payload.Timestamp) {
		return &ValidationResult{
			Body:   sSignatureTooOldError,
			Status: http.StatusUnauthorized,
		}
	}

	if config.Endpoint != payload.Endpoint {
		return &ValidationResult{
			Body:   sBadEndpointError,
			Status: http.StatusForbidden,
		}
	}

	return nil
}

func validateSignature(key *rsa.PublicKey, signature string, body []byte) error {
	hash := sha256.Sum256(body)
	decodedSignature, err := base64.StdEncoding.DecodeString(signature)

	if err != nil {
		return err
	}

	err = rsa.VerifyPKCS1v15(key, crypto.SHA256, hash[:], decodedSignature)

	if err != nil {
		return err
	}

	return nil
}
