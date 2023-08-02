![](./misc/banner.png)

A high-precision scientific calculator with support for units, derivatives, and more.

Many features are missing, this is still under development.

# 📦 Installation
 - **Arch:** `yay -S daisy`
 - **Debian:** coming soon

From source: `cargo build --release` \
Binary will be in `target/release/daisy`


# 📹 Screenshot

![](https://betalupi.com/static/git/daisy.png)

# 🛠️ Features
 - Open-source
 - Extremely high precision
   - Uses a rational datatype when possible, and a high-precision float when not.
 - Pretty printing in prompt (with special substitutions)
 - Supports many physical units, with metric and binary prefixes
 - Supports exponential notation
 - Clear syntax, parsed input is always re-printed as a sanity check.
 - Useful, detailed error messages


# 📑 Usage

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


# 🌹 Additional Notes

## Unit Conversion

The conversion operator `to` converts its left argument to the *unit* of its right argument, ignoring its value. For example, `5m to mi` and `5m to 10mi` are identical.


## Celsius and Fahrenheit

Celsius and Fahrenheit are not supported as first-class units because they require an offset when converting from other temperature units. This leads to ambiguity when adding units, since one temperature must be seen as a *difference* rather than an absolute temperature.

Daisy instead provides four functions (`fromCelsius`, `toCelsius`, `fromFahrenheit`, `toFahrenheit`) which convert between scalars and Kelvin.
 - "from" functions take a scalar and return a value in Kelvin: `fromCelsius(0) = 273.15K`
 - "to" functions take a value in Kelvin and return a scalar: `toCelsius(273.15 K) = 0`


## Multiplication Order

Implicit multiplication has a higher priority than division. `pi/2 radians` will parse as `pi/(2 radians)`. Type `(pi/2) radians` or `pi/2 * radians` to get 90 degrees.


## Inline Assignment

The assignment operator `=` returns its value, and can thus be used inside of an expression. For example, `(a = 2) + 2` assigns `a` to `2` and returns `4`. This only works for variable assignment.