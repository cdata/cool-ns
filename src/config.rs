use anyhow::Result;
use cosmwasm_std::{Coin, StdResult, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Config {
    pub registration_fee: Option<Coin>,
    pub allowed_tlds: Vec<String>,
}

pub fn get_config_storage(storage: &mut dyn Storage) -> Singleton<Config> {
    singleton(storage, b"config")
}

pub fn get_config_storage_read(storage: &dyn Storage) -> ReadonlySingleton<Config> {
    singleton_read(storage, b"config")
}

pub fn get_config(storage: &dyn Storage) -> StdResult<Config> {
    get_config_storage_read(storage).load()
}

pub fn set_config<'a>(storage: &'a mut dyn Storage, config: Config) -> Result<()> {
    get_config_storage(storage).save(&config)?;
    Ok(())
}
