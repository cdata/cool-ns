use cosmwasm_std::Coin;
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
    Register { name: String, tld: String },
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CoolNSQueryMessage {
    Resolve { name: String, tld: String },
}
