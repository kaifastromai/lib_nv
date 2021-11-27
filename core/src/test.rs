use super::*;

#[test]
fn test_add_entity() {
    let mut entity_manager = EntityManager::new();
    let entity_indx = entity_manager.create_entity("test");
    let entity = entity_manager.get_entity(entity_indx).unwrap();

    assert_eq!(entity.name, "test");
}
