#! /bin/sh
 
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk $*

