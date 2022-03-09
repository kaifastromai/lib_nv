use super::super::*;
use nvproc::Component;
use petgraph::graph::*;
pub enum RelationshipGradient<Major: RelationshipTy, Minor: RelationshipTy> {
    Positive(Major, Minor),
    Negative(Major, Minor),
    Neutral(Major, Minor),
}

// impl<const Name: &'static str, const P: &'static str, const N: &'static str> crate::ecs::ComponentTy
//     for Relationship<Name, P, N>
// {
//     fn clean(&mut self) {
//         todo!()
//     }
// }
//A generic relationship between two entities.

pub trait RelationshipTy: 'static {
    fn get_name() -> String {
        std::any::type_name::<Self>().to_string()
    }
    fn get_relationship_name() -> String;
}
pub struct Father {}
impl RelationshipTy for Father {
    fn get_relationship_name() -> String {
        "FatherChild".to_string()
    }
}
pub struct Mother {}

impl RelationshipTy for Mother {
    fn get_relationship_name() -> String {
        "MotherChild".to_string()
    }
}
pub struct Son {}
impl RelationshipTy for Son {
    fn get_relationship_name() -> String {
        "SonParent".to_string()
    }
}

impl<Major: RelationshipTy, Minor: RelationshipTy> ecs::ComponentTy for Relationship<Major, Minor> {
    fn clean(&mut self) {
        todo!()
    }
}

pub struct Relationship<Major: RelationshipTy = Father, Minor: RelationshipTy = Son> {
    pub relationship_name: String,
    //The relationship's "direction"
    pub gradient: RelationshipGradient<Major, Minor>,
    //The entities the relationship is between.
    pairs: (Id, Id),
}
impl<Major: RelationshipTy, Minor: RelationshipTy> Relationship<Major, Minor> {
    pub fn new_static(pairs: (Id, Id), gradient: RelationshipGradient<Major, Minor>) -> Self {
        Self {
            relationship_name: Major::get_relationship_name(),
            gradient: gradient,
            pairs,
        }
    }
    pub fn get_pairs(&self) -> (Id, Id) {
        self.pairs
    }
    pub fn get_gradient(&self) -> RelationshipGradient<Major, Minor> {
        self.gradient
    }
}

pub struct RelationshipGraph {
    pub graph: DiGraph<Id, Relationship>,
}
pub struct Relation<T: RelationshipTy> {
    phantom: std::marker::PhantomData<T>,
}
impl<T: RelationshipTy> From<&'static str> for Relation<T> {
    fn from(name: &'static str) -> Self {
        Self {
            phantom: std::marker::PhantomData,
        }
    }
}

#[cfg(test)]
mod test_relationship {
    use super::*;

    #[test]
    fn test_basic() {
        let graph = RelationshipGraph {
            graph: DiGraph::new(),
        };
        let ent1 = uuid::generate();
        let ent2 = uuid::generate();
        let ent3 = uuid::generate();
        let father = Relationship {
            relationship_name: "FatherSon",
            gradient: RelationshipGradient::Positive("Father", "Son"),
            pairs: (ent1, ent2),
        };
        let mother = Relationship::new_static(
            (ent1, ent2),
            RelationshipGradient::Positive("Mother", "Son"),
        );

        graph.graph.add_edge(ent1, ent2, father);
        graph.graph.add_edge(ent3, ent2, mother);
    }
}
