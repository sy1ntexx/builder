
# Builder. Implementation of `builder` pattern in Rust.

## Install
To use library just add it to dependencies in your `Cargo.toml` file.
```toml
[dependencies]
builder = { git = "https://github.com/sy1ntexx/builder" }
```

## Usage
Example usage can be found in `examples/01-test.rs`.
```rs
use builder::Builder;

#[derive(Builder)]
pub struct Resource {
    pub inline: bool,
    pub value: u32,
    #[default(5)] // [default(value)] assigns a default value to a field
    pub five: i8,
    #[skip] // Use [skip] to ignore field in builder
    #[default(String::from("Hello :D"))]
    pub hidden: String
}

fn main() {
    let t: Resource = Resource::builder()
        .inline(false)
        // .with_hidden(String::from("Test")) - Error
        .with_value(321)
        .build();

    assert_eq!(t.five, 5);
    assert_eq!(t.hidden, String::from("Hello :D"));
    assert_eq!(t.value, 321);
    assert_eq!(t.inline, false);
}
```
