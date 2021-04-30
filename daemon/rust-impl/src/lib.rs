#[path = "static/constants.rs"]
mod constants;

#[path = "functionality/git_updater.rs"]
mod git_updater;

#[path = "functionality/signature_verifier.rs"]
mod signature_verifier;

#[path = "event_handlers/http.rs"]
mod http_event_handler;

#[path = "event_handlers/stdio.rs"]
mod stdio_event_handler;

#[path = "static/types.rs"]
mod types;

use {
    anyhow::{bail, Context, Result},
    constants::{
        FAILED_TO_KILL_CHILD_PROCESS_ERROR, FAILED_TO_START_CHILD_PROCESS_ERROR,
        GIT_FETCH_FAILED_STARTUP_WARN, INVALID_CONFIG_JSON_ERROR, MISSING_CONFIG_FILE_ERROR,
        PUBLIC_KEY_PARSE_ERROR, REPLIT_DEPLOY_JSON_PATH, STAT_PROGRAM_STARTED,
    },
    log::{debug, error, info, warn},
    rsa::RSAPublicKey,
    serde_json,
    std::{
        cell::RefCell,
        fs,
        process::{self, Child, Command, Stdio},
        rc::Rc,
        sync::{Arc, Mutex},
    },
    types::Config,
};

/*
The included public_key.bin is a decoded version of the key below. To decode
it yourself, remove the first and last line, and newlines, and run through a
base64 decoder.

-----BEGIN RSA PUBLIC KEY-----
MIIECgKCBAEAswkDXZlAqW1UpGiLFBW1ohSvUIqqcwrOt1ubbWrltrYT+3SQV24C
Su9j93+DX9tsFBuVDE3DSutddBmdWh0zFxDdSO+uA8JBJki9GfHNoynFcPLl3AxA
4iUh6nD6uSdXIGkJaJ+U8/Jix2AXS7Qk5Jfoktx88GtKoHAwznmfxdJwrFeiX8D8
Lqh34enh7pnntMp0vrpiTHu37H/VPGEAWkFoHuQMLoaHPgzF/Nk8NsjL2Uzvp8+Z
Vda8cXk2DeEm0x4q6kCWwchEcZF2jHcARjQ7ov7Vh5qZzlXcODt6i7NWUFX5h6g4
IodZXteh9apPaWSwXuMO+vCM3peYYfpFgVf/u2rh+wH6PjDiZE+keoA2PkPfvxVg
BUL54z6EYMR5pItN5MIqFigqBqUcrmoQhtwMZyU/bAVjqTjXa1pyE1wn18h1ufFf
6WXY/poVnmru+iA6IYG/D5YAolombTfA9U74qF1LWCIkahoNKjtX7cHRFDRT9OCo
inCiWiVG9WAbxMDU08j1CEut/yXhpSx8J4p878+LMapFChs7yIYV6TDS5UELKtBz
Ij6XWQKzT/PtwCYTxlZ+PlgMQw5ybG2imFzFy7JJpADkgWHGIn2j7Gzqo+DxcVC4
lotNBlZQTy5SVq+x6KwdJPG9+a6ECSiv7W+yyBh8QBPcC7oJAFdngSuvaE12TZvO
myRA05TX/Ron4/s0FbMrrP2K4oSuaCX6WlGcHcLNXz8OX0Egzyg3KKh5umzH8Ce9
ORoPwubbzXfZpbUGQb+iF8GPEp14z7VsDivjvzB/gaDqZ6+wSnPR6U+dk4SmP+Uk
/4Dc6ICxqct/BJOTMm9Fagp5mRcjXrTJ2TM+1ZKd/8lwL+gdcEYiNbb65d0ESN/1
qFWcjdihPqKjmn/5+PUdSl+wYfdbfnaT6fL01cOm/3xRS3l2A+9G5Bfh0PCdrg+A
+qKkGUp9cRD1w53ZS3zv/AmhY5e1VPc3mggpGn3uSseAc1NY5facH8ziiNfXLhQp
mjnOO5EsSjiXBXJ4uBisAbtiAaYELXYHOR1qf8catdI7jyUplCMpmqKT5ebUuhyh
6IP54Zx0YPznqwJSKJrPDoIxiD7iePQq0tOhxnMfGT8xeDZkTZ9sdgzbyqOnthX3
PUN9Kexr5nSWWfb0AJRTaZBxiXx4SKdo2yw6aaoIAOo6SyJLm0u0Qwa5Xm7GG0NS
0LsYDDPt/NNu+0tztpJM5DU6eRKePj9Lx8Xn8Hku3HqVR2LleSIyk7Z0G5yTZwdM
+9P0tsivT3+qKNy4BGin8mSBOCixhrL2YnNK5pOHrCXot562HTFKgvYz35u6sS6L
yggLIsW8CUnOIhj0AKovh9OvyC//N/GRLQIDAQAB
-----END RSA PUBLIC KEY-----
*/
const REPL_DEPLOY_PUBLIC_KEY: &[u8; 1038] = include_bytes!("static/public_key.bin");

