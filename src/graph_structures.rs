pub mod graph_structures {
    use petgraph::*;

    use std::collections::HashSet;
    use std::collections::HashMap;

    pub type Vertex = u64;
    pub type Edge = (Vertex, Vertex);

    pub type VertexBag = HashSet<Vertex>;


    pub mod new_ntd {
        use std::cmp::max;
        use std::collections::{HashMap, HashSet};
        use std::env::current_exe;
        use std::process::Child;
        use petgraph::matrix_graph::NodeIndex;

        type TreeNode = u64;
        pub(crate) type Vertex = NodeIndex;
        pub(crate) type Bag = HashSet<Vertex>;

        #[derive(PartialEq, Eq, Debug)]
        pub enum NodeType {
            Leaf,
            Introduce,
            Forget,
            Join
        }

        /*
        This structure contains data of a single node
         */
        #[derive(PartialEq, Eq, Debug)]
        pub struct NodeData {
            node_type: NodeType,
            bag: Bag,
        }

        impl NodeData {
            /*
            Simple constructor for NodeData
            */
            pub fn new(node_type: NodeType, bag: Bag) -> NodeData {
                NodeData {
                    node_type,
                    bag
                }
            }

            /*
            Returns a reference to the Node Type of this NodeData
             */
            pub fn node_type(&self) -> &NodeType { &self.node_type }

            /*
            Returns a reference to the bag
             */
            pub fn bag(&self) -> &Bag { &self.bag }
        }

        /*
        This structure contains the data of a nice tree decomposition

        Nodes: 0,1,...,N-1
         */
        #[derive(PartialEq, Eq, Debug)]
        pub struct TreeStructure {
            number_of_nodes: TreeNode,
            node_data: HashMap<TreeNode, NodeData>,
            children_list: HashMap<TreeNode, Vec<TreeNode>>,
            parents_list: HashMap<TreeNode, TreeNode>
        }

        impl TreeStructure {
            /*
            Returns an empty TreeAdjacency Structure with given node size
             */
            pub fn new(number_of_nodes: u64) -> TreeStructure {
                if number_of_nodes == 0 { panic!("Tree needs at least one node"); }

                TreeStructure {
                    number_of_nodes,
                    node_data: HashMap::new(),
                    children_list: HashMap::new(),
                    parents_list: HashMap::new(),
                }
            }

            /*
            returns the bag of a given node
             */
            pub fn bag(&self, node : TreeNode) -> Option<&Bag>{
                if let Some(nd) = self.node_data.get(&node){
                    Some(&nd.bag)
                }
                else { None }
            }

            /*
            returns the node type of a given node
             */
            pub fn node_type(&self, node : TreeNode) -> Option<&NodeType>{
                if let Some(nd) = self.node_data.get(&node) {
                    Some(&nd.node_type)
                }
                else { None }
            }


            /*
            Returns the number of nodes of the tree decomposition
             */
            pub fn node_count(&self) -> TreeNode {
                self.number_of_nodes
            }

            /*
            returns the children of a given node
             */
            pub fn children(&self, node: TreeNode) -> Option<&Vec<TreeNode>> {
                self.children_list.get(&node)
            }

            /*
            counts the number of children of a given node
             */
            pub fn children_count(&self, node: TreeNode) -> TreeNode {
                (if let Some(i) = self.children(node) {
                    i.len()
                } else {
                    0
                }) as TreeNode
            }

            /*
            returns the parent of a given node
             */
            pub fn parent(&self, node: TreeNode) -> Option<&TreeNode> {
                self.parents_list.get(&node)
            }


            /*
            Checks if child node relation ship
             */
            pub fn is_parent_of(&self, parent: TreeNode, child: TreeNode) -> bool {
                if let Some(p) = self.parent(child) {
                    (parent == *p)
                } else {
                    false
                }
            }

            /*
            adds the parent-child-relation ship if possible:
            - Only one parent
            - Number of parents depend on the kind of node
             */
            pub fn set_child(&mut self, parent: TreeNode, child: TreeNode) {

                // Checks if child already has a parent
                if self.parent(child) != None {
                    panic!("Node {:?} already has a parent", child);
                }
                // Checks if node_types have been considered correctly
                match self.node_data.get(&parent).unwrap().node_type {
                    NodeType::Leaf => { panic!("{:?} could not have children; its a leaf!", parent) },
                    NodeType::Introduce | NodeType::Forget => {
                        if self.children(parent) != None {
                            panic!("{:?} already has a child!\
                             Introduce & Forget Nodes can have only one child!", parent);
                        }
                    },
                    NodeType::Join => {
                        if self.children_count(parent) > 1 {
                            panic!("{:?} already has two ore more childs. \
                            Join Nodes can have a maximum of 2 children", parent);
                        }
                    },
                }

                // Checks if the given nodes do not already have the child-parent relationship
                if !self.is_parent_of(child, parent) {
                    // sets the parent
                    self.parents_list.insert(child, parent);

                    // inserts child
                    if let Some(children) = self.children_list.get_mut(&parent) {
                        children.push(child);
                    } else {
                        self.children_list.insert(parent, vec![child]);
                    }
                }
            }

            /*
            Sets the node_data of given node, node_type and the bag
             */
            pub fn set_node_data(&mut self, node: TreeNode, node_type : NodeType, bag : Bag)
            {
                self.node_data.insert(node, NodeData::new(node_type, bag));
            }

            /*
            calculates the root of the tree
             */
            pub fn root(&self) -> TreeNode {
                let mut current_node: TreeNode = 0; // arbitrary taken node with index = 0
                loop {
                    if let Some(parent) = self.parent(current_node)
                    {
                        current_node = *parent;
                    } else { break }
                }
                current_node
            }
        }

        /*
        A structure organizing and containing all useful methods for the
        Nice Tree Decomposition
         */
        pub struct NiceTreeDecomposition{
            tree_structure : TreeStructure,
        }

        impl NiceTreeDecomposition{

            /*
            Simple constructor for the NiceTreeDecomposition
             */
            pub fn new(tree_structure : TreeStructure) -> NiceTreeDecomposition{
                NiceTreeDecomposition{
                    tree_structure
                }
            }

            /*
            Returns the bag of a given tree node
             */
            pub fn bag(&self, node : TreeNode) -> Option<&Bag>{
                self.tree_structure.bag(node)
            }

            /*
            Returns the node type of a given node
             */
            pub fn node_type(&self, node : TreeNode) -> Option<&NodeType>{
                self.tree_structure.node_type(node)
            }

            pub fn stingy_ordering(&self) -> Vec<TreeNode>{
                self.recursive_stingy_ordering(self.tree_structure.root()).0
            }

            /*
            recursively calculating stingy ordering by returning (stingy_ordering, branch_number)
             */
            pub fn recursive_stingy_ordering(&self, current_node: TreeNode) -> (Vec<TreeNode>, TreeNode)
            {
                let mut stingy_order : Vec<TreeNode>= Vec::new();
                let mut branch_number : TreeNode = 0;

                match self.node_type(current_node){
                    Some(NodeType::Leaf) => (), // vertex will be pushed later and branch number is already 0
                    Some(NodeType::Introduce) | Some(NodeType::Forget) => {
                        if let Some(children) = self.tree_structure.children(current_node){
                            if let Some(child) = children.get(0){
                                let (so, bn) = self.recursive_stingy_ordering(*child);
                                stingy_order = so;
                                branch_number = bn;
                            }
                        }
                    },
                    Some(NodeType::Join) => {
                        if let Some(children) = self.tree_structure.children(current_node){

                            let mut child1 = children.get(0).unwrap();
                            let mut child2 = children.get(1).unwrap();

                            let (mut so1, bn1) = self.recursive_stingy_ordering(*child1);
                            let (mut so2, bn2) = self.recursive_stingy_ordering(*child2);

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
                    },
                    None => ()
                }

                // inserting current vertex at last
                stingy_order.push(current_node);

                // return result
                (stingy_order, branch_number)
            }
        }



    }



    /*
    pub mod nice_tree_decomposition{
        use std::ptr::hash;
        use crate::graph_structures::graph_structures::adjacency::AdjList;
        use super::*;

        #[derive(Debug,PartialEq)]
        pub enum NodeType{
            Leaf(VertexBag),
            Introduce(VertexBag),
            Forget(VertexBag),
            Join(VertexBag)
        }



        #[derive(Debug, PartialEq)]
        pub struct NiceTreeDecomposition{
            pub adjacency_list : AdjList,
            pub node_data : HashMap<Vertex, NodeType>,
            pub root : Vertex
        }

        impl NiceTreeDecomposition {
            /*
            creates a tree decomposition given
            -> adjacency List
            -> node_data
            -> root
             */
            pub fn new(adjacency_list: AdjList, node_data: HashMap<Vertex, NodeType>, root: Vertex) -> NiceTreeDecomposition {
                NiceTreeDecomposition { adjacency_list, node_data, root }
            }

            /*
            gets the node data
             */
            pub fn get_node_data(&self, v: &Vertex) -> Option<&NodeType>{
                self.node_data.get(v)
            }

            /*
            returns the reference to the bag of the given node
            */
            pub fn get_bag(&self, v : &Vertex) -> Option<&VertexBag>{
                match self.get_node_data(v){
                    None => None,
                    Some(NodeType::Leaf(n)) => Some(&n),
                    Some(NodeType::Introduce(n)) => Some(&n),
                    Some(NodeType::Forget(n)) => Some(&n),
                    Some(NodeType::Join(n)) => Some(&n),
                }
            }

            /*
            This function calculates a single branch number for a node
             by summing up the branch numbers of its children
             and eventually adding +1 for join nodes
             */
            pub fn calculate_single_branch_number(&self, current_node : &Vertex) -> Vertex
            {
                if let Some(out_neighbours) = self.adjacency_list.out_neighbours(*current_node)
                {
                    let mut sum =  out_neighbours.iter().map(|cur| self.calculate_single_branch_number(cur)).sum();
                    if let Some(NodeType::Join(_)) = self.get_node_data(&current_node){ sum += 1 }
                    sum
                }
                else {
                    0
                }
            }
            /*
            Calculates the branch number for each
            TODO: Make it more efficient!
             */
            pub fn calculate_branch_numbers_naive(&self) -> HashMap<Vertex, Vertex>{
                let mut result: HashMap<Vertex,Vertex> = HashMap::new();
                for j in 1..(self.adjacency_list.number_of_vertices() + 1){
                    result.insert(j, self.calculate_single_branch_number(&j));
                }
                result
            }

            /*
            This function returns a stingy ordering of the tree decomposition
             */
            pub fn stingy_ordering(&self) -> Vec<Vertex>
            { self.recursive_stingy_ordering(self.get_root()) }

            /*
            This function calculates the stingy ordering recursively
            -> for explanation see thesis
             */
            pub fn recursive_stingy_ordering(&self, current_vertex : Vertex) -> Vec<Vertex>
            {
                let mut result: Vec<Vertex>= Vec::new();

                // compares degree of nodes
                match self.adjacency_list.out_degree(current_vertex){
                    1 => {
                        let child = self.adjacency_list.out_neighbours(current_vertex).unwrap().get(0).unwrap();
                        result = self.recursive_stingy_ordering(*child);
                    },
                    2 =>{
                        let mut child1 = self.adjacency_list.out_neighbours(current_vertex).unwrap().get(0).unwrap();
                        let mut child2 = self.adjacency_list.out_neighbours(current_vertex).unwrap().get(1).unwrap();

                        let mut order1 = self.recursive_stingy_ordering(*child1);
                        let mut order2 = self.recursive_stingy_ordering(*child2);

                        if self.calculate_single_branch_number(child1) >= self.calculate_single_branch_number(child2){
                            result = order1;
                            result.append(&mut order2);
                        }
                        else {
                            result = order2;
                            result.append(&mut order1);
                        }
                    },
                    _=>(),
                }

                result.push(current_vertex);
                result
            }

            /*
           clones the bag of the given vertex
             */
            pub fn get_bag_clone(&self, v : &Vertex) -> VertexBag
            {
                match self.get_node_data(v){
                    None => VertexBag::new(),
                    Some(NodeType::Leaf(n)) => n.clone(),
                    Some(NodeType::Introduce(n)) => n.clone(),
                    Some(NodeType::Forget(n)) => n.clone(),
                    Some(NodeType::Join(n)) => n.clone(),
                }
            }

            /*
            Returns the union of all bags of the subtree rooted at the given node
             */
            pub fn get_union(&self, v : &Vertex) -> VertexBag
            {
                let children = self.adjacency_list.out_neighbours(*v);
                let mut union: VertexBag = self.get_bag_clone(v);

                match children {
                    Some(vec) => {
                        for i in vec{
                            union.extend(&self.get_union(i));
                        }
                    },
                    None => (),
                }
                union
            }

            /*
            Returns the root of the nice tree decomposition
             */
            pub fn get_root(&self) -> Vertex{
                self.root
            }

        }


    }

}

     */
}
#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use petgraph::matrix_graph::NodeIndex;
    use crate::graph_structures::graph_structures::new_ntd::{NiceTreeDecomposition, NodeData, NodeType, TreeStructure};
    use crate::graph_structures::graph_structures::new_ntd::NodeType::{Forget, Introduce, Join, Leaf};
    use crate::graph_structures::graph_structures::new_ntd::{Vertex, Bag};

    fn tree_adjacency_example_one() -> TreeStructure {
        let mut ta = TreeStructure::new(10);
        ta.set_node_data(0, Leaf, Bag::from([Vertex::new(1)]));
        ta.set_node_data(1,
                         Introduce, Bag::from([Vertex::new(1), Vertex::new(2)]));
        ta.set_node_data(2,
                         Forget, Bag::from([Vertex::new(2)]));
        ta.set_node_data(3, Leaf, Bag::from([Vertex::new(2)]));
        ta.set_node_data(4, Introduce, Bag::from([Vertex::new(1), Vertex::new(1)]));
        ta.set_node_data(5,Forget, Bag::from([Vertex::new(2)]));
        ta.set_node_data(6, Join, Bag::from([Vertex::new(2)]));
        ta.set_node_data(7, Introduce, Bag::from([Vertex::new(2), Vertex::new(4)]));
        ta.set_node_data(8, Forget, Bag::from([Vertex::new(4)]));
        ta.set_node_data(9, Forget, Bag::from([]));

        ta.set_child(9,8);
        ta.set_child(8,7);
        ta.set_child(7,6);
        ta.set_child(6,2);
        ta.set_child(2,1);
        ta.set_child(1,0);
        ta.set_child(6,5);
        ta.set_child(5,4);
        ta.set_child(4,3);

        ta
    }

    #[test]
    fn test_stingy_ordering(){
        let test_object = tree_adjacency_example_one();
        let test_ntd = NiceTreeDecomposition::new(test_object);
        assert_eq!(test_ntd.stingy_ordering(),vec![0,1,2,3,4,5,6,7,8,9]);

        // with example 2
        /*
        let ntd = create_ntd_from_file("data/nice_tree_decompositions/example_2.ntd").unwrap();
        assert_eq!(ntd.stingy_ordering(),vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13]);
         */
    }

    #[test]
    fn test_tree_structure(){
        let test_object = tree_adjacency_example_one();

        assert_eq!(test_object.node_count(), 10);


        // test children for each node type
        assert_eq!(test_object.children(0), None); // Leaf
        assert_eq!(test_object.children(7), Some(&vec![6])); // Introduce
        assert_eq!(test_object.children(2), Some(&vec![1])); // Forget
        assert_eq!(test_object.children(6), Some(&vec![2,5])); // Join

        // test parent for each node type and the root
        assert_eq!(test_object.parent(9), None); // root
        assert_eq!(test_object.parent(0), Some(&(1 as u64))); // leaf
        assert_eq!(test_object.parent(4), Some(&(5 as u64))); // Introduce
        assert_eq!(test_object.parent(8), Some(&(9 as u64))); // Forget
        assert_eq!(test_object.parent(6), Some(&(7 as u64))); // Join

        // test children count
        assert_eq!(test_object.children_count(6), 2);
        assert_eq!(test_object.children_count(0), 0);
        assert_eq!(test_object.children_count(2), 1);

        // test is_child_of
        assert!(test_object.is_parent_of(6, 2));
        assert!(test_object.is_parent_of(6, 5));
        assert!(test_object.is_parent_of(9, 8));
        assert!(!test_object.is_parent_of(9, 0));
        assert!(!test_object.is_parent_of(2, 6));

        // test root
        assert_eq!(test_object.root(), 9);

        // test bag
        assert_eq!(test_object.bag(0), Some(&HashSet::from([NodeIndex::new(1)])));
        assert_eq!(test_object.bag(7), Some(&HashSet::from([NodeIndex::new(2), NodeIndex::new(4)])));
        assert_eq!(test_object.bag(6), Some(&HashSet::from([NodeIndex::new(2)])));
        assert_eq!(test_object.bag(9), Some(&HashSet::from([])));
        assert_eq!(test_object.bag(8), Some(&HashSet::from([NodeIndex::new(4)])));

        // test node_type()
        assert_eq!(test_object.node_type(0), Some(&NodeType::Leaf));
        assert_eq!(test_object.node_type(4), Some(&NodeType::Introduce));
        assert_eq!(test_object.node_type(8), Some(&NodeType::Forget));
        assert_eq!(test_object.node_type(6), Some(&NodeType::Join));

    }
}
