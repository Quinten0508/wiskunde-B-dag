import math
import array
# start de timer voor een race tegen Rust:
# import time
# start_time = time.time()

n=1
lowerBound=-10000000
upperBound=10000000

list_x=[]    # definieert (en leegt) lijsten
list_a=[]
list_v=[]

print("Waardes tussen " + str(lowerBound) + " en " + str(upperBound) + " aan het berekenen...")
for x in (range(lowerBound,upperBound)):
    y=(1-5*x)/math.pi
    a = int(y)
    v=5*x+a*math.pi
    list_x.append(x)    # voegt alle waardes van x, a en v binnen het gegeven bereik toe aan de lijsten
    list_a.append(a)
    list_v.append(v)

print("Zoeken naar waardes die het dichtst bij " + str(n) + " liggen...")
if lowerBound*-1+upperBound >= 100000000:
    print("Dit kan eventjes duren")

closest = min(list_v, key=lambda x: abs(x-n))   # waarde vinden die het dichste bij n ligt
index = [x for x in range(len(list_v)) if list_v[x] == closest] # index vinden van die waarde in de list

if len(index) > 1:
    print("Er zijn meerdere opties die het dichtst bij " + str(n) + " liggen, omdat Python met 16 bit floats gebruikt.")

for z in range(len(index)):   # bij hoge bounds zijn er meerdere waardes die het "dichtst" bij n liggen (16 bit floats) dus printen we ze allemaal
    good=index[z]
    print( "------------------------------")
    print("Optie " + str(z+1))
    print(" uitkomst: " + str(closest))
    print(" x-waarde: " + str(list_x[good]))
    print(" y-waarde: " + str(list_a[good]))
print( "------------------------------")

# Beeindigt de timer:
# print("Dit programma heeft " + str(time.time() - start_time) + " seconden geduurd")