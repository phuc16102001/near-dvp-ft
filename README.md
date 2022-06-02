# DVP Fungible Token

## What is Fungible Token (FT)

Fungible tokens (shorten as FT) are digital assets which:
- Interchangable
- Divisible
- Non-unique

Usually, FT has metadata such as: name, symbol, icon, decimals, etc. However, many people may misunderstand that coins and tokens are same, but they aren't. Regarding coins, they have their own block-chain technology behind (e.g. Bitcoin, ETH, NEAR). On the other hand, tokens are created by using smart contract on those chain (e.g. Dai, wNEAR, USDT). You can check whether they are on the [Coin marketcap](https://coinmarketcap.com/).

These FTs are frequently used for representing membership, trading, etc. Furthermore, tokens are built using the shared standard like ERC20, NEP141. Another explaination is that if you think block-chain is your country, coins will be your native currency, while tokens are business stocks.

This repository is a smart contract for creating a token called DVP (Do Vuong Phuc). It based on the [NEP141](https://nomicon.io/Standards/Tokens/FungibleToken/Core) standard of NEAR Protocol

## Build and deploy contract

To build the source code, you just need to run the script `build.sh`:
```bash
./build.sh
```

After that, you can deploy the built file `out/dvp-ft.wasm` by yourself:
```bash
near dev-deploy out/dvp-ft.wasm
```

## Usage

After deploying, the NEAR CLI will create a `neardev` folder which store the contract name:
```bash
source neardev/dev-account.env
```

Now, you can initialize the contract by specifying the `owner_id` and `total_supply`:
```bash
near call $CONTRACT_NAME new_default_meta '{"owner_id": "phuc16102001.testnet", "total_supply": "1000"}' --accountId phuc16102001.testnet
```

However, bare in mind that once you have initialized, you cannot undo again. 

## Operation - Fungible token
### Get total supply

```bash
near view $CONTRACT_NAME ft_total_supply
```
### Get metadata

```bash
near view $CONTRACT_NAME ft_metadata
```

### Get balance

```bash
near view $CONTRACT_NAME ft_balance_of '{"account_id": "phuc16102001.testnet"}'
```

### Transfer

```bash
near call $CONTRACT_NAME ft_transfer '{"receiver_id": "thanhhoang4869.testnet", "amount": "3", "memo": "Invest tokens"}' --accountId phuc16102001.testnet --depositYocto 1
```

### Transfer and callback

```bash
near call $CONTRACT_NAME ft_transfer_call '{"receiver_id": "faucet.phuc16102001.testnet", "amount": "100", "msg": "faucet-increase"}' --accountId phuc16102001.testnet --depositYocto 1
```

This method usually used to do the cross-contract call. For instance, the example above called for increasing faucet pool tokens. After the transfer occuring, the contract receiving contract's `ft_on_transfer` will be called. If the receiver did not implement that, the `ft_resolve_transfer` MUST be implemented in order to roll-back

## Operation - Storage staking

These operations are for implementing application. Because of allocating account require an amount of money, so the user must pay it to registry. This schema was based-on the [NEP145](https://nomicon.io/Standards/StorageManagement) standard.

### Verify storage staking
```bash
near view $CONTRACT_NAME storage_balance_of '{"account_id": "phuc16102001.testnet"}'
```

### Registry
```bash
near call $CONTRACT_NAME storage_deposit '{"account_id":"thanhhoang4869.testnet", "registration_only": true}' --accountId thanhhoang4869.testnet --amount 1
```

In this method, you need to stake a small amount of NEAR in order to allocate storage. By default, I staked 1 NEAR and the method will automatically refund the unused coin since using `registration_only` parameter.


## Data structure

The DVP fungible token uses the NEP141 standard of NEAR Protocol whose structure likes:

```rs
pub struct FungibleToken {
    pub accounts: LookupMap<AccountId, Balance>,
    pub total_supply: Balance,
    pub account_storage_usage: StorageUsage,
}
```

And there are meta data related to the token:

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

The icon field is a string which can be either Base64 image, or XML-SVG image.

## Interface

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

Finally, because of using storage staking, indeed, it must implement the `Storage management` trait:

```rs
pub trait StorageManagement {
    fn storage_deposit;
    fn storage_withdraw;
    fn storage_unregister;
    fn storage_balance_bounds;
    fn storage_balance_of;
}
```
