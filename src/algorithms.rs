//! Algorithms for counting graph homomorphisms
//!


/// A module containing operations for working with integer functions as
/// presented in the paper "Counting subgraph patterns in large graphs" by
/// Emil Ruhwald Nielsen, Otto Stadel Clausen and Elisabeth Terp Reeve.
pub mod integer_functions {

    /// Defining the type Mapping to distinguish the operation from normal u64 variables.
    pub type Mapping = u64;

    /// Given the integer function f of basis n. Apply returns the digit with significance s.
    /// This is achieved by by shifting all digits s positions to the right and then take the rest
    /// of the division by n which removes the least significant digit.
    pub fn apply(n : Mapping, f : Mapping, s : Mapping) -> Mapping{
        ( f / (n.pow(s as u32) as u64) ) % n
    }

    /// Given the integer function f of basis n. Extend increases the number of digits by one.
    /// This will be done by shifting all digits with significance higher than s one position
    /// to the left(increase their significance by one). Then the digit with significance s will
    /// be set to v
    pub fn extend(n : Mapping, f : Mapping, s : Mapping, v : Mapping) -> Mapping{
        let r = f % (n.pow(s as u32) as Mapping);
        let l = f - r;
        (n * l) + (n.pow(s as u32) as Mapping) * v + r
    }

    /// Given the integer function f of basis n. Reduce decreases the number of digits by one.
    /// This will be done by deleting the digit with significance s and then shifting all digits
    /// with higher significance one to the right (decrease their significance by one).
    pub fn reduce(n : Mapping, f : Mapping, s : Mapping) -> Mapping{
        let r = f % n.pow(s as u32);
        let l = f - (f % n.pow((s + 1) as u32));
        (l / n) + r
    }

    /// Returns the maximal amount of mappings from a set of d elements to
    /// a set of n elements. This mappings can be represented by the integers
    /// {0,1,...,max_mapping - 1}
    pub fn max_mappings(d : Mapping, n : Mapping) -> Mapping{
       n.pow(d as u32)
    }

    /// Takes an mapping fn to the base n as input and returns the mapping in a readable form
    /// todo: could be done more efficiently
    pub fn debug_mapping(n : Mapping, f : Mapping) -> Vec<(Mapping, Mapping)>{

        // Returns the number of digits the mapping would have in base n
        let max_digit = ((f as f64).log(n as f64)) as Mapping + 1;

        let mut result : Vec<(Mapping, Mapping)> = vec![];

        for s in 0..max_digit{
            let image = apply(n,f,s);
            result.push((s,image));
        }

        result
    }
}

/// A module containing brute force homomorphism counter
pub mod brute_force_homomorphism_counter{

    use petgraph::matrix_graph::MatrixGraph;
    use petgraph::Undirected;
    use crate::algorithms::integer_functions;
    use crate::graph_structures::graph_structures::nice_tree_decomposition::Vertex;

    /// a simple brute force algorithm which iterates over all possible mappings from "from_graph" to "to_graph"
    /// todo: a possible improvement would be to first seperate the graph into its connected components and then execute this algo for each of them
    /// todo: generalize them for more graph types
    pub fn simple_brute_force(from_graph : &MatrixGraph<(),(), Undirected>, to_graph : &MatrixGraph<(),(), Undirected>) -> u64{

        let h = from_graph.node_count();
        let g = to_graph.node_count();

        // Checks if mapping is a homomorphism
        let check_mapping = |f : usize|{

            let mut ret = true;

            for u in 0..h{
                for v in 0..h{
                    if from_graph.has_edge(Vertex::new(u ), Vertex::new(v )){

                        // this is basically the apply functions of the integer functions
                        let map_u = f / (g.pow(u as u32) as u64) as usize % g ;
                        let map_v = f / (g.pow(v as u32) as u64) as usize % g ;

                        if !to_graph.has_edge(Vertex::new(map_u), Vertex::new(map_v))
                        {
                            ret = false;
                        }
                    }
                }
            }
            ret
        };

        let max = g.pow(h as u32);
        let mut counter = 0;

        // for all mapings from H to G
        for f in 0..max{
            if check_mapping(f){counter += 1;}
        }
        counter
    }
}

