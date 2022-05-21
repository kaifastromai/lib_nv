pub mod component;
pub mod prelude;
pub mod query;
mod tests;
use self::component::archetypes::{Archetype, ArchetypeTy};
use crate::ecs::query::*;

use super::*;
use crate::ecs::component::*;
use common::{
    exports::{serde::ser::SerializeMap, *},
    type_id::*,
    type_name_any, uuid,
};
use component::components::*;
pub use serde::{Deserialize, Serialize};

use anyhow::{anyhow, Result};
use std::{
    any::Any,
    collections::{BTreeSet, HashMap},
    fmt::Display,
};
pub trait ComponentTypeIdTy: TypeIdTy {}

pub struct ComponentTypeId(TypeId);
impl ComponentTypeIdTy for ComponentTypeId {}

pub type Id = u128;
#[nvproc::bincode_derive]
#[derive(Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ComponentId {
    id: u128,
}

//impl deref for ComponentId that returns the inner type
impl std::ops::Deref for ComponentId {
    type Target = u128;
    fn deref(&self) -> &Self::Target {
        &self.id
    }
}
//impl from u128 for ComponentId
impl From<u128> for ComponentId {
    fn from(id: u128) -> Self {
        ComponentId { id }
    }
}
//implement display for ComponentId
impl Display for ComponentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.id)
    }
}

impl From<ComponentId> for u128 {
    fn from(id: ComponentId) -> Self {
        id.id
    }
}
//A component type.
pub trait ComponentTy: 'static {
    fn get_component_type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
    fn get_component_type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
    fn get_component_name(&self) -> &'static str;
    //Prepare this component to be used by a new entity
    fn clean(&mut self);
    fn get_any(&self) -> &dyn Any;
    fn get_any_mut(&mut self) -> &mut dyn Any;
    fn get_component_type(&self) -> EComponentTypes;
    fn serialize(&self) -> Result<Vec<u8>> {
        todo!()
    }
}

pub trait ComponentTyReqs: 'static + Clone + ComponentTy {
    fn get_req_component_type_id() -> TypeId {
        TypeId::of::<Self>()
    }
    fn get_req_type_name() -> &'static str {
        std::any::type_name::<Self>()
    }
}
impl<T: ComponentTy + Clone> ComponentTyReqs for T {}
impl ComponentTy for () {
    fn get_component_type_id(&self) -> TypeId {
        TypeId::of::<()>()
    }
    fn get_component_type_name(&self) -> &'static str {
        std::any::type_name::<()>()
    }
    fn get_component_name(&self) -> &'static str {
        "()"
    }
    fn get_any(&self) -> &dyn Any {
        self
    }
    fn get_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn clean(&mut self) {}
    fn get_component_type(&self) -> EComponentTypes {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[nvproc::bincode_derive]
#[nvproc::serde_derive]
pub struct Signature(Vec<TypeId>);
impl Signature {
    pub fn new() -> Self {
        Signature(Vec::new())
    }
    pub fn add(&mut self, type_id: TypeId) {
        self.0.push(type_id);
    }
    pub fn get_type_ids(&self) -> &Vec<TypeId> {
        &self.0
    }
    pub fn get_type_ids_mut(&mut self) -> &mut Vec<TypeId> {
        &mut self.0
    }
    //remove the first occurence of type_id
    pub fn remove_component(&mut self, type_id: TypeId) -> Result<()> {
        let index = self
            .0
            .iter()
            .position(|&x| x == type_id)
            .ok_or(anyhow!("Could not find component type {:?}", type_id))?;
        self.0.remove(index);
        Ok(())
    }
    pub fn contains(&self, type_id: TypeId) -> bool {
        self.0.contains(&type_id)
    }
    pub fn merge(&mut self, other: &Signature) {
        self.0.extend(other.0.iter().cloned());
    }
}
impl From<Vec<TypeId>> for Signature {
    fn from(vec: Vec<TypeId>) -> Self {
        Signature(vec)
    }
}
impl From<TypeId> for Signature {
    fn from(type_id: TypeId) -> Self {
        Signature(vec![type_id])
    }
}

#[derive(Debug)]
#[nvproc::bincode_derive]
pub struct Entity {
    id: Id,
    is_alive: bool,
    sig: Signature,
}
impl Entity {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            is_alive: true,
            sig: Signature::new(),
        }
    }
    pub fn from_sig(id: Id, sig: Signature) -> Self {
        Self {
            id,
            is_alive: true,
            sig,
        }
    }
    pub fn is_valid(&self) -> bool {
        self.is_alive
    }
    pub fn get_id(&self) -> Id {
        self.id
    }
    pub fn add_component<T: ComponentTyReqs>(&mut self) -> Result<()> {
        let tid = TypeId::of::<T>();
        self.sig.add(tid);

        Ok(())
    }
    pub fn remove_component<T: ComponentTyReqs>(&mut self) -> Result<()> {
        let tid = TypeId::of::<T>();
        self.sig.remove_component(tid)?;
        Ok(())
    }
    pub fn has_component<T: ComponentTyReqs>(&self) -> bool {
        let tid = TypeId::of::<T>();
        self.sig.contains(tid)
    }
    pub fn get_signature(&self) -> Signature {
        self.sig.clone()
    }
    pub fn get_signature_ref(&self) -> &Signature {
        &self.sig
    }
}

