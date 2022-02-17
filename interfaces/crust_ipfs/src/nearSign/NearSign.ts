const { keyStores, connect } = require("near-api-js");
import { u8aToHex } from '@polkadot/util'
require('dotenv').config()

const path = require("path");
const homedir = require("os").homedir();


const ACCOUNT_ID = process.env.NEAR_ID;
const networkId = "testnet";
const CREDENTIALS_DIR = ".near-credentials";

const credentialsPath = path.join(homedir, CREDENTIALS_DIR);
const keyStore = new keyStores.UnencryptedFileSystemKeyStore(credentialsPath);


// Login
// const keyStore = new keyStores.BrowserLocalStorageKeyStore();
const config = {
    keyStore, // instance of BrowserLocalStorageKeyStore
    networkId: 'testnet',
    nodeUrl: 'https://rpc.testnet.near.org',
    walletUrl: 'https://wallet.testnet.near.org',
    helperUrl: 'https://helper.testnet.near.org',
    explorerUrl: 'https://explorer.testnet.near.org'
};


async function verifySignature() {
    const near = await connect(config)
    const keyPair = await keyStore.getKey(config.networkId, ACCOUNT_ID);
    const msg = Buffer.from("hi");

    const { signature } = keyPair.sign(msg);

    const isValid = keyPair.verify(msg, signature);

    console.log("Signature Valid?:", isValid);

    return isValid;
}

export async function signIpfsHeader() {
    console.log(networkId)
    console.log(ACCOUNT_ID)
    // const near = await connect(config)
    const keyPair = await keyStore.getKey(config.networkId, ACCOUNT_ID);
    // get address
    const addressRaw = keyPair.getPublicKey().toString();
    console.log(addressRaw)
    const address = addressRaw.substring(8);
    console.log(address)

    // get singature 
    const { signature } = keyPair.sign(Buffer.from(address));
    const sig = u8aToHex(signature).substring(2);

    const authHeaderRaw = `near-${address}:${sig}`;

    return authHeaderRaw;
}
