pub struct Manpage {
    pub name: &'static str,
    pub description: &'static str,
    pub text: &'static str,
}

pub const MANPAGES: &[Manpage] = &[
    Manpage {
        name: "attr-build_path",
        description: "build_path config attribute",
        text: "(String)\nsets the directory where the built server will go",
    },
    Manpage {
        name: "attr-version",
        description: "version config attribute",
        text: "(String)\nsets the paper serer game version",
    },
    Manpage {
        name: "attr-build",
        description: "build config attribute",
        text: "(Number)\nsets the paper server jar build number",
    },
    Manpage {
        name: "attr-hash",
        description: "hash config attribute",
        text: "(String)\nsets the paper server jar sha256 hash",
    },
    Manpage {
        name: "attr-server_properties",
        description: "server_properties config attribute",
        text: r#"(Record)
sets the server.properties config values

value names can be derived from actual papermc's server.properties, for example:
```
allow-flight=false
rcon.port=25575
```
in server.properties is the same as:
```
server_properties = {
  allow_flight = false,
  rcon_port = 25575,
},
```
in paperstrap
"#,
    },
    Manpage {
        name: "attr-paper_global_config",
        description: "paper_global_config config attribute",
        text: r#"(Record)
sets the paper-global.yml config values

default:
{
    unsupported_settings = {
        allow_headless_pistons = false,
        allow_piston_duplication = false,
    },
}
"#,
    },
    Manpage {
        name: "attr-plugins",
        description: "plugins config attribute",
        text: r#"(Record)
plugin declarations

example:
```
plugins = {
  vault = plugin.url "https://github.com/MilkBowl/Vault/releases/download/1.7.3/Vault.jar",
},
```

Refer to record-plugin manpage for documentation on `plugin.url`
"#,
    },
    Manpage {
        name: "attr-world_source",
        description: "world_source optional config attribute",
        text: r#"(String)
set the directory path for the world
default value is "world"
"#,
    },
    Manpage {
        name: "record-plugin",
        description: "plugin helpers",
        text: r#"(Record)

plugin.modrinth:
  generate a modrinth plugin reference

  downloads a plugin from modrinth according to `config` and puts it into plugins/

  args:
    + config: Record                       plugin config
      + fields:
        + optional version: String         set a specific plugin version
        + game_version: String             set a specific game version
        + channel: String                  set a download channel, valid values are: release, beta, alpha

plugin.local:
  generate a local plugin reference

  adds a symlink to `path` in the plugins/ directory

  args:
    + path: String                         plugin jar path

plugin.url:
  generate a url plugin reference

  downloads a plugin from `url` and puts it into plugins/

  args:
    + path: String                         plugin url
"#,
    },
];
