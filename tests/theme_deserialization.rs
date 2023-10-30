use theme_test::theme::ThemeVariant;

#[test]
fn can_deserialize_user_theme() {
    const PATH: &'static str = "tests/test_themes/user_theme.toml";
    let contents = std::fs::read_to_string(&PATH).unwrap();
    let _: ThemeVariant = toml::from_str(&contents).unwrap();
}

#[test]
#[should_panic]
fn cannot_deserialize_overrides_with_oob_values() {
    const PATH: &'static str = "tests/test_themes/overrides_out_of_bounds.toml";
    let contents = std::fs::read_to_string(&PATH).unwrap();
    let _: ThemeVariant = toml::from_str(&contents).unwrap();
}
