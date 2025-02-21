def choosek(L, k):
    n = len(L)
    assert (n >= k)
    if k == 0:
        yield []
    elif k == n:
        yield L
    else:
        for s in choosek(L[1:], k-1):
            yield [L[0]] + s
        for s in choosek(L[1:], k):
            yield s


def choosek_gray(L, k):
    n = len(L)
    assert (n >= k)
    if k == 0:
        yield []
    elif k == n:
        yield L
    else:
        for s in choosek_gray(L[1:], k-1):
            yield [L[0]] + s
        for s in reversed([x for x in choosek_gray(L[1:], k)]):
            yield s


if __name__ == "__main__":
    print([x for x in choosek_gray(list(range(0, 5)), 3)])
