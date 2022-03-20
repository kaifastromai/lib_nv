use super::component::*;
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
#[test]
fn test_get_component() {
    let mut em = Entman::new();
    let entity = em.add_entity();
    let comp_id = em
        .add_component::<Field>(
            entity,
            Field {
                name: "name".to_string(),
                value: "value".to_string(),
            },
        )
        .unwrap();
    let field = em
        .get_entity_component_by_id::<Field>(entity, comp_id)
        .unwrap();

    assert_eq!(field.name, "name");
    assert_eq!(field.value, "value");
    {
        let name_comp_id = em.add_default::<Name>(entity).unwrap();
        let name_comp = em
            .get_entity_component_by_id::<Name>(entity, name_comp_id)
            .unwrap();
        assert_eq!(name_comp.name, String::default());
    }
    //get solely by id
    let field_comp_id = em.add_default::<Field>(entity).unwrap();
    let field_comp = em.get_component_with_id::<Field>(field_comp_id).unwrap();
    assert_eq!(field_comp.name, String::default());
    assert_eq!(field_comp.value, String::default());
}
fn test_get_components() {
    let mut em = Entman::new();
    let entity = em.add_entity();
    let comp1 = em
        .add_component::<Field>(
            entity,
            Field {
                name: "name".to_string(),
                value: "value".to_string(),
            },
        )
        .unwrap();
    let comp2 = em
        .add_component::<Name>(
            entity,
            Name {
                name: "name".to_string(),
                aliases: vec![],
            },
        )
        .unwrap();
        
}
