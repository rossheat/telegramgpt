// main.rs
pub mod handler;
pub mod utils;

use handler::lambda_handler;

use lambda_http::{run, service_fn, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_current_span(false)
        .without_time()
        .init();
    run(service_fn(lambda_handler)).await
}
