//! For now, the server is included in the lib_nv repo, but will be moved later.
//! The server obviously acts as the front-facing interface for interacting with the novella kernel. It is currently implented
//! as a Rocket rest api, but this may change later. It is important now to just get something working
use std::sync::Arc;

use nvcore::ecs::Entman;
use rocket::{
    http::{ContentType, Status},
    launch,
    request::{FromRequest, Outcome},
    response::Responder,
    tokio::sync::RwLock,
    Request,
};
///The routes that the server provides.
mod routes {
    use nvcore::ecs::{component::*, Entity, EntityOwned, Id};
    use rocket::serde::json::*;
    use rocket::{get, State};

    use super::*;
    #[get("/")]
    pub fn index() -> &'static str {
        "Welcome to the novella API!"
    }
    #[get("/entman/get_entity?<id>")]
    pub async fn get_entity(
        id: String,
        em: &State<EntmanState>,
    ) -> Result<Json<EntityOwned>, rocket::http::Status> {
        let em = &em.em.write().await;
        let e = em.get_entity_owned(id.parse::<Id>().unwrap());
        //check for error, return 404 if not found
        match e {
            Ok(v) => Ok(Json(v)),
            Err(_) => Err(rocket::http::Status::NotFound),
        }
    }
    #[post("/entman/create_entity")]
    pub async fn create_entity(em: &State<EntmanState>) -> Json<u128> {
        let em = &em.em;
        //get rwlock lock
        let mut em_lock = em.write().await;
        let e = em_lock.add_entity();
        Json(e)
    }
}
#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/api",
            routes![routes::index, routes::get_entity, routes::create_entity],
        )
        .manage(EntmanState::new())
}

pub struct EntmanState {
    em: Arc<RwLock<Entman>>,
}
impl EntmanState {
    fn new() -> Self {
        EntmanState {
            em: Arc::new(RwLock::new(Entman::new())),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
}
