pub mod component;
pub mod prelude;
mod tests;
use self::component::archetypes::ArchetypeTy;

use super::*;
use crate::ecs::component::*;
use common::{exports::*, type_id::*, type_name_any, uuid};
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
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[nvproc::serde_derive("common::exports::serde")]
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
//A component type. It's id corrosponds to the entity it belongs to.
pub trait ComponentTy: 'static {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
    fn get_type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
    fn get_component_name(&self) -> &'static str;
    //Prepare this component to be used by a new entity
    fn clean(&mut self);
    fn get_any(&self) -> &dyn Any;
    fn get_any_mut(&mut self) -> &mut dyn Any;
}

pub trait ComponentReqsTy: 'static {
    fn get_type_id() -> TypeId {
        TypeId::of::<Self>()
    }
    fn get_type_name() -> &'static str {
        std::any::type_name::<Self>()
    }
}
impl<T> ComponentReqsTy for T where T: ComponentTy + Default {}
impl ComponentTy for () {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<()>()
    }
    fn get_type_name(&self) -> &'static str {
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
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct EntityComponent {
    pub type_id: TypeId,
    pub count: usize,
}

//implement From<TypeID> for EntityComponent
impl From<TypeId> for EntityComponent {
    fn from(type_id: TypeId) -> Self {
        EntityComponent { type_id, count: 0 }
    }
} //implement PartialOrd for EntityComponent
impl PartialOrd for EntityComponent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.count.cmp(&other.count))
    }
}
impl Ord for EntityComponent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.count.cmp(&other.count)
    }
}
#[derive(Debug, Clone)]
pub struct Entity {
    id: Id,
    is_alive: bool,
    components: Option<Vec<TypeId>>,
}
impl Entity {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            is_alive: true,
            components: None,
        }
    }
    pub fn is_valid(&self) -> bool {
        self.is_alive
    }
    pub fn get_id(&self) -> Id {
        self.id
    }
    pub fn add_component<T: ComponentTy>(&mut self) -> Result<()> {
        let tid = TypeId::of::<T>();
        match self.components.as_mut() {
            Some(c) => {
                c.push(tid);
            }
            None =>
            //if no components, create a new vec
            {
                self.components = Some(vec![tid]);
            }
        }

        Ok(())
    }
    pub fn remove_component<T: ComponentTy>(&mut self) -> Result<()> {
        let tid = TypeId::of::<T>();
        //remove the fisrt occurence of the type
        match self.components.as_mut() {
            Some(c) => {
                let index = c.iter().position(|&x| x == tid).ok_or(anyhow!(
                    "Entity {} has no component of type {}",
                    self.id,
                    std::any::type_name::<T>()
                ))?;
                c.remove(index);
            }
            None => {
                return Err(anyhow!("Entity {} has no components", self.id));
            }
        }
        Ok(())
    }
    pub fn has_component<T: ComponentTy>(&self) -> bool {
        let tid = TypeId::of::<T>();
        match &self.components {
            Some(c) => c.contains(&tid),
            None => false,
        }
    }
    pub fn get_signature(&self) -> Vec<TypeId> {
        self.components.as_ref().unwrap().clone()
    }
    pub fn get_signature_ref(&self) -> &Vec<TypeId> {
        &self.components.as_ref().unwrap()
    }
    pub fn get_signature_str(&self) -> String {
        let mut signature = String::new();
        for tid in self.components.as_ref().unwrap().iter() {
            signature.push_str(std::any::type_name::<TypeId>());
            signature.push_str(" ");
        }
        signature
    }
}

//A reference to a specific entity and its components
pub struct EntityRef<'a> {
    pub id: Id,
    entman: &'a Entman,
}
impl<'a> EntityRef<'a> {
    pub fn has_component<T: ComponentTy>(&self) -> bool {
        self.entman.get_entity(self.id).has_component::<T>()
    }
}
pub struct EntityRefMut<'a> {
    entity: Id,
    entman: &'a mut Entman,
}

pub trait CommonComponentStoreTy: 'static {
    fn get_type_id(&self) -> TypeId;
    fn get_any(&self) -> &dyn Any;
    fn get_any_mut(&mut self) -> &mut dyn Any;
}
impl dyn CommonComponentStoreTy {
    fn get_type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
    fn into_store<T: ComponentTy>(&self) -> &CommonComponentStore<T> {
        self.get_any()
            .downcast_ref::<CommonComponentStore<T>>()
            .unwrap()
    }
    fn into_store_mut<T: ComponentTy>(&mut self) -> &mut CommonComponentStore<T> {
        self.get_any_mut()
            .downcast_mut::<CommonComponentStore<T>>()
            .unwrap()
    }
}
impl<T: ComponentTy> CommonComponentStoreTy for CommonComponentStore<T> {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
    fn get_any(&self) -> &dyn Any {
        self
    }
    fn get_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
//----------------------------------------------------------------------------------------------------------------------//

pub struct Component<T: ComponentTy> {
    id: ComponentId,
    pub owning_entity: Option<Id>,
    pub component: T,
}
impl<T: ComponentTy + Default> Component<T> {
    fn new(entity: Id) -> Self {
        Self {
            id: uuid::gen_128().into(),
            owning_entity: Some(entity),
            component: T::default(),
        }
    }
}
impl<T: ComponentTy> Component<T> {
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
impl<T: ComponentTy> ComponentTy for Component<T> {
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
}

//implement Eq and Hash for Component<T>
impl<T: ComponentTy> Eq for Component<T> {}
//implement partial_eq for Component<T>
impl<T: ComponentTy> PartialEq for Component<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<T: ComponentTy> std::hash::Hash for Component<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
//implement PartialOrd and Ord for Component
impl<T: ComponentTy> PartialOrd for Component<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.id.cmp(&other.id))
    }
}
impl<T: ComponentTy> Ord for Component<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

