#!/usr/bin/env ruby

include Math

def entropy(*ps)
  -ps.map { |p_i|
    p_i * log2(p_i)
  }.reduce(&:+)
end

puts entropy(7.0 / 24, 10.0 / 24, 7.0 / 24)
puts entropy(6.0 / 24, 12.0 / 24, 6.0 / 24)
puts entropy(8.0 / 24,  8.0 / 24, 8.0 / 24)

puts

puts entropy(4.0 / 8, 0.0 / 8, 4.0 / 8)
puts entropy(2.0 / 8, 4.0 / 8, 2.0 / 8)
puts entropy(3.0 / 8, 2.0 / 8, 3.0 / 8)
