# Squeeze

Squeeze consecutive, repeating characters to a single one.

Same as `echo "heeelo" | tr -s 'e'`, where the result would be `helo`.

# Installation

Add it to your dependencies:

```toml
[dependencies]
squeeze = "0.0.1"
```

# Usage

Import the trait:

```rust
use squeeze::Squeeze;

fn main() {
    let mut my_string = String::from("heeelo");
    my_string.squeeze_in_place('e');

    assert_eq!(my_string, "helo");
}
```
