use std::marker::PhantomData;

use common::type_id::TypeIdTy;

use super::anyhow::{anyhow, Result};
use super::*;
///The trait representing queryable types
pub trait QueryTy {
    fn generate_sig() -> Signature;
    fn contains<T: ComponentTy>() -> bool;
}
//implement QueryTy for all T that implement TypeIdTy and are ComponentTy
impl<T: TypeIdTy + ComponentTy> QueryTy for T {
    fn generate_sig() -> Signature {
        <T as TypeIdTy>::get_type_id().into()
    }
    fn contains<Q: ComponentTy>() -> bool {
        <Q as TypeIdTy>::get_type_id() == <T as TypeIdTy>::get_type_id()
    }
}

mod query_impls {
    use crate::ecs::ComponentTy;

    use super::super::Signature;

    use super::QueryTy;

    //implement Query for tuple of QueryTy
    impl<R1: QueryTy> QueryTy for (R1,) {
        fn generate_sig() -> Signature {
            R1::generate_sig()
        }
        fn contains<Q: ComponentTy>() -> bool {
            R1::contains::<Q>()
        }
    }

    impl<R1: QueryTy, R2: QueryTy> QueryTy for (R1, R2) {
        fn generate_sig() -> Signature {
            let mut sig = R1::generate_sig();
            sig.merge(&R2::generate_sig());
            sig
        }
        fn contains<Q: ComponentTy>() -> bool {
            R1::contains::<Q>() || R2::contains::<Q>()
        }
    }

    impl<R1: QueryTy, R2: QueryTy, R3: QueryTy> QueryTy for (R1, R2, R3) {
        fn generate_sig() -> Signature {
            let mut sig = R1::generate_sig();
            sig.merge(&R2::generate_sig());
            sig.merge(&R3::generate_sig());
            sig
        }
        fn contains<Q: ComponentTy>() -> bool {
            R1::contains::<Q>() || R2::contains::<Q>() || R3::contains::<Q>()
        }
    }

    impl<R1: QueryTy, R2: QueryTy, R3: QueryTy, R4: QueryTy> QueryTy for (R1, R2, R3, R4) {
        fn generate_sig() -> Signature {
            let mut sig = R1::generate_sig();
            sig.merge(&R2::generate_sig());
            sig.merge(&R3::generate_sig());
            sig.merge(&R4::generate_sig());
            sig
        }
        fn contains<Q: ComponentTy>() -> bool {
            R1::contains::<Q>() || R2::contains::<Q>() || R3::contains::<Q>() || R4::contains::<Q>()
        }
    }

    impl<R1: QueryTy, R2: QueryTy, R3: QueryTy, R4: QueryTy, R5: QueryTy> QueryTy
        for (R1, R2, R3, R4, R5)
    {
        fn generate_sig() -> Signature {
            let mut sig = R1::generate_sig();
            sig.merge(&R2::generate_sig());
            sig.merge(&R3::generate_sig());
            sig.merge(&R4::generate_sig());
            sig.merge(&R5::generate_sig());
            sig
        }
        fn contains<Q: ComponentTy>() -> bool {
            R1::contains::<Q>()
                || R2::contains::<Q>()
                || R3::contains::<Q>()
                || R4::contains::<Q>()
                || R5::contains::<Q>()
        }
    }

    impl<R1: QueryTy, R2: QueryTy, R3: QueryTy, R4: QueryTy, R5: QueryTy, R6: QueryTy> QueryTy
        for (R1, R2, R3, R4, R5, R6)
    {
        fn generate_sig() -> Signature {
            let mut sig = R1::generate_sig();
            sig.merge(&R2::generate_sig());
            sig.merge(&R3::generate_sig());
            sig.merge(&R4::generate_sig());
            sig.merge(&R5::generate_sig());
            sig.merge(&R6::generate_sig());
            sig
        }
        fn contains<Q: ComponentTy>() -> bool {
            R1::contains::<Q>()
                || R2::contains::<Q>()
                || R3::contains::<Q>()
                || R4::contains::<Q>()
                || R5::contains::<Q>()
                || R6::contains::<Q>()
        }
    }

