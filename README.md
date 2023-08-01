![](./misc/banner.png)

A high-precision scientific calculator with support for units, derivatives, and more.

Many features are missing, this is still under development.


# Features
 - Open-source
 - Extremely high precision
   - Uses a rational datatype when possible, and a high-precision float when not.
 - Pretty printing in prompt (with special substitutions)
 - Supports many physical units, with metric and binary prefixes
 - Supports exponential notation
 - Clear syntax, parsed input is always re-printed as a sanity check.
 - Useful, detailed error messages


# Usage

All documentation is built into the prompt. Use the `help` command to view it.

## Evaluate expressions:
 - Basic math: ``103 / 2 * 43``
 - Functions: ``sqrt(1.4^3 + 4) * sin(pi / 4)``
 - Scientific notation: ``1.2e12 * 1e-5``

## Physical units
 - Unit operations: ``2 day + 1 hour``
 - Unit conversion: ``2 day + 1 hour to minutes``
 - Compound units: ``10 m/s to mph``
 - Conversion errors: ``1 liter to volt``

## Varables
 - Previous answer: `ans + 2`
 - Variable assignment: `a = 143`


# Notes and Tricks

## Unit Conversion

The conversion operator `to` converts its left argument to the *unit* of its right argument, ignoring its value. For example, `5m to mi` and `5m to 10mi` are identical.


## Multiplication Order

Implicit multiplication has a higher priority than division. `1/2 pi` will parse as `1/(2pi)`. Type `(1/2) pi` or `1/2 * pi` to get half of pi.

## Inline Assignment

The assignment operator `=` returns its value, and can thus be used inside of an expression. For example, `(a = 2) + 2` assigns `a` to `2` and returns `4`. This only works for variable assignment.