use std::{borrow::Cow, num::ParseIntError};
use color::Hsla;
use anyhow::Result;

mod color;

impl Hsla {
    fn from_hex(hex: &str) -> Result<Self, ParseIntError> {
        let rgba = color::Rgba::try_from(hex)?;
        Ok(Self::from(rgba))
    }

    fn to_hex(&self) -> String {
        let rgba: color::Rgba = (*self).into();
        format!("#{:02X}{:02X}{:02X}{:02X}",
            (rgba.r*255.0) as u8,
            (rgba.g*255.0) as u8,
            (rgba.b*255.0) as u8,
            (rgba.a*255.0) as u8)
    }
}

// ---------------------
// Color Type
// ---------------------

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ColorType {
    Fg,
    Bg,
    Border,
}

impl ColorType {
    pub fn name(&self) -> String {
        match self {
            ColorType::Fg => "fg".into(),
            ColorType::Bg => "bg".into(),
            ColorType::Border => "border".into(),
        }
    }
}

impl From<String> for ColorType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "fg" => ColorType::Fg,
            "bg" => ColorType::Bg,
            "border" => ColorType::Border,
            _ => panic!("Unknown color type: {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_type_conversion() {
        let test_cases = [
            ("fg", ColorType::Fg),
            ("bg", ColorType::Bg),
            ("border", ColorType::Border),
        ];

        for (input, expected) in &test_cases {
            let from_str = ColorType::from(input.to_string());
            assert_eq!(from_str, *expected, "ColorType::from failed for {}", input);
            assert_eq!(from_str.name(), *input, "ColorType::name failed for {}", input);
        }
    }

    #[test]
    #[should_panic(expected = "Unknown color type: unknown")]
    fn test_unknown_color_type() {
        let _ = ColorType::from("unknown".to_string());
    }
}

// ---------------------
// ThemeColor
// ---------------------

#[derive(Debug, Clone)]
pub struct ThemeColor<'a> {
    value: Cow<'a, color::Hsla>,
    name: String,
    documentation: Option<String>,
}

impl<'a> ThemeColor<'a> {
    pub fn new(color: ColorType, hsla: color::Hsla, documentation: Option<String>) -> Self {
        Self {
            value: Cow::Owned(hsla),
            name: color.name().into(),
            documentation
        }
    }

    pub fn from_hex(color: ColorType, hex: String, documentation: Option<String>) -> Result<Self, ParseIntError> {
        let hsla = color::Hsla::from_hex(&hex)?;

        Ok(Self {
            value: Cow::Owned(hsla),
            name: color.name().into(),
            documentation
        })
    }

    pub fn color_type(&self) -> ColorType {
        ColorType::from(self.name.clone())
    }
}

#[cfg(test)]
mod theme_color_tests {
    use super::*;

    #[test]
    fn test_theme_color_from_hsla() {
        let hsla = color::Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.0 };

        let color = ThemeColor::new(ColorType::Fg, hsla, Some("Test Color".to_string()));
        assert_eq!(color.color_type(), ColorType::Fg, "ThemeColor::from_hsla created color with incorrect type");
        assert_eq!(color.name, "fg", "ThemeColor::from_hsla created color with incorrect name");
        assert_eq!(color.documentation.unwrap(), "Test Color", "ThemeColor::from_hsla created color with incorrect documentation");
    }

    #[test]
    fn test_theme_color_from_hex() {
        let color = ThemeColor::from_hex(ColorType::Fg, "#FEFEFE".to_string(), None);
        assert!(color.is_ok(), "ThemeColor::from_hex failed for valid 6 value hex input");
    }

    #[test]
    fn test_theme_color_from_hex8() {
        let color = ThemeColor::from_hex(ColorType::Fg, "#FAFAFAEE".to_string(), None);
        assert!(color.is_ok(), "ThemeColor::from_hex failed for valid 8 value hex input");
    }

    #[test]
    fn test_theme_color_from_hex_fail() {
        let color = ThemeColor::from_hex(ColorType::Fg, "invalid".to_string(), None);
        assert!(color.is_err(), "ThemeColor::from_hex did not fail for invalid input");
    }

    #[test]
    fn test_theme_color_color_type() {
        let hsla = color::Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.0 };

        let color = ThemeColor::new(ColorType::Fg, hsla, None);
        assert_eq!(color.color_type(), ColorType::Fg, "ThemeColor::color_type returned incorrect type");
    }
}

