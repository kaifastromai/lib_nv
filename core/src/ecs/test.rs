use super::component::*;
use super::*;

#[test]
fn test_add_component() {
    let mut em = Entman::new();
    let entity = em.add_entity();
    em.add_component(
        entity,
        StringFieldComponent {
            name: "name".to_string(),
            value: "value".to_string(),
        },
    );
    em.add_default::<StringFieldComponent>(entity);
    assert!(em
        .get_entity_ref(entity)
        .unwrap()
        .has_component::<StringFieldComponent>());
}
#[test]
fn test_get_component() {
    let mut em = Entman::new();
    let entity = em.add_entity();
    let comp_id = em
        .add_component::<StringFieldComponent>(
            entity,
            StringFieldComponent {
                name: "name".to_string(),
                value: "value".to_string(),
            },
        )
        .unwrap();
    let field = em
        .get_entity_component_by_id::<StringFieldComponent>(entity, comp_id)
        .unwrap();

    assert_eq!(field.name, "name");
    assert_eq!(field.value, "value");
    {
        let name_comp_id = em.add_default::<NameComponent>(entity).unwrap();
        let name_comp = em
            .get_entity_component_by_id::<NameComponent>(entity, name_comp_id)
            .unwrap();
        assert_eq!(name_comp.name, String::default());
    }
    //get solely by id
    let field_comp_id = em.add_default::<StringFieldComponent>(entity).unwrap();
    let field_comp = em
        .get_component_with_id::<StringFieldComponent>(field_comp_id)
        .unwrap();
    assert_eq!(field_comp.name, String::default());
    assert_eq!(field_comp.value, String::default());
}
fn test_get_components() {
    let mut em = Entman::new();
    let entity = em.add_entity();
    let comp1 = em
        .add_component::<StringFieldComponent>(
            entity,
            StringFieldComponent {
                name: "name".to_string(),
                value: "value".to_string(),
            },
        )
        .unwrap();
    let comp2 = em
        .add_component::<NameComponent>(
            entity,
            NameComponent {
                name: "name".to_string(),
                aliases: vec![],
            },
        )
        .unwrap();
}
