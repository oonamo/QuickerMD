<h1 align='center'>
Quicker MD
</h1>

<p align='center'>
    <b>Quickly run any compiled/interpreted language</b>
</p>

> [!CAUTION]
> Expect breaking changes until this goes away!!

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
    - [Overview](#overview)
        - [Compiled vs Interpreted](#compiled-vs-interpreted)
    - [Config](#config)
    - [Examples](#examples)
        - [Example Config](#example-config)

## Features
- [x] Create a template for any compiled language
- [x] Use commands for use with interpreters (i.e. python, node)
- [x] Use input as stdin for your command 

## Installation
```
TODO
```

Config Locations:
| **Platform** | **Config Location** |
|---|---|
| Windows | `%LOCALAPPDATA%\QuickMD\config\config.toml` |
| MacOS | `$HOME/Library/Application Support/QuickMD/config.toml` |
| Linux | `$XDG_CONFIG_HOME/QuickMD/config.toml or $HOME/.config/QuickMD/config.toml` |
## Usage

### Overview
```sh
quicker_md --lang <LANG> [INPUT]
```

#### Compiled vs Interpreted

To decide whether to compile or run the command, Quicker MD follows a principle to determine this.

1. If the user provided the `redir_input` key
    - It is assumed that the program is interpreted
2. If the user uses the variable **INPUT**
    - It is assumed that the program is interpreted
3. Other wise
    - It is assumed that the program is compiled
        - If the user does not provide a template field
            1. Write the raw input to a file
            2. Run the provided command with variables
        - If the user provided a template field
            1. Write the input between the `<<< TEMPLATE` blocks to a file
            2. Run the provided command with variables


### Config
The config uses the `toml` format.

Each individual entry is composed of the following:
| Key | Required | Default | Description | Example |
|:---:|:---:|:---:|:---:|---|
| `[lang.{name}]` | **True** | N/A | Table that holds all the languages | `[lang.c]` |
| `command` | **True** | N/A | Command to run | `command = ["gcc", "-o", "{{OUT}}", "{{IN}}"]` |
| `redir_input` | **False** | False | Whether to use input as **stdin** for `command` | `redir_input = true`<br>`command = ["node"]` |
| `prefix` | **False** | "" |Prefix to use on *non-code* output | `prefix = "# "` |
| `template` | **False** | None | Template string for compiled languages | See [Example Config](#example-config) |
| `extension` | **False** | {name} | File extension to use if file on compiled language  | `extension = "rs"` |

#### Adding Languages
To add a language, you must add a new entry to the `langs` table.

The format is `[langs.{name}]`. The `{name}` part will be the part that will be searched for in the `--lang` argument.

> [!TIP]
> The `{name}` can be **anything** 
> It is not tied to  the file extension or the language name, but rather just an *alias* for running the template
> See [File Extensions](#file-extensions)

<details>
    <summary>Examples</summary>

    | Language | Config | CLI |
    |:--:|:--:|---|
    | C | `[langs.c]` | `--lang c` |
    | Rust | `[langs.rust]` | `--lang rust` |
    | Python | `[langs.py]` | `--lang py` |

</details>

##### Command

> [!IMPORTANT]
> Currently, their is **no** fallback for executing a command that has no **OUT** variable
> The **OUT** variable is **required** for executing the file

> [!TIP]
> If a language is **interpreted**, you can simplify the command field by using `redir_input`
> **See** [Redir Input](#redir-input)

The command field tells **QuickerMD** on how to handle the language.

**Quicker MD** provides variables that can be used inside the `command` field
| Variable | Descritption | Example |
|:---:|:--:|----|
| **IN**| Refers to the file that is created for the template, if needed.| `command = ["node", "{{IN}}"]` |
| **OUT**| Refers to the file that will be executed | `command = ["gcc", "{{IN}}", "-o", "{{OUT}}"]` |
| **INPUT**| Refers to the input passed by the command line | `command = ["python", "-c", "{{INPUT}}"]` |

See [Compiled vs Interpreted](#compiled-vs-interpreted) for understanding what determines interpreted vs compiled

##### Redir Input
If a language is *interpreted* or has a *REPL* their is a *high* chance that by default, it accepts **piped** input.

Python:
```sh
echo "print('hello, python!')" | python
```

JavaScript:
```sh
echo 'console.log("hello, js!")' | node
```

To mimic this behavior, **QuickMD** offers the `redir_input` key.

If `redir_input` is set for the language, the input is "*piped*" to the `command` field.

##### Prefix

> [!TIP]
> You can use `prefix` as the languages *comment string*, 

To allow for easy parsing of output, or even replacing a code block inline, **QuickMD** offers a `prefix` field
This field adds a *prefix* to every **non-code** output.

```toml
[langs.js]
command = ["node"]
redir_input = true
prefix = "// "
```

```sh
echo 'console.log("What will this output")' | quicker_md --lang js --show-input
```

Will output:
```txt
// Input:
console.log("What will this output")

// Output:
// hello, world
```

##### Template

> [!NOTE]
> If `redir_input` is present, template **will be ignored**

The template fields allows you to write default's for compiled languages that require boilerplate.

I.e. C#
```toml
[langs.cs]
template = """
using System;

namespace QuickerMD
{
    internal class Program
    {
        static void Main(string[] args)
        {
            <<< TEMPLATE START

            <<< TEMPLATE END
        }
    }
}
"""
```

##### File Extensions
By default, the file extension for the created file is the name.

In the following case, the `extension` field is needed to tell **QuickerMD** that the file extensions should be `rs`. (Result is `out.rs`)
If the extension is omitted, the fallback extension will be `rust`. (i.e. `out.rust`)
```toml
[langs.rust]
command = ["rustc", "{{IN}}", "-o", "{{OUT}}""]
extension = "rs"
```

### Examples

#### Example config
```toml
[langs.c]
command = ["gcc", "{{IN}}", "-o", "{{OUT}}"]
prefix = "// "
template ="""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
  <<< TEMPLATE START

  <<< TEMPLATE END
}
"""

[langs.js]
command = ["node"]
prefix = "// "
redir_input = true

[langs.py]
command = ["python"]
prefix = "# "
redir_input = true

[langs.ps]
command = ["pwsh", "-NoProfile", "-NonInteractive", "-Command", "{{INPUT}}"]
prefix = "# "

[langs.rust]
extension = "rs"
command = ["rustc", "{{IN}}", "-o", "{{OUT}}"]
prefix = "// "
template = """
pub fn main() {
  <<< TEMPLATE START

  <<< TEMPLATE END
}
"""
```

```sh
quicker_md --lang c 'printf("Hello, from Quicker MD!\n");'

echo "console.log('Hello js, from Quicker MD!')" | quicker_md --lang js

quicker_md --lang rust 'println!("Rust inside Quicker MD!")'
```
