defmodule Lab4a do
  {:ok, contents} =
    File.read("atomic-weights.txt")

  elements =
    String.split(contents, "\n", trim: true)
    |> Enum.map(&String.split(&1, "\t", trim: true))
    |> Enum.map(fn [_, symbol, _, weight] ->
      {String.downcase(symbol) |> String.to_atom(), String.to_float(weight)}
    end)

  for {symbol, weight} <- elements do
    def unquote(symbol)() do
      unquote(weight)
    end
  end
end
