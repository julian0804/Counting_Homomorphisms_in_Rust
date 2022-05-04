/// A module containing all functions necessary for generating graphs.
pub mod graph_generation{
    use std::collections::HashMap;
    use crate::tree_decompositions::nice_tree_decomposition::{NiceTreeDecomposition, NodeType};
    use crate::tree_decompositions::tree_structure::TreeNode;


    /// Returns true if the *undirected* edge is contained in the list.
    pub fn edge_in_list((u,v) : (usize, usize), list : &Vec<(usize, usize)>) -> bool{
        list.iter().any(|&i| i == (u , v) || i == (v , u))
    }

    /// Given a nice tree decomposition, this functions computes a hashmap that maps each node p to the set of
    /// possible edges that could occur in the subtree rooted at p.
    pub fn generate_possible_edges(ntd : &NiceTreeDecomposition) -> HashMap<TreeNode, Vec<(usize, usize)>>
    {
        let stingy_ordering = ntd.stingy_ordering();
        let mut possible_edges: HashMap<TreeNode, Vec<(usize, usize)>> = HashMap::new();

        // follow the stingy ordering
        for p in stingy_ordering{

            // match the Type of node p
            match ntd.node_type(p) {
                Some(NodeType::Leaf) => {
                    //returns the only vertex in the bag of p
                    let vertex = ntd.bag(p).unwrap().iter().next().unwrap();
                    possible_edges.insert(p, vec![(vertex.index(), vertex.index())]);
                }
                Some(NodeType::Introduce) => {
                    let q = ntd.unique_child(p).unwrap();
                    let v = ntd.unique_vertex(p).unwrap();
                    let mut edges = possible_edges.get(q).unwrap().clone();

                    let bag = ntd.bag(p).unwrap();

                    for u in bag{
                        // checks if edge has already been added
                        if  !edge_in_list((u.index(), v.index()), &edges){
                            edges.push((u.index(), v.index()));
                        }
                    }
                    possible_edges.insert(p, edges);

                }
                Some(NodeType::Forget) => {
                    let q = ntd.unique_child(p).unwrap();
                    // just clone the set of possible edges
                    possible_edges.insert(p, possible_edges.get(q).unwrap().clone());
                }
                Some(NodeType::Join) => {
                    let children = ntd.children(p).unwrap();

                    let q1 = children.get(0).unwrap();
                    let q2 = children.get(1).unwrap();

                    let first : &TreeNode;
                    let second : &TreeNode;

                    if possible_edges.get(q1).unwrap().len() >= possible_edges.get(q2).unwrap().len(){
                        first = q1;
                        second = q2;
                    }
                    else {
                        first = q2;
                        second = q1;
                    }

                    let mut edges = possible_edges.get(first).unwrap().clone();
                    // merge the edges
                    for (u,v) in possible_edges.get(second).unwrap(){
                        if !edge_in_list((*u, *v), &edges){
                            edges.push((*u , *v));
                        }
                    }
                    possible_edges.insert(p,edges);
                }
                None => ()
            }
        }

        possible_edges
    }

}