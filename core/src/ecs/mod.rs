pub mod archetypes;

pub mod component;
pub mod prelude;
use super::*;
pub use serde::{Deserialize, Serialize};
use utils::{prelude::*, uuid};

use anyhow::{anyhow, Result};
use std::{
    any::{Any, TypeId},
    collections::{BTreeSet, HashMap},
};

pub type Id = u128;
//A component type. It's id corrosponds to the entity it belongs to.
pub trait ComponentTy: Any {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
    fn get_type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
    //Prepare this component to be used by a new entity
    fn clean(&mut self);
}
pub trait ComponentReqsTy: 'static + Default {
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
        let tid = std::any::TypeId::of::<T>();
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
        let tid = std::any::TypeId::of::<T>();
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
        let tid = std::any::TypeId::of::<T>();
        match &self.components {
            Some(c) => c.contains(&tid),
            None => false,
        }
    }
    pub fn get_components_ref(&mut self) -> &Vec<TypeId> {
        self.components.as_ref().unwrap()
    }
    pub fn get_components(&mut self) -> Vec<TypeId> {
        self.components.as_ref().unwrap().clone()
    }
}

//A reference to a specific entity and its components
pub struct EntityRef {}
pub struct EntityRefMut {}
// pub struct ComponentRef<'a, T: ComponentTy> {
//     component: &'a Component<T>,
// }
// pub struct ComponentMut<'a, T: ComponentTy> {
//     entity: Id,
//     component: &'a mut Component<T>,
// }
//Stores all the components.
//The components are stored in bins of common types hashed by their type id.

pub trait CommonComponentStoreTy: Any {
    fn get_type_id(&self) -> TypeId;
}
impl dyn CommonComponentStoreTy {
    fn get_type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
    fn downcast_ref<T: CommonComponentStoreTy>(&self) -> Option<&T> {
        if self.get_type_id() == std::any::TypeId::of::<T>() {
            Some(self.downcast_ref::<T>().unwrap())
        } else {
            None
        }
    }
    fn downcast_mut<T: CommonComponentStoreTy>(&mut self) -> Option<&mut T> {
        if self.get_type_id() == std::any::TypeId::of::<T>() {
            Some(self.downcast_mut::<T>().unwrap())
        } else {
            None
        }
    }
}
impl<T: ComponentTy> CommonComponentStoreTy for CommonComponentStore<T> {
    fn get_type_id(&self) -> TypeId {
        std::any::TypeId::of::<T>()
    }
}
//----------------------------------------------------------------------------------------------------------------------//

