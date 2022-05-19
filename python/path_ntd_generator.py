# A simple script generating path looking nice tree decompositions
# where the following conditions hold
# number of vertices = n is given
# number of possible edges n + n - 1
# width = 1
# number of nodes = 2 * n


FILE_PATH = '../data/nice_tree_decompositions/benchmark_ntds/path_ntds/'

# simply generating a path like nice tree decomposition
def path_ntd(n):
    output = "# auto generated nice path tree decomposition with " + str(n) + " vertices and " + str((2 * n - 1)) +  " possible edges. \n"
    output += "s " + str(2*n) + " 2 " + str(n) + "\n"
    output += "n 1 l 1 \n"

    node_counter = 2

    for i in range(2,n+1):
        output += "n " + str(node_counter) + " i " + str(i-1) + " " + str(i) + "\n"
        node_counter += 1
        output += "n " + str(node_counter) + " f " + str(i) + "\n"
        node_counter += 1

    output += "n " + str(node_counter) + " f \n"

    for i in range(1, node_counter):
        output += "a " + str(i + 1) + " " + str(i) + "\n"

    return output

# a function that exports the generated graphs into files
def text_to_file(text, file_name):
    f = open(file_name, "w")
    f.write(text)
    f.close()


# generate path nice tree decompositions for n to m vertices
def generate_path_ntds(n,m):

    for i in range(n,m + 1):
        text = path_ntd(i)
        filename = "ntd_path_" + str(i) + ".ntd"
        text_to_file(text, FILE_PATH + filename)


generate_path_ntds(2,16)