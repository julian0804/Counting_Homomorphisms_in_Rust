mod lib;
mod file_handler;
mod graph_structures;
mod algorithms;

use crate::file_handler::file_handler::*;

fn main(){

    let i = create_ntd_from_file("example.ntd").unwrap();
    println!(i);
}