 - Constant definitions
 - Reference previous results
 - Minimize parenthesis when printing expressions
 - Sane autoconversion (mi + km)
 - "per" as division?
 - -> as "to"
 - strip_unit function


## Pre-release
 - Commands to list constants, units, and substitutions
 - Documentation (usage)
 - Documentation (comments)
 - Build.rs for constants
 - Print with highlighting
 - Cleanup tests, test commands, test units, trig/function tests
 - Releases


## General
 - Optional config file
 - Optional history file
 - Compile to WASM, publish a webapp
 - CLI Options: evaluate, disable replacement, disable special characters
 - Trigonometry & function tests
 - vhs demo


## Internals
 - Non-recursive treeify
 - Faster factorial function. Maybe use gamma instead?
 - Remove extra calls to `.clone()` in quantity module
 - Arbitrary precision float (rug doesn't offer arbitrary exponents)
 - Backend-independent colorful printing

## Math Features
 - Variable definitions (list and delete commands)
 - Function definitions
 - Dice
 - Mean, Median, Min, Max: arrays and multi-argument functions
 - Arbitrary base logarithm
 - Derivatives
 - CAS features (trig, roots and powers)
 - Complex numbers
 - acot/acoth functions
 - Binary, hex, octal numbers
 - Extra roots? (for convenience)
 - Sums and products
 - Unit info command
 - Constant info command
 - Convert to base unit

## Prompt
 - Live syntax/output (like firefox js terminal)
 - Syntax highlight input and output
 - fish-style tab completion
 - Numbered expressions, history recall
 - Better power printing
 - Color configuration
 - Show base units on error

## Units
 - long prefixes (megatonne, etc)
 - Unit order h\*kW vs kW\*h
 - Print units with powers instead of /
 - HMS for degrees
 - Exact radians, using pi constant?
 - Constant toml file (update doc in units.toml)
 - Weird units: flops, lumen, lux, bel

## Bonus
 - Plural unit names
 - Manpage