//A reference to a specific entity and its components
pub struct EntityRef<'a> {
    pub id: Id,
    entman: &'a Entman,
}
impl<'a> EntityRef<'a> {
    pub fn has_component<T: ComponentTyReqs>(&self) -> bool {
        self.entman
            .get_entity(self.id)
            .unwrap()
            .has_component::<T>()
    }
}
pub struct EntityRefMut<'a> {
    entity: Id,
    entman: &'a mut Entman,
}
///Represents an entity that owns all its components
pub struct EntityOwned {
    id: Id,
    signature: Signature,
    components: Vec<DynamicComponent>,
}
impl EntityOwned {
    pub fn get_signature(&self) -> Signature {
        self.signature.clone()
    }
}

pub trait CommonComponentStoreTy: Any {
    fn get_type_id(&self) -> TypeId;
    fn get_common_type_name(&self) -> &str;
    fn get_any(&self) -> &dyn CommonComponentStoreTy;
    fn get_any_owned(&self) -> Box<dyn CommonComponentStoreTy>;
    fn get_any_mut(&mut self) -> &mut dyn Any;
    fn insert_dyn(&mut self, component: DynamicComponent) -> Result<()>;
    fn remove_entity_components(&mut self, entity: Id) -> Result<()>;
    fn get_owned_entity_components(&self, entity: Id) -> Result<Vec<DynamicComponent>>;
}

impl dyn CommonComponentStoreTy {
    fn get_type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
    fn into_store<T: ComponentTyReqs>(&self) -> Result<&CommonComponentStore<T>> {
        //convert to Any
        //convert to CommonComponentStore
        let any: &CommonComponentStore<T> = self
            .downcast_ref()
            .ok_or(anyhow!("Could not downcast to CommonComponentStore"))?;
        Ok(any)
    }
    fn into_store_mut<T: ComponentTyReqs>(&mut self) -> &mut CommonComponentStore<T> {
        self.get_any_mut()
            .downcast_mut::<CommonComponentStore<T>>()
            .unwrap()
    }
    fn downcast_ref<T: Any>(&self) -> Option<&T> {
        let any: &dyn Any = self;
        any.downcast_ref::<T>()
    }
}
impl<T: ComponentTyReqs> CommonComponentStoreTy for CommonComponentStore<T> {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
    fn get_any(&self) -> &dyn CommonComponentStoreTy {
        self
    }
    fn get_any_owned(&self) -> Box<dyn CommonComponentStoreTy> {
        let clone = (*self).clone();
        Box::new(clone)
    }
    fn get_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn insert_dyn(&mut self, component: DynamicComponent) -> Result<()> {
        self.insert_dynamic(component)
    }

    fn get_common_type_name(&self) -> &str {
        self.get_common_type_name_internal()
    }
    fn remove_entity_components(&mut self, entity: Id) -> Result<()> {
        self.remove_entity_components_internal(entity)
    }
    fn get_owned_entity_components(&self, entity: Id) -> Result<Vec<DynamicComponent>> {
        self.get_owned_entity_components_internal(entity)
    }
}

//----------------------------------------------------------------------------------------------------------------------//