pub mod first_approach{

}

pub mod diaz {
    use std::borrow::Borrow;
    use std::cmp::max;
    use std::collections::{HashMap, HashSet};
    use std::iter::Map;
    use petgraph::matrix_graph::{MatrixGraph, NodeIndex};
    use petgraph::Undirected;
    use itertools::Itertools;
    use crate::algorithms::integer_functions;
    use crate::graph_structures::graph_structures::nice_tree_decomposition::{NiceTreeDecomposition, NodeType, TreeNode, Vertex};

    pub type Mapping = u64;

    /*
    a struct containing all infos about the dynamic program
     */
    pub struct DPData<'a>{
        table : HashMap<TreeNode, HashMap<Mapping, u64>>,
        nice_tree_decomposition : &'a NiceTreeDecomposition,
        from_graph : &'a MatrixGraph<(),(), Undirected>,
        to_graph : &'a MatrixGraph<(),(), Undirected>,
    }


    impl<'a> DPData<'a>{

        pub fn new<'b>(from_graph : &'b MatrixGraph<(),(), Undirected>,
                       to_graph : &'b MatrixGraph<(),(), Undirected>,
                       nice_tree_decomposition : &'b NiceTreeDecomposition,) -> DPData<'b> {
            DPData { table : HashMap::new(), nice_tree_decomposition, from_graph, to_graph }
        }

        /*
        Gets the entry for a given vertex and mapping
         */
        pub fn get(&self, node : &TreeNode, mapping : &Mapping) -> Option<&u64> {
            if let Some(mappings) = self.table.get(node){ mappings.get(mapping) }
            else { None }
        }

        /*
        sets the entry for a given vertex
         */
        pub fn set(&mut self, node : TreeNode, mapping : Mapping, value : u64){
            if let Some(mappings) = self.table.get_mut(&node){
                mappings.insert(mapping,value);
            }
            else {
                self.table.insert(node, HashMap::from([(mapping, value)]));
            }
        }

        /*
        Returns digit of mapping f with significance s of basis b
         */
        pub fn apply(&self, f : Mapping, s : Mapping) -> Mapping{
            integer_functions::apply(self.to_graph.node_count() as Mapping, f, s)
        }

        /*
        "increases the number of digits in f by one. It should
        shift all digits with significance > s one position to the left and then set the
        digit with significance s equal to v."
         */
        pub fn extend(&self, f : Mapping, s : Mapping, v : Mapping) -> Mapping{
            integer_functions::extend(self.to_graph.node_count() as Mapping, f, s, v)
        }

        /*
        "should remove the digit with significance s and shift
        all digits with higher significance one position to the right."
         */
        pub fn reduce(&self, f : Mapping, s : Mapping) -> Mapping{
            integer_functions::reduce(self.to_graph.node_count() as Mapping, f, s)
        }

        /*
        Returns the maximum mapping from a bag of a given node to the to_graph + 1
        for the iterators
         */
        pub fn max_bag_mappings(&self, node : TreeNode) -> Mapping{
            integer_functions::max_mappings(self.nice_tree_decomposition.bag(node).unwrap().len() as Mapping,
                                            self.to_graph.node_count() as Mapping )
        }

        /*
        bag to sorted bag
         */
        pub fn sorted_bag(&self, node : TreeNode) -> Vec<Vertex>{
            let mut v: Vec<&Vertex> = Vec::from_iter(self.nice_tree_decomposition.bag(node).unwrap().iter());
            v.sort();
            v.iter().map(|e| **e).collect()
        }


        /*
        A function mapping the integer function on to a tuple
         */
        pub fn debug_mapping(&self, f : Mapping, sig : Mapping) -> Vec<(Mapping,Mapping)>{
            let mut vec = Vec::new();
            for i in 0..(sig){
                vec.push((i as Mapping, self.apply(f, i as Mapping)));
            }
            vec
        }


    }



    /*
    Generates the set of all possible edges given a nice tree decomposition
    Returns a list of tuples
     */
    pub fn generate_edges(ntd : NiceTreeDecomposition) -> Vec<(usize,usize)>{
        let number_of_nodes = ntd.tree_structure.vertex_code();
        let stingy_ordering = ntd.stingy_ordering();
        let mut edge_list: Vec<(usize,usize)> = vec![];


        for p in stingy_ordering {
            // kartesian product
            for u in ntd.bag(p).unwrap(){
                for v in ntd.bag(p).unwrap(){

                    // checks if edge has already been added
                    // we are using tuples but since we are looking at undirected graphs if we have to check both ways
                    if !edge_list.iter().any(|&i| i == (u.index() , v.index()) || i == (v.index() , u.index())){
                        edge_list.push((u.index() , v.index()));
                    }

                }
            }
        }
        edge_list
    }

    /*
    Generates graphs based on a list of possible edge list and the number of vertices
    Used for generating H_\tau
     */
    pub fn generate_graphs(number_of_nodes : u64, possible_edges : Vec<(usize,usize)>) -> Vec<petgraph::matrix_graph::MatrixGraph<(),(), Undirected>>{

        let mut graphs : Vec<petgraph::matrix_graph::MatrixGraph<(),(), Undirected>> = vec![];

        for edges in possible_edges.iter().powerset().collect::<Vec<_>>(){

            let mut graph : MatrixGraph<(), (), Undirected> = petgraph::matrix_graph::MatrixGraph::new_undirected();

            for i in 0..number_of_nodes{
                graph.add_node(());
            }

            for (u,v) in edges{
                graph.add_edge(NodeIndex::new(*u),NodeIndex::new(*v), ());
            }

            graphs.push(graph);
        }

        graphs
        
    }



    /*
    Based on the following algorithm of Diaz, Serna, Thilikos
    https://www.sciencedirect.com/science/article/pii/S0304397502000178?via%3Dihub

    1. Use Integerfunctions
     */
    pub fn diaz(from_graph : &MatrixGraph<(),(), Undirected>, ntd : NiceTreeDecomposition, to_graph : &MatrixGraph<(),(), Undirected>) -> u64 {
        let stingy_ordering = ntd.stingy_ordering();

        //println!("{:?}", stingy_ordering);

        let mut table = DPData::new( from_graph, to_graph, &ntd);

        for p in stingy_ordering {
            //println!("################# node {:?}", &p);

            match ntd.node_type(p){
                Some(NodeType::Leaf) => {

                    //println!("Leaf");

                    let unique_vertex = ntd.bag(p).unwrap().iter().next().unwrap();
                    // be carefully, we return the number of vertices

                    // inserts the mapping (unique_vertex -> aim_vertex) for each
                    // aim_vertex in the aim graph

                    // todo: Problem we need to check if the unique_vertex has a self loop or not
                    // todo: done this by simply checking the
                    if from_graph.has_edge(*unique_vertex, *unique_vertex){
                        for aim_vertex in 0..to_graph.node_count(){
                            if to_graph.has_edge(Vertex::new(aim_vertex), Vertex::new(aim_vertex))
                            {
                                table.set(p, aim_vertex as Mapping, 1);
                            }
                            else {
                                table.set(p, aim_vertex as Mapping, 0);
                            }

                        }
                    }
                    else {
                        for aim_vertex in 0..to_graph.node_count(){
                            table.set(p, aim_vertex as Mapping, 1);
                        }
                    }


                },
                Some(NodeType::Introduce) => {
                    //println!("Introduce");
                    // TODO: make unique_child & introduced_vertex also to methods of NiceTreeDecomposition
                    // So tree_structure do not have to be public
                    let q = *ntd.tree_structure.unique_child(p).unwrap();
                    let v = ntd.tree_structure.introduced_vertex(p).unwrap();

                    // For calculating S_q
                    let neighbours : Vec<Vertex> = from_graph.neighbors(v).collect();
                    let mut neighbour_set: HashSet<Vertex> = HashSet::from_iter(neighbours);
                    let mut s_q : Vec<&Vertex> = neighbour_set.intersection(ntd.bag(q).unwrap()).collect(); // possible error case, explanation below

                    /*
                    The abstract algorithm uses {u,v} \in  E(G_p)
                    -> I think only edges to the bag of q are necessary otherwise the separator property of the nice tree decomposition would be harmed
                     */

                    // transforms the bag into a sorted vertex used for integer functions
                    let sorted_p_bag = table.sorted_bag(p);
                    let sorted_q_bag = table.sorted_bag(q);

                    // significance
                    let significance_list_p = {
                        let mut hs = HashMap::new();
                        for i in 0..sorted_p_bag.len(){
                            hs.insert(sorted_p_bag[i], i);
                        }
                        hs.clone()
                    };

                    //returns the significance of a given vertex in the bag
                    let significance_p = |a : &Vertex|{
                        significance_list_p.get(a).unwrap()
                    };


                    // significance for q
                    let significance_list_q = {
                        let mut hs = HashMap::new();
                        for i in 0..sorted_q_bag.len(){
                            hs.insert(sorted_q_bag[i], i);
                        }
                        hs
                    };

                    //returns the significance of a given vertex in the bag
                    let significance_q = |a : &Vertex|{
                        significance_list_q.get(a).unwrap()
                    };

                    for f_q in 0..table.max_bag_mappings(q){
                        for a in 0..to_graph.node_count(){

                            //println!("old mapping {:?}, s_q {:?}", f_q,  s_q.clone() );
                            //println!("plus mapping {:?} to {:?}", v, a);

                            let test_condition = {
                                //println!("testing condition");
                                let mut t = true;
                                for u in s_q.clone(){

                                    let first_vertex = Vertex::new(a);
                                    let second_vertex = Vertex::new(table.apply(f_q,*significance_q(u) as Mapping ) as usize);
                                    //println!("{:?} mapped to {:?}", u, second_vertex);

                                    //println!("checking edge ({:?}, {:?})", first_vertex, second_vertex);

                                    if !to_graph.has_edge( first_vertex, second_vertex){
                                        //println!("graph G does not have that edge");
                                        t = false;
                                        break;
                                    }
                                    else {
                                        //println!("graph G has that edge");
                                    }


                                }

                                //additonal sucht that self loops will be mapped on self loops
                                if from_graph.has_edge(v,v) && !to_graph.has_edge(Vertex::new(a),Vertex::new(a))
                                {
                                    t = false;
                                }


                                t
                            };



                            let f = table.extend(f_q, *significance_p(&v) as Mapping, a as Mapping).clone();

                            if test_condition{
                                let value = table.get(&q, &f_q).unwrap().clone();

                                table.set(p,
                                          f,
                                          value);
                            }
                            else {
                                table.set(p,
                                          f,
                                          0);
                            }

                        }
                    }


                    table.table.remove(&q);


                },
                Some(NodeType::Forget) => {
                    //println!("Forget");
                    let q = ntd.tree_structure.unique_child(p).unwrap();
                    let v = ntd.tree_structure.forgotten_vertex(p).unwrap();

                    // transforms the bag into a sorted vertex used for integer functions
                    let sorted_bag = table.sorted_bag(p);


                    let old_significance = |a : &Vertex|{
                        if let Some(i) = sorted_bag.iter().position(|i| a > i){
                            i + 1 // added here a plus one since i think we has some index shift here
                            // todo: check if that is correct!
                            // todo: check if the extend function may have an index shift
                        }
                        else {
                            sorted_bag.len()
                        }
                    };

                    //println!("old signicants of {:?} is {:?}", v, old_significance(&v));

                    for f in 0..table.max_bag_mappings(p){
                        //println!("summing up new mapping {:?}", f);
                        let mut sum = 0;
                        for a in 0..to_graph.node_count(){

                            let f_old = table.extend(f,old_significance(&v) as Mapping, a as Mapping);
                            let additional_mappings = table.get(&q, &f_old).unwrap();

                            //println!("adding old mapping {:?} with number {:?}", f_old, additional_mappings);
                            sum += additional_mappings;
                        }
                        table.set(p, f, sum);
                    }

                    table.table.remove(q);


                },
                Some(NodeType::Join) => {
                    //println!("Join");
                    if let Some(children) = ntd.tree_structure.children(p){
                        let q1 = children.get(0).unwrap();
                        let q2 = children.get(1).unwrap();

                        // Updates every new mapping
                        for f in 0..table.max_bag_mappings(p){
                            table.set(p,
                                      f as Mapping,
                                      table.get(q1, &(f as Mapping)).unwrap() *
                                          table.get(q2, &(f as Mapping)).unwrap());
                        }

                        // Deletes entries after use...
                        table.table.remove(q1);
                        table.table.remove(q2);

                    }
                },
                None => {

                },
            }

            //println!("table entries {:?}" ,table.table.get(&p).unwrap());
        }

        table.get(&ntd.tree_structure.root(), &0).unwrap().clone()
    }



}


