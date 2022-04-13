import Config

config :poketwo_database, Poketwo.Database.Repo,
  database: "",
  username: "",
  password: "",
  hostname: ""

config :poketwo_database,
  ecto_repos: [Poketwo.Database.Repo]

if File.exists?("config/#{Mix.env()}.secret.exs") do
  import_config("#{Mix.env()}.secret.exs")
end