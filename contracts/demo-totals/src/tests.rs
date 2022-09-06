use cosmwasm_std::{coin, coins, Addr, Empty, StdResult, Uint64};
use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn demo_totals_template() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    )
    .with_reply(crate::contract::reply);
    Box::new(contract)
}

fn whitelist_template() -> Box<dyn Contract<Empty>> {
    let whitelist = ContractWrapper::new(
        cw1_whitelist::contract::execute,
        cw1_whitelist::contract::instantiate,
        cw1_whitelist::contract::query,
    );
    Box::new(whitelist)
}

fn dinner_template() -> Box<dyn Contract<Empty>> {
    let dinner = ContractWrapper::new(
        cross_contract_dinner::contract::execute,
        cross_contract_dinner::contract::instantiate,
        cross_contract_dinner::contract::query,
    );
    Box::new(dinner)
}

const ADMIN: &str = "cosmos1sjllsnramtg3ewxqwwrwjxfgc4n4ef9u0tvx7u";
const ALICE: &str = "cosmos1a7uhnpqthunr2rzj0ww0hwurpn42wyun6c5puz";
const BOB: &str = "cosmos1wze8mn5nsgl9qrgazq6a92fvh7m5e6psjcx2du";
const NATIVE_DENOM: &str = "atom";

fn mock_app() -> App {
    AppBuilder::new().build(|router, _, storage| {
        let accounts: Vec<(u128, String)> = vec![
            (10_000_000, ADMIN.to_string()),
            (10_000_000, ALICE.to_string()),
            (10_000_000, BOB.to_string()),
        ];
        for (amt, address) in accounts.iter() {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(address),
                    vec![coin(amt.clone(), NATIVE_DENOM.to_string())],
                )
                .unwrap();
        }
    })
}

fn proper_instantiate() -> (App, Addr, Addr) {
    let mut app = mock_app();
    let owner_addr = Addr::unchecked(ADMIN);

    // Instantiate whitelist, put ALICE in it
    let whitelist_id = app.store_code(whitelist_template());
    let msg = cw1_whitelist::msg::InstantiateMsg {
        admins: vec![ALICE.to_string()],
        mutable: true,
    };
    let whitelist_addr = app
        .instantiate_contract(
            whitelist_id,
            owner_addr.clone(),
            &msg,
            &[],
            "Whitelist",
            None,
        )
        .unwrap();

    // Instantiate dinner
    let dinner_id = app.store_code(dinner_template());
    let msg = cross_contract_dinner::msg::InstantiateMsg {
        owner: None,
        denom: NATIVE_DENOM.to_string(),
        scholarship_address: whitelist_addr.to_string(),
    };
    let dinner_addr = app
        .instantiate_contract(dinner_id, owner_addr.clone(), &msg, &[], "Dinner", None)
        .unwrap();

    // Instantiate demo-total
    let demo_totals_id = app.store_code(demo_totals_template());
    let msg = InstantiateMsg {
        owner: None,
        denom: NATIVE_DENOM.to_string(),
    };
    let contract_addr = app
        .instantiate_contract(
            demo_totals_id,
            owner_addr,
            &msg,
            &coins(2_000_000, NATIVE_DENOM),
            "Demo-totals",
            None,
        )
        .unwrap();
    (app, dinner_addr, contract_addr)
}

fn query_number_of_registrants(app: &App, contract: Addr) -> u64 {
    let amount: Uint64 = app
        .wrap()
        .query_wasm_smart(contract, &QueryMsg::RegistrantNumber {})
        .unwrap();
    amount.u64()
}

fn query_all_registrants(app: &App, dinner: Addr) -> Vec<Addr> {
    app.wrap()
        .query_wasm_smart(
            dinner,
            &cross_contract_dinner::msg::QueryMsg::GetAllRegistrants {},
        )
        .unwrap()
}

fn query_is_registered(app: &App, dinner: Addr, address: String) -> bool {
    app.wrap()
        .query_wasm_smart(
            dinner,
            &cross_contract_dinner::msg::QueryMsg::IsAddressRegistered { address },
        )
        .unwrap()
}

