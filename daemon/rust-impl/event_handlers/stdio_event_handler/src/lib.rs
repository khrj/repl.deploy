use {
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
		io::{self, BufRead, BufReader, Write},
		process::Child,
	},
	types::{Config, ValidationResult},
};

pub fn listen<'a>(
	config: &Config,
	public_key: &RSAPublicKey,
	handler: impl Fn() -> Result<&'a mut Child, &'static str>,
	cmd: &mut Child,
) {
	let mut child_ref = cmd;
	let stdin_regex = Regex::new(STDIN_REGEX).unwrap();
	loop {
		scan_process_stdout_until_successful_request(child_ref, &stdin_regex, config, public_key);

		match handler() {
			Ok(new_cmd) => {
				child_ref = new_cmd;
			}
			Err(e) => logger::error(e),
		}
	}
}

fn scan_process_stdout_until_successful_request(
	cmd: &mut Child,
	stdin_regex: &Regex,
	config: &Config,
	public_key: &RSAPublicKey,
) {
	let reader = BufReader::new(cmd.stdout.as_mut().unwrap());
	let writer = cmd.stdin.as_mut().unwrap();

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
	if let Err(_) = writer.write(response) {
		logger::error(PROBLEMS_WRITING_TO_STDIN_OF_SUBPROCESS_ERROR)
	};
}

fn get_matches<'a>(line: &'a str, stdin_regex: &Regex) -> Option<(&'a str, &'a str)> {
	let matches = stdin_regex.captures(line)?;
	let payload = matches.get(1)?.as_str();
	let signature = matches.get(2)?.as_str();

	Some((payload, signature))
}

fn validate_and_return_response(
	payload: &str,
	input_signature: &str,
	config: &Config,
	public_key: &RSAPublicKey,
) -> ValidationResult<'static> {
	match signature_verifier::validate_payload_and_signature(
		payload,
		input_signature,
		config,
		public_key,
	) {
		Ok(()) => {
			logger::success(STAT_SIGNATURE_VALIDATION_SUCCESS);

			return ValidationResult {
				body: "OK",
				status: 200,
			};
		}
		Err(e) => {
			logger::warn(STAT_SIGNATURE_VALIDATION_FAILED);
			return e;
		}
	};
}

fn validation_result_to_bytes(r: ValidationResult) -> Option<Vec<u8>> {
	match serde_json::to_vec(&r) {
		Ok(json) => Some(json),
		Err(_) => {
			logger::error(PROBLEMS_SERIALIZING_JSON_ERROR);
			None
		}
	}
}