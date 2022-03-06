use super::lib::*;

pub mod file_handler {
    use std::collections::{HashMap, HashSet};
    use std::error::Error;
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;
    use crate::graph_structures::graph_structures::adjacency::*;
    use crate::graph_structures::graph_structures::nice_tree_decomposition::*;
    use crate::graph_structures::graph_structures::{Vertex, VertexBag};

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
    pub fn create_ntd_from_file<P>(filename: P) -> Option<NiceTreeDecomposition>
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
                            Some("l") => { node_data.insert(nr, NodeType::Leaf(constructed_bag())); },
                            Some("i") => { node_data.insert(nr, NodeType::Introduce(constructed_bag())); },
                            Some("f") => { node_data.insert(nr, NodeType::Forget(constructed_bag())); },
                            Some("j") => { node_data.insert(nr, NodeType::Join(constructed_bag())); },
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

            println!("number of nodes = {:?}, max bag size = {:?}, number of vertices = {:?}",
                     number_of_nodes,
                     max_bag_size,
                     number_of_vertices);
            println!("{:?}", node_data);
            println!("{:?}", adjacency_list);

            Some(NiceTreeDecomposition::new(adjacency_list, node_data,0))
        }
        else { None }


    }
}


