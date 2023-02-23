use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;
use cw_storage_plus::{Item};
use universe::species::{SapienceScale, Sapient};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub planet_name: String,
    pub planet_sapients: Vec<Sapient>,
    pub minimum_sapience: SapienceScale,
    pub passport_contract: Addr,
    pub potion_contract: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");