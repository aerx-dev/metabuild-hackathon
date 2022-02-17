#!bin/bash

source .env
echo $NFT

near deploy --wasmFile res/aerx_nft.wasm --accountId $NFT

near call $FT new_default_meta '{"owner_id":"'$NFT'"}' --account_id $ID

near call $NFT nft_mint '{"receiver_id":"'$MAMI'", "token_metadata":{    <much metadata>    }}' --accountId $NFT --deposit 0.1

near call $NFT nft_transfer '{"receiver_id": "'$NFT'", "token_id":"1"}' --accountId $ID --depositYocto 1

near call $NFT nft_total_supply '{}' --accountId $ID
