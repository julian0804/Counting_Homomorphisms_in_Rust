/// A module containing brute force homomorphism counter
pub mod brute_force_homomorphism_counter{

    use petgraph::matrix_graph::MatrixGraph;
    use petgraph::Undirected;
    use crate::graph_generation::graph_generation_algorithms::{generate_graphs, generate_possible_edges};
    use crate::integer_functions::integer_functions_methods;
    use crate::integer_functions::integer_functions_methods::{apply, Mapping, max_mappings};
    use crate::tree_decompositions::nice_tree_decomposition::NiceTreeDecomposition;
    use crate::tree_decompositions::tree_structure::Vertex;

    /// a simple brute force algorithm which iterates over all possible mappings from "from_graph" to "to_graph"
    /// todo: a possible improvement would be to first seperate the graph into its connected components and then execute this algo for each of them
    /// todo: generalize them for more graph types
    pub fn simple_brute_force(from_graph : &MatrixGraph<(),(), Undirected>, to_graph : &MatrixGraph<(),(), Undirected>) -> u64{

        let h = from_graph.node_count();
        let g = to_graph.node_count();

        // Checks if mapping is a homomorphism
        let check_mapping = |f : Mapping|{

            let mut ret = true;

            for u in 0..h{
                for v in 0..h{
                    if from_graph.has_edge(Vertex::new(u ), Vertex::new(v )){

                        let map_u = integer_functions_methods::apply(g as Mapping, f, u as Mapping);
                        let map_v = integer_functions_methods::apply(g as Mapping, f, v as Mapping);

                        if !to_graph.has_edge(Vertex::new(map_u as usize), Vertex::new(map_v as usize))
                        {
                            ret = false;
                        }
                    }
                }
            }

            ret
        };

        let max = max_mappings(h as Mapping, g as Mapping);
        let mut counter = 0;

        // for all mapings from H to G
        for f in 0..max{
            if check_mapping(f){counter += 1;}
        }
        counter
    }


    /// Implementation of simple_brute_force for all graphs in $H_\tau$
    pub fn simple_brute_force_for_ntd_set(ntd : &NiceTreeDecomposition, to_graph : &MatrixGraph<(),(), Undirected>) -> Vec<(MatrixGraph<(), (), Undirected>, u64)>{
        let mut result = vec![];

        let possible_edges = generate_possible_edges(ntd);

        let graphs = generate_graphs(ntd.vertex_count() as u64,
                                     possible_edges.get(&ntd.root()).unwrap().clone() );

        for graph in graphs{

            let hom_number = simple_brute_force(&graph, to_graph);
            result.push(( graph, hom_number));
        }

        result
    }
}