#[cfg(test)]
mod tests{
    use itertools::interleave;
    use crate::algorithms::diaz::{DPData, generate_edges};
    use crate::{diaz, file_handler, simple_brute_force};
    use crate::algorithms::integer_functions;
    use crate::algorithms::brute_force_homomorphism_counter;
    use crate::file_handler::file_handler::{create_ntd_from_file, metis_to_graph};

    #[test]
    fn test_dpdata(){
        let from_graph = metis_to_graph("data/metis_graphs/from_2.graph").unwrap();
        let to_graph = metis_to_graph("data/metis_graphs/to_2.graph").unwrap();
        let ntd = create_ntd_from_file("data/nice_tree_decompositions/example_2.ntd").unwrap();
        let mut test_dp_data = DPData::new(&from_graph,
                                                    &to_graph,
                                           &ntd,
                                           );

        test_dp_data.set(5,4,4);
        test_dp_data.set(2,3,5);
        test_dp_data.set(9,12,3);

        assert_eq!(*test_dp_data.get(&5,&4).unwrap(), 4);
        assert_eq!(*test_dp_data.get(&2,&3).unwrap(), 5);
        assert_eq!(*test_dp_data.get(&9,&12).unwrap(), 3);
    }

    #[test]
    fn test_integer_functions(){


        let n = 5;
        // 2 * 125 + 3 * 25 + 0 * 5 + 3 * 1    = 328
        let f : u64 = 328;

        assert_eq!(integer_functions::apply(n, f,0),3);
        assert_eq!(integer_functions::apply(n, f,1),0);
        assert_eq!(integer_functions::apply(n, f,2),3);
        assert_eq!(integer_functions::apply(n, f,3),2);

        // 3 * 1 + 0 * 5 + 3 * 25 + 2 * 125 = 328
        // -> 3 * 1 + 0 * 5 + (2 * 25) + 3 * 125 + 2 * 625 = 1678
        let f_2 = integer_functions::extend(n, f,2, 2);
        assert_eq!(f_2, 1678);

        // 3 * 1 + 0 * 5 + (2 * 25) + 3 * 125 + 2 * 625 = 1678
        // 0 * 1 + 2 * 5 + 3 * 25 + 2 * 125 = 335
        let f_3 = integer_functions::reduce(n, f_2, 0);
        assert_eq!(f_3, 335);

        // 3 * 1
        let f_4 = 3;
        // 2 * 1 + 3 * 5  = 17
        assert_eq!(integer_functions::extend(n,f_4,0,2), 17 );

        let n = 4;
        // 1 * 16 + 1 * 4 + 1 * 1 = 21
        let f_5 = 21;
        assert_eq!(integer_functions::apply(n, f_5, 1), 1);

        // 1 * 64 + 2 * 16 + 1 * 4 + 1 * 1 = 101
        let f_6 = integer_functions::extend(n, f_5, 2, 2);
        assert_eq!(f_6, 101);

        // 1 * 16 + 2 * 8 + 1 * 1 = 25
        let f_7 = integer_functions::reduce(n, f_6, 0);
        assert_eq!(f_7, 25);
    }

