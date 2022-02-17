
# MetaBUILD Hackathon
Our contribution to the [NEAR MetaBUILD](https://metabuild.devpost.com) hackathon. This includes, for now only the backend and some js scripts.

## Badges

Add badges from somewhere like: [shields.io](https://shields.io/)

![AppVeyor](https://img.shields.io/appveyor/build/AERX-dev/MetaBuildHackathon?style=plastic)

[![MIT License](https://img.shields.io/apm/l/atomic-design-ui.svg?)](https://github.com/tterb/atomic-design-ui/blob/master/LICENSEs)

## Smart Contracts

Smart contracts are stored in [`contracts`](./contracts/). Here's what you do with them:

1. Install ```near-cli```
1. Install ```cargo install wasm-pack``` to build the WASM file.
2. Write smart contract in Rust.
3. Compile with ```bash ./build.sh <folder>``` folder for now is [`contract/nft`](./contracts/nft/).
4. Deploy on dev-near ```near dev-deploy --wasmFile res/<contract>.wasm```

Every contract has its own readme.
Check the [`run.sh`](run.sh) file for a walk through from deployment till nft transfer.


### Decentralized Data Storage

IPFS is a gawd dam PROTOCOL, not a place you upload your files. You can run your own node and contribute to the network. More imporantly, you can pin your files so these will be available while you are online.



Testing:
```bash
cargo test -- --nocapture
# Test python scripts
pytest
```
    
## Run Locally

Clone the project

```bash
  git clone https://link-to-project
```

Go to the project directory

```bash
  cd my-project
```

Install dependencies

```bash
  npm install
```

<!-- Start the server

```bash
  npm run start
```
 -->


**Contracts:** Rust, Near-cli

**Interface:** JavaScript, Python


## Authors

- [@3lLobo](https://www.github.com/3lLobo)
- [@Kondwani7](https://github.com/Kondwani7)


## Acknowledgements

