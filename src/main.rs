use std::borrow::Cow;
use anyhow::{Result, Context};

pub trait Theme {
    fn colors(&self) -> &ThemeColors;
    fn to_file(&self, path: &str) -> Result<(), anyhow::Error> {
        let file = std::fs::File::create(path)?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self.colors()).map_err(|err| anyhow::anyhow!(err))
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct ThemeColors<'a> {
    pub fg: Cow<'a, str>,
    pub bg: Cow<'a, str>,
    pub border: Cow<'a, str>,
}

pub struct SystemTheme<'a> {
    colors: ThemeColors<'a>,
}

impl<'a> SystemTheme<'a> {
    fn new() -> Self {
        SystemTheme {
            colors: ThemeColors {
                fg: Cow::Borrowed("white"),
                bg: Cow::Borrowed("blue"),
                border: Cow::Borrowed("gray"),
            },
        }
    }
}

pub struct UserTheme<'a> {
    colors: ThemeColors<'a>,
}

impl<'a> UserTheme<'a> {
    fn from_file(path: &str) -> Result<Self> {
        let file = std::fs::File::open(path).context("Unable to open theme file")?;
        let reader = std::io::BufReader::new(file);
        let colors: ThemeColors = serde_json::from_reader(reader).context("Failed to deserialize theme")?;
        Ok(UserTheme { colors })
    }
}

impl<'a> Theme for UserTheme<'a> {
    fn colors(&self) -> &ThemeColors {
        &self.colors
    }
}

impl<'a> From<SystemTheme<'a>> for UserTheme<'a> {
    fn from(system_theme: SystemTheme<'a>) -> UserTheme<'a> {
        UserTheme {
            colors: system_theme.colors,
        }
    }
}

impl<'a> Theme for SystemTheme<'a> {
    fn colors(&self) -> &ThemeColors {
        &self.colors
    }
}

fn main() -> anyhow::Result<()> {
    let system_theme = SystemTheme::new();
    println!("System Theme:");
    print_theme_colors(system_theme.colors());

    println!("\nUser Theme:");
    let user_theme = UserTheme::from_file("theme/test_theme.json")?;
    print_theme_colors(user_theme.colors());

    let system_theme2 = SystemTheme::new();
    let user_theme_from_system: UserTheme = system_theme2.into();
    println!("\nUser Theme from System Theme:");
    print_theme_colors(user_theme_from_system.colors());

    println!("\nWriting themes.");
    system_theme.to_file("theme/system_theme.json")?;
    user_theme.to_file("theme/user_theme.json")?;
    user_theme_from_system.to_file("theme/user_theme_from_system.json")?;

    Ok(())
}

fn print_theme_colors(colors: &ThemeColors) {
    println!("fg: {}, bg: {}, border: {}", colors.fg, colors.bg, colors.border);
}
