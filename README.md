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

## Features
- [x] Create a template for any compiled language
- [x] Use commands for use with interpreters (i.e. python, node)
- [x] Use input as stdin for your command 

## Installation
```
TODO
```

## Usage

### Overview
```sh
quicker_md --lang <LANG> [INPUT]
```

#### Variables
**Quicker MD** provides variables that can be used to handle templating needs

| Variable | Example | Descritption |
|:--:|:--:|:--:|
| **IN**| `command = ["gcc", "{{IN}}"]` | Refers to the file that is created for the template, if needed. **See**|
| **OUT**| `command = ["gcc", "{{IN}}", "-o", "{{OUT}}"]` | Refers to the file that will be executed |
| **INPUT**| `command = ["python", "-c", "{{INPUT}}"]` | Refers to the input passed by the command line |

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

> [!IMPORTANT]
> Currently, their is **no** fallback for executing a command that has no **OUT** variable
> The **OUT** variable is **required** for executing the file

### Examples
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