// ---------------------
// Theme
// ---------------------

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Appearance {
    Light,
    Dark,
}

impl Appearance {
    pub fn name(&self) -> String {
        match self {
            Appearance::Light => "Light".into(),
            Appearance::Dark => "Dark".into(),
        }
    }
}

pub struct Theme<'a> {
    name: String,
    appearance: Appearance,
    colors: Vec<ThemeColor<'a>>,
}

impl<'a> Theme<'a> {
    pub fn new(name: String, appearance: Appearance, colors: Vec<ThemeColor<'a>>) -> Self {
        Self {
            name,
            appearance,
            colors,
        }
    }
}

#[cfg(test)]
mod theme_tests {
    use super::*;

    #[test]
    fn test_theme_creation() {
        let theme = Theme::new(
            "Test Theme".to_string(),
            Appearance::Dark,
            Vec::new()
        );
        assert_eq!(theme.name, "Test Theme");
        assert_eq!(theme.appearance, Appearance::Dark);
        assert!(theme.colors.is_empty());
    }

    #[test]
    fn test_adding_theme_colors() {
        let theme = Theme::new(
            "Test Theme".to_string(),
            Appearance::Dark,
            vec![
                ThemeColor::new(ColorType::Fg, color::red(), None),
                ThemeColor::new(ColorType::Bg, color::white(), None),
                ThemeColor::new(ColorType::Border, color::black(), None)
            ]
        );

        assert_eq!(theme.colors.len(), 3);
        assert_eq!(theme.colors[2].color_type(), ColorType::Border);
    }
}

// ---------------------
// ThemeFamily
// ---------------------

pub struct ThemeFamily<'a> {
    name: String,
    author: String,
    variants: Vec<Theme<'a>>,
}

impl<'a> ThemeFamily<'a> {
    pub fn new(name: String, author: String,) -> Self {
        Self {
            name,
            author,
            variants: Vec::new(),
        }
    }

    pub fn add_variant(&mut self, variant: Theme<'a>) {
        self.variants.push(variant);
    }
}

#[cfg(test)]
mod theme_family_tests {
    use super::*;

    #[test]
    fn test_theme_family_creation() {
        let theme_family = ThemeFamily::new("Test Family".to_string(), "Test Author".to_string());
        assert_eq!(theme_family.name, "Test Family");
        assert_eq!(theme_family.author, "Test Author");
        assert!(theme_family.variants.is_empty());
    }

    #[test]
    fn test_theme_family_variant_addition() {
        let mut theme_family = ThemeFamily::new("Test Family".to_string(), "Test Author".to_string());
        let theme = Theme::new("Test Theme".to_string(), Appearance::Dark, Vec::new());
        theme_family.add_variant(theme);
        assert_eq!(theme_family.variants.len(), 1);
    }
}

// ---------------------
// ThemeRegistry
// ---------------------

pub struct ThemeRegistry<'a> {
    families: Vec<ThemeFamily<'a>>,
}

impl<'a> ThemeRegistry<'a> {
    pub fn new() -> Self {
        Self {
            families: Vec::new(),
        }
    }

    pub fn add_family(&mut self, family: ThemeFamily<'a>) {
        self.families.push(family);
    }

    pub fn all_themes(&self) -> Vec<&Theme> {
        let mut themes = Vec::new();

        for family in &self.families {
            for variant in &family.variants {
                themes.push(variant);
            }
        }

        themes.sort_by(|a, b| a.name.cmp(&b.name));

        themes
    }