pub struct ComponentInfo {
    pub id: Id,
    pub owning_entity: Option<Id>,
}

//impl deref
impl<'a, T: ComponentTy> std::ops::Deref for Component<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.component
    }
}

//Stores all compoments of a common type.
type EntityId = u128;
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct OrphanComponent {
    id: ComponentId,
    previous_owner: Id,
}
pub struct CommonComponentStore<T: ComponentTy> {
    //the typs of component this store contains
    type_id: TypeId,
    //the components of this store, hashed by the owning entity id
    components: HashMap<EntityId, HashMap<ComponentId, Component<T>>>,
    orphans: Vec<OrphanComponent>,
}
impl<T: ComponentTy + Default> CommonComponentStore<T> {
    pub fn insert_default(&mut self, owning_entity: Id) -> Result<ComponentId> {
        //first check if there is an orphan component
        let orphan_component = self.orphans.pop();
        let component_id;
        match orphan_component {
            Some(orphan) => {
                //If we have an orphan component, transfer ownership to the new entity
                self.transfer_ownership(orphan, owning_entity)?;
                component_id = orphan.id;
            }
            None => {
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
            }
        }
        Ok(component_id)
    }
}
impl<T: ComponentTy> CommonComponentStore<T> {
    pub fn new() -> Self {
        Self {
            type_id: TypeId::of::<T>(),
            components: HashMap::new(),
            orphans: Vec::new(),
        }
    }
    pub fn get_type_id(&self) -> TypeId {
        self.type_id
    }
    pub fn insert(&mut self, owning_entity: Id, component: T) -> Result<ComponentId> {
        let orphan_component = self.orphans.pop();
        let id;
        match orphan_component {
            Some(orphan) => {
                //If we have an orphan component, transfer ownership to the new entity
                self.transfer_ownership(orphan, owning_entity)?;
                id = orphan.id;
            }
            None => {
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
            }
        }
        Ok(id)
    }

    //Marks this component as orphaned, and as a candiate to be reparented
    //It is the responsibility of the caller to ensure that the owning entity removes the component!
    pub fn orphan(&mut self, owning_entity: Id, component_id: ComponentId) -> Result<()> {
        self.components
            .get_mut(&owning_entity)
            .unwrap()
            .get_mut(&component_id)
            .unwrap()
            .orphan();
        //add the component to the orphan list
        self.add_to_orphan_list(OrphanComponent {
            id: component_id,
            previous_owner: owning_entity,
        })?;
        Ok(())
    }
    fn add_to_orphan_list(&mut self, orphan_id: OrphanComponent) -> Result<()> {
        //it is a logic error to add a component to the orphan list if it is already in the orphan list
        if !self.orphans.contains(&orphan_id) {
            self.orphans.push(orphan_id);
        } else {
            return Err(anyhow!(
                "Component already in orphan list! This is very bad!"
            ));
        }
        Ok(())
    }
    fn purge_from_orphan_list(&mut self, orphan_id: OrphanComponent) {
        self.orphans.retain(|&x| x != orphan_id);
    }
    //Actually deletes the component memory from the store
    pub fn remove(&mut self, owning_entity: Id, component_id: ComponentId) -> Result<()> {
        self.components
            .get_mut(&owning_entity)
            .unwrap()
            .remove(&component_id);
        self.purge_from_orphan_list(OrphanComponent {
            id: component_id,
            previous_owner: owning_entity,
        });
        Ok(())
    }
    pub fn get_orphans(&self) -> &Vec<OrphanComponent> {
        self.orphans.as_ref()
    }
    pub fn get_orphans_mut(&mut self) -> &mut Vec<OrphanComponent> {
        self.orphans.as_mut()
    }
    pub fn take_orphan(&mut self, orphan: OrphanComponent) -> Result<Component<T>> {
        //make sure this orphan exists in the orphan list
        if !self.orphans.contains(&orphan) {
            return Err(anyhow!("Tried to take an orphan that does not exist!"));
        }
        let component = self
            .components
            .get_mut(&orphan.previous_owner)
            .ok_or(anyhow!(
                "This orphan does not reference a valid previous owner!"
            ))?
            .remove(&orphan.id)
            .unwrap();
        self.purge_from_orphan_list(orphan);
        Ok(component)
    }
    pub fn transfer_ownership(&mut self, orphan: OrphanComponent, new_owner: Id) -> Result<()> {
        let mut taken = self.take_orphan(orphan)?;
        taken.set_owning_entity(new_owner);
        taken.clean();
        //add the component to the new owner
        self.components
            .get_mut(&new_owner)
            .unwrap()
            .insert(taken.get_id(), taken);

        Ok(())
    }
}

