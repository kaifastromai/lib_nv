#![feature(more_qualified_paths)]

use std::collections::BTreeSet;

use nvproc::component;

use crate::ecs::{
    components::{self, Names, NamesProp},
    Component, Entity,
};

use super::*;

#[test]
fn test_add_comp_to_entity() {
    let mut e = Entity::new("CompTest");
    //add component
    e.add_component::<components::Names>();
    e.add_component::<components::Fields>();
    assert!(e.has_component::<components::Names>());
    assert!(e.has_component::<components::Fields>());
    e.remove_component::<components::Names>();
    assert!(!e.has_component::<components::Names>());
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
    entity_manager.add_component::<components::Names>(
        test_entity,
        NamesProp {
            name: vec![String::from("Robert"), String::from("Rob")],
        },
    );
    let comp = entity_manager
        .get_component::<components::Names>(test_entity)
        .unwrap();
    assert_eq!(comp.name[0], "Robert");
}
#[test]
fn test_b_tree() {
    let btree = BTreeSet::<u32>::from_iter(vec![11, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    assert_eq!(btree.len(), 11);
    //print the tree
    for i in btree.iter() {
        println!("{}", i);
    }
}
