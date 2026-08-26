# SEP-0041 Token
Exposes the interface of the SEP-0041 Token alongside a mock token contract.

SEP-0041 Definition: https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0041.md

## Safety
This is **experimental software** and is provided on an "as is" and "as available" basis.

We do **not give any warranties** and **will not be liable for any loss** incurred through any use of this codebase.

## Getting Started

Add the package to your `Cargo.toml`:

```toml
[dependencies]
sep-41-token = "<desired version>"
```

You can optionally include the `testutils` feature in your `dev-dependencies` to deploy a mock version of the `sep-41-token` for testing:

```toml
[dev_dependencies]
sep-41-token = { version = "<desired version>", features = ["testutils"] }
```

### Client and Traits
This package exposes the Protocol 27 token interfaces and clients.

* `Token` and `TokenClient` re-export the Protocol 27 SEP-41 interface and client from `soroban-sdk`.
* `StellarAssetExtension` and `StellarAssetClient` re-export the Protocol 27 Stellar Asset Contract interface and client from `soroban-sdk`.

### Mock Token
This package exposes an example Soroban token implementation of the `SEP-0041` standard that can be used to test protocol interactions with Soroban tokens. This is important to test as interacting with Soroban tokens has a much larger cost impact than interacting with the Stellar Asset Contract.

A WASM version of the contract can be deployed as follows:
```rust
use sep_41_token::testutils::{MockTokenClient, MockTokenWASM};
use soroban_sdk::{testutils::Address as _, Address, Env, String};

let env = Env::default();

let admin = Address::generate(&env);
let token_id = env.register(MockTokenWASM, ());
let token_client = MockTokenClient::new(&env, &token_id);
token_client.initialize(
    &admin,
    &7,
    &String::from_str(&env, "Name"),
    &String::from_str(&env, "Symbol"),
);
```
