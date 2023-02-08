#![cfg(test)]
use serde::{de::DeserializeOwned, Serialize};
use cosmwasm_std::{
    Addr, Coin, Empty, from_binary, QueryRequest, to_binary, 
    StdError, Timestamp, Uint128, WasmQuery
};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cw_multi_test::{App, Contract, ContractWrapper, Executor};

use crate::contract::{instantiate as portal_instantiate, query as portal_query};
use crate::msg::{
    ExecuteMsg, InstantiateMsg, MintMsg, QueryMsg, 
};
use universe::species::{
    SapienceResponse, SapienceScale, Sapient, Species
};

use cw721::{NftInfoResponse};
use passport_token::{
    Extension, ExecuteMsg as Cw721ExecuteMsg, InstantiateMsg as Cw721InstantiateMsg,
    Metadata, QueryMsg as Cw721QueryMsg,
};

pub static DENOM: &str = "uport";   // Fractional representation of the PORT coin 
                                    // used as the native currency in our tests

// XXX TODO: 
// Refactor contract setup (store, instantiate for Portal and Passport contracts) 
// into a separate helper function, and remove redundant code

fn mock_app() -> App {
    App::default()
}
fn get_block_time(router: &mut App) -> u64 {
    router.block_info().time.seconds()
}
fn increment_block_time(router: &mut App, new_time: u64, height_incr: u64) {
    let mut curr = router.block_info();
    curr.height = curr.height + height_incr;
    curr.time = Timestamp::from_seconds(new_time);
    router.set_block(curr);
}
fn mint_native(app: &mut App, beneficiary: String, denom: String, amount: Uint128) {
    app.sudo(cw_multi_test::SudoMsg::Bank(
        cw_multi_test::BankSudo::Mint {
            to_address: beneficiary,
            amount: vec![Coin {
                denom: denom,
                amount: amount,
            }],
        },
    ))
    .unwrap();
}

pub fn query<M, T>(router: &mut App, target_contract: Addr, msg: M) -> Result<T, StdError>
where
    M: Serialize + DeserializeOwned,
    T: Serialize + DeserializeOwned,
{
    router.wrap().query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: target_contract.to_string(),
        msg: to_binary(&msg).unwrap(),
    }))
}

pub fn contract_portal() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    );
    Box::new(contract)
}

pub fn contract_cw721() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        passport_token::entry::execute,
        passport_token::entry::instantiate,
        passport_token::entry::query,
    );
    Box::new(contract)
}

fn create_cw721(
    router: &mut App, 
    minter: &Addr
) -> Addr {
    let cw721_id = router.store_code(contract_cw721());
    let msg = Cw721InstantiateMsg {
        name: "Passport Token".to_string(),
        symbol: "PASS".to_string(),
        minter: String::from(minter),
    };
    let contract = router
        .instantiate_contract(cw721_id, minter.clone(), &msg, &[], "passport-v1.0", None)
        .unwrap();
    contract
}

fn create_portal(
    router: &mut App,
    owner: Addr,
) -> Addr {
    let contract_id = router.store_code(contract_portal());
    let msg = InstantiateMsg {
        planet_name: "earth".to_string(),
        planet_sapients: vec![
            Sapient {
                name: "Some Cyborg".to_string(),
                species: Species {
                    name: "Cyborg".to_string(),
                    sapience_level: SapienceScale::High,
                },
                telepathic: true,
            },
            Sapient {
                name: "Some Human".to_string(),
                species: Species {
                    name: "Human".to_string(),
                    sapience_level: SapienceScale::Medium,
                },
                telepathic: false,
            }
        ],
        minimum_sapience: SapienceScale::High,
        passport_contract: Addr::unchecked("portal"),   // Must be updated after instantiation and token creation
        potion_contract: Addr::unchecked("potion"),     // Must be updated after instantiation and potion creation
    };
    let contract = router
        .instantiate_contract(contract_id, owner, &msg, &[], "portal-v1.0", None)
        .unwrap();
    contract
}

