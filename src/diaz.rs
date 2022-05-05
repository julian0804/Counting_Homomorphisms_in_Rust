
/// A module containing the algorithm of diaz [todo: add reference with all names]
pub mod diaz_algorithm {
    use std::collections::{HashMap, HashSet};
    use itertools::sorted;
    use petgraph::matrix_graph::MatrixGraph;
    use petgraph::Undirected;
    use petgraph::visit::NodeIndexable;
    use crate::integer_functions::integer_functions;
    use crate::integer_functions::integer_functions::Mapping;
    use crate::tree_decompositions::nice_tree_decomposition::{NiceTreeDecomposition, NodeType};
    use crate::tree_decompositions::tree_structure::{TreeNode, Vertex};

    /// A struct containing all important information for the dynamic program.
    pub(crate) struct DPData<'a> {
        table: HashMap<TreeNode, HashMap<Mapping, u64>>,
        nice_tree_decomposition: &'a NiceTreeDecomposition,
        from_graph: &'a MatrixGraph<(), (), Undirected>,
        to_graph: &'a MatrixGraph<(), (), Undirected>,
        sorted_bags : HashMap<TreeNode, Vec<Vertex>>,
    }

    /// Implementation of functions being necessary for writing and reading the table
    /// of the dynamic program.
    impl<'a> DPData<'a> {
        /// A simple constructor for creating an empty table
        pub fn new<'b>(from_graph: &'b MatrixGraph<(), (), Undirected>,
                       to_graph: &'b MatrixGraph<(), (), Undirected>,
                       nice_tree_decomposition: &'b NiceTreeDecomposition, ) -> DPData<'b> {
            let sorted_bags = DPData::sort_bags(nice_tree_decomposition);
            DPData { table: HashMap::new(), nice_tree_decomposition, from_graph, to_graph, sorted_bags }
        }

        /// Returns the entry I[p,f] where p is a tree node and f is a mapping.
        pub fn get(&self, p: &TreeNode, f: &Mapping) -> Option<&u64> {
            if let Some(mappings) = self.table.get(p) { mappings.get(f) } else { None }
        }

        /// Sets the entry I[p,f] of the dynamic table to the value of v.
        pub fn set(&mut self, p: TreeNode, f: Mapping, v: u64) {
            if let Some(mappings) = self.table.get_mut(&p) {
                mappings.insert(f, v);
            } else {
                self.table.insert(p, HashMap::from([(f, v)]));
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

    /// Implementation of the algorithm of diaz et all
    pub fn diaz(from_graph : &MatrixGraph<(),(), Undirected>, ntd : &NiceTreeDecomposition, to_graph : &MatrixGraph<(),(), Undirected>) -> u64{

        let stingy_ordering = ntd.stingy_ordering();
        let mut dp_data = DPData::new(from_graph, to_graph, ntd);

        // traversing the tree of the nice tree decomposition by following the stingy ordering.
        for p in stingy_ordering{

            // matching node types
            match ntd.node_type(p) {
                None => {}
                Some(NodeType::Leaf) => {
                    // get the unique vertex of p´s bag
                    if let Some(&unique_vertex) = ntd.unique_vertex(p){
                        // Checks if unique vertex has a self loop
                        if from_graph.has_edge(unique_vertex,unique_vertex){
                            // iterate over all possible images of unique_vertex
                            for image in 0..to_graph.node_count(){
                                // checks if image of unique_vertex also has self loop
                                if to_graph.has_edge(to_graph.from_index(image),
                                                     to_graph.from_index(image) ){ dp_data.set(p, image as Mapping, 1); }
                                else { dp_data.set(p, image as Mapping, 0); }
                            }
                        }
                        else {
                            // set all mappings to 1
                            for image in 0..to_graph.node_count(){ dp_data.set(p, image as Mapping, 1); }
                        }
                    }
                }
                Some(NodeType::Introduce) => {
                    // get the unique child of p
                    let q = *ntd.unique_child(p).unwrap();
                    // get the introduced vertex
                    let v = *ntd.unique_vertex(p).unwrap();


                    let mut neighbours_of_v: HashSet<Vertex> = HashSet::from_iter(from_graph.neighbors(v));
                    let mut s_q : Vec<&Vertex> = neighbours_of_v.intersection(ntd.bag(p).unwrap()).collect();


                    // sorted bag of q
                    let sorted_q_bag = dp_data.sorted_bag(q).unwrap();

                    // That is the case when no index will be found
                    // the mapping will be but to the end of the new mapping
                    let mut new_index = sorted_q_bag.len();

                    // Find the position of the introduce vertex in the new mapping
                    if let Some(index) = sorted_q_bag.iter().position(|&vertex| v.index() < vertex.index() ){ new_index = index; }


                    let sorted_p_bag = dp_data.sorted_bag(p).unwrap();
                    // maps vertex to its significance in the bag of p
                    let mut significance_hash = HashMap::new();
                    for i in 0..sorted_p_bag.len() {
                        significance_hash.insert(sorted_p_bag[i], i);
                    }

                    // iterate over all new mappings by inserting (introduced_vertex,a)
                    for f_q in 0..dp_data.max_bag_mappings(q){
                        for a in 0..to_graph.node_count(){

                            // extend mapping by a at the new index
                            let f_prime = dp_data.table_extend(f_q, new_index as Mapping, a as Mapping);

                            let condition = {
                                let mut value = true;

                                for u in &s_q{
                                    let image_of_unique_vertex = to_graph.from_index(a);

                                    // get the significance of vertex u in mapping f_prime
                                    let significance = *significance_hash.get(u).unwrap();

                                    let image_of_u = to_graph.from_index(dp_data.table_apply(f_prime, significance as Mapping) as usize);

                                    if !to_graph.has_edge(image_of_unique_vertex, image_of_u){
                                        value = false;
                                        break;
                                    }
                                }

                                value
                            };

                            dp_data.set(p, f_prime,dp_data.get(&q, &f_q).unwrap().clone() * (condition as u64 ));
                        }
                    }

                    dp_data.remove(q);

                }
                Some(NodeType::Forget) => {
                    // get the unique child of p
                    let q = *ntd.unique_child(p).unwrap();
                    // get the introduced vertex
                    let forgotten_vertex = *ntd.unique_vertex(p).unwrap();

                    // transforms the bag into a sorted vertex used for integer functions
                    let sorted_bag_q = dp_data.sorted_bag(q).unwrap();

                    // find significance of forgotten vertex in the mappings of F_q
                    let significance_forgotten_vertex = sorted_bag_q.iter().position(|x| *x == forgotten_vertex).unwrap();

                    // Iterate over all mappings
                    for f_prime in 0..dp_data.max_bag_mappings(p){

                        // summing up all extending homomorphisms
                        let mut sum = 0;

                        // iterate over all images of the forgotten node
                        for a in 0..to_graph.node_count(){
                            let f_old = dp_data.table_extend(f_prime, significance_forgotten_vertex as Mapping, a as Mapping);
                            sum += dp_data.get(&q, &f_old).unwrap();
                        }

                        dp_data.set(p, f_prime, sum);
                    }

                    dp_data.remove(q);
                }
                Some(NodeType::Join) => {
                    if let Some(children) = ntd.children(p){
                        let q1 = children.get(0).unwrap();
                        let q2 = children.get(1).unwrap();

                        // Updates every new mapping
                        for f in 0..dp_data.max_bag_mappings(p){
                            dp_data.set(p,
                                      f as Mapping,
                                        dp_data.get(q1, &(f as Mapping)).unwrap() *
                                            dp_data.get(q2, &(f as Mapping)).unwrap());
                        }

                        // Deletes entries og q1 and q2
                        dp_data.remove(*q1);
                        dp_data.remove(*q2);
                    }
                }
            }

        }

        dp_data.get(&ntd.root(), &0).unwrap().clone()
    }
}