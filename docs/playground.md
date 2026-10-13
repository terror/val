# Playground

The playground runs val in your browser using WebAssembly. You can try the
language without installing the command-line tool.

## Running a program

Choose an example or write a program in the editor. Select **Run**, or press
**Ctrl+Enter** (**Cmd+Enter** on macOS), to evaluate it.

The **Output** pane shows printed output, the final non-null value, errors, and
the exit code. Each run starts with a fresh environment. Use **Stop** to
interrupt a running program.

## Input

Enter standard input in the **Input** field before running the program. Each
call to `input()` reads one line:

```val
name = input("Name: ")
println("Hello, " + name)
```

Input is read from the field rather than from an interactive terminal prompt.
When there are no more lines, `input()` returns an empty string.

## Syntax tree

Select **Syntax tree** to inspect the parsed program. Expand nodes to explore
the structure and point to a node to highlight its source in the editor. Syntax
diagnostics appear as you edit; runtime errors appear after execution.

## Editor settings

The editor settings include font size, line numbers, line wrapping, tab size,
and default or Vim keybindings. The editor content, selected example, settings,
and panel layout are saved in your browser.

The playground uses the default calculation settings: 1024 bits of precision and
16 displayed digits. Use the [command line](usage.md) to select different
precision and rounding options.
