use std::collections::HashMap;

use num::Integer;
use uuid::Uuid;

type EntityIndex = u128;
type EntitySignature = u16;
const FIELD_COMPONENT_BITS: u16 = 0x1;
const ENTITY_REFERENCE_BITS: u16 = 0x2;
const NAME_COMPONENT_BITS: u16 = 0x3;
const AUDIO_COMPONENT_BITS: u16 = 0x4;
const VIDEO_COMPONENT_BITS: u16 = 0x5;
const BINARY_DATA_COMPONENT_BITS: u16 = 0x6;
const LOCATION_COMPONENT_BITS: u16 = 0x7;

pub trait Component {
    fn get_component_bits() -> u16;
    fn get_owning_entity(&self) -> Option<EntityIndex>;
    fn set_owning_entity(&mut self, entity: Option<EntityIndex>);
}

#[derive(Eq, Clone, Copy)]
struct Entity {
    _id: EntityIndex,
    pub name: &'static str,
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
    pub fn new(name: &'static str) -> Self {
        Entity {
            _id: Uuid::new_v4().as_u128(),
            name,
            signature: 0,
        }
    }
    pub fn id(&self) -> EntityIndex {
        self._id
    }
    pub fn add_component<T: Component>(&mut self, component: &mut T) {
        self.signature |= T::get_component_bits();
        component.set_owning_entity(Some(self.id()));
    }
    pub fn remove_component<T: Component>(&mut self, component: &mut T) {
        self.signature &= !T::get_component_bits();
        component.set_owning_entity(None);
    }
    pub fn has_component<T: Component>(&self) -> bool {
        self.signature & T::get_component_bits() != 0
    }
    pub fn get_signature(&self) -> EntitySignature {
        self.signature
    }
}
struct EntityReferencesComponent {
    _entity: Option<EntityIndex>,
    pub entity_references: Vec<EntityIndex>,
}

impl EntityReferencesComponent {
    fn new(_entity: EntityIndex, entity_references: Vec<EntityIndex>) -> Self {
        Self {
            _entity: Some(_entity),
            entity_references,
        }
    }
    pub fn new_empty() -> Self {
        Self {
            _entity: None,
            entity_references: vec![],
        }
    }
    pub fn add_entity_reference(&mut self, entity_reference: EntityIndex) {
        self.entity_references.push(entity_reference);
    }
    pub fn remove_entity_reference(&mut self, entity_reference: EntityIndex) {
        self.entity_references.retain(|&x| x != entity_reference);
    }
}
//impl PartialEq and Hash for EntityReferenceComponent based on entity id
impl PartialEq for EntityReferencesComponent {
    fn eq(&self, other: &Self) -> bool {
        self._entity == other._entity
    }
}
impl std::hash::Hash for EntityReferencesComponent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self._entity.hash(state);
    }
}
impl Component for EntityReferencesComponent {
    fn get_component_bits() -> u16 {
        ENTITY_REFERENCE_BITS
    }
    fn get_owning_entity(&self) -> Option<EntityIndex> {
        self._entity
    }
    fn set_owning_entity(&mut self, entity: Option<EntityIndex>) {
        self._entity = entity;
    }
}

struct Field {
    name: &'static str,
    value: &'static str,
}

impl Field {
    fn new(name: &'static str, value: &'static str) -> Self {
        Self { name, value }
    }
}
struct FieldsComponent {
    _entity: Option<EntityIndex>,
    _mask: u16,
    fields: Vec<Field>,
}

impl FieldsComponent {
    pub fn new(_entity: EntityIndex, name: &'static str, value: &'static str) -> Self {
        FieldsComponent {
            _entity: Some(_entity),
            _mask: FIELD_COMPONENT_BITS,
            fields: Vec::new(),
        }
    }

    pub fn add_field(&mut self, name: &'static str, value: &'static str) {
        self.fields.push(Field { name, value });
    }
    pub fn add_field_struct(&mut self, field: Field) {
        self.fields.push(field);
    }
}
//impl PartialEq and Hash for FieldsComponent based on entity id
impl PartialEq for FieldsComponent {
    fn eq(&self, other: &Self) -> bool {
        self._entity == other._entity
    }
}
impl std::hash::Hash for FieldsComponent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self._entity.hash(state);
    }
}

