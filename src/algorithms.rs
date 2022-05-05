//! Algorithms for counting graph homomorphisms
//!

/// A module containing all functions for explicit edge and graph generation.
pub mod generation {
    use itertools::Itertools;
    use petgraph::matrix_graph::{MatrixGraph, NodeIndex};
    use petgraph::Undirected;
    use crate::graph_structures::graph_structures::nice_tree_decomposition::NiceTreeDecomposition;


    /// Given a number of vertices and a set of possible edges this function computes all graphs
    /// with a subset of the possible edges and the same number of vertices.
    pub fn generate_graphs(number_of_vertices: u64, possible_edges : Vec<(usize, usize)>) -> Vec<petgraph::matrix_graph::MatrixGraph<(),(), Undirected>>{

        let mut graphs : Vec<petgraph::matrix_graph::MatrixGraph<(),(), Undirected>> = vec![];

        // iterate over the powerset of possible edges
        for edges in possible_edges.iter().powerset().collect::<Vec<_>>(){

            let mut graph : MatrixGraph<(), (), Undirected> = petgraph::matrix_graph::MatrixGraph::new_undirected();

            // add vertices
            for i in 0..number_of_vertices {
                graph.add_node(());
            }

            // add edges
            for (u,v) in edges{
                graph.add_edge(NodeIndex::new(*u),NodeIndex::new(*v), ());
            }

            graphs.push(graph);
        }

        graphs

    }


}

pub mod first_approach{
    use std::collections::{HashMap, HashSet};
    use std::hash::Hash;
    use itertools::{all, Itertools};
    use petgraph::dot::Dot;
    use petgraph::matrix_graph::{MatrixGraph, NodeIndex};
    use petgraph::Undirected;
    use crate::algorithms::integer_functions;
    use crate::algorithms::integer_functions::Mapping;
    use crate::generate_edges;
    use crate::graph_structures::graph_structures::nice_tree_decomposition::{NiceTreeDecomposition, NodeType, TreeNode, Vertex};

