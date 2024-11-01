WBG="wasm-bindgen 0.2.95"
if [ "$(wasm-bindgen -V)" != "$WBG" ]; then
	echo "Incorrect wasm-bindgen version: '$(wasm-bindgen -V)' != '$WBG'"
	exit 1
fi

RUSTFLAGS='-Zlocation-detail=none -Zwasm-c-abi=spec' cargo build --target wasm32-unknown-unknown -Z build-std=panic_abort,std -Z build-std-features=panic_immediate_abort,optimize_for_size --release
wasm-bindgen --target web --out-dir out/ target/wasm32-unknown-unknown/release/wasm_test.wasm


