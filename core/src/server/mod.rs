tonic::include_proto!("nvserver");

use entity_service_server::*;

use crate::nv::core::ecs::EntityManager;
use tonic::{transport::Server, Request, Response, Status};
#[derive(Debug, Default)]
pub struct EntityServiceImpl {
    entity_state: EntityManager,
}

#[tonic::async_trait]
impl EntityService for EntityServiceImpl {
    async fn create(&self, request: Request<Name>) -> Result<Response<EntityId>, Status> {
        println!("create");
        let res = EntityId {
            id: "hello".to_string(),
        };
        Ok(Response::new(res))
    }
}

async fn exec() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = EntityServiceImpl::default();
    Server::builder()
        .add_service(EntityServiceServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}
