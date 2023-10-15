use std::time::Instant;
use tonic::{Request, Response, Status};

use crate::usecases::config::proto::template::{
    template_server::Template, TemplateRequest, TemplateResponse,
};

#[derive(Debug, Default)]
pub struct TemplateHandler {}

#[tonic::async_trait]
impl Template for TemplateHandler {
    async fn say_hello(
        &self,
        request: Request<TemplateRequest>,
    ) -> Result<Response<TemplateResponse>, Status> {
        let start_time = Instant::now();
        let response = TemplateResponse {
            message: format!("Hello, {}!", request.into_inner().name).into(),
        };

        let end_time = Instant::now();
        let elapsed_time = end_time - start_time;

        println!("処理にかかった時間: {:?}", elapsed_time);

        Ok(Response::new(response))
    }
}
