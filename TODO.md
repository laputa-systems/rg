Prefix: RIP
Next: 3

### RIP-1 · P2 · Release binaries are built without PCRE2

- **Area:** Build
- **Where:** `.github/workflows/release.yml:55`, `crates/core/flags/hiargs.rs:449`, `crates/core/main.rs:63`, `Cargo.toml:75`
- **Problem:** The release workflow runs `cargo build --locked --release --bin rg` and never passes `--features pcre2`. That feature is off unless requested (`Cargo.toml` lists it and sets no default). `HiArgs::matcher_pcre2` then returns `PCRE2 is not available in this build of ripgrep`, and `main` exits with status 2. Every zip from the successful run [40239766661](https://github.com/laputa-systems/rg/actions/runs/40239766661) (commit `4570df7`, the current release workflow) was built that way. `-P` fails on all three published targets. Upstream's release build passed `--features pcre2`. This fork's rewrite dropped the flag and did not say so.
- **Recommendation:** Add `--features pcre2` to that `cargo build`. `musl-tools` is already installed for the Linux jobs. Re-run the workflow and keep the existing static ELF check (no `INTERP`, no `NEEDED`). Whether PCRE2 still links statically on those musl targets is not proven here.
- **Fix with:** none
- **Effort:** S

### RIP-2 · P3 · No security contact for this fork

- **Area:** Security
- **Where:** `README.md` Vulnerability reporting
- **Problem:** The only person named for vulnerability reports is Andrew Gallant, who maintains BurntSushi/ripgrep. He does not publish this fork's prerelease binaries. A report about those binaries has nowhere private to go.
- **Recommendation:** Publish a contact for this repository and point the README at that contact only.
- **Fix with:** none
- **Effort:** S
- **Design:** [docs/rfc/RIP-2-security-contact.md](docs/rfc/RIP-2-security-contact.md)