pub struct Component<T: ComponentTy> {
    id: Id,
    pub owning_entity: Option<Id>,
    pub component: T,
}
impl<T: ComponentTy + Default> Component<T> {
    pub fn new(entity: Id) -> Self {
        Self {
            id: uuid::generate(),
            owning_entity: Some(entity),
            component: Default::default(),
        }
    }
    //An orphan component has no owning entity
    pub fn new_orphan(component: T) -> Self {
        Self {
            id: uuid::generate(),
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
    pub fn get_id(&self) -> Id {
        self.id
    }
    pub fn set_owning_entity(&mut self, entity: Id) {
        self.owning_entity = Some(entity);
    }
    //Make this component an orphan
    pub fn orphan(&mut self) {
        self.owning_entity = None;
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
type ComponentId = u128;
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct OrphanComponent {
    id: ComponentId,
    previous_owner: Id,
}
pub struct CommonComponentStore<T: ComponentTy> {
    //the typs of component this store contains
    type_id: TypeId,
    //the components of this store
    components: HashMap<Id, HashMap<ComponentId, Component<T>>>,
    orphans: Vec<OrphanComponent>,
}
impl<T: ComponentTy + ComponentReqsTy> CommonComponentStore<T> {
    pub fn new() -> Self {
        Self {
            type_id: std::any::TypeId::of::<T>(),
            components: HashMap::new(),
            orphans: Vec::new(),
        }
    }
    pub fn get_type_id(&self) -> TypeId {
        self.type_id
    }
    pub fn insert(&mut self, owning_entity: Id) -> Result<()> {
        //first check if there is an orphan component
        let orphan_component = self.orphans.pop();
        match orphan_component {
            Some(orphan) => {
                //If we have an orphan component, transfer ownership to the new entity
                self.transfer_ownership(orphan, owning_entity)?;
            }
            None => {
                if self.components.contains_key(&owning_entity) {
                    let hshmp = self.components.get_mut(&owning_entity).unwrap();
                    let comp = Component::<T>::new(owning_entity);
                    hshmp.insert(comp.get_id(), comp);
                } else {
                    let mut hshmp = HashMap::new();
                    let component = Component::<T>::new(owning_entity);
                    hshmp.insert(component.get_id(), component);
                    self.components.insert(owning_entity, HashMap::new());
                }
            }
        }
        Ok(())
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
    pub fn remove(&mut self, owning_entity: Id, component_id: Id) -> Result<()> {
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
pub struct Storage {
    //The bins of components. Points to a vector of components.
    bins: HashMap<std::any::TypeId, Box<dyn CommonComponentStoreTy>>,
    component_infos: HashMap<Id, ComponentInfo>,
}
impl Storage {
    pub fn new() -> Self {
        Self {
            bins: HashMap::new(),
            component_infos: HashMap::new(),
        }
    }
    pub fn insert_component<T: ComponentTy + ComponentReqsTy>(&mut self, entity: Id) -> Result<()> {
        let id = std::any::TypeId::of::<T>();
        match self.bins.contains_key(&id) {
            true => {
                let mut store = self.bins.get_mut(&id).unwrap();
                //downcast to the correct type
                let store = (store.as_mut())
                    .downcast_mut::<CommonComponentStore<T>>()
                    .unwrap();
                store.insert(entity);
            }
            false => {
                let mut store = CommonComponentStore::<T>::new();
                store.insert(entity);
                self.bins.insert(id, Box::new(store));
            }
        }

        Ok(())
    }
    pub fn get_component_by_id<T: ComponentTy>(
        &self,
        entity: Id,
        component_id: Id,
    ) -> Result<&Component<T>> {
        let id = std::any::TypeId::of::<T>();
        match self.bins.contains_key(&id) {
            true => {
                let store = self.bins.get(&id).unwrap();
                //downcast to the correct type
                let store = (store.as_ref())
                    .downcast_ref::<CommonComponentStore<T>>()
                    .unwrap();
                match store.components.get(&entity) {
                    Some(component) => component
                        .get(&component_id)
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
    pub fn get_component_mut_by_id<T: ComponentTy>(
        &mut self,
        entity: Id,
        component_id: ComponentId,
    ) -> Result<&mut Component<T>> {
        let id = std::any::TypeId::of::<T>();
        match self.bins.contains_key(&id) {
            true => {
                let store = self.bins.get_mut(&id).unwrap();
                //downcast to the correct type
                let store = (store.as_mut())
                    .downcast_mut::<CommonComponentStore<T>>()
                    .unwrap();
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
        let id = std::any::TypeId::of::<T>();
        let store = self
            .bins
            .get_mut(&id)
            .ok_or(anyhow!(
                "No component store of type {} has yet been created",
                std::any::type_name::<T>()
            ))?
            .as_mut()
            .downcast_mut::<CommonComponentStore<T>>()
            .unwrap();
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
        todo!()
    }
    pub fn remove_entity(&mut self, entity: Id) {
        todo!()
    }
    pub fn add_component<T: ComponentTy>(&mut self, entity: Id, component: T) {
        todo!()
    }
    pub fn add_archetype<T: archetypes::ArchetypeTy>(&mut self, archetype: T) {}
    pub fn get_entity(&self, entity: Id) -> Option<bevy_ecs::world::EntityRef> {
        todo!()
    }
    pub fn get_entity_mut(&mut self, entity: Id) -> Option<bevy_ecs::world::EntityMut> {
        todo!()
    }
    pub fn get_entity_count(&self) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod test;
