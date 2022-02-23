use nvproc::Param;

use super::*;
#[derive(Clone, Param)]
pub struct TestParam {
    name: String,
}
#[derive(Debug, Clone, Resource)]
pub struct TestRsrc {
    pub name: String,
    pub ent_id: bevy_ecs::entity::Entity,
}
pub struct PubResponse {
    pub name: String,
    pub ent_id: bevy_ecs::entity::Entity,
}
pub fn get_entity(mir: &mut Mir, p: TestParam) -> Result<Box<dyn ResrcTy>> {
    let ent_id = mir.add_entity(p.name.clone());
    let mut rsrc = TestRsrc {
        name: p.name,
        ent_id,
    };
    Ok(Box::new(rsrc))
}
pub fn test_fn(mir: &mut Mir, p: TestParam) -> Result<Box<TestRsrc>> {
    let ent = mir.em.add_entity(p.name);
    let name = "test";
    let mut rsrc = TestRsrc {
        name: name.to_string(),
        ent_id: ent,
    };
    println!("Hello world");
    Ok(Box::new(rsrc))
}
pub fn undo(mir: &mut Mir, rsrc: Resrc<&TestRsrc>) -> Result<()> {
    mir.em.remove_entity(rsrc.ent_id);
    Ok(())
}

#[test]
fn test_action_register() {
    let action = Action::<TestRsrc, TestParam>::new(
        &test_fn,
        &undo,
        TestParam {
            name: "test".to_string(),
        },
    );
    let mut mir = Mir::new();

    {
        let mut act = Actman::new(&mut mir);
        act.register_action(action);
        act.advance();
        act.regress();
    }
    assert_eq!(mir.get_entity_count(), 0);
}
