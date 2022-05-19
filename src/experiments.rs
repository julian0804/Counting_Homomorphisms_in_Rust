
pub mod single_running_time_measurement{
    use std::fs;
    use std::fs::ReadDir;
    use std::path::Path;
    use chrono;
    use csv;

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
        let timestamp = chrono::Local::now().to_string();
        let filepath = format!("{}experiment_{}_results.csv",result_path, timestamp);
        let filepath = Path::new(&filepath);

        let mut wtr = csv::Writer::from_path(filepath);


        // here we can select the set of nice tree decompositions and the set of graphs (to_graph)
        // we want to compare with each other
        let ntd_paths = get_path_ntd_paths();
        let graph_paths = get_auto_generated_graph_paths();

        for Ok(ntd_path) in ntd_paths{
            for Ok(graph_path) in graph_paths{



            }
        }

        // iterate over all loaded graphs and ntds
        // for each combination test 5 times
        // calculate average
        // safe results in file


    }

}