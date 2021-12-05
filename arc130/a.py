import numpy as np

h,w,c,q = map(int, input().split())

np.zeros((h, w), dtype=np.uint32)
for i in range(q):
    t, n, c = map(int, input().split())
