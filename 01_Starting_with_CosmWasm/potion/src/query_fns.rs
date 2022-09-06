use crate::msg::SwigResponse;
use crate::state::config_read;
use cosmwasm_std::{Deps, StdResult};

pub fn number_of_swigs(deps: Deps) -> StdResult<SwigResponse> {
    let state = config_read(deps.storage).load()?;
    let swigs = state.swigs;
    Ok(SwigResponse { swigs: swigs })
}