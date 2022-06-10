# Generates path nice tree decomposition where only the parameter |E_tau| varies
# width <= 2
# vertices := n
# nodes = 2n ?
# ntd with |E_tau| from n to 2n - 1
# adjacency stays the same


FILE_PATH = '../data/nice_tree_decompositions/e_tau_modifying_paths/'


# Creates e_tau modyfying path of length n and with i additional edges
# 0 <= i < n
def e_tau_modifying_path(n,i):
    output = "# auto generated nice tree decomposition with " + str(n) + " vertices and " + str(n + i) +  " possible edges. \n"
    output += "s " + str(2*n) + " 2 " + str(n) + "\n"
    output += "n 1 l 1\n"

    vertex_counter = 1
    node_counter = 2

    for j in range(1, n):
        if vertex_counter <= i:
            output += "n " + str(node_counter) + " i " + str(vertex_counter) + " " + str(vertex_counter+1) + "\n"
            node_counter += 1
            output += "n " + str(node_counter) + " f " + str(vertex_counter + 1) + "\n"
            node_counter += 1
            vertex_counter += 1
        else:
            output += "n " + str(node_counter) + " f" + "\n"
            node_counter += 1
            output += "n " + str(node_counter) + " i " + str(vertex_counter + 1) + "\n"
            node_counter += 1
            vertex_counter += 1

    output += "n " + str(node_counter) + " f\n"

    for i in range(1, node_counter):
        output += "a " + str(i + 1) + " " + str(i)
        if i < node_counter - 1:
            output += "\n"

    return output

# a function that exports the generated graphs into files
def text_to_file(text, file_name):
    f = open(file_name, "w")
    f.write(text)
    f.close()


def e_tau_modifying_paths(n):

    for i in range(0,n):
        text = e_tau_modifying_path(n,i)
        filename = "e_tau_modifying_path_" + str(n) + "_" + str(i) + ".ntd"
        text_to_file(text, FILE_PATH + filename)


for j in range(2,14):
    e_tau_modifying_paths(j)