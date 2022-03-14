use super::lib::*;

pub mod file_handler {
    use std::collections::{HashMap, HashSet};
    use std::error::Error;
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;
    use petgraph::graph::NodeIndex;
    use petgraph::matrix_graph::*;
    use petgraph::Undirected;
    use crate::graph_structures::graph_structures::nice_tree_decomposition::{Bag, NiceTreeDecomposition, NodeType, TreeNode, TreeStructure, Vertex};
    /*
    Reads file and returns BufReader
    taken from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
     */
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    /*
    creates nice tree decomposition from file
    Does not check if the tree decomposition is correct
    TODO: make code more solid by catching errors of wrong formatting
     */
    pub fn create_ntd_from_file<P>(filename : P) -> Option<NiceTreeDecomposition>
        where P: AsRef<Path>
    {
        // Info given by the import format
        let mut number_of_nodes = 0;
        let mut max_bag_size = 0;
        let mut number_of_vertices = 0;
        
        let mut tree_structure : TreeStructure = TreeStructure::new(1);

        if let Ok(lines) = read_lines(filename){
            for line in lines {
                let i = line.unwrap();
                let mut args = i.split(" ");
                let type_arg = args.next();

                match type_arg {
                    Some("s") => {
                        number_of_nodes = args.next().unwrap().parse::<u64>().unwrap();
                        max_bag_size = args.next().unwrap().parse::<u64>().unwrap();
                        number_of_vertices = args.next().unwrap().parse::<u64>().unwrap();
                        
                        // Create the tree structure
                        tree_structure = TreeStructure::new(number_of_nodes);
                    },
                    Some("n") => {
                        let nr = args.next().unwrap().parse::<u32>().unwrap() - 1;
                        let node_type = args.next();
                        
                        // This closure is used to construct the bag out of the following arguments
                        let mut constructed_bag = || {
                            let mut bag = Bag::new();
                            loop {
                                if let Some(v) = args.next() {
                                    bag.insert(Vertex::new(((v.parse::<u64>().unwrap() - 1) as usize)) );
                                } else {
                                    break;
                                }
                            }
                            bag
                        };

                        match node_type {
                            Some("l") => {
                                tree_structure.set_node_data(nr as TreeNode, NodeType::Leaf, constructed_bag());
                            },
                            Some("i") => {
                                tree_structure.set_node_data(nr as TreeNode, NodeType::Introduce, constructed_bag()); 
                            },
                            Some("f") => {
                                tree_structure.set_node_data(nr as TreeNode, NodeType::Forget, constructed_bag()); 
                            },
                            Some("j") => {
                                tree_structure.set_node_data(nr as TreeNode, NodeType::Join, constructed_bag()); 
                            },
                            _ => {}
                        }
                    },
                    Some("a") => {
                        let parent = args.next().unwrap().parse::<TreeNode>().unwrap() - 1;
                        let child = args.next().unwrap().parse::<TreeNode>().unwrap() - 1;
                        tree_structure.set_child(parent, child);
                    }
                    _ => {}
                }
            }
            Some(NiceTreeDecomposition::new(tree_structure))


        }
        else { None }


    }

