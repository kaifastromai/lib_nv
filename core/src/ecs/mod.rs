pub mod archetypes;

pub mod prelude;
pub mod component;
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

pub struct Entity {
    id: Id,
    is_alive: bool,
    components: Option<BTreeSet<std::any::TypeId>>,
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
    pub fn add_component<T: ComponentTy>(&mut self, component: T) -> Result<()> {
        if self.components.is_none() {
            self.components = Some(BTreeSet::from([std::any::TypeId::of::<T>()]));
        } else {
            return match self
                .components
                .as_mut()
                .unwrap()
                .insert(std::any::TypeId::of::<T>())
            {
                true => Ok(()),
                false => Err(anyhow!("Component already exists")),
            };
        }

        Ok(())
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
pub struct ComponentInfo {
    pub id: Id,
    pub owning_entity: Option<Id>,
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
}

//impl deref
impl<'a, T: ComponentTy> std::ops::Deref for Component<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.component
    }
}

pub struct CommonComponentStore<T: ComponentTy> {
    //the typs of component this store contains
    type_id: TypeId,
    //the components of this store
    components: HashMap<Id, Component<T>>,
}
impl<T: ComponentTy + ComponentReqsTy> CommonComponentStore<T> {
    pub fn new() -> Self {
        Self {
            type_id: std::any::TypeId::of::<T>(),
            components: HashMap::new(),
        }
    }
    pub fn get_type_id(&self) -> TypeId {
        self.type_id
    }
    pub fn insert(&mut self, owning_entity: Id) -> Result<()> {
        if self.components.contains_key(&owning_entity) {
            return Err(anyhow!(
                "Entity already has component of type {}",
                <T as ComponentReqsTy>::get_type_name()
            ));
        }
        self.components
            .insert(owning_entity, Component::new(owning_entity));
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
    pub fn get_component<T: ComponentTy>(&self, entity: Id) -> Result<&Component<T>> {
        let id = std::any::TypeId::of::<T>();
        match self.bins.contains_key(&id) {
            true => {
                let store = self.bins.get(&id).unwrap();
                //downcast to the correct type
                let store = (store.as_ref())
                    .downcast_ref::<CommonComponentStore<T>>()
                    .unwrap();
                match store.components.get(&entity) {
                    Some(component) => Ok(component),
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
    pub fn get_component_mut<T: ComponentTy>(&mut self, entity: Id) -> Result<&mut Component<T>> {
        let id = std::any::TypeId::of::<T>();
        match self.bins.contains_key(&id) {
            true => {
                let store = self.bins.get_mut(&id).unwrap();
                //downcast to the correct type
                let store = (store.as_mut())
                    .downcast_mut::<CommonComponentStore<T>>()
                    .unwrap();
                match store.components.get_mut(&entity) {
                    Some(component) => Ok(component),
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
    pub fn add_archetype<T: archetypes::ArchetypeTy>(&mut self, archetype: T) {

    }
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
