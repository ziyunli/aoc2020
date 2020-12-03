# frozen_string_literal: true

require 'set'

def read_nums
  open('../input/01.txt') do |f|
    f.map { |line| line.chomp.to_i }
  end
end

def find_pair(nums)
  found = Set.new
  nums.each do |num|
    rem = 2020 - num
    break [num, rem] if found.include? rem

    found.add num
  end
end

def find_triple(nums)
  len = nums.length
  (0...(len - 1)).each do |i|
    found = Set.new
    ((i + 1)...len).each do |j|
      rem = 2020 - nums[i] - nums[j]
      return [nums[i], nums[j], rem] if found.include? rem

      found.add nums[j]
    end
  end
end

# part 1
a, b = find_pair(read_nums)
p "#{a} * #{b} = #{a * b}"

# part 2
x, y, z = find_triple(read_nums)
p "#{x} * #{y} * #{z} = #{x * y * z}"
