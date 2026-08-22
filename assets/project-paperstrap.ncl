let rec cfg = {
  build_path = "build/",
  version = "26.2",
  build = 112,
  hash = "bd3a58cf96874e5ea6643f5f6fe9b4f5bf9e34b795fa078c2f0ee8b98b2f907e",
  server_properties = {
    online_mode = false,
  },
  paper_global_config = {},
  plugins = {
    vault = plugin.url "https://github.com/MilkBowl/Vault/releases/download/1.7.3/Vault.jar",
  },
} in
cfg
