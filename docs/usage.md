# Command line

Use val interactively, evaluate an expression from your shell, or run a saved
program.

## Interactive REPL

Run `val` with no arguments:

```console
$ val
> 0.1 + 0.2
0.3
> x = 1 / 3
0.3333333333333333
> x * 3
1
```

Variables and functions remain available for the rest of the session. The REPL
supports syntax highlighting, filename completion, Emacs-style editing, and
hints from your command history. History is saved in `~/.val_history`.

Call `exit()` or `quit()` to end the session.

## Expressions

Use `--expression` or `-e` to evaluate code and print its result. Quote the
expression so your shell passes it to val unchanged.

```console
$ val -e '(1 / 3) * 3'
1
```

A `null` result is not printed. Output from `print` and `println` is still
written to standard output.

## Files

Save the following program as `factorial.val`:

```val
fn factorial(n) {
  if (n <= 1) {
    return 1
  }

  return n * factorial(n - 1)
}

println(factorial(5))
```

Then run it:

```console
$ val factorial.val
120
```

File execution does not print the value of the final expression. Use `print` or
`println` for output. A filename and `--expression` cannot be used together.

## Loading files into the REPL

Use `--load` or `-l` to evaluate a file before starting an interactive session:

```sh
val --load factorial.val
```

The file's variables and functions remain available in the REPL, and any output
from the file is printed while loading it. Repeat the option to load more files:

```sh
val --load foo.val --load bar.val
```

`--load` cannot be combined with `--expression` or a positional filename.

## Options

| Option                                  | Default   | Purpose                                                        |
| --------------------------------------- | --------- | -------------------------------------------------------------- |
| `-d`, `--digits <DIGITS>`               | `16`      | Significant decimal digits to display for approximate numbers. |
| `-e`, `--expression <EXPRESSION>`       |           | Evaluate an expression and exit.                               |
| `-l`, `--load <LOAD>`                   |           | Load a file before entering the REPL. Can be repeated.         |
| `-p`, `--precision <PRECISION>`         | `1024`    | Binary precision in bits for approximate calculations.         |
| `-r`, `--rounding-mode <ROUNDING_MODE>` | `to-even` | Rounding mode for calculations.                                |
| `--stack-size <STACK_SIZE>`             | `128`     | Evaluation thread stack size in MiB.                           |
| `-h`, `--help`                          |           | Print help.                                                    |
| `-V`, `--version`                       |           | Print the version.                                             |

Digits, precision, and stack size must be positive integers. See
[numbers and precision](numbers.md) for rounding modes and how precision affects
calculations.

Color output is enabled for terminal output only. Set `NO_COLOR` to a non-empty
value to disable it.
