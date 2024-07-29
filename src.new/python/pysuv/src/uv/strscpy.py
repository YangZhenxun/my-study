def uv__strscpy(d: str, s: str, n: int) -> int|Exception:
    i: int = 0

    for i in range(n):
        d[i] = s[i]
        if ('\0' == d[i]):
            return i

    if (i == 0):
        return 0

    i -= 1
    d[i] = '\0'

    raise ValueError("argument list too long")
