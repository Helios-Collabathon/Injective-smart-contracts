#[cfg(test)]
mod integration_tests {
    use cosmwasm_std::{Addr, Empty};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};
    use inj_interchain_persona::contract::{execute, instantiate, query};
    use inj_interchain_persona::msg::InstantiateMsg;

    const OWNER: &str = "admin0001";

    fn mock_app() -> App {
        App::default()
    }

    pub fn contract_interchain_persona() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }

    #[track_caller]
    fn instantiate_interchain_persona(app: &mut App) -> Addr {
        let code_id = app.store_code(contract_interchain_persona());
        let msg = InstantiateMsg {};
        app.instantiate_contract(code_id, Addr::unchecked(OWNER), &msg, &[], "test", None)
            .unwrap()
    }

    mod personas_tests {
        use super::*;
        use cosmwasm_std::Attribute;
        use inj_interchain_persona::chain::Chain;
        use inj_interchain_persona::msg::ExecuteMsg;
        use inj_interchain_persona::wallet::Wallet;

        #[test]
        fn add_wallet_to_persona() {
            let mut app = mock_app();

            let persona_addr = instantiate_interchain_persona(&mut app);

            let add_wallet_msg = ExecuteMsg::AddWallet {
                wallet: Wallet::new(
                    Chain::Injective,
                    "cosmwasm1mzdhwvvh22wrt07w59wxyd58822qavwkx5lcej7aqfkpqqlhaqfsgn6fq2"
                        .to_string(),
                ),
            };

            let app_response = app
                .execute_contract(Addr::unchecked(OWNER), persona_addr, &add_wallet_msg, &[])
                .unwrap();

            assert_eq!(
                app_response.custom_attrs(1),
                [
                    Attribute {
                        key: "action".to_string(),
                        value: "add_persona".to_string()
                    },
                    Attribute {
                        key: "persona".to_string(),
                        value: r#"{
  "linked_wallets": [
    {
      "chain": "injective",
      "address": "cosmwasm1mzdhwvvh22wrt07w59wxyd58822qavwkx5lcej7aqfkpqqlhaqfsgn6fq2"
    }
  ]
}"#
                        .to_string()
                    }
                ]
            );
        }
    }
}
