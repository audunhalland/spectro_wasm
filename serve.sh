cargo build --target=wasm32-unknown-emscripten --verbose
cp target/wasm32-unknown-emscripten/debug/deps/* www/
http-server www
