//! For now, the server is included in the lib_nv repo, but will be moved later.
//! The server obviously acts as the front-facing interface for interacting with the novella kernel. It is currently implented
//! as a Rocket rest api, but this may change later. It is important now to just get something working
use nvcore::{
    ecs::Entman,
    mir::{Aarc, Mir},
};
use rocket::{
    http::{ContentType, Status},
    launch,
    request::{FromRequest, Outcome},
    response::Responder,
    Request,
};
///The routes that the server provides.
mod routes {
    use nvcore::ecs::Entity;
    use rocket::get;
    use rocket::serde::json::*;

    use super::*;
    #[get("/")]
    pub fn index() -> &'static str {
        "Welcome to the novella API!"
    }
    #[get("/entman/get_entity?<id>")]
    pub fn get_entity(id: String) -> Json<Entity> {
        todo!()
    }
}
#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![routes::index])
}

#[cfg(test)]
mod tests {
    use super::*;
}
