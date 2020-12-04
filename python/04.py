import re


def parse_line(line, d):
    tokens = line.split(' ')
    for token in tokens:
        field, value = token.split(':')
        d[field] = value
    return d

def is_valid_hgt(hgt):
    val = int(hgt[:-2])
    if 'cm' in hgt:
        return 150 <= val <= 193
    else:
        return 59 <= val <= 76

def validate(passport):
    return (
            'byr' in passport and (1920 <= int(passport['byr']) <= 2002) and
            'iyr' in passport and (2010 <= int(passport['iyr']) <= 2020) and
            'eyr' in passport and (2020 <= int(passport['eyr']) <= 2030) and
            'hgt' in passport and is_valid_hgt(passport['hgt']) and
            'hcl' in passport and re.match(r'#[0-9a-z]{6}', passport['hcl']) and
            'ecl' in passport and re.match(r'amb|blu|brn|gry|grn|hzl|oth', passport['ecl']) and
            'pid' in passport and passport['pid'].isdigit() and len(passport['pid']) == 9
    )


if __name__ == '__main__':
    f = open('../input/04.txt', 'r')

    # part 1
    d = {}
    count = 0
    for line in f:
        line = line.strip()
        if line == '':
            if validate(d):
                count += 1
            d = {}
        else:
            d = parse_line(line, d)
    print(f"{count}")
