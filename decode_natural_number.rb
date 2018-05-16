#!/usr/bin/env ruby

def decode(number)
  len = 3 # Initial length is 3 bits.
  i = 0

  while i < number.length
    if i == number.length - 1
      return len if number[i] == '0' # Last digit has to be 0.
      break
    end

    next_len = number[i...(i + len)].to_i(2)

    i += len
    len = next_len
  end

  raise "#{number} is not a code word."
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
