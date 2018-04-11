#!/usr/bin/env ruby

def e_u(n_1)
  e = 0

  while 2**e < n_1
    u = n_1 / 2**e
    return [e, u] if n_1 % 2**e == 0 && u.odd?
    e += 1
  end

  nil
end

def miller_rabin(n)
  e, u = e_u(n - 1)

  puts "e = #{e}, u = #{u}"

  Hash[(1..(n - 1)).map { |x|
    next [x, :prime] if (e - 1).downto(0).any? { |i|
      (x**(2**i * u)) % n == -1 % n
    }

    next [x, :prime] if (x**u) % n == 1 % n
    [x, :non_prime]
  }]
end

a = miller_rabin 87
b = miller_rabin 89

def bad_values(h)
  h.select { |k, v| v == :prime }.map { |k, v| k }
end

bad_values_a = bad_values a
bad_values_b = bad_values b

puts "Schlechte Werte für a) #{bad_values_a.join(", ")} (#{bad_values_a.count / a.count.to_f * 100} %)"
puts "Schlechte Werte für b) #{bad_values_b.join(", ")} (#{bad_values_b.count / b.count.to_f * 100} %)"
