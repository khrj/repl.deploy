mod constants;

use {
    constants::*,
    logger,
    std::{io, process::Command},
};

pub fn update_git_from_remote() -> Result<(), io::Error> {
    let git_fetch = Command::new("git").args(&["fetch", "--all"]).output();

    if let Err(e) = git_fetch {
        logger::error(GIT_FETCH_FAILED_ERROR);
        return Err(e);
    }

    let git_reset = Command::new("git")
        .args(&["reset", "--hard", "origin/main"])
        .output();

    if let Err(e) = git_reset {
        logger::error(GIT_RESET_FAILED_ERROR);
        return Err(e);
    }

    Ok(())
}