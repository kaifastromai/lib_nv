use common::type_id::TypeIdTy;

use super::{ComponentTy, Signature};

///The trait representing queryable types
pub trait QueryTy {
    fn generate_sig() -> Signature;
}
//implement QueryTy for all T that implement TypeIdTy and are ComponentTy
impl<T: TypeIdTy + ComponentTy> QueryTy for T {
    fn generate_sig() -> Signature {
        <T as TypeIdTy>::get_type_id().into()
    }
}
//implement Query for tuple of QueryTy
impl<R1: QueryTy> QueryTy for (R1,) {
    fn generate_sig() -> Signature {
        R1::generate_sig()
    }
}
impl<R1: QueryTy, R2: QueryTy> QueryTy for (R1, R2) {
    fn generate_sig() -> Signature {
        let mut sig = R1::generate_sig();
        sig.merge(&R2::generate_sig());
        sig
    }
}
impl<R1: QueryTy, R2: QueryTy, R3: QueryTy> QueryTy for (R1, R2, R3) {
    fn generate_sig() -> Signature {
        let mut sig = R1::generate_sig();
        sig.merge(&R2::generate_sig());
        sig.merge(&R3::generate_sig());
        sig
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
}
impl<R1: QueryTy, R2: QueryTy, R3: QueryTy, R4: QueryTy, R5: QueryTy, R6: QueryTy, R7: QueryTy>
    QueryTy for (R1, R2, R3, R4, R5, R6, R7)
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
}

///The [NullPredicate] always returns true. For internal use only.
struct NullPredicate {}

impl PredicateTy for NullPredicate {
    type Query = ();
    fn check(&self, _: QueryFetch<Self::Query>) -> bool {
        true
    }
}
fn test_pred<T: QueryTy>(pred: QueryFetch<T>) -> bool {
    false
}
pub trait PredicateTy<Ph = ()> {
    type Query: QueryTy;
    fn check(&self, fetch: QueryFetch<Self::Query>) -> bool;
}
impl<Q: QueryTy, T: Fn<(QueryFetch<Q>,), Output = bool>> PredicateTy<Q> for T {
    type Query = Q;
    fn check(&self, fetch: QueryFetch<Self::Query>) -> bool {
        self(fetch)
    }
}
///A [Query] that retrieves components, or Entities from} the ECS (Entman)
pub struct Query<T: QueryTy, P>
where
    P: PredicateTy,
{
    phantom: std::marker::PhantomData<T>,
    predicate: P,
}
impl<T: QueryTy, P: PredicateTy> Query<T, P> {
    fn new(p: P) -> Self {
        Query {
            phantom: std::marker::PhantomData,
            predicate: p,
        }
    }
    fn get_query_sig() -> Signature {
        T::generate_sig()
    }
}

///A query fetch allows statically known access to the components of an entity (hopefully).
/// It essentially a wrapper over an Entity, but allows direct access to the components since
/// We can be guaranteed that components exist.
pub struct QueryFetch<T: QueryTy> {
    phantom: std::marker::PhantomData<T>,
}


pub struct QueryResult {}
///A sytem type is one that can execute logic on a given query
pub struct SystemTy {}

#[cfg(test)]
mod test_query {
    use crate::ecs::component::components::*;

    use super::*;

    #[test]
    fn test_sig_gen() {
        let query = Query::<(LocationComponent, NameComponent), NullPredicate>::get_query_sig();
        let comp_sig = Signature::from(vec![
            LocationComponent::get_type_id(),
            NameComponent::get_type_id(),
        ]);
        assert_eq!(query, comp_sig);
    }
    #[test]
    fn test_preds() {
        let pred = |fetch: QueryFetch<(LocationComponent, NameComponent)>| -> bool {
            return true;
        };
        let q = Query::<(LocationComponent, NameComponent), _>::new(test_pred);

    }
}
