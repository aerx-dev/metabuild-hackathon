# Let's build our own NFTs on NEAR


1. Install ```near-cli```
1. Install ```cargo install wasm-pack``` to build the WASM file.
2. Write smart contract in Rust.
3. Compile with ```bash ./build.sh <folder>``` folder for now is nft.
4. Deploy on dev-near ```near dev-deploy --wasmFile res/<contract>.wasm```


Once deployed, initialize the NFT and mint it:
```bash
export ID=<your-id>
# Initialize the contract with you as owner
near call <contract-id> new_default_meta '{"owner_id":<your-id>}' --account_id $ID
# Mint the fist NFT
near call <contract-id> nft_mint '{<nft-metadata>}' --account_id $ID
# View the NFT metadata
near view $ID nft_token '{"token_id": <token-id>}'
```

Testing:
```
cargo test -- --nocapture
```