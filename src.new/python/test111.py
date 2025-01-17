import asyncio
def run():
    i = 1
    n = i**2
    while n < 221:
        i += 1
        n = i**2
    return i

_run = run()
print(_run)
