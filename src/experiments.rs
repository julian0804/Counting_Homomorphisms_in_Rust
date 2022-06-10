
pub mod single_running_time_measurement {
    use std::fs;
    use std::fs::{OpenOptions, ReadDir};
    use std::ops::Add;
    use std::path::Path;
    use std::time::{Duration, Instant};
    use csv;
    use itertools::Itertools;
    use petgraph::matrix_graph::MatrixGraph;
    use petgraph::Undirected;
    use crate::brute_force::brute_force_homomorphism_counter::simple_brute_force_for_ntd_set;
    use crate::diaz_serna_thilikos::diaz_algorithm::diaz_serna_thilikos_for_ntd_set;
    use crate::modified_dp::algorithm::modified_dp;
    use crate::file_handler::graph_handler::import_metis;
    use crate::file_handler::tree_decomposition_handler::import_ntd;
    use crate::graph_generation::graph_generation_algorithms::generate_possible_edges;
    use crate::tree_decompositions::nice_tree_decomposition::NiceTreeDecomposition;

    const RESULT_PATH: &str = "./target/experiment_results/";
    const NTD_PATH: &str = "data/Experiments/ntds/";
    const GRAPH_PATH: &str = "data/Experiments/graphs/";

    /// lists necessary information of the tree decomposition and write them into a csv file
    pub fn list_ntd_data() {

        // Construct file path of output file
        let result_path = "./target/benchmark_results/";
        let filepath = format!("{}ntd_data.csv", result_path);
        let filepath = Path::new(&filepath);

        for ntd_path in fs::read_dir("./data/nice_tree_decompositions/benchmark_ntds/final_selection").unwrap() {
            let ntd_name = ntd_path.as_ref().unwrap().file_name();

            println!("file: {:?}", ntd_name);

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(filepath)
                .unwrap();

            let mut wtr = csv::Writer::from_writer(file);


            let ntd = import_ntd(ntd_path.as_ref().unwrap().path()).unwrap();

            let width = ntd.width();
            let v_t = ntd.node_count();
            let e_tau = generate_possible_edges(&ntd).get(&ntd.root()).unwrap().len();
            let v_tau = ntd.vertex_count();

            wtr.write_record(&["DATA",
                &ntd_name.to_str().unwrap(),
                &width.to_string(),
                &v_t.to_string(),
                &e_tau.to_string(),
                &v_tau.to_string()]);
        }
    }

    /// This methods executes the experiment given by matrix_path with the algorithm alg and the name alg_name
    pub fn measure_running_time(matrix_file : &Path, alg : fn(&NiceTreeDecomposition, &MatrixGraph<(), (), Undirected>) -> Vec<(MatrixGraph<(), (), Undirected>, u64)>, alg_name : &String){

        let test_name = matrix_file.file_stem().unwrap().to_str().unwrap();

        // Setting output path
        let filepath = format!("{}{}_{}_results.csv", RESULT_PATH, alg_name, test_name);
        let filepath = Path::new(&filepath);

        // Reading experiment matrix
        let mut reader = csv::Reader::from_path(matrix_file).unwrap();
        let headers = reader.headers().unwrap().clone();

        println!("###### Running time experiment for {} ####", alg_name);


        // iterates over all ntd
        for record in reader.records() {
            let record = record.unwrap();
            let ntd_name = &record[0];

            let single_ntd_path = format!("{}{}", NTD_PATH, ntd_name);
            let single_ntd_path = Path::new(&single_ntd_path);

            // iterate over all graphs
            for (u, v) in record.iter().enumerate() {
                // u = 0 is just the ntd_name or the graph should not been measured
                if u == 0 || v.parse::<u32>().unwrap() == 0 { continue; }

                let graph_name = &headers[u];
                let single_graph_path = format!("{}{}", GRAPH_PATH, graph_name);
                let single_graph_path = Path::new(&single_graph_path);

                let ntd = import_ntd(single_ntd_path).unwrap();
                let graph = import_metis(single_graph_path).unwrap();

                // Open the writer for the csv output
                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(true)
                    .open(filepath)
                    .unwrap();

                let mut wtr = csv::Writer::from_writer(file);

                let width = ntd.width();
                let v_t = ntd.node_count();
                let e_tau = generate_possible_edges(&ntd).get(&ntd.root()).unwrap().len();
                let v_tau = ntd.vertex_count();

                let v_g = graph.node_count();
                let e_g = graph.edge_count();

                //Equivalence class algorithm
                let mut measurements = vec![];
                println!("Running experiment for ntd {:?} and graph {:?}", ntd_name, graph_name);

                for i in 0..5 {
                    println!("running test number {}", i + 1);
                    let start = Instant::now();

                    alg(&ntd, &graph);

                    let duration = start.elapsed();
                    println!("time needed: {:?}", duration);
                    measurements.push(duration);
                }

                let sum: Duration = measurements.iter().sum();
                let avg_measurements = sum.div_f32(measurements.len() as f32);
                println!("average running time is {:?}", avg_measurements);

                wtr.write_record(&[
                    &alg_name,
                    &ntd_name.to_string(),
                    &width.to_string(),
                    &v_t.to_string(),
                    &e_tau.to_string(),
                    &v_tau.to_string(),
                    &graph_name.to_string(),
                    &v_g.to_string(),
                    &e_g.to_string(),
                    &measurements[0].as_micros().to_string(),
                    &measurements[1].as_micros().to_string(),
                    &measurements[2].as_micros().to_string(),
                    &measurements[3].as_micros().to_string(),
                    &measurements[4].as_micros().to_string(),
                    &avg_measurements.as_micros().to_string(),
                ]
                );
            }
        }

    }
}
