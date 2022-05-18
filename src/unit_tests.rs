use std::collections::HashMap;
use crate::tree_decompositions::nice_tree_decomposition::{Bag, NiceTreeDecomposition, NodeData, NodeType};
use crate::tree_decompositions::tree_structure::{TreeStructure, Vertex};

/// hard-wired example one
fn ntd_test_example() -> NiceTreeDecomposition{

    let mut tree_structure = TreeStructure::new(10);
    tree_structure.add_child(1,0);
    tree_structure.add_child(2,1);
    tree_structure.add_child(6,2);
    tree_structure.add_child(4,3);
    tree_structure.add_child(5,4);
    tree_structure.add_child(6,5);
    tree_structure.add_child(7,6);
    tree_structure.add_child(8,7);
    tree_structure.add_child(9,8);

    let mut nodes_data = HashMap::new();
    nodes_data.insert(0, NodeData::new(NodeType::Leaf, Bag::from([Vertex::new(0)])));
    nodes_data.insert(1, NodeData::new(NodeType::Introduce, Bag::from([Vertex::new(0), Vertex::new(1)])));
    nodes_data.insert(2, NodeData::new(NodeType::Forget, Bag::from([Vertex::new(1)])));
    nodes_data.insert(3, NodeData::new(NodeType::Leaf, Bag::from([Vertex::new(1)])));
    nodes_data.insert(4, NodeData::new(NodeType::Introduce, Bag::from([Vertex::new(1),Vertex::new(2)])));
    nodes_data.insert(5, NodeData::new(NodeType::Forget, Bag::from([Vertex::new(1)])));
    nodes_data.insert(6, NodeData::new(NodeType::Join, Bag::from([Vertex::new(1)])));
    nodes_data.insert(7, NodeData::new(NodeType::Introduce, Bag::from([Vertex::new(1), Vertex::new(3)])));
    nodes_data.insert(8, NodeData::new(NodeType::Forget, Bag::from([Vertex::new(3)])));
    nodes_data.insert(9, NodeData::new(NodeType::Forget, Bag::from([])));

    NiceTreeDecomposition::new(tree_structure, nodes_data, 4, 1)
}

// naive comparison of two edge lists.
// O(len(list1) * len(list2))
fn compare_edge_lists(list1 : &Vec<(usize, usize)>, list2 : &Vec<(usize, usize)>) -> bool
{
    for (u,v) in list1{
        if !&list2.iter().any(|&i| i == (*u , *v) || i == (*v , *u) ){
            return false;
        }
    }
    for (u,v) in list2{
        if !&list1.iter().any(|&i| i == (*u , *v) || i == (*v , *u)){
            return false;
        }
    }
    true
}

#[cfg(test)]
pub mod tree_structure_tests{
    use crate::tree_decompositions::tree_structure;

    #[test]
    pub fn test_tree_structure_methods(){

        let mut tree_structure = tree_structure::TreeStructure::new(5);

        // Before adding edges
        assert_eq!(tree_structure.is_parent_of(4,0), false);
        assert_eq!(tree_structure.node_count(), 5);
        assert_eq!(tree_structure.parent(1), None);
        assert_eq!(tree_structure.root(), 0);
        assert_eq!(tree_structure.children_count(0), 0);

        // Adding edges
        tree_structure.add_child(4,0);
        tree_structure.add_child(0,2);
        tree_structure.add_child(0,1);
        tree_structure.add_child(1,3);

        // After adding edges
        assert_eq!(tree_structure.is_parent_of(4,0), true);
        assert_eq!(tree_structure.node_count(), 5);
        assert_eq!(tree_structure.parent(1), Some(&0));
        assert_eq!(tree_structure.root(), 4);
        assert_eq!(tree_structure.children_count(0), 2);
    }
}

#[cfg(test)]
pub mod nice_tree_decomposition_tests{
    use std::collections::{HashMap, HashSet};
    use crate::file_handler::tree_decomposition_handler::import_ntd;
    use crate::tree_decompositions::nice_tree_decomposition::{Bag, NiceTreeDecomposition, NodeData, NodeType};
    use crate::tree_decompositions::tree_structure::{TreeStructure, Vertex};
    use crate::unit_tests::ntd_test_example;


