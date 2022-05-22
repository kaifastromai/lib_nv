use std::collections::BTreeMap;
use std::marker::PhantomData;

use common::type_id::TypeIdTy;

use crate::map::Map;

use super::anyhow::{anyhow, Result};
use super::*;
///The trait representing queryable types
pub trait QueryTy {
    fn generate_sig() -> Signature;
    fn contains<T: ComponentTy>() -> bool;
}
//implement QueryTy for all T that implement TypeIdTy and are ComponentTy
impl<'a, T: TypeIdTy + ComponentTy> QueryTy for T {
    fn generate_sig() -> Signature {
        <T as TypeIdTy>::get_type_id().into()
    }
    fn contains<Q: ComponentTy>() -> bool {
        <Q as TypeIdTy>::get_type_id() == <T as TypeIdTy>::get_type_id()
    }

    // fn from_vec(v: Vec<Vec<&dyn ComponentTy>>) -> &Self {
    //     //we know that the size of this vec must be 1
    //     let comps = v.into_iter().next().unwrap();
    //     //take first
    //     let comp = comps.into_iter().next().unwrap();
    //     //downcast
    //     comp.downcast_ref().unwrap()
    // }
}

pub trait IntoTuple<T> {
    fn into_tuple(&self) -> Result<T>;
    fn into_tuple_ref(&self) -> Result<T>;
}
impl<R1: ComponentTyReqs, R2: ComponentTyReqs> IntoTuple<(R1, R2)> for Vec<&dyn ComponentTy> {
    fn into_tuple(&self) -> Result<(R1, R2)> {
        (
            //make sure that the size of the vec is 2
            if (self.len() == 2) {
                let c1: R1 = if let Some(c) = self[0].downcast_ref::<R1>() {
                    c.clone()
                } else if let Some(c) = self[1].downcast_ref::<R1>() {
                    c.clone()
                } else {
                    return Err(anyhow!("Could not downcast"));
                };
                let c2: R2 = if let Some(c) = self[1].downcast_ref::<R2>() {
                    c.clone()
                } else if let Some(c) = self[0].downcast_ref::<R2>() {
                    c.clone()
                } else {
                    return Err(anyhow!("Could not downcast"));
                };

                Ok((c1, c2))
            } else {
                return Err(anyhow!("Vec<&dyn ComponentTy> must have size 2"));
            }
        )
    }
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
struct ConstAssert<const Assert: bool> {}

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

///[QueryResult] contains the matching entities and the components of the matching entities corrosponding to the query.
/// The 'qr lifetime represents how long we borrow the components.
pub struct QueryResult<'qr, T: QueryTy> {
    matches: BTreeMap<Id, Vec<&'qr T>>,
}
impl<'qr, T: QueryTy> QueryResult<'qr, T> {
    pub fn new(
        entities: impl IntoIterator<Item = Id>,
        components: impl IntoIterator<Item = Vec<&'qr T>>,
    ) -> Self {
        let mut matches = BTreeMap::<Id, Vec<&T>>::new();
        for (entity, comp) in entities.into_iter().zip(components.into_iter()) {
            matches.insert(entity, comp);
        }

        QueryResult { matches }
    }
}
///A sytem type is one that can execute logic on a given query
pub struct SystemTy {}
#[nvproc::query_predicate]
fn bob_predicate(f: QueryFetch<NameComponent>) -> bool {
    name_component.component.name == "Bob"
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
    }
    #[test]
    fn test_into_tuple() {
        let mut entman = Entman::new();
        let ent1 = entman.add_entity();
        //add two components to ent1
        entman.add_component_default::<NameComponent>(ent1);
        entman.add_component_default::<LocationComponent>(ent1);
        //get all components as vector
        let comps = entman.get_components_dyn_ref(ent1).unwrap();
        let (loc, name): (&LocationComponent, &NameComponent) = comps.into_tuple().unwrap();
        assert_eq!(name.name, String::default());
        //check in reverse
        let (name, loc): (NameComponent, LocationComponent) = comps.into_tuple().unwrap();
        assert_eq!(name.name, String::default());
        print!("Hello");
    }
}
