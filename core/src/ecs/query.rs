use std::collections::BTreeMap;
use std::marker::PhantomData;

use common::type_id::TypeIdTy;

use crate::map::Map;

use super::anyhow::{anyhow, Result};
use super::*;
///QueryResult holds a map of entities (ids) and their corrosponding components that match the original query
pub struct QueryResult<'a, Q: QueryTy> {
    query: std::marker::PhantomData<Q>,
    matches: BTreeMap<Id, Vec<&'a dyn ComponentTy>>,
}
impl<'a, Q: QueryTy> QueryResult<'a, Q> {
    pub fn new(
        ids: impl IntoIterator<Item = Id>,
        comps: impl IntoIterator<Item = Vec<&'a dyn ComponentTy>>,
    ) -> Self {
        let mut matches = BTreeMap::new();
        for (id, comp) in ids.into_iter().zip(comps) {
            matches.insert(id, comp);
        }
        QueryResult {
            query: PhantomData,
            matches,
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = (Id, &[&dyn ComponentTy])> {
        self.matches
            .iter()
            .map(|(id, comps)| (*id, comps.as_slice()))
    }
    pub fn get_component<'b, T: ComponentTy>(&'b self, ent: Id) -> Result<&'b T> {
        if !Q::contains::<T>() {
            return Err(anyhow!(
                "Tried to get component of type {} but query doesn't contain it",
                T::get_name()
            ));
        }
        let comps = self.matches.get(&ent).unwrap();
        let comp = comps
            .iter()
            .find(|c| c.get_component_type_id() == T::get_type_id())
            .unwrap();
        comp.downcast_ref()
            .ok_or(anyhow!("Could not downcast component"))
    }
}
///The trait representing queryable types
pub trait QueryTy {
    fn generate_sig() -> Signature;
    fn contains<T: ComponentTy>() -> bool;
    // fn from_dyn_vec(vec: Vec<&dyn ComponentTy>) -> Result<Self>
    // where
    //     Self: std::marker::Sized;
}
//implement QueryTy for all T that implement TypeIdTy and are ComponentTy
impl<'a, T: TypeIdTy + ComponentTy> QueryTy for T {
    fn generate_sig() -> Signature {
        <T as TypeIdTy>::get_type_id().into()
    }
    fn contains<Q: ComponentTy>() -> bool {
        <Q as TypeIdTy>::get_type_id() == <T as TypeIdTy>::get_type_id()
    }
    // fn from_dyn_vec(vec: Vec<&dyn ComponentTy>) -> Result<Self> {
    //     todo!()
    // }
}

nvproc::generate_query_ty_tuple_impls!();

///The [NullPredicate] always returns true. For internal use only.
pub struct NullPredicate<T> {
    _marker: PhantomData<T>,
}
impl<T> NullPredicate<T> {
    pub fn new() -> Self {
        NullPredicate {
            _marker: PhantomData,
        }
    }
}
impl<'em, Q: QueryTy> PredicateTy<'em, Q> for NullPredicate<Q> {
    fn check(&self, _: QueryFetch<Q>) -> bool {
        true
    }
}

///Lifetime 'em refers to the lifetime of the borrowed Entman construct.
pub trait PredicateTy<'em, Q>
where
    Q: QueryTy,
{
    fn check(&self, fetch: QueryFetch<'em, Q>) -> bool;
}
impl<'em, Q: QueryTy, T: Fn<(QueryFetch<'em, Q>,), Output = bool>> PredicateTy<'em, Q> for T {
    fn check(&self, fetch: QueryFetch<'em, Q>) -> bool {
        self(fetch)
    }
}
///A [Query] that retrieves components, or Entities from} the ECS (Entman)
pub struct Query<'em, T: QueryTy, P = NullPredicate<T>>
where
    P: PredicateTy<'em, T>,
{
    phantom: std::marker::PhantomData<&'em T>,
    predicate: P,
    matching_entities: Option<Vec<Id>>,
}
impl<'em, T: QueryTy, P: PredicateTy<'em, T>> Query<'em, T, P> {
    pub fn from_pred(p: P) -> Self {
        Query {
            phantom: std::marker::PhantomData,
            predicate: p,
            matching_entities: None,
        }
    }

    fn get_query_sig() -> Signature {
        T::generate_sig()
    }
    pub fn predicate(&self) -> &P {
        &self.predicate
    }
}
impl<'em, T: QueryTy> Query<'em, T, NullPredicate<T>> {
    pub fn new() -> Self {
        Query {
            phantom: std::marker::PhantomData,
            predicate: NullPredicate::new(),
            matching_entities: None,
        }
    }
}

