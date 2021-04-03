mod constants;

use {
    constants::{PUBLIC_KEY_PARSE_ERROR, REPL_DEPLOY_PUBLIC_KEY},
    rsa::RSAPublicKey,
    std::process,
};

fn watch_or_serve() {
    let repl_deploy_public_key =
        RSAPublicKey::from_pkcs1(REPL_DEPLOY_PUBLIC_KEY).unwrap_or_else(|_err| {
            println!("{}", PUBLIC_KEY_PARSE_ERROR);
            process::exit(0);
        });
}
