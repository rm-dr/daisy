 - Fix unit autoconversion (Hz * s)
 - Reference previous results
 - Minimize parenthesis when printing expressions
 - Sane autoconversion (mi + km)


## Pre-release
 - Commands to list constants, units, and substitutions
 - Documentation (usage)
 - Documentation (comments)
 - Print with highlighting
 - Cleanup tests
   - Test commands
   - Test functions (nounit, tobase)
 - Releases


## General
 - Optional config file
 - Optional history file
 - Compile to WASM, publish a webapp
 - CLI Options: evaluate, disable replacement, disable special characters
 - Trigonometry & function tests


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
 - HMS for degrees
 - Exact radians, using pi constant?
 - Weird units: flops, lumen, lux, bel

## Bonus
 - Plural unit names
 - Manpage