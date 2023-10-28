pub trait Theme {
    fn colors(&self) -> &ThemeColors;
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ThemeColors {
    pub fg: String,
    pub bg: String,
    pub border: String,
}

pub struct SystemTheme {
    colors: ThemeColors,
}

impl SystemTheme {
    fn new() -> Self {
        SystemTheme {
            colors: ThemeColors {
                fg: "white".into(),
                bg: "blue".into(),
                border: "gray".into(),
            }
        }
    }
}

pub struct UserTheme {
    colors: ThemeColors,
}

impl UserTheme {
    fn from_file(path: &str) -> std::io::Result<Self> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let colors: ThemeColors = serde_json::from_reader(reader)?;
        Ok(UserTheme { colors })
    }
}

impl Theme for UserTheme {
    fn colors(&self) -> &ThemeColors {
        &self.colors
    }
}

impl Theme for SystemTheme {
    fn colors(&self) -> &ThemeColors {
        &self.colors
    }
}

fn main() -> std::io::Result<()> {
    let system_theme = SystemTheme::new();
    println!("System Theme:");
    print_theme_colors(system_theme.colors());

    println!("\nUser Theme:");
    let user_theme = UserTheme::from_file("test_theme.json")?;
    print_theme_colors(user_theme.colors());

    Ok(())
}

fn print_theme_colors(colors: &ThemeColors) {
    println!("fg: {}, bg: {}, border: {}", colors.fg, colors.bg, colors.border);
}
