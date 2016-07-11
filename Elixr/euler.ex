defmodule Fib do
  def fib do
    Stream.unfold({0, 1}, fn {a, b} -> {a, {b, a + b}} end)
  end
end

defmodule Euler do
  def euler1 max do
    1..max-1
    |> Stream.filter(&(rem(&1, 3) == 0 || rem(&1, 5) == 0))
    |> Enum.sum
  end

  def euler2 max do
    Fib.fib
    |> Stream.take_while(&(&1 < max))
    |> Stream.filter(&(rem(&1,2) == 0))
    |> Enum.sum
  end
end

IO.puts Euler.euler1(10)
IO.puts Euler.euler1(1000)

IO.puts Euler.euler2(4_000_000)