    pub fn all_dark(&self) -> Vec<&Theme> {
        let mut themes = Vec::new();

        for family in &self.families {
            for variant in &family.variants {
                if variant.appearance == Appearance::Dark {
                    themes.push(variant);
                }
            }
        }

        themes.sort_by(|a, b| a.name.cmp(&b.name));

        themes
    }

    pub fn all_light(&self) -> Vec<&Theme> {
        let mut themes = Vec::new();

        for family in &self.families {
            for variant in &family.variants {
                if variant.appearance == Appearance::Light {
                    themes.push(variant);
                }
            }
        }

        themes.sort_by(|a, b| a.name.cmp(&b.name));

        themes
    }
}

#[cfg(test)]
mod theme_registry_tests {
    use super::*;

    #[test]
    fn test_theme_registry_creation() {
        let theme_registry = ThemeRegistry::new();
        assert!(theme_registry.families.is_empty());
    }

    #[test]
    fn test_theme_registry_family_addition() {
        let mut theme_registry = ThemeRegistry::new();
        let theme_family = ThemeFamily::new("Test Family".to_string(), "Test Author".to_string());
        theme_registry.add_family(theme_family);
        assert_eq!(theme_registry.families.len(), 1);
    }

    #[test]
    fn test_theme_registry_all_themes() {
        let mut theme_registry = ThemeRegistry::new();
        let mut theme_family = ThemeFamily::new("Test Family".to_string(), "Test Author".to_string());
        let theme = Theme::new("Test Theme".to_string(), Appearance::Dark, Vec::new());
        theme_family.add_variant(theme);
        theme_registry.add_family(theme_family);
        assert_eq!(theme_registry.all_themes().len(), 1);
    }

    #[test]
    fn test_theme_registry_all_dark() {
        let mut theme_registry = ThemeRegistry::new();
        let mut theme_family = ThemeFamily::new("Test Family".to_string(), "Test Author".to_string());
        let theme = Theme::new("Test Theme".to_string(), Appearance::Dark, Vec::new());
        theme_family.add_variant(theme);
        theme_registry.add_family(theme_family);
        assert_eq!(theme_registry.all_dark().len(), 1);
    }

    #[test]
    fn test_theme_registry_all_light() {
        let mut theme_registry = ThemeRegistry::new();
        let mut theme_family = ThemeFamily::new("Test Family".to_string(), "Test Author".to_string());
        let theme = Theme::new("Test Theme".to_string(), Appearance::Light, Vec::new());
        theme_family.add_variant(theme);
        theme_registry.add_family(theme_family);
        assert_eq!(theme_registry.all_light().len(), 1);
    }
}

// macro_rules! generate_colors {
//     ($($color:ident),*) => {
//         pub struct ThemeColors<'a> {
//             $(pub $color: ThemeColor<'a>),*
//         }

//         impl<'a> serde::Deserialize<'a> for ThemeColors<'a> {
//             fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//             where
//                 D: serde::Deserializer<'a>,
//             {
//                 let mut map = std::collections::HashMap::new();
//                 map = serde::Deserialize::deserialize(deserializer)?;

//                 Ok(ThemeColors {
//                     $($color: Cow::Owned(color::Hsla::from_hex(&map.remove(&stringify!($color)).unwrap().as_str()).unwrap())),*
//                 })
//             }
//         }

//         impl<'a> serde::Serialize for ThemeColors<'a> {
//             fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//             where
//                 S: serde::Serializer,
//             {
//                 let mut map = std::collections::HashMap::new();
//                 $(map.insert(stringify!($color), self.$color.to_hex());)*;
//                 map.serialize(serializer)
//             }
//         }
//     }
// }

// generate_colors!(fg, bg, border, player_1, player_2, player_3, player_4, player_5, player_6, player_7, player_8);

// #[derive(Serialize, Deserialize)]
// pub struct ThemeVariant<'a> {
//     pub title: Cow<'a, str>,
//     pub appearance: Cow<'a, str>,
//     pub colors: ThemeColors<'a>,
// }

