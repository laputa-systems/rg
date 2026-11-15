#[test]
fn release_builds_pass_pcre2() {
    let workflow = include_str!("../.github/workflows/release.yml");
    for step in ["Build macOS release", "Build and verify Linux release"] {
        let command = step_run(workflow, step)
            .unwrap_or_else(|| panic!("missing step {step}"));
        let tokens: Vec<&str> = command
            .split_whitespace()
            .filter(|token| *token != "\\")
            .collect();
        assert!(
            tokens.windows(2).any(|pair| pair == ["--features", "pcre2"]),
            "{step} must pass --features pcre2: {command}",
        );
        for required in ["--locked", "--release", "--bin", "--target"] {
            assert!(
                tokens.contains(&required),
                "{step} dropped {required}: {command}",
            );
        }
        assert!(
            tokens.windows(2).any(|pair| pair == ["--bin", "rg"]),
            "{step} must build bin rg: {command}",
        );
    }
}

fn step_run(workflow: &str, name: &str) -> Option<String> {
    let header = format!("- name: {name}");
    let mut lines = workflow.lines().skip_while(|line| line.trim() != header);
    let indent = lines.next()?.find('-')?;
    let mut command = String::new();
    let mut in_run = false;
    for line in lines {
        let trimmed = line.trim();
        let column = line.len() - line.trim_start().len();
        if !trimmed.is_empty() && column <= indent {
            break;
        }
        if in_run {
            command.push_str(trimmed);
            command.push('\n');
        } else if let Some(rest) = trimmed.strip_prefix("run:") {
            let rest = rest.trim();
            if rest != "|" {
                return Some(rest.to_string());
            }
            in_run = true;
        }
    }
    in_run.then_some(command)
}
