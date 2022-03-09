use super::super::*;
use nvproc::Component;
use petgraph::graph::*;
pub enum RelationshipGradient {
    Positive,
    Negative,
    Neutral,
}

// impl<const Name: &'static str, const P: &'static str, const N: &'static str> crate::ecs::ComponentTy
//     for Relationship<Name, P, N>
// {
//     fn clean(&mut self) {
//         todo!()
//     }
// }
//A generic relationship between two entities.

pub enum ERelationship {
    MajorMinor(Major, Minor),
    Symmetric(Symmetric),
}
impl ERelationship {
    fn parent_child(parent: Parent, child: Child) -> Self {
        ERelationship::MajorMinor(Major::Parent(parent), Minor::Child(child))
    }
}
pub enum Major {
    Parent(Parent),
}
pub enum Minor {
    Child(Child),
    Custom(String),
}
pub enum Symmetric {
    Friend,
    Enemy,
    Sibling,
    Custom(String),
}

pub enum Parent {
    Mother,
    Father,
}
pub enum Child {
    Daughter,
    Son,
}
pub enum Sibling {
    Sister,
    Brother,
}
pub enum Spouse {
    Husband,
    Wife,
}

pub struct Relationship {
    pub relationship_name: String,
    pub relation: ERelationship,
    //The entities the relationship is between.
    pairs: (Id, Id),
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
        let ent3 = uuid::generate();
        let father = Relationship {
            relationship_name: "FatherSon".to_string(),
            relation: ERelationship::parent_child(Parent::Father, Child::Son),
            pairs: (ent1, ent2),
        };
        let m = Parent::Mother;
        let mother = Relationship {
            relationship_name: "MotherSon".to_string(),
            relation: ERelationship::parent_child(Parent::Mother, Child::Son),

            pairs: (ent1, ent3),
        };

        //create nodes
        let n1 = graph.graph.add_node(ent1);
        let n2 = graph.graph.add_node(ent2);
        let n3 = graph.graph.add_node(ent3);

        graph.graph.add_edge(n1, n2, father);
        graph.graph.add_edge(n3, n2, mother);
        

    }
}
