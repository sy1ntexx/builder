use builder::Builder;

#[derive(Builder)]
pub struct Resource {
    pub inline: Option<bool>,
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