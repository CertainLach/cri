use crate::criapi::{
    self, image_service_server::ImageService, runtime_service_server::RuntimeService,
};
use std::collections::HashMap;
use tonic::{Request, Response, Status};

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
        _request: Request<criapi::CreateContainerRequest>,
    ) -> Result<Response<criapi::CreateContainerResponse>, Status> {
        let resp = criapi::CreateContainerResponse {
            container_id: String::from("stub"),
        };
        Ok(Response::new(resp))
    }

    async fn start_container(
        &self,
        _request: Request<criapi::StartContainerRequest>,
    ) -> Result<Response<criapi::StartContainerResponse>, Status> {
        let resp = criapi::StartContainerResponse {};
        Ok(Response::new(resp))
    }

    async fn stop_container(
        &self,
        _request: Request<criapi::StopContainerRequest>,
    ) -> Result<Response<criapi::StopContainerResponse>, Status> {
        let resp = criapi::StopContainerResponse {};
        Ok(Response::new(resp))
    }

    async fn remove_container(
        &self,
        _request: Request<criapi::RemoveContainerRequest>,
    ) -> Result<Response<criapi::RemoveContainerResponse>, Status> {
        let resp = criapi::RemoveContainerResponse {};
        Ok(Response::new(resp))
    }

    async fn list_containers(
        &self,
        _request: Request<criapi::ListContainersRequest>,
    ) -> Result<Response<criapi::ListContainersResponse>, Status> {
        let resp = criapi::ListContainersResponse {
            containers: Vec::new(),
        };
        Ok(Response::new(resp))
    }

    async fn container_status(
        &self,
        _request: Request<criapi::ContainerStatusRequest>,
    ) -> Result<Response<criapi::ContainerStatusResponse>, Status> {
        let resp = criapi::ContainerStatusResponse {
            info: HashMap::new(),
            status: None,
        };
        Ok(Response::new(resp))
    }

    async fn container_stats(
        &self,
        _request: Request<criapi::ContainerStatsRequest>,
    ) -> Result<Response<criapi::ContainerStatsResponse>, Status> {
        let resp = criapi::ContainerStatsResponse { stats: None };
        Ok(Response::new(resp))
    }

    async fn list_container_stats(
        &self,
        _request: Request<criapi::ListContainerStatsRequest>,
    ) -> Result<Response<criapi::ListContainerStatsResponse>, Status> {
        let resp = criapi::ListContainerStatsResponse { stats: Vec::new() };
        Ok(Response::new(resp))
    }

    async fn update_container_resources(
        &self,
        _request: Request<criapi::UpdateContainerResourcesRequest>,
    ) -> Result<Response<criapi::UpdateContainerResourcesResponse>, Status> {
        let resp = criapi::UpdateContainerResourcesResponse {};
        Ok(Response::new(resp))
    }

    async fn reopen_container_log(
        &self,
        _request: Request<criapi::ReopenContainerLogRequest>,
    ) -> Result<Response<criapi::ReopenContainerLogResponse>, Status> {
        let resp = criapi::ReopenContainerLogResponse {};
        Ok(Response::new(resp))
    }

    async fn exec_sync(
        &self,
        _request: Request<criapi::ExecSyncRequest>,
    ) -> Result<Response<criapi::ExecSyncResponse>, Status> {
        let resp = criapi::ExecSyncResponse {
            exit_code: -1,
            stderr: Vec::new(),
            stdout: Vec::new(),
        };
        Ok(Response::new(resp))
    }

    async fn exec(
        &self,
        _request: Request<criapi::ExecRequest>,
    ) -> Result<Response<criapi::ExecResponse>, Status> {
        let resp = criapi::ExecResponse {
            url: String::from("url"),
        };
        Ok(Response::new(resp))
    }

    async fn attach(
        &self,
        _request: Request<criapi::AttachRequest>,
    ) -> Result<Response<criapi::AttachResponse>, Status> {
        let resp = criapi::AttachResponse {
            url: String::from("url"),
        };
        Ok(Response::new(resp))
    }
    async fn port_forward(
        &self,
        _request: Request<criapi::PortForwardRequest>,
    ) -> Result<Response<criapi::PortForwardResponse>, Status> {
        let resp = criapi::PortForwardResponse {
            url: String::from("url"),
        };
        Ok(Response::new(resp))
    }

    async fn run_pod_sandbox(
        &self,
        _request: Request<criapi::RunPodSandboxRequest>,
    ) -> Result<Response<criapi::RunPodSandboxResponse>, Status> {
        let reply = criapi::RunPodSandboxResponse {
            pod_sandbox_id: String::from("1234567890"),
        };
        Ok(Response::new(reply))
    }

    async fn stop_pod_sandbox(
        &self,
        _request: Request<criapi::StopPodSandboxRequest>,
    ) -> Result<Response<criapi::StopPodSandboxResponse>, Status> {
        let reply = criapi::StopPodSandboxResponse {};
        Ok(Response::new(reply))
    }

    async fn remove_pod_sandbox(
        &self,
        _request: Request<criapi::RemovePodSandboxRequest>,
    ) -> Result<Response<criapi::RemovePodSandboxResponse>, Status> {
        let reply = criapi::RemovePodSandboxResponse {};
        Ok(Response::new(reply))
    }

    async fn list_pod_sandbox(
        &self,
        _request: Request<criapi::ListPodSandboxRequest>,
    ) -> Result<Response<criapi::ListPodSandboxResponse>, Status> {
        let reply = criapi::ListPodSandboxResponse { items: Vec::new() };
        Ok(Response::new(reply))
    }

    async fn pod_sandbox_status(
        &self,
        _request: Request<criapi::PodSandboxStatusRequest>,
    ) -> Result<Response<criapi::PodSandboxStatusResponse>, Status> {
        Err(Status::unimplemented("not implemented"))
    }

    async fn status(
        &self,
        _request: Request<criapi::StatusRequest>,
    ) -> Result<Response<criapi::StatusResponse>, Status> {
        let resp = criapi::StatusResponse {
            status: None,
            info: HashMap::new(),
        };
        Ok(Response::new(resp))
    }

    async fn update_runtime_config(
        &self,
        _request: Request<criapi::UpdateRuntimeConfigRequest>,
    ) -> Result<Response<criapi::UpdateRuntimeConfigResponse>, Status> {
        let resp = criapi::UpdateRuntimeConfigResponse {};
        Ok(Response::new(resp))
    }
}

