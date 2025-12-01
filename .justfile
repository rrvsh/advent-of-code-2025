nice: fmt lint
lint: lint-rust
fmt: fmt-rust
run: run-rust

lint-rust:
  cargo clippy --manifest-path ./rust/Cargo.toml
fmt-rust:
  cargo fmt --manifest-path ./rust/Cargo.toml
run-rust:
  cargo run --manifest-path ./rust/Cargo.toml
