use anyhow::{anyhow, Result};
use std::collections::{BTreeSet, HashMap};

pub struct Entity {
    id: u128,
    is_alive: bool,
    components: Option<BTreeSet<std::any::TypeId>>,
}
pub trait ComponentTy: 'static {}

impl Entity {
    pub fn new(id: u128) -> Self {
        Self {
            id,
            is_alive: true,
            components: None,
        }
    }
    pub fn is_valid(&self) -> bool {
        self.is_alive
    }
    pub fn get_id(&self) -> u128 {
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
