use cosmwasm_std::{Addr, Storage};
use cosmwasm_storage::{
    bucket, bucket_read, singleton, singleton_read, Bucket, ReadonlyBucket, ReadonlySingleton,
    Singleton,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use universe::species::Species;

static CONFIG_KEY: &[u8] = b"config";
static IMBIBER_KEY: &[u8] = b"imbiber";

pub fn imbiber(storage: &mut dyn Storage) -> Bucket<Imbiber> {
    bucket(storage, IMBIBER_KEY)
}

pub fn imbiber_read(storage: &dyn Storage) -> ReadonlyBucket<Imbiber> {
    bucket_read(storage, IMBIBER_KEY)
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct State {
    pub owner: Addr,
    pub dna_length: usize,
    pub dna_modulus: u8,
    pub swigs: u8,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Imbiber {
    pub address: Addr,
    pub species: Species,
    pub name: String,
    pub cyborg_dna: Vec<u8>,
}

pub fn config(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, CONFIG_KEY)
}