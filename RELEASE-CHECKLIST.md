# Release

`.github/workflows/release.yml` is the release. It runs only on
`workflow_dispatch`. It does not run when you push a tag, and it does not
publish to crates.io.

1. Run `cargo test --all` on the commit you want to ship.
2. Dispatch the `release` workflow on that commit.
3. The workflow builds `rg` for `aarch64-apple-darwin`,
   `x86_64-unknown-linux-musl`, and `aarch64-unknown-linux-musl`.
   The command is `cargo build --locked --release --features pcre2 --bin rg`.
   `rust-toolchain.toml` selects the compiler.
4. The workflow rejects a Linux ELF that has an interpreter or a `NEEDED`
   entry. It then uploads three zip archives and creates the prerelease tag
   `release-<sha>`.

Do not `cargo publish` this tree as `ripgrep`. That name is the upstream
crate. `pkg/brew/ripgrep-bin.rb` and `ci/sha256-releases` still describe
upstream tarballs. This workflow does not update them.
