use super::super::*;

pub enum RelationshipGradient {
    Positive,
    Negative,
    Neutral,
}
//A generic relationship between two entities.
pub struct Relationship {
    pub description: String,
    pub gradient: RelationshipGradient,
    pairs: (Id, Id),
}

pub struct RelationshipGraph{
    pub graph: Vec<Relationship>,
}