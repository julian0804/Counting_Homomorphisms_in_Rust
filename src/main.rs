extern crate core;

use petgraph::dot::Dot;
/*

mod lib;
mod file_handler;
mod graph_structures;
mod algorithms;

use std::collections::HashSet;
use petgraph::dot::{Dot, Config};
use petgraph::graph::NodeIndex;
use petgraph::matrix_graph::*;
use petgraph::matrix_graph::MatrixGraph;
use crate::algorithms::diaz::{diaz};
use std::time::{Duration, Instant};
use crate::algorithms::brute_force_homomorphism_counter::simple_brute_force;
use crate::algorithms::first_approach::first_approach;
use crate::algorithms::generation::{generate_edges, generate_graphs};
*/

fn main(){

/*

   let ntd = file_handler::file_handler::create_ntd_from_file("data/nice_tree_decompositions/ntd_4.ntd").unwrap();
   let g = file_handler::file_handler::metis_to_graph("data/metis_graphs/to_2.graph").unwrap();

   let list = first_approach(&ntd, &g);



   let start = Instant::now();
   let graphs = generate_graphs(5, generate_edges(ntd.clone()));
   let duration = start.elapsed();
   println!("time needed for graph generation: {:?}", duration);

   let start = Instant::now();
   for h in &graphs{
       let brute_force = simple_brute_force(h,&g);
   }
   let duration = start.elapsed();
   println!("{:?} homomorphism numbers have been calculated with brute force algorithm in time {:?}", graphs.len(), duration);

   let start = Instant::now();
   for h in &graphs{
       let diaz = diaz(h,ntd.clone(), &g);
   }
   let duration = start.elapsed();
   println!("{:?} homomorphism numbers have been calculated with diaz algorithm in time {:?}", graphs.len(), duration);

 */
}