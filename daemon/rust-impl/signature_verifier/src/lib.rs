mod constants;
mod helpers;
mod types;

use {
    base64,
    constants::{
        http_status, BAD_ENDPOINT_ERROR, BAD_PAYLOAD_ERROR, INVALID_SIGNATURE_ERROR,
        SIGNATURE_TOO_OLD_ERROR,
    },
    helpers::is_older_than_fifteen_seconds,
    rsa::{hash, PaddingScheme, PublicKey, RSAPublicKey},
    serde_json,
    sha2::{Digest, Sha256},
    types::{Config, Payload, ValidationResult},
};

pub fn validate_payload_and_signature<'a>(
    payload: String,
    signature: String,
    config: Config,
    public_key: RSAPublicKey,
) -> Result<(), ValidationResult<'a>> {
    validate_payload(&payload, &config)?;
    validate_signature(&payload, &signature, &public_key)?;

    Ok(())
}

fn validate_payload<'a>(body: &String, config: &Config) -> Result<(), ValidationResult<'a>> {
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
            body: SIGNATURE_TOO_OLD_ERROR,
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
    body: &String,
    signature: &String,
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
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    const TEST_ENDPOINT: &str = "https://endpoint.example.com/";

    #[test]
    fn correct_payload() {
        let correct_payload: String = format!(
            "{{\"timestamp\":{},\"endpoint\":\"{}\"}}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            TEST_ENDPOINT
        );

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
        let invalid_json_payload: String = format!("{{{}}}", TEST_ENDPOINT);

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
        let old_payload: String = format!(
            "{{\"timestamp\":{},\"endpoint\":\"{}\"}}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                - 20,
            TEST_ENDPOINT
        );

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
        let mismatch_payload: String = format!(
            "{{\"timestamp\":{},\"endpoint\":\"{}\"}}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            TEST_ENDPOINT
        );

        let result = validate_payload(
            &mismatch_payload,
            &Config {
                endpoint: "https://endpoint.bad-example.com/".to_owned(),
            },
        );

        assert!(result.is_err())
    }

    const SIGNED_REQUEST: &str =
        r#"{"timestamp":1617467088492,"endpoint":"https://endpoint.example.com/"}"#;

    #[test]
    fn correct_signature() {
        const SIGNATURE: &str = "rJ7tnUm5rr1hw0crlxuomuuLaD/symH7q89fJDsf/ZASQMTWFFQXiqmPPNnxHiSJaX8jNbKQyX5DlpjGMerCJLgpT1SPT1/kxauQ4w10HgieKlnODCQvgODemdVfiBQHnrNM3A5b0KuEQVNnxsw+62KHe9KWO/gc9i0pFaYq767G6/P/36XP2+pvKSHZck+kbnEPGWkQQLdsVJRhC90haVnDPdjpsPYHQCPbg7QGnMVZvz8hgfJ4SYImdyybcgZsxE49D6Bc9Dc0BwX8SCK4D1cmSXvu/iW/xG01XqILi9Q1/wquKv5h9Ykr723EQ33/wEtW5pJRWKmpb1qORBkgPKsI5beQhClSRxrE7AKuMioDvPa98Roq0HZ1mfersWa5qXGy8D1iZkQ/v6xa3SgqiXeic0eugqtpv6a9hOgLXUx44SxFHeb4KBhzkGSj9b0qV6UaB+6851F9XubWHXtb33MQ5IWdevUa/g/ym40MWgZLnopk1EO62r6TWOmPYuqohNqZjnmfkb/oCXd180P9ck4ZGaR3uHaQ7VCnYTnILQhOTrqzemotsX/B/L/S/XVTLir30kc5Lwc6mY5+ANclKpjdnnH5bzyWE8zzLy33OxuPQc+bUFm+D+s62m+KR04/hUoBntA2zV+9spOzXbcf+UcsWSEmVpoIF1zQqZt86j9P+TExZ2Vbkvj+x/sZBB4JcaOVkzA8f0ecE7nNyM30IFr+PybwxanzalDZ81MslPnTp4b2sQ8SzFM2XW8WEtoxj6o2O36JoH0SsUbwnmua2Xhn+v9D0LFksl3IJdWkzx5subhOXsgs7877+Vi1ctpS9AvLbZsqrztK/lp/q+Rq/pnt9kz/gKdkpiUAHwHTakD3X0WLgJlZV1U0ihqMi7PqsM0qBboSDBJqahsnuzhViyTCxGKd3BJ6mcWrfr2mAlLlP5bpw27r6QSpMufLAOBMpyw++ULCQeS1BbI21EGuUwRIzPmoUSOCfrxFMvj+fDIpdj89tGC1Fafb1UygjELsrKB8O7Xyh9IZsW6O0PpP+gWrr1eYzhPN4+UTAh+kA1TQkUXSrfwfv13KJMRC7BepDbtVzOyJaChElcUvzkxnOyApKpV5Tj1dVkpFqw5y9KP/dAd4hCzp1gtVCbx6rwKp/oSQgvJgq5CMmmNHEKUGmYhn5s1oXBAB4PGdBlw/lE/ucsdljnftiEOX7iFBrFPLAbcMzvSUiVbvlRlG2x5XUR+G58r1DWbsSGgitpkqCqQuTFq4Gp2ho/gkTWmXf2UP1KHpkw6pJLKGTnwQ2LBjpNugZO8gpGXdgVybRXhLIShF9hnJBIhL5oZzwv37Yft35vM4FN5wOrRgxpZOouHELA==";
        const REPL_DEPLOY_PUBLIC_KEY: &[u8; 1038] = include_bytes!("../../cli/public_key.bin");

        let repl_deploy_public_key = RSAPublicKey::from_pkcs1(REPL_DEPLOY_PUBLIC_KEY).unwrap();

        assert!(validate_signature(
            &SIGNED_REQUEST.to_owned(),
            &SIGNATURE.to_owned(),
            &repl_deploy_public_key,
        )
        .is_ok());
    }

    #[test]
    fn incorrect_signature() {
        const SIGNATURE: &str = "hi";
        const REPL_DEPLOY_PUBLIC_KEY: &[u8; 1038] = include_bytes!("../../cli/public_key.bin");

        let repl_deploy_public_key = RSAPublicKey::from_pkcs1(REPL_DEPLOY_PUBLIC_KEY).unwrap();

        assert!(validate_signature(
            &SIGNED_REQUEST.to_owned(),
            &SIGNATURE.to_owned(),
            &repl_deploy_public_key,
        )
        .is_err());
    }
}
