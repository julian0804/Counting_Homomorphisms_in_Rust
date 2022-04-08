from itertools import product



def verify(G, H, f):
   homomorphism = True

   for edge in G:
       if not ((f[edge[0]], f[edge[1]]) in H):
           homomorphism = False
           break

   return homomorphism

def solve(G, H, n, m):
   rangeG = [i for i in range(n)]
   assignments = list(product(rangeG, repeat=m))
   cnt = 0

   for f in assignments:
       if verify(G, H, f):
           cnt += 1

   return cnt


G = {(0,1),(1,0),(1,3),(3,1),(1,2),(2,1),(2,4),(4,2)}
H = {(0,1), (0,2), (0,3), (0,4),
     (1,0), (1,2), (1,3), (1,4),
     (2,0), (2,1), (2,3), (2,4),
     (3,0), (3,1), (3,2), (3,4),
     (4,0), (4,1), (4,2), (4,3)}

print("number:" + str(solve(G,H,5,5)))
