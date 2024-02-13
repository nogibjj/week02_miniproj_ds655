use lambda_runtime::{Context, Error};
use lambda_runtime::handler_fn;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct CustomEvent {
    #[serde(rename = "firstName")]
    first_name: Option<String>,
    original_name: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct CustomOutput {
    message: String,
}

async fn my_handler(mut e: CustomEvent, _c: Context) -> Result<CustomOutput, Error> {
    let first_name = e.first_name.unwrap_or("Divya".to_string());
    let original_name = e.original_name.unwrap_or("Divya".to_string());

    Ok(CustomOutput {
        message: format!("Hello, {}! the reverse of your name is {}!", original_name, original_name.chars().rev().collect::<String>()),
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Use the AWS Lambda handler
    let handler = handler_fn(my_handler);
    lambda_runtime::run(handler).await?;

    Ok(())
}