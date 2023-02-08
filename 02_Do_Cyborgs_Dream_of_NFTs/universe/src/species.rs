use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Species {
    pub name: String,
    pub sapience_level: SapienceScale,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
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
    pub fn as_num(&self) -> u8 {
        match self {
            SapienceScale::None => 0,
            SapienceScale::Low => 1,
            SapienceScale::Medium => 2,
            SapienceScale::High => 3,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Traveler {
    pub name: String,
    pub home: Addr,
    pub species: Species,
    pub cyberdized: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SapienceResponse {
    pub level: SapienceScale,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Sapient {
    pub name: String,
    pub species: Species,
    pub telepathic: bool,
}