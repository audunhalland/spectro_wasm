# spectro_wasm
A WebAssembly test project

# Build dependencies
$ curl -sf -L https://static.rust-lang.org/rustup.sh | sh
$ rustup toolchain add nightly
$ rustup target add wasm32-unknown-emscripten --toolchain nightly

$ curl -vs https://s3.amazonaws.com/mozilla-games/emscripten/releases/emsdk-portable.tar.gz | tar -xv
$ cd emsdk-portable
$ ./emsdk update
$ ./emsdk install sdk-incoming-64bit
$ source ./emsdk_env.sh

# Build
npm install --global http-server
./build.py
