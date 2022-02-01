# Let's build our own NFTs on NEAR


1. Install ```near-cli```
1. Install ```cargo install wasm-pack``` to build the WASM file.
2. Write smart contract in Rust.
3. Compile with ```bash ./build.sh <folder>``` folder for now is nft.
4. Deploy on dev-near ```near dev-deploy --wasmFile res/<contract>.wasm```


Testing:
```
cargo test -- --nocapture
```