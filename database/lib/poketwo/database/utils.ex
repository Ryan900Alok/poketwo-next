defmodule Poketwo.Database.Utils do
  def string_value(nil), do: nil
  def string_value(""), do: nil
  def string_value(value) when is_binary(value), do: Google.Protobuf.StringValue.new(value: value)

  def if_loaded(%Ecto.Association.NotLoaded{}, _func), do: nil
  def if_loaded(val, func), do: func.(val)
  def map_if_loaded(val, func), do: if_loaded(val, fn x -> Enum.map(x, func) end)

  defmacro from_info(queryable) do
    quote do
      import Ecto.Query

      from(gi in unquote(queryable),
        left_join: l in assoc(gi, :language),
        order_by: l.order,
        where: l.enabled,
        preload: :language
      )
    end
  end
end