pub struct DynamicComponent {
    id: ComponentId,
    pub owning_entity: Option<Id>,
    component: Box<dyn ComponentTy>,
    type_id: TypeId,
}
impl DynamicComponent {
    pub fn from_component<T: ComponentTyReqs>(component: Component<T>) -> Self {
        Self {
            id: component.id,
            owning_entity: component.owning_entity,
            component: Box::new(component.component),
            type_id: TypeId::of::<T>(),
        }
    }
    pub fn get_type_id(&self) -> TypeId {
        self.type_id
    }
}
#[nvproc::bincode_derive]
pub struct Component<T: ComponentTyReqs> {
    id: ComponentId,
    pub owning_entity: Option<Id>,
    pub component: T,
}
impl<T: ComponentTyReqs + Default> Component<T> {
    fn new(entity: Id) -> Self {
        Self {
            id: uuid::gen_128().into(),
            owning_entity: Some(entity),
            component: T::default(),
        }
    }
}
impl<T: ComponentTyReqs> Component<T> {
    pub fn from(entity: Id, component: T) -> Self {
        Self {
            id: uuid::gen_128().into(),
            owning_entity: Some(entity),
            component,
        }
    }
    pub fn from_orphan(component: T) -> Self {
        Self {
            id: uuid::gen_128().into(),
            owning_entity: None,
            component,
        }
    }
    pub fn from_dynamic(component: DynamicComponent) -> Self {
        Self {
            id: component.id,
            owning_entity: component.owning_entity,
            component: component
                .component
                .get_any()
                .downcast_ref::<T>()
                .unwrap()
                .clone(),
        }
    }
    pub fn into_dynamic(self) -> DynamicComponent {
        DynamicComponent {
            id: self.id,
            owning_entity: self.owning_entity,
            component: Box::new(self.component),
            type_id: TypeId::of::<T>(),
        }
    }
    //An orphan component has no owning entity
    pub fn new_orphan(component: T) -> Self {
        Self {
            id: uuid::gen_128().into(),
            owning_entity: None,
            component,
        }
    }
    pub fn into(self) -> T {
        self.component
    }
    //Clean setups this component to be used as a component of a new entity
    pub fn clean(&mut self) {
        self.component.clean();
    }
    pub fn get_id(&self) -> ComponentId {
        self.id
    }
    pub fn set_owning_entity(&mut self, entity: Id) {
        self.owning_entity = Some(entity);
    }
    //Make this component an orphan
    pub fn orphan(&mut self) {
        self.owning_entity = None;
    }
    pub fn get_inner(&self) -> &T {
        &self.component
    }
    pub fn get_inner_mut(&mut self) -> &mut T {
        &mut self.component
    }
    pub fn into_inner(self) -> T {
        self.component
    }
}
impl<T: ComponentTyReqs + common::exports::serde::Serialize> ComponentTy for Component<T> {
    fn get_component_name(&self) -> &'static str {
        self.get_inner().get_component_name()
    }
    fn get_any(&self) -> &dyn Any {
        self
    }
    fn get_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn clean(&mut self) {
        self.get_inner_mut().clean();
    }
    fn get_component_type(&self) -> EComponentTypes {
        self.get_inner().get_component_type()
    }
}

//implement Eq and Hash for Component<T>
impl<T: ComponentTyReqs> Eq for Component<T> {}
//implement partial_eq for Component<T>
impl<T: ComponentTyReqs> PartialEq for Component<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<T: ComponentTyReqs> std::hash::Hash for Component<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
//implement PartialOrd and Ord for Component
impl<T: ComponentTyReqs> PartialOrd for Component<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.id.cmp(&other.id))
    }
}
impl<T: ComponentTyReqs> Ord for Component<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

#[derive(bincode::Encode, bincode::Decode, serde::Serialize, serde::Deserialize)]
#[serde(crate = "common::exports::serde")]
#[bincode(crate = "common::exports::bincode")]
pub struct ComponentInfo {
    pub id: Id,
    pub owning_entity: Option<Id>,
}

//impl deref
impl<'a, T: ComponentTyReqs> std::ops::Deref for Component<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.component
    }
}

