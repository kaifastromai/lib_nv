#![feature(more_qualified_paths)]

use nvproc::component;

use crate::ecs::{
    components::{self, Name, NameProp},
    Component, Entity,
};

use super::*;

#[test]
fn test_add_comp_to_entity() {
    let mut e = Entity::new("CompTest");
    //add component
    e.add_component::<components::Name>();
    e.add_component::<components::Fields>();
    assert!(e.has_component::<components::Name>());
    assert!(e.has_component::<components::Fields>());
    e.remove_component::<components::Name>();
    assert!(!e.has_component::<components::Name>());
    assert!(e.has_component::<components::Fields>());
}

#[test]
fn test_add_entity() {
    let mut entity_manager = EntityManager::new();
    let entity_indx = entity_manager.create_entity(String::from("test"));
    let entity = entity_manager.get_entity(entity_indx).unwrap();

    assert_eq!(entity.entity_class, "test");
}
#[test]
fn test_add_get_component() {
    let mut entity_manager = EntityManager::new();
    let test_entity = entity_manager.create_entity(String::from("Entity1"));
    entity_manager.add_component::<components::Name>(
        test_entity,
        NameProp {
            name: "Bob",
            aliases: vec!["Robert", "Rob"],
        },
    );
    let comp = entity_manager
        .get_component::<components::Name>(test_entity)
        .unwrap();
    assert_eq!(comp.name, "Bob");
}
