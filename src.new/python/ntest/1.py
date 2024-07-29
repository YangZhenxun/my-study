def idk(bls: list[tuple[int]], whs: list[tuple[int]]):
    z = blks(bls)
    whts(whs)
    return z

def blks(bls: list[tuple[int]]):
    x: list[int] = [x for x, y in bls]
    y: list[int] = [y for x, y in bls]
    x.sort()
    y.sort()
    i1 = 1
    ii1 = 1
    for i in x[:len(x)-1]:
        if (i + 1) in x:
            locals()["i" + str(ii1)] = locals()["i" + str(ii1)] + 1
        else:
            ii1 += 1
            locals()["i" + str(ii1)] = 1
    return locals()

def whts(whs: list[tuple[int]]) -> None:
    return None

print(globals())
print(blks([(1, 1), (2, 1), (3,1), (4,1), (5,1), (7,1), (8,1), (9,1), (10,1)]))