//Stores all compoments of a common type.
type EntityId = u128;
#[nvproc::bincode_derive]
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy)]
pub struct OrphanComponent {
    id: ComponentId,
    previous_owner: Id,
}
#[nvproc::bincode_derive]
pub struct CommonComponentStore<T: ComponentTyReqs> {
    //the typs of component this store contains
    type_id: TypeId,
    type_name: String,
    //the components of this store, hashed by the owning entity id
    components: HashMap<EntityId, HashMap<ComponentId, Component<T>>>,
}
impl<T: ComponentTyReqs + Default> CommonComponentStore<T> {
    pub fn insert_default(&mut self, owning_entity: Id) -> Result<ComponentId> {
        //first check if there is an orphan component
        let component_id;

        if self.components.contains_key(&owning_entity) {
            let hshmp = self.components.get_mut(&owning_entity).unwrap();
            let comp = Component::<T>::new(owning_entity);
            component_id = comp.id.into();
            hshmp.insert(comp.get_id().into(), comp);
        } else {
            let mut hshmp = HashMap::new();
            let component = Component::<T>::new(owning_entity);
            component_id = component.id.into();
            hshmp.insert(component.get_id().into(), component);
            self.components.insert(owning_entity, hshmp);
        }

        Ok(component_id)
    }
}
impl<T: ComponentTyReqs> CommonComponentStore<T> {
    pub fn new() -> Self {
        Self {
            type_id: TypeId::of::<T>(),
            type_name: T::get_name().to_string(),
            components: HashMap::new(),
        }
    }
    pub fn get_type_id(&self) -> TypeId {
        self.type_id
    }

    pub fn insert(&mut self, owning_entity: Id, component: T) -> Result<ComponentId> {
        let id;

        if self.components.contains_key(&owning_entity) {
            let hshmp = self.components.get_mut(&owning_entity).unwrap();
            let comp = Component::<T>::from(owning_entity, component);
            id = comp.get_id();
            hshmp.insert(comp.get_id(), comp);
        } else {
            let mut hshmp = HashMap::new();
            let component = Component::<T>::from(owning_entity, component);
            id = component.get_id();
            hshmp.insert(component.get_id(), component);
            self.components.insert(owning_entity, hshmp);
        }

        Ok(id)
    }
    pub fn insert_dynamic(&mut self, component: DynamicComponent) -> Result<()> {
        let id;
        let owning_entity = component.owning_entity.unwrap();
        if self.components.contains_key(&owning_entity) {
            let hshmp = self.components.get_mut(&owning_entity).unwrap();
            let comp = Component::<T>::from_dynamic(component);
            id = comp.get_id();
            hshmp.insert(comp.get_id(), comp);
        } else {
            let mut hshmp = HashMap::new();
            let component = Component::<T>::from_dynamic(component);
            id = component.get_id();
            hshmp.insert(component.get_id(), component);
            self.components.insert(owning_entity, hshmp);
        }
        Ok(())
    }
    //Returns the type of this common storage
    pub fn get_common_type(&self) -> Result<EComponentTypes> {
        let res = EComponentTypes::from_name(self.type_id.get_name_ref())
            .ok_or(anyhow! {"Could not find type for {}",self.type_id.get_name_ref()})?;
        Ok(res)
    }
    //Returns the name of the common type as a String
    fn get_common_type_name_internal(&self) -> &str {
        self.type_name.as_str()
    }
    pub fn insert_dyn(
        &mut self,
        owning_entity: Id,
        component: impl ComponentTy,
    ) -> Result<ComponentId> {
        todo!()
    }

    //Actually deletes the component memory from the store
    pub fn remove(&mut self, owning_entity: Id, component_id: ComponentId) -> Result<()> {
        self.components
            .get_mut(&owning_entity)
            .unwrap()
            .remove(&component_id);
        Ok(())
    }
    fn remove_entity_components_internal(&mut self, owning_entity: Id) -> Result<()> {
        self.components.remove(&owning_entity);
        Ok(())
    }
    fn get_owned_entity_components_internal(
        &self,
        owning_entity: Id,
    ) -> Result<Vec<DynamicComponent>> {
        let mut res = Vec::new();
        let hshmp = self.components.get(&owning_entity).unwrap();
        for (_, comp) in hshmp {
            res.push(DynamicComponent::from_component(comp.clone()));
        }
        Ok(res)
    }
}
impl<'de> serde::Deserialize<'de> for Box<dyn CommonComponentStoreTy> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}

