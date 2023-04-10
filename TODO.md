# TODO

Roadmap for fixes and features.

## Pre-release
 - Commands (help, clear, reset, quit, list constants, list units)
 - Documentation (usage)
 - Documentation (comments)
 - Unit conversion, more units, better printing
 - Unit tests
 - Fix unit power
 - Better unit powers
 - Catch errors when units incompatible
 - Releases

## General
 - CLI Options: version, help, evaluate
 - Compile to WASM, publish a webapp
 - Trigonometry & function tests
 - Manpage
 - Better print parens


## Internals
 - Non-recursive treeify
 - Faster factorial function. Maybe use gamma instead?
 - Remove extra calls to `.clone()` in quantity module
 - Arbitrary precision float (rug doesn't offer arbitrary exponents)


## Math Features
 - Config file
 - History to file
 - Reference previous results
 - Variable definitions
 - Function definitions
 - Units
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
 - "per" as division?


## Prompt
 - Live syntax/output (like firefox js terminal)
 - Syntax highlight input and output
 - fish-style tab completion
 - Numbered expressions, history recall