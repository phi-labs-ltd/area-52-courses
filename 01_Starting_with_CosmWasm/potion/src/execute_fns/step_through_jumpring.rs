use crate::error::ContractError;
use crate::execute_fns::check_sapience_level::check_sapience_level;
use crate::execute_fns::check_sent_required_payment::check_sent_required_payment;
use cosmwasm_std::{to_binary, Addr, DepsMut, MessageInfo, Response, WasmMsg, Coin, Uint128};
use portal::msg::ExecuteMsg;
use universe::species::Traveler;

// Exporting the payment token expected 
// by our contract, is helpful
pub static DENOM: &str = "uport"; 

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
        denom: DENOM.to_string(),
        amount: Uint128::from(1000000u128),
    };
    check_sent_required_payment(&info.funds, Some(required_payment))?;

    let msg = WasmMsg::Execute {
        contract_addr: portal.to_string(),
        msg: to_binary(&ExecuteMsg::JumpRingTravel { to: destination }).unwrap(),
        funds: vec![],
    };

    Ok(Response::new().add_message(msg))
}

#[cfg(test)]
mod tests {

    use cosmwasm_std::{Coin, Uint128};
    use crate::execute_fns::check_sent_required_payment::check_sent_required_payment;
    use crate::execute_fns::step_through_jumpring::DENOM;

    #[test]
    fn testing_payment_checker() {
        let required_payment = Coin {
            denom: DENOM.to_string(),
            amount: Uint128::from(1000000u128),
        };

        // Sending payment lower than required should fail
        let sent_payment_too_low = vec![
            Coin {
                denom: DENOM.to_string(),
                amount: Uint128::from(1000u128),
            }
        ];

        let err_payment_too_low = check_sent_required_payment(&sent_payment_too_low, Some(required_payment.clone()));
        assert!(err_payment_too_low.is_err());

        // Sending the correct amount of a different Coin should fail
        // Even though we send `PORT`, the native chain handles it as microport (`uport`)
        let sent_payment_incorrect_coin = vec![
            Coin {
                denom: "PORT".to_string(),
                amount: Uint128::from(1000000u128),
            }
        ];

        let err_payment_incorrect_coin = check_sent_required_payment(&sent_payment_incorrect_coin, Some(required_payment.clone()));
        assert!(err_payment_incorrect_coin.is_err());

        // Sending exactly the required payment should succeed
        let sent_exact_payment = vec![
            Coin {
                denom: DENOM.to_string(),
                amount: Uint128::from(1000000u128),
            }
        ];

        let success1 = check_sent_required_payment(&sent_exact_payment, Some(required_payment.clone()));
        assert!(success1.is_ok());

        // Sending more funds than required payment should succeed
        let sent_higher_payment_than_required = vec![
            Coin {
                denom: DENOM.to_string(),
                amount: Uint128::from(1500000u128),
            }
        ];

        let success2 = check_sent_required_payment(&sent_higher_payment_than_required, Some(required_payment.clone()));
        assert!(success2.is_ok());
    }
}