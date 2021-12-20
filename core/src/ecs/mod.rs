mod archetypes;
pub mod prelude;

use std::{collections::BTreeSet, fmt::Debug};

use super::*;
type EntitySignature = u128;
use nvproc::{self, component, gen_components, Component};
pub struct Field {
    name: String,
    value: String,
}
pub struct Hierarchy{
    
}

#[gen_components]
pub mod components {
    use super::*;
    pub struct Fields {
        name: String,
        fields: Vec<Field>,
    }
    impl components::Fields {
        pub fn add_field(&mut self, name: &str, value: &str) {
            self.fields.push(Field {
                name: String::from(name),
                value: String::from(value),
            });
        }
        pub fn add_field_struct(&mut self, field: Field) {
            self.fields.push(field);
        }
        fn remove_field(&mut self, name: &'static str) {
            self.fields.retain(|field| field.name != name);
        }
        fn get_fields(&self) -> &Vec<Field> {
            &self.fields
        }
    }
    //A component that describes a hierachical relationship between entities (family members, Locations (country->city->town)), races, etc...
    pub struct HierarchichalTree{
        

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
        pub name: &'static str,
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

    pub struct References {
        entity_references: Vec<IndexType>,
    }
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

impl Field {
    fn new(name: &str, value: &str) -> Self {
        Self {
            name: String::from(name),
            value: String::from(value),
        }
    }
}

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
    pub fn create_entity(&mut self, entity_class: String) -> IndexType {
        let entity: &Entity = &Entity::new(entity_class.as_str());
        self.entities.insert(entity.id(), entity.clone());
        entity.id()
    }
    pub fn add_component<T: Component>(&mut self, entity: IndexType, props: T::Properties) {
        let entity_ref = self.entities.get_mut(&entity).unwrap();
        entity_ref.add_component::<T>();
        //create actual component
        let component = T::new(entity_ref.id(), props);
        //add component to entity
        let c = self.components.get_mut::<T>();
        c.insert(entity, component);
    }
    pub fn get_component<T: Component>(&self, entity: IndexType) -> Option<&T> {
        let c = self.components.get::<T>();
        c.get(&entity)
    }
    pub fn get_component_mut<T: Component>(&mut self, entity: IndexType) -> Option<&mut T> {
        let c = self.components.get_mut::<T>();
        c.get_mut(&entity)
    }

    pub fn get_entity(&self, entity_index: IndexType) -> Option<&Entity> {
        self.entities.get(&entity_index)
    }
    pub fn get_entity_mut(&mut self, entity_index: IndexType) -> Option<&mut Entity> {
        self.entities.get_mut(&entity_index)
    }
    pub fn get_entities_by_class(&self, entity_class: &str) -> Vec<&Entity> {
        self.entities
            .values()
            .filter(|entity| entity.entity_class == entity_class)
            .collect::<Vec<_>>()
    }
    pub fn get_entities_by_class_mut(&mut self, entity_class: &str) -> Vec<&mut Entity> {
        self.entities
            .values_mut()
            .filter(|entity| entity.entity_class == entity_class)
            .collect::<Vec<_>>()
    }
    pub fn get_all_entities(&self) -> Vec<&Entity> {
        self.entities.values().collect::<Vec<_>>()
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
