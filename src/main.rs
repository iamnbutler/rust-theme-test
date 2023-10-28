use std::borrow::Cow;
use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};

pub trait Theme {
    fn colors(&self) -> &ThemeColors<'_>;
    fn to_file(&self, path: &str) -> Result<(), anyhow::Error> {
        let file = std::fs::File::create(path)?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self.colors()).map_err(|err| anyhow::anyhow!(err))
    }
    fn player_colors(&self) -> [ThemeColor; 8] {
        [
            self.colors().player_1.clone(),
            self.colors().player_2.clone(),
            self.colors().player_3.clone(),
            self.colors().player_4.clone(),
            self.colors().player_5.clone(),
            self.colors().player_6.clone(),
            self.colors().player_7.clone(),
            self.colors().player_8.clone(),
        ]
    }
}

type ThemeColor<'a> = Cow<'a, str>;

#[derive(Serialize, Deserialize)]
pub struct ThemeColors<'a> {
    pub fg: Cow<'a, str>,
    pub bg: Cow<'a, str>,
    pub border: Cow<'a, str>,
    pub player_1: Cow<'a, str>,
    pub player_2: Cow<'a, str>,
    pub player_3: Cow<'a, str>,
    pub player_4: Cow<'a, str>,
    pub player_5: Cow<'a, str>,
    pub player_6: Cow<'a, str>,
    pub player_7: Cow<'a, str>,
    pub player_8: Cow<'a, str>,
}

macro_rules! system_theme {
    ($($color:ident => $value:expr),*) => {
        {
            SystemTheme { colors: ThemeColors {
                $($color: Cow::Borrowed($value)),*
            } }
        }
    };
}

pub struct SystemTheme<'a> {
    colors: ThemeColors<'a>,
}

impl<'a> SystemTheme<'a> {
    fn new() -> Self {
        let theme = system_theme! {
            fg => "#ffffff",
            bg => "#000000",
            border => "#333333",
            player_1 => "#ff0000",
            player_2 => "#00ff00",
            player_3 => "#0000ff",
            player_4 => "#ffff00",
            player_5 => "#ff00ff",
            player_6 => "#00ffff",
            player_7 => "#ffffff",
            player_8 => "#000000"
        };

        theme
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
    println!(
        "fg: {}, bg: {}, border: {}",
        colors.fg,
        colors.bg,
        colors.border
    );
}
