install:
	cargo install basic-http-server
	git submodule update --init --recursive

run:
	EMCC_CFLAGS="-sMIN_WEBGL_VERSION=2 -sMIN_WEBGL_VERSION=2" cargo build --target=wasm32-unknown-emscripten
	cp ./target/wasm32-unknown-emscripten/debug/sokol_fetch_rs.wasm ./dist/sokol_fetch_rs.wasm
	cp ./target/wasm32-unknown-emscripten/debug/sokol-fetch-rs.js ./dist/sokol-fetch-rs.js
	basic-http-server .\\dist