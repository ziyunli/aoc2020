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
    nums = [n for n in nums]
    length = len(nums)
    for i in range(0, length-1):
        found = set()
        rem = 2020 - nums[i]
        for j in range(i+1, length):
            if rem - nums[j] in found:
                return nums[i], nums[j], rem - nums[j]
            found.add(nums[j])


if __name__ == '__main__':
    f = open('../input/01.txt', 'r')

    # part 1
    # n, m = find_pair(numgen(f))
    # print(n, m, n * m)

    # part 2
    i, j, k = find_triple(numgen(f))
    print(i, j, k, i * j * k)