#[test]
pub fn checking_minimum_sapience_level() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let sender_name = "not on list";
    let info = mock_info(sender_name, &[]);

    // Set the minimum_sapience
    let init_msg = InstantiateMsg {
        planet_name: "earth".to_string(),
        planet_sapients: vec![
            Sapient {
                name: "Some Cyborg".to_string(),
                species: Species {
                    name: "Cyborg".to_string(),
                    sapience_level: SapienceScale::High,
                },
                telepathic: true,
            },
            Sapient {
                name: "Some Human".to_string(),
                species: Species {
                    name: "Human".to_string(),
                    sapience_level: SapienceScale::Medium,
                },
                telepathic: false,
            }
        ],
        minimum_sapience: SapienceScale::High,
        passport_contract: Addr::unchecked("archway1yvnw8xj5elngcq95e2n2p8f80zl7shfwyxk88858pl6cgzveeqtqy7xtf7"),
        potion_contract: Addr::unchecked("archway1u6clujjm2qnem09gd4y7hhmulftvlt6mej4q0dd742tzcnsstt2q70lpu6"),
    };

    portal_instantiate(deps.as_mut(), env.clone(), info, init_msg).unwrap();
    let res = portal_query(deps.as_ref(), env.clone(), QueryMsg::MinimumSapience {}).unwrap();
    let res: SapienceResponse = from_binary(&res).unwrap();

    assert_eq!(res.level, SapienceScale::High);
}

/// To see debugger output from any println! macros, uncomment the macro 
/// and run test using the `nocapture` flag
/// E.g. cargo test -- --nocapture
#[test]
pub fn minting_passport() {
    let mut app = mock_app();
    let owner = Addr::unchecked("owner");
    let user = Addr::unchecked("user");
    let current_time = get_block_time(&mut app);
    increment_block_time(&mut app, current_time + 1000, 7);
    assert_eq!(get_block_time(&mut app), current_time + 1000);

    // Mint tokens to pay for gas
    mint_native(
        &mut app,
        owner.to_string(),
        String::from(DENOM),
        Uint128::from(10000u128),
    );
    mint_native(
        &mut app,
        user.to_string(),
        String::from(DENOM),
        Uint128::from(10000u128),
    );

    // Instance of portal contract
    let portal_contract = create_portal(
        &mut app,
        owner.clone(),
    );

    // Instance of passport token contract
    let nft_contract = create_cw721(&mut app, &portal_contract);

    // Update portal with correct passport token address
    let update_msg = ExecuteMsg::SetPassportContract {
        contract: nft_contract.clone(),
    };
    let _portal_update = app.execute_contract(
        owner.clone(), 
        portal_contract.clone(), 
        &update_msg, 
        &[]
    );

    // Mint first passport
    let mint_msg = ExecuteMsg::MintPassport {
        msg: MintMsg {
            name: "Traveler Name".to_string(),
            description: "Ever since you became a cyborg, you've been feeling pretty weird...".to_string(),
            image: "ipfs://QmZdPdZzZum2jQ7jg1ekfeE3LSz1avAaa42G6mfimw9TEn".to_string(),
            dna: "Example DNA String".to_string(),
            species: "Cyborg".to_string(),
            sapience_level: SapienceScale::High,
            identity: user.clone(),
        }
    };
    let mint_res = app.execute_contract(
        owner.clone(), 
        portal_contract.clone(), 
        &mint_msg, 
        &[]
    );
    // println!("{:?}", &mint_res);
    assert!(mint_res.is_ok());

    // Verify metadata is correct for first minted passport
    let metadata_extension = Some(Metadata {
        name: Some("Traveler Name".to_string()),
        description: Some("Ever since you became a cyborg, you've been feeling pretty weird...".into()),
        image: Some("ipfs://QmZdPdZzZum2jQ7jg1ekfeE3LSz1avAaa42G6mfimw9TEn".to_string()),
        dna: Some("Example DNA String".to_string()),
        species: Some("Cyborg".to_string()),
        sapience_level: Some(SapienceScale::High),
        issuer: Some(Addr::unchecked("contract0")),
        origin: Some("earth".to_string()),
        identity: Some(Addr::unchecked(user.clone())),
    });
    let nft_query: Cw721QueryMsg<Extension> = Cw721QueryMsg::NftInfo {
        token_id: user.clone().to_string(),
    };
    let nft_info: NftInfoResponse<Extension> = query(&mut app, nft_contract.clone(), nft_query.clone()).unwrap();
    // println!("{:?}", nft_info);
    assert_eq!(
        nft_info,
        NftInfoResponse::<Extension> {
            token_uri: None,
            extension: metadata_extension.clone(),
        }
    );

    // Verify travelers cannot mint while already holding a passport
    let mint_msg2 = ExecuteMsg::MintPassport {
        msg: MintMsg {
            name: "Failed Passport".to_string(),
            description: "Invalid".to_string(),
            image: "Invalid".to_string(),
            dna: "Invalid".to_string(),
            species: "Invalid".to_string(),
            sapience_level: SapienceScale::Low,
            identity: user.clone(),
        }
    };
    let minting_again_fails = app.execute_contract(
        owner.clone(), 
        portal_contract.clone(), 
        &mint_msg2, 
        &[]
    );
    // println!("{:?}", &minting_again_fails);
    assert!(minting_again_fails.is_err());

    // If their passport is burned user can mint again
    let burn_msg = Cw721ExecuteMsg::Burn { 
        token_id: user.clone().to_string(),
    };
    let burn_res = app.execute_contract(
        user.clone(),
        nft_contract.clone(),
        &burn_msg, 
        &[]
    );
    // println!("{:?}", &burn_res);
    assert!(burn_res.is_ok());
    let second_mint_res = app.execute_contract(
        owner.clone(), 
        portal_contract.clone(), 
        &mint_msg, 
        &[]
    );
    // println!("{:?}", &second_mint_res);
    assert!(second_mint_res.is_ok());

    // Verify metadata is correct for second minted passport
    // (Same NFT metadata and token_id minted again after burn)
    let second_nft_info: NftInfoResponse<Extension> = query(&mut app, nft_contract.clone(), nft_query).unwrap();
    // println!("{:?}", &second_nft_info);
    assert_eq!(
        second_nft_info,
        NftInfoResponse::<Extension> {
            token_uri: None,
            extension: metadata_extension,
        }
    );
}