    #[test]
    fn test_integer_functions_in_DPData(){


        let from_graph = metis_to_graph("data/metis_graphs/from_2.graph").unwrap();
        let to_graph = metis_to_graph("data/metis_graphs/to_2.graph").unwrap();
        let ntd = create_ntd_from_file("data/nice_tree_decompositions/example_2.ntd").unwrap();
        let mut test_dp_data = DPData::new(&from_graph,
                                                    &to_graph,
                                           &ntd,
                                           );


        // testing apply function
        // to graph has 5 Vertices
        // 3 * 1 + 0 * 5 + 3 * 25 + 2 * 125 = 328

        let f : u64 = 328;
        assert_eq!(test_dp_data.apply(f,0),3);
        assert_eq!(test_dp_data.apply(f,1),0);
        assert_eq!(test_dp_data.apply(f,2),3);
        assert_eq!(test_dp_data.apply(f,3),2);

        // 3 * 1 + 0 * 5 + 3 * 25 + 2 * 125 = 328
        // -> 3 * 1 + 0 * 5 + (2 * 25) + 3 * 125 + 2 * 625 = 1678
        let f_2 = test_dp_data.extend(f,2, 2);
        assert_eq!(f_2, 1678);

        // 3 * 1 + 0 * 5 + (2 * 25) + 3 * 125 + 2 * 625 = 1678
        // 0 * 1 + 2 * 5 + 3 * 25 + 2 * 125 = 335
        let f_3 = test_dp_data.reduce(f_2, 0);
        assert_eq!(f_3, 335);

        // 3 * 1
        let f_4 = 3;
        // 2 * 1 + 3 * 5  = 17
        assert_eq!(test_dp_data.extend(f_4,0,2), 17 );



    }

