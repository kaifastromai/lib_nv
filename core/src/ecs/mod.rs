mod archetypes;
pub mod prelude;

use super::*;
pub use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, fmt::Debug};

use utils::prelude::*;
pub type EntitySignature = BTreeSet<components::ComponentType>;

use nvproc::{self, component, gen_components, Component};
pub trait Relationship {
    type RelationshipType;
    fn set_relationship(&mut self, r: Self::RelationshipType);
    fn get_relationship(&self) -> &Self::RelationshipType;
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
pub enum EFamilyRelationshipType {
    Father,
    Mother,
    Sister,
    Brother,
}
#[derive(Clone, serde::Serialize, serde::Deserialize, Copy)]
pub enum ERelationGradient {
    Positive,
    Negative,
    Neutral,
}
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct FamilyRelationshipType {
    pub relation_type: EFamilyRelationshipType,
    pub gradient: ERelationGradient,
}
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct StringRelationshipType {
    pub kind: String,
    pub gradient: ERelationGradient,
}
#[derive(serde::Serialize, serde::Deserialize, Clone)]

pub struct CustomRelationship {
    pub kind: StringRelationshipType,
}
impl Relationship for CustomRelationship {
    type RelationshipType = StringRelationshipType;
    fn set_relationship(&mut self, r: Self::RelationshipType) {
        self.kind = r;
    }
    fn get_relationship(&self) -> &Self::RelationshipType {
        &self.kind
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct FamilyRelationship {
    kind: FamilyRelationshipType,
}
impl Relationship for FamilyRelationship {
    type RelationshipType = EFamilyRelationshipType;
    fn set_relationship(&mut self, r: Self::RelationshipType) {
        self.kind = FamilyRelationshipType {
            relation_type: r,
            gradient: match r {
                EFamilyRelationshipType::Father | EFamilyRelationshipType::Mother => {
                    ERelationGradient::Positive
                }
                EFamilyRelationshipType::Sister | EFamilyRelationshipType::Brother => {
                    ERelationGradient::Neutral
                }
            },
        };
    }
    fn get_relationship(&self) -> &Self::RelationshipType {
        &self.kind.relation_type
    }
}

pub struct Hierarchy<R: Relationship> {
    graph: DiGraph<(IndexType, u32), R>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone)]

pub struct Field {
    name: String,
    value: String,
}
#[derive(serde::Serialize, Deserialize, Clone)]
pub struct Video {
    description: String,
    video_name: String,
    video_type: String,
    video_data: Vec<u8>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Audio {
    description: String,
    audio_name: String,
    audio_type: String,
    audio_data: Vec<u8>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Image {
    name: String,
    description: String,
    image_data: Vec<u8>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct BinaryDatum {
    data: Vec<u8>,
}

#[gen_components]
pub mod components {
    use super::*;
    pub struct Fields {
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
        pub fn get_fields(&self) -> &Vec<Field> {
            &self.fields
        }
    }

    //A component that describes a hierachical relationship between entities (family members, Locations (country->city->town)), races, etc...
    pub struct FamilyRelationship {
        relationships: Vec<(IndexType, EFamilyRelationshipType)>,
    }
    pub struct CustomRelationship {
        relationships: Vec<(IndexType, StringRelationshipType)>,
    }

    pub struct Videos {
        vidoes: Vec<Video>,
    }
    pub struct Audios {
        audios: Vec<Audio>,
    }

    pub struct Names {
        pub name: Vec<String>,
    }

    pub struct BinaryData {
        data: Vec<BinaryDatum>,
    }

    pub struct Images {
        images: Vec<Image>,
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
    _is_deleted: bool,
    pub entity_class: String,
    pub signature: EntitySignature,
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
            _is_deleted: false,
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
    pub fn replace_signature(&mut self, signature: BTreeSet<components::ComponentType>) {
        self.signature = signature;
    }
    fn mark_for_deletion(&mut self) {
        self._is_deleted = true;
    }
    pub fn is_marked_for_deletion(&self) -> bool {
        self._is_deleted
    }
    fn unmark_for_deletion(&mut self) {
        self._is_deleted = false;
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

#[derive(Default)]
pub struct EntityGraph {
    pub entities: Vec<Entity>,
    pub components: Components,
}
#[derive(Clone)]
pub struct EntityManager {
    entities: HashMap<IndexType, Entity>,
    components: Components,
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
    pub fn delete_entity(&mut self, entity_id: IndexType) {
        self.strip_entity(entity_id);
        match self.entities.remove(&entity_id) {
            Some(entity) => {}
            None => {
                println!("Entity with id {} does not exist", entity_id);
            }
        }
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
        let ret = c.get(&entity);
        match ret {
            Some(comp) => {
                if comp.get_is_deleted() {
                    None
                } else {
                    Some(comp)
                }
            }
            None => None,
        }
    }
    pub fn get_component_mut<'a, T: Component>(
        &'a mut self,
        entity: IndexType,
    ) -> Option<&'a mut T> {
        let c = self.components.get_mut::<'a, T>();
        let ret = c.get_mut(&entity);
        match ret {
            Some(comp) => {
                if comp.get_is_deleted() {
                    None
                } else {
                    Some(comp)
                }
            }
            None => None,
        }
    }

    pub fn get_entity(&self, entity_index: IndexType) -> Option<&Entity> {
        let e = self.entities.get(&entity_index);
        match e {
            Some(entity) => {
                if entity.is_marked_for_deletion() {
                    None
                } else {
                    Some(entity)
                }
            }
            None => None,
        }
    }
    pub fn get_entity_mut(&mut self, entity_index: IndexType) -> Option<&mut Entity> {
        let e = self.entities.get_mut(&entity_index);
        match e {
            Some(entity) => {
                if entity.is_marked_for_deletion() {
                    None
                } else {
                    Some(entity)
                }
            }
            None => None,
        }
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
    //Gets all currently living entities
    pub fn get_all_entities(&self) -> Vec<&Entity> {
        //Iterate and only return entities that are not marked for deletion
        self.entities
            .values()
            .filter(|entity| !entity.is_marked_for_deletion())
            .collect()
    }
    pub fn add_from_entity_graph(&mut self, entity_graph: EntityGraph) {
        for entity in entity_graph.entities.iter() {
            self.entities.insert(entity.id(), entity.clone());
        }
        self.components.merge(entity_graph.components);
    }
    pub fn merge_components(&mut self, components: Components) {
        let s = &String::new();
        let s2 = s.clone();
        self.components.merge(components);
    }
    ///Removes all components from the entity
    pub fn strip_entity(&mut self, entity: IndexType) {
        let entity_ref = self
            .entities
            .get_mut(&entity)
            .expect("Entity does not exist");
        entity_ref.signature.clear();
        self.components.delete_components(entity);
    }
    //Returns a new Components object with all components of the given entity
    pub fn get_components(&self, entity: IndexType) -> Components {
        self.components.get_components(entity)
    }
    pub fn mark_entity_for_deletion(&mut self, entity: IndexType) {
        let entity_ref = self.entities.get_mut(&entity).unwrap();
        entity_ref.mark_for_deletion();
        //mark all components of the entity for deletion
        entity_ref.mark_for_deletion();
        self.components.set_mark_for_deletion(entity, true);
    }
    pub fn unmark_entity_for_deletion(&mut self, entity: IndexType) {
        let entity_ref = self.entities.get_mut(&entity).unwrap();
        entity_ref.unmark_for_deletion();
        self.components.set_mark_for_deletion(entity, false);
    }
    pub fn get_components_ref<'a>(
        &'a self,
        entity_id: IndexType,
    ) -> Result<Vec<ComponentRef<'a>>, &'static str> {
        let res = self.components.get_components_ref(entity_id)?;
        Ok(res)
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

#[cfg(test)]
mod test;