    #[test]
    fn test_stingy_ordering(){

        let ntd = ntd_test_example();
        assert_eq!(ntd.stingy_ordering(), vec![0,1,2,3,4,5,6,7,8,9]);

        let ntd = import_ntd("data/nice_tree_decompositions/example_2.ntd").unwrap();
        assert_eq!(ntd.stingy_ordering(),vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13]);
    }

    #[test]
    fn test_nice_tree_decomposition_basic(){
        let ntd = ntd_test_example();

        assert_eq!(ntd.node_count(), 10);
        assert_eq!(ntd.vertex_count(), 4);

        // test children for each node type
        assert_eq!(ntd.children(0), None); // Leaf
        assert_eq!(ntd.children(7), Some(&vec![6])); // Introduce
        assert_eq!(ntd.children(2), Some(&vec![1])); // Forget
        assert_eq!(ntd.children(6), Some(&vec![2, 5])); // Join

        // test parent for each node type and the root
        assert_eq!(ntd.parent(9), None); // root
        assert_eq!(ntd.parent(0), Some(&(1 as u64))); // leaf
        assert_eq!(ntd.parent(4), Some(&(5 as u64))); // Introduce
        assert_eq!(ntd.parent(8), Some(&(9 as u64))); // Forget
        assert_eq!(ntd.parent(6), Some(&(7 as u64))); // Join

        // test children count
        assert_eq!(ntd.children_count(6), 2);
        assert_eq!(ntd.children_count(0), 0);
        assert_eq!(ntd.children_count(2), 1);

        // test is_child_of
        assert!(ntd.is_parent_of(6, 2));
        assert!(ntd.is_parent_of(6, 5));
        assert!(ntd.is_parent_of(9, 8));
        assert!(!ntd.is_parent_of(9, 0));
        assert!(!ntd.is_parent_of(2, 6));

        // test root
        assert_eq!(ntd.root(), 9);

        // test bag
        assert_eq!(ntd.bag(0), Some(&HashSet::from([Vertex::new(0)])));
        assert_eq!(ntd.bag(7), Some(&HashSet::from([Vertex::new(1), Vertex::new(3)])));
        assert_eq!(ntd.bag(6), Some(&HashSet::from([Vertex::new(1)])));
        assert_eq!(ntd.bag(9), Some(&HashSet::from([])));
        assert_eq!(ntd.bag(8), Some(&HashSet::from([Vertex::new(3)])));

        // test node_type()
        assert_eq!(ntd.node_type(0), Some(&NodeType::Leaf));
        assert_eq!(ntd.node_type(4), Some(&NodeType::Introduce));
        assert_eq!(ntd.node_type(8), Some(&NodeType::Forget));
        assert_eq!(ntd.node_type(6), Some(&NodeType::Join));

        // test unique child
        assert_eq!(ntd.unique_child(7), Some(&6));
        assert_eq!(ntd.unique_child(2), Some(&1));
        assert_eq!(ntd.unique_child(0), None);
        assert_eq!(ntd.unique_child(6), None);

        // Join nodes do not have an unique vertex
        assert_eq!(ntd.unique_vertex(6), None);

        // test introduced vertices
        assert_eq!(ntd.unique_vertex(1), Some(&Vertex::new(1)));
        assert_eq!(ntd.unique_vertex(4), Some(&Vertex::new(2)));
        assert_eq!(ntd.unique_vertex(7), Some(&Vertex::new(3)));

        // test forgotten vertices
        assert_eq!(ntd.unique_vertex(2), Some(&Vertex::new(0)));
        assert_eq!(ntd.unique_vertex(5), Some(&Vertex::new(2)));
        assert_eq!(ntd.unique_vertex(8), Some(&Vertex::new(1)));

        // test leaf nodes
        assert_eq!(ntd.unique_vertex(0), Some(&Vertex::new(0)));
        assert_eq!(ntd.unique_vertex(3), Some(&Vertex::new(1)));
    }

}

#[cfg(test)]
pub mod tree_decomposition_handler_tests{
    use crate::file_handler::tree_decomposition_handler::import_ntd;
    use crate::unit_tests::ntd_test_example;

    #[test]
    pub fn test_ntd_import() {
        let ntd = ntd_test_example();
        assert_eq!(import_ntd("data/nice_tree_decompositions/example.ntd").unwrap(), ntd);
    }
}

#[cfg(test)]
pub mod graph_handler_tests{
    use crate::file_handler::graph_handler::{import_dimacs, import_metis};
    use crate::tree_decompositions::tree_structure::Vertex;

