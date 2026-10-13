# Numbers and precision

Val uses exact rational numbers where possible and arbitrary precision
floating-point numbers for approximate calculations.

## Exact arithmetic

Integer, decimal, and scientific-notation literals are exact:

```val
integer = 42
decimal = 0.1
small = 1e-5
large = 1.25E+2
```

Addition, subtraction, multiplication, and division of exact values preserve
their exact result. Integer powers of exact values are also exact when the
exponent fits in a signed 32-bit integer. Square roots remain exact when both
the numerator and denominator are perfect squares, as in `sqrt(4 / 9)`.

```console
$ val -e '0.1 + 0.2'
0.3
$ val -e '(1 / 3) * 3'
1
```

An exact fraction can still need rounding when displayed. `1 / 3` prints as
`0.3333333333333333` with the default settings, but its stored value remains
exact. Integers and terminating decimals are displayed without discarding
digits; very large or small decimals may use scientific notation.

## Approximate calculations

Trigonometric functions, logarithms, exponentials, and constants such as `pi`
produce approximate values. Roots and powers that cannot use exact arithmetic
also produce approximate results, as does arithmetic involving an approximate
value.

`float(x)` explicitly converts a number, numeric string, or boolean to an
approximate number. A decimal literal such as `0.1` is exact until converted or
used in an approximate calculation.

## Precision and display

`--precision` (`-p`) controls the binary precision used for approximate
calculations. The default is **1024 bits**.

`--digits` (`-d`) controls the number of significant decimal digits displayed
for approximate values and non-terminating exact fractions. The default is **16
digits**.

```sh
val --precision 2048 --digits 50 -e 'pi'
```

Increasing displayed digits does not increase calculation precision. Choose
sufficient precision for the calculation before increasing the display length.

## Rounding modes

Use `--rounding-mode` (`-r`) to choose how approximate calculations round:

| Mode        | Behavior                                                                                       |
| ----------- | ---------------------------------------------------------------------------------------------- |
| `to-even`   | Round to the nearest representable value; ties go to an even significand. This is the default. |
| `to-zero`   | Round toward zero.                                                                             |
| `from-zero` | Round away from zero.                                                                          |
| `up`        | Round toward positive infinity.                                                                |
| `down`      | Round toward negative infinity.                                                                |

```sh
val --precision 128 --rounding-mode down -e 'sqrt(2)'
```

The rounding mode applies to calculations. Decimal output is rounded to the
nearest value for the requested display length.
