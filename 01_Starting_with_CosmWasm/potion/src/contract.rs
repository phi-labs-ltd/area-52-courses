use crate::error::ContractError;
use crate::execute_fns::{
    imbibe_potion::imbibe_potion, step_through_jumpring::step_through_jumpring,
};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query_fns::number_of_swigs;
use crate::state::{config, State};
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
    SubMsgResult,
};

static DEFAULT_NUMBER_OF_SWIGS: u8 = 3;

#[entry_point]
pub fn reply(_deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.result {
        SubMsgResult::Ok(_) => Ok(Response::default()),
        SubMsgResult::Err(_) => Err(ContractError::NothingToSeeHere {}),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::NumberOfSwigs {} => to_binary(&number_of_swigs(deps)?),
    }
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ImbibePotion { name, species } => imbibe_potion(name, species, deps, info),
        ExecuteMsg::StepThroughJumpRing {
            portal,
            destination,
            traveler,
        } => step_through_jumpring(portal, destination, traveler, deps, info),
    }
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender,
        dna_length: msg.dna_length,
        dna_modulus: msg.dna_modulus,
        swigs: DEFAULT_NUMBER_OF_SWIGS,
    };
    config(deps.storage).save(&state)?;
    Ok(Response::default())
}
