use std::borrow::Cow;
use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};

mod color;
mod theme;

pub trait Theme {
    fn colors(&self, v: usize) -> Result<&ThemeColors>;
    fn player_colors(&self, v: usize) -> Result<[ThemeColor; 8]>;
    fn set_color(&mut self, v: usize, color: String, color_type: ColorType) -> Result<()>;
    fn to_file(&self, path: &str) -> Result<(), anyhow::Error> {
        let file = std::fs::File::create(path)?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self.get_config()).map_err(From::from)
    }
    fn get_config(&self) -> &ThemeConfig;
}

pub enum ColorType {
    Fg,
    Bg,
    Border,
    Player1,
    Player2,
    Player3,
    Player4,
    Player5,
    Player6,
    Player7,
    Player8,
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

// struct Theme {
//     color: String,
//     background: String
// }

// struct Override {
//     color: Option<String>,
//     background: Option<String>,
// }

// fn apply(base: Theme, override: Override) -> Theme {

// }

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

    fn set_color(&mut self, v: usize, color: String, color_type: ColorType) -> Result<()> {
        let variant = self.config.variants.get_mut(v)
            .ok_or_else(|| anyhow::anyhow!("Variant index out of range"))?;
        let color = Cow::Owned(color);

        match color_type {
            ColorType::Fg => variant.colors.fg = color,
            ColorType::Bg => variant.colors.bg = color,
            ColorType::Border => variant.colors.border = color,
            ColorType::Player1 => variant.colors.player_1 = color,
            ColorType::Player2 => variant.colors.player_2 = color,
            ColorType::Player3 => variant.colors.player_3 = color,
            ColorType::Player4 => variant.colors.player_4 = color,
            ColorType::Player5 => variant.colors.player_5 = color,
            ColorType::Player6 => variant.colors.player_6 = color,
            ColorType::Player7 => variant.colors.player_7 = color,
            ColorType::Player8 => variant.colors.player_8 = color,
        }

        Ok(())
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

    fn set_color(&mut self, v: usize, color: String, color_type: ColorType) -> Result<()> {
        let variant = self.config.variants.get_mut(v)
            .ok_or_else(|| anyhow::anyhow!("Variant index out of range"))?;
        let color = Cow::Owned(color);

        match color_type {
            ColorType::Fg => variant.colors.fg = color,
            ColorType::Bg => variant.colors.bg = color,
            ColorType::Border => variant.colors.border = color,
            ColorType::Player1 => variant.colors.player_1 = color,
            ColorType::Player2 => variant.colors.player_2 = color,
            ColorType::Player3 => variant.colors.player_3 = color,
            ColorType::Player4 => variant.colors.player_4 = color,
            ColorType::Player5 => variant.colors.player_5 = color,
            ColorType::Player6 => variant.colors.player_6 = color,
            ColorType::Player7 => variant.colors.player_7 = color,
            ColorType::Player8 => variant.colors.player_8 = color,
        }

        Ok(())
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

    let mut new_user_theme: UserTheme = SystemTheme::new().into();
    new_user_theme.set_color(0, "#FE3F5D".to_string(), ColorType::Player1)?;
    println!("\nUser Theme from System Theme:");
    print_theme_colors(new_user_theme.colors(0)?)?;

    println!("\nWriting themes.");
    system_theme.to_file("theme/system_theme.json")?;
    user_theme.to_file("theme/user_theme.json")?;
    new_user_theme.to_file("theme/user_theme_from_system_theme.json")?;

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