    impl<
            R1: QueryTy,
            R2: QueryTy,
            R3: QueryTy,
            R4: QueryTy,
            R5: QueryTy,
            R6: QueryTy,
            R7: QueryTy,
        > QueryTy for (R1, R2, R3, R4, R5, R6, R7)
    {
        fn generate_sig() -> Signature {
            let mut sig = R1::generate_sig();
            sig.merge(&R2::generate_sig());
            sig.merge(&R3::generate_sig());
            sig.merge(&R4::generate_sig());
            sig.merge(&R5::generate_sig());
            sig.merge(&R6::generate_sig());
            sig.merge(&R7::generate_sig());
            sig
        }
        fn contains<Q: ComponentTy>() -> bool {
            R1::contains::<Q>()
                || R2::contains::<Q>()
                || R3::contains::<Q>()
                || R4::contains::<Q>()
                || R5::contains::<Q>()
                || R6::contains::<Q>()
                || R7::contains::<Q>()
        }
    }

    impl<
            R1: QueryTy,
            R2: QueryTy,
            R3: QueryTy,
            R4: QueryTy,
            R5: QueryTy,
            R6: QueryTy,
            R7: QueryTy,
            R8: QueryTy,
        > QueryTy for (R1, R2, R3, R4, R5, R6, R7, R8)
    {
        fn generate_sig() -> Signature {
            let mut sig = R1::generate_sig();
            sig.merge(&R2::generate_sig());
            sig.merge(&R3::generate_sig());
            sig.merge(&R4::generate_sig());
            sig.merge(&R5::generate_sig());
            sig.merge(&R6::generate_sig());
            sig.merge(&R7::generate_sig());
            sig.merge(&R8::generate_sig());
            sig
        }
        fn contains<Q: ComponentTy>() -> bool {
            R1::contains::<Q>()
                || R2::contains::<Q>()
                || R3::contains::<Q>()
                || R4::contains::<Q>()
                || R5::contains::<Q>()
                || R6::contains::<Q>()
                || R7::contains::<Q>()
                || R8::contains::<Q>()
        }
    }

    impl<
            R1: QueryTy,
            R2: QueryTy,
            R3: QueryTy,
            R4: QueryTy,
            R5: QueryTy,
            R6: QueryTy,
            R7: QueryTy,
            R8: QueryTy,
            R9: QueryTy,
        > QueryTy for (R1, R2, R3, R4, R5, R6, R7, R8, R9)
    {
        fn generate_sig() -> Signature {
            let mut sig = R1::generate_sig();
            sig.merge(&R2::generate_sig());
            sig.merge(&R3::generate_sig());
            sig.merge(&R4::generate_sig());
            sig.merge(&R5::generate_sig());
            sig.merge(&R6::generate_sig());
            sig.merge(&R7::generate_sig());
            sig.merge(&R8::generate_sig());
            sig.merge(&R9::generate_sig());
            sig
        }
        fn contains<Q: ComponentTy>() -> bool {
            R1::contains::<Q>()
                || R2::contains::<Q>()
                || R3::contains::<Q>()
                || R4::contains::<Q>()
                || R5::contains::<Q>()
                || R6::contains::<Q>()
                || R7::contains::<Q>()
                || R8::contains::<Q>()
                || R9::contains::<Q>()
        }
    }

