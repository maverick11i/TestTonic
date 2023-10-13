use template::template_client::TemplateClient;
use template::TemplateRequest;

use std::time::Instant;

pub mod template {
    tonic::include_proto!("template");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TemplateClient::connect("http://0.0.0.0:50051").await?;

    let request = tonic::Request::new(TemplateRequest {
        name: "Tonic".into(),
    });
    let start_time = Instant::now();

    let response = client.say_hello(request).await?;

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;

    println!("処理にかかった時間: {:?}", elapsed_time);

    println!("RESPONSE={:?}", response);

    Ok(())
}
