# Counting Homomorphisms in Rust

## input Format for graphs



## Input Format for NTD

Here is an example for the input format, representing the following
nice tree decomposition. Format has been inspired by 
[this format for tree decompositions](https://github.com/PACE-challenge/Treewidth#output-format)

<img src="secondary_ressources/example_graph_ntd_format.jpg" width="200">

- Consisting of three parts:
1. with `s` at the beginning: This line describes the main parameter of 
this nice tree decomposition : number of nodes, width + 1 & number of 
vertices of the original graph.
2. wit `n` at the beginning: These line describe the nodes. The first argument
stands for the ID of the node, the second argument describes the sort of node:
   1. `l` : Leaf Node
   2. `i` : Introduce Node
   3. `f` : Forget Node
   4. `j` : Join Node
3. with `a` at the beginning: Describe adjacency of the tree nodes. For example
`a 7 3` means there is an edge from node `7` to node `3` i. e. that node `3` is the 
child of node `7`.

```
s 10 2 4
n 1 l 1
n 2 i 1 2
n 3 f 2
n 4 l 2
n 5 i 2 3
n 6 f 2
n 7 j 2
n 8 i 2 4
n 9 f 4
n 10 f
a 2 1
a 3 2
a 7 3
a 5 4
a 6 5
a 7 6
a 8 7
a 9 8
a 10 9
```

Note that the indices in the file go from 1 to N while the internal representation consists of indices 0 to N-1.

## How to run the Experiments

1. clone the complete repository. Test data is already included.
2. Run the command `cargo test` in the main project folder to run unit tests.
3. Run the command `cargo run --release` in the main project folder to start tests.
4. finish
5. To visualize the results use the `evaluation.ipynb` file, which can 
be executed with jupyter-lab and immediately shows the results. Make sure, you have installed
the `seaborn` python package to run the code.