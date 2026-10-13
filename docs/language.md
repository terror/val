# Language

Val programs are made up of expressions and statements. Semicolons are optional,
and blocks use braces.

## Values and variables

Assign a value with `=`:

```val
x = 42
fraction = 1 / 3
enabled = true
name = "foo"
items = [1, "bar", false]
missing = null
```

Values can be numbers, booleans, strings, lists, functions, or `null`. Strings
can use single or double quotes. Lists can contain values of different types.

Variables, functions, and built-in constants share a namespace. Reads and calls
use the nearest binding. Assignment replaces the nearest existing binding, or
creates a binding in the current scope if the name is new. Function definitions,
parameters, and `for` loop variables bind names in the current scope.

## Operators

| Kind          | Operators                        | Example            |
| ------------- | -------------------------------- | ------------------ |
| Arithmetic    | `+`, `-`, `*`, `/`, `%`, `^`     | `2 ^ 3 + 1`        |
| Comparison    | `==`, `!=`, `<`, `<=`, `>`, `>=` | `x >= 10`          |
| Logical       | `&&`, `\|\|`, `!`                | `enabled && x > 0` |
| Negation      | `-`                              | `-x`               |
| Function call | `f(...)`                         | `sqrt(16)`         |
| Indexing      | `value[index]`                   | `items[0]`         |

`^` means exponentiation. Use parentheses to make the order of evaluation
explicit:

```val
println(2 + 3 * 4)
println((2 + 3) * 4)
```

These expressions print `14` and `20`. See [numbers and precision](numbers.md)
for exact and approximate arithmetic.

## Strings and lists

Use `+` to concatenate strings or lists. Lists use zero-based indices and
support indexed assignment:

```val
name = "foo"
println("Hello, " + name)

items = [1, 2, 3]
items[0] = 10
items = items + [4, 5]
println(items)
```

`append` returns a list with the extra item. Assign its result to keep the
change:

```val
items = append(items, 6)
```

## Conditionals

Use `if` and an optional `else` block. The condition goes in parentheses:

```val
x = 10

if (x > 5) {
  println("greater")
} else {
  println("smaller or equal")
}
```

## Loops

Use `for` to iterate over a list. `range(start, end)` excludes the end value:

```val
for n in range(0, 5) {
  println(n ^ 2)
}
```

Use `while` to repeat a block while its condition is true:

```val
n = 0

while (n < 5) {
  println(n)
  n = n + 1
}
```

`loop` repeats until a `break`, `return`, or program exit. `continue` skips to
the next iteration of a loop:

```val
n = 0

loop {
  n = n + 1

  if (n == 5) {
    break
  }

  if (n % 2 == 0) {
    continue
  }

  println(n)
}
```

## Functions

Define a function with `fn` and use `return` to return a value:

```val
fn square(x) {
  return x * x
}

println(square(5))
```

Functions are values. Assign them to variables, pass them to another function,
or return them from a function. Anonymous functions use the same block syntax:

```val
fn apply(x, f) {
  return f(x)
}

println(apply(2, fn(x) {
  return x * 3
}))
```

This prints `6`. Functions capture their defining environment and can call
themselves recursively. A `return` without a value returns `null`. Without an
explicit return, a function evaluates to the value of its final statement; an
empty body evaluates to `null`.

See the [built-in reference](builtins.md) for the functions and constants
available in every program, and the
[examples directory](https://github.com/terror/val/tree/master/examples) for
complete programs.
