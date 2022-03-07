use super::super::*;
use nvproc::Component;
use petgraph::graph::*;
pub enum RelationshipGradient<const P: &'static str = "", const N: &'static str = ""> {
    Positive(&'static str, &'static str),
    Negative(&'static str, &'static str),
    Neutral(&'static str, &'static str),
}

// impl<const Name: &'static str, const P: &'static str, const N: &'static str> crate::ecs::ComponentTy
//     for Relationship<Name, P, N>
// {
//     fn clean(&mut self) {
//         todo!()
//     }
// }
//A generic relationship between two entities.
#[derive(Default, Component)]
pub struct Relationship<
    const Name: &'static str = "",
    const P: &'static str = "",
    const N: &'static str = "",
> {
    pub relationship_name: &'static str,
    //The relationship's "direction"
    pub gradient: RelationshipGradient<P, N>,
    //The entities the relationship is between.
    pairs: (Id, Id),
}
impl<const Name: &'static str, const P: &'static str, const N: &'static str>
    Relationship<Name, P, N>
{
    pub fn new(pairs: (Id, Id)) -> Self {
        Self {
            relationship_name: Name,
            gradient: RelationshipGradient::Neutral(P, N),
            pairs,
        }
    }
    pub fn get_pairs(&self) -> (Id, Id) {
        self.pairs
    }
    pub fn get_gradient(&self) -> RelationshipGradient<P, N> {
        self.gradient
    }
    pub fn set_gradient(&mut self, gradient: RelationshipGradient<P, N>) {
        self.gradient = gradient;
    }
}

pub struct RelationshipGraph {
    pub graph: DiGraph<Id, Relationship>,
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
        let rel = Relationship {
            relationship_name: "ParentChild",
            gradient: RelationshipGradient::Positive("Parent", "Child"),
            pairs: (ent1, ent2),
        };
    }
}
