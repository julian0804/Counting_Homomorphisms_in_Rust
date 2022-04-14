extern crate core;

mod lib;
mod file_handler;
mod graph_structures;
mod algorithms;

use std::collections::HashSet;
use petgraph::dot::{Dot, Config};
use petgraph::graph::NodeIndex;
use petgraph::matrix_graph::*;
use petgraph::matrix_graph::MatrixGraph;
use crate::algorithms::diaz::{diaz, generate_edges};

fn main(){

    let ntd = file_handler::file_handler::create_ntd_from_file("data/nice_tree_decompositions/example_2.ntd").unwrap();
    println!("{:?}", generate_edges(ntd));

}