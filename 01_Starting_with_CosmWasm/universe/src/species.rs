use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct Species {
    pub name: String,
    pub sapience_level: SapienceScale,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub enum SapienceScale {
    None = 0,   // bugs
    Low = 1,    // cats, dogs
    Medium = 2, // ravens, rats, Terran humans
    High = 3,   // proper intelligent beings
}

impl SapienceScale {
    pub fn as_str(&self) -> &str {
        match self {
            SapienceScale::None => "None",
            SapienceScale::Low => "Low",
            SapienceScale::Medium => "Medium",
            SapienceScale::High => "High",
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Traveler {
    pub name: String,
    pub home: Addr,
    pub species: Species,
    pub cyberdized: bool,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct SapienceResponse {
    pub level: SapienceScale,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Sapient {
    name: String,
    telepathic: bool,
}