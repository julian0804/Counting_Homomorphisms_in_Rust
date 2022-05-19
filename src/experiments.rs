
pub mod single_running_time_measurement{
    use std::fs;
    use std::fs::{OpenOptions, ReadDir};
    use std::ops::Add;
    use std::path::Path;
    use std::time::{Duration, Instant};
    use csv;
    use itertools::Itertools;
    use crate::equivalence_class_algorithm::algorithm::equivalence_class_algorithm;
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




    /// measure runtime
    pub fn measure_running_time(){


        // Construct file path of output file
        let result_path = "./target/benchmark_results/";
        let date = chrono::Local::now().timestamp().to_string();

        let filepath = format!("{}experiment_{}_results.csv", result_path, date);
        let filepath = Path::new(&filepath);




        // here we can select the set of nice tree decompositions and the set of graphs (to_graph)
        // we want to compare with each other
        for ntd_path in fs::read_dir("./data/nice_tree_decompositions/benchmark_ntds/final_selection").unwrap(){

            let ntd_name = ntd_path.as_ref().unwrap().file_name();

            for graph_path in fs::read_dir("./data/metis_graphs/final_selection").unwrap(){

                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(true)
                    .open(filepath)
                    .unwrap();

                let mut wtr = csv::Writer::from_writer(file);

                let graph_name = graph_path.as_ref().unwrap().file_name();

                println!("Calculating number of homomorphisms for nice tree decomposition {:?} and graph {:?}", ntd_name.to_str(), graph_name.to_str());

                let ntd = import_ntd(ntd_path.as_ref().unwrap().path()).unwrap();
                let graph = import_metis(graph_path.as_ref().unwrap().path()).unwrap();

                let width = ntd.width();
                let v_t = ntd.node_count();
                let e_tau = generate_possible_edges(&ntd).get(&ntd.root()).unwrap().len();
                let v_tau = ntd.vertex_count();

                let v_g = graph.node_count();
                let e_g = graph.edge_count();

                let mut measurements = vec![];

                for i in 0..5{

                    println!("running test number {}",i);
                    let start = Instant::now();

                    equivalence_class_algorithm(&ntd, &graph);

                    let duration = start.elapsed();
                    println!("time needed: {:?}", duration );
                    measurements.push(duration);

                }

                let sum : Duration = measurements.iter().sum();
                let avg = sum.div_f32(measurements.len() as f32);
                println!("average running time is {:?}", avg);

                wtr.write_record(&[
                    "equivalence algorithm",
                    &ntd_name.to_str().unwrap(),
                    &width.to_string(),
                    &v_t.to_string(),
                    &e_tau.to_string(),
                    &v_tau.to_string(),
                    &graph_name.to_str().unwrap(),
                    &v_g.to_string(),
                    &e_g.to_string(),
                    &measurements[0].as_millis().to_string(),
                    &measurements[1].as_millis().to_string(),
                    &measurements[2].as_millis().to_string(),
                    &measurements[3].as_millis().to_string(),
                    &measurements[4].as_millis().to_string(),
                    &avg.as_micros().to_string()
                ]
                );

                wtr.flush();

            }
        }



    }

}