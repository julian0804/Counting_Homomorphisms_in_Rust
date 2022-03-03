mod lib;
mod file_handler;
mod graph_structures;

use lib::nice_tree_decomposition;
use crate::file_handler::file_handler::*;

fn main(){

    create_ntd_from_file("example.ntd");
}