import math
import numpy
from numpy import longlong

def mcf(a:int,b:int):
    l = 1
    if a < b:
        c = b
        d = a
    else:
        c = a
        d = b
    n = c%d
    if n == 0:
        return d
    else:
        try:
            andes = numpy.arange(2, d, dtype=int)
            print(andes)
        except Exception as e:
            andes = range(2, d)
            print(andes)
            print(e)
        for i in andes:
            if is_prime(i):
                while True:
                    if ((d%i) == 0) and (c%i == 0):
                        l *= i
                        d /= i
                        c /= i
                    else:
                        break
        if l != 1:
            return l
        else:
            return 0

def is_prime(x) -> bool:
    if x == 1:
        raise EOFError("")
    if (x == 2) or (x == 3) or (x == 5) or (x == 7) or (x == 11):
        return True
    if (x % 6 != 1) or (x % 6 != 5):
        return False
    if x % 2 == 0:
        return False
    try: andess = numpy.arange(numpy.longlong(5), numpy.longlong(math.sqrt(x))+1, numpy.longlong(6), longlong)
    except: andess = range(5, int(math.sqrt(x))+1, 6)
    for i in andess:
        if (x % i == 0) or (x % (i + 2) == 0):
            return False
    return True

a = int(input("请输入第一个数："))
b = int(input("请输入第二个数："))
print(mcf(a, b))
