fn main() {
    set_git_revision_hash();
}

/// Make the current git hash available to the build as the environment
/// variable `RIPGREP_BUILD_GIT_HASH`.
fn set_git_revision_hash() {
    use std::process::Command;

    let args = &["rev-parse", "--short=10", "HEAD"];
    let output = Command::new("git").args(args).output();
    match output {
        Ok(output) => {
            let rev =
                String::from_utf8_lossy(&output.stdout).trim().to_string();
            if rev.is_empty() {
                println!(
                    "cargo:warning=output from `git rev-parse` is empty, \
                     so skipping embedding of commit hash"
                );
                return;
            }
            println!("cargo:rustc-env=RIPGREP_BUILD_GIT_HASH={rev}");
        }
        Err(e) => {
            println!(
                "cargo:warning=failed to run `git rev-parse`, \
                 so skipping embedding of commit hash: {e}"
            );
        }
    }
}
