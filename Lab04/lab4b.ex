defmodule Lab4b do
  {:ok, contents} =
    File.read("atomic-weights.txt")

  elements =
    String.split(contents, "\n", trim: true)
    |> Enum.map(&String.split(&1, "\t", trim: true))
    |> Enum.map(fn [_, symbol, _, weight] ->
      {symbol, String.to_float(weight)}
    end)

  for {s, w} <- elements do
    def atomic_weight(unquote(s)) do
      unquote(w)
    end
  end

  def atomic_weight(_), do: -1
end
