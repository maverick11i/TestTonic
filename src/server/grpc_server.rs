use std::env;
use tonic::transport::Server;

use crate::usecases::config::proto::template::template_server::TemplateServer;
use crate::usecases::handlers::template::TemplateHandler;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    let addr = format!(
        "{}:{}",
        env::var("APPLICATION_ADDRESS")?,
        env::var("APPLICATION_PORT")?
    )
    .parse()?;

    let template_service = TemplateHandler::default();

    Server::builder()
        .add_service(TemplateServer::new(template_service))
        .serve(addr)
        .await?;

    Ok(())
}
