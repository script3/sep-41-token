mod wasm {
    soroban_sdk::contractimport!(file = "./src/testutils/mock_sep_41_token.wasm");
}

pub use wasm::{
    Client as MockTokenClient, Contract as MockToken, TokenError, WASM as MockTokenWASM,
};

#[cfg(test)]
mod tests {
    use super::{MockTokenClient, MockTokenWASM};
    use soroban_sdk::{testutils::Address as _, Address, Env, MuxedAddress, String};

    #[test]
    fn mock_wasm_runs_on_protocol_27() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let recipient = Address::generate(&env);
        let token_id = env.register(MockTokenWASM, ());
        let token = MockTokenClient::new(&env, &token_id);

        token.initialize(
            &admin,
            &7,
            &String::from_str(&env, "Name"),
            &String::from_str(&env, "Symbol"),
        );
        token.mint(&admin, &100);
        token.transfer(&admin, MuxedAddress::from(&recipient), &40);

        assert_eq!(token.balance(&admin), 60);
        assert_eq!(token.balance(&recipient), 40);
    }
}
