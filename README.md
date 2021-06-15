
# Builder. Implementation of `builder` pattern in Rust
### Example usage can be found in `examples/01-test.rs`
```rs
use builder::Builder;

#[derive(Builder)]
pub struct Resource {
    pub inline: bool,
    pub value: u32
}

fn main() {
    let t = Resource::builder()
        .inline(false)
        .with_value(321)
        .build();

    assert_eq!(t.value, 321);
    assert_eq!(t.inline, false);
}
```