#[test]
fn test_with_payment_success() -> StdResult<()> {
    let (mut app, dinner, contract) = proper_instantiate();

    // Check number of registrants
    assert_eq!(query_number_of_registrants(&app, contract.clone()), 0);
    // Check state of dinner contract
    assert_eq!(query_all_registrants(&app, dinner.clone()).len(), 0);
    assert_eq!(
        query_is_registered(&app, dinner.clone(), ALICE.to_string()),
        false
    );
    assert_eq!(
        query_is_registered(&app, dinner.clone(), BOB.to_string()),
        false
    );

    // Can register with 10000 NATIVE_DENOM
    let msg_register_with_payment = ExecuteMsg::RegisterWithPayment {
        dinner_contract: dinner.to_string(),
    };
    let res = app.execute_contract(
        Addr::unchecked(ALICE),
        contract.clone(),
        &msg_register_with_payment,
        &coins(10000, NATIVE_DENOM),
    );
    println!("{:#?}", res);
    assert!(res.is_ok());
    assert_eq!(query_number_of_registrants(&app, contract.clone()), 1);
    assert_eq!(query_all_registrants(&app, dinner.clone()).len(), 1);
    assert_eq!(
        query_is_registered(&app, dinner.clone(), ALICE.to_string()),
        true
    );
    assert_eq!(
        query_is_registered(&app, dinner.clone(), BOB.to_string()),
        false
    );

    // Can register with more than 10000 NATIVE_DENOM
    let res = app.execute_contract(
        Addr::unchecked(BOB),
        contract.clone(),
        &msg_register_with_payment,
        &coins(15000, NATIVE_DENOM),
    );
    println!("{:#?}", res);
    assert!(res.is_ok());
    // There are two registrants
    assert_eq!(query_number_of_registrants(&app, contract.clone()), 2);
    assert_eq!(query_all_registrants(&app, dinner.clone()).len(), 2);
    // Both are registered
    assert_eq!(
        query_is_registered(&app, dinner.clone(), ALICE.to_string()),
        true
    );
    assert_eq!(
        query_is_registered(&app, dinner.clone(), BOB.to_string()),
        true
    );

    Ok(())
}

#[test]
fn test_with_payment_failure() -> StdResult<()> {
    let (mut app, dinner, contract) = proper_instantiate();

    // Can't register with less than 10000 NATIVE_DENOM
    let msg_register_with_payment = ExecuteMsg::RegisterWithPayment {
        dinner_contract: dinner.to_string(),
    };
    let res = app.execute_contract(
        Addr::unchecked(ALICE),
        contract.clone(),
        &msg_register_with_payment,
        &coins(9999, NATIVE_DENOM),
    );
    println!("{:#?}", res);
    assert!(res.is_err());
    // Error if provided a wrong dinner address
    let msg_wrong_address = ExecuteMsg::RegisterWithPayment {
        dinner_contract: contract.to_string(),
    };
    let res = app.execute_contract(
        Addr::unchecked(ALICE),
        contract.clone(),
        &msg_wrong_address,
        &coins(10000, NATIVE_DENOM),
    );
    println!("{:#?}", res);
    assert!(res.is_err());
    // Number of registrants didn't change
    assert_eq!(query_number_of_registrants(&app, contract.clone()), 0);
    // Check that state of dinner contract didn't change
    assert_eq!(query_all_registrants(&app, dinner.clone()).len(), 0);
    assert_eq!(
        query_is_registered(&app, dinner.clone(), ALICE.to_string()),
        false
    );

    // Register ALICE and check that she can't register again
    app.execute_contract(
        Addr::unchecked(ALICE),
        contract.clone(),
        &msg_register_with_payment,
        &coins(10000, NATIVE_DENOM),
    )
    .unwrap();
    let res = app.execute_contract(
        Addr::unchecked(ALICE),
        contract.clone(),
        &msg_register_with_payment,
        &coins(10000, NATIVE_DENOM),
    );
    println!("{:#?}", res);
    assert!(res.is_err());

    Ok(())
}

#[test]
fn test_with_scholarship_success() -> StdResult<()> {
    let (mut app, dinner, contract) = proper_instantiate();

    // ALICE can register because she is in the whitelist
    let msg_register_with_payment = ExecuteMsg::RegisterWithScholarship {
        dinner_contract: dinner.to_string(),
    };
    let res = app.execute_contract(
        Addr::unchecked(ALICE),
        contract.clone(),
        &msg_register_with_payment,
        &vec![],
    );
    println!("{:#?}", res);
    assert!(res.is_ok());
    assert_eq!(query_number_of_registrants(&app, contract.clone()), 1);
    // Check that ALICE is registered
    assert_eq!(query_all_registrants(&app, dinner.clone()).len(), 1);
    assert_eq!(
        query_is_registered(&app, dinner.clone(), ALICE.to_string()),
        true
    );

    Ok(())
}

#[test]
fn test_with_scholarship_failure() -> StdResult<()> {
    let (mut app, dinner, contract) = proper_instantiate();

    // BOB can't register because he is not in the whitelist
    let msg_register_with_payment = ExecuteMsg::RegisterWithScholarship {
        dinner_contract: dinner.to_string(),
    };
    let res = app.execute_contract(
        Addr::unchecked(BOB),
        contract.clone(),
        &msg_register_with_payment,
        &vec![],
    );
    println!("{:#?}", res);
    assert!(res.is_err());
    // Number of registrants didn't change
    assert_eq!(query_number_of_registrants(&app, contract.clone()), 0);
    // Check that state of dinner contract didn't change
    assert_eq!(query_all_registrants(&app, dinner.clone()).len(), 0);
    assert_eq!(
        query_is_registered(&app, dinner.clone(), BOB.to_string()),
        false
    );

    // Register ALICE and check that she can't register twice
    app.execute_contract(
        Addr::unchecked(ALICE),
        contract.clone(),
        &msg_register_with_payment,
        &vec![],
    )
    .unwrap();
    let res = app.execute_contract(
        Addr::unchecked(ALICE),
        contract.clone(),
        &msg_register_with_payment,
        &vec![],
    );
    println!("{:#?}", res);
    assert!(res.is_err());

    Ok(())
}
