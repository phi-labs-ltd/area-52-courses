use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query_fns::{jump_ring_check, minimum_sapience};
use crate::execute_fns::{
    initiate_jump_ring_travel, mint_passport, set_minimum_sapience, set_passport_contract, 
    set_planet_name, set_potion_contract, set_sapient_names,
};
use crate::state::{Config, CONFIG};

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::JumpRingPreCheck { traveler } => jump_ring_check(traveler),
        QueryMsg::MinimumSapience {} => minimum_sapience(deps),
    }
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetPlanetName { to } => set_planet_name(to, deps, info),
        ExecuteMsg::SetSapientNames { to } => set_sapient_names(to, deps, info),
        ExecuteMsg::SetMinimumSapience { to } => set_minimum_sapience(to, deps, info),
        ExecuteMsg::SetPassportContract { contract } => set_passport_contract(contract, deps, info),
        ExecuteMsg::SetPotionContract { contract } => set_potion_contract(contract, deps, info),
        ExecuteMsg::MintPassport { msg } => mint_passport(msg, deps, env, info),
        ExecuteMsg::JumpRingTravel { to, traveler } => initiate_jump_ring_travel(to, traveler, deps, env, info),
    }
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        owner: info.sender,
        planet_name: msg.planet_name,
        planet_sapients: msg.planet_sapients,
        minimum_sapience: msg.minimum_sapience,
        passport_contract: msg.passport_contract,
        potion_contract: msg.potion_contract,
    };
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
        .add_attribute("owner", config.owner)
        .add_attribute("minimum_sapience", config.minimum_sapience.as_str()))
}
