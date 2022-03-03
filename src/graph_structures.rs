
pub mod graph_structures {

    use std::collections::HashSet;
    use std::collections::HashMap;

    pub type Vertex = u32;
    pub type VertexBag = HashSet<Vertex>;

    pub mod adjacency{
        use super::*;
        /*
        Implementation of a simple adjacency list
        */
        #[derive(Debug,PartialEq)]
        pub struct AdjList{
            pub list : HashMap<Vertex, Vec<Vertex>>,
        }

        impl AdjList{

            /*
           Returns an empty AdjList
            */
            pub fn new() -> AdjList{
                AdjList{list : HashMap::new() }
            }

            /*
            Returns the number of edges in the adjacency list
             */
            pub fn number_of_edges(&self) -> usize
            {
                let mut sum = 0;
                for key in self.list.keys()
                {
                    sum += self.list.get(key).unwrap().len();
                }
                sum
            }

            /*
            returning the number of vertices by searching the maximum key
             */
            pub fn number_of_vertices(&self) -> usize
            {
                let max = self.list.keys().iter().max();
                match  max{
                    Some(i) => i,
                    None => 0,
                }
            }

            /*
            Checks if the edge (from, to) exists in this adjacency list
             */
            pub fn check_edge(&self, from : Vertex, to : Vertex) -> bool {
                if self.list.get(&from) == None{
                    return false;
                }
                self.list.get(&from).unwrap().contains(&to)
            }
            /*
            inserts edge if not already available
             */
            pub fn insert_edge(&mut self, from : Vertex,to : Vertex){
                // Checks if edge already exists
                if !self.check_edge(from,to) {

                    // Checks if we already have an edge going out of "from"
                    if self.list.get(&from) != None{
                        if let mut nlist = self.list.get_mut(&from).unwrap(){
                            nlist.push(to);
                        }
                    }
                    else{
                        self.list.insert(from,vec![to]);
                    }
                }
            }

            /*
            Returns an &Vec<Vertex> of the out neighbours if the vertex "from" has some, None otherwise
             */
            pub fn out_neighbours(&self, from : Vertex) -> Option<&Vec<Vertex>>
            {
                if let Some(i) = self.list.get(&from){
                    Some(i)
                }
                else {
                    None
                }
            }

            /*
            Returns the out degree of the vertex "from"
             */
            pub fn out_degree(&self, from : Vertex) -> usize
            {
                if let Some(i) = self.out_neighbours(from){ i.len() }
                else { 0 }
            }
        }
    }



}

#[cfg(test)]
mod tests {
    use crate::graph_structures::graph_structures::adjacency::AdjList;
    #[test]
    fn adjacency_list(){
        let mut adjlist = AdjList::new();
        assert_eq!(adjlist.number_of_edges(),0);

        adjlist.insert_edge(1,2);
        assert_eq!(adjlist.number_of_edges(),1);

        adjlist.insert_edge(1,2);
        assert_eq!(adjlist.number_of_edges(),1);

        adjlist.insert_edge(3,2);
        adjlist.insert_edge(1,3);
        assert_eq!(adjlist.number_of_edges(), 3);

        assert_eq!(adjlist.check_edge(1,3), true);
        assert_eq!(adjlist.check_edge(3,1), false);
    }
}