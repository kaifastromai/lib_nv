mod ds {
    mod graphs {
        pub struct DigraphNode<T> {
            pub data: T,
            pub edges: Option<Vec<usize>>,
        }
        //a simple implementation of a directed graph using an adjacency list
        pub struct DirectedGraph<T> {
            //A vector of nodes, each node has a vector of edges
            pub adj: Vec<DigraphNode<T>>,
        }
        impl<T> DirectedGraph<T> {
            pub fn new() -> Self {
                DirectedGraph { adj: Vec::new() }
            }
            pub fn add_node(&mut self, data: T, v: usize,) -> usize {
                self.adj.push(DigraphNode {
                    data,
                    edges: None,
                });
                self.adj[v].edges.get_or_insert(Vec::new()).push(self.adj.len() - 1);
                self.adj.len() - 1
            }
            pub fn add_edge(&mut self, from: usize, to: usize) {
                match &mut self.adj[from].edges {
                    Some(edges) => edges.push(to),
                    None => {
                        self.adj[from].edges = Some(vec![to]);
                    }
                };
            }
            pub fn get_node(&self, index: usize) -> &DigraphNode<T> {
                &self.adj[index]
            }
            pub fn get_node_mut(&mut self, index: usize) -> &mut DigraphNode<T> {
                &mut self.adj[index]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
