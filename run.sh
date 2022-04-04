# !bin/bash

# innazh.testnet
# near CNFT-deploy --wasmFile res/aex_token.wasm 

export ID=$1
echo "Your NEAR ID is '$ID'. Double check!"
# export ID=3llobo.testnet
# export ID=test.near
export PNFT=aerx_profile.$ID
export CNFT=aerx_content.$ID
export FT=aex_token.$ID 
export PREMIE=1110000000000000000000000
export TOTSUP=1110000000000000000000000000000

near create-account $PNFT --masterAccount $ID
near create-account $FT --masterAccount $ID
near create-account $CNFT --masterAccount $ID

near state $ID
near state $FT
near state $PNFT
near state $CNFT

near deploy --wasmFile res/aex_token.wasm --accountId $FT
near deploy --wasmFile res/aerx_profile_nft.wasm --accountId $PNFT
near deploy --wasmFile res/aerx_content_nft.wasm --accountId $CNFT
near call $PNFT new_default_meta '{"owner_id":"'$PNFT'", "aerx_token_id":"'$FT'"}' --accountId $PNFT
near call $CNFT new_default_meta '{"owner_id":"'$CNFT'"}' --accountId $CNFT

near call $FT new_default_meta '{"owner_id":"'$FT'", "total_supply":"'$TOTSUP'", "nft_id": "'$PNFT'"}' --accountId $ID

near call $PNFT nft_mint '{"receiver_id":"'$ID'", "token_metadata":{    "title": "AERX Profile",    "description": "Wolf Profile account",    "media": "https://ipfs.io/ipfs/QmPKF4jtqCZPpD7x44fxFU5nRLKioY7uMzT6RKz2dVf62c/",  "media_hash": "mjc6wYlkNHgRJ9RfblKEuPG9uvR+O8m09eRWNhc9rtQ"}}' --accountId $PNFT --amount 1 --gas=75000000000000 --depositYocto 9420000000000000000111

near call $PNFT get_owned_tokens '{"owner_id":"'$ID'"}' --accountId $ID

near call $FT ft_balance_of '{"account_id":"'$FT'"}' --accountId $ID 
near call $FT ft_balance_of '{"account_id":"'$ID'"}' --accountId $ID 

# near call $FT ft_transfer '{"receiver_id":"'$ID'", "amount":"'$PREMIE'"}' --accountId $FT --depositYocto 1

# near call $FT storage_deposit '' --accountId $FT --amount .1


# near call $FT  internal_withdraw '{"account_id":"ellobo.testnet", "amount":111}' --accountId $FT --depositYocto 1

# near call $FT internal_register_account '{"account_id":"3llobo.testnet"}' --accountId $FT --depositYocto 1

# near view $FT  ft_total_supply '{}' --accountId $ID

# near state $FT
# near state $ID


# near call cnft.3llobo.testnet nft_mint '{"receiver_id":"3llobo.testnet", "token_metadata":{    "title": "ChargeTest",    "description": "ChargeTest",    "media": "https://ipfs.io/ipfs/QmPKF4jtqCZPpD7x44fxFU5nRLKioY7uMzT6RKz2dVf62c/",  "media_hash": "mjc6wYlkNHgRJ9RfblKEuPG9uvR+O8m09eRWNhc9rtQ", "charge": "111"}}' --accountId cnft.3llobo.testnet --amount 1 --gas=75000000000000 --depositYocto 9420000000000000000111