    #[test]
    pub fn test_import_metis()
    {
        let edges = vec![
            (0, 4), (0, 2), (0, 1),
            (1, 0), (1, 2), (1, 3),
            (2, 4), (2, 3), (2, 1), (2, 0),
            (3, 1), (3, 2), (3, 5), (3, 6),
            (4, 0), (4, 2), (4, 5),
            (5, 4), (5, 3), (5, 6),
            (6, 5), (6, 3)];

        let g = import_metis("data/metis_graphs/tiny_01.graph").unwrap();

        assert_eq!(g.node_count(), 7);
        assert_eq!(g.edge_count(), 11);
        for (a,b) in edges{
            assert!(g.has_edge(Vertex::new(a), Vertex::new(b)));
        }
    }

    #[test]
    pub fn test_import_gr()
    {
        let edges = vec![
            (0, 4), (0, 2), (0, 1),
            (1, 0), (1, 2), (1, 3),
            (2, 4), (2, 3), (2, 1), (2, 0),
            (3, 1), (3, 2), (3, 5), (3, 6),
            (4, 0), (4, 2), (4, 5),
            (5, 4), (5, 3), (5, 6),
            (6, 5), (6, 3)];

        let g = import_dimacs("data/dimacs_graphs/tiny_01.gr").unwrap();

        assert_eq!(g.node_count(), 7);
        assert_eq!(g.edge_count(), 11);
        for (a,b) in edges{
            assert!(g.has_edge(Vertex::new(a), Vertex::new(b)));
        }
    }
}

#[cfg(test)]
pub mod brute_force_tests{
    use crate::brute_force::brute_force_homomorphism_counter::simple_brute_force;
    use crate::file_handler::graph_handler::import_metis;

    #[test]
    fn test_brute_force() {
        let from_graph = import_metis("data/metis_graphs/from_2.graph").unwrap();
        let to_graph = import_metis("data/metis_graphs/to_2.graph").unwrap();
        let i = simple_brute_force(&from_graph, &to_graph);
        assert_eq!(i,1280);

        let from_graph = import_metis("data/metis_graphs/from_3.graph").unwrap();
        let to_graph = import_metis("data/metis_graphs/to_3.graph").unwrap();
        let i = simple_brute_force(&from_graph, &to_graph);
        assert_eq!(i,256);

        let from_graph = import_metis("data/metis_graphs/from_4.graph").unwrap();
        let to_graph = import_metis("data/metis_graphs/to_4.graph").unwrap();
        let i = simple_brute_force(&from_graph, &to_graph);
        assert_eq!(i,0);

        let from_graph = import_metis("data/metis_graphs/from_5.graph").unwrap();
        let to_graph = import_metis("data/metis_graphs/to_4.graph").unwrap();
        let i = simple_brute_force(&from_graph, &to_graph);
        assert_eq!(i,0);

        let from_graph = import_metis("data/metis_graphs/from_6.graph").unwrap();
        let to_graph = import_metis("data/metis_graphs/to_4.graph").unwrap();
        let i = simple_brute_force(&from_graph, &to_graph);
        assert_eq!(i,0);

        let from_graph = import_metis("data/metis_graphs/from_7.graph").unwrap();
        let to_graph = import_metis("data/metis_graphs/to_2.graph").unwrap();
        let i = simple_brute_force(&from_graph, &to_graph);
        assert_eq!(i,960);
    }

}

#[cfg(test)]
pub mod diaz_tests{
    use crate::diaz;
    use crate::file_handler::graph_handler::import_metis;
    use crate::file_handler::tree_decomposition_handler::import_ntd;
    use crate::tree_decompositions::tree_structure::Vertex;

