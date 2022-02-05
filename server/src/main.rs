tonic::include_proto!("nvproto");

use nvcore::mir::Mir;
use nvcore::Project;
use std::sync::Mutex;
use tokio::sync::RwLock;
use tonic::{transport::Server, Request, Response, Status};
type Action = (fn(&mut Mir), i32);
pub struct NvServer {
    queue: RwLock<Vec<Action>>,
}

impl NvServer {
    fn new() -> NvServer {
        NvServer {
            queue: RwLock::new(Vec::new()),
        }
    }
}

fn test_action(mir: &mut Mir) {
    mir.say_hello();
}

#[tonic::async_trait]
impl entity_server::Entity for NvServer {
    async fn get_entity(
        &self,
        request: Request<EntityRequest>,
    ) -> Result<Response<FullEntityResponse>, Status> {
        //get write lock
        let mut queue = self.queue.write().await;
        queue.push((test_action, 1));
        Err(Status::already_exists("already exists"))
    }
    async fn add_component(
        &self,
        request: Request<ComponentRequest>,
    ) -> Result<Response<EntityResponse>, Status> {
        todo!()
    }
    async fn remove_component(
        &self,
        request: Request<RemoveComponentRequest>,
    ) -> Result<Response<EntityResponse>, Status> {
        todo!()
    }
    async fn add_entity(&self, request: Request<Name>) -> Result<Response<EntityResponse>, Status> {
        todo!()
    }
}

#[tonic::async_trait]
impl project_server::Project for NvServer {
    async fn create_project(
        &self,
        request: Request<project_message::Create>,
    ) -> Result<Response<EntityResponse>, Status> {
        todo!()
    }
    async fn get_project(
        &self,
        request: Request<project_message::Get>,
    ) -> Result<Response<project_message::Response>, Status> {
        todo!()
    }
    async fn get_entity(
        &self,
        request: Request<EntityRequest>,
    ) -> Result<Response<FullEntityResponse>, Status> {
        todo!()
    }
    async fn add_component(
        &self,
        request: Request<ComponentRequest>,
    ) -> Result<Response<EntityResponse>, Status> {
        todo!()
    }
    async fn remove_component(
        &self,
        request: Request<RemoveComponentRequest>,
    ) -> Result<Response<EntityResponse>, Status> {
        unimplemented!()
    }
    async fn add_entity(&self, request: Request<Name>) -> Result<Response<EntityResponse>, Status> {
        todo!()
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = NvServer::new();
    Server::builder()
        .add_service(entity_server::EntityServer::new(service))
        .serve(addr)
        .await?;
        
    Ok(())
}