// #[derive(Serialize, Deserialize)]
// pub struct ThemeConfig<'a> {
//     pub name: Cow<'a, str>,
//     pub author: Option<Cow<'a, str>>,
//     pub url: Option<Cow<'a, str>>,
//     pub variants: Vec<ThemeVariant<'a>>,
// }

// pub struct SystemTheme<'a> {
//     config: ThemeConfig<'a>,
// }

// macro_rules! system_colors {
//     ($($color:ident => $value:expr),*) => {
//         {
//             ThemeColors {
//                 $($color: Cow::Borrowed($value)),*
//             }
//         }
//     };
// }

// impl<'a> SystemTheme<'a> {
//     fn new() -> Self {
//         let config = ThemeConfig {
//             name: Cow::Borrowed("Zed"),
//             author: Some(Cow::Borrowed("Zed Industries")),
//             url: None,
//             variants: vec![ThemeVariant {
//                 title: Cow::Borrowed("Zed Light"),
//                 appearance: Cow::Borrowed("light"),
//                 colors: system_colors! {
//                     fg => "#ffffff",
//                     bg => "#000000",
//                     border => "#333333",
//                     player_1 => "#ff0000",
//                     player_2 => "#00ff00",
//                     player_3 => "#0000ff",
//                     player_4 => "#ffff00",
//                     player_5 => "#ff00ff",
//                     player_6 => "#00ffff",
//                     player_7 => "#ffffff",
//                     player_8 => "#000000"
//                 },
//             }],
//         };

//         SystemTheme { config }
//     }
// }

// impl<'a> Theme for SystemTheme<'a> {
//     fn colors(&self, v: usize) -> Result<&ThemeColors> {
//         self.config.variants.get(v).map(|variant| &variant.colors).context("Index out of range")
//     }

//     fn set_color(&mut self, v: usize, color: color::Hsla, color_type: ColorType) -> Result<()> {
//         let variant = self.config.variants.get_mut(v)
//             .ok_or_else(|| anyhow::anyhow!("Variant index out of range"))?;

//         let color = Cow::Owned(color);

//         match color_type {
//             ColorType::Fg => variant.colors.fg = color,
//             ColorType::Bg => variant.colors.bg = color,
//             ColorType::Border => variant.colors.border = color,
//             ColorType::Player1 => variant.colors.player_1 = color,
//             ColorType::Player2 => variant.colors.player_2 = color,
//             ColorType::Player3 => variant.colors.player_3 = color,
//             ColorType::Player4 => variant.colors.player_4 = color,
//             ColorType::Player5 => variant.colors.player_5 = color,
//             ColorType::Player6 => variant.colors.player_6 = color,
//             ColorType::Player7 => variant.colors.player_7 = color,
//             ColorType::Player8 => variant.colors.player_8 = color,
//         }

//         Ok(())
//     }

//     fn player_colors(&self, v: usize) -> Result<[ThemeColor; 8]> {
//         let colors = self.colors(v)?;
//         Ok([
//             colors.player_1.clone(),
//             colors.player_2.clone(),
//             colors.player_3.clone(),
//             colors.player_4.clone(),
//             colors.player_5.clone(),
//             colors.player_6.clone(),
//             colors.player_7.clone(),
//             colors.player_8.clone(),
//         ])
//     }
//     fn get_config(&self) -> &ThemeConfig {
//         &self.config
//     }
// }

// pub struct UserTheme<'a> {
//     config: ThemeConfig<'a>,
// }

// impl<'a> UserTheme<'a> {
//     fn from_file(path: &str) -> Result<Self> {
//         let file = std::fs::File::open(path).context("Unable to open theme file")?;
//         let reader = std::io::BufReader::new(file);
//         let config: ThemeConfig = serde_json::from_reader(reader)
//             .context("Failed to deserialize theme")?;
//         Ok(UserTheme{ config })
//     }
// }

// impl<'a> Theme for UserTheme<'a> {
//     fn colors(&self, v: usize) -> Result<&ThemeColors> {
//         self.config.variants.get(v).map(|variant| &variant.colors).context("Index out of range")
//     }

