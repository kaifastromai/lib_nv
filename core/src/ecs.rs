use super::*;
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
    fn get_owning_entity(&self) -> Option<IndexType>;
    fn set_owning_entity(&mut self, entity: Option<IndexType>);
}

#[derive(Eq, Clone, Copy)]
pub struct Entity {
    _id: IndexType,
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
    pub fn id(&self) -> IndexType {
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
pub struct EntityReferencesComponent {
    _entity: Option<IndexType>,
    pub entity_references: Vec<IndexType>,
}

impl EntityReferencesComponent {
    fn new(_entity: IndexType, entity_references: Vec<IndexType>) -> Self {
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
    pub fn add_entity_reference(&mut self, entity_reference: IndexType) {
        self.entity_references.push(entity_reference);
    }
    pub fn remove_entity_reference(&mut self, entity_reference: IndexType) {
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
    fn get_owning_entity(&self) -> Option<IndexType> {
        self._entity
    }
    fn set_owning_entity(&mut self, entity: Option<IndexType>) {
        self._entity = entity;
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
pub struct FieldsComponent {
    _entity: Option<IndexType>,
    _mask: u16,
    fields: Vec<Field>,
}

impl FieldsComponent {
    pub fn new(_entity: IndexType, name: &'static str, value: &'static str) -> Self {
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
    fn remove_field(&mut self, name: &'static str) {
        self.fields.retain(|field| field.name != name);
    }
    fn get_field(&self, name: &'static str) -> Option<&'static str> {
        self.fields.iter().find(|field| field.name == name).map(|field| field.value)
    }
    fn get_fields(&self) -> &Vec<Field> {
        &self.fields
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
    fn get_owning_entity(&self) -> Option<IndexType> {
        self._entity
    }
    fn set_owning_entity(&mut self, entity: Option<IndexType>) {
        self._entity = entity;
    }
}

pub struct EntityManager {
    entities: HashMap<IndexType, Entity>,
    fields_components: HashMap<IndexType, FieldsComponent>,
    entity_references_components: HashMap<IndexType, EntityReferencesComponent>,
}
impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            entities: HashMap::new(),
            fields_components: HashMap::new(),
            entity_references_components: HashMap::new(),
        }
    }
    pub fn create_entity(&mut self, name: &'static str) -> IndexType {
        let entity = Entity::new(name);
        self.entities.insert(entity.id(), entity);
        entity.id()
    }
    pub fn add_fields_component(
        &mut self,
        entity: IndexType,
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
        entity: IndexType,
        entity_reference: IndexType,
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
    pub fn get_fields_component(&self, entity: IndexType) -> Option<&FieldsComponent> {
        self.fields_components.get(&entity)
    }
    pub fn get_fields_component_mut(&mut self, entity: IndexType) -> Option<&mut FieldsComponent> {
        self.fields_components.get_mut(&entity)
    }

    pub fn get_entity_references_component(
        &self,
        entity: IndexType,
    ) -> Option<&EntityReferencesComponent> {
        self.entity_references_components.get(&entity)
    }
    pub fn get_entity_references_component_mut(
        &mut self,
        entity: IndexType,
    ) -> Option<&mut EntityReferencesComponent> {
        self.entity_references_components.get_mut(&entity)
    }

    pub fn remove_entity_reference_component(
        &mut self,
        entity: IndexType,
        entity_reference: IndexType,
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
    pub fn get_entity_references(&self, entity: IndexType) -> Option<&Vec<IndexType>> {
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
    pub fn get_fields(&self, entity: IndexType) -> Option<&Vec<Field>> {
        //check if entity has fields component
        let entity = self.entities.get(&entity).unwrap();
        if entity.has_component::<FieldsComponent>() {
            let fields_component = self.fields_components.get(&entity.id()).unwrap();
            Some(&fields_component.fields)
        } else {
            None
        }
    }

    pub fn get_entity(&self, entity_index: IndexType) -> Option<&Entity> {
        self.entities.get(&entity_index)
    }
    pub fn get_entity_mut(&mut self, entity_index: IndexType) -> Option<&mut Entity> {
        self.entities.get_mut(&entity_index)
    }
}pub struct Location {

}
pub struct LocationComponent {
    _entity: Option<IndexType>,
    location: Location,
}
impl LocationComponent {
    pub fn new(_entity: IndexType, location: Location) -> Self {
        LocationComponent {
            _entity: Some(_entity),
            location,
        }
    }
}
impl Component for LocationComponent {
    fn get_component_bits() -> u16 {
        LOCATION_COMPONENT_BITS
    }
    fn get_owning_entity(&self) -> Option<IndexType> {
        self._entity
    }
    fn set_owning_entity(&mut self, entity: Option<IndexType>) {
        self._entity = entity;
    }
}
//impl PartialEq and Hash for LocationComponent based on entity id
impl PartialEq for LocationComponent {
    fn eq(&self, other: &Self) -> bool {
        self._entity == other._entity
    }
}
impl std::hash::Hash for LocationComponent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self._entity.hash(state);
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
pub struct Image<'a> {
    id: IndexType,
    name: &'static str,
    description: TextChunk,
    image_data: &'a [u8],
}
pub struct VideoComponent<'a> {
    id: IndexType,
    owning_entity: Option<IndexType>,
    description: TextChunk,
    video_name: &'static str,
    video_type: &'static str,
    video_data: &'a [u8],
}

//impl video component
impl<'a> VideoComponent<'a> {
    pub fn new(
        entity: Option<IndexType>,
        video_name: &'static str,
        video_type: &'static str,
        video_data: &'a [u8],
        description: TextChunk,
    ) -> Self {
        VideoComponent {
            id: Uuid::new_v4().as_u128(),
            owning_entity: entity,
            description,
            video_name,
            video_type,
            video_data,
        }
    }
}
impl Component for VideoComponent<'_> {
    fn get_component_bits() -> u16 {
        VIDEO_COMPONENT_BITS
    }
    fn get_owning_entity(&self) -> Option<IndexType> {
        self.owning_entity
    }
    fn set_owning_entity(&mut self, entity: Option<IndexType>) {
        self.owning_entity = entity;
    }
}

pub struct AudioComponent<'a> {
    id: IndexType,
    owning_entity: Option<IndexType>,
    description: TextChunk,
    audio_name: &'static str,
    audio_type: &'static str,
    audio_data: &'a [u8],
}
//impl audio component
impl<'a> AudioComponent<'a> {
    pub fn new(
        entity: Option<IndexType>,
        audio_name: &'static str,
        audio_type: &'static str,
        audio_data: &'a [u8],
        description: TextChunk,
    ) -> Self {
        AudioComponent {
            id: Uuid::new_v4().as_u128(),
            owning_entity: entity,
            description,
            audio_name,
            audio_type,
            audio_data,
        }
    }
}
impl Component for AudioComponent<'_> {
    fn get_component_bits() -> u16 {
        AUDIO_COMPONENT_BITS
    }
    fn get_owning_entity(&self) -> Option<IndexType> {
        self.owning_entity
    }
    fn set_owning_entity(&mut self, entity: Option<IndexType>) {
        self.owning_entity = entity;
    }
}
//impl PartialEq for AudioComponent based on entity id and then implement Hash
impl PartialEq for AudioComponent<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.get_owning_entity() == other.get_owning_entity()
    }
}
impl std::hash::Hash for AudioComponent<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_owning_entity().hash(state);
    }
}

