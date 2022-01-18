use anyhow::{Context, Result};
use cosmwasm_std::{Addr, Storage};
use cosmwasm_storage::{bucket, bucket_read, Bucket, ReadonlyBucket};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct NameRecord {
    pub owner: Addr,
    pub value: Vec<u8>,
}

pub fn get_name_registry_storage<'a>(
    storage: &'a mut dyn Storage,
    tld: &String,
) -> Bucket<'a, NameRecord> {
    bucket(storage, format!("{}_name_registry", tld).as_bytes())
}

pub fn get_name_registry_storage_read<'a>(
    storage: &'a dyn Storage,
    tld: &String,
) -> ReadonlyBucket<'a, NameRecord> {
    bucket_read(storage, format!("{}_name_registry", tld).as_bytes())
}

pub fn get_name_record<'a>(
    storage: &'a dyn Storage,
    name: &String,
    tld: &String,
) -> Result<NameRecord> {
    let name_registry = get_name_registry_storage_read(storage, tld);

    name_registry
        .load(name.as_bytes())
        .context(format!("Could not find name: {}.{}", name, tld))
}
