//! Represents familial relationships between persons (that reproduce offspring via dimorphic sexual relations).

use std::{
    collections::{hash_map::DefaultHasher, BTreeMap, BTreeSet, VecDeque},
    hash::{Hash, Hasher},
    ops::{Add, Sub},
};

///Describes the possible fundamental types of relationships (that is, all others
/// can be represented as a combination of these).
///
//generic graph library
use common::{
    exports::anyhow::{anyhow, Result},
    exports::petgraph::algo::*,
    exports::petgraph::Direction,
    exports::petgraph::{graph::*, visit::IntoEdgesDirected},
    uuid,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Parent = 1,
    //The inverse of a parent
    Child = -1,
    //symmetric
    Sibling = 2,
    //Reproductive partner, symmetric
    Repat = 3,
}
impl Kind {
    fn get_value(&self) -> i32 {
        match self {
            Kind::Parent => 1,
            Kind::Child => -1,
            //This should probably be changed later, but for now this is easier...
            Kind::Sibling => 1,
            //Experiment with zero length repat
            Kind::Repat => 0,
        }
    }
    fn get_cost(&self) -> u32 {
        match self {
            Kind::Parent => 1,
            Kind::Child => 1,
            Kind::Sibling => 1,
            Kind::Repat => 1,
        }
    }
}
//This represents the location of the person in the family tree.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Location {
    ///depth of the person in the tree, indicated by [Kind::Parent]
    d: i32,
    ///the 'width' of the person in the tree, this the 'sideways' movement indicated by [Kind::Sibling] or [Kind::Repat]
    w: i32,
}
impl Location {
    fn dot(&self, other: Location) -> i32 {
        self.d * other.w + self.w * other.d
    }
}
//impl display for Location
impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(d: {},w: {})", self.d, self.w)
    }
}
//impl Add and Sub for Location
impl Add for Location {
    type Output = Location;
    fn add(self, other: Location) -> Location {
        Location {
            d: self.d + other.d,
            w: self.w + other.w,
        }
    }
}
impl Sub for Location {
    type Output = Location;
    fn sub(self, other: Location) -> Location {
        Location {
            d: self.d - other.d,
            w: self.w - other.w,
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, PartialOrd, Ord, Debug)]
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
#[derive(Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Debug)]
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
    depth_map: Option<BTreeMap<Person, Location>>,
}
impl KinGraph {
    fn new() -> Self {
        KinGraph {
            graph: Graph::default(),
            id_indx: BTreeMap::new(),
            depth_map: None,
        }
    }
    fn add_person(&mut self, p: &Person) {
        let id = p.id;
        let idx = self.graph.add_node(p.clone());
        self.id_indx.insert(id, idx);
    }
    ///Adds a relationship.
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
    ///Depracated
    pub fn get_canonical_relationship(&mut self, p1: &Person, p2: &Person) -> Result<String> {
        //make sure the nodes are not the same
        if p1 == p2 {
            return Err(anyhow!("Cannot have a relationship with yourself"));
        }

        self.build_map(&p1);
        let path = self.find_shortest_path_astar(p1, p2);
        let dm_ref = self.depth_map.as_ref().unwrap();
        let depth_delta = dm_ref.get(p1).unwrap().d - dm_ref.get(p2).unwrap().d;
        let width_delta = dm_ref.get(p1).unwrap().w - dm_ref.get(p2).unwrap().w;

        let n_child = |n| {
            let gendered_name = match p2.sex {
                Sex::Male => "son",
                Sex::Female => "daughter",
            };
            match n {
                1 => gendered_name.to_owned(),
                2 => format!("grand{:}", gendered_name),
                n => {
                    let mut greats = vec![];
                    for i in 2..n {
                        greats.push("great".to_owned());
                    }
                    format!("{:} grand{:}", greats.join(" "), gendered_name)
                }
            }
        };
        //check simple cases
        match (depth_delta, width_delta, path) {
            //child
            (n, 0, p) => {
                //if the path is the same length as the depth, then it is an n child
                if p.len() - 1 == n.abs() as usize {
                    return Ok(n_child(n.abs() as usize));
                } else {
                    return Err(anyhow!("Not tracked yet"));
                }
            }
            _ => {
                return Err(anyhow!("Not tracked yet"));
            }
        }
    }

