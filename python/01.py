def numgen(f):
    for line in f:
        yield int(line)


def find_pair(nums):
    found = set()
    for n in nums:
        m = 2020 - n
        if m in found:
            return n, m
        found.add(n)


def find_triple(nums):
    sums = {}
    nums = [n for n in nums]
    length = len(nums)
    for i in range(0, length - 2):
        for j in range(1, length - 1):
            s = nums[i] + nums[j]
            if s not in sums:
                sums[s] = (nums[i], nums[j])
            else:
                continue
            for k in range(2, length):
                if (2020 - nums[k]) in sums:
                    return nums[i], nums[j], nums[k]


if __name__ == '__main__':
    f = open('../input/01.txt', 'r')

    # part 1
    # n, m = find_pair(numgen(f))
    # print(n, m, n * m)

    # part 2
    i, j, k = find_triple(numgen(f))
    print(i, j, k, i * j * k)