    #[test]
    fn test_dpddata() {

        let from_graph = import_metis("data/metis_graphs/from_5.graph").unwrap();
        let to_graph = import_metis("data/metis_graphs/to_3.graph").unwrap();
        let ntd = import_ntd("data/nice_tree_decompositions/example_3.ntd").unwrap();

        let mut dp_data = diaz::diaz_algorithm::DPData::new(&from_graph, &to_graph, &ntd);

        // test empty table
        assert_eq!(dp_data.get(&4, &10) , None);
        assert_eq!(dp_data.get(&9, &3) , None);

        // try to set the values
        dp_data.set(4, 10, 5);
        dp_data.set(9,3,2);

        // check values again
        assert_eq!(dp_data.get(&4, &10) , Some(&5));
        assert_eq!(dp_data.get(&9, &3) , Some(&2));

        // Check table_apply
        assert_eq!(dp_data.table_apply(30,1), 3);
        assert_eq!(dp_data.table_apply(28,0), 0);

        // Check table_extend
        assert_eq!(dp_data.table_extend(15, 1, 2), 59);
        assert_eq!(dp_data.table_extend(0,2,3), 48);

        // Check table_reduce
        assert_eq!(dp_data.table_reduce(59,0), 14);
        assert_eq!(dp_data.table_reduce(15,1), 3);

        // Check max_bag_mappings
        assert_eq!(dp_data.max_bag_mappings(16), 64);
        assert_eq!(dp_data.max_bag_mappings(0), 4);
        assert_eq!(dp_data.max_bag_mappings(5), 16);

        // check sorted bags
        assert_eq!(*dp_data.sorted_bag(8).unwrap(), vec![Vertex::new(0),Vertex::new(2),Vertex::new(3)]);
        assert_eq!(*dp_data.sorted_bag(16).unwrap(), vec![Vertex::new(0),Vertex::new(2),Vertex::new(3)]);
        assert_eq!(*dp_data.sorted_bag(7).unwrap(), vec![Vertex::new(0),Vertex::new(2),Vertex::new(3)]);
        assert_eq!(*dp_data.sorted_bag(11).unwrap(), vec![Vertex::new(0),Vertex::new(3)]);
        assert_eq!(*dp_data.sorted_bag(2).unwrap(), vec![Vertex::new(2)]);

        assert_eq!(*dp_data.sorted_bag(8).unwrap(), vec![Vertex::new(0),Vertex::new(2),Vertex::new(3)]);
        assert_eq!(*dp_data.sorted_bag(16).unwrap(), vec![Vertex::new(0),Vertex::new(2),Vertex::new(3)]);
        assert_eq!(*dp_data.sorted_bag(7).unwrap(), vec![Vertex::new(0),Vertex::new(2),Vertex::new(3)]);
        assert_eq!(*dp_data.sorted_bag(11).unwrap(), vec![Vertex::new(0),Vertex::new(3)]);
        assert_eq!(*dp_data.sorted_bag(2).unwrap(), vec![Vertex::new(2)]);


        // todo : Add test for edge to index and index to edge
        // /// Given the index of an edge this functions returns the edge as a tuple
        //         pub fn index_to_edge(&self, index : &usize) -> Option<&(usize, usize)> { self.index_to_edge.get(index) }
        //
        //         /// Given a specific edge as a tuple, return the index of this edge.
        //         pub fn edge_to_index(&self, edge : &(usize,usize)) -> Option<&usize> { self.edge_to_index.get(edge) }
        //
        //         /// Returns the vector of all possible edges.
        //         pub fn all_possible_edges(&self) -> &Vec<(usize, usize)> { &self.all_possible_edges }



    }

    #[test]
    fn test_diaz(){

        let from_graph = import_metis("data/metis_graphs/from_2.graph").unwrap();
        let to_graph = import_metis("data/metis_graphs/to_2.graph").unwrap();
        let ntd = import_ntd("data/nice_tree_decompositions/example_2.ntd").unwrap();
        let i = diaz::diaz_algorithm::diaz(&from_graph, &ntd, &to_graph);
        assert_eq!(i,1280);

        let from_graph = import_metis("data/metis_graphs/from_3.graph").unwrap();
        let to_graph = import_metis("data/metis_graphs/to_3.graph").unwrap();
        let ntd = import_ntd("data/nice_tree_decompositions/example_2.ntd").unwrap();
        let i = diaz::diaz_algorithm::diaz(&from_graph, &ntd, &to_graph);
        assert_eq!(i,256);

        let from_graph = import_metis("data/metis_graphs/from_4.graph").unwrap();
        let to_graph = import_metis("data/metis_graphs/to_4.graph").unwrap();
        let ntd = import_ntd("data/nice_tree_decompositions/example_3.ntd").unwrap();
        let i = diaz::diaz_algorithm::diaz(&from_graph, &ntd, &to_graph);
        assert_eq!(i,0);

        let from_graph = import_metis("data/metis_graphs/from_5.graph").unwrap();
        let to_graph = import_metis("data/metis_graphs/to_4.graph").unwrap();
        let ntd = import_ntd("data/nice_tree_decompositions/example_3.ntd").unwrap();
        let i = diaz::diaz_algorithm::diaz(&from_graph, &ntd, &to_graph);
        assert_eq!(i,0);

        let from_graph = import_metis("data/metis_graphs/from_6.graph").unwrap();
        let to_graph = import_metis("data/metis_graphs/to_4.graph").unwrap();
        let ntd = import_ntd("data/nice_tree_decompositions/example_3.ntd").unwrap();
        let i = diaz::diaz_algorithm::diaz(&from_graph, &ntd, &to_graph);
        assert_eq!(i,0);

        let from_graph = import_metis("data/metis_graphs/from_7.graph").unwrap();
        let to_graph = import_metis("data/metis_graphs/to_2.graph").unwrap();
        let ntd = import_ntd("data/nice_tree_decompositions/ntd_4.ntd").unwrap();
        let i = diaz::diaz_algorithm::diaz(&from_graph, &ntd, &to_graph);
        assert_eq!(i,960);

    }
}

