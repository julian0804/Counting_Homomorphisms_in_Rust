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
use crate::algorithms::brute_force_homomorphism_counter::simple_brute_force;

fn main(){

    let ntd = file_handler::file_handler::create_ntd_from_file("data/nice_tree_decompositions/example_3.ntd").unwrap();
    let g = file_handler::file_handler::metis_to_graph("data/metis_graphs/to_2.graph").unwrap();


    let start = Instant::now();
    let graphs = generate_graphs(5, generate_edges(ntd.clone()));



    for h in &graphs{


       //println!("h : {:?}", Dot::new(h));

        let diaz = diaz(h,ntd.clone(), &g);
        let brute_force = simple_brute_force(h,&g);

        println!("diaz: {:?}, brute force: {:?}", diaz, brute_force);
        if diaz != brute_force{ println!("wrong");}

    }
    let duration = start.elapsed();
    println!("{:?} homomorphism numbers have been calculated in time {:?}", graphs.len(), duration);
}