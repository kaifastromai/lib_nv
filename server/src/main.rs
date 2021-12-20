tonic::include_proto!("nvproto");

use nvcore::ecs::components::{FieldsProp, ReferencesProp};
use nvcore::ecs::{components, EntityManager};
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
impl entity_server::Entity for NvServer {
    async fn get_entity(
        &self,
        request: Request<EntityRequest>,
    ) -> Result<Response<FullEntityResponse>, Status> {
        let entity_id = request.into_inner().id;
        let entity_manager = self.entity_manager.lock().unwrap();
        let entity = entity_manager.get_entity(entity_id.parse().unwrap());
        match entity {
            Some(entity) => Ok(Response::new(FullEntityResponse {
                id: entity.id().to_string(),
                name: entity.entity_class.clone(),
                signature: entity
                    .signature
                    .iter()
                    .map(|c| {
                        
                        *c as i32
                    })
                    .collect(),
            })),
            None => Err(Status::new(tonic::Code::NotFound, "Entity not found")),
        }
    }
    async fn add_component(
        &self,
        request: Request<ComponentRequest>,
    ) -> Result<Response<EntityResponse>, Status> {
        match request.get_ref().component_type {
            0 => self
                .entity_manager
                .lock()
                .unwrap()
                .add_component::<components::Fields>(
                    request.get_ref().entity_id.parse().unwrap(),
                    FieldsProp {
                        name: String::from("Name"),
                        fields: Vec::new(),
                    },
                ),
            1 => self
                .entity_manager
                .lock()
                .unwrap()
                .add_component::<components::References>(
                    request.get_ref().entity_id.parse().unwrap(),
                    ReferencesProp {
                        entity_references: Vec::new(),
                    },
                ),
            _ => {}
        };
        Ok(Response::new(EntityResponse {
            id: request.get_ref().clone().entity_id,
        }))
    }
    async fn remove_component(
        &self,
        request: Request<RemoveComponentRequest>,
    ) -> Result<Response<EntityResponse>, Status> {
        unimplemented!()
    }
    async fn add_entity(&self, request: Request<Name>) -> Result<Response<EntityResponse>, Status> {
        unimplemented!()
    }
}

#[tonic::async_trait]
impl project_server::Project for NvServer {
    async fn create_project(
        &self,
        request: Request<project_message::Create>,
    ) -> Result<Response<EntityResponse>, Status> {
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

        let res = EntityResponse { id: id.to_string() };

        Ok(Response::new(res))
    }
    async fn get_project(
        &self,
        request: Request<project_message::Get>,
    ) -> Result<Response<project_message::Response>, Status> {
        let p = self.project.lock().unwrap();
        let p = p.as_ref();
        match p {
            Some(p) => {
                let project_response = project_message::Response {
                    id: p.id.to_string(),
                    name: p.name.clone(),
                    description: p.description.clone(),
                };
                Ok(Response::new(project_response))
            }
            None => Err(Status::new(
                tonic::Code::NotFound,
                "No project has been loaded!",
            )),
        }
    }
    async fn get_entity(
        &self,
        request: Request<EntityRequest>,
    ) -> Result<Response<FullEntityResponse>, Status> {
        let entity_id = request.into_inner().id;
        let entity_manager = self.entity_manager.lock().unwrap();
        let entity = entity_manager.get_entity(entity_id.parse().unwrap());
        match entity {
            Some(entity) => Ok(Response::new(FullEntityResponse {
                id: entity.id().to_string(),
                name: entity.entity_class.clone(),
                signature: entity.signature.iter().map(|c| *c as i32).collect(),
            })),
            None => Err(Status::new(tonic::Code::NotFound, "Entity not found")),
        }
    }
    async fn add_component(
        &self,
        request: Request<ComponentRequest>,
    ) -> Result<Response<EntityResponse>, Status> {
        match request.get_ref().component_type {
            0 => self
                .entity_manager
                .lock()
                .unwrap()
                .add_component::<components::Fields>(
                    request.get_ref().entity_id.parse().unwrap(),
                    FieldsProp {
                        name: String::from("Name"),
                        fields: Vec::new(),
                    },
                ),
            1 => self
                .entity_manager
                .lock()
                .unwrap()
                .add_component::<components::References>(
                    request.get_ref().entity_id.parse().unwrap(),
                    ReferencesProp {
                        entity_references: Vec::new(),
                    },
                ),
            _ => {}
        };
        Ok(Response::new(EntityResponse {
            id: request.get_ref().clone().entity_id,
        }))
    }
    async fn remove_component(
        &self,
        request: Request<RemoveComponentRequest>,
    ) -> Result<Response<EntityResponse>, Status> {
        unimplemented!()
    }
    async fn add_entity(&self, request: Request<Name>) -> Result<Response<EntityResponse>, Status> {
        let mut em = self.entity_manager.lock().unwrap();
        let entity_ref = em.create_entity(request.into_inner().name);
        println!("Created entity with id: {}", entity_ref);
        Ok(Response::new(EntityResponse {
            id: entity_ref.to_string(),
        }))
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = NvServer::new();
    Server::builder()
        .add_service(project_server::ProjectServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}