    /// a structure containing all necessary data for the Dynamic Program
    pub(crate) struct DPData<'a>{
        // table[p,e,phi], p = tree node, e = subset of edges represented by an integer, phi = mapping
        table : HashMap<TreeNode, HashMap<(u64, Mapping), u64>>,
        pub possible_edges_until: HashMap<TreeNode, Vec<(usize, usize)>>,
        nice_tree_decomposition : &'a NiceTreeDecomposition,
        to_graph : &'a MatrixGraph<(),(), Undirected>,

    }

    /// implementation of methods on DPData
    impl<'a> DPData<'a>{

        /// a basic constructor which takes only the nice tree decomposition as an argument
        pub fn new<'b>(nice_tree_decomposition : &'b NiceTreeDecomposition,
                       to_graph : &'b MatrixGraph<(),(), Undirected>) -> DPData<'b>{
            DPData{table : HashMap::new(),
                possible_edges_until: HashMap::new(),
                nice_tree_decomposition,
                to_graph}
        }

        /// given p = tree node, e = subset of edges represented by an integer, phi = mapping
        /// this functions returns the entry : table[p,e,phi]
        pub fn get(&self, node : TreeNode, edge_set : u64 , mapping : Mapping) -> Option<&u64> {

            if let Some(node_data) = self.table.get(&node){
                node_data.get(&(edge_set, mapping))
            }
            else { None }
        }

        /// sets the entry table[p,e,phi] to value
        pub fn set(&mut self, node : TreeNode, edge_set : u64 , mapping : Mapping, value : u64) {
            if let Some(node_data) = self.table.get_mut(&node)
            {
                node_data.insert((edge_set,mapping), value);
            }
            else {
                self.table.insert(node, HashMap::from([ ((edge_set, mapping),value) ]));
            }
        }

        /// integer_functions::apply where the base is set to |V(to_graph)|
        pub fn apply(&self, f : Mapping, s : Mapping) -> Mapping{
            integer_functions::apply(self.to_graph.node_count() as Mapping, f, s)
        }

        /// integer_functions::extend where the base is set to |V(to_graph)|
        pub fn extend(&self, f : Mapping, s : Mapping, v : Mapping) -> Mapping{
            integer_functions::extend(self.to_graph.node_count() as Mapping, f, s, v)
        }

        /// integer_functions::reduce where the base is set to |V(to_graph)|
        pub fn reduce(&self, f : Mapping, s : Mapping) -> Mapping{
            integer_functions::reduce(self.to_graph.node_count() as Mapping, f, s)
        }

        /// integer_functions::max_mappings where the base is set to |V(to_graph)|
        /// and the number of digits is set to the size of the bag of node
        pub fn max_bag_mappings(&self, node : TreeNode) -> Mapping{
            integer_functions::max_mappings(self.nice_tree_decomposition.bag(node).unwrap().len() as Mapping,
                                            self.to_graph.node_count() as Mapping )
        }

        /// returns the sorted bag of a given node as a Vector of Vertices
        pub fn sorted_bag(&self, node : TreeNode) -> Vec<Vertex>{
            let mut v: Vec<&Vertex> = Vec::from_iter(self.nice_tree_decomposition.bag(node).unwrap().iter());
            v.sort();
            v.iter().map(|e| **e).collect()
        }


    }

    pub fn first_approach(ntd : &NiceTreeDecomposition, to_graph : &MatrixGraph<(),(), Undirected> ) -> Vec<(MatrixGraph<(),(), Undirected>, u64)>
    {
        let stingy_ordering = ntd.stingy_ordering();

        let mut table = DPData::new(ntd,to_graph);

        // todo: Clone is not nice -> Just borrow later
        let possible_edges = generate_possible_edges(ntd);

        // Mapping each edge onto its index
        let mut edge_to_index : HashMap<(usize,usize), usize> = HashMap::new();
        let all_possible_edges = possible_edges.get(&ntd.root()).unwrap();
        for (pos, (u,v)) in all_possible_edges.iter().enumerate(){
            // Inserting edges in both direction such that thex will always be found
            // possible edges contain edges only in one direction
            edge_to_index.insert((*u, *v), pos );
            edge_to_index.insert((*v, *u), pos );
        }

        // Creat a list of possible edges by their indices
        let mut possible_edge_indices = HashMap::new();
        for (a,b) in possible_edges.iter(){

            let mut index_list : Vec<usize> = vec![];
            for i in b{
                index_list.push(*edge_to_index.get(i).unwrap());
            }
            possible_edge_indices.insert(*a, index_list);

        }

        // go through every node of the stingy ordering
        for p in stingy_ordering{
            println!("{:?}",p);
            match ntd.node_type(p){
                Some(NodeType::Leaf) => {
                    println!("Leaf");
                    let unique_vertex = ntd.bag(p).unwrap().iter().next().unwrap();

                    // go through all mappings
                    for aim_vertex in 0..to_graph.node_count() {

                        // sets the entry for the node p the empty graph with
                        // 0 edges and the mapping (v, aim_vertex) to 1
                        table.set(p, 0, aim_vertex as Mapping, 1);
                    }

                    //find index of the edge (v,v)
                    // todo: make this more beautiful
                    let index = *edge_to_index.get(&(unique_vertex.index(), unique_vertex.index())).unwrap();
                    // we inserting a 1 at the index (of the self loop edge) position of the binary number
                    let edges = 2_u32.pow(index as u32) as u64;
                    println!("leaf edge set {:?}", edges);

                    for aim_vertex in 0..to_graph.node_count() {
                        // check aim_vertex also has a self_loop
                        if to_graph.has_edge(Vertex::new(aim_vertex),Vertex::new(aim_vertex)){
                            // sets the entry for the node p the empty graph with
                            // 0 edges and the mapping (v, aim_vertex) to 1
                            table.set(p, edges, aim_vertex as Mapping, 1);
                        }
                        else
                        {
                            table.set(p, edges, aim_vertex as Mapping, 0);
                        }

                    }

                    println!("table state {:?}", table.table);
                }
                Some(NodeType::Introduce) => {
                    println!("Intro");
                    let q = *ntd.unique_child(p).unwrap();
                    let v = ntd.introduced_vertex(p).unwrap();

                    let pos_edges_of_p = possible_edge_indices.get(&p).unwrap();

                    // MAIN LOOP
                    for edges in pos_edges_of_p.iter().powerset().collect::<Vec<_>>(){

                        // number representation of the edge set
                        let edges_number = {
                            let mut n = 0;
                            for i in edges.clone(){
                                n += 2_u32.pow(*i as u32)
                            }
                            n
                        };

                        //let neighbours : Vec<Vertex> = from_graph.neighbors(v).collect();
                        //let mut neighbour_set: HashSet<Vertex> = HashSet::from_iter(neighbours);
                        let mut s_q : Vec<Vertex> = vec![];
                        for edge in edges{
                            let (a,b) = all_possible_edges[*edge];

                            // self loops will be dealt separately
                            if a == v.index() && b == v.index()
                            {
                                break;
                            }

                            if a == v.index(){
                                s_q.push(Vertex::new(b));
                            }

                            if b == v.index() {
                                s_q.push(Vertex::new(a));
                            }
                        }

                        // ############################### Integer function stuff #################################
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
                            for a in 0..to_graph.node_count() {

                                let test_condition = {
                                    //println!("testing condition");
                                    let mut t = true;
                                    for u in s_q.clone(){

                                        let first_vertex = Vertex::new(a);
                                        let second_vertex = Vertex::new(table.apply(f_q,*significance_q(&u) as Mapping ) as usize);
                                        //println!("{:?} mapped to {:?}", u, second_vertex);

                                        //println!("checking edge ({:?}, {:?})", first_vertex, second_vertex);

                                        if !to_graph.has_edge( first_vertex, second_vertex){
                                            //println!("graph G does not have that edge");
                                            t = false;
                                            break;
                                        }


                                    }

                                    //additonal sucht that self loops will be mapped on self loops
                                    let self_loop_index = edge_to_index.get(&(v.index(),v.index())).unwrap();

                                    // Checks if bit of the self loop edge has been set.
                                    let decider = edges_number / 2_u32.pow(*self_loop_index as u32 - 1) % 2;

                                    if decider == 1 && !to_graph.has_edge(Vertex::new(a),Vertex::new(a))
                                    {
                                        t = false;
                                    }


                                    t
                                };

                                let f = table.extend(f_q, *significance_p(&v) as Mapping, a as Mapping).clone();


                                if test_condition{
                                    //possible edges of q
                                    let pos_edges_of_q = possible_edge_indices.get(&q).unwrap();

                                    // Representation of possible edges of q as a number
                                    let pos_edges_of_q_number = {
                                        let mut n = 0;
                                        for i in pos_edges_of_q.clone(){
                                            n += 2_u32.pow(i as u32)
                                        }
                                        n
                                    };

                                    // intersection of both edge sets by bitwise AND
                                    let old_edge_set_number = edges_number & pos_edges_of_q_number;

                                    //println!("{:?} AND {:?} = {:?}", edges_number, pos_edges_of_q_number ,old_edge_set_number);

                                    println!("table get node : {:?}, edge_set : {:?}, mapping : {:?} ",q, old_edge_set_number as u64, f_q);

                                    let value = table.get(q, old_edge_set_number as u64, f_q).unwrap().clone();

                                    table.set(p,
                                              edges_number as u64,
                                              f,
                                              value);
                                }
                                else {
                                    table.set(p,
                                              edges_number as u64,
                                              f,
                                              0);
                                }

                                //todo: continue
                            }
                        }

                    }
                    table.table.remove(&q);

                }
                Some(NodeType::Forget) => {
                    println!(">Forget");
                    let q = *ntd.unique_child(p).unwrap();
                    let v = ntd.forgotten_vertex(p).unwrap();

                    // transforms the bag into a sorted vertex used for integer functions
                    let sorted_bag = table.sorted_bag(p);

                    let pos_edges_of_p = possible_edge_indices.get(&p).unwrap();

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

                    // MAIN LOOP
                    for edges in pos_edges_of_p.iter().powerset().collect::<Vec<_>>() {

                        // number representation of the edge set
                        let edges_number = {
                            let mut n = 0;
                            for i in edges.clone() {
                                n += 2_u32.pow(*i as u32)
                            }
                            n
                        };

                        //let neighbours : Vec<Vertex> = from_graph.neighbors(v).collect();
                        //let mut neighbour_set: HashSet<Vertex> = HashSet::from_iter(neighbours);
                        let mut s_q: Vec<Vertex> = vec![];
                        for edge in edges {
                            let (a, b) = all_possible_edges[*edge];
                            if a == v.index() {
                                s_q.push(Vertex::new(b));
                            }

                            if b == v.index() {
                                s_q.push(Vertex::new(a));
                            }
                        }

                        for f in 0..table.max_bag_mappings(p) {
                            let mut sum = 0;
                            for a in 0..to_graph.node_count() {
                                let f_old = table.extend(f,old_significance(&v) as Mapping, a as Mapping);

                                let additional_mappings = table.get(q, edges_number as u64, f_old).unwrap();
                                sum += additional_mappings;

                            }
                            table.set(p, edges_number as u64, f, sum);
                        }

                    }

                    table.table.remove(&q);
                }
                Some(NodeType::Join) => {
                    println!("join");
                    if let Some(children) = ntd.children(p){
                        let q1 = children.get(0).unwrap();
                        let q2 = children.get(1).unwrap();

                        let pos_edges_of_q1 = possible_edge_indices.get(&q1).unwrap();
                        let pos_edges_of_q2 = possible_edge_indices.get(&q2).unwrap();
                        // number representation of the edge set
                        let pos_edges_q1_number = {
                            let mut n = 0;
                            for i in pos_edges_of_q1.clone(){
                                n += 2_u32.pow(i as u32)
                            }
                            n
                        };
                        let pos_edges_q2_number = {
                            let mut n = 0;
                            for i in pos_edges_of_q2.clone(){
                                n += 2_u32.pow(i as u32)
                            }
                            n
                        };


                        let pos_edges_of_p = possible_edge_indices.get(&p).unwrap();

                        for edges in pos_edges_of_p.iter().powerset().collect::<Vec<_>>(){

                            // number representation of the edge set
                            let edges_number = {
                                let mut n = 0;
                                for i in edges.clone() {
                                    n += 2_u32.pow(*i as u32)
                                }
                                n
                            };

                            // Updates every new mapping
                            for f in 0..table.max_bag_mappings(p){

                                let intersection_q1 = edges_number & pos_edges_q1_number;
                                let intersection_q2 = edges_number & pos_edges_q2_number;

                                table.set(p,
                                          edges_number as u64,
                                          f as Mapping,
                                          table.get(*q1, intersection_q1 as u64, (f as Mapping)).unwrap() *
                                              table.get(*q2, intersection_q2 as u64, (f as Mapping)).unwrap()
                                );
                            }


                        }
                        // Deletes entries after use...
                        table.table.remove(q1);
                        table.table.remove(q2);


                    }
                }
                None => {println!("test")}
            }

        }

        let final_list = table.table.get(&ntd.root()).unwrap();

        let number_of_vertices= {

            let mut max = 0;

            for (u,v) in all_possible_edges{
                if *u > max {max = *u}
                if *v > max {max = *v}
            }

            // cause we get the biggest vertex index n, and the iterator always iterates from 0..n-1
            max + 1
        };

        let integer_to_graph = |x : u64| {

            let mut edges = vec![];

            for i in 0..all_possible_edges.len() as u32{
                let filter = 2_u32.pow(i) as u64;
                if x & filter == filter{
                    edges.push(all_possible_edges[i as usize]);
                }
            }

            let mut graph : MatrixGraph<(), (), Undirected> = petgraph::matrix_graph::MatrixGraph::new_undirected();


            // add vertices
            for i in 0..number_of_vertices {
                graph.add_node(());
            }
            // add edges
            for (u,v) in edges{
                graph.add_edge(NodeIndex::new(u),NodeIndex::new(v), ());
            }
            graph

        };

        //println!("lenght : {:?}",final_list.len());
        println!("{:?}",final_list);

        let mut graph_hom_number_list = vec![];

        for ((graph_number, i),hom_number) in final_list{
            println!("graph number {:?}", graph_number);
            println!("hom number {:?}", hom_number.clone());
            println!("graph {:?}", Dot::new(&integer_to_graph(*graph_number)));
            graph_hom_number_list.push((integer_to_graph(*graph_number), hom_number.clone()));
        }

        graph_hom_number_list

    }


}

