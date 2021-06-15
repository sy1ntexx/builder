use builder::Builder;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Val {
    Test,
    Group,
}

#[derive(Builder)]
pub struct Resource {
    pub inline: Option<bool>,
    #[skip]
    #[default(Val::Test)]
    pub value: Val,
    pub r#type: String,
    pub some: (u32, u32),
    pub r#match: Option<bool>,
}

fn main() {
    let t: Resource = Resource::builder()
        .inline(false)
        .with_type(String::from("Test"))
        .with_some((3, 5))
        .r#match(true)
        .build();

    assert_eq!(t.some, (3, 5));
    assert_eq!(t.value, Val::Test);
    assert_eq!(t.inline, Some(false));
    assert_eq!(t.r#match, Some(true));
    assert_eq!(t.r#type, String::from("Test"));
}
