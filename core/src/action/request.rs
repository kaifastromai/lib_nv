use std::any::{self, Any};

use crate::mir::{Mir, MirData};

use super::ParamTy;
use common::exports::anyhow::{anyhow, Result};

///A request is ultimately just a function that takes a reference to Mir, and a generic parameter P that is the type of the parameter
/// And the type of data it is expected to return

//A request asks for data/information from Mir
pub trait RequestTy {}
//A response returns the requested data/information to Mir.
//A response cannot contain any references to Mir. Everything must be cloned.
pub trait ResponseTy {}
impl<T: Any> ResponseTy for T {}

pub struct Request<'a, R: ResponseTy, P: Clone> {
    pub param: P,
    req_fn: &'a dyn Fn(&mut MirData, P) -> Result<R>,
}
impl<'a, R: ResponseTy, P: Clone> Request<'a, R, P> {
    pub fn new(req_fn: &'a dyn Fn(&mut MirData, P) -> Result<R>, param: P) -> Self {
        Request { param, req_fn }
    }
    pub fn exec(&self, mir: &mut MirData) -> Result<R> {
        (self.req_fn)(mir, self.param.clone())
    }
}
//Manages requests and responses for Mir.
pub struct Reqman<'r> {
    mir_ref: &'r mut MirData,
}
impl<'r> Reqman<'r> {
    pub fn new(mir_ref: &'r mut MirData) -> Self {
        Reqman { mir_ref }
    }
    pub fn request<R: ResponseTy, P: Clone>(&mut self, req: Request<R, P>) -> Result<R> {
        let res = req.exec(self.mir_ref)?;
        Ok(res)
    }
}
