use {
    anyhow::Result,
    constants::{
        PROBLEMS_SERIALIZING_JSON_ERROR, PROBLEMS_WRITING_TO_STDIN_OF_SUBPROCESS_ERROR,
        STAT_REQUEST_RECEIVED, STAT_SIGNATURE_VALIDATION_FAILED, STAT_SIGNATURE_VALIDATION_SUCCESS,
        STDIN_REGEX, STDIN_RESPONDED_SUCCESSFULLY, STDIN_SUCCESS,
    },
    logger,
    regex::Regex,
    rsa::RSAPublicKey,
    serde_json, signature_verifier,
    std::{
        cell::RefCell,
        io::{self, BufRead, BufReader, Write},
        process::Child,
        rc::Rc,
    },
    types::{Config, ValidationResult},
};

pub fn listen(
    public_key: &RSAPublicKey,
    config: &Config,
    child: Rc<RefCell<Child>>,
    handler: &mut impl FnMut() -> Result<Rc<RefCell<Child>>>,
) {
    let mut child_ref = child;
    let stdin_regex = Regex::new(STDIN_REGEX).unwrap();
    loop {
        scan_process_stdout_until_successful_request(
            child_ref.clone(),
            &stdin_regex,
            config,
            public_key,
        );

        match handler() {
            Ok(new_child) => {
                child_ref = new_child;
            }
            Err(e) => logger::error(&e.to_string()),
        }
    }
}

fn scan_process_stdout_until_successful_request(
    child: Rc<RefCell<Child>>,
    stdin_regex: &Regex,
    config: &Config,
    public_key: &RSAPublicKey,
) {
    let mut child = child.borrow_mut();

    let mut stdout = child.stdout.take();
    let mut stdin = child.stdin.take();

    let reader = BufReader::new(stdout.as_mut().unwrap());
    let writer = stdin.as_mut().unwrap();

    for line in reader.lines().filter_map(filter_valid_lines) {
        if line == STDIN_SUCCESS {
            logger::success(STDIN_RESPONDED_SUCCESSFULLY);
            break;
        }

        match get_matches(&line, stdin_regex) {
            Some((payload, input_signature)) => {
                logger::info(STAT_REQUEST_RECEIVED);

                let response = match validation_result_to_bytes(validate_and_return_response(
                    payload,
                    input_signature,
                    config,
                    public_key,
                )) {
                    Some(r) => r,
                    None => continue,
                };

                write_response(&response, writer);
            }
            None => println!("{}", &line),
        }
    }
}

// Helpers

fn filter_valid_lines(line: Result<String, io::Error>) -> Option<String> {
    match line {
        Ok(line) => Some(line),
        Err(_) => None,
    }
}

fn write_response(response: &[u8], writer: &mut std::process::ChildStdin) {
    if writer.write(response).is_err() {
        logger::error(PROBLEMS_WRITING_TO_STDIN_OF_SUBPROCESS_ERROR)
    };
}

fn get_matches<'a>(line: &'a str, stdin_regex: &Regex) -> Option<(&'a [u8], &'a str)> {
    let matches = stdin_regex.captures(line)?;
    let payload = matches.get(1)?.as_str().as_bytes();
    let signature = matches.get(2)?.as_str();

    Some((payload, signature))
}

fn validate_and_return_response(
    payload: &[u8],
    input_signature: &str,
    config: &Config,
    public_key: &RSAPublicKey,
) -> ValidationResult {
    match signature_verifier::validate_payload_and_signature(
        payload,
        input_signature,
        config,
        public_key,
    ) {
        Ok(res) => {
            logger::success(STAT_SIGNATURE_VALIDATION_SUCCESS);
            res
        }
        Err(e) => {
            logger::warn(STAT_SIGNATURE_VALIDATION_FAILED);
            e
        }
    }
}

use serde::Serialize;

#[derive(Serialize)]
struct ValidationResultSerializable {
    body: &'static str,
    status: u16,
}

fn validation_result_to_bytes(r: ValidationResult) -> Option<Vec<u8>> {
    let r = ValidationResultSerializable {
        body: r.body,
        status: r.status.as_u16(),
    };

    match serde_json::to_vec(&r) {
        Ok(json) => Some(json),
        Err(_) => {
            logger::error(PROBLEMS_SERIALIZING_JSON_ERROR);
            None
        }
    }
}
