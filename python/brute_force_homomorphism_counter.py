# from https://stackoverflow.com/questions/68782863/graph-homomorphism-using-python

from itertools import product



def verify(G, H, f):
   homomorphism = True

   for edge in G:
       if not ((f[edge[0]], f[edge[1]]) in H):
           homomorphism = False
           break

   return homomorphism

def solve(G, H, n, m):
   rangeH = [i for i in range(m)]
   assignments = list(product(rangeH, repeat=n))
   cnt = 0

   for f in assignments:
       if verify(G, H, f):
           cnt += 1

   return cnt

"""
G = {(0,1),(1,0),(1,3),(3,1),(1,2),(2,1),(2,4),(4,2)}
H = {(0,1), (0,2), (0,3), (0,4),
     (1,0), (1,2), (1,3), (1,4),
     (2,0), (2,1), (2,3), (2,4),
     (3,0), (3,1), (3,2), (3,4),
     (4,0), (4,1), (4,2), (4,3)}"""

G = {(2,4),(4,2),(1,2),(2,1)}
H = {(0,1),(1,0),(1,2),(2,1),(2,3),(3,2),(0,3),(3,0)}
print("number:" + str(solve(G, H, 5, 4)))
