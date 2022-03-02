/*
TODO: What about not passing a reference into NodeType but the Node itself...
*/

mod file_handler{
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    // function taken from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

}

pub mod nice_tree_decomposition{
    use std::collections::HashSet;

    pub type Vertex = u32;
    pub type Bag = HashSet<Vertex>;

    #[derive(Debug,PartialEq)]
    pub struct NiceTreeDecomposition<'a> {
        pub(crate) root : Node<'a>
    }

    #[derive(Debug, PartialEq)]
    pub enum NodeType<'a>{
        Leaf,
        Introduce(&'a Node<'a>),
        Forget(&'a Node<'a>),
        Join(&'a Node<'a>, &'a Node<'a>)
    }


    #[derive(Debug, PartialEq)]
    pub struct Node<'a>{
        bag : Bag,
        node_type : NodeType<'a>,
    }


    impl<'a> Node<'a>{
        /*
        Constructor of Node
         */
        pub fn new(bag : Bag, node_type : NodeType) -> Node{
            Node{
                bag,
                node_type
            }
        }
        /*
        returns the reference of the node´s bag
         */
        pub fn get_bag(&self) -> &Bag{
            &self.bag
        }
        /*
        returns a clone of the node´s bag
         */
        pub fn get_bag_clone(&self) -> Bag{
            self.bag.clone()
        }
        /*
        Returns children as an Option containing either a Vector of Node References
        or None if it is a leaf
         */
        pub fn get_children(&self) -> Option<Vec<&Node>>
        {
            match self.node_type{
                NodeType::Leaf => None,
                NodeType::Introduce(_c) => Some(vec![_c]),
                NodeType::Forget(_c) => Some(vec![_c]),
                NodeType::Join(_c1,_c2) => Some(vec![_c1,_c2])
            }
        }

        /*
        Returns the union of all bags in the subtree rooted at this node
         */
        pub fn union_bags(&self) -> Option<Bag>
        {
            // TODO: this function could be improved using vectors of bool for the bags
            match self.node_type{
                NodeType::Leaf => Some(self.get_bag_clone()),
                NodeType::Introduce(_c) => {
                    let mut result = _c.union_bags().unwrap();
                    result.extend(self.get_bag());
                    Some(result)
                },
                NodeType::Forget(_c) => {
                    _c.union_bags()
                },
                NodeType::Join(_c1,_c2) => {
                    let mut result = _c1.union_bags().unwrap();
                    result.extend(_c2.union_bags().unwrap());
                    Some(result)
                },
            }
        }
    }


}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::nice_tree_decomposition::*;

    #[test]
    fn test_union_bags()
    {
        // Creating test tree
        let l1 = Node::new(Bag::from([1]), NodeType::Leaf);
        let i1 = Node::new(Bag::from([1,2]), NodeType::Introduce(&l1));
        let f1 = Node::new(Bag::from([2]), NodeType::Forget(&i1));

        let l2 = Node::new(Bag::from([2]), NodeType::Leaf);
        let i2 = Node::new(Bag::from([2,3]), NodeType::Introduce(&l2));
        let i3 = Node::new(Bag::from([2,3,4]), NodeType::Introduce(&i2));
        let f2 = Node::new(Bag::from([2,4]), NodeType::Forget(&i3));
        let f3 = Node::new(Bag::from([2]), NodeType::Forget(&f2));

        let j1 = Node::new(Bag::from([2]),
                           NodeType::Join(&i1,&f3));
        let i4 = Node::new(Bag::from([2,5]), NodeType::Introduce(&j1));
        let f4 = Node::new(Bag::from([5]), NodeType::Forget(&i4));


        //#########################################################################
        // Test the union bags function
        assert_eq!(f4.union_bags(), Some(Bag::from([1,2,3,4,5])));
        assert_eq!(f3.union_bags(), Some(Bag::from([2,3,4])));
        assert_eq!(j1.union_bags(), Some(Bag::from([1,2,3,4])));
        assert_eq!(i4.union_bags(), Some(Bag::from([1,2,3,4,5])));
        assert_eq!(i3.union_bags(), Some(Bag::from([2,3,4])));

        // Test the getbag function
        assert_eq!(*l1.get_bag(), Bag::from([1]));
        assert_eq!(*i3.get_bag(), Bag::from([2,3,4]));
        assert_eq!(*j1.get_bag(), Bag::from([2]));
        assert_eq!(*f4.get_bag(), Bag::from([5]));

        // Test getChild
        // TODO: Test the getChild method...
    }
}



