## Pre-release
 - Commands to list units and substitutions
 - Fix large power printing (superscript exponential notation)

## Parser
 - Better error when `sin = 2`
 - Define functions (f(x) = ??)
 - Should functions be operators?
 - Multi-argument functions
 - Binary, hex, octal numbers


## General
 - Optional config file
 - Optional history file
 - Compile to WASM, publish a webapp
 - Options:
   - disable replacement
   - disable special characters
   - 1/ as -1 power toggle
   - powers as superscripts toggle
 - evaluate straight from command line
 - Auto-push to crates.io
 - When should we subsitute? Should variables like "tau" be stored as unicode internally?
   - Make sure unicode variable deletion works!


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

## Units
 - long prefixes (megatonne, etc)
 - HMS for degrees
 - Exact radians, using pi constant?
 - Weird units: flops, lumen, lux, bel