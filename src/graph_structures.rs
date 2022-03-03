
pub mod graph_structures {

    use std::collections::HashSet;
    use std::collections::HashMap;

    pub type Vertex = u32;
    pub type VertexBag = HashSet<Vertex>;

    pub mod adjacency{
        use super::*;
        /*
        Implementation of a simple adjacency list for a directed graph with loop but without multi-edges
        Vertices: 1 ... N
        */
        #[derive(Debug,PartialEq)]
        pub struct AdjList{
            pub list : HashMap<Vertex, Vec<Vertex>>,
        }

        impl AdjList{

            /*
           Returns an empty AdjList
            */
            pub fn new() -> AdjList{
                AdjList{list : HashMap::new() }
            }

            /*
            Returns the number of edges in the adjacency list
             */
            pub fn number_of_edges(&self) -> usize
            {
                self.list.iter().map(|(i,v)| v.len()).sum()
            }

            /*
            returning the number of vertices by searching the maximum key
             */
            pub fn number_of_vertices(&self) -> Vertex
            {
                *self.list.keys().max().unwrap_or(&0)
            }

            /*
            Checks if the edge (from, to) exists in this adjacency list
             */
            pub fn check_edge(&self, from : Vertex, to : Vertex) -> bool {
                if self.list.get(&from) == None{
                    return false;
                }
                self.list.get(&from).unwrap().contains(&to)
            }
            /*
            inserts edge if not already available
             */
            pub fn insert_edge(&mut self, from : Vertex,to : Vertex){
                // Checks if edge already exists
                if !self.check_edge(from,to) {

                    // Checks if we already have an edge going out of "from"
                    if self.list.get(&from) != None{
                        if let mut nlist = self.list.get_mut(&from).unwrap(){
                            nlist.push(to);
                        }
                    }
                    else{
                        self.list.insert(from,vec![to]);
                    }
                }
            }

            /*
            Returns an &Vec<Vertex> of the out neighbours if the vertex "from" has some, None otherwise
             */
            pub fn out_neighbours(&self, from : Vertex) -> Option<&Vec<Vertex>>
            {
                if let Some(i) = self.list.get(&from){
                    Some(i)
                }
                else {
                    None
                }
            }

            /*
            Returns the out degree of the vertex "from"
             */
            pub fn out_degree(&self, from : Vertex) -> usize
            {
                if let Some(i) = self.out_neighbours(from){ i.len() }
                else { 0 }
            }
        }
    }

    pub mod nice_tree_decomposition{
        use crate::graph_structures::graph_structures::adjacency::AdjList;
        use super::*;

        enum NodeType{
            Leaf(VertexBag),
            Introduce(VertexBag),
            Forget(VertexBag),
            Join(VertexBag)
        }

        struct NiceTreeDecomposition{
            adjacency_list : AdjList,
            node_data : HashMap<usize, NodeType>,
            root : usize
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::graph_structures::graph_structures::adjacency::AdjList;
    #[test]
    fn adjacency_list(){
        let mut adjlist = AdjList::new();
        assert_eq!(adjlist.number_of_edges(),0);

        adjlist.insert_edge(1,2);
        assert_eq!(adjlist.number_of_edges(),1);

        adjlist.insert_edge(1,2);
        assert_eq!(adjlist.number_of_edges(),1);

        adjlist.insert_edge(3,2);
        adjlist.insert_edge(1,3);
        assert_eq!(adjlist.number_of_edges(), 3);

        assert_eq!(adjlist.check_edge(1,3), true);
        assert_eq!(adjlist.check_edge(3,1), false);

        assert_eq!(adjlist.number_of_vertices(), 3);
        assert_eq!(adjlist.out_neighbours(2), None);
        assert_eq!(adjlist.out_degree(2), 0);

        assert_eq!(*(adjlist.out_neighbours(1).unwrap()), vec![2,3]);
        assert_ne!(*(adjlist.out_neighbours(1).unwrap()), vec![4]);
        assert_ne!(*(adjlist.out_neighbours(1).unwrap()), vec![3,2]);

        let mut adjlist1 = AdjList::new();
        assert_eq!(adjlist1.number_of_vertices(),0);
    }
}