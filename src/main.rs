use airtable_api::Airtable;
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
pub async fn main() {
    let args = Cli::from_args();
    let airtable = Airtable::new(&args.api_key, &args.base_key);
    let records = airtable
        .list_records(&args.table, &args.view)
        .await
        .unwrap();

    records.iter().enumerate().for_each(|(_, record)| {
        match record.fields[&args.key_field].as_str() {
            Some(key) => match record.fields[&args.value_field].as_str() {
                Some(value) => println!("export '{}'='{}'", key, value),
                None => (),
            },
            None => (),
        }
    })
}
