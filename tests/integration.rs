#[cfg(test)]
mod integration_tests {
    use cosmwasm_std::{Addr, Empty};
    use cw_multi_test::{App, AppResponse, Contract, ContractWrapper, Executor};
    use inj_interchain_persona::chain::Chain;
    use inj_interchain_persona::contract::{execute, instantiate};
    use inj_interchain_persona::msg::{ExecuteMsg, InstantiateMsg};
    use inj_interchain_persona::query::query;
    use inj_interchain_persona::wallet::Wallet;

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

    fn create_persona() -> (App, AppResponse, Addr) {
        let mut app = mock_app();

        let persona_addr = instantiate_interchain_persona(&mut app);

        let add_wallet_msg = ExecuteMsg::AddWallet {
            wallet: Wallet::new(
                Chain::Injective,
                "cosmwasm1mzdhwvvh22wrt07w59wxyd58822qavwkx5lcej7aqfkpqqlhaqfsgn6fq2".to_string(),
            ),
        };

        let app_response = app
            .execute_contract(
                Addr::unchecked(OWNER),
                persona_addr.clone(),
                &add_wallet_msg,
                &[],
            )
            .unwrap();

        return (app, app_response, persona_addr);
    }

    mod personas_tests {
        use super::*;
        use inj_interchain_persona::chain::Chain;
        use inj_interchain_persona::msg::{ExecuteMsg, QueryMsg};
        use inj_interchain_persona::persona::Persona;
        use inj_interchain_persona::wallet::Wallet;

        #[test]
        fn add_wallet_to_persona() {
            let (_app, app_response, _persona_address) = create_persona();

            let action_attr = app_response.custom_attrs(1)[0].clone();
            let persona_attr = app_response.custom_attrs(1)[1].clone();

            let expected_persona = Persona::new(vec![Wallet::new(
                Chain::Injective,
                "cosmwasm1mzdhwvvh22wrt07w59wxyd58822qavwkx5lcej7aqfkpqqlhaqfsgn6fq2".to_string(),
            )]);

            assert_eq!(action_attr.key, "action");
            assert_eq!(action_attr.value, "add_persona");

            assert_eq!(persona_attr.key, "persona");
            assert_eq!(
                persona_attr.value.clone(),
                serde_json::to_string_pretty(&expected_persona).unwrap()
            )
        }

        #[test]
        fn remove_wallet_from_persona() {
            let mut app = mock_app();

            let persona_addr = instantiate_interchain_persona(&mut app);

            let add_wallet_msg = ExecuteMsg::AddWallet {
                wallet: Wallet::new(
                    Chain::Injective,
                    "cosmwasm1mzdhwvvh22wrt07w59wxyd58822qavwkx5lcej7aqfkpqqlhaqfsgn6fq2"
                        .to_string(),
                ),
            };

            app.execute_contract(
                Addr::unchecked(OWNER),
                persona_addr.clone(),
                &add_wallet_msg,
                &[],
            )
            .unwrap();

            let remove_wallet_msg = ExecuteMsg::RemoveWallet {
                wallet: Wallet::new(
                    Chain::Injective,
                    "cosmwasm1mzdhwvvh22wrt07w59wxyd58822qavwkx5lcej7aqfkpqqlhaqfsgn6fq2"
                        .to_string(),
                ),
            };

            let remove_app_response = app
                .execute_contract(
                    Addr::unchecked(OWNER),
                    persona_addr.clone(),
                    &remove_wallet_msg,
                    &[],
                )
                .unwrap();

            let action_attr = remove_app_response.custom_attrs(1)[0].clone();
            let persona_attr = remove_app_response.custom_attrs(1)[1].clone();

            let expected_persona = Persona::new(vec![]);

            assert_eq!(action_attr.key, "action");
            assert_eq!(action_attr.value, "remove_persona");

            assert_eq!(persona_attr.key, "persona");
            assert_eq!(
                persona_attr.value.clone(),
                serde_json::to_string_pretty(&expected_persona).unwrap()
            );
        }

        #[test]
        fn query_persona_by_inj_wallet() {
            let (app, _app_response, persona_address) = create_persona();

            let persona: Persona = app
                .wrap()
                .query_wasm_smart(
                    persona_address,
                    &(QueryMsg::GetPersona {
                        address: Addr::unchecked(OWNER),
                    }),
                )
                .unwrap();

            assert!(persona.get_linked_wallets().len() > 0);
        }

        #[test]
        fn query_empty_persona_by_inj_wallet() {
            let (app, _app_response, persona_address) = create_persona();

            let persona: Persona = app
                .wrap()
                .query_wasm_smart(
                    persona_address,
                    &(QueryMsg::GetPersona {
                        address: Addr::unchecked("blabla"),
                    }),
                )
                .unwrap();

            assert!(persona.get_linked_wallets().len() == 0);
        }

        #[test]
        fn query_persona_by_linked_wallet() {
            let (app, _app_response, persona_address) = create_persona();

            let wallet = Wallet::new(
                Chain::Injective,
                "cosmwasm1mzdhwvvh22wrt07w59wxyd58822qavwkx5lcej7aqfkpqqlhaqfsgn6fq2".to_string(),
            );

            let personas: Vec<Persona> = app
                .wrap()
                .query_wasm_smart(
                    persona_address,
                    &(QueryMsg::GetPersonaFromLinkedWallet { wallet }),
                )
                .unwrap();

            assert!(personas.len() == 1);
        }
    }
}
