# Redirecting Input

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

The configuration can be simplified to the following:

```toml
[langs.py]
redir_input = true
command = ["python"]

[langs.js]
redir_input = true
command = ["node"]
```
