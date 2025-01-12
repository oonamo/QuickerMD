# Adding Languages
To add a language, you must add a new entry to the `langs` table.

The format is `[langs.{name}]`. The `{name}` part will be the part that will be searched for in the `--lang` argument.

!!! TIP
    The `{name}` can be **anything** 
    It is not tied to  the file extension or the language name, but rather just an *alias* for running the template
    See [File Extensions](file-extensions.md)

## Examples

| Language | Config | CLI |
|:--:|:--:|---|
| C | `[langs.c]` | `--lang c` |
| Rust | `[langs.rust]` | `--lang rust` |
| Python | `[langs.py]` | `--lang py` |
