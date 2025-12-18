#import numpy as np
import random

lim: int = 100000000
Pi: float = 3.14159
valid: int = 0

for i in range(lim):
    if (random.uniform(0, 1)**2 + random.uniform(0, 1)**2) <= 1:
        #print(random.uniform(0, 1))
        valid += 1

aprox = valid*4/lim
err = (1-aprox/Pi)*100

print(f"Aproximation de Pi: {aprox}\n")
print(f"Real  valor  of Pi: {Pi}\n")
print(f"Error: {err}%")