//the binary storage component stores and manages the access of binary data (like audio/video)
//the components that need access
//Stores all the component data in hashmaps indexed by the owning entity id

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
    pub fn insert_default<T: ComponentTy + Default>(&mut self, entity: Id) -> Result<ComponentId> {
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
    pub fn insert_component_dyn(entity: Id, component: impl ComponentTy) -> Result<()> {
        let id = component.get_type_id();
        todo!();
    }
    pub fn insert_component<T: ComponentTy>(
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
    pub fn get_entity_component_by_id<T: ComponentTy>(
        &self,
        entity: Id,
        component_id: ComponentId,
    ) -> Result<&Component<T>> {
        let id = TypeId::of::<T>();
        match self.bins.contains_key(&id) {
            true => {
                let store = self.bins.get(&id).unwrap();
                //downcast to the correct type
                let store: &CommonComponentStore<T> = store.into_store();

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
    pub fn get_component_with_id<T: ComponentTy>(
        &self,
        component_id: ComponentId,
    ) -> Result<&Component<T>> {
        let id = TypeId::of::<T>();
        match self.bins.contains_key(&id) {
            true => {
                let store = self.bins.get(&id).unwrap();
                //downcast to the correct type
                let store = store.into_store();
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
    pub fn get_components_of_type<T: ComponentTy>(
        &self,
        entity: EntityId,
    ) -> Result<Vec<&Component<T>>> {
        match self.bins.contains_key(&TypeId::of::<T>()) {
            true => {
                let mut res = Vec::<&Component<T>>::new();

                //iterate through all component stores, and get the components and add to the list
                for (_, store) in self.bins.iter() {
                    let store: &CommonComponentStore<T> = store.into_store();

                    if let Some(component) = store.components.get(&entity) {
                        res.append(&mut component.values().collect::<Vec<&Component<T>>>());
                    }
                }
                Ok(res)
            }
            false => Err(anyhow!(
                "No component store of type {} has yet been created",
                std::any::type_name::<T>()
            )),
        }
    }

    pub fn get_component_mut_by_id<T: ComponentTy>(
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
    //Give an orphan component a new entity parent.
    pub fn reparent<T: ComponentTy + ComponentReqsTy>(&mut self, new_parent: Id) -> Result<()> {
        //first collect all orphaned components
        let id = TypeId::of::<T>();
        let store = self
            .bins
            .get_mut(&id)
            .ok_or(anyhow!(
                "No component store of type {} has yet been created",
                std::any::type_name::<T>()
            ))?
            .into_store_mut::<T>();
        let orphans = store.get_orphans_mut();
        //get the first orphan
        let orphan = orphans
            .get(0)
            .ok_or(anyhow!(
                "No orphaned components of type {}",
                std::any::type_name::<T>()
            ))?
            .clone();
        //get the component
        let component = self.get_component_mut_by_id::<T>(orphan.previous_owner, orphan.id)?;
        //set the owning entity
        component.owning_entity = Some(new_parent);
        Ok(())
    }
    pub fn add_archetype<T: ArchetypeTy>(&mut self, archetype: &dyn ArchetypeTy) -> Result<()> {
        todo!()
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
    //Removes the entity from the entity manager, and marks all it's components as orphaned.

    pub fn remove_entity(&mut self, entity: Id) {}
    //Adds a component to an entity
    pub fn add_component<T: ComponentTy>(
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
    //Adds a component to an entity, calling the default "constructor"
    pub fn add_default<T: ComponentTy + Default>(&mut self, entity: Id) -> Result<ComponentId> {
        let ent = self.entities.get_mut(&entity).unwrap();
        ent.add_component::<T>()?;
        self.storage.insert_default::<T>(entity)
    }
    pub fn add_archetype<T: component::archetypes::ArchetypeTy>(
        &mut self,
        archetype: T,
    ) -> Result<()> {
        todo!()
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
    fn get_entity(&self, entity: Id) -> &Entity {
        self.entities.get(&entity).unwrap()
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
    pub fn get_entity_component_by_id<T: ComponentTy>(
        &self,
        entity: Id,
        component_id: ComponentId,
    ) -> Result<&Component<T>> {
        self.storage
            .get_entity_component_by_id(entity, component_id)
    }
    //Returns the component of type T and id ComponentId.
    pub fn get_component_with_id<T: ComponentTy>(&self, id: ComponentId) -> Result<&Component<T>> {
        self.storage.get_component_with_id(id)
    }
    //Get's all components of a common type beling to an entity.
    pub fn get_components_of_type<T: ComponentTy>(&self, entity: Id) -> Result<Vec<&Component<T>>> {
        self.storage.get_components_of_type::<T>(entity)
    }
}

#[cfg(test)]
mod test;
