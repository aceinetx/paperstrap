# paperstrap
A tool to declaratively create PaperMC minecraft servers using the Nickel programming language.

Paperstrap keeps the world data separate from the configration, meaning with paperstrap you can create server templates that other people can use.

## Why did I make paperstrap?
I run a private minecraft server on which I play with my friends, the most tedious part of maintaining it is updating the minecraft version, more specifically updating all the plugins to the new versions. Paperstrap is designed to fix this problem. You declare plugins you need in a paperstrap.ncl file, then paperstrap downloads the plugins from either a url or modrinth and sets up the server which you can just run. That is another case where you may like paperstrap.

## Pureness
Even though paperstrap servers are configured in a pure functional programming language, the end result may vary. For example, when you are using modrinth plugins, you may not specify a version so that paperstrap picks the latest one for you:
```nickel
{
  "coreprotect" = plugin.modrinth {
    game_version = cfg.version,
    channel = "release",
  },
}
```
But you can eliminate the impureness by specifying a version:
```nickel
{
  "coreprotect" = plugin.modrinth {
    version = "24.0",
  },
}
```

## Example server configuration
```nickel
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
```
Build the example server like this:
```sh
$ paperstrap build
$ paperstrap download-plugins
```
And run like this:
```sh
$ cd build
$ ./start.sh
```