set -e

cargo build --target=wasm32-unknown-emscripten --verbose

cp target/wasm32-unknown-emscripten/debug/deps/*.js www/
cp target/wasm32-unknown-emscripten/debug/deps/*.wasm www/
cp target/wasm32-unknown-emscripten/debug/deps/*.wast www/

# Currently *.js.map triggers weird exceptions in browser.
# rm www/*.js.map

http-server www
