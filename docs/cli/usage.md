# Using the CLI
`QuickerMD` offers a simple CLI for using the program.

At any time, you can use the `--help` flag to discover any options available for any subcommand.

## Basic Usage
### Running a Config
To run a program, use the appropriately name command, `run`.

```sh
quicker_md run --help
quicker_md run py "print('Hello, this is cool!')"

# The basic format is
# quicker_md run <lang> <input>
```

You can also pipe commands!

```sh
echo "print('My cool script that echos!')" | quicker_md run py
```

A useful case for this is using input from files!

!!! WARNING
    Make **sure** that **you** are aware of the contents of the file before using


### Getting Your Template
To get your template, you can use the `dump-template` command

```sh
quicker_md dump-template --help

quicker_md dump-template py

# The basic format is
# quicker_md dump-template <lang>
```
