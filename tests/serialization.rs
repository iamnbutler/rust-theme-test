use theme_test::theme::{ThemeVariant, serialize_theme};

// TODO: Test failing due to newlines in the serialized theme.
#[test]
fn can_serialize_user_theme() {
    const PATH: &'static str = "tests/test_themes/user_theme.toml";
    let contents = std::fs::read_to_string(&PATH).unwrap();
    let theme: ThemeVariant = toml::from_str(&contents).unwrap();

    let new_contents = serialize_theme(theme).unwrap();
    assert_eq!(contents, new_contents);
}
