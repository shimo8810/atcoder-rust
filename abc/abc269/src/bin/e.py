def main():
    N = int(input())
    head = 0
    tail = N
    mid = (head + tail) // 2
    while tail - head > 1:
        print(f"? {head + 1} {mid} 1 {N}", flush=True)

        if mid - head > int(input()):
            head = head
            tail = mid
        else:
            head = mid
            tail = tail
        mid = (head + tail) // 2
    ansx = tail

    head = 0
    tail = N
    mid = (head + tail) // 2
    while tail - head > 1:
        print(f"? 1 {N} {head + 1} {mid}", flush=True)

        if mid - head > int(input()):
            head = head
            tail = mid
        else:
            head = mid
            tail = tail
        mid = (head + tail) // 2
    ansy = tail

    print(f"! {ansx} {ansy}", flush=True)


if __name__ == "__main__":
    main()