///A query fetch allows statically known access to the components of an entity (hopefully).
/// It essentially a wrapper over an Entity, but allows direct access to the components since
/// We can be guaranteed that components exist.
pub struct QueryFetch<'em, T: QueryTy> {
    phantom: std::marker::PhantomData<T>,
    entity_id: Id,

    entman_ref: &'em Entman,
}
impl<'a, T: QueryTy> QueryFetch<'a, T> {
    pub fn new(entity_id: Id, entman_ref: &'a Entman) -> Self {
        QueryFetch {
            phantom: std::marker::PhantomData,
            entity_id,
            entman_ref,
        }
    }
    //Returns component of the given type for the entity the QueryFetch is associated with.
    fn get_component<C: ComponentTyReqs>(&'a self) -> Result<&Component<C>> {
        //check if comp sig exists in signature
        if T::contains::<C>() {
            //access component
            let component = self.entman_ref.get_component_ref::<C>(self.entity_id)?;
            return Ok(component);
        }
        Err(anyhow!("Component does not match signature"))
    }
}

macro_rules! into_tuple {
    ($entman:expr, $ent:expr, $($type:ident),*) => {
        ($($entman.get_component_ref::<$type>($ent).unwrap()),*)
    };

}
///A sytem type is one that can execute logic on a given query
pub struct SystemTy {}
#[nvproc::query_predicate]
fn bob_predicate(f: QueryFetch<NameComponent>) -> bool {
    name_component.name == "Bob"
}

#[cfg(test)]
mod test_query {

    use crate::ecs::component::components::*;

    use super::*;

    #[test]
    fn test_sig_gen() {
        let query = Query::<(LocationComponent, NameComponent)>::get_query_sig();
        let comp_sig = Signature::from(vec![
            LocationComponent::get_type_id(),
            NameComponent::get_type_id(),
        ]);
        assert_eq!(query, comp_sig);
    }
    #[test]
    fn test_predicates() {
        let pred = |fetch: QueryFetch<(LocationComponent, NameComponent)>| -> bool {
            return true;
        };
        let q = Query::from_pred(pred);
    }
    #[test]
    fn test_query_fetch() {
        let mut entman = Entman::new();
        let ent1 = entman.add_entity();
        let ent2 = entman.add_entity();
        let ent3 = entman.add_entity();
        let ent4 = entman.add_entity();

        entman
            .add_component::<NameComponent>(
                ent1,
                NameComponent {
                    name: "Bob".to_string(),
                    aliases: vec![],
                },
            )
            .unwrap();
        entman.add_component_default::<StringFieldComponent>(ent2);
        entman.add_component_default::<StringFieldComponent>(ent3);
        entman.add_component(
            ent4,
            NameComponent {
                name: "Jane".to_string(),
                aliases: vec![],
            },
        );
        let q = Query::from_pred(bob_predicate);
        let qres = entman.query(&q);

        //test null predicate
        let q2 = Query::<NameComponent>::new();
        let qres2 = entman.query(&q2);
        let name = qres2.get_component::<NameComponent>(ent1).unwrap();
        assert_eq!(name.name, "Bob");

        //test multiple components
        let q3 = Query::<(NameComponent, StringFieldComponent)>::new();

        //add extra components to ent1
        entman.add_component(
            ent1,
            StringFieldComponent {
                name: "Name".to_string(),
                value: "Bob".to_string(),
            },
        );
        let qres3 = entman.query(&q3);
        let name = qres3.get_component::<NameComponent>(ent1).unwrap();
        assert_eq!(name.name, "Bob");
        let field = qres3.get_component::<StringFieldComponent>(ent1).unwrap();
        assert_eq!(field.name, "Name");
    }
}
