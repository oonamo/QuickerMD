# File Extensions
By default, the file extension for the created file is the name.

In the following case:
```toml
[langs.rust]
command = ["rustc", "{{IN}}", "-o", "{{OUT}}"]
template = """
pub fn main() {
    <<< TEMPLATE START

    <<< TEMPLATE END
}
"""
```

Running the command:
```sh
quicker_md --lang rust "println!('Hello, world!');"
```

Would produce the temporary file `out.rust`. While this *may* run, it is **not recommended** to use incorrect file types, as this may through warnings to the compiler.

To prevent this the field `extension` is provided to tell `QuickerMD` the correct file type for the language.

In the rust example, the correct usage of the `extension` field would be:
```toml
[langs.rust]
extension = "rs"
command = ["rustc", "{{IN}}", "-o", "{{OUT}}"]
template = """
pub fn main() {
    <<< TEMPLATE START

    <<< TEMPLATE END
}
"""
```
