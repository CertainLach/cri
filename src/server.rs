use tonic::{transport::Server, Request, Response, Status};

pub mod criapi {
    tonic::include_proto!("criapi");
}
use criapi::runtime_service_server::{RuntimeService,RuntimeServiceServer};

#[derive(Default)]
pub struct MyRuntime {}

#[tonic::async_trait]
impl RuntimeService for MyRuntime {
    async fn version(
        &self,
        request: Request<criapi::VersionRequest>,
    ) -> Result<Response<criapi::VersionResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let resp = criapi::VersionResponse {
            version: String::from("0.1.0"),
            runtime_api_version: String::from("v1alpha1"),
            runtime_name: String::from("crust"),
            runtime_version: String::from("0.0.1"),
        };
        Ok(Response::new(resp))
    }

    async fn create_container(
        &self,
        request: Request<criapi::CreateContainerRequest>,
    ) -> Result<Response<criapi::CreateContainerResponse>, Status> {
        let resp = criapi::CreateContainerResponse {
            container_id: String::from("stub"),
        };
        Ok(Response::new(resp))
    }

    async fn start_container(
        &self,
        request: Request<criapi::StartContainerRequest>,
    ) -> Result<Response<criapi::StartContainerResponse>, Status> {
        let resp = criapi::StartContainerResponse {
        };
        Ok(Response::new(resp))
    }

    async fn stop_container(
        &self,
        request: Request<criapi::StopContainerRequest>,
    ) -> Result<Response<criapi::StopContainerResponse>, Status> {
        let resp = criapi::StopContainerResponse {
        };
        Ok(Response::new(resp))
    }

    async fn remove_container(
        &self,
        request: Request<criapi::RemoveContainerRequest>,
    ) -> Result<Response<criapi::RemoveContainerResponse>, Status> {
        let resp = criapi::RemoveContainerResponse {
        };
        Ok(Response::new(resp))
    }

    async fn list_containers(
        &self,
        request: Request<criapi::ListContainersRequest>,
    ) -> Result<Response<criapi::ListContainersResponse>, Status> {
        let resp = criapi::ListContainersResponse {
            containers: Vec::new()
        };
        Ok(Response::new(resp))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let rt = MyRuntime::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(RuntimeServiceServer::new(rt))
        .serve(addr)
        .await?;

    Ok(())
}