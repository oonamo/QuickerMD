# Default Config
`QuickerMD`'s essentially allows for infinite customizability, as it allows for any language and code to be configured.

## config.toml
This example config has support for simple use cases for `C`, `JavaScript`, `Rust`, and `Python`. Feel free to use this as a preliminary config, and modify this as your workflow changes.

```toml
[langs.c] # Configuration for C, --lang c
command = ["gcc", "{{IN}}", "-o", "{{OUT}}"] 
comment = "// "
template ="""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
  {{INPUT}}
}
"""

[langs.js] # Configuration for JavaScript, --lang js
command = ["node"] 
comment = "// "
redir_input = true

[langs.py] # Configuration for Python, --lang py
command = ["python"]
comment = "# "
redir_input = true

[langs.ps] # Configuration for Powershell, --lang ps
command = ["pwsh", "-NoProfile", "-NonInteractive", "-Command", "{{INPUT}}"]
comment = "# "

[langs.rust] # Configuration for Rust, --lang rust
extension = "rs"
command = ["rustc", "{{IN}}", "-o", "{{OUT}}"]
comment = "// "
template = """
pub fn main() {
    {{INPUT}}
}
"""
```

Try it with the following commands!
```sh
quicker_md run js "console.log('Hello, from QuickerMD!')" --show-input
```

## Understanding Configuration
The config uses the `toml` format.

Each individual entry is composed of the following:

| Key | Required | Default | Description | Example | Documentation |
|---|:---:|----|-----|---|---|
| `[lang.{name}]` | **True** | N/A | Table that holds all the languages | `[lang.c]` | N/A|
| `command` | **True** | N/A | Command to run | `command = ["gcc", "-o", "{{OUT}}", "{{IN}}"]` | [Command](command.md) |
| `redir_input` | **False** | False | Whether to use input as **stdin** for `command` | `redir_input = true`<br>`command = ["node"]` | [Redirecting Input](redirecting-input.md) |
| `template` | **False** | None | Template string for compiled languages | See [Example Config](example-config.md#configtoml) | [Templating](templating.md) |
| `extension` | **False** | {name} | File extension to use if file on compiled language  | `extension = "rs"` | [File Extensions](file-extensions.md) |
| `run` | False | True/{{OUT}} | Whether to run a 'run' command after `command`, and optionally a command | run = ["my_compiled_program", "Hello!" | [Running your Program](running.md) |
