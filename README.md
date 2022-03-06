# Counting Homomorphisms in Rust

## Input Format for NTD

Here is an example for the input format, representing the following
nice tree decomposition [need a sketch]

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
