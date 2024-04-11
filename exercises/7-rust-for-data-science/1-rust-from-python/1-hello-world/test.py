import hello_py
import math

hello_py.say_hello()

fact = math.factorial(1024)
print(fact)

fact = hello_py.fact(1024)
print(fact)

point1 = hello_py.Point(1,1)
point2 = hello_py.Point(0,2)
print(point1.dist(point2))
print(point2)
point2.x = 3
print(point2.x)

hello_py.throws_error()