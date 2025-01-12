# Templates
!!! NOTE
    If `redir_input` is present, template **will be ignored**

The template fields allows you to write default's for compiled languages that require boilerplate.

I.e. C#
```toml
[langs.cs]
template = """
using System;

namespace QuickerMD
{
    internal class Program
    {
        static void Main(string[] args)
        {
            <<< TEMPLATE START

            <<< TEMPLATE END
        }
    }
}
"""
```
