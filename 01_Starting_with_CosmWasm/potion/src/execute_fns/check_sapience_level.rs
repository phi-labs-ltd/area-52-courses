use crate::error::ContractError;
use crate::state::imbiber_read;
use cosmwasm_std::{to_binary, Addr, DepsMut, MessageInfo, QueryRequest, Response, WasmQuery};
use portal::msg::QueryMsg;
use universe::species::SapienceResponse;
use universe::utils::sapience_value;

pub fn check_sapience_level(
    portal: &Addr,
    deps: &DepsMut,
    info: &MessageInfo,
) -> Result<Response, ContractError> {
    let query = WasmQuery::Smart {
        contract_addr: portal.to_string(),
        msg: to_binary(&QueryMsg::MinimumSapience {})?,
    };
    let res: SapienceResponse = deps.querier.query(&QueryRequest::Wasm(query))?;

    let key = info.sender.as_bytes();
    let imbiber = imbiber_read(deps.storage).load(key).unwrap();
    let species_sapience = imbiber.species.sapience_level;

    if sapience_value(&res.level) < sapience_value(&species_sapience) {
        return Err(ContractError::NotSapientEnough {});
    };
    Ok(Response::default())
}