    #[test]
    fn test_diaz(){
        let from_graph = file_handler::file_handler::metis_to_graph("data/metis_graphs/from_2.graph").unwrap();
        let to_graph = file_handler::file_handler::metis_to_graph("data/metis_graphs/to_2.graph").unwrap();
        let ntd = file_handler::file_handler::create_ntd_from_file("data/nice_tree_decompositions/example_2.ntd").unwrap();
        let i = diaz(&from_graph, ntd, &to_graph);
        assert_eq!(i,1280);

        let from_graph = file_handler::file_handler::metis_to_graph("data/metis_graphs/from_3.graph").unwrap();
        let to_graph = file_handler::file_handler::metis_to_graph("data/metis_graphs/to_3.graph").unwrap();
        let ntd = file_handler::file_handler::create_ntd_from_file("data/nice_tree_decompositions/example_2.ntd").unwrap();
        let i = diaz(&from_graph, ntd, &to_graph);
        assert_eq!(i,256);

        let from_graph = file_handler::file_handler::metis_to_graph("data/metis_graphs/from_4.graph").unwrap();
        let to_graph = file_handler::file_handler::metis_to_graph("data/metis_graphs/to_4.graph").unwrap();
        let ntd = file_handler::file_handler::create_ntd_from_file("data/nice_tree_decompositions/example_3.ntd").unwrap();
        let i = diaz(&from_graph, ntd, &to_graph);
        assert_eq!(i,0);

        let from_graph = file_handler::file_handler::metis_to_graph("data/metis_graphs/from_5.graph").unwrap();
        let to_graph = file_handler::file_handler::metis_to_graph("data/metis_graphs/to_4.graph").unwrap();
        let ntd = file_handler::file_handler::create_ntd_from_file("data/nice_tree_decompositions/example_3.ntd").unwrap();
        let i = diaz(&from_graph, ntd, &to_graph);
        assert_eq!(i,0);

        let from_graph = file_handler::file_handler::metis_to_graph("data/metis_graphs/from_6.graph").unwrap();
        let to_graph = file_handler::file_handler::metis_to_graph("data/metis_graphs/to_4.graph").unwrap();
        let ntd = file_handler::file_handler::create_ntd_from_file("data/nice_tree_decompositions/example_3.ntd").unwrap();
        let i = diaz(&from_graph, ntd, &to_graph);
        assert_eq!(i,0);

        let from_graph = file_handler::file_handler::metis_to_graph("data/metis_graphs/from_7.graph").unwrap();
        let to_graph = file_handler::file_handler::metis_to_graph("data/metis_graphs/to_2.graph").unwrap();
        let ntd = file_handler::file_handler::create_ntd_from_file("data/nice_tree_decompositions/ntd_4.ntd").unwrap();
        let i = diaz(&from_graph, ntd, &to_graph);
        assert_eq!(i,960);
    }

