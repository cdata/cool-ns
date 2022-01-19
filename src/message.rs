use cosmwasm_std::{Addr, Coin};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CoolNSInstantiateMessage {
    pub registration_fee: Option<Coin>,
    pub allowed_tlds: Vec<String>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CoolNSExecuteMessage {
    Register {
        name: String,
        tld: String,
    },
    SetValue {
        name: String,
        tld: String,
        value: Option<String>,
    },
    SetOwner {
        name: String,
        tld: String,
        owner: Addr,
    },
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CoolNSQueryMessage {
    ResolveName { name: String, tld: String },
    ResolveLineage { tld: String, lineage: String },
}
