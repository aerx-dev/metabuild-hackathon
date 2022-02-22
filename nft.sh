#!bin/bash

alias local_near='NEAR_ENV="local" NEAR_CLI_LOCALNET_NETWORK_ID="localnet" NEAR_NODE_URL="http://127.0.0.1:51935" NEAR_CLI_LOCALNET_KEY_PATH="/home/wolf/.neartosis/2022-02-21T15.41.43/validator-key.json" NEAR_WALLET_URL="http://127.0.0.1:51958" NEAR_HELPER_URL="http://127.0.0.1:51941" NEAR_HELPER_ACCOUNT="test.near" NEAR_EXPLORER_URL="http://127.0.0.1:51953" near'

source .env
echo $NFT

local_near create-account ft.test.near --masterAccount $ID

# near deploy --wasmFile res/aerx_nft.wasm --accountId $NFT

# near call $FT new_default_meta '{"owner_id":"'$NFT'"}' --account_id $ID

# near call $NFT nft_mint '{"receiver_id":"'$MAMI'", "token_metadata":{    <much metadata>    }}' --accountId $NFT --deposit 0.1

# near call $NFT nft_transfer '{"receiver_id": "'$NFT'", "token_id":"1"}' --accountId $ID --depositYocto 1

# near call $NFT nft_total_supply '{}' --accountId $ID
