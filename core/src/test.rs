use crate::ecs::Component;

use super::*;

// #[test]
// fn test_add_entity() {
//     let mut entity_manager = EntityManager::new();
//     let entity_indx = entity_manager.create_entity(String::from("test"));
//     let entity = entity_manager.get_entity(entity_indx).unwrap();

//     assert_eq!(entity.entity_class, "test");
// }
// #[test]
// fn test_add_component() {
//     let mut entity_manager = EntityManager::new();
//     let test_entity = entity_manager.create_entity(String::from("Entity1"));

//     let entity_index = entity_manager.create_entity(String::from("Character entity"));
//     entity_manager.add_entity_reference_component(entity_index, test_entity);
//     let res = entity_manager.get_entity_references_component(entity_index);
//     assert_eq!(res.unwrap().get_owning_entity(), entity_index);
// }

