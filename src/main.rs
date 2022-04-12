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
use crate::algorithms::diaz::diaz;

fn main(){

    let from_graph = file_handler::file_handler::metis_to_graph("data/metis_graphs/from_2.graph").unwrap();
    let to_graph = file_handler::file_handler::metis_to_graph("data/metis_graphs/to_2.graph").unwrap();
    let ntd = file_handler::file_handler::create_ntd_from_file("data/nice_tree_decompositions/example_2.ntd").unwrap();
    let i = diaz(&from_graph, ntd, &to_graph);
    println!("{:?}", i);

    let from_graph = file_handler::file_handler::metis_to_graph("data/metis_graphs/from_3.graph").unwrap();
    let to_graph = file_handler::file_handler::metis_to_graph("data/metis_graphs/to_3.graph").unwrap();
    let ntd = file_handler::file_handler::create_ntd_from_file("data/nice_tree_decompositions/example_2.ntd").unwrap();
    let i = diaz(&from_graph, ntd, &to_graph);
    println!("{:?}", i);

}