- Fix units

## Pre-release
 - Commands to list constants, units, and substitutions
 - Documentation (usage)
 - Documentation (comments)
 - More units, better printing
 - Cleanup tests, test commands
 - Unit tests
 - Plural unit names
 - Releases
 - Unit substitutions
 - mpg and gal replacement (10 mpg * 3 gal)

## General
 - Compile to WASM, publish a webapp
 - CLI Options: evaluate, disable replacement
 - Trigonometry & function tests
 - Manpage


## Internals
 - Non-recursive treeify
 - Faster factorial function. Maybe use gamma instead?
 - Remove extra calls to `.clone()` in quantity module
 - Arbitrary precision float (rug doesn't offer arbitrary exponents)
 - Backend-independent colorful printing

## Math Features
 - Config file
 - History to file
 - Reference previous results
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
 - "per" as division?
 - -> as "to"
 - strip_unit function
 - Unit info command
 - Constant info command


## Prompt
 - Live syntax/output (like firefox js terminal)
 - Syntax highlight input and output
 - fish-style tab completion
 - Numbered expressions, history recall
 - Better power printing
 - Color configuration?