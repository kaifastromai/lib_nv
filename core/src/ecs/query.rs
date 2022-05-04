///The trait representing queryable types
pub trait QueryTy {}

///A [Query] that retrieves components, or Entities from the ECS (Entman)
pub struct Query<T: QueryTy> {}

///A sytem type is one that can execute logic on a given query
pub struct SystemTy {}