#[cfg(test)]
pub mod graph_generation_test{
    use std::fmt::format;
    use petgraph::dot::Dot;
    use petgraph::visit::GetAdjacencyMatrix;
    use crate::file_handler::graph_handler::import_metis;
    use crate::file_handler::tree_decomposition_handler::import_ntd;
    use crate::graph_generation::graph_generation::{equal_graphs, generate_graphs, generate_possible_edges};
    use crate::unit_tests::compare_edge_lists;

    #[test]
    fn test_generate_possible_edges()
    {
        let ntd = import_ntd("data/nice_tree_decompositions/example_2.ntd").unwrap();
        let possible_edge_hash = generate_possible_edges(&ntd);

        assert!(compare_edge_lists(possible_edge_hash.get(&1).unwrap() , &vec![(4,2), (2,2), (4,4)] ));
        assert!(compare_edge_lists(possible_edge_hash.get(&5).unwrap() , &vec![(4,2), (2,2), (4,4), (1,2), (1,1)] ));
        assert!(compare_edge_lists(possible_edge_hash.get(&7).unwrap() , &vec![(0,0)] ));
        assert!(compare_edge_lists(possible_edge_hash.get(&8).unwrap() , &vec![(0,0),(1,1),(0,1)] ));
        assert!(compare_edge_lists(possible_edge_hash.get(&10).unwrap() , &vec![(0,0),(1,1),(0,1), (4,2), (2,2), (4,4), (1,2)] ));
        assert!(compare_edge_lists(possible_edge_hash.get(&13).unwrap() , &vec![(0,0),(1,1),(0,1), (4,2), (2,2), (4,4), (1,2), (1,3), (3,3)] ));

    }

    #[test]
    fn test_generate_graphs()
    {
        let gen_graphs = generate_graphs(4, vec![(0,1),(0,3),(0,2),(2,3)]);
        let mut import_graphs = vec![];

        // import all graphs
        for i in 1..17{
            let source = format!("data/metis_graphs/graph_generation_test/gen_{}.graph",i);
            import_graphs.push(import_metis(source).unwrap());
        }

        // check if all imports are in the generated list of graphs
        for g in &import_graphs{
            assert!(gen_graphs.iter().any(|x| {equal_graphs(x,g)}));
        }
    }

    #[test]
    fn test_equal_graphs()
    {
        let graph1 = import_metis("data/metis_graphs/graph_generation_test/gen_1.graph").unwrap();
        let graph2 = import_metis("data/metis_graphs/graph_generation_test/gen_2.graph").unwrap();
        assert!(!equal_graphs(&graph1, &graph2));
        assert!(equal_graphs(&graph1, &graph1));
        assert!(equal_graphs(&graph2, &graph2));
    }


}


#[cfg(test)]
pub mod algorithm_comparison_test{
    use crate::brute_force::brute_force_homomorphism_counter::simple_brute_force;
    use crate::diaz::diaz_algorithm::diaz;
    use crate::file_handler::graph_handler::import_metis;
    use crate::file_handler::tree_decomposition_handler::import_ntd;
    use crate::graph_generation::graph_generation::{generate_graphs, generate_possible_edges};