impl bincode::Encode for Box<dyn CommonComponentStoreTy> {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        let name = self.get_common_type_name();
        //write the name
        name.encode(encoder)?;
        ComponentStoreSerializer::serialize(name, self, encoder)
    }
}
impl bincode::Decode for Box<dyn CommonComponentStoreTy> {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let name = String::decode(decoder)?;
        let res = ComponentStoreSerializer::deserialize(name.as_str(), decoder)?;
        Ok(res)
    }
}

//the binary storage component stores and manages the access of binary data (like audio/video)
//the components that need access
//Stores all the component data in hashmaps indexed by the owning entity id
#[derive(bincode::Encode, bincode::Decode)]
#[bincode(crate = "common::exports::bincode")]
pub struct Storage {
    //The bins of components. Points to a vector of components, hashed by type id
    bins: HashMap<TypeId, Box<dyn CommonComponentStoreTy>>,
    component_infos: HashMap<Id, ComponentInfo>,
}
impl Storage {
    pub fn new() -> Self {
        Self {
            bins: HashMap::new(),
            component_infos: HashMap::new(),
        }
    }
    pub fn insert_default<T: ComponentTyReqs + Default + serde::Serialize + Clone>(
        &mut self,
        entity: Id,
    ) -> Result<ComponentId> {
        let id = TypeId::of::<T>();
        let component_id;
        match self.bins.contains_key(&id) {
            true => {
                let mut store = self.bins.get_mut(&id).unwrap();
                //downcast to the correct type
                let store = store
                    .as_mut()
                    .get_any_mut()
                    .downcast_mut::<CommonComponentStore<T>>()
                    .unwrap();
                component_id = store.insert_default(entity);
            }
            false => {
                let mut store = CommonComponentStore::<T>::new();
                component_id = store.insert_default(entity);
                self.bins.insert(id, Box::new(store));
            }
        }
        component_id
    }
    pub fn insert_dynamic(&mut self, component: DynamicComponent) -> Result<()> {
        let tid = component.get_type_id();
        let mut store = self.bins.get_mut(&tid).unwrap();
        store.insert_dyn(component)
    }

    pub fn insert_component<T: ComponentTyReqs + serde::Serialize + Clone>(
        &mut self,
        entity: Id,
        component: T,
    ) -> Result<ComponentId> {
        let id = TypeId::of::<T>();
        let component_id;
        match self.bins.contains_key(&id) {
            true => {
                let mut store = self.bins.get_mut(&id).unwrap();
                //downcast to the correct type
                let store = (store.as_mut().get_any_mut())
                    .downcast_mut::<CommonComponentStore<T>>()
                    .unwrap();
                component_id = store.insert(entity, component);
            }
            false => {
                let mut store = CommonComponentStore::<T>::new();
                component_id = store.insert(entity, component);
                self.bins.insert(id, Box::new(store));
            }
        }

        component_id
    }
    pub fn get_entity_component_by_id<T: ComponentTyReqs + Clone>(
        &self,
        entity: Id,
        component_id: ComponentId,
    ) -> Result<&Component<T>> {
        let id = TypeId::of::<T>();
        match self.bins.contains_key(&id) {
            true => {
                let store = self.bins.get(&id).unwrap();
                //downcast to the correct type
                let store: &CommonComponentStore<T> = store.into_store().unwrap();

                match store.components.get(&entity) {
                    Some(component) => component.get(&component_id).ok_or(anyhow!(
                        "Component of type {} with id {:x} not found",
                        type_name_any::<T>(),
                        u128::from(component_id)
                    )),
                    None => Err(anyhow!(
                        "Entity does not have component of type {}",
                        std::any::type_name::<T>()
                    )),
                }
            }
            false => Err(anyhow!(
                "No component store of type {} has yet been created",
                std::any::type_name::<T>()
            )),
        }
    }
    pub fn get_component_with_id<T: ComponentTyReqs + Clone>(
        &self,
        component_id: ComponentId,
    ) -> Result<&Component<T>> {
        let id = TypeId::of::<T>();
        match self.bins.contains_key(&id) {
            true => {
                let store = self.bins.get(&id).unwrap();
                //downcast to the correct type
                let store = store.into_store().unwrap();
                //iterate over all components and return the first one that matches
                for (_, component) in store.components.iter() {
                    if component.contains_key(&component_id) {
                        return Ok(component.get(&component_id).unwrap());
                    }
                }
                Err(anyhow!("Component with id {} not found", component_id))
            }
            false => Err(anyhow!(
                "No component store of type {} has yet been created",
                std::any::type_name::<T>()
            )),
        }
    }
    //Returns a list of all components of type T for the given entity
    pub fn get_components_of_type<T: ComponentTyReqs>(
        &self,
        entity: EntityId,
    ) -> Result<Vec<&Component<T>>> {
        match self.bins.contains_key(&TypeId::of::<T>()) {
            true => {
                let mut res = Vec::<&Component<T>>::new();

                let mut entity_contains_component_type = false;
                //iterate through all component stores, and get the components and add to the list
                for (_, store) in self.bins.iter() {
                    let store = store.into_store();

                    match store {
                        Ok(store) => {
                            entity_contains_component_type = true;
                            if let Some(component) = store.components.get(&entity) {
                                res.append(&mut component.values().collect::<Vec<&Component<T>>>());
                            }
                        }
                        Err(_) => {}
                    }
                }
                if entity_contains_component_type {
                    Ok(res)
                } else {
                    Err(anyhow!(
                        "Entity {} does not have any components of type {}",
                        entity,
                        T::get_name()
                    ))
                }
            }
            false => Err(anyhow!(
                "No component store of type {} has yet been created",
                std::any::type_name::<T>()
            )),
        }
    }

