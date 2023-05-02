town_demo:
	cargo build --bin town_demo --release
town_demo_run:
	cargo run --bin town_demo --release
wasm_gfx_demo:
	cargo build --target wasm32-unknown-unknown --lib --release
	mkdir -p www/
	wasm-opt -o www/mercurylib.wasm -Oz target/wasm32-unknown-unknown/release/mercurylib.wasm
	ls -lh www/mercurylib.wasm
wasm_gfx_demo_run: wasm_gfx_demo
	python3 -m http.server
