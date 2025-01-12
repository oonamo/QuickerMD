# Getting Started
**QuickerMD** is a tool that allows for easily running short snippets of both interpreted and compiled languages. This tool allows for easy gathering output of various languages with predefined templates.

## Installation
- [Building From Source](#building-from-source)

???+ TIP "Installation on Windows"
    To prevent *Windows* from truncating a path that is to large, it is recommend to run the following command in an admin terminal:
    ```powershell
    Set-ItemProperty 'HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem' -Name 'LongPathsEnabled' -Value 1
    ```

### Building From source
```sh
git clone https://github.com/oonamo/QuickerMD.git quickermd
cd quickermd
cargo install --path .
```

Once installed, head over to [Example Configuration](example-config.md)!
