use anyhow::{anyhow, Result};
use cosmwasm_std::{to_binary, Addr, DepsMut, MessageInfo, Response};

use crate::{config::get_config, registry::NameRegistry};

/// Attempt to set the owner for a name
/// Validates that the current owner address is the one asking for the change
/// Returned data includes the new name record, including updated lineage hash
pub fn try_set_owner(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
    tld: String,
    new_owner: Addr,
) -> Result<Response> {
    let config = get_config(deps.storage)?;

    if !config.allowed_tlds.contains(&tld) {
        return Err(anyhow!("Unknown TLD: {}", tld));
    }

    let mut name_registry = NameRegistry::new(&tld);
    let name_record = name_registry.try_resolve_name(deps.storage, &name)?;

    if name_record.owner != info.sender {
        return Err(anyhow!("Only the name owner can change its owner"));
    }

    let name_record = name_registry.try_set_owner(deps.storage, &name, new_owner)?;

    Ok(Response::default().set_data(to_binary(&name_record)?))
}

/// Attempt to set the value for a name record
/// Validates that the owner address is the one setting the value
/// Returned data includes the new name record, including updated lineage hash
pub fn try_set_value(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
    tld: String,
    value: Option<String>,
) -> Result<Response> {
    let config = get_config(deps.storage)?;

    if !config.allowed_tlds.contains(&tld) {
        return Err(anyhow!("Unknown TLD: {}", tld));
    }

    let mut name_registry = NameRegistry::new(&tld);
    let name_record = name_registry.try_resolve_name(deps.storage, &name)?;

    if name_record.owner != info.sender {
        return Err(anyhow!("Only the name owner can change its value"));
    }

    let name_record = name_registry.try_set_value(deps.storage, &name, value)?;

    Ok(Response::default().set_data(to_binary(&name_record)?))
}

/// Attempt to register a name
/// Validates that the configured fee (if any) has been paid
/// Names that have been registered cannot be re-registered
pub fn try_register_name(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
    tld: String,
) -> Result<Response> {
    let config = get_config(deps.storage)?;

    match config.registration_fee {
        Some(fee) => {
            let paid = info
                .funds
                .iter()
                .any(|coin| coin.denom == fee.denom && coin.amount.u128() >= fee.amount.u128());

            if paid {
                Ok(())
            } else {
                Err(anyhow!(
                    "Insufficient payment sent for registration fee. Required amount is {}.{}",
                    fee.amount,
                    fee.denom
                ))
            }
        }
        _ => Ok(()),
    }?;

    if !config.allowed_tlds.contains(&tld) {
        return Err(anyhow!("Unknown TLD: {}", tld));
    }

    let mut name_registry = NameRegistry::new(&tld);

    name_registry.try_register(deps.storage, &name, info.sender)?;

    Ok(Response::default())
}
