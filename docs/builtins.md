# Built-ins

These functions and constants are available without imports. Arguments in square
brackets are optional.

## Constants

| Name  | Value                   |
| ----- | ----------------------- |
| `pi`  | π, the circle constant. |
| `tau` | τ, equal to 2π.         |
| `e`   | Euler's number.         |
| `phi` | The golden ratio.       |

Constants are numeric values, not functions. Write `pi`, rather than `pi()`.
They use the configured [calculation precision](numbers.md).

## Mathematics

Trigonometric functions take angles in radians. Inverse trigonometric functions
return angles in radians.

| Function                        | Purpose                                                                 |
| ------------------------------- | ----------------------------------------------------------------------- |
| `sin(x)`, `cos(x)`, `tan(x)`    | Sine, cosine, and tangent.                                              |
| `csc(x)`, `sec(x)`, `cot(x)`    | Cosecant, secant, and cotangent.                                        |
| `asin(x)`, `acos(x)`            | Inverse sine and cosine, for inputs from -1 to 1.                       |
| `arc(x)`                        | Inverse tangent.                                                        |
| `acsc(x)`, `asec(x)`            | Inverse cosecant and secant, for inputs with absolute value at least 1. |
| `acot(x)`                       | Inverse cotangent.                                                      |
| `sinh(x)`, `cosh(x)`, `tanh(x)` | Hyperbolic sine, cosine, and tangent.                                   |
| `ln(x)`                         | Natural logarithm of a positive number.                                 |
| `log2(x)`, `log10(x)`           | Base-2 and base-10 logarithms of a positive number.                     |
| `exp(x)`                        | e raised to the power x.                                                |
| `sqrt(x)`                       | Square root of a non-negative number.                                   |
| `abs(x)`                        | Absolute value.                                                         |
| `ceil(x)`, `floor(x)`           | Round up or down to an integer.                                         |
| `gcd(a, b)`                     | Greatest common divisor of two integers.                                |
| `lcm(a, b)`                     | Least common multiple of two integers.                                  |

```val
println(sqrt(16))
println(gcd(12, 8))
println(log2(8))
```

These calls print `4`, `4`, and `3`.

## Collections and strings

| Function                    | Purpose                                                                    |
| --------------------------- | -------------------------------------------------------------------------- |
| `len(x)`                    | Number of elements in a list or Unicode scalar values in a string.         |
| `sum(list)`                 | Sum the numeric elements of a list. An empty list sums to zero.            |
| `append(list, value)`       | Return a list with a value added at the end.                               |
| `range(start, end[, step])` | Create a list of integers, excluding the end value. The default step is 1. |
| `split(string, delimiter)`  | Split a string into a list of strings.                                     |
| `join(list, delimiter)`     | Join the displayed values of list elements into a string.                  |

`range` accepts integers that fit in a signed 64-bit value. Its step must be
nonzero. Use a negative step for a descending range:

```val
println(range(5, 0, -2))
println(sum([1, 2, 3]))
println(join(["foo", "bar"], ", "))
```

These calls print `[5, 3, 1]`, `6`, and `foo, bar`.

## Conversion

| Function   | Purpose                                                                                                          |
| ---------- | ---------------------------------------------------------------------------------------------------------------- |
| `int(x)`   | Convert a number or numeric string by rounding down. Booleans become 0 or 1.                                     |
| `float(x)` | Convert a number, numeric string, or boolean to an approximate number.                                           |
| `bool(x)`  | Convert to a boolean. Zero, empty strings and lists, and `null` are false. Functions cannot be converted.        |
| `list(x)`  | Return a list unchanged, split a string into Unicode scalar values, or wrap another value in a one-element list. |

`int` rounds toward negative infinity, so `int(-1.5)` is `-2`.

## Input, output, and exit

| Function          | Purpose                                                              |
| ----------------- | -------------------------------------------------------------------- |
| `print(...)`      | Print values separated by spaces, without a trailing newline.        |
| `println(...)`    | Print values separated by spaces, followed by a newline.             |
| `input([prompt])` | Read a line from standard input, optionally printing a prompt first. |
| `exit([code])`    | Exit with a status code, which defaults to 0.                        |
| `quit([code])`    | Alias for `exit`.                                                    |

`print` and `println` return `null`. `input` returns a string without its
trailing line ending. In the browser, input comes from the playground's
**Input** field.
