#!/usr/bin/env ruby

def decode(number)
  len = 3 # Initial length is 3 bits.
  i = 0

  numbers = []

  while i < number.length
    raise "#{number} is not a code word." if (i + len) >= number.length

    n = number[i...(i + len)].to_i(2)
    i += len

    if number[i] == '0'
      i += 1
      numbers << n
      len = 3
    else
      len = n
    end
  end

  numbers
end

[
  '1001011110000110000',
  '1001011111111000',
  '10010111111110001001010101100010',
].each do |number|
  begin
    decoded_number = decode(number)
    puts "#{number} == #{decoded_number}"
  rescue => e
    STDERR.puts e
  end
end
