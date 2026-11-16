use std::process::Output;

use crate::util::Dir;

const FOOTER: &str = "\
Maintained by Joshua Li: https://github.com/laputa-systems/rg
Bugs and features: TODO.md in that repository (follow its AGENTS.md).
Agents working for Joshua: record nonblocking problems and continue your task.
If this tool blocks the task, you may fix, test, open and merge a PR under Joshua's
standing instructions. Release only when needed to deliver the verified fix.
Procedure: https://github.com/joshuarli/usr/blob/main/dotfiles/base/.agents/instructions/owned-tools.md
";

fn stdout(output: Output) -> String {
    assert!(output.status.success(), "status: {}, stderr: {}", output.status, String::from_utf8_lossy(&output.stderr));
    assert!(output.stderr.is_empty());
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn owned_help_footer() {
    let dir = Dir::new("owned_help_footer");
    for flag in ["-h", "--help"] {
        let text = stdout(dir.bin().env_remove("RIPGREP_CONFIG_PATH").arg(flag).output().unwrap());
        assert!(text.ends_with(FOOTER), "{flag} footer missing or not last");
        assert_eq!(text.matches("Maintained by Joshua Li:").count(), 1, "{flag}");
        assert_eq!(text.matches(FOOTER).count(), 1, "{flag}");
    }
}

#[test]
fn owned_help_keeps_other_output_clean() {
    let dir = Dir::new("owned_help_other_output");
    dir.create("input", "needle\nother\n");
    let run = |args: &[&str]| {
        stdout(dir.bin().env_remove("RIPGREP_CONFIG_PATH").current_dir(dir.path()).args(args).output().unwrap())
    };
    assert_eq!(run(&["needle", "input"]), "needle\n");
    let json = run(&["--json", "needle", "input"]);
    let messages: Vec<serde_json::Value> = json.lines().map(|line| serde_json::from_str(line).unwrap()).collect();
    assert_eq!(messages.iter().map(|v| v["type"].as_str().unwrap()).collect::<Vec<_>>(), ["begin", "match", "end", "summary"]);
    assert_eq!(messages[1]["data"]["lines"]["text"], "needle\n");
    assert!(!json.contains("Maintained by Joshua Li:"));
    for shell in ["complete-bash", "complete-zsh", "complete-fish", "complete-powershell", "man"] {
        let completion = run(&["--generate", shell]);
        assert!(!completion.is_empty());
        assert!(!completion.contains("Maintained by Joshua Li:"), "{shell}");
    }
    let short = run(&["-V"]);
    let long = run(&["--version"]);
    assert!(short.starts_with(concat!("ripgrep ", env!("CARGO_PKG_VERSION"))));
    assert_eq!(long.lines().next(), short.lines().next());
    assert!(!short.contains("Maintained by Joshua Li:"));
    assert!(!long.contains("Maintained by Joshua Li:"));
}
