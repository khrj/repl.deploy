use {
    super::constants::{GIT_FETCH_FAILED_ERROR, GIT_RESET_FAILED_ERROR},
    anyhow::{bail, Result},
    std::{path::Path, process::Command},
};

pub fn update_git_from_remote(cwd: Option<&Path>) -> Result<()> {
    let git_fetch = match cwd {
        Some(path) => Command::new("git")
            .args(&["fetch", "--all"])
            .current_dir(path)
            .output(),
        None => Command::new("git").args(&["fetch", "--all"]).output(),
    };

    if git_fetch.is_err() {
        bail!(GIT_FETCH_FAILED_ERROR);
    }

    let git_reset = match cwd {
        Some(path) => Command::new("git")
            .args(&["reset", "--hard", "origin/main"])
            .current_dir(path)
            .output(),
        None => Command::new("git")
            .args(&["reset", "--hard", "origin/main"])
            .output(),
    };

    if git_reset.is_err() {
        bail!(GIT_RESET_FAILED_ERROR);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::{fs, path::Path},
    };

    #[test]
    fn try_reset() {
        prepare_repos();
        update_git_from_remote(Some(Path::new("./test_repo2"))).expect("Failed to update from git");
        let file_contents = fs::read_to_string("./test_repo2/temp")
            .expect("Failed to read contents of test_repo2/temp");
        assert_eq!(file_contents, "hi")
    }

    fn prepare_repos() {
        println!(
            "{}",
            String::from_utf8_lossy(
                &Command::new("./src/functionality/git_updater_test_prep.sh")
                    .output()
                    .expect("Failed to prepare repos")
                    .stdout
            )
        );
    }
}
