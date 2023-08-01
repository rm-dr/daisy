 - Minimize parenthesis when printing expressions
 - list and delete variables
 - sin() with units (rad and deg)
 - Prettyprint: no space when implicit multiply?
 - Fix printing 1/2pi, square root parenthesis
 - Re-evaluate variables (a = q + 2, q = 3, a should evaluate to 5)
 - Remove rug dependency (too big, incompatible)

## Pre-release
 - Commands to list constants, units, and substitutions
 - Documentation (usage)
 - Documentation (comments)
 - Print with highlighting
 - Cleanup tests
   - Test commands
   - Test functions (nounit, tobase)
 - Update README
   - demo
   - tricks (a = 2)

## Parser rework
 - Better error when `sin = 2`
 - Define functions (f(x) = ??)
 - Should functions be operators?


## General
 - Optional config file
 - Optional history file
 - Compile to WASM, publish a webapp
 - CLI Options: evaluate, disable replacement, disable special characters
 - Trigonometry & function tests
 - Auto-push to crates.io


## Internals
 - Non-recursive treeify
 - Faster factorial function. Maybe use gamma instead?
 - Remove extra calls to `.clone()` in quantity module
 - Arbitrary precision float (rug doesn't offer arbitrary exponents)
 - Backend-independent colorful printing

## Math Features
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

## Prompt
 - Live syntax/output (like firefox js terminal)
 - Syntax highlight input and output
 - fish-style tab completion
 - Numbered expressions, history recall
 - Better power printing
 - Color configuration
 - Enable/disable unit sets (defaults?)
 - Show base units on error

## Units
 - long prefixes (megatonne, etc)
 - HMS for degrees
 - Exact radians, using pi constant?
 - Weird units: flops, lumen, lux, bel
 - Plural unit names