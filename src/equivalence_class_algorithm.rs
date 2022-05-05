/// This module contains the first approach to speed up the
/// algorithm of diaz et all.
pub mod equivalence_class_algorithm{
    use std::collections::HashMap;
    use petgraph::matrix_graph::MatrixGraph;
    use petgraph::Undirected;
    use crate::graph_generation::graph_generation::generate_possible_edges;
    use crate::integer_functions::integer_functions;
    use crate::integer_functions::integer_functions::Mapping;
    use crate::tree_decompositions::nice_tree_decomposition::{NiceTreeDecomposition, NodeType};
    use crate::tree_decompositions::tree_structure::{TreeNode, Vertex};

    /// A pseudonym for u64 since EdgeList will represented as u64
    /// note: maximum number of possible Edges is therefore 64
    pub type EdgeList = u64;

    // 1. Implement table
    // 2. Implement algorithm

    /// A struct containing all important information for the dynamic program.
    pub struct DPData<'a>{
        table : HashMap<TreeNode, HashMap<(EdgeList, Mapping), u64>>, // table[p,e,phi], p = tree node, e = subset of edges represented by an integer, phi = mapping
        nice_tree_decomposition: &'a NiceTreeDecomposition,
        to_graph: &'a MatrixGraph<(), (), Undirected>,
        sorted_bags : HashMap<TreeNode, Vec<Vertex>>,
        possible_edges : HashMap<TreeNode, Vec<usize>>, // list of possible indices of edges until the given tree node
        index_to_edge : HashMap<usize, (usize,usize)>, // maps the edge_index to the actual edge
        edge_to_index : HashMap<(usize,usize), usize>, // maps the edge to its index
    }

    /// Implementation of functions being necessary for writing and reading the table
    /// of the dynamic program.
    impl<'a> DPData<'a> {
        /// A simple constructor for creating an empty table
        pub fn new<'b>(nice_tree_decomposition: &'b NiceTreeDecomposition,
                        to_graph: &'b MatrixGraph<(), (), Undirected>,
                        ) -> DPData<'b> {

            let sorted_bags = DPData::sort_bags(nice_tree_decomposition);

            let generated_possible_edges = generate_possible_edges(nice_tree_decomposition);
            let all_possible_edges = generated_possible_edges.get(&nice_tree_decomposition.root()).unwrap();

            // Hashmaps for faster accessing later on
            let mut index_to_edge = HashMap::new();
            let mut edge_to_index = HashMap::new();

            // build index_to_edge and edge_to_index
            for (i, (u,v))  in all_possible_edges.iter().enumerate(){
                index_to_edge.insert(i, (*u,*v));
                //map both direction onto the same index
                edge_to_index.insert((*u,*v), i);
                edge_to_index.insert((*v,*u), i);
            }

            let mut possible_edges = HashMap::new();

            for (u,v) in generated_possible_edges.iter(){
                let edges : Vec<usize> = v.iter().map(|x| { *edge_to_index.get(x).unwrap() }).collect();
                possible_edges.insert(*u, edges);
            }

            DPData { table: HashMap::new(), nice_tree_decomposition, to_graph, sorted_bags, possible_edges, index_to_edge, edge_to_index }
        }

        /// Returns the entry I[p,e,f] where p is a tree node, e a subset of possible edges and f is a mapping.
        pub fn get(&self, p: &TreeNode, e : &EdgeList ,f: &Mapping) -> Option<&u64> {

            if let Some(mappings) = self.table.get(p) { mappings.get(&(*e,*f)) } else { None }
        }

        /// Sets the entry I[p,e,f] of the dynamic table to the value of v.
        pub fn set(&mut self, p: TreeNode, e : EdgeList, f: Mapping, v: u64) {
            if let Some(mappings) = self.table.get_mut(&p) {
                mappings.insert((e, f), v);
            } else {
                self.table.insert(p, HashMap::from([((e, f), v)] ) );
            }
        }

        /// Apply function where the dimension is already set to |V(G)|.
        pub fn table_apply(&self, f : Mapping, s : Mapping) -> Mapping{
            integer_functions::apply(self.to_graph.node_count() as Mapping, f, s)
        }

        /// Extend function where the dimension is already set to |V(G)|.
        pub fn table_extend(&self, f : Mapping, s : Mapping, v : Mapping) -> Mapping{
            integer_functions::extend(self.to_graph.node_count() as Mapping, f, s, v)
        }

        /// Reduce function where the dimension is already set to |V(G)|.
        pub fn table_reduce(&self, f : Mapping, s : Mapping) -> Mapping{
            integer_functions::reduce(self.to_graph.node_count() as Mapping, f, s)
        }

        /// This is basically the max mapping function applied to the bag(p) and |V(G)|.
        /// It returns the number of mappings from bag(p) to |V(G)|
        pub fn max_bag_mappings(&self, node : TreeNode) -> Mapping{
            integer_functions::max_mappings(self.nice_tree_decomposition.bag(node).unwrap().len() as Mapping,
                                            self.to_graph.node_count() as Mapping )
        }

        /// Create a hashmap which maps each node p to a sorted vector of Vertices representing the bag of p.
        fn sort_bags(nice_tree_decomposition : &NiceTreeDecomposition) -> HashMap<TreeNode, Vec<Vertex>>{
            let mut sorted_bags = HashMap::new();

            for p in nice_tree_decomposition.stingy_ordering(){
                let mut vertex_vector = Vec::from_iter(nice_tree_decomposition.bag(p).unwrap().iter());
                vertex_vector.sort();
                sorted_bags.insert(p, vertex_vector.iter().map(|e| **e).collect());
            }

            sorted_bags
        }

        /// Given a node p, this function returns the sorted bag of p as a vector of Vertices.
        pub fn sorted_bag(&self, p : TreeNode) -> Option<&Vec<Vertex>>{ self.sorted_bags.get(&p) }

        /// A function removing all entries for a given Node.
        pub fn remove(&mut self, p : TreeNode){
            self.table.remove(&p);
        }
    }


    // Additional functions:
    // - Edge to index
    // - Index to edge
    // - possible edges: mapping TreeNode -> Vec<Indices>


    pub fn equivalence_class_algorithm(ntd : & NiceTreeDecomposition){

        let stingy_ordering = ntd.stingy_ordering();

        for p in stingy_ordering{

            match ntd.node_type(p){
                Some(NodeType::Leaf) => {

                }
                Some(NodeType::Introduce) => {

                }
                Some(NodeType::Forget) => {

                }
                Some(NodeType::Join) => {

                }
                None => {}
            }

        }
    }

}