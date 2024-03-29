/*!
Non-Fungible Token implementation with JSON serialization.
NOTES:
  - The maximum balance value is limited by U128 (2**128 - 1).
  - JSON calls should pass U128 as a base-10 string. E.g. "100".
  - The contract optimizes the inner trie structure by hashing account IDs. It will prevent some
    abuse of deep tries. Shouldn't be an issue, once NEAR clients implement full hashing of keys.
  - The contract tracks the change in storage before and after the call. If the storage increases,
    the contract requires the caller of the contract to attach enough deposit to the function call
    to cover the storage cost.
    This is done to prevent a denial of service attack on the contract by taking all available storage.
    If the storage decreases, the contract will issue a refund for the cost of the released storage.
    The unused tokens from the attached deposit are also refunded, so it's safe to
    attach more deposit than required.
  - To prevent the deployed contract from being modified or deleted, it should not have any access
    keys on its account.
*/
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{Token, TokenId, refund_deposit};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::ValidAccountId;
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, Balance, BorshStorageKey, Gas, PanicOnDefault,
    Promise, PromiseOrValue, PromiseResult,
};

near_sdk::setup_alloc!();

#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn callback_premie(receiver_id: ValidAccountId, token_metadata: TokenMetadata) -> Token ;
    // fn callback_min_bal() -> u128;
    // fn callback_arg_macro(#[callback] val: u128) -> u128;
}

#[ext_contract(ext_aex_token)]
pub trait AEXToken {
    fn collect_premie(account_id: AccountId) -> bool;
    // fn get_min_balance() -> u128;
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    aerx_token_id: ValidAccountId,
}

const AERX_ICON_URL: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 -30 230 210'%3E%3Cg id='l' data-name='l'%3E%3Cpath fill='%23C4A2FC' d='M57,3.2c-15.4,2-25.4,4.7-35.8,9.4L16,15l5.7,11.5C25.2,33.5,28,38,29,38c.8,0,5.3-1.3,10-2.9C51.4,30.9,56.1,30,65,30c11.1,0,17,2.4,20.6,8.6,1.9,3.3,2.4,5.5,2.4,11.2,0,4.4-.5,7.4-1.2,7.8-.6.4-8.8,1.1-18.2,1.6-37,1.8-54.4,9.1-62.2,26.1-2.7,5.8-2.9,7-2.9,19.2,0,13,0,13,3.7,20.6,5.8,11.9,15.3,19.3,28,22,7,1.5,27.7.6,35.3-1.5,10.5-3,20.9-9.5,28.7-18l4.7-5.1,7,7.1c7.3,7.3,15.9,12.4,27.1,16.1,8,2.6,39.3,2.6,50-.1,13.1-3.2,19.1-5.7,20.1-8.3,1-2.7,1.3-24.6.3-25.6-.3-.3-2.9.3-5.8,1.5-13.4,5.3-28.3,8.1-41.6,7.6-9.6-.3-11.2-.6-17.2-3.6-10.2-5-16.5-14.4-17.5-26.3l-.6-5.9,4.4-.1c2.4-.1,22.9-.2,45.4-.3l41-.1-.1-15.6c0-23.3-4.2-36.3-15.6-48.4C191,9.9,180.1,4.9,163.3,3.1c-17.4-1.9-34,2.4-46.4,12.1-2,1.5-4.2,2.8-4.9,2.8-.7,0-2.8-1.4-4.7-3-4-3.6-11.9-7.7-18.2-9.6C84.2,4,67.8,1.9,64.5,2.2c-1.1.1-4.5.6-7.5,1zm109.5,27.7c8.9,4,15.3,14.8,15.5,25.8V60h-55v-3.9c0-11.8,8-22.8,19-26.2,4.9-1.5,16.3-.9,20.5,1zM87.8,88.8c.4,10.9-1.5,17.5-6.8,23.4-5.8,6.6-11.1,8.8-21.2,8.8-9.2,0-13.2-1.6-16.5-6.7-2.7-4.1-2.5-17.6.4-22.1,5.4-8.6,15.7-12.1,34.8-11.8l9,.1.3,8.3z'/%3E%3C/g%3E%3C/svg%3E";

