



if __name__ == "__main__":
    f = open("../input/06.txt", "r")

    # count = 0
    # found = set()
    # for line in f:
    #     line = line.strip()
    #     if line == '':
    #         count += len(found)
    #         found = set()
    #     else:
    #         # Part 1
    #         found |= set([ch for ch in line])
    #         # Part 2
    #         found &= set([ch for ch in line])

    count = 0
    found = None
    for line in f:
        line = line.strip()
        if line == '':
            count += len(found)
            found = None
        else:
            if found is None:
                found = set([ch for ch in line])
            else:
                found &= set([ch for ch in line])

    count += len(found)
    print(f"Counts: {count}")
