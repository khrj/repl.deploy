use {
    super::constants::{
        PROBLEMS_SERIALIZING_JSON_ERROR, PROBLEMS_WRITING_TO_STDIN_OF_SUBPROCESS_ERROR,
        STAT_REQUEST_RECEIVED, STAT_SIGNATURE_VALIDATION_FAILED, STAT_SIGNATURE_VALIDATION_SUCCESS,
        STDIN_REGEX, STDIN_RESPONDED_SUCCESSFULLY, STDIN_SUCCESS,
    },
    super::signature_verifier,
    super::types::{Config, ValidationResult},
    anyhow::Result,
    log::{debug, error, info, warn},
    regex::Regex,
    rsa::RSAPublicKey,
    serde_json,
    std::{
        cell::RefCell,
        io::{self, BufRead, BufReader, Write},
        process::{self, Child},
        rc::Rc,
    },
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
        scan_process_stdout_until_success(
            child_ref.clone(),
            &stdin_regex,
            |payload, signature, writer| {
                info!("{}", STAT_REQUEST_RECEIVED);

                let response = match validation_result_to_string(validate_and_return_response(
                    payload, signature, config, public_key,
                )) {
                    Some(r) => r,
                    None => return,
                };

                debug!("Writing response: {}", &response);
                write_response(&response, writer);
            },
        );

        info!("{}", STDIN_RESPONDED_SUCCESSFULLY);
        debug!("Successful request, trying to restart process");

        match handler() {
            Ok(new_child) => {
                child_ref = new_child;
            }
            Err(e) => error!("{}", e),
        }
    }
}

fn scan_process_stdout_until_success(
    child: Rc<RefCell<Child>>,
    stdin_regex: &Regex,
    handle_request: impl Fn(&[u8], &str, &mut process::ChildStdin) -> (),
) {
    let mut child = child.borrow_mut();

    let mut writer = child.stdin.take().unwrap();
    let reader = BufReader::new(child.stdout.as_mut().unwrap());

    for line in reader.lines().filter_map(filter_valid_lines) {
        if line == STDIN_SUCCESS {
            break;
        }

        match get_matches(&line, stdin_regex) {
            Some((payload, signature)) => handle_request(payload, signature, &mut writer),
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

fn write_response(response: &str, writer: &mut std::process::ChildStdin) {
    if writer
        .write_all(response.as_bytes())
        .and(writer.flush())
        .is_err()
    {
        error!("{}", PROBLEMS_WRITING_TO_STDIN_OF_SUBPROCESS_ERROR)
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
            info!("{}", STAT_SIGNATURE_VALIDATION_SUCCESS);
            res
        }
        Err(e) => {
            warn!("{}", STAT_SIGNATURE_VALIDATION_FAILED);
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

fn validation_result_to_string(r: ValidationResult) -> Option<String> {
    let r = ValidationResultSerializable {
        body: r.body,
        status: r.status.as_u16(),
    };

    match serde_json::to_string(&r) {
        Ok(json) => Some(json + "\n"),
        Err(_) => {
            error!("{}", PROBLEMS_SERIALIZING_JSON_ERROR);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::{
            fs,
            process::{Command, Stdio},
        },
    };

    #[test]
    fn test_stdio() {
        println!(
            "{}",
            fs::canonicalize("./src/event_handlers/stdio_test/")
                .unwrap()
                .to_str()
                .unwrap()
        );

        compile_test_bin();

        let test_bin =
            Command::new(fs::canonicalize("./src/event_handlers/stdio_test/test_bin").unwrap())
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();

        let stdin_regex = Regex::new(STDIN_REGEX).unwrap();

        scan_process_stdout_until_success(
            Rc::new(RefCell::new(test_bin)),
            &stdin_regex,
            |_payload, _signature, writer| {
                writer.write_all(b"ok\n").and(writer.flush()).unwrap();
            },
        )
    }

    fn compile_test_bin() {
        Command::new("rustc")
            .arg("test_bin.rs")
            .current_dir(fs::canonicalize("./src/event_handlers/stdio_test/").unwrap())
            .output()
            .unwrap();
    }
}
