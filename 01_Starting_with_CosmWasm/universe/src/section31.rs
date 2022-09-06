use crate::species::Species;
use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub enum ExecuteMsg {
  Snitch {
    address: Addr,
    name: String,
    species: Species,
  },
}