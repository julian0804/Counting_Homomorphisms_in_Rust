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
         */
        #[derive(PartialEq, Eq, Debug)]
        pub struct TreeAdjacency {
            number_of_nodes: TreeNode,
            node_data: HashMap<TreeNode, NodeData>,
            children_list: HashMap<TreeNode, Vec<TreeNode>>,
            parents_list: HashMap<TreeNode, TreeNode>
        }

        impl TreeAdjacency {
            /*
            Returns an empty TreeAdjacency Structure with given node size
             */
            pub fn new(number_of_nodes: u64) -> TreeAdjacency {
                if number_of_nodes == 0 { panic!("Tree needs at least one node"); }

                TreeAdjacency {
                    number_of_nodes,
                    node_data: HashMap::new(),
                    children_list: HashMap::new(),
                    parents_list: HashMap::new(),
                }
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
            pub fn is_child_of(&self, child: TreeNode, parent: TreeNode) -> bool {
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
                if !self.is_child_of(child, parent) {
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
    }


    pub mod adjacency {
        use super::*;
        /*
        Implementation of a simple adjacency list for a directed graph with possible loops
        but without multi-edges

        Vertices: 1 ... N

        TODO: This structure is currently only needed by the tree decomposition function
        TODO: Do some better tree structure with more checking...

        TODO: create checker

        */
        #[derive(Debug, PartialEq)]
        pub struct AdjList {
            pub adjacency_list: HashMap<Vertex, Vec<Vertex>>,
            // safes all out-going edges
            pub reversed_adjacency_list: HashMap<Vertex, Vec<Vertex>>, // safes all in-going edges
        }

        impl AdjList {
            /*
           Returns an empty AdjList
            */
            pub fn new() -> AdjList {
                AdjList { adjacency_list: HashMap::new(), reversed_adjacency_list: HashMap::new() }
            }

            /*
            Returns the number of edges in the adjacency list
             */
            pub fn number_of_edges(&self) -> usize
            {
                self.adjacency_list.iter().map(|(i, v)| v.len()).sum()
            }

            /*
            returning the number of vertices by searching the maximum key in the adjacency list, but not how many vertices are in the graph
             */
            pub fn number_of_vertices(&self) -> Vertex
            {
                *self.adjacency_list.keys().max().unwrap_or(&0)
            }

            /*
            returning a vector of all vertices that has at least one edge going in or out
             */
            /*
            pub fn connected_vertices(&self) -> Vec<&Vertex>{
                let l1 = HashSet::from(self.adjacency_list.keys().collect());
                let l2 = HashSet::from(self.reversed_adjacency_list.keys().collect());
                l1.union(&l2).collect()
            }
             */

            /*
            Checks if the edge (from, to) exists in this adjacency list
             */
            pub fn check_edge(&self, from: Vertex, to: Vertex) -> bool {
                if self.adjacency_list.get(&from) == None {
                    return false;
                }
                self.adjacency_list.get(&from).unwrap().contains(&to)
            }
            /*
            inserts edge if not already available
             */
            pub fn insert_edge(&mut self, from: Vertex, to: Vertex) {
                // Checks if edge already exists
                if !self.check_edge(from, to) {

                    // Inserts edge into the adjacency_list
                    // Checks if we already have an edge going out of "from"
                    if let Some(out_list) = self.adjacency_list.get_mut(&from) {
                        out_list.push(to);
                    } else {
                        self.adjacency_list.insert(from, vec![to]);
                    }


                    // Inserts edge into the reversed_adjacency_list
                    // Checks if we already have an edge going in "to"
                    if let Some(in_list) = self.reversed_adjacency_list.get_mut(&to)
                    {
                        in_list.push(from);
                    } else {
                        self.reversed_adjacency_list.insert(to, vec![from]);
                    }
                }
            }


            /*
            insert multiple edges given by a Vector of tuples
             */
            pub fn insert_edges(&mut self, edges: Vec<(Vertex, Vertex)>)
            {
                for (a, b) in edges {
                    self.insert_edge(a, b);
                }
            }

            /*
            Returns an &Vec<Vertex> of the out neighbours if the vertex "from" has some, None otherwise
             */
            pub fn out_neighbours(&self, from: Vertex) -> Option<&Vec<Vertex>>
            {
                self.adjacency_list.get(&from)
            }

            /*
            Returns a Vector of neighbours going in
             */
            pub fn in_neighbours(&self, to: Vertex) -> Option<&Vec<Vertex>> {
                self.reversed_adjacency_list.get(&to)
            }

            /*
            Returns the out degree of the vertex "from"
             */
            pub fn out_degree(&self, from: Vertex) -> usize
            {
                if let Some(i) = self.out_neighbours(from) { i.len() } else { 0 }
            }

            /*
            Returns the in degree of the vertex "from"
             */
            pub fn in_degree(&self, to: Vertex) -> usize
            {
                if let Some(i) = self.in_neighbours(to) { i.len() } else { 0 }
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
    use crate::graph_structures::graph_structures::new_ntd::{NodeData, TreeAdjacency};
    use crate::graph_structures::graph_structures::new_ntd::NodeType::{Forget, Introduce, Join, Leaf};
    use crate::graph_structures::graph_structures::new_ntd::{Vertex, Bag};

    /*
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

        assert_eq!(adjlist.number_of_vertices(), 3);
        assert_eq!(adjlist.out_neighbours(2), None);
        assert_eq!(adjlist.out_degree(2), 0);

        assert_eq!(*(adjlist.out_neighbours(1).unwrap()), vec![2,3]);
        assert_ne!(*(adjlist.out_neighbours(1).unwrap()), vec![4]);
        assert_ne!(*(adjlist.out_neighbours(1).unwrap()), vec![3,2]);

        let mut adjlist1 = AdjList::new();
        assert_eq!(adjlist1.number_of_vertices(),0);

        // test cases to check in_neighbours
        assert_eq!(adjlist.in_neighbours(1), None);
        assert_eq!(*(adjlist.in_neighbours(2).unwrap()), vec![1,3]);
        assert_eq!(*(adjlist.in_neighbours(3).unwrap()), vec![1]);

        // test cases to check in_degree
        assert_eq!(adjlist.in_degree(1),0);
        assert_eq!(adjlist.in_degree(2),2);
        assert_eq!(adjlist.in_degree(3),1);

    }

     */
    fn tree_adjacency_example_one() -> TreeAdjacency{
        let mut ta = TreeAdjacency::new(10);
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
    fn test_tree_adjacency(){
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
        assert!(test_object.is_child_of(2,6));
        assert!(test_object.is_child_of(5,6));
        assert!(test_object.is_child_of(8,9));
        assert!(!test_object.is_child_of(0,9));
        assert!(!test_object.is_child_of(6,2));

        // test root
        assert_eq!(test_object.root(), 9);
    }

/*
    #[test]
    fn tree_decomposition()
    {
        // Uses the tree from github
        let mut node_data: HashMap<Vertex, NodeType> = HashMap::new();
        node_data.insert(1, NodeType::Leaf(VertexBag::from([1])));
        node_data.insert(2, NodeType::Introduce(VertexBag::from([1,2])));
        node_data.insert(3, NodeType::Forget(VertexBag::from([2])));
        node_data.insert(4, NodeType::Leaf(VertexBag::from([2])));
        node_data.insert(5, NodeType::Introduce(VertexBag::from([2,3])));
        node_data.insert(6, NodeType::Forget(VertexBag::from([2])));
        node_data.insert(7, NodeType::Join(VertexBag::from([2])));
        node_data.insert(8, NodeType::Introduce(VertexBag::from([2,4])));
        node_data.insert(9, NodeType::Forget(VertexBag::from([4])));
        node_data.insert(10, NodeType::Forget(VertexBag::from([])));

        let mut adj_list = AdjList::new();
        adj_list.insert_edge(2,1);
        adj_list.insert_edge(3,2);
        adj_list.insert_edge(7,3);
        adj_list.insert_edge(5,4);
        adj_list.insert_edge(6,5);
        adj_list.insert_edge(7,6);
        adj_list.insert_edge(8,7);
        adj_list.insert_edge(9,8);
        adj_list.insert_edge(10,9);

        let mut adj_list_2 = AdjList::new();
        adj_list_2.insert_edges(vec![(2,1),(3,2),(7,3),(5,4),(6,5),(7,6),(8,7),(9,8),(10,9),]);
        assert_eq!(adj_list, adj_list_2);

        let treedecomp = NiceTreeDecomposition::new(adj_list, node_data, 10);

        assert_eq!(treedecomp.get_bag(&7), Some(&VertexBag::from([2])));
        assert_eq!(treedecomp.get_bag(&2), Some(&VertexBag::from([2,1])));
        assert_eq!(treedecomp.get_bag(&2), Some(&VertexBag::from([1,2])));
        assert_eq!(treedecomp.get_bag(&10), Some(&VertexBag::from([])));

        assert_eq!(treedecomp.get_node_data(&2), Some(&NodeType::Introduce(VertexBag::from([1,2]))));
        assert_eq!(treedecomp.get_union(&2), VertexBag::from([1,2]));


        assert_eq!(treedecomp.adjacency_list.out_neighbours(7), Some(&Vec::from([3,6])) );

        assert_eq!(treedecomp.get_union(&7), VertexBag::from([1,2,3]));
        assert_eq!(treedecomp.get_union(&10), VertexBag::from([1,2,3,4]));

        // Tests for calculating branch number
        assert_eq!(1, treedecomp.calculate_single_branch_number(&7));
        assert_eq!(1, treedecomp.calculate_single_branch_number(&8));
        assert_eq!(0, treedecomp.calculate_single_branch_number(&3));
        assert_eq!(0, treedecomp.calculate_single_branch_number(&4));

        // Tests for calculating all branch numbers
        let branch_numbers = HashMap::from([
            (1,0),
            (2,0),
            (3,0),
            (4,0),
            (5,0),
            (6,0),
            (7,1),
            (8,1),
            (9,1),
            (10,1),
        ]);

        assert_eq!(branch_numbers, treedecomp.calculate_branch_numbers_naive());
    }

    #[test]
    fn test_stingy_ordering() {
        let ntd = create_ntd_from_file("data/nice_tree_decompositions/example_2.ntd").unwrap();
        assert_eq!(ntd.stingy_ordering(),vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14]);

        let ntd = create_ntd_from_file("data/nice_tree_decompositions/example.ntd").unwrap();
        assert_eq!(ntd.stingy_ordering(),vec![1,2,3,4,5,6,7,8,9,10]);

    }

 */
}
