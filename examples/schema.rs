use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use cool_ns::{
    config::Config,
    message::{CoolNSExecuteMessage, CoolNSInstantiateMessage, CoolNSQueryMessage},
    registry::NameRecord,
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(Config), &out_dir);
    export_schema(&schema_for!(NameRecord), &out_dir);
    export_schema(&schema_for!(CoolNSExecuteMessage), &out_dir);
    export_schema(&schema_for!(CoolNSInstantiateMessage), &out_dir);
    export_schema(&schema_for!(CoolNSQueryMessage), &out_dir);
}