pub const XCC_GAS: Gas = 20000000000000;
pub const NFT_YOCTODEPOSIT: u128 = 9070000000000000000000;

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract owned by `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: ValidAccountId, aerx_token_id: ValidAccountId) -> Self {
        let nftea_meta: NFTContractMetadata = NFTContractMetadata {
            spec: NFT_METADATA_SPEC.to_string(), // required, essentially a version like "nft-1.0.0"
            name: "AERX Profile NFT".to_string(), // required, ex. "Mochi Rising — Digital Edition" or "Metaverse 3"
            symbol: "AEXNFT".to_string(),  // required, ex. "MOCHI"
            icon: Some(AERX_ICON_URL.to_string()), // Data URL
            base_uri: Some("https://ipfs.io/ipfs/".to_string()), // Centralized gateway known to have reliable access to decentralized storage assets referenced by `reference` or `media` URLs
            reference: None,      // URL to a JSON file with more info
            reference_hash: None, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
        };
        Self::new(
            owner_id,
            nftea_meta,
            aerx_token_id,
            // NFTContractMetadata {
            //     spec: NFT_METADATA_SPEC.to_string(),
            //     name: "AERX non-fungible token".to_string(),
            //     symbol: "AERX".to_string(),
            //     icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
            //     base_uri: None,
            //     reference: None,
            //     reference_hash: None,
            // },
        )
    }

    #[init]
    pub fn new(
        owner_id: ValidAccountId,
        metadata: NFTContractMetadata,
        aerx_token_id: ValidAccountId,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();

        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            aerx_token_id: aerx_token_id,
        }
    }

    /// Mint a new token with ID=`token_id` belonging to `receiver_id`.
    ///
    /// Since this example implements metadata, it also requires per-token metadata to be provided
    /// in this call. `self.tokens.mint` will also require it to be Some, since
    /// `StorageKey::TokenMetadata` was provided at initialization.
    ///
    /// `self.tokens.mint` will enforce `predecessor_account_id` to equal the `owner_id` given in
    /// initialization call to `new`.
    
    pub fn get_owned_tokens(&mut self, owner_id: ValidAccountId) -> Vec<TokenId> {
        let set_nfts = match &self.tokens.tokens_per_owner {
            Some(s) => s.get(owner_id.as_ref()),
            None => env::panic(b"This ID owns nothing, and will be happy x)")
        };
        match set_nfts {
            Some(s) => s.to_vec(),
            None => Vec::new()
        }

    }

    #[payable]
    pub fn nft_mint(
        &mut self,
        receiver_id: ValidAccountId,
        token_metadata: TokenMetadata,
    ) {
        // Assert len tokens_per_owner[receiver_id] == 0
        let already_minted = match &self.tokens.tokens_per_owner {
            Some(a) => a.contains_key(&receiver_id.as_ref()),
            None => false,
        };
        assert!(already_minted == false, "You already minted a profile-NFT!");
        let amount: Balance = env::attached_deposit();
        assert!(amount > NFT_YOCTODEPOSIT, "Deposit for NFTStorage too low!");
        // // Collect premie
        self.xcc_premie(receiver_id, &amount, token_metadata);

    }
    
    #[payable]
    pub fn nft_update_profile(
        &mut self, 
        receiver_id: ValidAccountId,
        token_metadata: Option<TokenMetadata>,
    ) -> Token {
        let initial_storage_usage = env::storage_usage();
        // Delete old token metadata
        self.tokens.token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.remove(&receiver_id.to_string()));
        // Metadata extension: Save metadata, keep variable around to return later.
        // Note that check above already panicked if metadata extension in use but no metadata
        // provided to call.
        self.tokens.token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&receiver_id.to_string(), &token_metadata.as_ref().unwrap()));

            // Approval Management extension: return empty HashMap as part of Token
        let approved_account_ids =
            if self.tokens.approvals_by_id.is_some() { Some(HashMap::new()) } else { None };

        // Return any extra attached deposit not used for storage
        refund_deposit(env::storage_usage() - initial_storage_usage);

        Token { token_id: receiver_id.to_string(), owner_id: receiver_id.to_string(), metadata: token_metadata, approved_account_ids }
    }


    pub fn xcc_premie(&mut self, receiver_id: ValidAccountId, amount: &Balance, token_metadata: TokenMetadata) -> Promise {
        ext_aex_token::collect_premie(receiver_id.to_string(), &self.aerx_token_id, *amount, XCC_GAS)
            .then(ext_self::callback_premie(receiver_id, token_metadata, &env::current_account_id(), NFT_YOCTODEPOSIT, XCC_GAS))
    }

    #[private]#[payable]
    pub fn callback_premie(&mut self, receiver_id: ValidAccountId, token_metadata: TokenMetadata)  -> Token {
        assert!(env::promise_results_count() == 1, "ERR_TOO_MANY_RESULTS_PREMIE");
        let collected: bool = match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => env::panic(b"ERR_CALL_FAILED"),
            PromiseResult::Successful(_val) => {
                if let Ok(got_premie) = near_sdk::serde_json::from_slice::<bool>(&_val) {
                    got_premie
                } else {
                    env::panic(b"ERR_WRONG_VAL_RECEIVED")
                }
            }
        };
        assert!(collected, "Something went wrong fetching your premie!");
        // NFT is equal to the callerid
        let token_id = receiver_id.to_string();
        self.tokens.mint(token_id, receiver_id, Some(token_metadata))
    }

}

near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    use super::*;

    const MINT_STORAGE_COST: u128 = 5870000000000000000000;

    fn get_context(predecessor_account_id: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn sample_token_metadata() -> TokenMetadata {
        TokenMetadata {
            title: Some("Olympus Mons".into()),
            description: Some("The tallest mountain in the charted solar system".into()),
            media: None,
            media_hash: None,
            copies: Some(1u64),
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        }
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new_default_meta(accounts(1).into(), accounts(11).into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.nft_token("1".to_string()), None);
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }

    #[test]
    fn test_mint() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into(), accounts(11).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());

        let token_id = "0".to_string();
        let token = contract.nft_mint(accounts(0), sample_token_metadata());
        assert_eq!(token.token_id, token_id);
        assert_eq!(token.owner_id, accounts(0).to_string());
        assert_eq!(token.metadata.unwrap(), sample_token_metadata());
        assert_eq!(token.approved_account_ids.unwrap(), HashMap::new());
    }

    #[test]
    fn test_transfer() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into(), accounts(11).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());
        let token_id = "0".to_string();
        contract.nft_mint(accounts(0), sample_token_metadata());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_transfer(accounts(1), token_id.clone(), None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        if let Some(token) = contract.nft_token(token_id.clone()) {
            assert_eq!(token.token_id, token_id);
            assert_eq!(token.owner_id, accounts(1).to_string());
            assert_eq!(token.metadata.unwrap(), sample_token_metadata());
            assert_eq!(token.approved_account_ids.unwrap(), HashMap::new());
        } else {
            panic!("token not correctly created, or not found by nft_token");
        }
    }

    #[test]
    fn test_approve() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into(), accounts(11).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());
        let token_id = "0".to_string();
        contract.nft_mint(accounts(0), sample_token_metadata());

        // alice approves bob
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(150000000000000000000)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_approve(token_id.clone(), accounts(1), None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert!(contract.nft_is_approved(token_id.clone(), accounts(1), Some(1)));
    }

    #[test]
    fn test_revoke() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into(), accounts(11).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());
        let token_id = "0".to_string();
        contract.nft_mint(accounts(0), sample_token_metadata());

        // alice approves bob
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(150000000000000000000)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_approve(token_id.clone(), accounts(1), None);

        // alice revokes bob
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_revoke(token_id.clone(), accounts(1));
        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert!(!contract.nft_is_approved(token_id.clone(), accounts(1), None));
    }

    #[test]
    fn test_revoke_all() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into(), accounts(11).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());
        let token_id = "0".to_string();
        contract.nft_mint(accounts(0), sample_token_metadata());

        // alice approves bob
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(150000000000000000000)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_approve(token_id.clone(), accounts(1), None);

        // alice revokes bob
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_revoke_all(token_id.clone());
        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert!(!contract.nft_is_approved(token_id.clone(), accounts(1), Some(1)));
    }
}
