![](./misc/banner.png)

A high-precision scientific calculator with support for units, derivatives, and more.

**todo:** ~~Try the [web version!]()~~

Many features are missing, this is still under development.


# Features
 - Extremely high precision
   - Uses a rational datatype when possible,
   - Uses a high-precision float when not.
 - Pretty printing in prompt
   - Makes expressions shorter and easier to understand
 - **todo:** ~~Supports nearly all physical units~~
   - ~~Accounts for units in calculations~~
   - ~~Explicit conversion operator `->` or `to`~~
 - **todo:** ~~Variable and function definitions~~
 - **todo:** ~~Easy history recall~~
 - **todo:** ~~Sums and products~~


# Reference

## **Operators, sorted by priority (high to low)**

| Operator                  | Syntax               |
|---------------------------|----------------------|
| function application      | `sin`, `cos`, etc    |
| factorial                 | `!`                  |
| powers                    | `^`, `**`            |
| implicit multiplication   | `3π`, `3(2+1)`, etc  |
| square root               | `sqrt`, `rt`, `√`    |
| negation                  | `-3`, `-(1 + 2)`     |
| modulo (short)            | `%`                  |
| multiplication, division  | `*`, `/`, `×`, `÷`   |
| addition, subtraction     | `+`, `-`             |
| modulo (long)             | `mod`                |

Note that implicit multiplication has a higher priority than mulitiplication and division.

## **Functions**

| Function                  | Syntax                      |
|---------------------------|-----------------------------|
| Absolute Value            | `abs`                       |
| Floor, Ceiling, Round     | `floor`, `ceil`, `round`    |
| Logarithm (base e)        | `ln`                        |
| Logarithm (base 10)       | `log`                       |
| sin, arcsin, cosecant     | `sin`, `asin`, `csc`        |
| cos, arccos, secant       | `cos`, `acos`, `sec`        |
| tan, arctan, cotan        | `tan`, `atan`, `cot`        |
| hyperbolic sin, etc       | `sinh`, `asinh`, `csch`     |
| hyperbolic cos, etc       | `cosh`, `acosh`, `sech`     |
| hyperbolic tan, etc       | `tanh`, `atanh`, `coth`     |