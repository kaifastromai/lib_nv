use std::io::{BufReader, BufWriter};

use common::exports::bincode::config::Config;
use common::exports::bincode::{Decode, Encode};

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
    em.add_component_default::<StringFieldComponent>(entity);
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
        let name_comp_id = em.add_component_default::<NameComponent>(entity).unwrap();
        let name_comp = em
            .get_entity_component_by_id::<NameComponent>(entity, name_comp_id)
            .unwrap();
        assert_eq!(name_comp.name, String::default());
    }
    //get solely by id
    let field_comp_id = em
        .add_component_default::<StringFieldComponent>(entity)
        .unwrap();
    let field_comp = em
        .get_component_with_id::<StringFieldComponent>(field_comp_id)
        .unwrap();
    assert_eq!(field_comp.name, String::default());
    assert_eq!(field_comp.value, String::default());
}
#[test]
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
#[test]
fn test_add_archetype() {
    let character_archetype = archetypes::CharacterArchetype {};
    let mut em = Entman::new();
    let entity = em.entity_from_archetype(character_archetype);
    let entity = em.get_entity(entity);
    //compare signatures
    assert_eq!(
        character_archetype.describe().get_signature(),
        entity.unwrap().get_signature()
    );
}
#[test]
fn test_dynamic() {
    let mut em = Entman::new();
    let entity = em.add_entity();
    em.add_component(
        entity,
        StringFieldComponent {
            name: "name".to_string(),
            value: "value".to_string(),
        },
    );
    em.add_component_default::<NameComponent>(entity);
    em.add_component_default::<StringFieldComponent>(entity);
    let owned_entity = em.get_entity_owned(entity).unwrap();

    let entinfo = em.get_entity(entity).unwrap();
    //assert signatures are the same
    assert_eq!(entinfo.get_signature(), owned_entity.get_signature());
}
#[test]
pub fn test_common_store_serde() {
    let mut ccsc = CommonComponentStore::<StringFieldComponent>::new();
    ccsc.insert_default(0);
    ccsc.insert_default(1);
    //convert to trait object
    let ccs = ccsc.get_any_owned();
    //serialize
    let bw = BufWriter::new(Vec::new());
    let res = bincode::encode_to_vec(ccs, bincode::config::standard()).unwrap();
    let ccs2 = bincode::decode_from_slice::<
        Box<dyn CommonComponentStoreTy>,
        bincode::config::Configuration,
    >(&*res, bincode::config::standard())
    .unwrap();
    //downcast to CommonComponentStore
    let ccs2: &CommonComponentStore<StringFieldComponent> = ccs2.0.into_store().unwrap();
    //compare
    assert_eq!(ccsc.get_name_ref(), ccs2.get_name_ref());

    //store entire storage object
    let mut store = Storage::new();
    store.insert_default::<StringFieldComponent>(0);
    store.insert_default::<StringFieldComponent>(1);
    store.insert_default::<NameComponent>(0);
    //serialize
    let bw = BufWriter::new(Vec::new());
    let res = bincode::encode_to_vec(store, bincode::config::standard()).unwrap();
    let store2 = bincode::decode_from_slice::<Storage, bincode::config::Configuration>(
        &*res,
        bincode::config::standard(),
    )
    .unwrap()
    .0;
    //compare
}
#[test]
fn test_storage_bincode() {
    let mut s = Storage::new();
    s.insert_default::<NameComponent>(0);
    s.insert_default::<StringFieldComponent>(1);
    let res = bincode::encode_to_vec(&s, bincode::config::standard()).unwrap();
    let s2 = bincode::decode_from_slice::<Storage, bincode::config::Configuration>(
        &*res,
        bincode::config::standard(),
    )
    .unwrap();
    let comp2 =
        s2.0.get_components_of_type::<StringFieldComponent>(0)
            .unwrap();
    let comp1 = s.get_components_of_type::<StringFieldComponent>(0).unwrap();
    assert_eq!(comp1.len(), comp2.len());
}

#[test]
fn test_downcast_store() {
    let ccs = CommonComponentStore::<StringFieldComponent>::new();
    let ccs_any = ccs.get_any_owned();
    let ccs_any_downcast = ccs_any.into_store::<StringFieldComponent>();
}
#[test]
fn test_ecomponent_type_from_name() {
    let name = "StringFieldComponent";
    let e_type = EComponentTypes::from_name(name).unwrap();
    assert_eq!(e_type, EComponentTypes::StringFieldComponent);
}
#[test]
fn test_get_type_name_from_store() {
    let ccs = CommonComponentStore::<StringFieldComponent>::new();
    let ccs_any = ccs.get_any_owned();
    let name = ccs_any.get_common_type_name();
    assert_eq!(name, "StringFieldComponent");
}
