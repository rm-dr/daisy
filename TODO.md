## Pre-release
 - Fix linelocation (consistent, what does an operator's linelocation mean?)
 - Tuple operations
 - we don't need vectors as arguments to operators
 - Assignment tests
 - Clean up entrypoint mess

## Parser
 - Better error when `sin = 2`
 - Should functions be operators?
 - Binary, hex, octal numbers


## General
 - Optional config file
 - Optional history file
 - daisyrc file
 - Compile to WASM, publish a webapp
 - Options:
   - disable replacement
   - disable special characters
   - 1/ as -1 power toggle
   - powers as superscripts toggle
 - evaluate straight from command line
 - Auto-push to crates.io
 - Package for debian



## Internals
 - Non-recursive treeify
 - Faster factorial function. Maybe use gamma instead?
 - Arbitrary precision float (rug doesn't offer arbitrary exponents)
 - Remove rug dependency (too big, incompatible)

## Math Features
 - Dice
 - Mean, Median, Min
 - Arbitrary base logarithm
 - Derivatives
 - CAS features (trig, roots and powers)
 - Complex numbers
 - acot/acoth functions
 - Sums and products with functional arguments

## Prompt
 - Live syntax/output (like firefox js terminal)
 - Syntax highlight input and output
 - fish-style tab completion
 - Numbered expressions, history recall
 - Color configuration
 - Enable/disable unit sets (defaults?)
 - Consistent unit ordering
 - Better linelocation
   - we shouldn't need to re-print user input on evaluation errors, red arrows should adjust themselves to the prettyprinted string
 - Backend-independent colorful printing
   - Better colors in error texts
 - Better substitution. Consistent: when ascii, when unicode?
 - Command to list substitutions

## Units
 - long prefixes (megatonne, etc)
 - HMS for degrees
 - Exact radians, using pi constant?
 - Weird units: flops, lumen, lux, bel
 - Command to list units