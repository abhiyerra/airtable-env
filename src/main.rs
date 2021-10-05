use airtable_api::{Airtable, Record};
use std::collections::HashMap;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    api_key: String,
    base_key: String,
    table: String,
    view: String,
    key_field: String,
    value_field: String,
}

#[tokio::main]
async fn main() {
    let args = Cli::from_args();
    let airtable = Airtable::new(&args.api_key, &args.base_key, "");
    let records: Vec<Record<HashMap<String, String>>> = airtable
        .list_records(
            &args.table,
            &args.view,
            vec![&args.key_field, &args.value_field],
        )
        .await
        .unwrap();

    records.iter().enumerate().for_each(|(_, record)| {
        println!(
            "export {}='{}'",
            record.fields[&args.key_field], record.fields[&args.value_field]
        )
    })
}
