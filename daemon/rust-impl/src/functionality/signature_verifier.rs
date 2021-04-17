use {
    super::constants::{
        BAD_ENDPOINT_ERROR, BAD_PAYLOAD_ERROR, INVALID_SIGNATURE_ERROR, OK, PAYLOAD_TOO_OLD_ERROR,
    },
    super::types::{Config, Payload, ValidationResult},
    anyhow::Result,
    base64,
    reqwest::StatusCode,
    rsa::{hash, PaddingScheme, PublicKey, RSAPublicKey},
    serde_json,
    sha2::{Digest, Sha256},
    std::time::{SystemTime, UNIX_EPOCH},
};

pub fn validate_payload_and_signature(
    payload: &[u8],
    signature: &str,
    config: &Config,
    public_key: &RSAPublicKey,
) -> Result<ValidationResult, ValidationResult> {
    validate_payload(&payload, &config)?;
    validate_signature(&payload, &signature, &public_key)
}

fn validate_payload(body: &[u8], config: &Config) -> Result<ValidationResult, ValidationResult> {
    let payload: Payload = match serde_json::from_slice(body) {
        Ok(payload) => payload,
        Err(_) => {
            return Err(ValidationResult {
                body: BAD_PAYLOAD_ERROR,
                status: StatusCode::BAD_REQUEST,
            });
        }
    };

    if is_older_than_fifteen_seconds(payload.timestamp) {
        return Err(ValidationResult {
            body: PAYLOAD_TOO_OLD_ERROR,
            status: StatusCode::UNAUTHORIZED,
        });
    };

    if config.endpoint != payload.endpoint {
        return Err(ValidationResult {
            body: BAD_ENDPOINT_ERROR,
            status: StatusCode::FORBIDDEN,
        });
    };

    Ok(ValidationResult {
        body: OK,
        status: StatusCode::OK,
    })
}

fn validate_signature(
    body: &[u8],
    signature: &str,
    key: &RSAPublicKey,
) -> Result<ValidationResult, ValidationResult> {
    let decoded_signature = match base64::decode(signature) {
        Ok(sig) => sig,
        Err(_) => {
            return Err(ValidationResult {
                body: INVALID_SIGNATURE_ERROR,
                status: StatusCode::BAD_REQUEST,
            })
        }
    };

    let mut hasher = Sha256::new();
    hasher.update(body);
    let hashed = hasher.finalize();

    if key
        .verify(
            PaddingScheme::PKCS1v15Sign {
                hash: Some(hash::Hash::SHA2_256),
            },
            hashed.as_slice(),
            &decoded_signature,
        )
        .is_err()
    {
        return Err(ValidationResult {
            body: INVALID_SIGNATURE_ERROR,
            status: StatusCode::BAD_REQUEST,
        });
    }

    Ok(ValidationResult {
        body: OK,
        status: StatusCode::OK,
    })
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        rsa::{hash::Hash, PaddingScheme, RSAPrivateKey},
        serde_json,
        std::time::{SystemTime, UNIX_EPOCH},
    };

    const TEST_ENDPOINT: &str = "https://endpoint.example.com/";

    #[test]
    fn correct_payload() {
        let correct_payload = serde_json::to_vec(&Payload {
            timestamp: now_ms(),
            endpoint: TEST_ENDPOINT.to_owned(),
        })
        .unwrap();

        let result = validate_payload(
            &correct_payload,
            &Config {
                endpoint: TEST_ENDPOINT.to_owned(),
            },
        );

        assert!(result.is_ok());
    }

    #[test]
    fn invalid_json_payload() {
        let invalid_json_payload = TEST_ENDPOINT.as_bytes();

        let result = validate_payload(
            invalid_json_payload,
            &Config {
                endpoint: TEST_ENDPOINT.to_owned(),
            },
        );

        assert!(result.is_err());
    }

    #[test]
    fn old_payload() {
        let old_payload = serde_json::to_vec(&Payload {
            timestamp: now_ms() - 20000,
            endpoint: TEST_ENDPOINT.to_owned(),
        })
        .unwrap();

        let result = validate_payload(
            &old_payload,
            &Config {
                endpoint: TEST_ENDPOINT.to_owned(),
            },
        );

        assert!(result.is_err());
    }

    #[test]
    fn config_mismatch_payload() {
        let mismatch_payload = serde_json::to_vec(&Payload {
            timestamp: now_ms(),
            endpoint: TEST_ENDPOINT.to_owned(),
        })
        .unwrap();

        let result = validate_payload(
            &mismatch_payload,
            &Config {
                endpoint: "https://endpoint.bad-example.com/".to_owned(),
            },
        );

        assert!(result.is_err());
    }

    #[test]
    fn correct_signature() {
        const SAMPLE_BODY: &[u8] = "signature-body-test".as_bytes();
        let (pub_key, priv_key) = new_keypair();
        let signature = sign_and_hash(SAMPLE_BODY, &priv_key);
        let result = validate_signature(SAMPLE_BODY, &signature, &pub_key);
        assert!(result.is_ok());
    }

    #[test]
    fn incorrect_signature() {
        const SAMPLE_BODY: &[u8] = "signature-body-test".as_bytes();
        const SIGNATURE: &str = "hi";
        let (pub_key, _) = new_keypair();
        let result = validate_signature(SAMPLE_BODY, SIGNATURE, &pub_key);
        assert!(result.is_err());
    }

    #[test]
    fn verify_payload_and_signature() {
        let (pub_key, priv_key) = new_keypair();

        let payload = serde_json::to_vec(&Payload {
            timestamp: now_ms(),
            endpoint: TEST_ENDPOINT.to_owned(),
        })
        .unwrap();

        let signature = sign_and_hash(&payload, &priv_key);

        let result = validate_payload_and_signature(
            &payload,
            &signature,
            &Config {
                endpoint: TEST_ENDPOINT.to_owned(),
            },
            &pub_key,
        );

        assert!(result.is_ok(), "{:#}", result.unwrap_err());
    }

    // Helpers

    fn new_keypair() -> (RSAPublicKey, RSAPrivateKey) {
        use rand::rngs::OsRng;
        let mut rng = OsRng;
        let bits = 2048;
        let private_key =
            RSAPrivateKey::new(&mut rng, bits).expect("Failed to generate private key");
        let public_key = RSAPublicKey::from(&private_key);
        (public_key, private_key)
    }

    fn sign_and_hash(body: &[u8], priv_key: &RSAPrivateKey) -> String {
        base64::encode(
            priv_key
                .sign(
                    PaddingScheme::PKCS1v15Sign {
                        hash: Some(Hash::SHA2_256),
                    },
                    &hash(body),
                )
                .expect("signing request failed"),
        )
    }

    fn now_ms() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }

    fn hash(body: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(body);
        Vec::from(hasher.finalize().as_slice())
    }
}

fn is_older_than_fifteen_seconds(ts: u128) -> bool {
    ts < (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        - 15000)
}