    pub fn get_component_mut_by_id<T: ComponentTyReqs>(
        &mut self,
        entity: Id,
        component_id: ComponentId,
    ) -> Result<&mut Component<T>> {
        let id = TypeId::of::<T>();
        match self.bins.contains_key(&id) {
            true => {
                let store = self.bins.get_mut(&id).unwrap();
                //downcast to the correct type
                let store = store.into_store_mut::<T>();
                match store.components.get_mut(&entity) {
                    Some(component) => component
                        .get_mut(&component_id)
                        .ok_or(anyhow!("Component with id {} not found", component_id)),
                    None => Err(anyhow!(
                        "Entity does not have component of type {}",
                        std::any::type_name::<T>()
                    )),
                }
            }
            false => Err(anyhow!(
                "No component store of type {} has yet been created",
                std::any::type_name::<T>()
            )),
        }
    }
    pub fn get_entity_owned_components(&self, entity: Id) -> Result<Vec<DynamicComponent>> {
        let mut comps: Vec<DynamicComponent> = Vec::new();
        for (_, store) in self.bins.iter() {
            let mut cs = store.get_owned_entity_components(entity)?;
            comps.append(&mut cs);
        }
        Ok(comps)
    }
    ///Removes all components associated with the given entity
    pub fn remove_entity_components(&mut self, entity: Id) {
        for (_, store) in self.bins.iter_mut() {
            let store = store;
            store.remove_entity_components(entity);
        }
    }
}

#[derive(Debug, Copy, Clone)]

pub struct TypeIdInternal(TypeId);

//implement deref into internal typeid
impl std::ops::Deref for TypeIdInternal {
    type Target = TypeId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
//impl from TypeId
impl From<TypeId> for TypeIdInternal {
    fn from(id: TypeId) -> Self {
        TypeIdInternal(id)
    }
}

impl Display for TypeIdInternal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = &self.0 as *const TypeId;
        let id = id as usize;
        write!(f, "{}", id)
    }
}

