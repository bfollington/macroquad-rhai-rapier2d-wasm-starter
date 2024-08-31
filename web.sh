cargo build --target wasm32-unknown-unknown
cp index.html target/wasm32-unknown-unknown/debug
cp -r assets target/wasm32-unknown-unknown/debug
cp -r script target/wasm32-unknown-unknown/debug
cd target/wasm32-unknown-unknown/debug
npx serve