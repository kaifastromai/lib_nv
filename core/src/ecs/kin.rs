//! Represents familial relationships between persons (that reproduce offspring via dimorphic sexual relations).

use std::{
    collections::{hash_map::DefaultHasher, BTreeMap, BTreeSet, VecDeque},
    hash::{Hash, Hasher},
};

///Describes the possible fundamental types of relationships (that is, all others
/// can be represented as a combination of these).
///
//generic graph library
use common::{
    exports::petgraph::Direction,
    exports::petgraph::{graph::*, visit::IntoEdgesDirected},
    uuid,
};
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Parent,
    //The inverse of a parent
    Child,
    //symmetric
    Sibling,
    //Reproductive partner, symmetric
    Repat,
}
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum Sex {
    Male,
    Female,
}
pub struct AncestorGraphElement {
    id: usize,
    kind: Kind,
    depth: u32,
}
type Angre = AncestorGraphElement;
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct Person {
    id: usize,
    sex: Sex,
}
impl Person {
    pub fn new(sex: Sex) -> Self {
        //assign random (hopefully) unique id.
        let id = uuid::gen_64();
        //on 32bit os's this will be a problem, but we'll think about that later
        Person {
            id: id as usize,
            sex,
        }
    }
}
impl From<Person> for NodeIndex<usize> {
    fn from(p: Person) -> Self {
        NodeIndex::from(p.id as usize)
    }
}
impl From<&Person> for NodeIndex<usize> {
    fn from(p: &Person) -> Self {
        NodeIndex::from(p.id as usize)
    }
}
type Id = usize;
///Represents the general directed graph
pub struct KinGraph {
    graph: DiGraph<Person, Kind, usize>,
    //a map between a node index and the id of the person. This is probably
    //not necessary since it appears that DiGraph already increments directly,
    //but I have no absoulte proof of this, so...
    id_indx: BTreeMap<usize, NodeIndex<usize>>,
}
impl KinGraph {
    fn new() -> Self {
        KinGraph {
            graph: Graph::default(),
            id_indx: BTreeMap::new(),
        }
    }
    fn add_person(&mut self, p: &Person) {
        let id = p.id;
        let idx = self.graph.add_node(p.clone());
        self.id_indx.insert(id, idx);
    }
    fn add_relation(&mut self, p1: &Person, p2: &Person, kind: Kind) {
        let p1x = self.idx(p1).unwrap();
        let p2x = self.idx(p2).unwrap();
        self.graph.add_edge(p1x, p2x, kind.clone());
        //add the inverse relation
        match kind {
            Kind::Parent => self.graph.add_edge(p2x, p1x, Kind::Child),
            Kind::Child => self.graph.add_edge(p1x, p2x, Kind::Parent),
            Kind::Sibling => self.graph.add_edge(p2x, p1x, Kind::Sibling),
            Kind::Repat => self.graph.add_edge(p2x, p1x, Kind::Repat),
        };
    }
    ///Does a depth first traversal of the graph, and finding the shortest path
    /// returns a list of AncestorGraphElements
    fn build_ancestor_tree(&self, p: &Person) -> Vec<Angre> {
        let nbs = self.graph.neighbors_directed(p.into(), Direction::Outgoing);
        todo!()
    }
    pub fn find_eldest_ancestor(&self, p: &Person) -> Person {
        let mut cidx = self.idx(p).unwrap();
        let nbs = self
            .graph
            .neighbors_directed(self.idx(p).unwrap(), Direction::Outgoing);
        //find using depth-first search
        let mut visited_stack = VecDeque::<NodeIndex<usize>>::new();
        //used to back track
        let mut v2 = VecDeque::<NodeIndex<usize>>::new();
        //We use this to keep track of the depth when we reset to a previous node in the depth first search
        let mut depth_set = BTreeMap::<usize, i32>::new();

        v2.push_back(cidx);

        println!("->|The value is {:?}|", cidx);
        //represents the depth of eldest element, the sign of this to communicate the idea that all children
        //are geneologically 'below' their ancestors.
        let mut eldest_depth = 0;
        //How many times sideways (siblings, and maybe Repat? )we go to find common ancestor
        let mut eldest_side_drift = 0u32;
        let mut current_depth = 0;

        while !v2.is_empty() {
            depth_set.insert(cidx.index(), current_depth);
            //add this to visited
            visited_stack.push_back(cidx);
            let mut next_i = 0;
            let nit = self
                .graph
                .neighbors_directed(cidx, Direction::Outgoing)
                .collect::<Vec<NodeIndex<usize>>>();

            //This is known to be 1 (only 1 outgoing edge between two nodes)
            //We only want to follow the paths that are even or climb the ancestor tree (the links that make us the child)
            let e = |n: usize| self.graph.edges_connecting(cidx, nit[n]).next().unwrap();
            let edge_cond = |n| {
                (*e(n).weight() == Kind::Child
                    || *e(n).weight() == Kind::Sibling
                    || *e(n).weight() == Kind::Repat)
            };
            while next_i < nit.len() && (visited_stack.contains(&nit[next_i]) || !edge_cond(next_i))
            {
                next_i += 1;
            }

            //We've searched all neighbors, and already visited them
            if next_i == nit.len() {
                //go back, we're done
                cidx = v2.pop_back().unwrap();
                current_depth = *depth_set.get(&cidx.index()).unwrap();

                continue;
            }

            //debugging purposes
            let nidx = nit[next_i];

            //if the chosen path is Child, then our depth decreases
            //else if the chosen path is a sibling or a repat, our sideways drift increases
            match e(next_i).weight() {
                //Depth
                Kind::Child => {
                    current_depth -= 1;
                }
                //Sideways
                _ => {
                    //I believe this most be changed to include the direction of sideways movement
                }
            }

            v2.push_back(cidx);
            cidx = nit[next_i];
            println!("->|The value is {:?}|", cidx);
            if current_depth < eldest_depth {
                eldest_depth = current_depth
            }
        }
        //print depth
        println!("->| Eldest depth is: {:}", eldest_depth);

        Person::new(Sex::Male)
    }
    ///Get NodeIndex from person
    fn idx(&self, p: &Person) -> Option<NodeIndex<usize>> {
        self.id_indx.get(&p.id).cloned()
    }
}
#[cfg(test)]
mod test_kin {
    use super::*;