    ///This builds the depth map. It must be done after all the relations are added.
    /// Starts at the given root person, instead of some global.
    ///  This allows us to store multiple disconnected family trees.
    pub fn build_map(&mut self, root: &Person) {
        let mut depth_map = BTreeMap::<Person, Location>::new();
        //first index
        let mut cidx = self.idx(root).unwrap();
        let nbs = self.graph.neighbors_directed(cidx, Direction::Outgoing);
        //find using depth-first search
        let mut visited_stack = VecDeque::<NodeIndex<usize>>::new();
        //used to back track
        let mut v2 = VecDeque::<NodeIndex<usize>>::new();
        //We use this to keep track of the depth when we reset to a previous node in the depth first search
        let mut depth_set = BTreeMap::<usize, i32>::new();

        v2.push_back(cidx);

        //our current depth
        let mut cur_loc = Location { d: 0, w: 0 };

        while !v2.is_empty() {
            visited_stack.push_back(cidx);
            //add this to the map
            depth_map.insert(self.graph[cidx].clone(), cur_loc.clone());
            let mut next_i = 0;
            let nit = self
                .graph
                .neighbors_directed(cidx, Direction::Outgoing)
                .collect::<Vec<NodeIndex<usize>>>();

            //This is known to be 1 (an invariant) (only 1 outgoing edge between two nodes), if broken, our graph is illformed
            //We only want to follow the paths that are even or climb the ancestor tree (the links that make us the child)
            let e = |n: usize| self.graph.edges_connecting(cidx, nit[n]).next().unwrap();

            while next_i < nit.len() && (visited_stack.contains(&nit[next_i])) {
                next_i += 1;
            }

            //We've searched all neighbors, and already visited them
            if next_i == nit.len() {
                //go back, we're done
                cidx = v2.pop_back().unwrap();
                cur_loc = *depth_map.get(&self.graph[cidx]).unwrap();
                continue;
            }

            //if the chosen path is Child, then our depth decreases
            //else if the chosen path is a sibling or a repat, our sideways drift increases
            match e(next_i).weight() {
                //Depth
                Kind::Child => {
                    cur_loc.d += 1;
                }
                Kind::Parent => {
                    cur_loc.d -= 1;
                }
                //Sideways
                k => cur_loc.w += k.get_value(),
            }

            v2.push_back(cidx);
            cidx = nit[next_i];
        }
        //print all the nodes in depth map
        for (k, v) in depth_map.iter() {
            println!("Node {:} -> {:}", self.idx(k).unwrap().index(), v);
        }
        self.depth_map = Some(depth_map);
    }

    ///Finds the shortest path between two people. Implementes A*
    pub fn find_shortest_path_astar(&self, p1: &Person, p2: &Person) -> Vec<usize> {
        let p1x = self.idx(p1).unwrap();
        let goalx = self.idx(p2).unwrap();
        let map = match &self.depth_map {
            Some(m) => m,
            None => panic!("Depth map not built"),
        };
        let est_cost = |n: NodeIndex<usize>| {
            let l1 = map.get(&self.graph[n]).unwrap();
            let l2 = map.get(&self.graph[goalx]).unwrap();
            //returns square of euclidean distance
            l1.dot(*l2).abs() as u32
        };
        let path = astar(
            &self.graph,
            p1x,
            |f| goalx == f,
            |e| e.weight().get_cost(),
            est_cost,
        )
        .unwrap()
        .1
        .iter()
        .map(|n| (*n).index())
        .collect();
        path
    }

    ///Finds all paths between two people, returns a vec of vec of tuples of (index, [Kind])
    /// This should allow us to include relationships arising from various levels of incest/intermarriage (e.g. there are cycles in the graph)
    pub fn find_all_paths(&self, p1: &Person, p2: &Person) -> Result<Vec<Vec<(usize, Kind)>>> {
        let p1x = self.idx(p1).unwrap();
        let goalx = self.idx(p2).unwrap();
        let map = match &self.depth_map {
            Some(m) => m,
            None => panic!("Depth map not built"),
        };
        //use dfs to find all paths
        let mut paths = Vec::<Vec<(usize, Kind)>>::new();

        let ways: Vec<Vec<_>> = all_simple_paths(&self.graph, p1x, goalx, 0, None).collect();

        todo!()
    }

    ///Currently uses breadth first search to find the oldest ancestor.
    /// We could actually encode this in a grid, where parent-child go up and own, and sibling, and retpat go side to
    /// side. This one allow us to use euclidean distance as an heuristic to an A* algorithm.
    /// But given the average expected size of our graph (probably only ever in the tens maybe a hundred), this is unlikely to actually matter.
    /// Returns a tuple representing the depth of the eldest and then the id of the person itself.
    /// The eldest person can be any person in the eldest generation
    /// This function is probably deprecated
    pub fn find_eldest_ancestor(&self, p: &Person) -> (i32, usize) {
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

            //This is known to be 1 (an invariant) (only 1 outgoing edge between two nodes), if broken, our graph is illformed
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

        //get eldest person assoicated with the depth
        let eldest_person = depth_set
            .iter()
            .find(|(_, d)| *d == &eldest_depth)
            .unwrap()
            .0;

        (eldest_depth, *eldest_person)
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
        let p7 = Person::new(Sex::Male);
        let p8 = Person::new(Sex::Female);
        let p9 = Person::new(Sex::Female);
        //add them as nodes
        kg.add_person(&p0);
        kg.add_person(&p1);
        kg.add_person(&p2);
        kg.add_person(&p3);
        kg.add_person(&p4);
        kg.add_person(&p5);
        kg.add_person(&p6);
        kg.add_person(&p7);
        kg.add_person(&p8);
        kg.add_person(&p9);

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
        //p3 is parent of p7
        kg.add_relation(&p3, &p7, Kind::Parent);
        //p7 is parent of p8
        kg.add_relation(&p7, &p8, Kind::Parent);

        //p5 is parent of p4
        kg.add_relation(&p4, &p3, Kind::Parent);
        //p6 is sibling of p4
        kg.add_relation(&p5, &p3, Kind::Sibling);
        //and p6 is parent of p7
        kg.add_relation(&p5, &p6, Kind::Parent);

        let path = kg.get_canonical_relationship_depr(&p0, &p8).unwrap();
        println!("Node {:?} is the {:} of {:?}", &p8, path, &p0);
    }
}
