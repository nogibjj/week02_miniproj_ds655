use lambda_runtime::{Context, Error, Handler};
use lambda_runtime::handler_fn;
use serde_derive::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;

#[derive(Deserialize, Clone)]
pub struct CustomEvent {
    #[serde(rename = "firstName")]
    first_name: String,
}

#[derive(Serialize, Clone)]
pub struct CustomOutput {
    message: String,
}

async fn my_handler(e: CustomEvent, _c: Context) -> Result<CustomOutput, Error> {
    if e.first_name.is_empty() {
        return Err(Error::from("Missing first name"));
    }

    Ok(CustomOutput {
        message: format!("Hello, {}!", e.first_name),
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = handler_fn(my_handler);
    lambda_runtime::run(handler).await?;
    Ok(())
}
