

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
    /// Bag-Type of Bags attached to each Node of the (nice) tree decomposition
    pub(crate) type Bag = HashSet<Vertex>;

    /// ## Tree Structure
    /// a simple tree structure to organize the data of tree decompositions
    /// Nodes will be numbered by 0,1,...,N-1 where N is the total amount of nodes
    #[derive(PartialEq, Eq, Debug)]
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

    /// An enum containing types of Nodes in a nice tree decomposition
    #[derive(PartialEq, Eq, Debug, Clone)]
    pub enum NodeType {
        Leaf,
        Introduce,
        Forget,
        Join
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
