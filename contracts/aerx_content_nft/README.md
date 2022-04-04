# Let's build our own NFTs on NEAR


1. Install ```near-cli```
1. Install ```cargo install wasm-pack``` to build the WASM file.
2. Write smart contract in Rust.
3. Compile with ```bash ./build.sh <folder>``` folder for now is nft.
4. Deploy on dev-near ```near dev-deploy --wasmFile res/<contract>.wasm ```


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

## The Metadata

There are two metadatas. The metadata for the NFT contract and for each mined NFT.

Templates are [here](/res/meta/nft_contract_meta.rs) and for the token [here](/res/meta/nft_token_meta.rs)

The icon has to be url compatible. Toke me while to figure it out. Follow these steps:

1. Choose a niec svg icon. You can find inspiration [online](https://icons8.com/).
2. Compress it using [omgsvg](https://jakearchibald.github.io/svgomg/)
3. Read how to make [dataURLs](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URIs)!!! Yes, this might take half hour.
4. Follow those steps and URL-encode weird symbols and you should end up with something like this:
```url
data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E
```
If you copy past this into your browser, you should see the NEAR icon. Same should happen with your icon.


## SHA encrption

NEAR wants you to provide a hash for every media or refference link you add to your NFT.
Copy the link text and encrypt get the hash [here](https://approsto.com/sha-generator/)
