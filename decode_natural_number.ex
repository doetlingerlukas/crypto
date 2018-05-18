defmodule NaturalNumber do
  def decode(number) do
    decode(number, [])
  end

  def decode(number, acc) do
    decode(number, 3, acc)
  end

  def decode(number, _, acc) when number == "" do
    acc
  end

  def decode(number, len, acc) do
    result = Integer.parse(String.slice(number, 0..(len - 1)), 2)

    if len > String.length(number) || result == :error do
      []
    else
      {n, _} = result

      if Integer.parse(String.slice(number, len..len), 2) == {0, ""} do
        decode(String.slice(number, (len + 1)..-1), acc ++ [n])
      else
        decode(String.slice(number, len..-1), n, acc)
      end
    end
  end
end

Enum.each(
  [
    "1001011110000110000",
    "1001011111111000",
    "10010111111110001001010101100010",
    "10010111111110001001010101100011"
  ],
  fn number ->
    IO.puts inspect NaturalNumber.decode(number)
  end
)
