# Squeeze

Squeeze consecutive, repeating characters to a single one.

Same as `echo "heeelo" | tr -s 'e'`, where the result would be `helo`.

This crate provides an extension trait, [`SqueezeExt`], which when brought
into scope will provide two methods on [`String`]s, `String::squeeze` and
`String::squeeze_in_place`.

The methods are implemented with a standalone function, `squeeze`, which may
be used when preferred.

Note that multiple occurences of consecutive characters are also replaces.
For example, given the input "heeeleee", the output would be "hele".

Additionally, variable cased consecutive characters will not be removed.
For example, given the input "hEeElEeE", the output will be unchanged.

# Installation

Add it to your dependencies:

```toml
[dependencies]
squeeze = "0.0.1"
```

# Usage

Import the trait:

```rust
use squeeze::{squeeze, SqueezeExt};

// With standalone function
let mut my_string = String::from("heeelo");
squeeze(&mut my_string, 'e');

assert_eq!(my_string, "helo");

// With method
let mut squeezable = String::from("heeelo");
let squeezed = squeezable.squeeze('e');

assert_eq!(squeezed, "helo");

// With method, but operate in place
let mut squeezable_in_place = String::from("heeelo");
squeezable_in_place.squeeze_in_place('e');

assert_eq!(squeezable_in_place, "helo");

// Happy squeezing!
```
