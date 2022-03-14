
pub mod diaz {

    use std::collections::HashMap;
    use petgraph::matrix_graph::MatrixGraph;
    use petgraph::Undirected;
    use crate::graph_structures::graph_structures::nice_tree_decomposition::{NiceTreeDecomposition, NodeType, TreeNode};


    /*
    A mapping based on the representation in
    **Counting subgraph patterns in large graphs**
     */
    type Mapping = u64;

    /*
    a struct containing all infos about the dynamic program
     */
    pub struct DPData {
        table : HashMap<TreeNode, HashMap<Mapping, u64>>,
        from_graph : MatrixGraph<(),(), Undirected>,
        to_graph : MatrixGraph<(),(), Undirected>,
    }



    impl DPData {
        pub fn new(from_graph : MatrixGraph<(),(), Undirected>, to_graph : MatrixGraph<(),(), Undirected>) -> DPData {
            DPData { table : HashMap::new(), from_graph, to_graph }
        }

        /*
        Gets the entry for a given vertex and mapping
         */
        pub fn get(&self, node : &TreeNode, mapping : &Mapping) -> Option<&u64> {
            if let Some(mappings) = self.table.get(node){ mappings.get(mapping) }
            else { None }
        }

        /*
        sets the entry for a given vertex
         */
        pub fn set(&mut self, node : TreeNode, mapping : Mapping, value : u64){
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
    pub fn diaz(from_graph : MatrixGraph<(),(), Undirected>, ntd : NiceTreeDecomposition, to_graph : MatrixGraph<(),(), Undirected>) -> u64
    {
        let stingy_order = ntd.stingy_ordering();

        let mut table = DPData::new(from_graph, to_graph);

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



#[cfg(test)]
mod tests{
    use crate::algorithms::diaz::DPData;
    use crate::file_handler::file_handler::metis_to_graph;

    #[test]
    fn test_dpdata(){
        let from_graph = metis_to_graph("data/metis_graphs/from_2.graph").unwrap();
        let to_graph = metis_to_graph("data/metis_graphs/to_2.graph").unwrap();
        let mut test_dp_data = DPData::new(from_graph, to_graph);

        test_dp_data.set(5,4,4);
        test_dp_data.set(2,3,5);
        test_dp_data.set(9,12,3);

        assert_eq!(*test_dp_data.get(&5,&4).unwrap(), 4);
        assert_eq!(*test_dp_data.get(&2,&3).unwrap(), 5);
        assert_eq!(*test_dp_data.get(&9,&12).unwrap(), 3);
    }

    #[test]
    fn test_integer_functions(){

    }

}
