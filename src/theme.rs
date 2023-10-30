use std::{collections::{HashMap, BTreeMap}, sync::Arc, str::FromStr, path::{Path, PathBuf}};
use crate::color::{Hsla, hsla};

use paste::paste;

use serde::{Deserialize, Serialize};
// ====================
// Color Scales
// ====================

pub fn hsla_to_zed_hsla(input_hsla:&str) -> Hsla {
    let h = input_hsla.split(",").collect::<Vec<&str>>()[0].parse::<f32>().unwrap() / 360.0;
    let s = input_hsla.split(",").collect::<Vec<&str>>()[1].parse::<f32>().unwrap() / 100.0;
    let l = input_hsla.split(",").collect::<Vec<&str>>()[2].parse::<f32>().unwrap() / 100.0;
    let a = input_hsla.split(",").collect::<Vec<&str>>()[3].parse::<f32>().unwrap() / 100.0;

    hsla(h, s, l, a)
}

pub type ColorScale = [Hsla; 12];

/// A set of color scales used in a theme.
///
/// Each set contains four color scales: `light`, `dark`, `light_alpha`, and `dark_alpha`.
/// The `light` and `dark` scales are used for solid colors, while the `light_alpha` and `dark_alpha` scales are used for transparent colors.
#[derive(Debug, Clone)]
pub struct ColorScaleSet {
    pub name: String,
    pub light: ColorScale,
    pub dark: ColorScale,
    pub light_alpha: ColorScale,
    pub dark_alpha: ColorScale,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ColorScaleName {
    Slate,
    Red
}

pub type ColorScales = HashMap<ColorScaleName, ColorScaleSet>;

// ====================
// UI Color
// ====================

/// UIColor represents a UI color with a name, a value, and a description.
/// The value is an index of ColorScale.
#[derive(Debug, Clone)]
pub struct UIColor {
    pub name: String,
    pub value: Hsla,
    pub description: Option<String>,
}

impl UIColor {
    /// Creates a new UIColor.
    pub fn new(name: &str, value: Hsla, description: Option<String>) -> Self {
        Self {
            name: name.to_string(),
            value,
            description
        }
    }
}

struct Colors {
    background: String,
    border: String,
    text: String,
}

// invocation (list of colors) (defines what colors there are)
// import from toml -> kv (color name, value (string))
// map color name to UI color
// run hsla_to_zed_hsla on each color value
// overrided that UI color with the new value

/// StandardHsla is a Hsla color using standard units:
/// - h: 0-360
/// - s: 0-100
/// - l: 0-100
/// - a: 0-100
#[derive(PartialEq, Clone, Debug, Serialize)]
struct StandardHsla([u16; 4]);

impl<'a> serde::de::Deserialize<'a> for StandardHsla {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'a>
    {
        let elements = <[u16; 4]>::deserialize(deserializer)?;
        let [h, s, l, a] = elements;
        if h > 360 {
            return Err(serde::de::Error::custom("H is out of bounds"));
        }
        if s > 100 {
            return Err(serde::de::Error::custom("S is out of bounds"));
        }
        if l > 100 {
            return Err(serde::de::Error::custom("L is out of bounds"));
        }
        if a > 100 {
            return Err(serde::de::Error::custom("A is out of bounds"));
        }

        Ok(StandardHsla([h,s,l,a]))
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
enum ZedHsla {
    StandardHsla(StandardHsla),
    Hsla(Hsla)
}

//
macro_rules! create_ui_color_overrides_impl {
    ($($field:ident: $t:ty),*) => {

        paste! { #[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Deserialize, Serialize)]
            #[serde(rename_all="snake_case")]
            enum UiColorName {
            $([<$field:camel>]),*
        }
        }
        struct SystemColors {
            $($field: $t),*
        }
        #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
        struct ColorOverrides(BTreeMap<UiColorName, ZedHsla>);
    };
}

macro_rules! create_ui_color_overrides {
    ($($field:ident),+) => {
        create_ui_color_overrides_impl! {$($field: Hsla),*}
    }
}
create_ui_color_overrides!(background, border, text);

// ====================
// Theme
// ====================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum Appearance {
    Light,
    Dark
}

type ThemeId = usize;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ThemeVariant {
    #[serde(skip)]
    id: ThemeId,
    name: String,
    author: String,
    appearance: Appearance,
    overrides: ColorOverrides,
}

pub fn serialize_theme(t: ThemeVariant) -> Result<String, anyhow::Error> {
    Ok(toml::to_string_pretty(&t)?)
}
fn write_theme_to(t: ThemeVariant, path: &Path) -> Result<(), anyhow::Error> {
    let contents = serialize_theme(t)?;
    _ = std::fs::write(path, contents);
    Ok(())
}

pub fn write_theme(t: ThemeVariant) -> Result<(), anyhow::Error> {
    write_theme_to(t, &PathBuf::from("theme"))
}

#[derive(Debug)]
pub struct ThemeRegistry {
    themes: HashMap<ThemeId, Arc<ThemeVariant>>,
    current: Option<Arc<ThemeVariant>>
}

// ====================
// Tests
// ====================

#[cfg(test)]
mod tests {}
