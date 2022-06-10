extern crate core;

use std::path::Path;
use Counting_Homomorphisms::brute_force::brute_force_homomorphism_counter::simple_brute_force_for_ntd_set;
use Counting_Homomorphisms::diaz_serna_thilikos::diaz_algorithm::diaz_serna_thilikos_for_ntd_set;
use Counting_Homomorphisms::experiments::single_running_time_measurement::{measure_running_time};
use Counting_Homomorphisms::modified_dp::algorithm::modified_dp;

fn main(){

    // measure single running times
    measure_running_time(Path::new("data/Experiments/experiment_matrices/running_time/brute_force_growth_with_e_tau.csv"),
                         simple_brute_force_for_ntd_set,
                         &"brute_force".to_string());

    measure_running_time(Path::new("data/Experiments/experiment_matrices/running_time/brute_force_growth_with_graph.csv"),
                         simple_brute_force_for_ntd_set,
                         &"brute_force".to_string());

    measure_running_time(Path::new("data/Experiments/experiment_matrices/running_time/diaz_serna_thilikos_growth_with_e_tau.csv"),
                         diaz_serna_thilikos_for_ntd_set,
                         &"diaz_serna_thilikos".to_string());

    measure_running_time(Path::new("data/Experiments/experiment_matrices/running_time/diaz_serna_thilikos_growth_with_graph.csv"),
                         diaz_serna_thilikos_for_ntd_set,
                         &"diaz_serna_thilikos".to_string());

    measure_running_time(Path::new("data/Experiments/experiment_matrices/running_time/modified_dp_growth_with_e_tau.csv"),
                         modified_dp,
                         &"modified_dp".to_string());



    measure_running_time(Path::new("data/Experiments/experiment_matrices/running_time/modified_dp_growth_with_graph.csv"),
                         modified_dp,
                         &"modified_dp".to_string());


    // new measurements

    measure_running_time(Path::new("data/Experiments/experiment_matrices/running_time/mixed_combinations.csv"),
                         diaz_serna_thilikos_for_ntd_set,
                         &"diaz_serna_thilikos".to_string());


    measure_running_time(Path::new("data/Experiments/experiment_matrices/running_time/mixed_combinations.csv"),
                         modified_dp,
                         &"modified_dp".to_string());


    // Comparison

}