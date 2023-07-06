use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client, Error};
use aws_sdk_dynamodb::types::{AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType};
use aws_sdk_dynamodb::operation::create_table::CreateTableOutput;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let create_result = create_table(&client, "Movies", "year").await?;

    let resp = client.list_tables().send().await?;

    println!("Tables: ");

    let names = resp.table_names().unwrap_or_default();

    for name in names {
        println!("  {}", name);
    }

    println!();
    println!("Found {} tables", names.len());

    Ok(())
}


async fn create_table(
    client: &Client,
    table: &str,
    key: &str,
) -> Result<CreateTableOutput, Error> {
    let a_name: String = key.into();
    let table_name: String = table.into();

    let ad = AttributeDefinition::builder()
        .attribute_name(&a_name)
        .attribute_type(ScalarAttributeType::S)
        .build();

    let ks = KeySchemaElement::builder()
        .attribute_name(&a_name)
        .key_type(KeyType::Hash)
        .build();

    let pt = ProvisionedThroughput::builder()
        .read_capacity_units(10)
        .write_capacity_units(5)
        .build();

    let create_table_response = client.create_table()
        .table_name(table_name)
        .key_schema(ks)
        .attribute_definitions(ad)
        .provisioned_throughput(pt)
        .send()
        .await;

    match create_table_response {
        Ok(output) => {
            println!("Created table: {:?}", output);
            Ok(output)
        }
        Err(error) => {
            println!("Error creating table: {:?}", error);
            Err(error.into()) 
        }
    }
}

