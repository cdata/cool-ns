use anyhow::{anyhow, Context, Result};
use cosmwasm_std::{Addr, Storage};
use cosmwasm_storage::{bucket, bucket_read, Bucket, ReadonlyBucket};
use multihash::MultihashDigest;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct NameRecord {
    pub owner: Addr,
    pub value: Option<String>,
    pub lineage: Option<String>,
}

impl NameRecord {
    pub fn hash(&self) -> Vec<u8> {
        let multihash = multihash::Code::Sha2_256
            .digest(format!("{}.{:?}.{:?}", self.owner, self.value, self.lineage).as_bytes());
        Vec::from(multihash.digest())
    }
}

pub struct NameRegistry<'a> {
    name_registry_key: String,
    name_lineage_key: String,
    tld: &'a String,
}

impl<'a> NameRegistry<'a> {
    pub fn new(tld: &'a String) -> Self {
        NameRegistry {
            name_registry_key: format!("{}_name_registry", tld),
            name_lineage_key: format!("{}_name_lineage", tld),
            tld,
        }
    }

    /// Returns true if the name has been registered
    pub fn name_is_registered<'b>(&self, storage: &'b dyn Storage, name: &String) -> bool {
        let name_bytes = name.as_bytes();

        match self.get_registry(storage).may_load(name_bytes) {
            Ok(Some(_)) => true,
            Err(_) => true,
            _ => false,
        }
    }

    /// Attempt to resolve a NameRecord given a name
    /// Will return an Err if the name is not registered
    pub fn try_resolve_name<'b>(
        &self,
        storage: &'b dyn Storage,
        name: &String,
    ) -> Result<NameRecord> {
        self.get_registry(storage)
            .load(name.as_bytes())
            .context(format!("Could not find name: {}.{}", name, self.tld))
    }

    /// Attempt to lookup a lineage given a lineage hash
    /// Will return an Err if the lineage is not found
    pub fn try_resolve_lineage<'b>(
        &self,
        storage: &'b dyn Storage,
        lineage: &String,
    ) -> Result<NameRecord> {
        match base32::decode(
            base32::Alphabet::RFC4648 { padding: false },
            lineage.as_str(),
        ) {
            Some(lineage_bytes) => self
                .get_lineage(storage)
                .load(lineage_bytes.as_slice())
                .context(format!(
                    "Could not find lineage {} in .{}",
                    lineage, self.tld
                )),
            None => Err(anyhow!("Unable to decode specified lineage as base32")),
        }
    }

    /// Attempt to set the owner of a given name
    /// NOTE: Access control validation must be performed by the caller
    /// NOTE: If successful, this will unset the value associated with the name
    pub fn try_set_owner<'b>(
        &mut self,
        storage: &'b mut dyn Storage,
        name: &String,
        owner: Addr,
    ) -> Result<NameRecord> {
        let mut name_record = self.try_resolve_name(storage, name)?;
        let old_hash = name_record.hash();
        let old_hash = base32::encode(
            base32::Alphabet::RFC4648 { padding: false },
            old_hash.as_slice(),
        );

        name_record.owner = owner;
        name_record.lineage = Some(old_hash);

        self.try_save_record(storage, name, &name_record)?;

        Ok(name_record)
    }

    /// Attempt to set the value associated with a given name
    /// NOTE: Access control validation must be performed by the caller
    pub fn try_set_value<'b>(
        &mut self,
        storage: &'b mut dyn Storage,
        name: &String,
        value: Option<String>,
    ) -> Result<NameRecord> {
        if let Some(value) = &value {
            if value.len() > 256 {
                return Err(anyhow!(
                    "Value length {} exceeds max allowed length (256)",
                    value.len()
                ));
            }
        }

        let mut name_record = self.try_resolve_name(storage, name)?;
        let old_hash = name_record.hash();
        let old_hash = base32::encode(
            base32::Alphabet::RFC4648 { padding: false },
            old_hash.as_slice(),
        );

        name_record.value = value;
        name_record.lineage = Some(old_hash);

        self.try_save_record(storage, name, &name_record)?;

        Ok(name_record)
    }

    /// Attempt to register a name to a given owner
    /// An Err will be returned if the name is already registered
    pub fn try_register<'b>(
        &mut self,
        storage: &'b mut dyn Storage,
        name: &String,
        owner: &Addr,
    ) -> Result<()> {
        match self.name_is_registered(storage, &name) {
            true => Err(anyhow!("Name already registered: {}.{}", name, self.tld)),
            false => Ok(()),
        }?;

        self.try_save_record(
            storage,
            name,
            &NameRecord {
                owner: owner.clone(),
                value: None,
                lineage: None,
            },
        )?;

        Ok(())
    }

    /// Get a readonly bucket for name registry storage
    fn get_registry<'b>(&self, storage: &'b dyn Storage) -> ReadonlyBucket<'b, NameRecord> {
        bucket_read(storage, self.name_registry_key.as_bytes())
    }

    /// Get a writable bucket for name registry storage
    fn get_registry_mut<'b>(&mut self, storage: &'b mut dyn Storage) -> Bucket<'b, NameRecord> {
        bucket(storage, self.name_registry_key.as_bytes())
    }

    /// Get a readonly bucket for name lineage storage
    fn get_lineage<'b>(&self, storage: &'b dyn Storage) -> ReadonlyBucket<'b, NameRecord> {
        bucket_read(storage, self.name_lineage_key.as_bytes())
    }

    /// Get a writable bucket for name lineage storage
    fn get_lineage_mut<'b>(&mut self, storage: &'b mut dyn Storage) -> Bucket<'b, NameRecord> {
        // let &mut storage = self.storage;
        bucket(storage, self.name_lineage_key.as_bytes())
    }

    /// Save a name record in both the registry and lineage buckets
    fn try_save_record<'b>(
        &mut self,
        storage: &'b mut dyn Storage,
        name: &String,
        name_record: &NameRecord,
    ) -> Result<()> {
        self.get_registry_mut(storage)
            .save(name.as_bytes(), &name_record)?;

        self.get_lineage_mut(storage)
            .save(name_record.hash().as_slice(), &name_record)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::NameRegistry;
    use cosmwasm_std::{
        testing::{mock_info, MockStorage},
        Addr,
    };

    #[test]
    fn it_can_register_a_name() {
        let mut storage = MockStorage::default();
        let info = mock_info("foo", &[]);
        let sender = info.sender;

        let name = String::from("cdata");
        let tld = String::from("rad");
        let mut name_registry = NameRegistry::new(&tld);

        name_registry
            .try_register(&mut storage, &name, &sender)
            .unwrap();

        let name_record = name_registry.try_resolve_name(&storage, &name).unwrap();

        assert_eq!(name_record.owner, sender);
    }

    #[test]
    fn it_can_set_a_value_for_a_name() {
        let mut storage = MockStorage::default();
        let info = mock_info("foo", &[]);
        let sender = info.sender;

        let name = String::from("cdata");
        let tld = String::from("rad");
        let value = String::from("bar");

        let mut name_registry = NameRegistry::new(&tld);

        name_registry
            .try_register(&mut storage, &name, &sender)
            .unwrap();

        let name_record = name_registry
            .try_set_value(&mut storage, &name, Some(value.clone()))
            .unwrap();

        println!("Name record: {:#?}", name_record);

        assert_eq!(name_record.value, Some(value));
    }
}
