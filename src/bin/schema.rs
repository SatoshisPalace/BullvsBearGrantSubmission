use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use satoshis_palace_bullvsbear::{
    msgs::{
        execute::execute_msg::ExecuteMsg, instantiate::InstantiateMsg,
        invoke::invoke_msg::InvokeMsg, query::query_msg::QueryMsg,
    },
    responses::{execute::execute_response::ExecuteResponse, query::query_response::QueryResponse},
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(ExecuteResponse), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(QueryResponse), &out_dir);
    export_schema(&schema_for!(InvokeMsg), &out_dir);
}
