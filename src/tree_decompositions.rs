

/// A module containing data structures and algorithms for the tree structure
/// of (nice) tree decompositions
pub mod tree_structure{
    use std::cmp::max;
    use std::collections::{HashMap, HashSet};
    use petgraph::matrix_graph::NodeIndex;

    /// ## Type alias for better readability
    /// Nodes of the underlying tree simply represented by unsigned integers from 0,..., N-1
    pub type TreeNode = u64;
    /// Vertices contained in bag equal vertices of graphs
    pub type Vertex = NodeIndex;

    /// ## Tree Structure
    /// a simple tree structure to organize the data of tree decompositions
    /// Nodes will be numbered by 0,1,...,N-1 where N is the total amount of nodes
    #[derive(PartialEq, Eq, Debug, Clone)]
    pub struct TreeStructure{
        number_of_nodes: TreeNode,
        children_list: HashMap<TreeNode, Vec<TreeNode>>,
        parents_list: HashMap<TreeNode, TreeNode>,
    }

    /// ## Tree Structure Methods
    /// Implementation of basic functionalities of the tree structure
    impl TreeStructure{

        /// Given the number of nodes this method creates a new Tree Structure
        /// without edges and with empty bags. The number of nodes is static and does not change over time.
        pub fn new(number_of_nodes: u64) -> TreeStructure{
            TreeStructure{
                number_of_nodes,
                children_list : HashMap::new(),
                parents_list : HashMap::new(),
            }
        }

        /// Returns the number of nodes.
        pub fn node_count(&self) -> TreeNode {self.number_of_nodes}

        /// Returns an Option<&Vector> of the children of a given node p if Node could be found
        /// in the list of children. Else return None, which means that a Node does not exist or have children.
        pub fn children(&self, p: TreeNode) -> Option<&Vec<TreeNode>> {
            self.children_list.get(&p)
        }

        /// Counts and returns the number of children of a given node p.
        pub fn children_count(&self, p: TreeNode) -> TreeNode {
            (if let Some(i) = self.children(p) { i.len() } else { 0 }) as TreeNode
        }

        /// Returns the a reference to the parent of a given node p.
        /// If p does not have a parent node return None.
        pub fn parent(&self, node: TreeNode) -> Option<&TreeNode> {
            self.parents_list.get(&node)
        }

        /// Checks if node p is the parent of node q and returns a boolean.
        pub fn is_parent_of(&self, p: TreeNode, q: TreeNode) -> bool {
            if let Some(&node) = self.parent(q) { p == node } else { false }
        }

        /// Adds q as a child of p if possible. Each node can only have one parent.
        /// Each node can be set only once as a child node. This should prevent data
        /// inconsistency between children_list and parent list.
        pub fn add_child(&mut self, p : TreeNode, q : TreeNode){

            // Checks if node q already has a parent. Resetting the parent could lead to
            // data inconsistency between children_list and parent list. For each node we
            // can set its parent only ones.
            if self.parent(q) != None {
                panic!("Node {:?} already has a parent!", q);
            }

            // Controls that index is not out of bounds
            if max(p,q) >= self.number_of_nodes{
                panic!("Node index out of bounds!");
            }

            self.parents_list.insert(q,p);

            // Insert node q into the list of children of node p.
            if let Some(children) = self.children_list.get_mut(&p) {
                children.push(q);
            } else {
                self.children_list.insert(p, vec![q]);
            }

        }

        /// This method calculates and returns the root of the tree by starting arbitrary at the
        /// node 0 and going "up" until the root has been reached.
        pub fn root(&self) -> TreeNode{
            let mut current_node: TreeNode = 0;
            loop {
                if let Some(&parent) = self.parent(current_node) { current_node = parent; } else { break }
            }
            current_node
        }
    }


}

pub mod tree_decomposition{

}

pub mod nice_tree_decomposition{
    use std::collections::{HashMap, HashSet};
    use crate::tree_decompositions::tree_structure::{Vertex, TreeStructure, TreeNode};

    /// Bag-Type of Bags attached to each Node of the (nice) tree decomposition
    pub(crate) type Bag = HashSet<Vertex>;

    /// An enum containing types of Nodes in a nice tree decomposition
    #[derive(PartialEq, Eq, Debug, Clone)]
    pub enum NodeType {
        Leaf,
        Introduce,
        Forget,
        Join
    }

    /// A data structure containing additional information about each node.
    #[derive(PartialEq, Eq, Debug, Clone)]
    pub struct NodeData {
        node_type: NodeType,
        bag: Bag,
    }

    /// Implementation of methods needed for accessing and creating data of NodeData
    impl NodeData {

        /// A simple constructor for creating a new NodeData struct.
        pub fn new(node_type: NodeType, bag: Bag) -> NodeData {
            NodeData { node_type, bag }
        }

