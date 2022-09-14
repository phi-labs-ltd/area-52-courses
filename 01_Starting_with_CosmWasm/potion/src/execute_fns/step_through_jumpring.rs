use crate::error::ContractError;
use crate::execute_fns::check_sapience_level::check_sapience_level;
use crate::execute_fns::check_sent_required_payment::check_sent_required_payment;
use cosmwasm_std::{to_binary, Addr, DepsMut, MessageInfo, Response, WasmMsg, Coin, Uint128};
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

    let required_payment = Coin {
        denom: "PORT".to_string(),
        amount: Uint128::from(1u128),
    };
    check_sent_required_payment(&info.funds, Some(required_payment))?;

    let msg = WasmMsg::Execute {
        contract_addr: portal.to_string(),
        msg: to_binary(&ExecuteMsg::JumpRingTravel { to: destination }).unwrap(),
        funds: vec![],
    };

    Ok(Response::new().add_message(msg))
}
