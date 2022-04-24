use super::super::*;
use nvproc::Component;
use petgraph::graph::*;
use petgraph::visit::*;
#[nvproc::bincode_derive]
#[nvproc::serde_derive]
pub enum ERelationship {
    MajorMinor(Major, Minor),
    Symmetric(Symmetric),
}
impl ERelationship {
    pub fn parent_child(parent: Parent, child: Child) -> Self {
        ERelationship::MajorMinor(Major::Parent(parent), Minor::Child(child))
    }
}
impl Default for ERelationship {
    fn default() -> Self {
        ERelationship::Symmetric(Symmetric::default())
    }
}
#[nvproc::bincode_derive]
#[nvproc::serde_derive]
pub enum Major {
    Parent(Parent),
    Custom(Custom),
}
#[nvproc::bincode_derive]
#[nvproc::serde_derive]

pub struct Custom {
    pub name: String,
    pub description: String,
}
#[nvproc::bincode_derive]
#[nvproc::serde_derive]
pub enum Minor {
    Child(Child),
    Custom(Custom),
}
#[nvproc::bincode_derive]
#[nvproc::serde_derive]

pub enum Symmetric {
    Friend,
    Enemy,
    Sibling,
    Spouse,
    Custom(Custom),
}
impl Default for Symmetric {
    fn default() -> Self {
        Symmetric::Friend
    }
}

#[nvproc::bincode_derive]
#[nvproc::serde_derive]

pub enum Parent {
    Mother,
    Father,
}
#[nvproc::bincode_derive]
#[nvproc::serde_derive]

pub enum Child {
    Daughter,
    Son,
}
#[nvproc::bincode_derive]
#[nvproc::serde_derive]

pub enum Sibling {
    Sister,
    Brother,
}
#[nvproc::bincode_derive]
#[nvproc::serde_derive]

pub enum Spouse {
    Husband,
    Wife,
}
#[nvproc::bincode_derive]
#[nvproc::serde_derive]
#[derive(Default)]

pub struct Relationship {
    pub relationship_name: String,
    pub relation: ERelationship,
    //The entities the relationship is between.
    pairs: (Id, Id),
}
impl Relationship {
    pub fn new(name: String, relation: ERelationship, pairs: (Id, Id)) -> Self {
        Relationship {
            relationship_name: name,
            relation,
            pairs,
        }
    }
    pub fn get_name(&self) -> &str {
        &self.relationship_name
    }
    pub fn get_relation(&self) -> &ERelationship {
        &self.relation
    }
    pub fn get_major_pair(&self) -> Id {
        self.pairs.0
    }
    pub fn get_minor_pair(&self) -> Id {
        self.pairs.1
    }
    pub fn parent_child<T, U>(parent: T, child: U, parent_id: Id, child_id: Id) -> Self
    where
        T: Into<Parent>,
        U: Into<Child>,
    {
        Self::new(
            String::from("ParentChild"),
            ERelationship::parent_child(parent.into(), child.into()),
            (parent_id, child_id),
        )
    }
    pub fn symmetric<T>(relation: T, first: Id, second: Id) -> Self
    where
        T: Into<Symmetric>,
    {
        Self::new(
            String::from("Symmetric"),
            ERelationship::Symmetric(relation.into()),
            (first, second),
        )
    }
}
pub struct RelationshipGraph {
    pub graph: DiGraph<Id, Relationship>,
}
impl RelationshipGraph {
    pub fn new() -> Self {
        RelationshipGraph {
            graph: DiGraph::new(),
        }
    }
    pub fn find_node_index(&self, id: Id) -> Option<NodeIndex<u32>> {
        for (i, n) in self.graph.node_weights().enumerate() {
            if *n == id {
                return Some(NodeIndex::new(i.into()));
            }
        }
        None
    }
}
#[cfg(test)]
mod test_relationship {
    use std::{fs::File, io::Write};

    use super::*;

    #[test]
    fn test_basic() {
        let mut graph = RelationshipGraph {
            graph: DiGraph::new(),
        };
        let ent1 = uuid::gen_128();
        let ent2 = uuid::gen_128();
        let ent3 = uuid::gen_128();
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