        /// Returns a reference to the node type of this node.
        pub fn node_type(&self) -> &NodeType { &self.node_type }

        /// Returns a reference to the bag of this node.
        pub fn bag(&self) -> &Bag { &self.bag }

    }

    /// A structure organizing all data need for a nice tree decomposition. Containing the following
    /// - a tree structure
    /// - a Hashmap which maps a TreeNode to its NodeData
    #[derive(PartialEq, Eq, Debug, Clone)]
    pub struct NiceTreeDecomposition{
        tree_structure : TreeStructure,
        nodes_data: HashMap<TreeNode, NodeData>,
        stingy_ordering: Vec<TreeNode>,
        unique_vertices: HashMap<TreeNode, Vertex>
    }

    /// Implementation of methods for nice tree decompositions
    impl NiceTreeDecomposition{

        /// A simple constructor for the NiceTreeDecomposition
        pub fn new(tree_structure : TreeStructure, nodes_data : HashMap<TreeNode, NodeData>) -> NiceTreeDecomposition{
            // Computes stingy ordering of Nice Tree Decomposition in advance
            let stingy_ordering = NiceTreeDecomposition::compute_stingy_ordering(&tree_structure, &nodes_data);
            let unique_vertices = NiceTreeDecomposition::compute_unique_vertices(&tree_structure, &nodes_data, &stingy_ordering);

            NiceTreeDecomposition{ tree_structure , nodes_data, stingy_ordering, unique_vertices}
        }

        /// ## Functions for getting node data

        /// Returns the bag of the given node p.
        pub fn bag(&self, p : TreeNode) -> Option<&Bag>{
            if let Some(node_data) = self.nodes_data.get(&p){ Some( node_data.bag() ) } else { None }
        }

        /// Returns the node type of the given node p.
        pub fn node_type(&self, p : TreeNode) -> Option<&NodeType>{
            if let Some(node_data) = self.nodes_data.get(&p) {Some( node_data.node_type() ) } else { None }
        }

        /// ## Structural functions on tree nodes

        /// An Interface function for the root() method of the private field tree_structure.
        pub fn root(&self) -> TreeNode{
            self.tree_structure.root()
        }

        /// An Interface function for the parent() method of the private field tree_structure.
        pub fn parent(&self, p : TreeNode) -> Option<&TreeNode> {
            self.tree_structure.parent(p)
        }

        /// An Interface function for the children() method of the private field tree_structure.
        pub fn children(&self, p : TreeNode) -> Option<&Vec<TreeNode>> {
            self.tree_structure.children(p)
        }

        /// Returns the unique child node q of a given node p. Note that
        /// this function can only be used for Introduce or Forget Nodes.
        pub fn unique_child(&self, p : TreeNode) -> Option<&TreeNode>{
            match self.node_type(p){
                Some(NodeType::Introduce) | Some(NodeType::Forget) => {
                    self.children(p).unwrap().get(0)
                },
                _ => {None}
            }
        }

        /// ## Functions for node bag properties

        /// This private function computes the Hashmap of  unique vertices by following the stingy ordering and compute this entry for each
        /// Introduce, Forget and Leaf nodes. Join nodes do not have unique vertices.
        /// - The unique vertex of a Leaf node is its only contained vertex.
        /// - The unique vertex of a Introduce node is the vertex that have been introduced.
        /// - The unique vertex of a Forget node is the vertex that have been forgotten.
        fn compute_unique_vertices(tree_structure : &TreeStructure, nodes_data : &HashMap<TreeNode, NodeData>, stingy_ordering : &Vec<TreeNode>) -> HashMap<TreeNode, Vertex>
        {
            // set initial value of the unique vertex Hashmap
            let mut unique_vertices = HashMap::new();

            // For each node in the stingy ordering
            for &p in stingy_ordering{
                // get the node_data of the current node
                let node_data = nodes_data.get(&p).unwrap();

                match node_data.node_type()
                {
                    NodeType::Leaf => {
                        let v = *node_data.bag().iter().next().unwrap();
                        unique_vertices.insert(p,v);
                    }
                    NodeType::Introduce => {
                        // get child node q of p.
                        let q = tree_structure.children(p).unwrap().iter().next().unwrap();

                        // get bags of both nodes p and q.
                        let bag_p = node_data.bag();
                        let bag_q = nodes_data.get(q).unwrap().bag();

                        // get the difference of both bags
                        let difference: HashSet<&Vertex> = bag_p.difference(bag_q).collect();
                        let v = **difference.iter().next().unwrap();

                        unique_vertices.insert(p,v);

                    }
                    NodeType::Forget => {
                        // get child node q of p.
                        let q = tree_structure.children(p).unwrap().iter().next().unwrap();

                        // get bags of both nodes p and q.
                        let bag_p = node_data.bag();
                        let bag_q = nodes_data.get(q).unwrap().bag();

                        // get the difference of both bags
                        let difference: HashSet<&Vertex> = bag_q.difference(bag_p).collect();
                        let v = **difference.iter().next().unwrap();

                        unique_vertices.insert(p,v);
                    }
                    NodeType::Join => {}
                }
            }

            unique_vertices
        }

