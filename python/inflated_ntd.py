# Create complete nice tree decomposition which provides all possible edges
# the following conditions hold for these kind of nice tree decompositions
# number of vertices = n is given
# number of possible edges is (n^2 + n) / 2
# width n - 1
# number of nodes = 2 * n


FILE_PATH = '../data/nice_tree_decompositions/benchmark_ntds/complete_ntds/'

# generates a ntd with all edges possible
# this may be a worst case instance
def complete_ntd(n):

    output = "# auto generated nice complete tree decomposition with " + str(n) + " vertices and " + str(int(n * (n-1) / 2 + n)) +  " possible edges.\n"
    output += "s " + str(2*n) + " 2 " + str(n) + "\n"
    output += "n 1 l 1\n"

    node_counter = 2

    for i in range(2,n+1):
        output += "n " + str(node_counter) + " i"
        for j in range(1,i+1):
            output += " " + str(j)
        output += "\n"

        node_counter += 1

    for i in range(n - 1,-1, -1):
        output += "n " + str(node_counter) + " f"
        for j in range(1,i+1):
            output +=  " " + str(j)
        output += "\n"

        node_counter += 1

    for i in range(1, node_counter - 1):
        output += "a " + str(i + 1) + " " + str(i)

        if i < node_counter - 2:
            output += "\n"

    return output


# a function that exports the generated graphs into files
def text_to_file(text, file_name):
    f = open(file_name, "w")
    f.write(text)
    f.close()


# generate complete nice tree decompositions for n to m vertices
def generate_complete_ntds(n,m):

    for i in range(n,m + 1):
        text = complete_ntd(i)
        filename = "ntd_complete_" + str(i) + ".ntd"
        text_to_file(text, FILE_PATH + filename)


generate_complete_ntds(2,8)