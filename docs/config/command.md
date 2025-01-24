# Command
!!! BUG
    Currently, their is **no** fallback for executing a command that has no **OUT** variable
    The **OUT** variable is **required** for executing the file

!!! TIP
    If a language is **interpreted**, you can simplify the command field by using `redir_input`
    **See** [Redir Input](redirecting-input.md)

The command field tells **QuickerMD** on how to handle the language.

## Variables
**Quicker MD** provides variables that can be used inside the `command` field

| Variable  | Descritption                                                    | Example                                        |
| :---:     | :--:                                                            | ----                                           |
| **IN**    | Refers to the file that is created for the template, if needed. | `command = ["node", "{{IN}}"]`                 |
| **OUT**   | Refers to the file that will be executed                        | `command = ["gcc", "{{IN}}", "-o", "{{OUT}}"]` |
| **INPUT** | Refers to the input passed by the command line                  | `command = ["python", "-c", "{{INPUT}}"]`      |

See [Compiled vs Interpreted](compiled-vs-interpreted.md) for understanding what determines interpreted vs compiled
