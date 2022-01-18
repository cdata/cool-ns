use anyhow::Result;
use cosmwasm_std::{
    entry_point, to_binary, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response,
};

use crate::{
    config::{get_config_storage, Config},
    execute::register_name,
    message::{CoolNSExecuteMessage, CoolNSInstantiateMessage, CoolNSQueryMessage},
    registry::get_name_record,
};

/// NOTE: Called once for the lifetime of the contract
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: CoolNSInstantiateMessage,
) -> Result<Response> {
    let config = Config {
        registration_fee: msg.registration_fee,
        allowed_tlds: msg.allowed_tlds.clone(),
    };

    let mut config_storage = get_config_storage(deps.storage);
    config_storage.save(&config)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: CoolNSExecuteMessage,
) -> Result<Response> {
    match msg {
        CoolNSExecuteMessage::Register { name, tld } => register_name(deps, info, name, tld),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: CoolNSQueryMessage) -> Result<QueryResponse> {
    match msg {
        CoolNSQueryMessage::Resolve { name, tld } => {
            let record = get_name_record(deps.storage, &name, &tld)?;
            let response_bytes = to_binary(&record)?;

            Ok(response_bytes)
        }
    }
}
