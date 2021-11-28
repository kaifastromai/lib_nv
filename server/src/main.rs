tonic::include_proto!("nvserver");

use entity_service_server::*;
use nvcore::ecs::EntityManager;
use nvcore::Project;
use std::sync::{Arc, Mutex};
use tonic::{transport::Server, Request, Response, Status};

pub struct NvServer {
    entity_manager: Mutex<EntityManager>,
    project: Mutex<Option<Project>>,
}
impl NvServer {
    fn new() -> NvServer {
        NvServer {
            entity_manager: Mutex::new(EntityManager::new()),
            project: Mutex::new(None),
        }
    }
}
#[tonic::async_trait]
impl EntityService for NvServer {
    async fn create(&self, request: Request<Name>) -> Result<Response<EntityId>, Status> {
        println!("create");
        let res = EntityId {
            id: self
                .entity_manager
                .lock()
                .unwrap()
                .create_entity(request.into_inner().name)
                .to_string(),
        };
        Ok(Response::new(res))
    }
}
#[tonic::async_trait]
impl project_server::Project for NvServer {
    async fn create(&self, request: Request<ProjectParams>) -> Result<Response<EntityId>, Status> {
        let project_params = request.into_inner();
        //get id of project or create new one
        let id = match self.project.lock().unwrap().as_ref() {
            Some(e) => e.id,
            None => {
                let project = Project::new(
                    project_params.name.as_str(),
                    project_params.description.as_str(),
                );
                self.project.lock().unwrap().replace(project);
                self.project.lock().unwrap().as_ref().unwrap().id
            }
        };

        let res = EntityId { id: id.to_string() };

        Ok(Response::new(res))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = NvServer::new();
    Server::builder()
        .add_service(EntityServiceServer::new(service))
        .add_service(project_server::ProjectServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}
