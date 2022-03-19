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
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::ValidAccountId;
use near_sdk::{
    env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue,
};
mod counter;
use counter::{Counter};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    nft_count: Counter
}

const MATE_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath fill='%23C4A2FC' d='M269.9 130.8c-20.5-42.4-51.3-42.3-71.7 0-8.6 18.2-13.2 38.1-13.6 58.2 0 23.7 16.9 44.1 40.1 48.6-6.1 18-18 33.7-40.8 37-2.5.4-4.2 2.6-3.8 5.1s2.6 4.2 5.1 3.8c23.9-3.4 40.5-18.9 48.8-45 5.9-20.7 7.5-42.3 4.6-63.6-.4-2.5 1.3-4.7 3.8-5.1 2.5-.4 4.7 1.3 5.1 3.8 2.8 21.4 1.5 43.1-3.8 64 23.2-4.6 39.9-24.9 39.9-48.6-.5-20.1-5.1-40-13.7-58.2zM170.4 99c-23.9 3.4-40.5 18.9-48.8 45-5.8 20.7-7.3 42.3-4.6 63.6.4 2.5-1.3 4.7-3.8 5.1-2.5.4-4.7-1.3-5.1-3.8-2.8-21.4-1.5-43.1 3.8-64-23.1 4.6-39.9 25-39.9 48.6.4 20.1 5 40 13.6 58.2 20.5 42.4 51.3 42.3 71.7 0 8.6-18.2 13.2-38.1 13.6-58.2 0-23.7-16.9-44.1-40.1-48.6 6.1-18 18-33.7 40.8-37 2.5-.4 4.2-2.6 3.8-5.1s-2.6-4.1-5-3.8z'/%3E%3Cpath fill='%236D2ED3' d='M189 202.5h-45.5c25.8-12 44.8-35 51.7-62.6s1-56.8-16.1-79.6c-.9-1.1-2.2-1.8-3.6-1.8h-42.2l18.3-36.6 38.4-8.5c1.6-.3 2.9-1.5 3.3-3 .5-1.5.1-3.2-1-4.4-1.1-1.2-2.7-1.7-4.3-1.3l-40.5 9c-1.3.3-2.4 1.2-3 2.4l-21.3 42.5H31.5c-1.4 0-2.7.7-3.6 1.8-17.1 22.8-23 52-16.1 79.6s25.9 50.6 51.7 62.6H18c-7.5 0-13.5 6-13.5 13.5s6 13.5 13.5 13.5h171c7.5 0 13.5-6 13.5-13.5 0-7.6-6-13.6-13.5-13.6zM33.8 67.5h139.4c3 4.3 5.6 8.8 7.8 13.5H26c2.2-4.7 4.8-9.3 7.8-13.5zM18 117c0-9.2 1.5-18.3 4.4-27h162.3c2.9 8.7 4.4 17.8 4.4 27 0 1.4-.1 2.8-.2 4.2l-21.2 8.9-19.7-8.3c-1.1-.5-2.4-.5-3.5 0l-19.6 8.3-19.6-8.3c-1.1-.5-2.4-.5-3.5 0l-19.6 8.3-19.6-8.3c-1.1-.5-2.4-.5-3.5 0l-19.6 8.3-21.2-8.9c-.2-1.4-.3-2.8-.3-4.2zm1.3 14.4 18.3 7.7c1.1.5 2.4.5 3.5 0l19.6-8.3 19.6 8.3c1.1.5 2.4.5 3.5 0l19.6-8.3 19.6 8.3c1.1.5 2.4.5 3.5 0l19.6-8.3 19.7 8.3c1.1.5 2.4.5 3.5 0l18.3-7.7c-1.1 6.8-3.1 13.4-5.8 19.7l-14.2 6-19.7-8.3c-1.1-.5-2.4-.5-3.5 0l-19.6 8.3-19.6-8.3c-1.1-.5-2.4-.5-3.5 0l-19.6 8.3-19.6-8.3c-1.1-.5-2.4-.5-3.5 0l-19.6 8.3-14.2-6c-2.8-6.3-4.7-12.9-5.9-19.7zM32 163.8l5.6 2.4c1.1.5 2.4.5 3.5 0l19.6-8.3 19.6 8.3c1.1.5 2.4.5 3.5 0l19.6-8.3 19.6 8.3c1.1.5 2.4.5 3.5 0l19.6-8.3 19.7 8.3c1.1.5 2.4.5 3.5 0l5.6-2.4c-15.8 24.2-42.6 38.7-71.5 38.7-28.7 0-55.6-14.6-71.4-38.7zm157 56.7H18c-2.5 0-4.5-2-4.5-4.5s2-4.5 4.5-4.5h171c2.5 0 4.5 2 4.5 4.5s-2 4.5-4.5 4.5z'/%3E%3C/g%3E%3C/svg%3E";


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
    pub fn new_default_meta(owner_id: ValidAccountId) -> Self {
        let nftea_meta: NFTContractMetadata = NFTContractMetadata {
            spec: NFT_METADATA_SPEC.to_string(), // required, essentially a version like "nft-1.0.0"
            name: "Wolf NFT".to_string(), // required, ex. "Mochi Rising — Digital Edition" or "Metaverse 3"
            symbol: "NFTea".to_string(), // required, ex. "MOCHI"
            icon: Some(MATE_ICON.to_string()), // Data URL
            base_uri: Some("https://ipfs.io/ipfs/".to_string()), // Centralized gateway known to have reliable access to decentralized storage assets referenced by `reference` or `media` URLs
            reference: None, // URL to a JSON file with more info
            reference_hash: None, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
        };
        Self::new(
            owner_id,
            nftea_meta,
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
    pub fn new(owner_id: ValidAccountId, metadata: NFTContractMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        let val_zero: i8 = 0;
        let nft_counter = Counter::counter_init(val_zero);
        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            nft_count: nft_counter
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
    #[payable]
    pub fn nft_mint(
        &mut self,
        receiver_id: ValidAccountId,
        token_metadata: TokenMetadata,
    ) -> Token {
        // NFT count increments with +1 everytime a NFT is minted. The tokenID is therefore incremental and unique.
        self.nft_count.increment();
        let token_id = self.nft_count.get_num().to_string();
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
        let contract = Contract::new_default_meta(accounts(1).into());
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
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());

        let token_id = "0".to_string();
        let token = contract.nft_mint(token_id.clone(), accounts(0), sample_token_metadata());
        assert_eq!(token.token_id, token_id);
        assert_eq!(token.owner_id, accounts(0).to_string());
        assert_eq!(token.metadata.unwrap(), sample_token_metadata());
        assert_eq!(token.approved_account_ids.unwrap(), HashMap::new());
    }

    #[test]
    fn test_transfer() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());
        let token_id = "0".to_string();
        contract.nft_mint(token_id.clone(), accounts(0), sample_token_metadata());

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
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());
        let token_id = "0".to_string();
        contract.nft_mint(token_id.clone(), accounts(0), sample_token_metadata());

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
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());
        let token_id = "0".to_string();
        contract.nft_mint(token_id.clone(), accounts(0), sample_token_metadata());

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
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());
        let token_id = "0".to_string();
        contract.nft_mint(token_id.clone(), accounts(0), sample_token_metadata());

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