pub struct NameComponent {
    owning_entity: Option<IndexType>,
    name: &'static str,
    aliases: Vec<&'static str>,
}
impl NameComponent {
    pub fn new(_entity: IndexType, name: &'static str) -> Self {
        NameComponent {
            owning_entity: Some(_entity),
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
    fn get_owning_entity(&self) -> Option<IndexType> {
        self.owning_entity
    }
    fn set_owning_entity(&mut self, entity: Option<IndexType>) {
        self.owning_entity = entity;
    }
}

//impl PartialEq and Hash for NameComponent based on entity id
impl PartialEq for NameComponent {
    fn eq(&self, other: &Self) -> bool {
        self.owning_entity == other.owning_entity
    }
}
//impl Hash for NameComponent based on entity id
impl std::hash::Hash for NameComponent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.owning_entity.hash(state);
    }
}

pub struct BinaryDataComponent {
    id: IndexType,
    owning_entity: Option<IndexType>,
    data: Vec<u8>,
}

impl BinaryDataComponent {
    pub fn new(_entity: IndexType, data: Vec<u8>) -> Self {
        BinaryDataComponent {
            id: Uuid::new_v4().as_u128(),
            owning_entity: Some(_entity),
            data,
        }
    }
}
//implement Component for BinaryDataComponent
impl Component for BinaryDataComponent {
    fn get_component_bits() -> u16 {
        BINARY_DATA_COMPONENT_BITS
    }
    fn get_owning_entity(&self) -> Option<IndexType> {
        self.owning_entity
    }
    fn set_owning_entity(&mut self, entity: Option<IndexType>) {
        self.owning_entity = entity;
    }
}
//impl PartialEq and Hash for BinaryDataComponent based on entity id
impl PartialEq for BinaryDataComponent {
    fn eq(&self, other: &Self) -> bool {
        self.owning_entity == other.owning_entity
    }
}
//impl Hash for BinaryDataComponent based on entity id
impl std::hash::Hash for BinaryDataComponent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.owning_entity.hash(state);
    }
}
