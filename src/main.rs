mod lib;
mod file_handler;
mod graph_structures;
mod algorithms;

use crate::file_handler::file_handler::*;
use crate::graph_structures::graph_structures::graph::SimpleGraph;
use crate::graph_structures::graph_structures::nice_tree_decomposition::NiceTreeDecomposition;

fn main(){
    let ntd = file_handler::file_handler::create_ntd_from_file("example_2.ntd").unwrap();
    println!("{:?}", ntd.stingy_ordering());
}