def validate(line):
    policy, password = [token.strip() for token in line.split(':')]
    rule, char = policy.split(' ')
    lo, hi = [int(token) for token in rule.split('-')]

    # part 1
    # freq = password.count(char)
    # return lo <= freq <= hi

    # part 2
    return (password[lo - 1] == char) ^ (password[hi - 1] == char)


if __name__ == '__main__':
    f = open('../input/02.txt', 'r')

    # part 1
    count = 0
    for line in f:
        if validate(line):
            count += 1
    print(f"{count}")
