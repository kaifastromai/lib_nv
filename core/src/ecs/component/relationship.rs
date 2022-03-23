use super::super::*;
use nvproc::Component;
use petgraph::graph::*;
use petgraph::visit::*;
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
    Custom(Custom),
}
pub struct Custom {
    pub name: String,
    pub description: String,
}
pub enum Minor {
    Child(Child),
    Custom(Custom),
}
pub enum Symmetric {
    Friend,
    Enemy,
    Sibling,
    Custom(Custom),
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
