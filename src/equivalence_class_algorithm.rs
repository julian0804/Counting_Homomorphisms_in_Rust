/// This module contains the first approach to speed up the
/// algorithm of diaz et all.
pub mod algorithm {
    use std::arch::x86_64::_mm256_div_ps;
    use std::collections::HashMap;
    use itertools::Itertools;
    use petgraph::matrix_graph::{MatrixGraph, NodeIndex};
    use petgraph::Undirected;
    use petgraph::visit::NodeIndexable;
    use crate::diaz::diaz_algorithm::diaz;
    use crate::graph_generation::graph_generation_algorithms::generate_possible_edges;
    use crate::integer_functions::integer_functions_methods;
    use crate::integer_functions::integer_functions_methods::Mapping;
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
        all_possible_edges : Vec<(usize,usize)>,
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

            DPData { table: HashMap::new(),
                nice_tree_decomposition,
                to_graph,
                sorted_bags,
                possible_edges,
                index_to_edge,
                edge_to_index,
                all_possible_edges : all_possible_edges.clone() }
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
            integer_functions_methods::apply(self.to_graph.node_count() as Mapping, f, s)
        }

        /// Extend function where the dimension is already set to |V(G)|.
        pub fn table_extend(&self, f : Mapping, s : Mapping, v : Mapping) -> Mapping{
            integer_functions_methods::extend(self.to_graph.node_count() as Mapping, f, s, v)
        }

        /// Reduce function where the dimension is already set to |V(G)|.
        pub fn table_reduce(&self, f : Mapping, s : Mapping) -> Mapping{
            integer_functions_methods::reduce(self.to_graph.node_count() as Mapping, f, s)
        }

        /// This is basically the max mapping function applied to the bag(p) and |V(G)|.
        /// It returns the number of mappings from bag(p) to |V(G)|
        pub fn max_bag_mappings(&self, node : TreeNode) -> Mapping{
            integer_functions_methods::max_mappings(self.nice_tree_decomposition.bag(node).unwrap().len() as Mapping,
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

        /// Given the index of an edge this functions returns the edge as a tuple
        pub fn index_to_edge(&self, index : &usize) -> Option<&(usize, usize)> { self.index_to_edge.get(index) }

        /// Given a specific edge as a tuple, return the index of this edge.
        pub fn edge_to_index(&self, edge : &(usize,usize)) -> Option<&usize> { self.edge_to_index.get(edge) }

        /// Returns the vector of all possible edges.
        pub fn all_possible_edges(&self) -> &Vec<(usize, usize)> { &self.all_possible_edges }

        /// Returns a vector of the indices of all possible edges until node p
        pub fn possible_edges(&self, p : TreeNode) -> Option<&Vec<usize>> { self.possible_edges.get(&p) }

        /// A function removing all entries for a given Node.
        pub fn remove(&mut self, p : TreeNode){
            self.table.remove(&p);
        }

        /// A function transforming possible edge indices to the corresponding integer representation
        /// todo: make ugly casting more beautiful
        pub fn edges_to_integer_representation(&self, edges : &Vec<usize>) -> EdgeList{
            let mut sum : u64 = 0;
            for &e in edges{
                sum += 2_u64.pow(e as u32);
            }
            sum
        }

        /// Given to edge sets in integer representation regarding the order of
        /// possible edges of the nice tree decomposition, this function calculates
        /// the intersection of both edge sets by using the bitwise AND.
        pub fn intersection(&self, edge_set_1 : EdgeList, edge_set_2 : EdgeList) -> EdgeList { edge_set_1 & edge_set_2 }

        // Given an edge set in integer representation, this functions returns a graph with the given edges.
        pub fn edges_to_graph(&self, edges : EdgeList) -> MatrixGraph<(), (), Undirected>{

            let mut graph : MatrixGraph<(), (), Undirected> = petgraph::matrix_graph::MatrixGraph::new_undirected();
            let number_of_vertices = self.nice_tree_decomposition.vertex_count();

            for _ in 0..number_of_vertices{
                graph.add_node(());
            }

            // todo: create generate_graph function which creates a single graph and reduce amount of code

            let mut edge_list = vec![];
            // extract possible edges by looping over all possibles indices
            for i in 0..self.all_possible_edges.len() as u32
            {
                let filter = 2_u64.pow(i);
                if self.intersection(filter, edges) == filter{
                    edge_list.push(self.index_to_edge(&(i as usize)).unwrap());
                }
            }

            for (u,v) in edge_list{
                graph.add_edge(NodeIndex::new(*u),NodeIndex::new(*v), ());
            }

            graph

        }
    }


    // Additional functions:
    // - Edge to index
    // - Index to edge
    // - possible edges: mapping TreeNode -> Vec<Indices>

    /// implementation of the equivalence class algorithm
    pub fn equivalence_class_algorithm(ntd : &NiceTreeDecomposition, to_graph : &MatrixGraph<(),(), Undirected>) -> Vec<(MatrixGraph<(), (), Undirected>, u64)> {

        let stingy_ordering = ntd.stingy_ordering();
        let mut dpdata = DPData::new(ntd,to_graph);

        for p in stingy_ordering{

            match ntd.node_type(p){
                Some(NodeType::Leaf) =>  {
                    let unique_vertex = (*ntd.unique_vertex(p).unwrap()).index();

                    // Set entries for the graph with one vertex without a self loop
                    // Iterate over all possible images of unique_vertex in to_graph
                    for image in 0..to_graph.node_count(){

                        // sets the entry I[p,0,image] = 1 which is the number of extending
                        // homomorphisms of the mapping (v,a) from the graph with only one vertex without a self loop
                        // to the graph to_graph.
                        dpdata.set(p,0, image as Mapping, 1);

                    }

                    // find the vertex of the edge (unique_vertex, unique_vertex)
                    let unique_vertex_loop_index = *dpdata.edge_to_index( &( unique_vertex, unique_vertex) ).unwrap();

                    // Construct the edge set which only contains the edge (unique_vertex, unique_vertex)
                    let edge_set = 2_u32.pow(unique_vertex_loop_index as u32) as u64;

                    // Set entries for the graph with one vertex with a self loop
                    // Iterate over all possible images of unique_vertex in to_graph
                    for image in 0..to_graph.node_count(){

                        // Check if the image vertex has a self loop
                        if to_graph.has_edge(to_graph.from_index(image), to_graph.from_index(image)){
                            dpdata.set(p,edge_set, image as Mapping, 1);
                        }else {
                            dpdata.set(p,edge_set, image as Mapping, 0);
                        }
                    }

                }
                Some(NodeType::Introduce) => {


                    // get the unique child of p
                    let q = *ntd.unique_child(p).unwrap();
                    // get the introduced vertex
                    let v = *ntd.unique_vertex(p).unwrap();

                    // get the indices of all possible edges in the subtree rooted at p
                    let possible_edges_until_p = dpdata.possible_edges(p).unwrap();

                    // sorted bag of q
                    let sorted_q_bag = dpdata.sorted_bag(q).unwrap();

                    // sorted bag of p
                    let sorted_p_bag = dpdata.sorted_bag(p).unwrap();

                    // That is the case when no index will be found
                    // the mapping will be but to the end of the new mapping
                    let mut new_index = sorted_q_bag.len();

                    // Find the position of the introduce vertex in the new mapping
                    if let Some(index) = sorted_q_bag.iter().position(|&vertex| v.index() < vertex.index() ){ new_index = index; }

                    // maps vertex to its significance in the bag of p
                    let mut significance_hash = HashMap::new();
                    for (i, item) in sorted_p_bag.iter().enumerate(){
                        significance_hash.insert(*item, i);
                    }

                    // get the integer representation of all possible edges until q
                    let possible_edges_of_q_integer = dpdata.possible_edges(q).unwrap();
                    let possible_edges_of_q_integer = dpdata.edges_to_integer_representation(possible_edges_of_q_integer);

                    // loop over all subsets of possible_edges_until_p


                    for edges in possible_edges_until_p.clone().iter().powerset().collect::<Vec<_>>(){

                        let mut s_q = vec![];

                        let v_index = v.index();
                        // generate the set s_q, which corresponds to the neighbors of v in edges
                        for edge_index in &edges {
                            let (x,u) = dpdata.index_to_edge(*edge_index).unwrap();

                            if *x == v_index {
                                if !s_q.contains(u) {
                                    s_q.push(*u);
                                }
                            }

                            if *u == v_index{
                                if !s_q.contains(x){
                                    s_q.push(*x);
                                }
                            }
                        }

                        let edges_without_ref = edges.iter().map(|x| { **x } ).collect();

                        let edges_integer = dpdata.edges_to_integer_representation(&edges_without_ref);

                        // iterate over all new mappings by inserting (introduced_vertex,a)
                        for f_q in 0..dpdata.max_bag_mappings(q){
                            for a in 0..to_graph.node_count(){
                                // extend mapping by a at the new index
                                let f_prime = dpdata.table_extend(f_q, new_index as Mapping, a as Mapping);

                                let condition = {
                                    let mut value = true;

                                    for u in &s_q{
                                        let image_of_unique_vertex = to_graph.from_index(a);

                                        // get the significance of vertex u in mapping f_prime
                                        let significance = *significance_hash.get(&Vertex::new(*u) ).unwrap();

                                        let image_of_u = to_graph.from_index(dpdata.table_apply(f_prime, significance as Mapping) as usize);

                                        if !to_graph.has_edge(image_of_unique_vertex, image_of_u){
                                            value = false;
                                            break;
                                        }
                                    }

                                    value
                                };

                                let old_edges_list = dpdata.intersection(edges_integer, possible_edges_of_q_integer);
                                dpdata.set(p, edges_integer ,f_prime,
                                           *dpdata.get(&q, &old_edges_list,&f_q).unwrap() * (condition as u64 ));

                            }
                        }


                    }
                    dpdata.remove(q);

                }
                Some(NodeType::Forget) => {

                    // get the unique child of p
                    let q = *ntd.unique_child(p).unwrap();
                    // get the introduced vertex
                    let forgotten_vertex = *ntd.unique_vertex(p).unwrap();

                    // transforms the bag into a sorted vertex used for integer functions
                    let sorted_bag_q = dpdata.sorted_bag(q).unwrap();

                    // find significance of forgotten vertex in the mappings of F_q
                    let significance_forgotten_vertex = sorted_bag_q.iter().position(|x| *x == forgotten_vertex).unwrap();

                    // get the indices of all possible edges in the subtree rooted at p
                    let possible_edges_until_p = dpdata.possible_edges(p).unwrap();

                    // iterate over all possible edge lists
                    for edges in possible_edges_until_p.clone().iter().powerset().collect::<Vec<_>>() {

                        let edges_without_ref = edges.iter().map(|x| { **x } ).collect();

                        // integer representation of edge list
                        let edges_integer = dpdata.edges_to_integer_representation(&edges_without_ref);

                        // loop over all possible mappings from bag(p) to to_graph
                        for f_prime in 0..dpdata.max_bag_mappings(p) {

                            let mut sum = 0;

                            // sum up over all possible images of the forgotten vertex
                            for a in 0..to_graph.node_count(){
                                let f_old = dpdata.table_extend(f_prime, significance_forgotten_vertex as Mapping, a as Mapping);
                                sum += dpdata.get(&q, &edges_integer,&f_old).unwrap();
                            }

                            dpdata.set(p, edges_integer, f_prime, sum);

                        }

                    }

                    dpdata.remove(q)

                }
                Some(NodeType::Join) => {

                    if let Some(children) = ntd.children(p){
                        let q1 = children.get(0).unwrap();
                        let q2 = children.get(1).unwrap();

                        // get the integer representation of all possible edges until q
                        let possible_edges_of_q1_integer = dpdata.possible_edges(*q1).unwrap();
                        let possible_edges_of_q1_integer = dpdata.edges_to_integer_representation(possible_edges_of_q1_integer);

                        let possible_edges_of_q2_integer = dpdata.possible_edges(*q2).unwrap();
                        let possible_edges_of_q2_integer = dpdata.edges_to_integer_representation(possible_edges_of_q2_integer);

                        // get the indices of all possible edges in the subtree rooted at p
                        let possible_edges_until_p = dpdata.possible_edges(p).unwrap();

                        // iterate over all possible edge lists
                        for edges in possible_edges_until_p.clone().iter().powerset().collect::<Vec<_>>() {

                            let edges_without_ref = edges.iter().map(|x| { **x } ).collect();

                            // integer representation of edge list
                            let edges_integer = dpdata.edges_to_integer_representation(&edges_without_ref);

                            // Updates every new mapping
                            for f in 0..dpdata.max_bag_mappings(p){

                                let intersection1 = dpdata.intersection(edges_integer, possible_edges_of_q1_integer);
                                let intersection2 = dpdata.intersection(edges_integer, possible_edges_of_q2_integer);

                                dpdata.set(p, edges_integer, f,
                                dpdata.get(q1, &intersection1, &(f as Mapping)).unwrap() *
                                    dpdata.get(q2, &intersection2, &(f as Mapping)).unwrap() );
                            }

                        }

                        // Deletes entries og q1 and q2
                        dpdata.remove(*q1);
                        dpdata.remove(*q2);
                    }

                }
                None => {}
            }

        }

        // final return of all hom numbers
        let mut graph_hom_number_list = vec![];

        let final_list = dpdata.table.get(&ntd.root()).unwrap();
        for ((graph_number, i),hom_number) in final_list{

            if *i == 0 {
                graph_hom_number_list.push((dpdata.edges_to_graph(*graph_number), *hom_number) );
            }
        }
        graph_hom_number_list
    }

}