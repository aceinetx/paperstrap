let plugin = {
  modrinth = fun config => { source = "modrinth" } & config,
  local = fun path => { source = "local", include path },
  url = fun url => { source = "url", include url },
}
