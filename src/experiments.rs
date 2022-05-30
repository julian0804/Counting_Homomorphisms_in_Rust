
pub mod single_running_time_measurement {
    use std::fs;
    use std::fs::{OpenOptions, ReadDir};
    use std::ops::Add;
    use std::path::Path;
    use std::time::{Duration, Instant};
    use csv;
    use itertools::Itertools;
    use crate::diaz_serna_thilikos::diaz_algorithm::diaz_serna_thilikos_for_ntd_set;
    use crate::modified_DP::algorithm::equivalence_class_algorithm;
    use crate::file_handler::graph_handler::import_metis;
    use crate::file_handler::tree_decomposition_handler::import_ntd;
    use crate::graph_generation::graph_generation_algorithms::generate_possible_edges;

    /// a function getting all paths of tree decomposition
    fn get_nice_tree_decomposition_paths() -> ReadDir
    {
        let ntd_path = Path::new("./data/nice_tree_decompositions/benchmark_ntds");
        fs::read_dir(ntd_path).unwrap()
    }

    /// A function getting all paths of path ntds
    fn get_path_ntd_paths() -> ReadDir
    {
        let ntd_path = Path::new("./data/nice_tree_decompositions/benchmark_ntds/path_ntds");
        fs::read_dir(ntd_path).unwrap()
    }

    /// A function getting all paths of path ntds
    fn get_complete_ntd_paths() -> ReadDir
    {
        let ntd_path = Path::new("./data/nice_tree_decompositions/benchmark_ntds/complete_ntds");
        fs::read_dir(ntd_path).unwrap()
    }

    /// a function returning all paths of tree decomposition
    fn get_auto_generated_graph_paths() -> ReadDir
    {
        let graph_path = Path::new("./data/metis_graphs/auto_generated_graphs");
        fs::read_dir(graph_path).unwrap()
    }


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

    /// This function imports a experiment matrix and measures the marked combinations
    pub fn run_experiment(matrix_file: &Path) {

        // Construct file path of output file
        let result_path = "./target/experiment_results/";
        let date = chrono::Local::now().timestamp().to_string();

        let name = matrix_file.file_name().unwrap().to_str().unwrap();

        let filepath = format!("{}experiment_{}_results.csv", result_path, name);
        let filepath = Path::new(&filepath);


        // fixing the experiment paths
        let ntd_path = "./data/nice_tree_decompositions/benchmark_ntds/final_selection/";
        let graph_path = "./data/metis_graphs/final_selection/";

        let mut reader = csv::Reader::from_path(matrix_file).unwrap();

        let headers = reader.headers().unwrap().clone();

        // iterates over all ntd
        for record in reader.records() {
            let record = record.unwrap();
            let ntd_name = &record[0];

            let single_ntd_path = format!("{}{}", ntd_path, ntd_name);
            let single_ntd_path = Path::new(&single_ntd_path);

            // iterate over all graphs
            for (u, v) in record.iter().enumerate() {
                // u = 0 is just the ntd_name or the graph should not been measured
                if u == 0 || v.parse::<u32>().unwrap() == 0 { continue; }

                let graph_name = &headers[u];
                let single_graph_path = format!("{}{}", graph_path, graph_name);
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
                let mut measurements_equivalence_algorithm = vec![];
                println!("Equivalence Algorithm: running experiment for ntd {:?} and graph {:?}", ntd_name, graph_name);

                for i in 0..5 {
                    println!("running test number {}", i + 1);
                    let start = Instant::now();

                    equivalence_class_algorithm(&ntd, &graph);

                    let duration = start.elapsed();
                    println!("time needed: {:?}", duration);
                    measurements_equivalence_algorithm.push(duration);
                }

                let sum: Duration = measurements_equivalence_algorithm.iter().sum();
                let avg_equivalence_algorithm = sum.div_f32(measurements_equivalence_algorithm.len() as f32);
                println!("average running time is {:?}", avg_equivalence_algorithm);

                println!("Diaz: running experiment for ntd {:?} and graph {:?}", ntd_name, graph_name);
                let mut measurements_diaz = vec![];

                for i in 0..5 {
                    println!("running test number {}", i + 1);
                    let start = Instant::now();

                    diaz_serna_thilikos_for_ntd_set(&ntd, &graph);

                    let duration = start.elapsed();
                    println!("time needed: {:?}", duration);
                    measurements_diaz.push(duration);
                }

                let sum: Duration = measurements_diaz.iter().sum();
                let avg_diaz = sum.div_f32(measurements_diaz.len() as f32);
                println!("average running time is {:?}", avg_diaz);

                wtr.write_record(&[
                    "equivalence algorithm",
                    &ntd_name,
                    &width.to_string(),
                    &v_t.to_string(),
                    &e_tau.to_string(),
                    &v_tau.to_string(),
                    &graph_name,
                    &v_g.to_string(),
                    &e_g.to_string(),
                    &measurements_equivalence_algorithm[0].as_millis().to_string(),
                    &measurements_equivalence_algorithm[1].as_millis().to_string(),
                    &measurements_equivalence_algorithm[2].as_millis().to_string(),
                    &measurements_equivalence_algorithm[3].as_millis().to_string(),
                    &measurements_equivalence_algorithm[4].as_millis().to_string(),
                    &avg_equivalence_algorithm.as_micros().to_string(),
                    &measurements_diaz[0].as_millis().to_string(),
                    &measurements_diaz[1].as_millis().to_string(),
                    &measurements_diaz[2].as_millis().to_string(),
                    &measurements_diaz[3].as_millis().to_string(),
                    &measurements_diaz[4].as_millis().to_string(),
                    &avg_diaz.as_micros().to_string(),
                ]
                );
            }
        }
    }
}
