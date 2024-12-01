body = File.read!("files/input.txt")

defmodule Day01 do
  def part1(input) do
    all = parse(input)
    {left, right} = Enum.unzip(all)
    Enum.zip([Enum.sort(left), Enum.sort(right)])
    |> Enum.map(fn {a, b} ->
      abs(a - b)
    end)
    |> Enum.sum
  end

  def part2(input) do
    all = parse(input)
    {left, right} = Enum.unzip(all)
    frequencies = Enum.frequencies(right)
    left
    |> Enum.map(fn n ->
      n * Map.get(frequencies, n, 0)
    end)
    |> Enum.sum
  end

  defp parse(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(fn line ->
      {left, line} = Integer.parse(line)
      line = String.trim(line)
      {right, ""} = Integer.parse(line)
      {left, right}
    end)
  end
end


resultsPt1 = Day01.part1(body)
resultsPt2 = Day01.part2(body)
IO.puts("Part 1: " <> Integer.to_string(resultsPt1))
IO.puts("Part 2: " <> Integer.to_string(resultsPt2))
