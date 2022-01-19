use anyhow::Result;
use cosmwasm_std::{
    entry_point, to_binary, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response,
};

use crate::{
    config::{set_config, Config},
    execute::{try_register_name, try_set_owner, try_set_value},
    message::{CoolNSExecuteMessage, CoolNSInstantiateMessage, CoolNSQueryMessage},
    registry::NameRegistry,
};

/// NOTE: Called once for the lifetime of the contract
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: CoolNSInstantiateMessage,
) -> Result<Response> {
    set_config(
        deps.storage,
        Config {
            registration_fee: msg.registration_fee,
            allowed_tlds: msg.allowed_tlds.clone(),
        },
    )?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    mut deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: CoolNSExecuteMessage,
) -> Result<Response> {
    match msg {
        CoolNSExecuteMessage::Register { name, tld } => {
            try_register_name(&mut deps, &info, &name, &tld)
        }
        CoolNSExecuteMessage::SetValue { name, tld, value } => {
            try_set_value(&mut deps, &info, &name, &tld, value)
        }
        CoolNSExecuteMessage::SetOwner { name, tld, owner } => {
            try_set_owner(&mut deps, &info, &name, &tld, owner)
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: CoolNSQueryMessage) -> Result<QueryResponse> {
    match msg {
        CoolNSQueryMessage::ResolveName { name, tld } => {
            let name_registry = NameRegistry::new(&tld);
            let record = name_registry.try_resolve_name(deps.storage, &name)?;
            let response_bytes = to_binary(&record)?;

            Ok(response_bytes)
        }
        CoolNSQueryMessage::ResolveLineage { lineage, tld } => {
            let name_registry = NameRegistry::new(&tld);
            let record = name_registry.try_resolve_lineage(deps.storage, &lineage)?;
            let response_bytes = to_binary(&record)?;

            Ok(response_bytes)
        }
    }
}
