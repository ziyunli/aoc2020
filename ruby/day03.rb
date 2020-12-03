# # frozen_string_literal: true

open('../input/03.txt') do |f|
  prev = row = col = nil
  state = f.reduce(0) do |count, curr|
    curr = curr.chomp
    width = curr.size
    if prev.nil?
      row = col = 0
    else
      row += 1
      col += 3
      count += 1 if curr[col % width] == '#'
    end
    prev = curr
    count
  end
  p state
end

map = open('../input/03.txt') { |f| f.each_with_object([]) { |i, a| a << i.chomp } }

def count(map, right, down)
  height = map.size
  width = map.first.size

  count = 0

  col = 0
  (0...height).step(down) do |row|
    count += 1 if map[row][col % width] == '#'
    col += right
  end

  count
end

a = count(map, 1, 1)
b = count(map, 3, 1)
c = count(map, 5, 1)
d = count(map, 7, 1)
e = count(map, 1, 2)

p "#{a} * #{b} * #{c} * #{d} * #{e} = #{a * b * c * d * e}"

