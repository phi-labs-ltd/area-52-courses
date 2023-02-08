use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use std::env::current_dir;
use std::fs::create_dir_all;

use universe::section31::ExecuteMsg;
use universe::species::{SapienceResponse, SapienceScale, Sapient, Traveler};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(Traveler), &out_dir);
    export_schema(&schema_for!(SapienceScale), &out_dir);
    export_schema(&schema_for!(SapienceResponse), &out_dir);
    export_schema(&schema_for!(Sapient), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
}
