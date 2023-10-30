use std::{collections::{HashMap, BTreeMap}, sync::Arc};
use crate::color::{Hsla, hsla};

use paste::paste;
use serde::Deserialize;

pub fn hsla_to_zed_hsla(input_hsla:String) -> Hsla {
    let h = input_hsla.split(",").collect::<Vec<&str>>()[0].parse::<f32>().unwrap() / 360.0;
    let s = input_hsla.split(",").collect::<Vec<&str>>()[1].parse::<f32>().unwrap() / 100.0;
    let l = input_hsla.split(",").collect::<Vec<&str>>()[2].parse::<f32>().unwrap() / 100.0;
    let a = input_hsla.split(",").collect::<Vec<&str>>()[3].parse::<f32>().unwrap() / 100.0;

    hsla(h, s, l, a)
}

// ====================
// Color Scales
// ====================

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum UIColorName {
    Background,
    Border,
    Text,
}

impl UIColorName {
    pub fn label(&self) -> &str {
        match self {
            UIColorName::Background => "background",
            UIColorName::Border => "border",
            UIColorName::Text => "text",
        }
    }
}

struct Colors {
    background: String,
    border: String,
    text: String,
}

enum UiColorName {
    Background,
    Border,
    Text
}

struct ColorOverrides(BTreeMap<UiColorName, String>);
// create_ui_color_overrides

macro_rules! create_ui_color_overrides_impl {
    ($($field:ident: $t:ty),+) => {

        paste! { #[derive(Clone, Debug)]
            enum UiColorNamea {
            $([<$field:camel>]),+
        }
        }
        struct Colorsa {
            $($field: $t),+
        }
        #[derive(Clone, Debug)]
        struct ColorOverridesa(BTreeMap<UiColorNamea, String>);
    };
}

macro_rules! create_ui_color_overrides {
    ($($field:ident),+) => {
        create_ui_color_overrides_impl! {($field: String)+}
    }
}
create_ui_color_overrides_impl! {
    color: String,
    background: String
}

/// UIColors should never be edited directly, only a theme should be edited.
#[derive(Debug, Clone)]
pub struct UIColors(BTreeMap<UIColorName, UIColor>);


// ====================
// Theme
// ====================

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Appearance {
    Light,
    Dark
}

type ThemeId = usize;

#[derive(Debug, Clone)]
pub struct ThemeVariant {
    id: ThemeId,
    name: String,
    author: String,
    appearance: Appearance,
    overrides: ColorOverridesa,
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