    #[test]
    fn test_brute_force() {
        let from_graph = file_handler::file_handler::metis_to_graph("data/metis_graphs/from_2.graph").unwrap();
        let to_graph = file_handler::file_handler::metis_to_graph("data/metis_graphs/to_2.graph").unwrap();
        let i = simple_brute_force(&from_graph, &to_graph);
        assert_eq!(i,1280);

        let from_graph = file_handler::file_handler::metis_to_graph("data/metis_graphs/from_3.graph").unwrap();
        let to_graph = file_handler::file_handler::metis_to_graph("data/metis_graphs/to_3.graph").unwrap();
        let i = simple_brute_force(&from_graph, &to_graph);
        assert_eq!(i,256);

        let from_graph = file_handler::file_handler::metis_to_graph("data/metis_graphs/from_4.graph").unwrap();
        let to_graph = file_handler::file_handler::metis_to_graph("data/metis_graphs/to_4.graph").unwrap();
        let i = simple_brute_force(&from_graph, &to_graph);
        assert_eq!(i,0);

        let from_graph = file_handler::file_handler::metis_to_graph("data/metis_graphs/from_5.graph").unwrap();
        let to_graph = file_handler::file_handler::metis_to_graph("data/metis_graphs/to_4.graph").unwrap();
        let i = simple_brute_force(&from_graph, &to_graph);
        assert_eq!(i,0);

        let from_graph = file_handler::file_handler::metis_to_graph("data/metis_graphs/from_6.graph").unwrap();
        let to_graph = file_handler::file_handler::metis_to_graph("data/metis_graphs/to_4.graph").unwrap();
        let i = simple_brute_force(&from_graph, &to_graph);
        assert_eq!(i,0);

        let from_graph = file_handler::file_handler::metis_to_graph("data/metis_graphs/from_7.graph").unwrap();
        let to_graph = file_handler::file_handler::metis_to_graph("data/metis_graphs/to_2.graph").unwrap();
        let i = simple_brute_force(&from_graph, &to_graph);
        assert_eq!(i,960);
    }

