cd fitl-wasm
wasm-pack build --target web --release
cd ..
pnpm run wasminstall
pnpm run test