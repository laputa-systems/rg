.PHONY: lint test publish publish-dry-run

lint:
	cargo clippy --workspace --all-targets -- --deny warnings

test:
	cargo test --locked --workspace

LAPUTA_CRATES := -p laputa-globset -p laputa-ignore

publish:
	stuff publish cargo $(LAPUTA_CRATES)

publish-dry-run:
	stuff publish cargo $(LAPUTA_CRATES) --dry-run
