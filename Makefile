WORKDIR=$(shell pwd)

bench:
	rustup run nightly cargo bench --features bench
run:
	cargo run -- --debug --lint ./
run\:json:
	cargo run -- --lint --format json ./
build:
	cargo build --release
	sudo ln -s -f $(WORKDIR)/target/release/autocorrect /usr/local/bin/autocorrect
test:
	@cargo test --manifest-path autocorrect/Cargo.toml 
	@cargo test
test\:lint:
	@cargo run -q -- --debug --lint tests/fixtures/*.fixed.*
test\:bench:
	tests/bench.sh
test\:lint-json:
	tests/test_lint_json.sh
wasm:
	wasm-pack build --release --scope huacnlee -d $(WORKDIR)/pkg autocorrect
	wasm-opt -Os -o pkg/autocorrect_bg.wasm pkg/autocorrect_bg.wasm
wasm\:publish:
	make wasm
	cd pkg && npm publish
crate\:publish:
	cargo release --manifest-path autocorrect/Cargo.toml --config autocorrect/release.toml