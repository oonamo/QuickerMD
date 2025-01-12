# Using the CLI
`QuickerMD` offers a simple CLI for using the program.

## Arguments

| Argument | Required | Default | Description |
|:--:|:--:|:--:|:--:|
| `--lang`, `-l` | Yes | N/A | The language to run, from your config |
| `input` | No[^1] | N/A | The input to be run |
| `--show-input`, `-s` | No | False | Whether to show the original input as output|
| `--raw`, `-r` | No | False | Whether to show the output *as-is* |
| `--no-prefix`, `-n` | No | False | Disables the `prefix` feature in config |


[^1]: **Not** required if input is piped


## Example Output
### Default
```sh
quicker_md --lang py 'print("Hello, python!")'
```

Output:
```txt
# Output:
# Hello, python!
```
### Show Input
```sh
quicker_md --show-input --lang c 'printf("Is this a JS skill issue?: %s", 0.1 + 0.2 == 0.3 ? "Only JS sucks" : "All languages suck");'
```

Output:
```c
// Input:
printf("Is this a JS skill issue?: %s", 0.1 + 0.2 == 0.3 ? "Only JS sucks" : "All languages suck");

// Output:
// Is this a JS skill issue?: All languages suck
```
### With Error
```sh
quicker_md --show-input --lang c 'int x = 5; printf("%d\n", x)'
```

Output:
```c
// Input:
int x = 5; printf("%d\n", x)

// Error:
// C:\Users\USER\AppData\Local\Temp\.tmp6ZlN94\tmp.c: In function 'main':
// C:\Users\USER\AppData\Local\Temp\.tmp6ZlN94\tmp.c:6:29: error: expected ';' before '}' token
//     6 | int x = 5; printf("%d\n", x)
//       |                             ^
//       |                             ;
//     7 | }
//       | ~
```

### Raw
```sh
quicker_md --lang js --raw 'console.log(("hello").split("").reverse().join(""))'
```

Output:
```txt
// olleh
```

#### Raw + No-Prefix
```sh
quicker_md --no-prefix --lang js --raw 'console.log(("hello").split("").reverse().join(""))'
```

Output:
```txt
olleh
```

#### Raw + Show-Input
```sh
quicker_md --show-input --no-prefix --lang js --raw 'console.log(("hello").split("").reverse().join(""))'
```

Output:
```text
console.log(("hello").split("").reverse().join(""))

olleh
```
