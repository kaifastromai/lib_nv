use std::{collections::BTreeSet, fmt::Debug};

use super::*;
type EntitySignature = u128;
use nvproc::{self, component, gen_components, Component};

#[gen_components]
pub mod components {
    use super::*;
    pub struct Fields {
        name: String,
        fields: Vec<Field>,
    }
    pub struct Video {
        description: TextChunk,
        video_name: String,
        video_type: String,
        video_data: Vec<u8>,
    }

    pub struct Audio {
        description: TextChunk,
        audio_name: String,
        audio_type: String,
        audio_data: Vec<u8>,
    }
    pub struct Name {
        name: &'static str,
        aliases: Vec<&'static str>,
    }
    impl Name {
        pub fn add_alias(&mut self, alias: &'static str) {
            self.aliases.push(alias);
        }
    }
    pub struct BinaryData {
        data: Vec<u8>,
    }
    pub struct Image {
        name: &'static str,
        description: TextChunk,
        image_data: Vec<u8>,
    }

    pub struct References {}
}

pub trait Component {
    type Properties;
    fn new(owning_entity: IndexType, props: Self::Properties) -> Self;
    fn get_owning_entity(&self) -> IndexType;
    fn set_owning_entity(&mut self, entity: IndexType);
    fn set_is_deleted(&mut self, is_deleted: bool);
    fn get_is_deleted(&self) -> bool;
    fn get_type() -> components::ComponentType;
}

#[derive(Eq, Clone)]
pub struct Entity {
    _id: IndexType,
    pub entity_class: String,
    pub signature: BTreeSet<components::ComponentType>,
}
//implement partialeq for entity based on id
impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self._id == other._id
    }
}
//implement hash for entity based on id
impl std::hash::Hash for Entity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self._id.hash(state);
    }
}

impl<'a> Entity {
    pub fn new(entity_class: &str) -> Self {
        Entity {
            _id: Uuid::new_v4().as_u128(),
            entity_class: String::from(entity_class),
            signature: BTreeSet::new(),
        }
    }
    pub fn id(&self) -> IndexType {
        self._id
    }
    pub fn add_component<T: Component>(&mut self) {
        self.signature.insert(T::get_type());
    }
    pub fn remove_component<T: Component>(&mut self) {
        self.signature.remove(&T::get_type());
    }
    pub fn has_component<T: Component>(&self) -> bool {
        self.signature.contains(&T::get_type())
    }
    pub fn get_signature(&self) -> Vec<&components::ComponentType> {
        self.signature.iter().collect::<Vec<_>>()
    }
}

pub struct Field {
    name: &'static str,
    value: &'static str,
}

impl Field {
    fn new(name: &'static str, value: &'static str) -> Self {
        Self { name, value }
    }
}

// impl components::Fields {
//     pub fn add_field(&mut self, name: &'static str, value: &'static str) {
//         self.fields.push(Field { name, value });
//     }
//     pub fn add_field_struct(&mut self, field: Field) {
//         self.fields.push(field);
//     }
//     fn remove_field(&mut self, name: &'static str) {
//         self.fields.retain(|field| field.name != name);
//     }
//     fn get_field(&self, name: &'static str) -> Option<&'static str> {
//         self.fields
//             .iter()
//             .find(|field| field.name == name)
//             .map(|field| field.value)
//     }
//     fn get_fields(&self) -> &Vec<Field> {
//         &self.fields
//     }
// }

