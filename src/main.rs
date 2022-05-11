extern crate core;

use std::time::Instant;
use Counting_Homomorphisms::diaz::diaz_algorithm::diaz;
use Counting_Homomorphisms::equivalence_class_algorithm::equivalence_class_algorithm::equivalence_class_algorithm;
use Counting_Homomorphisms::file_handler::graph_handler::import_metis;
use Counting_Homomorphisms::file_handler::tree_decomposition_handler::import_ntd;
use Counting_Homomorphisms::graph_generation::graph_generation::{generate_graphs, generate_possible_edges};

fn main(){

    let ntd = import_ntd("data/nice_tree_decompositions/ntd_k_5.ntd").unwrap();
    let to_graph = import_metis("data/metis_graphs/to_2.graph").unwrap();


    let start = Instant::now();

    let possible_edges = generate_possible_edges(&ntd).get(&ntd.root()).unwrap().clone();

    println!("### Parameters ###");
    println!("|V(H)| : {:?}", ntd.vertex_count());
    println!("number of possible edges : {:?}", possible_edges.len());
    println!("number of graphs : {:?}", 2_u32.pow(possible_edges.len() as u32));
    println!("G Info: ");
    println!("|V(G)| = {:?}  |E(G)| = {:?}", to_graph.node_count(), to_graph.edge_count());

    let graphs = generate_graphs(ntd.vertex_count() as u64, possible_edges);

    let duration = start.elapsed();
    println!("Time elapsed for graph generation {:?}", duration);

    let start = Instant::now();
    for graph in &graphs{
        let diaz = diaz(graph, &ntd, &to_graph);
    }
    let duration = start.elapsed();
    println!("Time elapsed for diaz {:?}", duration);


    let start = Instant::now();
    let graph_hom_values = equivalence_class_algorithm(&ntd, &to_graph);
    let duration = start.elapsed();
    println!("Time elapsed for equivalence class algorithm {:?}", duration);



}