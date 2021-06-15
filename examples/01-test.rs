use builder::Builder;

#[derive(Builder)]
pub struct Resource {
    pub inline: bool,
    pub value: u32,
    #[default(5)]
    pub five: i8,
    #[skip]
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