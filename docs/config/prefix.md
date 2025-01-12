# Prefixing Output

!!! TIP
    You can use `prefix` as the languages *comment string*. This provides an easy method to inline the codes output in a way that won't affect the language's syntax highlighting!

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
```js
// Input:
console.log("What will this output")

// Output:
// hello, world
```
