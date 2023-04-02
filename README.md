![](./misc/banner.png)

A high-precision partially-symbolic calculator with support for units, derivatives, and more.

This is nowhere near complete. Stay tuned.


# TODO

## Before For 1.0 release
 - `+` as a unary operator
 - Compare operators => *, / should have the same priority!
 - Commands
 - Documentation (usage)
 - Documentation (comments)
 - Branding
 - Units


## General
 - CLI Options: version, help, evaluate
 - Compile to WASM => webapp
 - Trig tests
 - Fix tests
   - Direct expression printing
   - Better comparison


## Internals
 - Copy expression
 - Non-recursive treeify
 - Faster factorial function. Maybe use gamma instead?
 - Remove extra calls to `.clone()`
 - Arbitrary precision float (rug doesn't offer arbitrary exponents)

## Features
 - Variable definitions
 - Function definitions
 - Units
 - Dice
 - Mean, Median, Min, Max: arrays and multi-arg functions
 - Derivatives
 - MiniCAS
 - Complex numbers
 - Config file
 - History to file
 - Reference previous results
 - acot/acoth functions

## Prompt
 - Live syntax/output