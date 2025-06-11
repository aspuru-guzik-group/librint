import numpy as np

def f(t):
    x, y = t[0], t[1]
    return 3*y*y*y*y + 6*y*x + x*x + 3

def df(t):
    x, y = t[0], t[1]
    return np.array([6*y + 2*x, 12*y*y*y + 6*x])


alpha = 1e-2
max_i = 1000
epsilon = 1e-3
delta = 1
i = 0

t = np.array([-5, -1])

while not (delta < epsilon or i == max_i):
    dt = df(t)
    t = t - alpha * dt
    delta = np.linalg.norm(dt)

    print("{0:.3f} {1:.3f} {2:.3f}".format(t[0], t[1], delta))

    i += 1

yopt = np.sqrt(6)/2
t1 = (-3*yopt, yopt)
t2 = (3*yopt, -yopt)
t3 = (0, 0)

print(t1, f(t1), df(t1))
print(t2, f(t2), df(t2))
print(t3, f(t3), df(t3))