#[derive(bincode::Encode, bincode::Decode)]
#[bincode(crate = "common::exports::bincode")]
pub struct Entman {
    entities: HashMap<Id, Entity>,
    storage: Storage,
}
impl Entman {
    pub fn new() -> Self {
        Entman {
            entities: HashMap::new(),
            storage: Storage::new(),
        }
    }
    pub fn add_entity(&mut self) -> Id {
        let ent = uuid::gen_128();
        self.entities.insert(ent, Entity::new(ent));
        ent
    }
    //Creates a new entity from an archetype
    pub fn entity_from_archetype<T: ArchetypeTy>(&mut self, archetype: T) -> Id {
        let ent = uuid::gen_128();
        let desc = archetype.describe();
        let sig = desc.get_signature();
        self.entities.insert(ent, Entity::from_sig(ent, sig));
        let entity_mut = self.get_entity_mut(ent).unwrap();

        for c in desc.take_components().into_iter() {
            c.insert_component_into_storage(&mut self.storage, ent);
        }

        ent
    }
    //Removes the entity from the entity manager
    pub fn remove_entity(&mut self, entity: Id) {
        self.entities.remove(&entity);
        self.storage.remove_entity_components(entity);
    }
    //Adds a component to an entity
    pub fn add_component<T: ComponentTyReqs + serde::Serialize + Clone>(
        &mut self,
        entity: Id,
        component: T,
    ) -> Result<ComponentId> {
        let component_id = self.storage.insert_component::<T>(entity, component);
        //add to entity signature
        let ent = self.entities.get_mut(&entity).unwrap();
        ent.add_component::<T>()?;
        component_id
    }
    ///Adds a component to an entity, calling the default "constructor"
    pub fn add_component_default<T: ComponentTyReqs + Default + serde::Serialize + Clone>(
        &mut self,
        entity: Id,
    ) -> Result<ComponentId> {
        let ent = self.entities.get_mut(&entity).unwrap();
        ent.add_component::<T>()?;
        self.storage.insert_default::<T>(entity)
    }
    pub fn get_entity_ref(&self, entity: Id) -> Option<EntityRef> {
        Some(EntityRef {
            id: entity,
            entman: self,
        })
    }
    pub fn get_entity_mut(&mut self, entity: Id) -> Option<EntityRefMut> {
        Some(EntityRefMut {
            entity,
            entman: self,
        })
    }
    pub fn get_entity_count(&self) -> usize {
        self.entities.len()
    }
    //Can only be accessed internally
    fn get_entity(&self, entity: Id) -> Result<&Entity> {
        self.entities
            .get(&entity)
            .ok_or(anyhow!("Entity with id {} not found", entity))
    }
    pub fn get_entity_clone(&self, entity: Id) -> Entity {
        let e = self.entities.get(&entity).unwrap();
        e.clone()
    }
    pub fn get_all_living_entities(&self) -> Vec<Id> {
        self.entities
            .iter()
            .filter_map(|(_, e)| match e.is_valid() {
                true => Some(e.id),
                false => None,
            })
            .collect()
    }
    //Returns the component of type T and id ComponentId for the given entity.
    pub fn get_entity_component_by_id<T: ComponentTyReqs>(
        &self,
        entity: Id,
        component_id: ComponentId,
    ) -> Result<&Component<T>> {
        self.storage
            .get_entity_component_by_id(entity, component_id)
    }
    //Returns the component of type T and id ComponentId.
    pub fn get_component_with_id<T: ComponentTyReqs>(
        &self,
        id: ComponentId,
    ) -> Result<&Component<T>> {
        self.storage.get_component_with_id(id)
    }
    //Get's all components of a common type belonging to an entity.
    pub fn get_components_of_type<T: ComponentTyReqs>(
        &self,
        entity: Id,
    ) -> Result<Vec<&Component<T>>> {
        self.storage.get_components_of_type::<T>(entity)
    }
    pub fn get_entity_owned(&self, entity: Id) -> Result<EntityOwned> {
        let sig = self.get_entity(entity).unwrap().get_signature();
        let components = self.storage.get_entity_owned_components(entity)?;
        Ok(EntityOwned {
            id: entity,
            signature: sig,
            components,
        })
    }
    ///Runs the given query, returning a vector of entities that match the query.
    pub fn query<'a, Q: QueryTy, P: PredicateTy<'a, Q>>(
        &'a self,
        query: &ecs::query::Query<'a, Q, P>,
    ) -> Vec<EntityRef> {
        let sig = Q::generate_sig();
        //iterate over all entities, and check if they match the signature of the query
        let mut result = Vec::new();
        for (id, entity) in self.entities.iter() {
            if entity.get_signature() == sig {
                let entity_ref = self.get_entity_ref(id.clone()).unwrap();
                let qf = QueryFetch::new(*id, &self);
                if query.predicate().check(qf) {
                    result.push(entity_ref);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod test;