/// For now this test is focused on validating passport requirements
/// To see debugger output from any println! macros, uncomment the macro 
/// and run test using the `nocapture` flag
/// E.g. cargo test -- --nocapture
#[test]
pub fn initiating_jump_ring_travel() {
    let mut app = mock_app();
    let owner = Addr::unchecked("owner");
    let user = Addr::unchecked("user");
    let another_user = Addr::unchecked("random");
    let another_portal = Addr::unchecked("jupiter");
    let current_time = get_block_time(&mut app);
    increment_block_time(&mut app, current_time + 1000, 7);
    assert_eq!(get_block_time(&mut app), current_time + 1000);

    // Mint tokens to pay for gas
    mint_native(
        &mut app,
        owner.to_string(),
        String::from(DENOM),
        Uint128::from(10000u128),
    );
    mint_native(
        &mut app,
        user.to_string(),
        String::from(DENOM),
        Uint128::from(10000u128),
    );

    // Instance of portal contract
    let portal_contract = create_portal(
        &mut app,
        owner.clone(),
    );

    // Instance of passport token contract
    let nft_contract = create_cw721(&mut app, &portal_contract);

    // Update portal with correct passport token address
    let update_msg = ExecuteMsg::SetPassportContract {
        contract: nft_contract.clone(),
    };
    let _portal_update = app.execute_contract(
        owner.clone(), 
        portal_contract.clone(), 
        &update_msg, 
        &[]
    );

    // Mint first passport
    let mint_msg = ExecuteMsg::MintPassport {
        msg: MintMsg {
            name: "Traveler Name".to_string(),
            description: "Ever since you became a cyborg, you've been feeling pretty weird...".to_string(),
            image: "ipfs://QmZdPdZzZum2jQ7jg1ekfeE3LSz1avAaa42G6mfimw9TEn".to_string(),
            dna: "Example DNA String".to_string(),
            species: "Cyborg".to_string(),
            sapience_level: SapienceScale::High,
            identity: user.clone(),
        }
    };
    let mint_res = app.execute_contract(
        owner.clone(), 
        portal_contract.clone(), 
        &mint_msg, 
        &[]
    );
    assert!(mint_res.is_ok());

    // Traveling without a valid passport fails
    let failing_travel_msg = ExecuteMsg::JumpRingTravel {
        to: another_portal.clone(), 
        traveler: another_user,
    };
    let failed_travel_res = app.execute_contract(
        owner.clone(), 
        portal_contract.clone(), 
        &failing_travel_msg,
        &[]
    );
    // println!("{:?}", &failed_travel_res);
    assert!(failed_travel_res.is_err());

    // Traveling with a valid passport succeeds
    let travel_msg = ExecuteMsg::JumpRingTravel { 
        to: another_portal, 
        traveler: user.clone(),
    };
    let travel_res = app.execute_contract(
        owner.clone(), 
        portal_contract.clone(), 
        &travel_msg,
        &[]
    );
    // println!("{:?}", &travel_res);
    assert!(travel_res.is_ok());
}