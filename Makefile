desktop_gfx_demo:
	cargo build --bin light_gfx_demo --release
desktop_gfx_demo_run:
	cargo run --bin light_gfx_demo --release
wasm_gfx_demo:
	cargo build --target wasm32-unknown-unknown --lib --release
	mkdir -p www/
	#wasm-opt -o www/mercurylib.wasm -Oz target/wasm32-unknown-unknown/release/mercurylib.wasm
	cp target/wasm32-unknown-unknown/release/mercurylib.wasm www/mercurylib.wasm
	ls -lh www/mercurylib.wasm
wasm_gfx_demo_run: wasm_gfx_demo
	python3 -m http.server
setup:
	rustup target add wasm32-unknown-unknown
