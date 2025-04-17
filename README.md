## val

[![release](https://img.shields.io/github/release/terror/val.svg?label=release&style=flat&labelColor=282c34&logo=github)](https://github.com/terror/val/releases/latest)
[![crates.io](https://shields.io/crates/v/val.svg)](https://crates.io/crates/val)
[![CI](https://github.com/terror/val/actions/workflows/ci.yaml/badge.svg)](https://github.com/terror/val/actions/workflows/ci.yaml)
[![docs.rs](https://img.shields.io/docsrs/val)](https://docs.rs/val)
[![dependency status](https://deps.rs/repo/github/terror/val/status.svg)](https://deps.rs/repo/github/terror/val)

**val** (e**val**) is a simple arbitrary precision calculator language built
on top of [**chumsky**](https://github.com/zesterer/chumsky) and
[**ariadne**](https://github.com/zesterer/ariadne).

<img width="1667" alt="Screenshot 2025-04-16 at 1 57 23 AM" src="https://github.com/user-attachments/assets/c295e572-90fa-4b33-ace0-baad1ead64fd" />

## Installation

`val` should run on any system, including Linux, MacOS, and the BSDs.

The easiest way to install it is by using [cargo](https://doc.rust-lang.org/cargo/index.html),
the Rust package manager:

```bash
cargo install val
```

### Pre-built binaries

Pre-built binaries for Linux, MacOS, and Windows can be found on [the releases
page](https://github.com/terror/val/releases).

## Usage

The primary way to use **val** is via the provided command-line interface. There
is currently ongoing work on a Rust library and web playground, which will
provide a few extra ways to interact with the runtime.

Below is the output of `val --help`, which describes some of the
arguments/options we support:

```present cargo run -- --help
Usage: val [OPTIONS] [FILENAME]

Arguments:
  [FILENAME]

Options:
  -e, --expression <EXPRESSION>
  -p, --precision <PRECISION>          [default: 1024]
  -r, --rounding-mode <ROUNDING_MODE>  [default: to-even]
  -h, --help                           Print help
  -V, --version                        Print version
```

Running **val** on its own will spawn a repl (read–eval–print loop) environment,
where you can evaluate arbitrary **val** code and see its output immediately. We
use [rustyline](https://github.com/kkawakam/rustyline) for its implementation,
and we support a few quality of life features:

- Syntax highlighting (see image above)
- Persistent command history
- Emacs-style editing support by default
- Filename completions
- Hints (virtual text pulled from history)

The **val** language supports not only expressions, but quite a few
[statements](https://github.com/terror/val/blob/ea0c163934ee3f4afe118384b1281d296f116539/src/ast.rs#L35) as well.
You may want to save **val** programs and execute them later, so
the command-line interface provides a way to evaluate entire files.

For instance, lets say you have the following **val** program at
`factorial.val`:

```rust
fn factorial(n) {
  if (n <= 1) {
    return 1
  } else {
    return n * factorial(n - 1)
  }
}

println(factorial(5));
```

You can execute this program by running `val factorial.val`, which will write to
standard output `120`.

Lastly, you may want to evaluate a **val** expression and use it within another
program. The tool supports executing arbitrary expressions inline using the
`--expression` or `-e` option:

```bash
val -p 53 -e 'sin(2) * e ^ pi * cos(sum([1, 2, 3]))'
1.6481455793912883588e+1
```

**n.b.** The `--expression` option and `filename` argument are mutually
exclusive.

## Features

This section describes some of the language features **val** implements in
detail, and should serve as a guide to anyone wanting to write a **val**
program.

### Statements

**val** supports a few statement constructs such as `if`, `while`, `loop`, `fn`,
`return`, etc. Check out the [grammar](https://github.com/terror/val/blob/master/GRAMMAR.txt)
for all of the various statement types.

Here's an example showcasing most of them in action:

```rust
fn fib(n) {
  if (n <= 1) { return n }
  return fib(n - 1) + fib(n - 2)
}

i = 0

while (i < 10) {
  println('fib(' + i + ') = ' + fib(i))
  i = i + 1
}
```

### Expressions

**val** supports a variety of expressions that can be combined to form more
complex operations:

| Category       | Operation             | Syntax                        | Example                              |
| -------------- | --------------------- | ----------------------------- | ------------------------------------ |
| **Arithmetic** | Addition              | `a + b`                       | `1 + 2`                              |
|                | Subtraction           | `a - b`                       | `5 - 3`                              |
|                | Multiplication        | `a * b`                       | `4 * 2`                              |
|                | Division              | `a / b`                       | `10 / 2`                             |
|                | Modulo                | `a % b`                       | `7 % 3`                              |
|                | Exponentiation        | `a ^ b`                       | `2 ^ 3`                              |
|                | Negation              | `-a`                          | `-5`                                 |
| **Logical**    | And                   | `a && b`                      | `true && false`                      |
|                | Or                    | <code>a &#124;&#124; b</code> | <code>true &#124;&#124; false</code> |
|                | Not                   | `!a`                          | `!true`                              |
| **Comparison** | Equal                 | `a == b`                      | `x == 10`                            |
|                | Not Equal             | `a != b`                      | `y != 20`                            |
|                | Less Than             | `a < b`                       | `a < b`                              |
|                | Less Than or Equal    | `a <= b`                      | `i <= 5`                             |
|                | Greater Than          | `a > b`                       | `count > 0`                          |
|                | Greater Than or Equal | `a >= b`                      | `value >= 100`                       |
| **Other**      | Function Call         | `function(args)`              | `sin(x)`                             |
|                | List Indexing         | `list[index]`                 | `numbers[0]`                         |
|                | List Creation         | `[item1, item2, ...]`         | `[1, 2, 3]`                          |
|                | List Concatenation    | `list1 + list2`               | `[1, 2] + [3, 4]`                    |
|                | String Concatenation  | `string1 + string2`           | `"Hello, " + name`                   |
|                | Variable Reference    | `identifier`                  | `x`                                  |

### Values

**val** has several primitive value types:

#### Number

Numeric values are represented as arbitrary floating point numbers:

```rust
> pi
3.141592653589793115997963468544185161590576171875e+0
> e
2.718281828459045090795598298427648842334747314453125e+0
> sin(2) * e ^ pi * cos(sum([1, 2, 3]))
1.64814557939128835908118223753548409318930600432600320575175542910885566534716862696709583557263450637540094805515971245058657340687939442764118452427864231041058959960049996970569867866035825048029794926250103816423751837050040821914044725396611746570949840536443560831710407959633707222226883928822125018008e+1
>
```

You can specify the rounding mode, and what sort of precision you'd like to see
in the output by using the `--rounding-mode` and `--precision` options
respectively.

#### Boolean

Boolean values represent truth values:

```rust
a = true
b = false
c = a && b
d = a || b
e = !a
```

#### String

Text values enclosed in single or double quotes:

```rust
greeting = "Hello"
name = 'World'
message = greeting + ", " + name + "!"
```

#### List

Collections of values of any type:

```rust
numbers = [1, 2, 3, 4, 5]
mixed = [1, "two", true, [3, 4]]
empty = []
first = numbers[0]
numbers[0] = 10
combined = numbers + [6, 7]
```

#### Function

A function is a value, and can be used in assignments, passed around to other
functions, etc.

Check out the [higher order functions example](https://github.com/terror/val/blob/master/examples/hoc.val)
for how this works.

```rust
fn reduce(l, f, initial) {
  i = 0

  result = initial

  while (i < len(l)) {
    result = f(result, l[i])
    i = i + 1
  }

  return result
}

fn sum(a, b) {
  return a + b
}

l = [1, 2, 3, 4, 5]

println(reduce(l, sum, 0))
```

### Built-ins

**val** offers a ton of built-in functions and constants. Below documents them:

| Category          | Function/Constant   | Description                        | Example                  |
| ----------------- | ------------------- | ---------------------------------- | ------------------------ |
| **Constants**     | `pi`                | Mathematical constant π (≈3.14159) | `area = pi * r^2`        |
|                   | `e`                 | Mathematical constant e (≈2.71828) | `growth = e^rate`        |
| **Trigonometric** | `sin(x)`            | Sine of x (radians)                | `sin(pi/2)`              |
|                   | `cos(x)`            | Cosine of x (radians)              | `cos(0)`                 |
|                   | `tan(x)`            | Tangent of x (radians)             | `tan(pi/4)`              |
|                   | `csc(x)`            | Cosecant of x (radians)            | `csc(pi/6)`              |
|                   | `sec(x)`            | Secant of x (radians)              | `sec(0)`                 |
|                   | `cot(x)`            | Cotangent of x (radians)           | `cot(pi/4)`              |
| **Inverse Trig**  | `asin(x)`           | Arc sine (-1≤x≤1)                  | `asin(0.5)`              |
|                   | `acos(x)`           | Arc cosine (-1≤x≤1)                | `acos(0.5)`              |
|                   | `arc(x)`            | Arc tangent                        | `arc(1)`                 |
|                   | `acsc(x)`           | Arc cosecant (abs(x)≥1)            | `acsc(2)`                |
|                   | `asec(x)`           | Arc secant (abs(x)≥1)              | `asec(2)`                |
|                   | `acot(x)`           | Arc cotangent                      | `acot(1)`                |
| **Hyperbolic**    | `sinh(x)`           | Hyperbolic sine                    | `sinh(1)`                |
|                   | `cosh(x)`           | Hyperbolic cosine                  | `cosh(1)`                |
|                   | `tanh(x)`           | Hyperbolic tangent                 | `tanh(1)`                |
| **Logarithmic**   | `ln(x)`             | Natural logarithm                  | `ln(e)`                  |
|                   | `log2(x)`           | Base-2 logarithm                   | `log2(8)`                |
|                   | `log10(x)`          | Base-10 logarithm                  | `log10(100)`             |
|                   | `e(x)`              | e raised to power x                | `e(2)`                   |
| **Numeric**       | `sqrt(x)`           | Square root (x≥0)                  | `sqrt(16)`               |
|                   | `ceil(x)`           | Round up to integer                | `ceil(4.3)`              |
|                   | `floor(x)`          | Round down to integer              | `floor(4.7)`             |
|                   | `abs(x)`            | Absolute value                     | `abs(-5)`                |
| **Collections**   | `len(x)`            | Length of list or string           | `len("hello")`           |
|                   | `sum(list)`         | Sum list elements                  | `sum([1,2,3])`           |
| **Conversion**    | `int(x)`            | Convert to integer                 | `int("42")`              |
|                   | `float(x)`          | Convert to float                   | `float("3.14")`          |
|                   | `bool(x)`           | Convert to boolean                 | `bool(1)`                |
|                   | `list(x)`           | Convert to list                    | `list("abc")`            |
| **I/O**           | `print(...)`        | Print without newline              | `print("Hello")`         |
|                   | `println(...)`      | Print with newline                 | `println("World")`       |
|                   | `input([prompt])`   | Read line from stdin               | `name = input("Name: ")` |
| **String**        | `split(str, delim)` | Split string                       | `split("a,b,c", ",")`    |
|                   | `join(list, delim)` | Join list elements                 | `join(["a","b"], "-")`   |
| **Program**       | `exit([code])`      | Exit program                       | `exit(1)`                |
|                   | `quit([code])`      | Alias for exit                     | `quit(0)`                |

## Prior Art

[bc(1)](https://linux.die.net/man/1/bc) - An arbitrary precision calculator
language
