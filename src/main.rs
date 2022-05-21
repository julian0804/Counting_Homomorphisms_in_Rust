extern crate core;

use std::path::Path;
use Counting_Homomorphisms::experiments::single_running_time_measurement::{run_experiment};

fn main(){
    run_experiment(Path::new("data/Experiments/runtime.csv"));
}