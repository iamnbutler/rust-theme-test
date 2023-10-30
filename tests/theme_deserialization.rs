use theme_test::theme::ThemeVariant;

#[test]
fn can_deserialize_user_theme() {
    const PATH: &'static str = "theme/user_theme.toml";
    let contents = std::fs::read_to_string(&PATH).unwrap();
    let theme: ThemeVariant = toml::from_str(&contents).unwrap();
}