#[cfg(test)]
mod tests{

    /*
    useful commands
    - cargo test : runs unit tests
    - cargo test -- --nocapture : runs unit tests and produces the hidden outputs such as println!
     */

    use itertools::interleave;
    use crate::algorithms::diaz::{diaz, DPData};
    use crate::{diaz, file_handler, generate_edges, simple_brute_force};
    use crate::algorithms::{first_approach, integer_functions};
    use crate::algorithms::brute_force_homomorphism_counter;
    use crate::algorithms::brute_force_homomorphism_counter::simple_brute_force;
    use crate::algorithms::generation::generate_edges;
    use crate::file_handler::file_handler::{create_ntd_from_file, metis_to_graph};

    #[test]
    fn test_my_approach_dpdata(){
        let ntd = create_ntd_from_file("data/nice_tree_decompositions/example_2.ntd").unwrap();
        let to_graph = metis_to_graph("data/metis_graphs/to_2.graph").unwrap();
        let mut test_dp_data =  first_approach::DPData::new(&ntd, &to_graph);


        assert_eq!(test_dp_data.get(1,1,1), None);
        assert_eq!(test_dp_data.get(1,2,3), None);

        test_dp_data.set(1,1,1,5);
        test_dp_data.set(1,2,3,4);

        assert_eq!(*test_dp_data.get(1,1,1).unwrap(), 5);
        assert_eq!(*test_dp_data.get(1,2,3).unwrap(), 4);

    }