    // compares if two lists of edges have the same edges
    // O(n^2)
    fn compare_edge_lists(list1 : Vec<(usize, usize)>, list2 : Vec<(usize, usize)>) -> bool
    {
        //TODO: better notation with (de)reference operation
        for (u,v) in &list1{
            if !&list2.iter().any(|&i| i == (*u , *v) || i == (*v , *u) ){
                return false;
            }
        }

        for (u,v) in &list2{
            if !&list1.iter().any(|&i| i == (*u , *v) || i == (*v , *u)){
                return false;
            }
        }

        true
    }


    #[test]
    fn test_generate_edges(){
        let ntd = file_handler::file_handler::create_ntd_from_file("data/nice_tree_decompositions/example_2.ntd").unwrap();
        assert!(compare_edge_lists(vec![(4, 4), (4, 2), (2, 2), (2, 1), (1, 1), (0, 0), (1, 0), (3, 3), (3, 1)], generate_edges(ntd)));

        let ntd = file_handler::file_handler::create_ntd_from_file("data/nice_tree_decompositions/example.ntd").unwrap();
        assert!(compare_edge_lists(vec![(0,0),(1,1),(2,2),(3,3),(0,1),(1,2),(1,3)], generate_edges(ntd)));
    }

    #[test]
    fn test_generate_graphs(){
        //todo: create a test that checks if generated graph set is correct
    }
}
