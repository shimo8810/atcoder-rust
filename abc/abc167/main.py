import numpy as np

with open('sample.txt', 'w') as f:
    N = 100
    K = 10
    A = np.random.randint(1, N + 1, N)
    A = ' '.join(map(str, A))

    f.write(f'{N} {K}\n')
    f.write(f'{A}\n')
