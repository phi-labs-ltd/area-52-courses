use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use universe::species::{Species, Traveler};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub dna_length: usize,
    pub dna_modulus: u8,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub enum ExecuteMsg {
    ImbibePotion {
        name: String,
        species: Species,
    },
    StepThroughJumpRing {
        portal: Addr,
        destination: Addr,
        traveler: Traveler,
    },
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub enum QueryMsg {
    NumberOfSwigs {},
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct SwigResponse {
    pub swigs: u8,
}