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
use crate::algorithms::diaz::{diaz, generate_edges, generate_graphs};
use std::time::{Duration, Instant};

fn main(){

    let ntd = file_handler::file_handler::create_ntd_from_file("data/nice_tree_decompositions/example_3.ntd").unwrap();
    let g = file_handler::file_handler::metis_to_graph("data/metis_graphs/to_2.graph").unwrap();


    let start = Instant::now();
    let graphs = generate_graphs(generate_edges(ntd.clone()));


    for h in &graphs{
        //println!("{:?}", Dot::new(g));
        println!("{:?}", diaz(h,ntd.clone(), &g));
    }
    let duration = start.elapsed();
    println!("{:?} homomorphism numbers have been calculated in time {:?}", graphs.len(), duration);
}