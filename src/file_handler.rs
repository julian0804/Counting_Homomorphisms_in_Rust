use super::lib::*;


pub mod file_handler {
    use std::collections::{HashMap, HashSet};
    use std::error::Error;
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;
    use petgraph::graph::NodeIndex;
    use crate::graph_structures::graph_structures::adjacency::*;
    use crate::graph_structures::graph_structures::nice_tree_decomposition::*;
    use crate::graph_structures::graph_structures::{Vertex, VertexBag};


    use petgraph::matrix_graph::*;
    use petgraph::Undirected;

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
        // These values
        let mut number_of_nodes = 0;
        let mut max_bag_size = 0;
        let mut number_of_vertices = 0;

        let mut node_data: HashMap<Vertex, NodeType> = HashMap::new();
        let mut adjacency_list = AdjList::new();

        if let Ok(lines) = read_lines(filename){
            for line in lines {
                let i = line.unwrap();
                let mut args = i.split(" ");
                let type_arg = args.next();

                match type_arg {
                    Some("s") => {
                        number_of_nodes = args.next().unwrap().parse::<i32>().unwrap();
                        max_bag_size = args.next().unwrap().parse::<i32>().unwrap();
                        number_of_vertices = args.next().unwrap().parse::<i32>().unwrap();
                    },
                    Some("n") => {
                        let nr = args.next().unwrap().parse::<u32>().unwrap();
                        let node_type = args.next();

                        let mut constructed_bag = || {
                            let mut bag = VertexBag::from([]);
                            loop {
                                if let Some(v) = args.next() {
                                    bag.insert(v.parse::<Vertex>().unwrap());
                                } else {
                                    break;
                                }
                            }
                            bag
                        };

                        match node_type {
                            Some("l") => { node_data.insert(nr.into(), NodeType::Leaf(constructed_bag())); },
                            Some("i") => { node_data.insert(nr.into(), NodeType::Introduce(constructed_bag())); },
                            Some("f") => { node_data.insert(nr.into(), NodeType::Forget(constructed_bag())); },
                            Some("j") => { node_data.insert(nr.into(), NodeType::Join(constructed_bag())); },
                            _ => {}
                        }
                    },
                    Some("a") => {
                        let from = args.next().unwrap().parse::<Vertex>().unwrap();
                        let to = args.next().unwrap().parse::<Vertex>().unwrap();
                        adjacency_list.insert_edge(from,to);
                    }
                    _ => {}
                }
            }

            // naive root finding
            let mut root : Vertex = 1;
            while let Some(i) = adjacency_list.in_neighbours(root){
                root = *i.get(0).unwrap();
            }

            Some(NiceTreeDecomposition::new(adjacency_list, node_data, root))


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

        let mut number_of_vertices : Vertex = 0;
        let mut number_of_edges : Vertex = 0;
        let mut current_vertex : usize = 0;


        if let Ok(lines) = read_lines(filename) {
            for line in lines {
                let content = line.unwrap();

                // % means comment -> ignore
                if content.chars().next().unwrap() == '%' {continue; }
                let mut args = content.split(" ");

                if number_of_vertices == 0 {
                    number_of_vertices = args.next().unwrap().parse::<Vertex>().unwrap();
                    number_of_edges = args.next().unwrap().parse::<Vertex>().unwrap();

                    for i in 1..(number_of_vertices + 1){
                        let j = graph.add_node(());
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
    use crate::{create_ntd_from_file, file_handler, metis_to_graph};
    use crate::graph_structures::graph_structures::adjacency::AdjList;
    use crate::graph_structures::graph_structures::{Vertex, VertexBag};
    use crate::graph_structures::graph_structures::nice_tree_decomposition::{NiceTreeDecomposition, NodeType};
    use crate::graph_structures::graph_structures::nice_tree_decomposition::NodeType::{Forget, Introduce, Join, Leaf};

    #[test]
    fn test_ntd_creation_from_file(){
        let mut example_adjacency_list = AdjList::new();
        example_adjacency_list.insert_edges(vec![
            (10,9), (6,5), (9,8), (8,7), (7,3), (7,6), (5,4), (2,1), (3,2)
        ]);
        let example_node_data: HashMap<Vertex, NodeType> = HashMap::from([
            (1 , Leaf(VertexBag::from([1]))),
            (2 , Introduce(VertexBag::from([1,2]))),
            (3 , Forget(VertexBag::from([2]))),
            (4 , Leaf(VertexBag::from([2]))),
            (5 , Introduce(VertexBag::from([2,3]))),
            (6 , Forget(VertexBag::from([2]))),
            (7 , Join(VertexBag::from([2]))),
            (8 , Introduce(VertexBag::from([2,4]))),
            (9 , Forget(VertexBag::from([4]))),
            (10 , Forget(VertexBag::from([]))),
        ]);

        assert_eq!(create_ntd_from_file("data/nice_tree_decompositions/example.ntd").unwrap(),
                   NiceTreeDecomposition::new(example_adjacency_list, example_node_data, 10));
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

        let g = file_handler::file_handler::metis_to_graph("data/metis_graphs/tiny_01.graph").unwrap();

        assert_eq!(g.node_count(), 7);
        assert_eq!(g.edge_count(), 11);
        for (a,b) in edges{
            assert!(g.has_edge(NodeIndex::new(a), NodeIndex::new(b)));
        }
    }
}