// Recursive implementation
/*

pub mod nice_tree_decomposition{
    use std::collections::HashSet;

    pub type Vertex = u32;
    pub type Bag = HashSet<Vertex>;

    #[derive(Debug,PartialEq)]
    pub struct NiceTreeDecomposition<'a> {
        pub(crate) root : Node<'a>
    }

    #[derive(Debug, PartialEq)]
    pub enum NodeType<'a>{
        Leaf,
        Introduce(&'a Node<'a>),
        Forget(&'a Node<'a>),
        Join(&'a Node<'a>, &'a Node<'a>)
    }


    #[derive(Debug, PartialEq)]
    pub struct Node<'a>{
        bag : Bag,
        node_type : NodeType<'a>,
    }


    impl<'a> Node<'a>{
        /*
        Constructor of Node
         */
        pub fn new(bag : Bag, node_type : NodeType) -> Node{
            Node{
                bag,
                node_type
            }
        }
        /*
        returns the reference of the node´s bag
         */
        pub fn get_bag(&self) -> &Bag{
            &self.bag
        }
        /*
        returns a clone of the node´s bag
         */
        pub fn get_bag_clone(&self) -> Bag{
            self.bag.clone()
        }
        /*
        Returns children as an Option containing either a Vector of Node References
        or None if it is a leaf
         */
        pub fn get_children(&self) -> Option<Vec<&Node>>
        {
            match self.node_type{
                NodeType::Leaf => None,
                NodeType::Introduce(_c) => Some(vec![_c]),
                NodeType::Forget(_c) => Some(vec![_c]),
                NodeType::Join(_c1,_c2) => Some(vec![_c1,_c2])
            }
        }

        /*
        Returns the union of all bags in the subtree rooted at this node
         */
        pub fn union_bags(&self) -> Option<Bag>
        {
            // TODO: this function could be improved using vectors of bool for the bags
            match self.node_type{
                NodeType::Leaf => Some(self.get_bag_clone()),
                NodeType::Introduce(_c) => {
                    let mut result = _c.union_bags().unwrap();
                    result.extend(self.get_bag());
                    Some(result)
                },
                NodeType::Forget(_c) => {
                    _c.union_bags()
                },
                NodeType::Join(_c1,_c2) => {
                    let mut result = _c1.union_bags().unwrap();
                    result.extend(_c2.union_bags().unwrap());
                    Some(result)
                },
            }
        }
    }


}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::nice_tree_decomposition::*;

    #[test]
    fn test_union_bags()
    {
        // Creating test tree
        let l1 = Node::new(Bag::from([1]), NodeType::Leaf);
        let i1 = Node::new(Bag::from([1,2]), NodeType::Introduce(&l1));
        let f1 = Node::new(Bag::from([2]), NodeType::Forget(&i1));

        let l2 = Node::new(Bag::from([2]), NodeType::Leaf);
        let i2 = Node::new(Bag::from([2,3]), NodeType::Introduce(&l2));
        let i3 = Node::new(Bag::from([2,3,4]), NodeType::Introduce(&i2));
        let f2 = Node::new(Bag::from([2,4]), NodeType::Forget(&i3));
        let f3 = Node::new(Bag::from([2]), NodeType::Forget(&f2));

        let j1 = Node::new(Bag::from([2]),
                           NodeType::Join(&i1,&f3));
        let i4 = Node::new(Bag::from([2,5]), NodeType::Introduce(&j1));
        let f4 = Node::new(Bag::from([5]), NodeType::Forget(&i4));


        //#########################################################################
        // Test the union bags function
        assert_eq!(f4.union_bags(), Some(Bag::from([1,2,3,4,5])));
        assert_eq!(f3.union_bags(), Some(Bag::from([2,3,4])));
        assert_eq!(j1.union_bags(), Some(Bag::from([1,2,3,4])));
        assert_eq!(i4.union_bags(), Some(Bag::from([1,2,3,4,5])));
        assert_eq!(i3.union_bags(), Some(Bag::from([2,3,4])));

        // Test the getbag function
        assert_eq!(*l1.get_bag(), Bag::from([1]));
        assert_eq!(*i3.get_bag(), Bag::from([2,3,4]));
        assert_eq!(*j1.get_bag(), Bag::from([2]));
        assert_eq!(*f4.get_bag(), Bag::from([5]));

        // Test getChild
        // TODO: Test the getChild method...
    }
}

 */