impl Component for FieldsComponent {
    fn get_component_bits() -> u16 {
        FIELD_COMPONENT_BITS
    }
    fn get_owning_entity(&self) -> Option<EntityIndex> {
        self._entity
    }
    fn set_owning_entity(&mut self, entity: Option<EntityIndex>) {
        self._entity = entity;
    }
}

struct EntityManager {
    entities: HashMap<EntityIndex, Entity>,
    fields_components: HashMap<EntityIndex, FieldsComponent>,
    entity_references_components: HashMap<EntityIndex, EntityReferencesComponent>,
}
impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            entities: HashMap::new(),
            fields_components: HashMap::new(),
            entity_references_components: HashMap::new(),
        }
    }
    pub fn create_entity(&mut self, name: &'static str) -> EntityIndex {
        let entity = Entity::new(name);
        self.entities.insert(entity.id(), entity);
        entity.id()
    }
    pub fn add_fields_component(
        &mut self,
        entity: EntityIndex,
        name: &'static str,
        value: &'static str,
    ) {
        let mut fields_component = FieldsComponent::new(entity, name, value);
        self.entities
            .get_mut(&entity)
            .unwrap()
            .add_component(&mut fields_component);
        self.fields_components.insert(entity, fields_component);
    }
    pub fn add_entity_reference_component(
        &mut self,
        entity: EntityIndex,
        entity_reference: EntityIndex,
    ) {
        let mut entity_references_component =
            EntityReferencesComponent::new(entity, vec![entity_reference]);
        self.entities
            .get_mut(&entity)
            .unwrap()
            .add_component(&mut entity_references_component);
        self.entity_references_components
            .insert(entity, entity_references_component);
    }
    pub fn get_fields_component(&self, entity: EntityIndex) -> Option<&FieldsComponent> {
        self.fields_components.get(&entity)
    }
    pub fn get_fields_component_mut(
        &mut self,
        entity: EntityIndex,
    ) -> Option<&mut FieldsComponent> {
        self.fields_components.get_mut(&entity)
    }

    pub fn get_entity_references_component(
        &self,
        entity: EntityIndex,
    ) -> Option<&EntityReferencesComponent> {
        self.entity_references_components.get(&entity)
    }
    pub fn get_entity_references_component_mut(
        &mut self,
        entity: EntityIndex,
    ) -> Option<&mut EntityReferencesComponent> {
        self.entity_references_components.get_mut(&entity)
    }

    pub fn remove_entity_reference_component(
        &mut self,
        entity: EntityIndex,
        entity_reference: EntityIndex,
    ) {
        let mut entity_references_component =
            self.entity_references_components.get_mut(&entity).unwrap();
        entity_references_component.remove_entity_reference(entity_reference);
        if entity_references_component.entity_references.is_empty() {
            self.entities
                .get_mut(&entity)
                .unwrap()
                .remove_component(entity_references_component);
            self.entity_references_components.remove(&entity);
        }
    }
    pub fn get_entity_references(&self, entity: EntityIndex) -> Option<&Vec<EntityIndex>> {
        //check if entity has entity references component
        let entity = self.entities.get(&entity).unwrap();
        if entity.has_component::<EntityReferencesComponent>() {
            let entity_references_component =
                self.entity_references_components.get(&entity.id()).unwrap();
            Some(&entity_references_component.entity_references)
        } else {
            None
        }
    }
    pub fn get_fields(&self, entity: EntityIndex) -> Option<&Vec<Field>> {
        //check if entity has fields component
        let entity = self.entities.get(&entity).unwrap();
        if entity.has_component::<FieldsComponent>() {
            let fields_component = self.fields_components.get(&entity.id()).unwrap();
            Some(&fields_component.fields)
        } else {
            None
        }
    }

    pub fn get_entity(&self, entity_index: EntityIndex) -> Option<&Entity> {
        self.entities.get(&entity_index)
    }
    pub fn get_entity_mut(&mut self, entity_index: EntityIndex) -> Option<&mut Entity> {
        self.entities.get_mut(&entity_index)
    }
}
pub struct Timeline {}
pub struct Arc {}
pub struct Scene {}
pub struct LocationComponent {}
pub struct Event {}
pub struct Image {}
pub struct VideoComponent<'a> {
    _entity: Option<EntityIndex>,
    _mask: u16,
    video_name: &'static str,
    video_type: &'static str,
    video_data: &'a [u8],
}

