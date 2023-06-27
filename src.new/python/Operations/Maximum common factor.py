def mcf(a:int,b:int):
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
        n1 = d%n
        while type(n1) != int:
            n = d%n
        return n1

a = int(input("请输入第一个数："))
b = int(input("请输入第二个数："))
print(mcf(a, b))
