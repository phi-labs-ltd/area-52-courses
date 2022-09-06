use crate::msg::JumpRingCheckResponse;
use crate::state::config_read;
use cosmwasm_std::{to_binary, Binary, Deps, StdResult};
use universe::species::{SapienceResponse, Traveler};

pub fn minimum_sapience(deps: Deps) -> StdResult<Binary> {
    let state = config_read(deps.storage).load()?;
    let out = to_binary(&SapienceResponse {
        level: state.minimum_sapience,
    })?;
    Ok(out)
}

pub fn jumpring_check(traveler: Traveler) -> StdResult<Binary> {
    let out = to_binary(&JumpRingCheckResponse {
        valid: traveler.cyberdized,
    })?;
    Ok(out)
}