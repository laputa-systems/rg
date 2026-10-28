.PHONY: lint test

lint:
	cargo clippy --workspace --all-targets -- --deny warnings

test:
	cargo test --locked --workspace
