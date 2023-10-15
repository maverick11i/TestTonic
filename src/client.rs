use template::template_client::TemplateClient;
use template::TemplateRequest;

use std::time::Instant;

pub mod template {
    tonic::include_proto!("template");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TemplateClient::connect("http://localhost:50051").await?;

    for i in 0..100000 {
        let start_time = Instant::now();

        let request = tonic::Request::new(TemplateRequest {
            name: "Tonic".into(),
        });

        let mut response = client.say_hello(request).await?;

        let end_time = Instant::now();
        let elapsed_time = end_time - start_time;

        println!(
            "RESPONSE COUNT: {:?} RESPONSE TIME: {:?}",
            i + 1,
            elapsed_time
        );

        println!("RESPONSE={:?}", response.get_mut().message);
    }

    Ok(())
}
