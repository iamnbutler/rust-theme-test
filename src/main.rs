use std::{borrow::Cow, num::ParseIntError};
use gpui_color::Hsla;
use anyhow::Result;

mod scale;
mod gpui_color;

impl Hsla {
    fn from_hex(hex: &str) -> Result<Self, ParseIntError> {
        let rgba = gpui_color::Rgba::try_from(hex)?;
        Ok(Self::from(rgba))
    }

    fn to_hex(&self) -> String {
        let rgba: gpui_color::Rgba = (*self).into();
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
    value: Cow<'a, gpui_color::Hsla>,
    name: String,
    documentation: Option<String>,
}

impl<'a> ThemeColor<'a> {
    pub fn new(color: ColorType, hsla: gpui_color::Hsla, documentation: Option<String>) -> Self {
        Self {
            value: Cow::Owned(hsla),
            name: color.name().into(),
            documentation
        }
    }

    pub fn from_hex(color: ColorType, hex: String, documentation: Option<String>) -> Result<Self, ParseIntError> {
        let hsla = gpui_color::Hsla::from_hex(&hex)?;

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
        let hsla = gpui_color::Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.0 };

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
        let hsla = gpui_color::Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.0 };

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

#[derive(Debug)]
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
                ThemeColor::new(ColorType::Fg, gpui_color::red(), None),
                ThemeColor::new(ColorType::Bg, gpui_color::white(), None),
                ThemeColor::new(ColorType::Border, gpui_color::black(), None)
            ]
        );

        assert_eq!(theme.colors.len(), 3);
        assert_eq!(theme.colors[2].color_type(), ColorType::Border);
    }
}

// ---------------------
// ThemeFamily
// ---------------------

#[derive(Debug)]
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

#[derive(Debug)]
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

fn main() -> Result<(), anyhow::Error> {
    let mut theme_registry = ThemeRegistry::new();

        let mut theme_family1 = ThemeFamily::new("Test Family 1".to_string(), "Test Author".to_string());
    let theme1 = Theme::new("Test Theme 1".to_string(), Appearance::Dark, Vec::new());
    theme_family1.add_variant(theme1);
    theme_registry.add_family(theme_family1);

    let mut theme_family2 = ThemeFamily::new("Test Family 2".to_string(), "Test Author".to_string());
    let theme2 = Theme::new("Test Theme 2".to_string(), Appearance::Dark, Vec::new());
    theme_family2.add_variant(theme2);
    theme_registry.add_family(theme_family2);

    let mut theme_family3 = ThemeFamily::new("Test Family 3".to_string(), "Test Author".to_string());
    let theme3 = Theme::new("Test Theme 3".to_string(), Appearance::Dark, Vec::new());
    theme_family3.add_variant(theme3);
    theme_registry.add_family(theme_family3);

    let mut theme_family4 = ThemeFamily::new("Test Family 4".to_string(), "Test Author".to_string());
    let theme4 = Theme::new("Test Theme 4".to_string(), Appearance::Light, Vec::new());
    theme_family4.add_variant(theme4);
    theme_registry.add_family(theme_family4);

    println!("All themes: {:?}", theme_registry.all_themes());
    println!("Light themes: {:?}", theme_registry.all_light());
    println!("Dark themes: {:?}", theme_registry.all_dark());

    Ok(())
}
