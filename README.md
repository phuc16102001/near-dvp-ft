# DVP Fungible Token

## What is Fungible Token (FT)

## Build and deploy contract

## Usage

## Structure

The DVP fungible token uses the NEP141 standard of NEAR Protocol whose structure likes:

```rs
pub struct FungibleToken {
    pub accounts: LookupMap<AccountId, Balance>,
    pub total_supply: Balance,
    pub account_storage_usage: StorageUsage,
}
```

And there are meta data related to the Token:

```rs
pub struct FungibleTokenMetadata {
    pub spec: String,
    pub name: String,
    pub symbol: String,
    pub icon: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<Base64VecU8>,
    pub decimals: u8,
}
```

## Operations

With all the fungible token, they have the same schema of basic function. In NEP141, NEAR team created an interface called:

```rs
pub trait FungibleTokenCore {
    fn ft_transfer;
    fn ft_transfer_call;
    fn ft_total_supply;
    fn ft_balance_of;
}
```

Also, you can verify the meta data of the FT with its provider:

```rs
pub trait FungibleTokenMetadataProvider {
    fn ft_metadata;
}
```

### Transfer

```rs
```

### Transfer and call

```rs
```

### Get total supply

```rs
```

### Get balance

```rs
```