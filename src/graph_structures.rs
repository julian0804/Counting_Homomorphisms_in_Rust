
pub mod graph_structures {

    use std::collections::HashSet;
    use std::collections::HashMap;

    pub type Vertex = u32;
    pub type VertexBag = HashSet<Vertex>;

    pub mod adjacency{
        use super::*;
        /*
        Implementation of a simple adjacency list for a directed graph with possible loops
        but without multi-edges

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

        #[derive(Debug,PartialEq)]
        pub enum NodeType{
            Leaf(VertexBag),
            Introduce(VertexBag),
            Forget(VertexBag),
            Join(VertexBag)
        }

        #[derive(Debug, PartialEq)]
        pub struct NiceTreeDecomposition{
            pub adjacency_list : AdjList,
            pub node_data : HashMap<Vertex, NodeType>,
            pub root : Vertex
        }

        pub struct StingyOrder{
            order : Vec<Vertex>,
        }

        impl NiceTreeDecomposition {
            /*
            creates a tree decomposition given
            -> adjacency List
            -> node_data
            -> root
             */
            pub fn new(adjacency_list: AdjList, node_data: HashMap<Vertex, NodeType>, root: Vertex) -> NiceTreeDecomposition {
                NiceTreeDecomposition { adjacency_list, node_data, root }
            }

            /*
            gets the node data
             */
            pub fn get_node_data(&self, v: &Vertex) -> Option<&NodeType>{
                self.node_data.get(v)
            }

            /*
            returns the reference to the bag of the given node
            */
            pub fn get_bag(&self, v : &Vertex) -> Option<&VertexBag>{
                match self.get_node_data(v){
                    None => None,
                    Some(NodeType::Leaf(n)) => Some(&n),
                    Some(NodeType::Introduce(n)) => Some(&n),
                    Some(NodeType::Forget(n)) => Some(&n),
                    Some(NodeType::Join(n)) => Some(&n),
                }
            }

            /*
           clones the bag of the given vertex
             */
            pub fn get_bag_clone(&self, v : &Vertex) -> VertexBag
            {
                match self.get_node_data(v){
                    None => VertexBag::new(),
                    Some(NodeType::Leaf(n)) => n.clone(),
                    Some(NodeType::Introduce(n)) => n.clone(),
                    Some(NodeType::Forget(n)) => n.clone(),
                    Some(NodeType::Join(n)) => n.clone(),
                }
            }

            /*
            Returns the union of all bags of the subtree rooted at the given node
             */
            pub fn get_union(&self, v : &Vertex) -> VertexBag
            {
                let children = self.adjacency_list.out_neighbours(*v);
                let mut union: VertexBag = self.get_bag_clone(v);

                match children {
                    Some(vec) => {
                        for i in vec{
                            union.extend(&self.get_union(i));
                        }
                    },
                    None => (),
                }
                union
            }

        }



    }

}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use crate::graph_structures::graph_structures::adjacency::AdjList;
    use crate::graph_structures::graph_structures::nice_tree_decomposition::{NiceTreeDecomposition, NodeType};
    use crate::graph_structures::graph_structures::nice_tree_decomposition::NodeType::Leaf;
    use crate::graph_structures::graph_structures::{Vertex, VertexBag};
    use crate::nice_tree_decomposition::Node;

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

    #[test]
    fn tree_decomposition()
    {
        let mut node_data: HashMap<Vertex, NodeType> = HashMap::new();
        node_data.insert(1, NodeType::Leaf(VertexBag::from([1])));
        node_data.insert(2, NodeType::Introduce(VertexBag::from([1,2])));
        node_data.insert(3, NodeType::Forget(VertexBag::from([2])));
        node_data.insert(4, NodeType::Leaf(VertexBag::from([2])));
        node_data.insert(5, NodeType::Introduce(VertexBag::from([2,3])));
        node_data.insert(6, NodeType::Forget(VertexBag::from([2])));
        node_data.insert(7, NodeType::Join(VertexBag::from([2])));
        node_data.insert(8, NodeType::Introduce(VertexBag::from([2,4])));
        node_data.insert(9, NodeType::Forget(VertexBag::from([4])));
        node_data.insert(10, NodeType::Forget(VertexBag::from([])));

        let mut adj_list = AdjList::new();
        adj_list.insert_edge(2,1);
        adj_list.insert_edge(3,2);
        adj_list.insert_edge(7,3);
        adj_list.insert_edge(5,4);
        adj_list.insert_edge(6,5);
        adj_list.insert_edge(7,6);
        adj_list.insert_edge(8,7);
        adj_list.insert_edge(9,8);
        adj_list.insert_edge(10,9);

        let treedecomp = NiceTreeDecomposition::new(adj_list, node_data, 10);

        assert_eq!(treedecomp.get_bag(&7), Some(&VertexBag::from([2])));
        assert_eq!(treedecomp.get_bag(&2), Some(&VertexBag::from([2,1])));
        assert_eq!(treedecomp.get_bag(&2), Some(&VertexBag::from([1,2])));
        assert_eq!(treedecomp.get_bag(&10), Some(&VertexBag::from([])));

        assert_eq!(treedecomp.get_node_data(&2), Some(&NodeType::Introduce(VertexBag::from([1,2]))));
        assert_eq!(treedecomp.get_union(&2), VertexBag::from([1,2]));


        assert_eq!(treedecomp.adjacency_list.out_neighbours(7), Some(&Vec::from([3,6])) );

        assert_eq!(treedecomp.get_union(&7), VertexBag::from([1,2,3]));
        assert_eq!(treedecomp.get_union(&10), VertexBag::from([1,2,3,4]));
    }
}