use anyhow::{anyhow, Result};
use cosmwasm_std::{DepsMut, MessageInfo, Response};

use crate::{
    config::get_config,
    registry::{get_name_registry_storage, NameRecord},
};

pub fn register_name(
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

    let mut name_registry = get_name_registry_storage(deps.storage, &tld);
    let name_bytes = name.as_bytes();

    if name_registry.may_load(name_bytes).is_ok() {
        return Err(anyhow!("Name already registered: {}.{}", name, tld));
    }

    name_registry.save(
        name_bytes,
        &NameRecord {
            owner: info.sender,
            value: Vec::new(),
        },
    )?;

    Ok(Response::default())
}
