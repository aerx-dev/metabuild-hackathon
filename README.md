
# MetaBUILD Hackathon
Our contribution to the [NEAR MetaBUILD](https://metabuild.devpost.com) hackathon. This includes both front and backend.

## Badges

Add badges from somewhere like: [shields.io](https://shields.io/)

![AppVeyor](https://img.shields.io/appveyor/build/AERX-dev/MetaBuildHackathon?style=plastic)

[![MIT License](https://img.shields.io/apm/l/atomic-design-ui.svg?)](https://github.com/tterb/atomic-design-ui/blob/master/LICENSEs)


## Installation

1. Install ```near-cli```
1. Install ```cargo install wasm-pack``` to build the WASM file.
2. Write smart contract in Rust.
3. Compile with ```bash ./build.sh <folder>``` folder for now is [`contract/nft`](./contracts/nft/).
4. Deploy on dev-near ```near dev-deploy --wasmFile res/<contract>.wasm```


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

Start the server

```bash
  npm run start
```


## Usage/Examples

```javascript
import Component from 'my-project'

function App() {
  return <Component />
}
```


## Features

- Light/dark mode toggle
- Live previews
- Fullscreen mode
- Cross platform


## Tech Stack

**Client:** React, Redux, TailwindCSS

**Server:** Node, Express


## Authors

- [@3lLobo](https://www.github.com/3lLobo)
- [@Kondwani7](https://github.com/Kondwani7)


## Acknowledgements

