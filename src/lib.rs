use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::U128;
use near_sdk::{env, log, near_bindgen, AccountId, Balance, PanicOnDefault, PromiseOrValue};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
}

const DATA_ICON: &str = "data:image/png;base64,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";

#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: AccountId, total_supply: U128) -> Self {
        Self::new(
            owner_id,
            total_supply,
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "DVP Token".to_string(),
                symbol: "DVP".to_string(),
                icon: Some(DATA_ICON.to_string()),
                reference: None,
                reference_hash: None,
                decimals: 0,
            },
        )
    }

    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(
        owner_id: AccountId,
        total_supply: U128,
        metadata: FungibleTokenMetadata,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        let mut this = Self {
            token: FungibleToken::new(b"a".to_vec()),
            metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
        };
        this.token.internal_register_account(&owner_id);
        this.token.internal_deposit(&owner_id, total_supply.into());
        near_contract_standards::fungible_token::events::FtMint {
            owner_id: &owner_id,
            amount: &total_supply,
            memo: Some("Initial tokens supply is minted"),
        }
        .emit();
        this
    }

    fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
        log!("Closed @{} with {}", account_id, balance);
    }

    fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
        log!("Account @{} burned {}", account_id, amount);
    }
}

near_contract_standards::impl_fungible_token_core!(Contract, token, on_tokens_burned);
near_contract_standards::impl_fungible_token_storage!(Contract, token, on_account_closed);

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, Balance};

    use super::*;

    const TOTAL_SUPPLY: Balance = 1_000_000_000_000_000;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new_default_meta(accounts(1).into(), TOTAL_SUPPLY.into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.ft_total_supply().0, TOTAL_SUPPLY);
        assert_eq!(contract.ft_balance_of(accounts(1)).0, TOTAL_SUPPLY);
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }

    #[test]
    fn test_transfer() {
        let mut context = get_context(accounts(2));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(2).into(), TOTAL_SUPPLY.into());
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(contract.storage_balance_bounds().min.into())
            .predecessor_account_id(accounts(1))
            .build());
        // Paying for account registration, aka storage deposit
        contract.storage_deposit(None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(2))
            .build());
        let transfer_amount = TOTAL_SUPPLY / 3;
        contract.ft_transfer(accounts(1), transfer_amount.into(), None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert_eq!(contract.ft_balance_of(accounts(2)).0, (TOTAL_SUPPLY - transfer_amount));
        assert_eq!(contract.ft_balance_of(accounts(1)).0, transfer_amount);
    }
}