# Compiled vs Interpreted
To understand the differences between compiled and interpreted programs, we will look at a configuration for each. 

## C example (Compiled)
The `command` fields holds `gcc`, `{{IN}}`, `-o`, `{{OUT}}`.

The first argument, `gcc` tells `QuickerMD` that the program I want to call for the C language is `gcc`. 
The rest are passed as arguments to the program, with `{{IN}}` and `{{OUT}}` being substituted with their respective variable.
See [Command Variables](command.md#variables).

`QuickerMD` will translate the `command` to `["gcc", "tmp.c", "-o", "tmp_exe"]`

The `template` field holds a multi-line string that will be used to create a default file for an input.
If we run the following in the terminal:

```sh
quicker_md run c 'printf("Hello, from QuickerMD");' --show-input
```

The created temporary file will look:

```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
printf("Hello, from QuickerMD");
}
```

??? WARNING
    Notice that `QuickerMD` does not add indents to the input? This *may* be a **problem** for white-space sensitive languages such as `python`.

## Python (Interpreted)
The `command` field has just `python` in the array.
This tells `QuickerMD` that it should run `python` with no arguments.

The `redir_input` is set to true. This tells `QuickerMD` that the input passed to it will be redirected to the program, which is `python`

If we run the following in the terminal:

```sh
quicker_md run py 'print("Hello, from QuickerMD")'
```

We are essentially running the following:

```sh
echo 'print("Hello, from QuickerMD")' | python
```

