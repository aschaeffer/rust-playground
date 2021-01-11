# Other useful libraries

## Colorgrad

* https://crates.io/crates/colorgrad

```rust
use colorgrad::Color;

pub fn main() {
    let g = colorgrad::CustomGradient::new()
        .colors(&[
            Color::from_rgb_u8(0, 206, 209),
            Color::from_rgb_u8(255, 105, 180),
            Color::from_rgb(0.274, 0.5, 0.7),
            Color::from_hsv(50., 1., 1.),
            Color::from_hsv(348., 0.9, 0.8),
        ])
        .build()
        .unwrap();
}
```
