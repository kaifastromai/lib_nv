use super::component::Field;
use super::*;
use archetypes::*;

#[test]
fn test_add_component() {
    let mut em = Entman::new();
    let entity = em.add_entity();
    em.add_component(
        entity,
        Field {
            name: "name".to_string(),
            value: "value".to_string(),
        },
    );
    em.add_default::<Field>(entity);
    assert!(em.get_entity_ref(entity).unwrap().has_component::<Field>());

}
