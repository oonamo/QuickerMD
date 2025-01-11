<h1 align='center'>
Quicker MD
</h1>

<p align='center'>
    <b>Quickly run any compiled/interpreted language</b>
</p>

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
```bash
quicker_md --lang <LANG> [INPUT]
```
### Examples
Config file
```toml
[langs.c]
command = ["gcc", "{{IN}}", "-o", "{{OUT}}"]
template = """
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
        <<< TEMPLATE START

        <<< TEMPLATE END
}
"""
```

```bash
quicker_md --lang c 'printf("Hello, from Quicker MD!\n");'
```
