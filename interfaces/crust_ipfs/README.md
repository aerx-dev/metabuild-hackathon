# Store file sample

Code example for storing file by using Crust.
Define you `.env` file here, in this directory.


If you want to store your data though a local IPFS node, start it up:

```bash
ipfs daemon
```


To connect your local node with Crust run:
```bash
ipfs config --json API.HTTPHeaders.Access-Control-Allow-Origin '["https://apps.crust.network", "http://localhost:3000", "http://127.0.0.1:5001", "https://webui.ipfs.io"]'
ipfs config --json API.HTTPHeaders.Access-Control-Allow-Methods '["PUT", "POST"]'
```

This will allow it to connect to the Crust wewbsite, where you can interact with the crust blockchain.



## Build

Same old, same old:
```shell
yarn & yarn build
```

## Run


```shell
yarn start
```

In `index.ts` go over the steps and comment/uncomment your preferred IPFS usage, local or through a gateway.
In `NearSign` you can define where you grab the keys, if form a local file or from a browser window.
All environment variables are saved in a dotenv file.

## Resources

Create a [Crust Wallet](https://wiki.crust.network/docs/en/crustAccount).
Compress and encrypt your data.