    #[test]
    fn test_integer_functions_in_diaz_dpdata(){


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
    fn test_generate_edges(){
        let ntd = file_handler::tree_decomposition_handler::import_ntd("data/nice_tree_decompositions/example_2.ntd").unwrap();
        assert!(compare_edge_lists(vec![(4, 4), (4, 2), (2, 2), (2, 1), (1, 1), (0, 0), (1, 0), (3, 3), (3, 1)], generate_edges(ntd)));

        let ntd = file_handler::tree_decomposition_handler::import_ntd("data/nice_tree_decompositions/example.ntd").unwrap();
        assert!(compare_edge_lists(vec![(0,0),(1,1),(2,2),(3,3),(0,1),(1,2),(1,3)], generate_edges(ntd)));
    }

    #[test]
    fn test_generate_edges_mapping(){
        let ntd = file_handler::tree_decomposition_handler::import_ntd("data/nice_tree_decompositions/example_2.ntd").unwrap();
        let possible_edges = first_approach::generate_possible_edges(&ntd);
        let all_possible_edges = possible_edges.get(&ntd.root()).unwrap().clone();
        assert!(compare_edge_lists(vec![(4, 4), (4, 2), (2, 2), (2, 1), (1, 1), (0, 0), (1, 0), (3, 3), (3, 1)],
                                   all_possible_edges));

        let ntd = file_handler::tree_decomposition_handler::import_ntd("data/nice_tree_decompositions/example.ntd").unwrap();
        let possible_edges = first_approach::generate_possible_edges(&ntd);
        let all_possible_edges = possible_edges.get(&ntd.root()).unwrap().clone();
        assert!(compare_edge_lists(vec![(0,0),(1,1),(2,2),(3,3),(0,1),(1,2),(1,3)],
                                   all_possible_edges));

        let ntd  = file_handler::tree_decomposition_handler::import_ntd("data/nice_tree_decompositions/ntd_4.ntd").unwrap();
        let possible_edges = first_approach::generate_possible_edges(&ntd);
        let all_possible_edges = possible_edges.get(&4).unwrap().clone();
        assert!(compare_edge_lists(vec![(0,0),(1,1),(2,2),(0,1),(1,2),(0,2)],
                                   all_possible_edges));

        let all_possible_edges = possible_edges.get(&10).unwrap().clone();
        assert!(compare_edge_lists(vec![(0,0),(1,1),(2,2),(3,3),(4,4),(0,1),(1,2),(0,2),(2,3),(3,4)],
                                   all_possible_edges));

    }

    #[test]
    fn test_generate_graphs(){
        //todo: create a test that checks if generated graph set is correct
    }
}
