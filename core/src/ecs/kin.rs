//! A rewrite of the relation system with a fancy new name!
pub trait MajorTy {}
pub trait MinorTy {}
pub trait SymmetricTy {}

///Represents an asymmetric relationship between two entities.
pub struct Asymmetric<T: MajorTy, U: MinorTy> {
    
    pub major: T,
    pub minor: U,
}
///Represents a symmetric relationship between two entities.
pub struct Symmetric<T: SymmetricTy> {
    pub symmetric: T,
}
pub struct Relation{

}