#[derive(Default)]
pub struct MyImage {}

#[tonic::async_trait]
impl ImageService for MyImage {
    async fn list_images(
        &self,
        _request: Request<criapi::ListImagesRequest>,
    ) -> Result<Response<criapi::ListImagesResponse>, Status> {
        let resp = criapi::ListImagesResponse { images: Vec::new() };
        Ok(Response::new(resp))
    }

    async fn pull_image(
        &self,
        _request: Request<criapi::PullImageRequest>,
    ) -> Result<Response<criapi::PullImageResponse>, Status> {
        let resp = criapi::PullImageResponse {
            image_ref: String::from("some_image"),
        };
        Ok(Response::new(resp))
    }

    async fn image_status(
        &self,
        _request: Request<criapi::ImageStatusRequest>,
    ) -> Result<Response<criapi::ImageStatusResponse>, Status> {
        let resp = criapi::ImageStatusResponse {
            image: None,
            info: HashMap::new(),
        };
        Ok(Response::new(resp))
    }

    async fn remove_image(
        &self,
        _request: Request<criapi::RemoveImageRequest>,
    ) -> Result<Response<criapi::RemoveImageResponse>, Status> {
        let resp = criapi::RemoveImageResponse {};
        Ok(Response::new(resp))
    }

    async fn image_fs_info(
        &self,
        _request: Request<criapi::ImageFsInfoRequest>,
    ) -> Result<Response<criapi::ImageFsInfoResponse>, Status> {
        let resp = criapi::ImageFsInfoResponse {
            image_filesystems: Vec::new(),
        };
        Ok(Response::new(resp))
    }
}