//impl video component
impl<'a> VideoComponent<'a> {
    pub fn new(
        entity: Option<EntityIndex>,
        mask: u16,
        video_name: &'static str,
        video_type: &'static str,
        video_data: &'a [u8],
    ) -> Self {
        VideoComponent {
            _entity: entity,
            _mask: mask,
            video_name,
            video_type,
            video_data,
        }
    }
}

pub struct AudioComponent<'a> {
    _entity: Option<EntityIndex>,
    audio_name: &'static str,
    audio_type: &'static str,
    audio_data: &'a [u8],
}
//impl audio component
impl<'a> AudioComponent<'a> {
    pub fn new(
        entity: Option<EntityIndex>,
        audio_name: &'static str,
        audio_type: &'static str,
        audio_data: &'a [u8],
    ) -> Self {
        AudioComponent {
            _entity: entity,
            audio_name,
            audio_type,
            audio_data,
        }
    }
}
pub struct NameComponent {
    _entity: Option<EntityIndex>,
    name: &'static str,
    aliases: Vec<&'static str>,
}
impl NameComponent {
    pub fn new(_entity: EntityIndex, name: &'static str) -> Self {
        NameComponent {
            _entity: Some(_entity),
            name,
            aliases: Vec::new(),
        }
    }
    pub fn add_alias(&mut self, alias: &'static str) {
        self.aliases.push(alias);
    }
}
//implement Component for NameComponent
impl Component for NameComponent {
    fn get_component_bits() -> u16 {
        NAME_COMPONENT_BITS
    }
    fn get_owning_entity(&self) -> Option<EntityIndex> {
        self._entity
    }
    fn set_owning_entity(&mut self, entity: Option<EntityIndex>) {
        self._entity = entity;
    }
}
pub struct BinaryDataComponent {
    _entity: Option<EntityIndex>,
    data: Vec<u8>,
}
impl BinaryDataComponent {
    pub fn new(_entity: EntityIndex, data: Vec<u8>) -> Self {
        BinaryDataComponent {
            _entity: Some(_entity),
            data,
        }
    }
}
//implement Component for BinaryDataComponent
impl Component for BinaryDataComponent {
    fn get_component_bits() -> u16 {
        BINARY_DATA_COMPONENT_BITS
    }
    fn get_owning_entity(&self) -> Option<EntityIndex> {
        self._entity
    }
    fn set_owning_entity(&mut self, entity: Option<EntityIndex>) {
        self._entity = entity;
    }
}

pub struct TextFile {}
pub struct Map {}
pub struct Progression<'a> {
    prev_progression: &'a Progression<'a>,
    next_progression: &'a Progression<'a>,
}
pub struct TextChunk<'a> {
    buffer: &'a String,
}
#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_add_entity() {
        let mut entity_manager = EntityManager::new();
        let entity_indx = entity_manager.create_entity("test");
        let entity = entity_manager.get_entity(entity_indx).unwrap();

        assert_eq!(entity.name, "test");
    }
    #[test]
    fn test_add_fields_component() {
        let mut entity_manager = EntityManager::new();
        let entity_indx = entity_manager.create_entity("test");
        entity_manager.add_fields_component(entity_indx, "name", "value");
        let mut entity = entity_manager.get_entity(entity_indx).unwrap();
        let mut fields_component = entity_manager
            .get_fields_component_mut(entity_indx)
            .unwrap();
        fields_component.add_field("name", "value");
        assert_eq!(fields_component.fields[0].name, "name");
    }
}
mod utilies {
    use core::num;

    use ::num::Integer;

    struct BitSet<T: Integer> {
        _bits: T,
    }
}
