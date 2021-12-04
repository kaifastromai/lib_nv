tonic::include_proto!("nvserver");

use nv_server::*;
use nvcore::ecs::EntityManager;
use nvcore::Project;
use std::sync::Mutex;
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
impl Nv for NvServer {
    async fn create_entity(&self, request: Request<Name>) -> Result<Response<EntityId>, Status> {
        println!("adding new entity");
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
    async fn create_project(
        &self,
        request: Request<ProjectRequest>,
    ) -> Result<Response<EntityId>, Status> {
        let project_params = request.into_inner();
        let mut p = self.project.lock().unwrap();
        //get id of project or create new one
        let id = match p.as_ref() {
            Some(_) => {
                println!("project already exists");
                return Err(Status::new(
                    tonic::Code::AlreadyExists,
                    "project already exists".to_string(),
                ));
            }
            None => {
                println!("creating new project");

                let project = Project::new(
                    project_params.name.as_str(),
                    project_params.description.as_str(),
                );
                p.replace(project);

                let nid = p.as_ref().unwrap().id;
                println!("created new project with id: {}", nid);
                nid
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
        .add_service(nv_server::NvServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}
