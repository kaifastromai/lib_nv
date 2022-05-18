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
struct NullPredicate<T> {
    _marker: PhantomData<T>,
}
impl<Q: QueryTy> PredicateTy<Q> for NullPredicate<Q> {
    fn check(&self, _: QueryFetch<Q>) -> bool {
        true
    }
}

pub trait PredicateTy<Q>
where
    Q: QueryTy,
{
    fn check(&self, fetch: QueryFetch<Q>) -> bool;
}
impl<Q: QueryTy, T: Fn<(QueryFetch<Q>,), Output = bool>> PredicateTy<Q> for T {
    fn check(&self, fetch: QueryFetch<Q>) -> bool {
        self(fetch)
    }
}
///A [Query] that retrieves components, or Entities from} the ECS (Entman)
pub struct Query<T: QueryTy, P = NullPredicate<T>>
where
    P: PredicateTy<T>,
{
    phantom: std::marker::PhantomData<T>,
    predicate: P,
    matching_entities: Option<Vec<Id>>,
}
impl<T: QueryTy, P: PredicateTy<T>> Query<T, P> {
    fn new(p: P) -> Self {
        Query {
            phantom: std::marker::PhantomData,
            predicate: p,
            matching_entities: None,
        }
    }
    fn get_query_sig() -> Signature {
        T::generate_sig()
    }
}
struct ConstAssert<const Assert: bool> {}

///A query fetch allows statically known access to the components of an entity (hopefully).
/// It essentially a wrapper over an Entity, but allows direct access to the components since
/// We can be guaranteed that components exist.
pub struct QueryFetch<T: QueryTy> {
    phantom: std::marker::PhantomData<T>,
    entity_ids: Vec<Id>,
    component_ids: Vec<Id>,
}
impl<T: QueryTy> QueryFetch<T> {
    fn new(entity_ids: Vec<Id>, component_ids: Vec<Id>) -> Self {
        QueryFetch {
            phantom: std::marker::PhantomData,
            entity_ids,
            component_ids,
        }
    }
    fn get_components<'a, C: ComponentTyReqs>(
        &'a self,
        entman: &'a Entman,
        entity: Id,
    ) -> Result<Vec<&Component<C>>>
    where
        ConstAssert<{ T::contains::<C>() }>:,
    {
        let signature = C::generate_sig();
        let comp_id = C::get_type_id();
        //check if comp sig exists in signature
        if signature.contains(comp_id) {
            //access component
            let component = entman.get_components_of_type::<C>(entity)?;
            return Ok(component);
        }
        Err(anyhow!("Component does not match signature"))
    }
}

pub struct QueryResult {}
///A sytem type is one that can execute logic on a given query
pub struct SystemTy {}

#[cfg(test)]
mod test_query {

    use crate::ecs::component::components::*;

    use super::*;

    fn test_pred(fetch: QueryFetch<LocationComponent>) -> bool {
        true
    }

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
        let q = Query::new(pred);
    }
    #[test]
    fn test_query_fetch() {
        let mut entman = Entman::new();
        let ent_id = entman.add_entity();
        let loc_id = entman
            .add_component_default::<LocationComponent>(ent_id)
            .unwrap();
        let name_id = entman
            .add_component_default::<NameComponent>(ent_id)
            .unwrap();
        let fetch = QueryFetch::<(LocationComponent, NameComponent)>::new(
            vec![ent_id],
            vec![loc_id.into(), name_id.into()],
        );
        let loc_comp = fetch
            .get_components::<LocationComponent>(&entman, ent_id)
            .unwrap();
        let name_comp = fetch
            .get_components::<NameComponent>(&entman, ent_id)
            .unwrap();
        assert_eq!(loc_comp.len(), 1);
        assert_eq!(name_comp.len(), 1);
    }
}
