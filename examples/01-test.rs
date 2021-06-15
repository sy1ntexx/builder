use builder::Builder;

#[derive(Builder)]
pub struct Resource {
    pub inline: Option<bool>,
    pub value: u32,
    pub r#type: String,
    pub some: (u32, u32)
}

fn main() {
    let t: Resource = Resource::builder()
        .inline(false)
        .with_value(321)
        .with_type(String::from("Test"))
        .with_some((3, 5))
        .build();

    assert_eq!(t.value, 321);
    assert_eq!(t.some, (3, 5));
    assert_eq!(t.inline, Some(false));
    assert_eq!(t.r#type, String::from("Test"));
}