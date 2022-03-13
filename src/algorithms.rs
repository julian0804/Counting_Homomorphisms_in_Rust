
/*
pub mod counting_graph_homomorphisms{
    use std::collections::{HashMap, HashSet};
    use std::iter::Map;
    use crate::{NiceTreeDecomposition, SimpleGraph};
    use crate::graph_structures::graph_structures::nice_tree_decomposition::NodeType;
    use crate::graph_structures::graph_structures::Vertex;

    // Define the type of a basically mapping
    //type Mapping = HashMap<Vertex, Vertex>;

    /*
    A mapping based on the representation in
    **Counting subgraph patterns in large graphs**
     */
    type Mapping = u64;



    /*
    a struct containing all infos about the dynamic program
     */
    struct DPInfo {
        table : HashMap<Vertex, HashMap<Mapping, u64>>,
        from_graph : SimpleGraph,
        to_graph : SimpleGraph,
    }

    impl DPInfo {
        pub fn new(from_graph : SimpleGraph, to_graph : SimpleGraph) -> DPInfo {
            DPInfo { table : HashMap::new(), from_graph, to_graph }
        }

        /*
        Gets the entry for a given vertex and mapping
         */
        pub fn get(&self, node : &Vertex, mapping : &Mapping) -> Option<&u64> {
            if let Some(mappings) = self.table.get(node){ mappings.get(mapping) }
            else { None }
        }

        /*
        sets the entry for a given vertex
         */
        pub fn set(&mut self, node : Vertex, mapping : Mapping, value : u64){
            if let Some(mappings) = self.table.get_mut(&node){
                mappings.insert(mapping,value);
            }
            else {
                self.table.insert(node, HashMap::from([(mapping, value)]));
            }
        }
    }

    /*
    Based on the following algorithm of Diaz, Serna, Thilikos
    https://www.sciencedirect.com/science/article/pii/S0304397502000178?via%3Dihub

    1. Use Hashmaps for representing the mappings
     */
    /*
    pub fn diaz(from_graph : SimpleGraph, ntd : NiceTreeDecomposition, to_graph : SimpleGraph) -> u64
    {
        let stingy_order = ntd.stingy_ordering();

        let mut table = DPInfo::new(from_graph, to_graph);

        for node in stingy_order{
            match ntd.get_node_data(&node){
                Some(NodeType::Leaf(..)) => {
                    let unique_vertex = ntd.get_bag(&node).unwrap().iter().next().unwrap();
                    // be carefully, we return the number of vertices

                    // inserts the mapping (unique_vertex -> aim_vertex) for each
                    // aim_vertex in the aim graph
                    for aim_vertex in 1..to_graph.adjacency_list.number_of_vertices(){
                        table.set(node, aim_vertex as Mapping, 1);
                    }
                }
                Some(NodeType::Introduce(..)) => {
                }
                Some(NodeType::Forget(..)) => {

                }
                Some(NodeType::Join(..)) => {

                }
                None => {

                }
            }
        }


        1
    }

     */

}
 */

#[cfg(test)]
mod tests{

    #[test]
    fn test_dpinfo(){

    }

    #[test]
    fn test_integer_functions(){

    }

}
