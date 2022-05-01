/// A module containing brute force homomorphism counter
pub mod brute_force_homomorphism_counter{

    use petgraph::matrix_graph::MatrixGraph;
    use petgraph::Undirected;
    use crate::tree_decompositions::tree_structure::Vertex;

    /// a simple brute force algorithm which iterates over all possible mappings from "from_graph" to "to_graph"
    /// todo: a possible improvement would be to first seperate the graph into its connected components and then execute this algo for each of them
    /// todo: generalize them for more graph types
    pub fn simple_brute_force(from_graph : &MatrixGraph<(),(), Undirected>, to_graph : &MatrixGraph<(),(), Undirected>) -> u64{

        let h = from_graph.node_count();
        let g = to_graph.node_count();

        // Checks if mapping is a homomorphism
        let check_mapping = |f : usize|{

            let mut ret = true;

            for u in 0..h{
                for v in 0..h{
                    if from_graph.has_edge(Vertex::new(u ), Vertex::new(v )){

                        // this is basically the apply functions of the integer functions
                        let map_u = f / (g.pow(u as u32) as u64) as usize % g ;
                        let map_v = f / (g.pow(v as u32) as u64) as usize % g ;

                        if !to_graph.has_edge(Vertex::new(map_u), Vertex::new(map_v))
                        {
                            ret = false;
                        }
                    }
                }
            }

            ret
        };

        let max = g.pow(h as u32);
        let mut counter = 0;

        // for all mapings from H to G
        for f in 0..max{
            if check_mapping(f){counter += 1;}
        }
        counter
    }
}