pub enum EventHandler {
    Http,
    Stdio,
}

pub async fn listen(event_handler: EventHandler, cmd: String, cmd_args: Vec<String>) {
    let repl_deploy_public_key =
        RSAPublicKey::from_pkcs1(REPL_DEPLOY_PUBLIC_KEY).unwrap_or_else(|_err| {
            error!("{}", PUBLIC_KEY_PARSE_ERROR);
            process::exit(1);
        });

    let config: Config = serde_json::from_str(
        &fs::read_to_string(REPLIT_DEPLOY_JSON_PATH).unwrap_or_else(|_err| {
            error!("{}", MISSING_CONFIG_FILE_ERROR);
            process::exit(1);
        }),
    )
    .unwrap_or_else(|_err| {
        error!("{}", INVALID_CONFIG_JSON_ERROR);
        process::exit(1)
    });

    if let Err(e) = git_updater::update_git_from_remote(None) {
        error!("{}", e);
        warn!("{}", GIT_FETCH_FAILED_STARTUP_WARN);
    }

    match event_handler {
        EventHandler::Http => listen_http(repl_deploy_public_key, config, cmd, cmd_args).await,
        EventHandler::Stdio => listen_stdio(repl_deploy_public_key, config, cmd, cmd_args),
    }
}

async fn listen_http(pub_key: RSAPublicKey, config: Config, cmd: String, cmd_args: Vec<String>) {
    let child = match Command::new(&cmd).args(&cmd_args).spawn() {
        Ok(child_handle) => child_handle,
        Err(_) => {
            error!("{}", FAILED_TO_START_CHILD_PROCESS_ERROR);
            process::exit(1)
        }
    };

    http_event_handler::listen(
        Arc::new(config),
        Arc::new(pub_key),
        Arc::new(Mutex::new(child)),
        move |child| -> Result<()> {
            let mut c = child.lock().unwrap();
            let cmd_args: Vec<_> = cmd_args.iter().map(|s| s.as_str()).collect();
            match update_and_restart_process(&mut c, &cmd, &cmd_args, EventHandler::Http) {
                Ok(new_handle) => {
                    *c = new_handle;
                    Ok(())
                }
                Err(e) => {
                    error!("{}", e);
                    bail!(e);
                }
            }
        },
    )
    .await
}

fn listen_stdio(pub_key: RSAPublicKey, config: Config, cmd: String, cmd_args: Vec<String>) {
    let mut child = Rc::new(RefCell::new(
        match Command::new(&cmd)
            .args(&cmd_args)
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()
        {
            Ok(child_handle) => child_handle,
            Err(_) => {
                error!("{}", FAILED_TO_START_CHILD_PROCESS_ERROR);
                process::exit(1)
            }
        },
    ));

    stdio_event_handler::listen(&pub_key, &config, child.clone(), &mut move || {
        let child_ref = child.clone();
        let cmd_args: Vec<_> = cmd_args.iter().map(|s| s.as_str()).collect();

        debug!("Updating and restarting process...");

        let result = update_and_restart_process(
            &mut *child_ref.borrow_mut(),
            &cmd,
            &cmd_args,
            EventHandler::Stdio,
        );

        debug!("Updated and restarted process!");

        match result {
            Ok(new_handle) => {
                child = Rc::new(RefCell::new(new_handle));
                Ok(child.clone())
            }
            Err(e) => {
                error!("{}", e);
                bail!(e);
            }
        }
    })
}

fn update_and_restart_process(
    child_handle: &mut Child,
    cmd: &str,
    cmd_args: &[&str],
    event_handler: EventHandler,
) -> Result<Child> {
    git_updater::update_git_from_remote(None)?;

    child_handle
        .kill()
        .with_context(|| FAILED_TO_KILL_CHILD_PROCESS_ERROR)?;

    let child = match event_handler {
        EventHandler::Http => Command::new(cmd).args(cmd_args).spawn(),
        EventHandler::Stdio => Command::new(cmd)
            .args(cmd_args)
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn(),
    };

    match child {
        Ok(child_handle) => {
            info!("{}", STAT_PROGRAM_STARTED);
            Ok(child_handle)
        }
        Err(_) => bail!(FAILED_TO_START_CHILD_PROCESS_ERROR),
    }
}
