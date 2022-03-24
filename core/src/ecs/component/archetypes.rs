//Archetypes map to bevy's bundles

use crate::ecs::component::*;
use crate::ecs::*;
struct SigType<T: ComponentTypeIdTy> {
    phantom: std::marker::PhantomData<T>,
}
impl<T: ComponentTypeIdTy> SigType<T> {
    pub fn into_type_id(self) -> ComponentTypeId {
        ComponentTypeId {
            0: TypeId::of::<T>(),
        }
    }
}

macro_rules! arch_sig {
    ($($comp:ident),*) => {
        {
            let mut sig=Vec::<ComponentTypeId>::new();
            $(
                let sigtype=SigType::<$comp>{phantom:std::marker::PhantomData};
                sig.push(sigtype.into_type_id());
            )*
            sig
        }
    };
}

//An archetype is an entity that has a predefined set of components
macro_rules! archetype {
    ( $($signature:tt),* ) => {};
}
pub struct ArchetypeDescriptor {
    signature: Vec<TypeId>,
    //A Json object that describes the archetype
    json_descriptor: String,

}
impl ArchetypeDescriptor {
    pub fn new_empty() -> Self {
        Self {
            signature: Vec::new(),
            json_descriptor: String::new(),
        }
    }
    pub fn new(components: Vec<impl ComponentTypeIdTy>) -> Self {
        Self {
            signature: components.iter().map(|c| c.get_type_id_ref()).collect(),
            json_descriptor: String::new(),
        }
    }

    pub fn with_component<T: ComponentTypeIdTy>(mut self) -> Self {
        self.signature.push(TypeId::of::<T>());
        self
    }
}

impl<T: ComponentTypeIdTy> From<T> for SigType<T> {
    fn from(_: T) -> Self {
        SigType {
            phantom: std::marker::PhantomData,
        }
    }
}

pub trait ArchetypeTy {
    fn describe(&self) -> ArchetypeDescriptor;
    fn consume(self) -> Vec<Box<dyn ComponentTy>>;
}
pub struct CharacterArchetype {
    archetype_name: &'static str,
    data: Vec<Box<dyn ComponentTy>>,
}
pub struct Archetype<T: ArchetypeTy> {
    archetype: T,
}
impl ArchetypeTy for CharacterArchetype {
    fn describe(&self) -> ArchetypeDescriptor {
        let descriptor = ArchetypeDescriptor::new(arch_sig!(
            StringField,
            StringField,
            StringField,
            StringField
        ));
        descriptor
    }
    fn consume(mut self) -> Vec<Box<dyn ComponentTy>> {
        todo!()
    }
}

//Given that bevy has bundles, the only point of this is to store user-made archetypes
struct SerializableArchetype {}
