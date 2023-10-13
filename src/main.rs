mod server;
mod usecases;
/// Main function for running the application.
///
/// This `main` function serves as the entry point of the application. It invokes
/// the `bootstrap::run()` function, which initializes and starts the application's
/// core logic asynchronously using the Tokio runtime.
///
/// # Errors
///
/// If an error occurs during the application's execution, it will be propagated
/// as an `Err` variant of the `Result`, containing an error type that implements
/// the `std::error::Error` trait.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    server::grpc_server::run().await?;

    Ok(())
}
