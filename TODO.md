## Version Bump checklist
 - update Cargo.toml
 - run cargo test
 - commit
 - push
 - merge
 - git tag -a v1.0.0 -m "Version 1.0.0" on merge commit
 - cargo publish
 - Build wasm & push changes
 - Update AUR package

## Pre-release
 - Tuple operations
 - we don't need vectors as arguments to operators

## Parser
 - Should functions be operators?
 - Binary, hex, octal numbers

## General
 - Better tests (assignment, many expressions in one context)
 - Optional config file
 - Optional history file
 - Package for debian, nix
 - Non-recursive treeify
 - Faster factorial function. Maybe use gamma instead?
 - Arbitrary precision floats

## Math Features
 - Mean, Median, Min
 - Arbitrary base logarithm
 - Complex numbers
 - acot/acoth functions
 - Sums and products with functional arguments
 - Add functions: gcd, inverse mod, dice

## Prompt
 - Fix terminal color detection
 - Live syntax/output (like firefox js terminal)
 - Syntax highlighting
 - Numbered history recall
 - Enable/disable unit sets (defaults?)
 - Consistent unit ordering

## Units
 - long prefixes (megatonne, etc)
 - HMS for degrees
 - Exact radians, using pi constant?
 - Weird units: flops, lumen, lux, bel
 - Command to list units