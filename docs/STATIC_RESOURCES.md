# Include static resources (files) in the binary

## Motivation

Some resources are necessary to run the program. It makes things much easier if essential resources
are compiled in the binary.

## Examples

* The logo
* The missing texture texture
* Immutable startup code (javascript)
* Immutable web resources for the user interface (HTML, JS, CSS)

## Resources

* https://crates.io/crates/rust-embed
* https://github.com/pyros2097/rust-embed
* https://crates.io/crates/include-flate
* include_bytes (standard library)