    /*
    Should read file (METIS Format) and returns a MatrixGraph of the petgraph package
    - directed graphs with possible loops
    - unweighted

    - Node-Indies will be subtracted by one (1,..,N -> 0,..,N-1)

    TODO: Make it more stable!

    https://www.lrz.de/services/software/mathematik/metis/metis_5_0.pdf
     */
    pub fn metis_to_graph<P>(filename : P) -> Option<petgraph::matrix_graph::MatrixGraph<(),(), Undirected>>
        where P: AsRef<Path>
    {

        let mut graph = petgraph::matrix_graph::MatrixGraph::new_undirected();

        let mut number_of_vertices : usize = 0;
        let mut number_of_edges : usize = 0;
        let mut current_vertex : usize = 0;


        if let Ok(lines) = read_lines(filename) {
            for line in lines {
                let content = line.unwrap();

                // % means comment -> ignore
                if content.chars().next().unwrap() == '%' {continue; }
                let mut args = content.split(" ");

                if number_of_vertices == 0 {
                    number_of_vertices = args.next().unwrap().parse::<usize>().unwrap();
                    number_of_edges = args.next().unwrap().parse::<usize>().unwrap();

                    for _ in 1..(number_of_vertices + 1){
                        graph.add_node(());
                    }
                    continue;
                }

                loop {
                    if let Some(ver) = args.next() {
                        let value = ver.parse::<usize>().unwrap();
                        if !graph.has_edge(NodeIndex::new(current_vertex), NodeIndex::new(value - 1)) {
                            graph.add_edge(NodeIndex::new(current_vertex), NodeIndex::new(value - 1), ());
                        }
                    } else {
                        break;
                    }
                }

                current_vertex += 1;
            }

        }
        Some(graph)
    }

}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use petgraph::graph::NodeIndex;
    use petgraph::matrix_graph::MatrixGraph;
    use crate::file_handler::file_handler::{create_ntd_from_file, metis_to_graph};
    use crate::graph_structures::graph_structures::nice_tree_decomposition::{Bag, NiceTreeDecomposition, NodeType, TreeStructure, Vertex};
    use crate::graph_structures::graph_structures::nice_tree_decomposition::NodeType::{Forget, Introduce, Join, Leaf};

    fn tree_adjacency_example_one() -> TreeStructure {
        let mut ta = TreeStructure::new(10);
        ta.set_node_data(0, Leaf, Bag::from([Vertex::new(0)]));
        ta.set_node_data(1,
                         Introduce, Bag::from([Vertex::new(0), Vertex::new(1)]));
        ta.set_node_data(2,
                         Forget, Bag::from([Vertex::new(1)]));
        ta.set_node_data(3, Leaf, Bag::from([Vertex::new(1)]));
        ta.set_node_data(4, Introduce, Bag::from([Vertex::new(1), Vertex::new(2)]));
        ta.set_node_data(5,Forget, Bag::from([Vertex::new(1)]));
        ta.set_node_data(6, Join, Bag::from([Vertex::new(1)]));
        ta.set_node_data(7, Introduce, Bag::from([Vertex::new(1), Vertex::new(3)]));
        ta.set_node_data(8, Forget, Bag::from([Vertex::new(3)]));
        ta.set_node_data(9, Forget, Bag::from([]));

        ta.set_child(9,8);
        ta.set_child(8,7);
        ta.set_child(7,6);
        ta.set_child(6,2);
        ta.set_child(2,1);
        ta.set_child(1,0);
        ta.set_child(6,5);
        ta.set_child(5,4);
        ta.set_child(4,3);

        ta
    }

    #[test]
    fn test_ntd_creation_from_file(){

        let example_tree_structure = tree_adjacency_example_one();
        let example_ntd = NiceTreeDecomposition::new(example_tree_structure);

        assert_eq!(create_ntd_from_file("data/nice_tree_decompositions/example.ntd").unwrap(), example_ntd);
    }

    
    #[test]
    fn test_METIS_to_graph(){

        let edges = vec![
            (0, 4), (0, 2), (0, 1),
            (1, 0), (1, 2), (1, 3),
            (2, 4), (2, 3), (2, 1), (2, 0),
            (3, 1), (3, 2), (3, 5), (3, 6),
            (4, 0), (4, 2), (4, 5),
            (5, 4), (5, 3), (5, 6),
            (6, 5), (6, 3)];

        let g = metis_to_graph("data/metis_graphs/tiny_01.graph").unwrap();

        assert_eq!(g.node_count(), 7);
        assert_eq!(g.edge_count(), 11);
        for (a,b) in edges{
            assert!(g.has_edge(NodeIndex::new(a), NodeIndex::new(b)));
        }
    }
}
