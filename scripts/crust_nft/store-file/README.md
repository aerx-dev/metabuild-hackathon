# Store file sample

The base sample for storing file by using Crust

Run local IPFS node:

```bash
ipfs daemon
```

## Build

```shell
yarn & yarn build
```

## Run

```shell
yarn start
```

In `index.ts` go over the steps and comment/uncomment your preferred IPFS usage, local or through a gateway.
In `NearSign` you can define where you grab the keys, if form a local file or from a browser window.
All environment variables are saved in a dorenv file.