pub struct EntityManager {
    entities: HashMap<IndexType, Entity>,
    components: components::Components,
}
impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            entities: HashMap::new(),
            components: Default::default(),
        }
    }
    pub fn create_entity(&mut self, name: String) -> IndexType {
        let entity: &Entity = &Entity::new(name.as_str());
        self.entities.insert(entity.id(), entity.clone());
        entity.id()
    }
    pub fn add_component<T: Component>(&mut self, entity: IndexType, props: T::Properties) {
        let entity = self.entities.get_mut(&entity).unwrap();
        entity.add_component::<T>();
        //create actual component
        let component = T::new(entity.id(), props);
    }
    // pub fn add_fields_component(
    //     &mut self,
    //     entity: IndexType,
    //     name: &'static str,
    //     value: &'static str,
    // ) {
    //     let mut fields_component = FieldsComponent::new(entity, name, value);
    //     self.entities
    //         .get_mut(&entity)
    //         .unwrap()
    //         .add_component(&mut fields_component);
    //     self.fields_components.insert(entity, fields_component);
    // }
    // pub fn add_entity_reference_component(
    //     &mut self,
    //     entity: IndexType,
    //     entity_reference: IndexType,
    // ) {
    //     let mut entity_references_component =
    //         EntityReferencesComponent::new(entity, vec![entity_reference]);
    //     self.entities
    //         .get_mut(&entity)
    //         .unwrap()
    //         .add_component(&mut entity_references_component);
    //     self.entity_references_components
    //         .insert(entity, entity_references_component);
    // }
    // pub fn get_fields_component(&self, entity: IndexType) -> Option<&FieldsComponent> {
    //     self.fields_components.get(&entity)
    // }
    // pub fn get_fields_component_mut(&mut self, entity: IndexType) -> Option<&mut FieldsComponent> {
    //     self.fields_components.get_mut(&entity)
    // }

    // pub fn get_entity_references_component(
    //     &self,
    //     entity: IndexType,
    // ) -> Option<&EntityReferencesComponent> {
    //     self.entity_references_components.get(&entity)
    // }
    // pub fn get_entity_references_component_mut(
    //     &mut self,
    //     entity: IndexType,
    // ) -> Option<&mut EntityReferencesComponent> {
    //     self.entity_references_components.get_mut(&entity)
    // }

    // pub fn remove_entity_reference_component(
    //     &mut self,
    //     entity: IndexType,
    //     entity_reference: IndexType,
    // ) {
    //     let mut entity_references_component =
    //         self.entity_references_components.get_mut(&entity).unwrap();
    //     entity_references_component.remove_entity_reference(entity_reference);
    //     if entity_references_component.entity_references.is_empty() {
    //         self.entities
    //             .get_mut(&entity)
    //             .unwrap()
    //             .remove_component(entity_references_component);
    //         self.entity_references_components.remove(&entity);
    //     }
    // }
    // pub fn get_entity_references(&self, entity: IndexType) -> Option<&Vec<IndexType>> {
    //     //check if entity has entity references component
    //     let entity = self.entities.get(&entity).unwrap();
    //     if entity.has_component::<EntityReferencesComponent>() {
    //         let entity_references_component =
    //             self.entity_references_components.get(&entity.id()).unwrap();
    //         Some(&entity_references_component.entity_references)
    //     } else {
    //         None
    //     }
    // }
    // pub fn get_fields(&self, entity: IndexType) -> Option<&Vec<Field>> {
    //     //check if entity has fields component
    //     let entity = self.entities.get(&entity).unwrap();
    //     if entity.has_component::<FieldsComponent>() {
    //         let fields_component = self.fields_components.get(&entity.id()).unwrap();
    //         Some(&fields_component.fields)
    //     } else {
    //         None
    //     }
    // }

    pub fn get_entity(&self, entity_index: IndexType) -> Option<&Entity> {
        self.entities.get(&entity_index)
    }
    pub fn get_entity_mut(&mut self, entity_index: IndexType) -> Option<&mut Entity> {
        self.entities.get_mut(&entity_index)
    }
}

pub struct TimeUnit<'a> {
    name: &'static str,
    related_unit: &'a TimeUnit<'a>,
    related_unit_multiplier: f32,
}
pub trait TimeSystem {
    fn walk_time_unit(&mut self) -> Vec<&TimeUnit>;
}
pub trait TimeTrait {
    type TimeSystem: TimeSystem;
}
pub struct Time {}
pub trait EventType {
    fn get_event(&self) -> &dyn EventType;
}
struct EventTypeDuration {
    start: Time,
    end: Time,
}
impl EventTypeDuration {
    fn new(start: Time, end: Time) -> Self {
        EventTypeDuration { start, end }
    }
}
impl EventType for EventTypeDuration {
    fn get_event(&self) -> &dyn EventType {
        self
    }
}

struct EventTypeMoment {
    moment: Time,
}
impl EventTypeMoment {
    fn new(moment: Time) -> Self {
        EventTypeMoment { moment }
    }
}
impl EventType for EventTypeMoment {
    fn get_event(&self) -> &dyn EventType {
        self
    }
}
pub struct Event<T: EventType> {
    name: &'static str,
    description: TextChunk,
    time: Time,
    involved_entities: Vec<IndexType>,
    event_type: T,
}
