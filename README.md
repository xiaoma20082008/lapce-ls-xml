lapce-lsp-xml

---
[Lapce](https://lapce.dev/) LSP plugin for XML, powered by [lemminx](https://github.com/eclipse/lemminx).


## Configuration

### Global

Add following configuration to `${LAPCE_INSTALL_DIR}/plugins/xiaoma20082008.lapce-lsp-xml/volt.toml`:

```toml

[config."lapce-lsp-xml.serverArgs"]
default = ""
description = "XML Language Server's Arguments"

[config."lapce-lsp-xml.serverPath"]
default = ""
description = "Path of `XML Language Server` executable. When empty, it points to the bundled binary `lemminx`."

[config."lapce-lsp-xml.lemminx.version"]
default = ""
description = "lemminx-uber's version"

[config."lapce-lsp-xml.lemminx.port"]
default = 5008
description = "lemminx-uber's connection port"

[config."lapce-lsp-xml.lemminx.args"]
default = ""
description = "lemminx-uber's JVM arguments. Like -Xms=32M -Xmx=32M"

```

### Workspace

Add following configuration to `${WORKSPACE_DIR}/.lapce/settings.toml`:

```toml

[lapce-lsp-xml]
serverPath = ""
serverArgs = ""

[lapce-java.lemminx]
version = "0.23.2"      # lemminx's version
port = 5008             # lemminx's port
args = "-Xms32M -Xmx32M"# lemminx's jvm args

```



## Licence

All the code in this repository is released under the Apache License, for more information take a look at
the [LICENSE](LICENSE) file.