use std::borrow::Cow;
use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};

pub trait Theme {
    fn colors(&self, v: usize) -> Result<&ThemeColors>;
    fn player_colors(&self, v: usize) -> Result<[ThemeColor; 8]>;
    fn to_file(&self, path: &str) -> Result<(), anyhow::Error> {
        let file = std::fs::File::create(path)?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self.get_config()).map_err(From::from)
    }
    fn get_config(&self) -> &ThemeConfig;
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

#[derive(Serialize, Deserialize)]
pub struct ThemeVariant<'a> {
    pub title: Cow<'a, str>,
    pub appearance: Cow<'a, str>,
    pub colors: ThemeColors<'a>,
}

#[derive(Serialize, Deserialize)]
pub struct ThemeConfig<'a> {
    pub name: Cow<'a, str>,
    pub author: Option<Cow<'a, str>>,
    pub url: Option<Cow<'a, str>>,
    pub variants: Vec<ThemeVariant<'a>>,
}

pub struct SystemTheme<'a> {
    config: ThemeConfig<'a>,
}

macro_rules! system_colors {
    ($($color:ident => $value:expr),*) => {
        {
            ThemeColors {
                $($color: Cow::Borrowed($value)),*
            }
        }
    };
}

impl<'a> SystemTheme<'a> {
    fn new() -> Self {
        let config = ThemeConfig {
            name: Cow::Borrowed("Zed"),
            author: Some(Cow::Borrowed("Zed Industries")),
            url: None,
            variants: vec![ThemeVariant {
                title: Cow::Borrowed("Zed Light"),
                appearance: Cow::Borrowed("light"),
                colors: system_colors! {
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
                },
            }],
        };

        SystemTheme { config }
    }
}

impl<'a> Theme for SystemTheme<'a> {
    fn colors(&self, v: usize) -> Result<&ThemeColors> {
        self.config.variants.get(v).map(|variant| &variant.colors).context("Index out of range")
    }
    fn player_colors(&self, v: usize) -> Result<[ThemeColor; 8]> {
        let colors = self.colors(v)?;
        Ok([
            colors.player_1.clone(),
            colors.player_2.clone(),
            colors.player_3.clone(),
            colors.player_4.clone(),
            colors.player_5.clone(),
            colors.player_6.clone(),
            colors.player_7.clone(),
            colors.player_8.clone(),
        ])
    }
    fn get_config(&self) -> &ThemeConfig {
        &self.config
    }
}

pub struct UserTheme<'a> {
    config: ThemeConfig<'a>,
}

impl<'a> UserTheme<'a> {
    fn from_file(path: &str) -> Result<Self> {
        let file = std::fs::File::open(path).context("Unable to open theme file")?;
        let reader = std::io::BufReader::new(file);
        let config: ThemeConfig = serde_json::from_reader(reader)
            .context("Failed to deserialize theme")?;
        Ok(UserTheme{ config })
    }
}

impl<'a> Theme for UserTheme<'a> {
    fn colors(&self, v: usize) -> Result<&ThemeColors> {
        self.config.variants.get(v).map(|variant| &variant.colors).context("Index out of range")
    }
    fn player_colors(&self, v: usize) -> Result<[ThemeColor; 8]> {
        let colors = self.colors(v)?;
        Ok([
            colors.player_1.clone(),
            colors.player_2.clone(),
            colors.player_3.clone(),
            colors.player_4.clone(),
            colors.player_5.clone(),
            colors.player_6.clone(),
            colors.player_7.clone(),
            colors.player_8.clone(),
        ])
    }
    fn get_config(&self) -> &ThemeConfig {
        &self.config
    }
}

impl<'a> From<SystemTheme<'a>> for UserTheme<'static> {
    fn from(system_theme: SystemTheme<'a>) -> UserTheme<'static> {
        // because 'a is 'static, we can simply transmute the lifetime
        // WARNING: transmuting a lifetime is generally not a good idea.
        // It is only safe here because we know 'a is 'static in SystemTheme
        let config: ThemeConfig<'static> = unsafe {
            std::mem::transmute(system_theme.config)
        };

        UserTheme { config }
    }
}

fn main() -> Result<(), anyhow::Error> {
    let system_theme = SystemTheme::new();
    println!("System Theme:");
    print_theme_colors(system_theme.colors(0)?)?;

    let user_theme = UserTheme::from_file("theme/test_theme.json")?;
    println!("\nUser Theme:");
    print_theme_colors(user_theme.colors(0)?)?;

    let system_theme_as_user_theme: UserTheme = system_theme.into();
    println!("\nUser Theme from System Theme:");
    print_theme_colors(system_theme_as_user_theme.colors(0)?)?;

    println!("\nWriting themes.");
    let system_theme2 = SystemTheme::new();
    system_theme2.to_file("theme/system_theme.json")?;
    user_theme.to_file("theme/user_theme.json")?;
    system_theme_as_user_theme.to_file("theme/user_theme_from_system_theme.json")?;

    Ok(())
}

fn print_theme_colors(colors: &ThemeColors) -> Result<(), anyhow::Error> {
    println!(
        "fg: {}, bg: {}, border: {}, player_1: {}",
        colors.fg,
        colors.bg,
        colors.border,
        colors.player_1,
    );
    Ok(())
}
