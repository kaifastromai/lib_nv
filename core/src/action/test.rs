use nvproc::Param;

use crate::ecs::Id;

use super::request::{Reqman, Request};
use super::*;
#[derive(Clone, Param)]
pub struct TestParam {
    name: String,
}
#[derive(Debug, Clone)]
pub struct TestRsrc {
    pub name: String,
    pub ent_id: Id,
}
pub struct PubResponse {
    pub name: String,
    pub ent_id: Id,
}
pub fn get_entity(mir: &mut Mir, p: (String, i32)) -> Result<(String, Id)> {
    let ent_id = mir.em.add_entity();
    let mut res = (p.0, ent_id);
    Ok(res)
}
pub fn test_fn(mir: &mut Mir, p: TestParam) -> Result<Box<TestRsrc>> {
    let ent = mir.em.add_entity();
    let name = "test";
    let mut rsrc = TestRsrc {
        name: name.to_string(),
        ent_id: ent,
    };
    Ok(Box::new(rsrc))
}
pub fn undo(mir: &mut Mir, rsrc: Resrc<&TestRsrc>) -> Result<()> {
    mir.em.remove_entity(rsrc.ent_id);
    Ok(())
}

#[test]
fn test_action_register() {
    let action = Action::new(
        &test_fn,
        &undo,
        TestParam {
            name: "test".to_string(),
        },
    );
    let mut mir = Mir::new();

    let mut act = Actman::new();
    act.register_action(action);
    act.advance(&mut mir);
    act.regress(&mut mir);

    assert_eq!(mir.exec(|m| { m.em.get_entity_count() }), 0);
}
#[test]
fn test_action_request() {
    let req = Request::new(&get_entity);
    let mut mir = Mir::new();
    let mut reqman = Reqman::new();
    let res = reqman
        .request(req, &mut mir, ("test".to_string(), 1))
        .unwrap();
    assert_eq!(res.0, "test");
}
