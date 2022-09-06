use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use universe::species::{SapienceScale, Sapient};

static CONFIG_KEY: &[u8] = b"config";

pub fn config(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, CONFIG_KEY)
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct State {
    pub owner: Addr,
    pub planet_name: String,
    pub planet_sapients: Vec<Sapient>,
    pub minimum_sapience: SapienceScale,
}