    impl<
            R1: QueryTy,
            R2: QueryTy,
            R3: QueryTy,
            R4: QueryTy,
            R5: QueryTy,
            R6: QueryTy,
            R7: QueryTy,
            R8: QueryTy,
            R9: QueryTy,
            R10: QueryTy,
        > QueryTy for (R1, R2, R3, R4, R5, R6, R7, R8, R9, R10)
    {
        fn generate_sig() -> Signature {
            let mut sig = R1::generate_sig();
            sig.merge(&R2::generate_sig());
            sig.merge(&R3::generate_sig());
            sig.merge(&R4::generate_sig());
            sig.merge(&R5::generate_sig());
            sig.merge(&R6::generate_sig());
            sig.merge(&R7::generate_sig());
            sig.merge(&R8::generate_sig());
            sig.merge(&R9::generate_sig());
            sig.merge(&R10::generate_sig());
            sig
        }
        fn contains<Q: ComponentTy>() -> bool {
            R1::contains::<Q>()
                || R2::contains::<Q>()
                || R3::contains::<Q>()
                || R4::contains::<Q>()
                || R5::contains::<Q>()
                || R6::contains::<Q>()
                || R7::contains::<Q>()
                || R8::contains::<Q>()
                || R9::contains::<Q>()
                || R10::contains::<Q>()
        }
    }

    //generate for 11
    impl<
            R1: QueryTy,
            R2: QueryTy,
            R3: QueryTy,
            R4: QueryTy,
            R5: QueryTy,
            R6: QueryTy,
            R7: QueryTy,
            R8: QueryTy,
            R9: QueryTy,
            R10: QueryTy,
            R11: QueryTy,
        > QueryTy for (R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11)
    {
        fn generate_sig() -> Signature {
            let mut sig = R1::generate_sig();
            sig.merge(&R2::generate_sig());
            sig.merge(&R3::generate_sig());
            sig.merge(&R4::generate_sig());
            sig.merge(&R5::generate_sig());
            sig.merge(&R6::generate_sig());
            sig.merge(&R7::generate_sig());
            sig.merge(&R8::generate_sig());
            sig.merge(&R9::generate_sig());
            sig.merge(&R10::generate_sig());
            sig.merge(&R11::generate_sig());
            sig
        }
        fn contains<Q: ComponentTy>() -> bool {
            R1::contains::<Q>()
                || R2::contains::<Q>()
                || R3::contains::<Q>()
                || R4::contains::<Q>()
                || R5::contains::<Q>()
                || R6::contains::<Q>()
                || R7::contains::<Q>()
                || R8::contains::<Q>()
                || R9::contains::<Q>()
                || R10::contains::<Q>()
                || R11::contains::<Q>()
        }
    }

    //generate for 12
    impl<
            R1: QueryTy,
            R2: QueryTy,
            R3: QueryTy,
            R4: QueryTy,
            R5: QueryTy,
            R6: QueryTy,
            R7: QueryTy,
            R8: QueryTy,
            R9: QueryTy,
            R10: QueryTy,
            R11: QueryTy,
            R12: QueryTy,
        > QueryTy for (R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12)
    {
        fn generate_sig() -> Signature {
            let mut sig = R1::generate_sig();
            sig.merge(&R2::generate_sig());
            sig.merge(&R3::generate_sig());
            sig.merge(&R4::generate_sig());
            sig.merge(&R5::generate_sig());
            sig.merge(&R6::generate_sig());
            sig.merge(&R7::generate_sig());
            sig.merge(&R8::generate_sig());
            sig.merge(&R9::generate_sig());
            sig.merge(&R10::generate_sig());
            sig.merge(&R11::generate_sig());
            sig.merge(&R12::generate_sig());
            sig
        }
        fn contains<Q: ComponentTy>() -> bool {
            R1::contains::<Q>()
                || R2::contains::<Q>()
                || R3::contains::<Q>()
                || R4::contains::<Q>()
                || R5::contains::<Q>()
                || R6::contains::<Q>()
                || R7::contains::<Q>()
                || R8::contains::<Q>()
                || R9::contains::<Q>()
                || R10::contains::<Q>()
                || R11::contains::<Q>()
                || R12::contains::<Q>()
        }
    }

