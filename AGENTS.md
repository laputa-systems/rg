# rg

Fork of ripgrep. The program is the `rg` binary. Windows is not supported.

## Commands

- Test: `cargo test --all` (see README, Running tests).
- Release build: `cargo build --release`.
- PCRE2: `cargo build --release --features pcre2`.

`rust-toolchain.toml` pins the compiler. The MSRV is 1.96 (`Cargo.toml`
`rust-version`). `profile.release` strips symbols, uses fat LTO, and aborts
on panic.

## Map

- Install and build: `README.md`
- Search behavior: `GUIDE.md`
- Flag questions: `FAQ.md`
- Publishing binaries: `RELEASE-CHECKLIST.md`
- Open tickets: `TODO.md`
- Security contact: [RIP-2](docs/rfc/RIP-2-security-contact.md)
- Help text and the man page: `crates/core/flags/doc/`
