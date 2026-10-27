#[test]
fn release_build_passes_pcre2() {
    let workflow = include_str!("../.github/workflows/release.yml");
    let command = build_release_run(workflow).expect("Build release step");
    let tokens: Vec<&str> = command.split_whitespace().collect();
    assert!(
        tokens.windows(2).any(|pair| pair == ["--features", "pcre2"]),
        "release build must pass --features pcre2: {command}",
    );
    for required in ["--locked", "--release", "--bin", "--target"] {
        assert!(
            tokens.contains(&required),
            "release build dropped {required}: {command}",
        );
    }
    assert!(
        tokens.contains(&"rg"),
        "release build must build bin rg: {command}",
    );
}

fn build_release_run(workflow: &str) -> Option<&str> {
    let mut in_step = false;
    for line in workflow.lines() {
        let trimmed = line.trim();
        if trimmed == "- name: Build release" {
            in_step = true;
            continue;
        }
        if !in_step {
            continue;
        }
        if trimmed.starts_with("- ") {
            break;
        }
        if let Some(command) = trimmed.strip_prefix("run:") {
            return Some(command.trim());
        }
    }
    None
}