    #[test]
    fn test_main() {
        let mut kg = KinGraph::new();
        //make some persons, the sexes aren't important
        let p0 = Person::new(Sex::Female);
        let p1 = Person::new(Sex::Female);
        let p2 = Person::new(Sex::Male);
        let p3 = Person::new(Sex::Male);
        let p4 = Person::new(Sex::Female);
        let p5 = Person::new(Sex::Female);
        let p6 = Person::new(Sex::Male);
        //add them as nodes
        kg.add_person(&p0);
        kg.add_person(&p1);
        kg.add_person(&p2);
        kg.add_person(&p3);
        kg.add_person(&p4);
        kg.add_person(&p5);
        kg.add_person(&p6);
        //make relationships
        kg.add_relation(&p0, &p1, Kind::Parent);
        kg.add_relation(&p2, &p1, Kind::Parent);
        //from here, we have enough information to deduce that p1 and p3 are Repat
        //In real life, we could run some preprocessor over the raw graph to make this and perhaps
        //other observations more explicit, and also to verify that the graph is well formed
        //(i.e. a node cannot have more than 2 parents, and those two cannot be of the same sex,
        // a node can't parent itself, and a node cannot be connected to another node more than once in incompatible ways )
        //give p2 a child
        kg.add_relation(&p1, &p3, Kind::Parent);
        //and a spouse
        kg.add_relation(&p1, &p4, Kind::Repat);

        //p5 is parent of p4
        kg.add_relation(&p4, &p3, Kind::Parent);
        //p6 is sibling of p4
        kg.add_relation(&p5, &p3, Kind::Sibling);
        //and p6 is parent of p7
        kg.add_relation(&p5, &p6, Kind::Parent);

        kg.find_eldest_ancestor(&p3);
    }
}
