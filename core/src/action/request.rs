/*!A request is ultimately just a function that takes an immutable reference to Mir, and a generic parameter P that is the type of the parameter
 And the type of data it is expected to return.
 It is analogous to actions without the notion of state or do/undo stacks.
*/
use std::any::{self, Any};

use crate::mir::Mir;

use super::ParamTy;
use common::exports::anyhow::{anyhow, Result};

//A request asks for data/information from Mir
pub trait ReqTy<P, R>: Fn(&mut Mir, P) -> Result<R> {}

impl<R, P, T: Fn(&mut Mir, P) -> Result<R>> ReqTy<P, R> for T {}
//A response returns the requested data/information to Mir.
//A response cannot contain any references to Mir. Everything must be cloned.
pub trait ResTy {}
impl<T: Any> ResTy for T {}

pub struct Request<R: ResTy, P: Clone, Rq: ReqTy<P, R>> {
    pub phantom_param: std::marker::PhantomData<P>,
    req_fn: Rq,
    phantom: std::marker::PhantomData<R>,
}
impl<R: ResTy, P: Clone, Rq: ReqTy<P, R>> Request<R, P, Rq> {
    pub const fn new(req_fn: Rq) -> Self {
        Request {
            phantom_param: std::marker::PhantomData,
            req_fn,
            phantom: std::marker::PhantomData,
        }
    }
    pub fn exec(&self, mir: &mut Mir, p: P) -> Result<R> {
        (self.req_fn)(mir, p)
    }
}
//Manages requests and responses for Mir.
pub struct Reqman {}
impl Reqman {
    pub fn new() -> Self {
        Reqman {}
    }
    pub fn request<R: ResTy, P: Clone, Rq: ReqTy<P, R>>(
        &mut self,
        req: Request<R, P, Rq>,
        mir: &mut Mir,
        param: P,
    ) -> Result<R> {
        let res = req.exec(mir, param)?;
        Ok(res)
    }
}
mod requests {
    use super::*;
    use crate::{
        ecs::{EntityOwned, Id},
        mir::Mir,
    };
    ///Get the number of current entities in the Mir.
    pub static R_GET_ENTITY_COUNT: Request<usize, (), fn(&mut Mir, ()) -> Result<usize>> =
        Request::new(|mir: &mut Mir, _: ()| Ok(mir.em.get_entity_count()));

        ///Get an owned entity by its id.
    pub static R_GET_ENTITY_OWNED: Request<
        EntityOwned,
        Id,
        fn(&mut Mir, Id) -> Result<EntityOwned>,
    > = Request::new(|mir: &mut Mir, id: Id| mir.em.get_entity_owned(id));
}
