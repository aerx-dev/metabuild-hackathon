# !bin/bash

# export ID=3llobo.testnet

# near dev-deploy --wasmFile res/aex_token.wasm

# export FT=dev-1644927834577-96225193029183

# near call $FT new_default_meta '{"owner_id":"3llobo.testnet", "total_supply":"1000000000"}' --account_id $ID

# near call $FT storage_deposit '' --accountId $FT --amount .1

# near call $FT ft_transfer '{"receiver_id":"'$FT'", "amount":"111"}' --accountId $ID --depositYocto 1

# near call $FT  internal_withdraw '{"account_id":"ellobo.testnet", "amount":111}' --accountId $FT --depositYocto 1

# near call $FT internal_register_account '{"account_id":"3llobo.testnet"}' --accountId $FT --depositYocto 1

# near view $FT  ft_total_supply '{}' --accountId $ID

# near call $FT ft_balance_of '{"account_id":"'$FT'"}' --accountId $ID 

# near state $FT
# near state $ID
