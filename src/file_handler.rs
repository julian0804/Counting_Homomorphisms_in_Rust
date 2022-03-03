use super::lib::*;

pub mod file_handler{

    use std::collections::{HashMap, HashSet};
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;
    use crate::graph_structures::graph_structures::adjacency::AdjList;
    use crate::nice_tree_decomposition::*;



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
     */
    pub fn create_ntd_from_file<P>(filename : P)
        where P : AsRef<Path>
    {
        // These values
        let mut number_of_nodes = 0;
        let mut max_bag_size = 0;
        let mut number_of_vertices = 0;

        let mut temporary_Nodes = HashMap::new();
        let mut adjlist = AdjList::new();

        if let Ok(lines) = read_lines(filename)
        {
            for line in lines {
                if let Ok(ip) = line {
                    // The following code is for evaluating line by line
                    let mut args = ip.split(" ");
                    let tag = args.next();

                    match tag{
                        None => {break;},
                        Some("s") => {
                            number_of_nodes = args.next().unwrap().parse::<i32>().unwrap();
                            max_bag_size = args.next().unwrap().parse::<i32>().unwrap();
                            number_of_vertices = args.next().unwrap().parse::<i32>().unwrap();
                        },
                        Some("n") => {
                            let nr = args.next().unwrap().parse::<i32>().unwrap();
                            let node_type = args.next();

                            // TODO: create closure for the loop in the following
                            match node_type {
                                None => {todo!("return Err")},
                                Some("l") => {
                                    let mut bag = Bag::from([]);
                                    loop{
                                         if let Some(v) = args.next(){
                                             bag.insert(v.parse::<Vertex>().unwrap());
                                         }
                                         else {
                                             break;
                                         }
                                    }
                                    temporary_Nodes.insert(nr,TemporaryNodeType::Leaf(bag));
                                },
                                Some("i") => {
                                    let mut bag = Bag::from([]);
                                    loop{
                                        if let Some(v) = args.next(){
                                            bag.insert(v.parse::<Vertex>().unwrap());
                                        }
                                        else {
                                            break;
                                        }
                                    }
                                    temporary_Nodes.insert(nr,TemporaryNodeType::Introduce(bag));
                                },
                                Some("f") => {
                                    let mut bag = Bag::from([]);
                                    loop {
                                        if let Some(v) = args.next(){
                                            bag.insert(v.parse::<Vertex>().unwrap());
                                        }
                                        else {
                                            break;
                                        }
                                    }
                                    temporary_Nodes.insert(nr,TemporaryNodeType::Forget(bag));
                                },
                                Some("j") => {
                                    let mut bag = Bag::from([]);
                                    loop {
                                        if let Some(v) = args.next(){
                                            bag.insert(v.parse::<Vertex>().unwrap());
                                        }
                                        else {
                                            break;
                                        }
                                    }
                                    temporary_Nodes.insert(nr,TemporaryNodeType::Join(bag));
                                },
                                _ => {todo!("return Err")}
                            }
                        },
                        Some("a") => {
                            let from = args.next().unwrap().parse::<Vertex>().unwrap();
                            let to = args.next().unwrap().parse::<Vertex>().unwrap();
                            adjlist.insert_edge(from,to);
                        },
                        _ => { todo!("return Err") },
                    }

                }
            }

        }
        println!("number of nodes = {:?}, max bag size = {:?}, number of vertices = {:?}",
                 number_of_nodes,
                 max_bag_size,
                 number_of_vertices);
        println!("{:?}", temporary_Nodes);
        println!("{:?}", adjlist);

        /*
        todo: make code more beautiful...

        todo: implement some form of post order to implement construction
         */
    }

}