//     fn set_color(&mut self, v: usize, color: color::Hsla, color_type: ColorType) -> Result<()> {
//         let variant = self.config.variants.get_mut(v)
//             .ok_or_else(|| anyhow::anyhow!("Variant index out of range"))?;

//         let color = Cow::Owned(color);

//         match color_type {
//             ColorType::Fg => variant.colors.fg = color,
//             ColorType::Bg => variant.colors.bg = color,
//             ColorType::Border => variant.colors.border = color,
//             ColorType::Player1 => variant.colors.player_1 = color,
//             ColorType::Player2 => variant.colors.player_2 = color,
//             ColorType::Player3 => variant.colors.player_3 = color,
//             ColorType::Player4 => variant.colors.player_4 = color,
//             ColorType::Player5 => variant.colors.player_5 = color,
//             ColorType::Player6 => variant.colors.player_6 = color,
//             ColorType::Player7 => variant.colors.player_7 = color,
//             ColorType::Player8 => variant.colors.player_8 = color,
//         }

//         Ok(())
//     }

//     fn player_colors(&self, v: usize) -> Result<[ThemeColor; 8]> {
//         let colors = self.colors(v)?;
//         Ok([
//             colors.player_1.clone(),
//             colors.player_2.clone(),
//             colors.player_3.clone(),
//             colors.player_4.clone(),
//             colors.player_5.clone(),
//             colors.player_6.clone(),
//             colors.player_7.clone(),
//             colors.player_8.clone(),
//         ])
//     }
//     fn get_config(&self) -> &ThemeConfig {
//         &self.config
//     }
// }

// impl<'a> From<SystemTheme<'a>> for UserTheme<'static> {
//     fn from(system_theme: SystemTheme<'a>) -> UserTheme<'static> {
//         // because 'a is 'static, we can simply transmute the lifetime
//         // WARNING: transmuting a lifetime is generally not a good idea.
//         // It is only safe here because we know 'a is 'static in SystemTheme
//         let config: ThemeConfig<'static> = unsafe {
//             std::mem::transmute(system_theme.config)
//         };

//         UserTheme { config }
//     }
// }

// pub trait Theme {
//     fn colors(&self, v: usize) -> Result<&ThemeColors>;
//     fn player_colors(&self, v: usize) -> Result<[ThemeColor; 8]>;
//     fn set_color(&mut self, v: usize, color: Hsla, color_type: ColorType) -> Result<()>;
//     fn to_file(&self, path: &str) -> Result<(), anyhow::Error> {
//         let file = std::fs::File::create(path)?;
//         let writer = std::io::BufWriter::new(file);
//         serde_json::to_writer_pretty(writer, &self.get_config()).map_err(From::from)
//     }
//     fn get_config(&self) -> &ThemeConfig;
// }

fn main() -> Result<(), anyhow::Error> {
    // let system_theme = SystemTheme::new();
    // println!("System Theme:");
    // print_theme_colors(system_theme.colors(0)?)?;

    // let user_theme = UserTheme::from_file("theme/test_theme.json")?;
    // println!("\nUser Theme:");
    // print_theme_colors(user_theme.colors(0)?)?;

    // let mut new_user_theme: UserTheme = SystemTheme::new().into();
    // new_user_theme.set_color(0, "#FE3F5D".to_string(), ColorType::Player1)?;
    // println!("\nUser Theme from System Theme:");
    // print_theme_colors(new_user_theme.colors(0)?)?;

    // println!("\nWriting themes.");
    // system_theme.to_file("theme/system_theme.json")?;
    // user_theme.to_file("theme/user_theme.json")?;
    // new_user_theme.to_file("theme/user_theme_from_system_theme.json")?;

    Ok(())
}

// fn print_theme_colors(colors: &ThemeColors) -> Result<(), anyhow::Error> {
//     println!(
//         "fg: {}, bg: {}, border: {}, player_1: {}",
//         colors.fg,
//         colors.bg,
//         colors.border,
//         colors.player_1,
//     );
//     Ok(())
// }