        /// returns the unique vertex of a given node p for nodes of the following types:
        /// - leaf
        /// - introduce
        /// - forget
        pub fn unique_vertex(&self, p : TreeNode) -> Option<&Vertex>{
            self.unique_vertices.get(&p)
        }

        /// ## stingy ordering functions

        /// Returns a copy of the stingy ordering. This should have no big overhead since
        /// all TreeNodes are represented by a simple u64 integer.
        pub fn stingy_ordering(&self) -> Vec<TreeNode>{ self.stingy_ordering.clone() }

        /// This function calculates the stingy ordering when a nice tree decomposition is constructed.
        fn compute_stingy_ordering(tree_structure : &TreeStructure, nodes_data : &HashMap<TreeNode, NodeData>) -> Vec<TreeNode>{
            NiceTreeDecomposition::recursive_stingy_ordering(tree_structure, nodes_data, tree_structure.root()).0
        }

        /// This function recursively calculate the stingy ordering of the subtree rooted at p.
        /// Therefore it returns a tuple consisting of first the calculated stingy ordering and second
        /// the branch number of this nodes which equals the number of nodes in the subtree with degree 2.
        fn recursive_stingy_ordering(tree_structure : &TreeStructure, nodes_data : &HashMap<TreeNode, NodeData>, p: TreeNode) -> (Vec<TreeNode>, TreeNode){

            // Initially set return values
            let mut stingy_order : Vec<TreeNode>= Vec::new();
            let mut branch_number : TreeNode = 0;


            // get the node_data of the current node
            let node_data = nodes_data.get(&p).unwrap();

            // matching the type of the current node
            match node_data.node_type(){
                NodeType::Leaf => (), // vertex will be pushed later and branch number is already 0
                NodeType::Introduce | NodeType::Forget => {
                    if let Some(children) = tree_structure.children(p){

                        if let Some(&q) = children.get(0){
                            // get the stingy ordering of the child node q and safe it
                            let (so, bn) = NiceTreeDecomposition::recursive_stingy_ordering(tree_structure, nodes_data, q);
                            stingy_order = so;
                            branch_number = bn;
                        }

                    }
                },
                NodeType::Join => {
                    if let Some(children) = tree_structure.children(p){

                        let &q1 = children.get(0).unwrap();
                        let &q2 = children.get(1).unwrap();

                        let (mut so1, bn1) = NiceTreeDecomposition::recursive_stingy_ordering(tree_structure, nodes_data, q1);
                        let (mut so2, bn2) = NiceTreeDecomposition::recursive_stingy_ordering(tree_structure, nodes_data, q2);

                        // Comparing the branch numbers of both subtrees
                        if bn1 >= bn2{
                            stingy_order = so1;
                            stingy_order.append(&mut so2);
                        }
                        else {
                            stingy_order = so2;
                            stingy_order.append(&mut so1);
                        }

                        branch_number = bn1 + bn2 + 1; // summing up the branch number
                    }
                }
            }

            // inserting current node at last
            stingy_order.push(p);

            // return result
            (stingy_order, branch_number)
        }

    }

}

#[cfg(test)]
pub mod tree_structure_tests{
    use crate::tree_decompositions::tree_structure;

    #[test]
    pub fn test_tree_structure_methods(){

        let mut tree_structure = tree_structure::TreeStructure::new(5);

        // Before adding edges
        assert_eq!(tree_structure.is_parent_of(4,0), false);
        assert_eq!(tree_structure.node_count(), 5);
        assert_eq!(tree_structure.parent(1), None);
        assert_eq!(tree_structure.root(), 0);
        assert_eq!(tree_structure.children_count(0), 0);

        // Adding edges
        tree_structure.add_child(4,0);
        tree_structure.add_child(0,2);
        tree_structure.add_child(0,1);
        tree_structure.add_child(1,3);

        // After adding edges
        assert_eq!(tree_structure.is_parent_of(4,0), true);
        assert_eq!(tree_structure.node_count(), 5);
        assert_eq!(tree_structure.parent(1), Some(&0));
        assert_eq!(tree_structure.root(), 4);
        assert_eq!(tree_structure.children_count(0), 2);
    }
}

#[cfg(test)]
pub mod nice_tree_decomposition_test{

}
