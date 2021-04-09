mod helpers;

use {
    base64,
    constants::{
        http_status, BAD_ENDPOINT_ERROR, BAD_PAYLOAD_ERROR, INVALID_SIGNATURE_ERROR,
        PAYLOAD_TOO_OLD_ERROR,
    },
    helpers::is_older_than_fifteen_seconds,
    rsa::{hash, PaddingScheme, PublicKey, RSAPublicKey},
    serde_json,
    sha2::{Digest, Sha256},
    types::{Config, Payload, ValidationResult},
};

pub fn validate_payload_and_signature<'a>(
    payload: &str,
    signature: &str,
    config: &Config,
    public_key: &RSAPublicKey,
) -> Result<(), ValidationResult<'a>> {
    validate_payload(&payload, &config)?;
    validate_signature(&payload, &signature, &public_key)?;

    Ok(())
}

fn validate_payload<'a>(body: &str, config: &Config) -> Result<(), ValidationResult<'a>> {
    let payload: Payload = match serde_json::from_str(body) {
        Ok(payload) => payload,
        Err(_) => {
            return Err(ValidationResult {
                body: BAD_PAYLOAD_ERROR,
                status: http_status::BAD_REQUEST,
            });
        }
    };

    if is_older_than_fifteen_seconds(payload.timestamp) {
        return Err(ValidationResult {
            body: PAYLOAD_TOO_OLD_ERROR,
            status: http_status::UNAUTHORIZED,
        });
    };

    if config.endpoint != payload.endpoint {
        return Err(ValidationResult {
            body: BAD_ENDPOINT_ERROR,
            status: http_status::FORBIDDEN,
        });
    };

    Ok(())
}

fn validate_signature<'a>(
    body: &str,
    signature: &str,
    key: &RSAPublicKey,
) -> Result<(), ValidationResult<'a>> {
    let decoded_signature = match base64::decode(signature) {
        Ok(sig) => sig,
        Err(_) => {
            return Err(ValidationResult {
                body: INVALID_SIGNATURE_ERROR,
                status: http_status::BAD_REQUEST,
            });
        }
    };

    let mut hasher = Sha256::new();
    hasher.update(body);
    let hashed = hasher.finalize();

    match key.verify(
        PaddingScheme::PKCS1v15Sign {
            hash: Some(hash::Hash::SHA2_256),
        },
        hashed.as_slice(),
        &decoded_signature,
    ) {
        Ok(_) => (),
        Err(_) => {
            return Err(ValidationResult {
                body: INVALID_SIGNATURE_ERROR,
                status: http_status::BAD_REQUEST,
            });
        }
    };

    Ok(())
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
        let correct_payload = serde_json::to_string(&Payload {
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
        let invalid_json_payload = TEST_ENDPOINT;

        let result = validate_payload(
            &invalid_json_payload,
            &Config {
                endpoint: TEST_ENDPOINT.to_owned(),
            },
        );

        assert!(result.is_err());
    }

    #[test]
    fn old_payload() {
        let old_payload = serde_json::to_string(&Payload {
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

        assert!(result.is_err())
    }

    #[test]
    fn config_mismatch_payload() {
        let mismatch_payload = serde_json::to_string(&Payload {
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

        assert!(result.is_err())
    }

    #[test]
    fn correct_signature() {
        const SAMPLE_BODY: &str = "signature-body-test";
        let (pub_key, priv_key) = new_keypair();
        let signature = sign_and_hash(SAMPLE_BODY, &priv_key);
        let result = validate_signature(SAMPLE_BODY, &signature, &pub_key);
        assert!(result.is_ok());
    }

    #[test]
    fn incorrect_signature() {
        const SAMPLE_BODY: &str = "signature-body-test";
        const SIGNATURE: &str = "hi";
        let (pub_key, _) = new_keypair();
        let result = validate_signature(SAMPLE_BODY, SIGNATURE, &pub_key);
        assert!(result.is_err());
    }

    #[test]
    fn verify_payload_and_signature() {
        let payload = serde_json::to_string(&Payload {
            timestamp: now_ms(),
            endpoint: TEST_ENDPOINT.to_owned(),
        })
        .unwrap();

        let (pub_key, priv_key) = new_keypair();
        let signature = sign_and_hash(&payload, &priv_key);

        let result = validate_payload_and_signature(
            &payload,
            &signature,
            &Config {
                endpoint: TEST_ENDPOINT.to_owned(),
            },
            &pub_key,
        );

        assert!(result.is_ok());
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

    fn sign_and_hash(body: &str, priv_key: &RSAPrivateKey) -> String {
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

    fn hash(body: &str) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(body);
        Vec::from(hasher.finalize().as_slice())
    }
}
