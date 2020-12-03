# frozen_string_literal: true

POLICY_FORMAT = /(?<lo>\d+)-(?<hi>\d+) (?<char>[a-z]): (?<password>[a-z]+)/.freeze

Policy = Struct.new(:password, :char, :lo, :hi)

def valid_freq?(policy)
  policy.lo <= policy.password.count(policy.char) && policy.password.count(policy.char) <= policy.hi
end

def valid_occurrence?(policy)
  (policy.password[policy.lo - 1] == policy.char) ^ (policy.password[policy.hi - 1] == policy.char)
end

def parse_policy(line)
  caps = line.match POLICY_FORMAT
  Policy.new(caps['password'], caps['char'], caps['lo'].to_i, caps['hi'].to_i)
end

open('../input/02.txt') do |f|
  counts = f.each_with_object({ a: 0, b: 0 }) do |line, hash|
    policy = parse_policy(line.chomp)
    hash[:a] += 1 if valid_freq? policy
    hash[:b] += 1 if valid_occurrence? policy
    hash
  end
  p counts
end
