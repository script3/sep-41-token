//! Interface for the SEP-41 Token Standard.
//! https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0041.md

#![no_std]

#[cfg(any(test, feature = "testutils"))]
pub mod testutils;

pub use soroban_sdk::token::{
    StellarAssetClient, StellarAssetInterface as StellarAssetExtension, TokenClient,
    TokenInterface as Token,
};

use soroban_sdk::{contractevent, Address, Env, MuxedAddress};

#[contractevent(topics = ["approve"], data_format = "vec")]
struct Approve {
    #[topic]
    from: Address,
    #[topic]
    spender: Address,
    amount: i128,
    expiration_ledger: u32,
}

#[contractevent(topics = ["transfer"], data_format = "map")]
struct Transfer {
    #[topic]
    from: Address,
    #[topic]
    to: Address,
    to_muxed_id: Option<u64>,
    amount: i128,
}

#[contractevent(topics = ["transfer"], data_format = "single-value")]
struct TransferWithAmountOnly {
    #[topic]
    from: Address,
    #[topic]
    to: Address,
    amount: i128,
}

#[contractevent(topics = ["burn"], data_format = "single-value")]
struct Burn {
    #[topic]
    from: Address,
    amount: i128,
}

#[contractevent(topics = ["mint"], data_format = "single-value")]
struct MintWithAmountOnly {
    #[topic]
    to: Address,
    amount: i128,
}

/// Helpers for publishing SEP-41 token events.
pub struct TokenEvents;

impl TokenEvents {
    /// Emitted when an allowance is set.
    pub fn approve(
        env: &Env,
        from: Address,
        spender: Address,
        amount: i128,
        live_until_ledger: u32,
    ) {
        Approve {
            from,
            spender,
            amount,
            expiration_ledger: live_until_ledger,
        }
        .publish(env);
    }

    /// Emitted by `transfer`, including an optional muxed-address identifier.
    pub fn transfer(env: &Env, from: Address, to: MuxedAddress, amount: i128) {
        Transfer {
            from,
            to: to.address(),
            to_muxed_id: to.id(),
            amount,
        }
        .publish(env);
    }

    /// Emitted by `transfer_from`, whose SEP-41 interface accepts an `Address`.
    pub fn transfer_from(env: &Env, from: Address, to: Address, amount: i128) {
        TransferWithAmountOnly { from, to, amount }.publish(env);
    }

    /// Emitted when tokens are burned.
    pub fn burn(env: &Env, from: Address, amount: i128) {
        Burn { from, amount }.publish(env);
    }

    /// Emitted when tokens are minted.
    pub fn mint(env: &Env, to: Address, amount: i128) {
        MintWithAmountOnly { to, amount }.publish(env);
    }
}
