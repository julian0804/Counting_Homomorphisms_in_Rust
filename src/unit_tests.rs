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
pub mod nice_tree_decomposition_test{
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
pub mod tree_decomposition_handler_test{
    use crate::file_handler::tree_decomposition_handler::import_ntd;
    use crate::unit_tests::ntd_test_example;

    #[test]
    pub fn test_ntd_import() {
        let ntd = ntd_test_example();
        assert_eq!(import_ntd("data/nice_tree_decompositions/example.ntd").unwrap(), ntd);
    }
}

#[cfg(test)]
pub mod graph_handler_test{
    use crate::file_handler::graph_handler::import_metis;
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
}