    //generate for 13
    impl<
            R1: QueryTy,
            R2: QueryTy,
            R3: QueryTy,
            R4: QueryTy,
            R5: QueryTy,
            R6: QueryTy,
            R7: QueryTy,
            R8: QueryTy,
            R9: QueryTy,
            R10: QueryTy,
            R11: QueryTy,
            R12: QueryTy,
            R13: QueryTy,
        > QueryTy for (R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12, R13)
    {
        fn generate_sig() -> Signature {
            let mut sig = R1::generate_sig();
            sig.merge(&R2::generate_sig());
            sig.merge(&R3::generate_sig());
            sig.merge(&R4::generate_sig());
            sig.merge(&R5::generate_sig());
            sig.merge(&R6::generate_sig());
            sig.merge(&R7::generate_sig());
            sig.merge(&R8::generate_sig());
            sig.merge(&R9::generate_sig());
            sig.merge(&R10::generate_sig());
            sig.merge(&R11::generate_sig());
            sig.merge(&R12::generate_sig());
            sig.merge(&R13::generate_sig());
            sig
        }
        fn contains<Q: ComponentTy>() -> bool {
            R1::contains::<Q>()
                || R2::contains::<Q>()
                || R3::contains::<Q>()
                || R4::contains::<Q>()
                || R5::contains::<Q>()
                || R6::contains::<Q>()
                || R7::contains::<Q>()
                || R8::contains::<Q>()
                || R9::contains::<Q>()
                || R10::contains::<Q>()
                || R11::contains::<Q>()
                || R12::contains::<Q>()
                || R13::contains::<Q>()
        }
    }

    //generate for 14
    impl<
            R1: QueryTy,
            R2: QueryTy,
            R3: QueryTy,
            R4: QueryTy,
            R5: QueryTy,
            R6: QueryTy,
            R7: QueryTy,
            R8: QueryTy,
            R9: QueryTy,
            R10: QueryTy,
            R11: QueryTy,
            R12: QueryTy,
            R13: QueryTy,
            R14: QueryTy,
        > QueryTy for (R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12, R13, R14)
    {
        fn generate_sig() -> Signature {
            let mut sig = R1::generate_sig();
            sig.merge(&R2::generate_sig());
            sig.merge(&R3::generate_sig());
            sig.merge(&R4::generate_sig());
            sig.merge(&R5::generate_sig());
            sig.merge(&R6::generate_sig());
            sig.merge(&R7::generate_sig());
            sig.merge(&R8::generate_sig());
            sig.merge(&R9::generate_sig());
            sig.merge(&R10::generate_sig());
            sig.merge(&R11::generate_sig());
            sig.merge(&R12::generate_sig());
            sig.merge(&R13::generate_sig());
            sig.merge(&R14::generate_sig());
            sig
        }
        fn contains<Q: ComponentTy>() -> bool {
            R1::contains::<Q>()
                || R2::contains::<Q>()
                || R3::contains::<Q>()
                || R4::contains::<Q>()
                || R5::contains::<Q>()
                || R6::contains::<Q>()
                || R7::contains::<Q>()
                || R8::contains::<Q>()
                || R9::contains::<Q>()
                || R10::contains::<Q>()
                || R11::contains::<Q>()
                || R12::contains::<Q>()
                || R13::contains::<Q>()
                || R14::contains::<Q>()
        }
    }
}

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
    //Returns all components of the given type for the entity the QueryFetch is associated with.
    fn get_components<C: ComponentTyReqs>(&'a self) -> Result<Vec<&Component<C>>> {
        //check if comp sig exists in signature
        if T::contains::<C>() {
            //access component
            let component = self
                .entman_ref
                .get_components_of_type::<C>(self.entity_id)?;
            return Ok(component);
        }
        Err(anyhow!("Component does not match signature"))
    }
}

pub struct QueryResult {}
///A sytem type is one that can execute logic on a given query
pub struct SystemTy {}
#[nvproc::query_predicate]
fn bob_predicate(f: QueryFetch<NameComponent>) -> bool {
    //Select entities with the name Bob
    name_components.into_iter().any(|c| c.name == "Bob")
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
        assert_eq!(qres.len(), 1);
        assert_eq!(qres[0].id, ent1);
        //test null predicate
        let q2 = Query::<NameComponent>::new();
        let qres2 = entman.query(&q2);
        assert_eq!(qres2.len(), 2);

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
        assert_eq!(qres3.len(), 1);
        assert_eq!(qres3[0].id, ent1);
        
    }
}
