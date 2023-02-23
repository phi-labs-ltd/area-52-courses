use cosmwasm_std::{
    Addr, CosmosMsg, DepsMut, Env, MessageInfo, QueryRequest, 
    to_binary, Response, WasmMsg, WasmQuery,
};

use cw721::{NftInfoResponse, TokensResponse};
use passport_token::{
    ExecuteMsg as Cw721ExecuteMsg, Extension, Metadata, 
    MintMsg as Cw721MintMsg, QueryMsg as Cw721QueryMsg,
};

use crate::error::ContractError;
use crate::state::CONFIG;
use crate::msg::MintMsg;

use universe::species::{SapienceScale, Sapient};

pub fn mint_passport(
    msg: MintMsg,
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    
    // Only potion contract can call this function
    let potion_contract = config.potion_contract;
    if info.sender != potion_contract {
        // XXX: Second `if` is for testing without instatiating all 3 contracts. Can
        // be removed later; e.g. after potion contract is updated to support minting
        if info.sender != config.owner {
            return Err(ContractError::Unauthorized {});
        }
    }

    // Minting fails if user already owns a passport
    let query_msg: passport_token::QueryMsg<Extension> = Cw721QueryMsg::Tokens {
        owner: msg.identity.clone().into(),
        start_after: None,
        limit: None,
    };
    let query_req = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: config.passport_contract.clone().into(),
        msg: to_binary(&query_msg).unwrap(),
    });
    let query_resp: TokensResponse = deps.querier.query(&query_req)?;
    if !query_resp.tokens.is_empty() {
        return Err(ContractError::IllegalAlien {});
    }

    let metadata_extension: Extension = Some(Metadata {
        name: Some(msg.name),
        description: Some(msg.description),
        image: Some(msg.image),
        dna: Some(msg.dna), // XXX TODO: Re-work the way DNA strings are built and parsed in Potion contract
        species: Some(msg.species),
        sapience_level: Some(msg.sapience_level),
        issuer: Some(env.contract.address.clone()),
        origin: Some(config.planet_name),
        identity: Some(msg.identity.clone()),
    });

    let mint_msg: passport_token::ExecuteMsg = Cw721ExecuteMsg::Mint(Cw721MintMsg {
        token_id: msg.identity.clone().into(),
        owner: msg.identity.into(),
        token_uri: None,
        extension: metadata_extension,
    });

    // Mint the passport
    let mint_resp: CosmosMsg = WasmMsg::Execute {
        contract_addr: config.passport_contract.into(),
        msg: to_binary(&mint_msg)?,
        funds: vec![],
    }
    .into();

    // After calling another contract we need to use a vector for responses
    // This allows for returning separate responses for the state transitions
    // of both this contract and the token contract it called via `CosmosMsg`
    let messages = vec![mint_resp];
    Ok(Response::new().add_messages(messages))
}

pub fn initiate_jump_ring_travel(
    _to: Addr,
    traveler: Addr,
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Only potion contract can call this function
    let potion_contract = config.potion_contract;
    if info.sender != potion_contract {
        // XXX: Second `if` is for testing without instatiating all 3 contracts. Can
        // be removed later; e.g. after potion contract is updated to support minting
        if info.sender != config.owner {
            return Err(ContractError::Unauthorized {});
        }
    }

    // Verify traveler's passport
    let query_msg: passport_token::QueryMsg<Extension> = Cw721QueryMsg::NftInfo {
        token_id: traveler.clone().into(),
    };
    let query_req = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: config.passport_contract.into(),
        msg: to_binary(&query_msg).unwrap(),
    });
    let query_resp: NftInfoResponse<Metadata> = deps.querier.query(&query_req)?;

    // Since we're using soulbound NFTs, and because only the JumpRing contract 
    // can mint, and because `token_id` is keyed by user address, identity theft 
    // shouldn't be possible. We can check as below, but this check could also be 
    // safely removed since the contract call would fail already fail with an error 
    // at `deps.querier.query(&query_req)?` since `token_id` is keyed by user address
    if query_resp.extension.identity.unwrap() != traveler {
        return Err(ContractError::Unauthorized {});
    }

    // XXX TODO: Process JumpRing travel -> _to: Addr

    Ok(Response::new()
        .add_attribute("action", "initiate_jump_ring_travel")
        .add_attribute("traveler", traveler))
}

pub fn set_minimum_sapience(
    to: SapienceScale,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }
    config.minimum_sapience = to;
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::default())
}

pub fn set_planet_name(
    to: String,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }
    config.planet_name = to;
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new().add_attribute("action", "set_planet_name"))
}

pub fn set_sapient_names(
    to: Vec<Sapient>,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }
    config.planet_sapients = to;
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new().add_attribute("action", "set_sapient_names"))
}

pub fn set_passport_contract(
    contract: Addr,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }
    config.passport_contract = contract;
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new().add_attribute("action", "set_passport_contract"))
}

pub fn set_potion_contract(
    contract: Addr,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }
    config.potion_contract = contract;
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new().add_attribute("action", "set_potion_contract"))
}