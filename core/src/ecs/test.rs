use super::*;
use archetypes::*;

#[test]
fn ecs_component_merge() {
    let mut c1: Components = Default::default();
    let mut c2: Components = Default::default();
    let n = c1.get_mut::<components::Names>();
    n.insert(
        0,
        components::Names::new(
            0,
            components::NamesProp {
                name: vec!["Name".to_string()],
            },
        ),
    );
    c2.merge(c1);
    assert_eq!(1, c2.get::<components::Names>().len());
}
#[test]
fn ecs_merge_from_entity_graph() {
    let mut em = EntityManager::new();
    let ca = archetypes::Character::new(
        "Bob".to_string(),
        "Male".into(),
        "Blah blah".into(),
        "23".into(),
        "Now".into(),
        "Britain".into(),
    );
    let egraph = ca.create_archetype();
    let eid = egraph.entities[0].id();
    em.add_from_entity_graph(egraph);
    assert_eq!(
        "Bob".to_string(),
        em.get_component::<components::Names>(eid).unwrap().name[0]
    );
}
#[test]
fn ecs_delete_entity() {
    let mut em = EntityManager::new();
    let id = em.create_entity("Default".to_string());
    em.add_component::<components::Names>(
        id,
        components::NamesProp {
            name: vec!["Bob".to_string()],
        },
    );
    em.delete_entity(id);
    assert_eq!(
        0,
        em.get_component::<components::Names>(id)
            .unwrap()
            .name
            .len()
    );
    assert!(em.get_component::<components::Names>(id).is_none());
}
