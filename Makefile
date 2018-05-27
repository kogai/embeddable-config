.PHONY: test
test:
	cargo test --doc --features=serde_json
	cargo test --lib --features=serde_yaml
	cargo test --lib --features=toml