    #[test]
    fn compare_brute_force_with_diaz()
    {
        let ntd = import_ntd("data/nice_tree_decompositions/example_2.ntd").unwrap();
        let possible_edges = generate_possible_edges(&ntd);

        let all_possible_edges = possible_edges.get(&ntd.root()).unwrap().clone();

        let graphs = generate_graphs(ntd.vertex_count() as u64, all_possible_edges);

        let second_graph = import_metis("data/metis_graphs/to_2.graph").unwrap();

        for g in &graphs{
            assert_eq!(diaz(g,&ntd, &second_graph), simple_brute_force(g, &second_graph));
        }
    }

}

#[cfg(test)]
pub mod equivalence_class_algorithm_test{
    use std::arch::x86_64::_mm256_div_ps;
    use petgraph::dot::Dot;
    use crate::diaz::diaz_algorithm::diaz;
    use crate::equivalence_class_algorithm::equivalence_class_algorithm::{DPData, equivalence_class_algorithm};
    use crate::file_handler::graph_handler::import_metis;
    use crate::file_handler::tree_decomposition_handler::import_ntd;
    use crate::graph_generation::graph_generation::{equal_graphs, generate_graphs, generate_possible_edges};
    use crate::tree_decompositions::tree_structure::Vertex;
    use crate::unit_tests::compare_edge_lists;

