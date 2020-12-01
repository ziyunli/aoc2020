def parse_numbers(lines):
    return [int(line) for line in lines]


def find_entries(nums):
    found = set()
    for n in nums:
        if (2020 - n) in found:
            return n, 2020 - n
        found.add(n)


def find_triple(nums):
    l = len(nums)
    for i in range(0, l-2):
        for j in range(1, l-1):
            for k in range(2, l):
                if nums[i] + nums[j] + nums[k] == 2020:
                    return nums[i], nums[j], nums[k]


if __name__ == '__main__':
    f = open('../input/01.txt', 'r')
    nums = parse_numbers(f.readlines())
    # part 1
    # pair = find_entries(nums)
    # print(pair)
    # print(pair[0] * pair[1])
    triple = find_triple(nums)
    print(triple, triple[0] * triple[1] * triple[2])
