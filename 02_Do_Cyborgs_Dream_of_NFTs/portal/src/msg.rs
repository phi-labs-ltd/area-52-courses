use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;
use universe::species::{SapienceScale, Sapient, Traveler};

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    JumpRingPreCheck { traveler: Traveler },
    MinimumSapience {},
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    SetPlanetName { to: String },
    SetSapientNames { to: Vec<Sapient> },
    SetMinimumSapience { to: SapienceScale },
    SetPassportContract { contract: Addr },
    SetPotionContract { contract: Addr },
    MintPassport { msg: MintMsg },
    JumpRingTravel { to: Addr, traveler: Addr, },
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub planet_name: String,
    pub planet_sapients: Vec<Sapient>,
    pub minimum_sapience: SapienceScale,
    pub passport_contract: Addr,
    pub potion_contract: Addr,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MintMsg {
    pub name: String,           // A human readable username (name is required for interoperability with NFT marketplaces)
    pub description: String,    // Description is also required for interoperability with NFT marketplaces
    pub image: String,          // Image is also required for interoperability with NFT Marketplaces
    pub dna: String,            // Allows for proving cyberdization and traveler authenticity
    pub species: String,
    pub sapience_level: SapienceScale,
    pub identity: Addr,         // The owner's wallet address
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JumpRingCheckResponse {
    pub valid: bool,
}