    #[test]
    fn test_dpddata() {

        let to_graph = import_metis("data/metis_graphs/to_3.graph").unwrap();
        let ntd = import_ntd("data/nice_tree_decompositions/example_3.ntd").unwrap();

        let mut dp_data = DPData::new(&ntd, &to_graph);

        // test empty table
        assert_eq!(dp_data.get(&4, &5,&10) , None);
        assert_eq!(dp_data.get(&9, &2, &3) , None);

        // try to set the values
        dp_data.set(4, 5, 10, 5);
        dp_data.set(9,2,3, 2);

        // Check values again
        assert_eq!(dp_data.get(&4, &5,&10) , Some(&5));
        assert_eq!(dp_data.get(&9, &2, &3) , Some(&2));

        // Check table_apply
        assert_eq!(dp_data.table_apply(30,1), 3);
        assert_eq!(dp_data.table_apply(28,0), 0);

        // Check table_extend
        assert_eq!(dp_data.table_extend(15, 1, 2), 59);
        assert_eq!(dp_data.table_extend(0,2,3), 48);

        // Check table_reduce
        assert_eq!(dp_data.table_reduce(59,0), 14);
        assert_eq!(dp_data.table_reduce(15,1), 3);

        // Check max_bag_mappings
        assert_eq!(dp_data.max_bag_mappings(16), 64);
        assert_eq!(dp_data.max_bag_mappings(0), 4);
        assert_eq!(dp_data.max_bag_mappings(5), 16);

        // check sorted bags
        assert_eq!(*dp_data.sorted_bag(8).unwrap(), vec![Vertex::new(0),Vertex::new(2),Vertex::new(3)]);
        assert_eq!(*dp_data.sorted_bag(16).unwrap(), vec![Vertex::new(0),Vertex::new(2),Vertex::new(3)]);
        assert_eq!(*dp_data.sorted_bag(7).unwrap(), vec![Vertex::new(0),Vertex::new(2),Vertex::new(3)]);
        assert_eq!(*dp_data.sorted_bag(11).unwrap(), vec![Vertex::new(0),Vertex::new(3)]);
        assert_eq!(*dp_data.sorted_bag(2).unwrap(), vec![Vertex::new(2)]);

        assert_eq!(*dp_data.sorted_bag(8).unwrap(), vec![Vertex::new(0),Vertex::new(2),Vertex::new(3)]);
        assert_eq!(*dp_data.sorted_bag(16).unwrap(), vec![Vertex::new(0),Vertex::new(2),Vertex::new(3)]);
        assert_eq!(*dp_data.sorted_bag(7).unwrap(), vec![Vertex::new(0),Vertex::new(2),Vertex::new(3)]);
        assert_eq!(*dp_data.sorted_bag(11).unwrap(), vec![Vertex::new(0),Vertex::new(3)]);
        assert_eq!(*dp_data.sorted_bag(2).unwrap(), vec![Vertex::new(2)]);

        // continue with testcases for

        assert!(compare_edge_lists(dp_data.all_possible_edges(),
                                   &vec![(0,0), (1,1), (2,2), (3,3), (4,4), (0,1), (1,3), (0,3), (0,2), (2,3), (0,4), (3,4)]));
        assert!(!compare_edge_lists(dp_data.all_possible_edges(),
                                   &vec![(0,0), (1,1), (2,2), (3,3), (4,4), (0,1), (1,3), (0,3), (0,2), (2,3), (0,4)]));

        // test for index to edge
        assert_eq!(dp_data.index_to_edge(&3), dp_data.all_possible_edges().get(3));
        assert_eq!(dp_data.index_to_edge(&4), dp_data.all_possible_edges().get(4));
        assert_eq!(dp_data.index_to_edge(&6), dp_data.all_possible_edges().get(6));
        assert_ne!(dp_data.index_to_edge(&2), dp_data.all_possible_edges().get(3));
        assert_ne!(dp_data.index_to_edge(&3), dp_data.all_possible_edges().get(4));

        // test for edge to index
        assert_eq!(*dp_data.edge_to_index(&(0 as usize,0 as usize)).unwrap(),
                   dp_data.all_possible_edges().iter().position(|x| *x == (0,0)).unwrap());

        assert_eq!(*dp_data.edge_to_index(&(2 as usize,3 as usize)).unwrap(),
                   dp_data.all_possible_edges().iter().position(|x| *x == (2,3) || *x == (3,2)).unwrap());


        // test the possible_edges function
        let pos_edges = dp_data.possible_edges(7).unwrap();
        let edges : Vec<(usize, usize)> = pos_edges.iter().map(|x| *dp_data.index_to_edge(x).unwrap()).collect();
        assert!(compare_edge_lists(&vec![(0,0), (2,2), (3,3), (0,2), (0,3), (2,3)], &edges));


        let pos_edges = dp_data.possible_edges(14).unwrap();
        let edges : Vec<(usize, usize)> = pos_edges.iter().map(|x| *dp_data.index_to_edge(x).unwrap()).collect();
        assert!(compare_edge_lists(&vec![(0,0), (1,1), (2,2), (3,3), (4,4), (0,1), (1,3), (0,3), (0,2), (2,3), (0,4), (3,4)], &edges));


        // test edges_to_integer_representation
        // 2^0 + 2^4 + 2^7 + 2^1 + 2^2 = 1 + 16 + 128 + 2 + 4 = 151
        let edges = vec![0,4,7,1,2];
        assert_eq!(dp_data.edges_to_integer_representation(&edges), 151);

        // 2^0 = 1
        let edges = vec![0];
        assert_eq!(dp_data.edges_to_integer_representation(&edges), 1);

        // no edge
        let edges = vec![];
        assert_eq!(dp_data.edges_to_integer_representation(&edges), 0);


        // test the intersection
        // a = [0,2,3] -> 2^0 + 2^2 + 2^3 = 1 + 4 + 8 = 13
        // b = [0,3,5] -> 2^0 + 2^3 + 2^5 = 1 + 8 + 32 = 41
        // intersection = [0, 3] -> 2^0 + 2^3 = 1 + 8 = 9
        assert_eq!(dp_data.intersection(13,41), 9);

        // test edges_to_graph()
        let mut edges = vec![];
        edges.push(*dp_data.edge_to_index(&(0,0)).unwrap());
        edges.push(*dp_data.edge_to_index(&(0,1)).unwrap());
        edges.push(*dp_data.edge_to_index(&(4,3)).unwrap());

        let edges_integer = dp_data.edges_to_integer_representation(&edges);
        let graph = dp_data.edges_to_graph(edges_integer);

        let imported_reference = import_metis("data/metis_graphs/equivalence_class_algorithm_tests/test_edges_to_graph.graph").unwrap();
        assert!(equal_graphs(&graph, &imported_reference));

    }

    #[test]
    fn test_equivalence_class_algorithm()
    {
        let ntd = import_ntd("data/nice_tree_decompositions/example_3.ntd").unwrap();
        let to_graph = import_metis("data/metis_graphs/to_3.graph").unwrap();

        let graphs_hom = equivalence_class_algorithm(&ntd, &to_graph);

        let graphs = generate_graphs(ntd.vertex_count() as u64, generate_possible_edges(&ntd).get(&ntd.root()).unwrap().clone());

        for graph in &graphs{

            let pos = graphs_hom.iter().position( |(g,h)| {equal_graphs(g,graph)} ).unwrap();
            let diaz = diaz(graph, &ntd, &to_graph);

            let (g,h) = graphs_hom.get(pos).unwrap();

            assert_eq!(diaz, *h);

        }

    }
}