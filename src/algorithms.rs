
pub mod diaz {

    use std::collections::HashMap;
    use std::iter::Map;
    use petgraph::matrix_graph::MatrixGraph;
    use petgraph::Undirected;
    use crate::graph_structures::graph_structures::nice_tree_decomposition::{NiceTreeDecomposition, NodeType, TreeNode};

    pub type Mapping = u64;

    /*
    a struct containing all infos about the dynamic program
     */
    pub struct DPData<'a>{
        table : HashMap<TreeNode, HashMap<Mapping, u64>>,
        from_graph : &'a MatrixGraph<(),(), Undirected>,
        to_graph : &'a MatrixGraph<(),(), Undirected>,
    }



    impl<'a> DPData<'a>{

        pub fn new<'b>(from_graph : &'b MatrixGraph<(),(), Undirected>, to_graph : &'b MatrixGraph<(),(), Undirected>) -> DPData<'b> {
            DPData { table : HashMap::new(),  from_graph, to_graph }
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

        /*
        The Following operations work on integer functions
        mappings from a bag to a graph are represented as simple integers

        based on  **Counting subgraph patterns in large graphs**

         */

        /*
        Returns digit of mapping f with significance s
         */
        pub fn apply(&self, f : Mapping, s : Mapping) -> Mapping{
            // TODO: ugly safe casting
            f / (self.to_graph.node_count().pow(s as u32) as u64) % (self.to_graph.node_count() as u64)
        }

        /*
        "increases the number of digits in f by one. It should
        shift all digits with significance > s one position to the left and then set the
        digit with significance s equal to v."
         */
        pub fn extend(&self, f : Mapping, s : Mapping, v : Mapping) -> Mapping{
            let n = self.to_graph.node_count();
            let r = f % (n.pow(s as u32) as Mapping);
            let l = f - r;
            ((n as Mapping) * l) + (n.pow(s as u32) as Mapping) * v + r
        }

        /*
        "should remove the digit with significance s and shift
        all digits with higher significance one position to the right."
         */
        pub fn reduce(&self, f : Mapping, s : Mapping) -> Mapping{
            let n = self.to_graph.node_count();
            let r = f % (n.pow(s as u32) as Mapping);
            let l = f - (f % n.pow((s + 1) as u32) as u64);
            l / (n as Mapping) + r
        }


    }

    /*
    Based on the following algorithm of Diaz, Serna, Thilikos
    https://www.sciencedirect.com/science/article/pii/S0304397502000178?via%3Dihub

    1. Use Hashmaps for representing the mappings
     */
    pub fn diaz(from_graph : &MatrixGraph<(),(), Undirected>, ntd : NiceTreeDecomposition, to_graph : &MatrixGraph<(),(), Undirected>) -> u64
    {
        let stingy_order = ntd.stingy_ordering();

        let mut table = DPData::new(from_graph, to_graph);

        for node in stingy_order{
            match ntd.node_type(node){
                Some(Leaf) => {
                    let unique_vertex = ntd.bag(node).unwrap().iter().next().unwrap();
                    // be carefully, we return the number of vertices

                    // inserts the mapping (unique_vertex -> aim_vertex) for each
                    // aim_vertex in the aim graph
                    for aim_vertex in 0..to_graph.node_count(){
                        table.set(node, aim_vertex as Mapping, 1);
                    }
                }
                Some(Introduce) => {
                }
                Some(Forget) => {

                }
                Some(Join) => {

                }
                None => {

                }
            }
        }
        1
    }

}



#[cfg(test)]
mod tests{
    use crate::algorithms::diaz::DPData;
    use crate::file_handler::file_handler::metis_to_graph;

    #[test]
    fn test_dpdata(){
        let from_graph = metis_to_graph("data/metis_graphs/from_2.graph").unwrap();
        let to_graph = metis_to_graph("data/metis_graphs/to_2.graph").unwrap();
        let mut test_dp_data = DPData::new(&from_graph, &to_graph);

        test_dp_data.set(5,4,4);
        test_dp_data.set(2,3,5);
        test_dp_data.set(9,12,3);

        assert_eq!(*test_dp_data.get(&5,&4).unwrap(), 4);
        assert_eq!(*test_dp_data.get(&2,&3).unwrap(), 5);
        assert_eq!(*test_dp_data.get(&9,&12).unwrap(), 3);
    }

    #[test]
    fn test_integer_functions(){

        let from_graph = metis_to_graph("data/metis_graphs/from_2.graph").unwrap();
        let to_graph = metis_to_graph("data/metis_graphs/to_2.graph").unwrap();
        let mut test_dp_data = DPData::new(&from_graph, &to_graph);

        // testing apply function
        // to graph has 5 Vertices
        // 3 * 1 + 0 * 5 + 3 * 25 + 2 * 125 = 328

        let f : u64 = 328;
        assert_eq!(test_dp_data.apply(f,0),3);
        assert_eq!(test_dp_data.apply(f,1),0);
        assert_eq!(test_dp_data.apply(f,2),3);
        assert_eq!(test_dp_data.apply(f,3),2);

        // 3 * 1 + 0 * 5 + 3 * 25 + 2 * 125 = 328
        // -> 3 * 1 + 0 * 5 + (2 * 25) + 3 * 125 + 2 * 625 = 1678
        let f_2 = test_dp_data.extend(f,2, 2);
        assert_eq!(f_2, 1678);

        // 3 * 1 + 0 * 5 + (2 * 25) + 3 * 125 + 2 * 625 = 1678
        // 0 * 1 + 2 * 5 + 3 * 25 + 2 * 125 = 335
        let f_3 = test_dp_data.reduce(f_2, 0);
        assert_eq!(f_3, 335);

    }

}
