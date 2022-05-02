use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

/// Reads file with given filename and returns BufReader
/// taken from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// A module containing the import and export functions for .ntd and (eventually .nt) files.
pub mod tree_decomposition_handler {
    use std::collections::HashMap;
    use std::fs::File;
    use std::path::Path;
    use crate::file_handler::read_lines;
    use crate::tree_decompositions::nice_tree_decomposition::{Bag, NiceTreeDecomposition, NodeData, NodeType};
    use crate::tree_decompositions::tree_structure::{TreeNode, TreeStructure, Vertex};

    /// Given a .ntd-file this functions returns a NiceTreeDecomposition if possible.
    pub fn import_ntd<P>(filename : P) -> Option<NiceTreeDecomposition>
        where P: AsRef<Path>
    {
        // Info given by the import format
        let mut number_of_nodes = 0;

        // This argument will not be used in the following function
        let mut max_bag_size = 0;

        // This information is not needed yet
        let mut number_of_vertices = 0;

        // create an dummy tree structure to late override it
        let mut tree_structure : TreeStructure = TreeStructure::new(1);

        // creat an empty hashmap saving the node_data
        let mut nodes_data : HashMap<TreeNode, NodeData> = HashMap::new();

        // read lines of file if possible
        if let Ok(lines) = read_lines(filename){

            // loop over all written lines in the file
            for line in lines {

                let line_string = line.unwrap();
                // get all args divided by a space
                let mut args = line_string.split(" ");
                // get the first argument, which denotes the function of this line
                let type_arg = args.next();

                // match the first argument of the line
                match type_arg {
                    // s is the start line, containing info about the nice tree decomposition
                    Some("s") => {

                        // get the arguments contained in the start line
                        number_of_nodes = args.next().unwrap().parse::<u64>().unwrap();
                        max_bag_size = args.next().unwrap().parse::<u32>().unwrap();
                        number_of_vertices = args.next().unwrap().parse::<u32>().unwrap();

                        // Create the tree structure when info has been found
                        tree_structure = TreeStructure::new(number_of_nodes);
                    },
                    // Manages node lines, which represent the node data
                    Some("n") => {

                        /*
                        The index of the node will be reduced by one since the internal
                        representation of node goes from 0 to N-1 while the nodes in the .ntd
                        files have indices 1..N.
                         */
                        let node_index = (args.next().unwrap().parse::<u32>().unwrap() - 1) as TreeNode;

                        // get the type of node
                        let node_type = args.next();

                        // This closure is used to construct the bag out of the following arguments
                        let mut constructed_bag = || {
                            let mut bag = Bag::new();
                            loop {
                                if let Some(v) = args.next() {
                                    bag.insert(Vertex::new((v.parse::<u64>().unwrap() - 1) as usize) );
                                } else {
                                    break;
                                }
                            }
                            bag
                        };

                        // construct node data from the information given
                        let node_data = match node_type {
                            Some("l") => NodeData::new(NodeType::Leaf, constructed_bag()),
                            Some("i") => NodeData::new(NodeType::Introduce, constructed_bag()),
                            Some("f") => NodeData::new(NodeType::Forget, constructed_bag()),
                            Some("j") => NodeData::new(NodeType::Join, constructed_bag()),
                            _ => {panic!("cannot identify this node type");} // This case should never happen
                        };

                        // inserts node data into the nodes_data hashmap.
                        nodes_data.insert(node_index, node_data);


                    },
                    // Manages adjacency lines
                    Some("a") => {
                        let p = (args.next().unwrap().parse::<TreeNode>().unwrap() - 1) as TreeNode;
                        let q = (args.next().unwrap().parse::<TreeNode>().unwrap() - 1) as TreeNode;
                        tree_structure.add_child(p, q);
                    }
                    _ => {}
                }
            }
            Some(NiceTreeDecomposition::new(tree_structure, nodes_data, number_of_vertices,max_bag_size - 1 ))
        }
        else { None }


    }


}

/// A module containing the import and export functions for several graph formats
pub mod graph_handler {
    use std::path::Path;
    use petgraph::matrix_graph::NodeIndex;
    use petgraph::Undirected;
    use crate::file_handler::read_lines;
    use crate::tree_decompositions::tree_structure::Vertex;

    /// Given a graph file f, import this graph as a Petgraph Matrix_Graph.
    /// Node-Indies will be subtracted by one (1,..,N) -> (0,..,N-1)
    /// More information on Metis could be found under https://www.lrz.de/services/software/mathematik/metis/metis_5_0.pdf
    pub fn import_metis<P>(filename : P) -> Option<petgraph::matrix_graph::MatrixGraph<(),(), Undirected>>
        where P: AsRef<Path>
    {
        let mut graph = petgraph::matrix_graph::MatrixGraph::new_undirected();

        let mut number_of_vertices : usize = 0;
        let mut number_of_edges : usize = 0;
        let mut current_vertex : usize = 0;

        if let Ok(lines) = read_lines(filename) {

            // go through each line of the file
            for line in lines {
                let content = line.unwrap();

                // % means comment -> ignore
                // empty lines are vertices without out-going edges
                match content.chars().next() {
                    Some('%') => {continue;}
                    None => {
                        current_vertex += 1;
                        continue;
                    }
                    Some(_) => {}
                }

                // separate entries by space
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
                        if !graph.has_edge(Vertex::new(current_vertex), Vertex::new(value - 1)) {
                            graph.add_edge(Vertex::new(current_vertex), Vertex::new(value - 1), ());
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

