
# MetaBUILD Hackathon
Our contribution to the [NEAR MetaBUILD](https://metabuild.devpost.com) hackathon. This is the backend repo, it includesthe [smart-contracts](./contracts/) and an [interface](./interfaces/) for dezentralized data storage.

## Badges

<!-- Add badges from somewhere like: [shields.io](https://shields.io/) -->



![MIT License](https://img.shields.io/apm/l/atomic-design-ui.svg?style=for-the-badge)
![GitHub language count](https://img.shields.io/github/languages/count/AERX-dev/MetaBuildHackathon?style=for-the-badge&color=magenta)
![GitHub top language](https://img.shields.io/github/languages/top/AERX-dev/MetaBuildHackathon?style=for-the-badge&color=red)
![Lines of code](https://img.shields.io/tokei/lines/github/AERX-dev/MetaBuildHackathon?style=for-the-badge&color=orange)
![GitHub commit activity](https://img.shields.io/github/commit-activity/y/AERX-dev/MetaBuildHackathon?color=lightblue&style=for-the-badge)

## Smart Contracts

Smart contracts are stored in [`contracts`](./contracts/). Here's what you do with them:

1. Install ```cargo install wasm-pack``` to build the WASM file.
3. Compile all contracts and test with  ```bash ./build.sh``` =, the wasm contracts will end up in [res](./res/).
1. Install ```near-cli```
3. Login to near ```near login``` with you `<ID`
4. Deploy, init and run predefined function calls with:
```bash
bash ./run.sh <your-id>
```

Every contract has its own readme.
The [`run.sh`](run.sh) walks you through the basic functionalities, from deployment till nft minting.


## Decentralized Data Storage

[IPFS](https://ipfs.io/) is a gawd dam PROTOCOL, not a place you upload your files. Luckily we have [Crust](https://crust.network/), a hosting-incentivize layer. You can run your own node and contribute to the network or use the public gateway. If you run your own node, it makes sense to pin your files so these will be available while you are online.

To store your files, you'll need a Crust account, also a NEAR account but I suppose you already have one if you read till here. Crust offers a testnet, I highly recommend that to play around. Once you made your account, define the environment variables as told in the [instructions](./interfaces/crust_ipfs/README.md).

## Testing:
```bash
cargo test -- --nocapture
# Test python scripts
pytest
```

<!-- Start the server

```bash
  npm run start
```
 -->


**Contracts:** Rust, Near-cli

**Interface:** JavaScript, Python


## Authors

- @3lLobo
- @innazh
- @samullman
- @pashq1
- @Kondwani7

## Acknowledgements

I thank the trees, for giving us life 

# :seedling:
