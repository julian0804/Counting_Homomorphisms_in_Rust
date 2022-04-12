
pub mod diaz {
    use std::borrow::Borrow;
    use std::cmp::max;
    use std::collections::{HashMap, HashSet};
    use std::iter::Map;
    use petgraph::matrix_graph::{MatrixGraph, NodeIndex};
    use petgraph::Undirected;
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
        The Following operations work on integer functions
        mappings from a bag to a graph are represented as simple integers

        based on  **Counting subgraph patterns in large graphs**

         */

        /*
        Returns digit of mapping f with significance s
         */
        pub fn apply(&self, f : Mapping, s : Mapping) -> Mapping{
            // TODO: ugly safe casting
            f / (self.to_graph.node_count().pow(s as u32) as u64) % (self.to_graph.node_count() as u64)
        }

        /*
        "increases the number of digits in f by one. It should
        shift all digits with significance > s one position to the left and then set the
        digit with significance s equal to v."
         */
        pub fn extend(&self, f : Mapping, s : Mapping, v : Mapping) -> Mapping{
            let n = self.to_graph.node_count();
            let r = f % (n.pow(s as u32) as Mapping);
            let l = f - r;
            ((n as Mapping) * l) + (n.pow(s as u32) as Mapping) * v + r
        }

        /*
        "should remove the digit with significance s and shift
        all digits with higher significance one position to the right."
         */
        pub fn reduce(&self, f : Mapping, s : Mapping) -> Mapping{
            let n = self.to_graph.node_count();
            let r = f % (n.pow(s as u32) as Mapping);
            let l = f - (f % n.pow((s + 1) as u32) as u64);
            l / (n as Mapping) + r
        }

        /*
        Returns the maximum mapping from a bag of a given node to the to_graph + 1
        for the iterators
         */
        pub fn max_bag_mappings(&self, node : TreeNode) -> Mapping{
            let bag_size = self.nice_tree_decomposition.bag(node).unwrap().len();
            let number_of_vertices = self.to_graph.node_count();
            number_of_vertices.pow(bag_size as u32) as Mapping
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
    Based on the following algorithm of Diaz, Serna, Thilikos
    https://www.sciencedirect.com/science/article/pii/S0304397502000178?via%3Dihub

    1. Use Hashmaps for representing the mappings
     */
    pub fn diaz(from_graph : &MatrixGraph<(),(), Undirected>, ntd : NiceTreeDecomposition, to_graph : &MatrixGraph<(),(), Undirected>) -> u64
    {
        let stingy_order = ntd.stingy_ordering();

        let mut table = DPData::new( from_graph, to_graph, &ntd);

        for p in stingy_order{
            match ntd.node_type(p){
                Some(NodeType::Leaf) => {
                    let unique_vertex = ntd.bag(p).unwrap().iter().next().unwrap();
                    // be carefully, we return the number of vertices

                    // inserts the mapping (unique_vertex -> aim_vertex) for each
                    // aim_vertex in the aim graph
                    for aim_vertex in 0..to_graph.node_count(){
                        table.set(p, aim_vertex as Mapping, 1);
                    }
                },
                Some(NodeType::Introduce) => {
                    // TODO: make unique_child & introduced_vertex also to methods of NiceTreeDecomposition
                    // So tree_structure do not have to be public
                    let q = *ntd.tree_structure.unique_child(p).unwrap();
                    let v = ntd.tree_structure.introduced_vertex(p).unwrap();

                    // For calculating S_q
                    let neighbours : Vec<Vertex> = from_graph.neighbors(v).collect();
                    let neighbour_set: HashSet<Vertex> = HashSet::from_iter(neighbours);
                    let s_q : Vec<&Vertex> = neighbour_set.intersection(ntd.bag(q).unwrap()).collect(); // possible error case, explanation below
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


                            let test_condition = {
                                let mut t = true;

                                for u in s_q.clone(){
                                    if !to_graph.has_edge(Vertex::new(a),
                                                          Vertex::new(table.apply(f_q,*significance_q(u) as Mapping ) as usize)){
                                        t = false;
                                        break;
                                    }
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
                    let q = ntd.tree_structure.unique_child(p).unwrap();
                    let v = ntd.tree_structure.forgotten_vertex(p).unwrap();

                    // transforms the bag into a sorted vertex used for integer functions
                    let sorted_bag = table.sorted_bag(p);


                    let old_significance = |a : &Vertex|{
                        if let Some(i) = sorted_bag.iter().position(|i| a > i){
                            i
                        }
                        else {
                            sorted_bag.len()
                        }
                    };

                    for f in 0..table.max_bag_mappings(p){
                        let mut sum = 0;
                        for a in 0..to_graph.node_count(){
                            let f_old = table.extend(f,old_significance(&v) as Mapping, a as Mapping);
                            sum += table.get(&q, &f_old).unwrap();
                        }
                        table.set(p, f, sum);
                    }

                    table.table.remove(q);


                },
                Some(NodeType::Join) => {
                    if let Some(children) = ntd.tree_structure.children(p){
                        let q1 = children.get(0).unwrap();
                        let q2 = children.get(1).unwrap();

                        let max_mapping = ||{
                            let b = ntd.bag(p).unwrap().len() as Mapping; // number of vertices in a bag
                            let n = to_graph.node_count();
                            n.pow((b) as u32)
                        };

                        for f in 0..table.max_bag_mappings(p){
                            table.set(p,
                                      f as Mapping,
                                      table.get(q1, &(f as Mapping)).unwrap() *
                                          table.get(q2, &(f as Mapping)).unwrap());
                        }

                        table.table.remove(q1);
                        table.table.remove(q2);

                    }
                },
                None => {

                },
            }
        }

        table.get(&ntd.tree_structure.root(), &0).unwrap().clone()
    }

}



#[cfg(test)]
mod tests{
    use crate::algorithms::diaz::DPData;
    use crate::file_handler::file_handler::{create_ntd_from_file, metis_to_graph};

    #[test]
    fn test_dpdata(){
        let from_graph = metis_to_graph("data/metis_graphs/from_2.graph").unwrap();
        let to_graph = metis_to_graph("data/metis_graphs/to_2.graph").unwrap();
        let mut test_dp_data = DPData::new(&from_graph,
                                           &create_ntd_from_file("data/nice_tree_decompositions/example_2.ntd"),
                                           &to_graph);

        test_dp_data.set(5,4,4);
        test_dp_data.set(2,3,5);
        test_dp_data.set(9,12,3);

        assert_eq!(*test_dp_data.get(&5,&4).unwrap(), 4);
        assert_eq!(*test_dp_data.get(&2,&3).unwrap(), 5);
        assert_eq!(*test_dp_data.get(&9,&12).unwrap(), 3);
    }

    #[test]
    fn test_integer_functions(){

        let from_graph = metis_to_graph("data/metis_graphs/from_2.graph").unwrap();
        let to_graph = metis_to_graph("data/metis_graphs/to_2.graph").unwrap();
        let mut test_dp_data = DPData::new(&from_graph,
                                           &create_ntd_from_file("data/nice_tree_decompositions/example_2.ntd"),
                                           &to_graph);

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

}
