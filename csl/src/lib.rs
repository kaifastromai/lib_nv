#![feature(nll)]

mod ds {
    mod graphs {
        pub trait Graph {
            type Node;
            type Edge;
        }
        pub struct GraphNode<T> {
            pub data: T,
            pub edges: Option<Vec<usize>>,
        }

        //a simple implementation of a directed graph using an adjacency list
        pub struct DirectedGraph<T> {
            //A vector of nodes, each node has a vector of edges
            pub adj: Vec<GraphNode<T>>,
        }
        impl<T> DirectedGraph<T> {
            pub fn new() -> Self {
                DirectedGraph { adj: Vec::new() }
            }
            pub fn add_node(
                &mut self,
                data: T,
                indegrees: &[usize],
                outdegrees: &[usize],
            ) -> Result<(), String> {
                let l = self.adj.len();
                for val in outdegrees {
                    self.validate_index(*val);
                }
                self.adj.push(GraphNode {
                    data,
                    edges: Some(outdegrees.to_owned()),
                });

                for i in indegrees {
                    if self.validate_index(*i) {
                        let outer_edges = self.adj[*i].edges.get_or_insert(Vec::new());
                        outer_edges.push(l + 1);
                    }
                }

                Ok(())
            }
            pub fn add_edge(&mut self, from: usize, to: usize) {
                match &mut self.adj[from].edges {
                    Some(edges) => edges.push(to),
                    None => {
                        self.adj[from].edges = Some(vec![to]);
                    }
                };
            }
            pub fn get_node(&self, index: usize) -> &GraphNode<T> {
                &self.adj[index]
            }
            pub fn get_node_mut(&mut self, index: usize) -> &mut GraphNode<T> {
                &mut self.adj[index]
            }
            pub fn validate_index(&self, index: usize) -> bool {
                index < self.adj.len()
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
