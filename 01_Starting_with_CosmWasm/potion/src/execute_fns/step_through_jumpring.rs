use crate::error::ContractError;
use crate::execute_fns::check_sapience_level::check_sapience_level;
use cosmwasm_std::{to_binary, Addr, DepsMut, MessageInfo, Response, WasmMsg};
use portal::msg::ExecuteMsg;
use universe::species::Traveler;

pub fn step_through_jumpring(
    portal: Addr,
    destination: Addr,
    traveler: Traveler,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    check_sapience_level(&portal, &deps, &info)?;

    if traveler.cyberdized != true {
        return Err(ContractError::NotACyborg {});
    }

    let msg = WasmMsg::Execute {
        contract_addr: portal.to_string(),
        msg: to_binary(&ExecuteMsg::JumpRingTravel { to: destination }).unwrap(),
        funds: vec![],
    };

    Ok(Response::new().add_message(msg))
}
