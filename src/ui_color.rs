use crate::ColorScale;
use std::collections::BTreeMap;

/// UIColor represents a UI color with a name, a value, and a description.
/// The value is an index of ColorScale.
#[derive(Debug, Clone, PartialEq)]
pub struct UIColor {
    pub name: String,
    pub value: ColorScale,
    pub description: String,
}

impl UIColor {
    /// Creates a new UIColor.
    pub fn new(name: &str, value: ColorScale, description: &str) -> Self {
        Self {
            name: name.to_string(),
            value,
            description: description.to_string(),
        }
    }
}

/// This module is responsible for managing a collection of UIColors.
/// It provides an easy way to add and access UIColors.
/// UIColors should never be edited directly, only a theme should be edited.
pub struct UIColors(BTreeMap<String, UIColor>);

impl UIColors {
    /// Creates a new UIColors collection.
    pub fn new() -> Self {
        UIColors(BTreeMap::new())
    }

    /// Adds a new UIColor to the collection.
    pub fn add(&mut self, color: UIColor) {
        self.0.insert(color.name.clone(), color);
    }

    /// Returns a reference to a UIColor in the collection.
    pub fn get(&self, name: &str) -> Option<&UIColor> {
        self.0.get(name)
    }
}

#[cfg(test)]
mod tests {
    use crate::hsla;

    use super::*;

    #[test]
    fn test_ui_color_new() {
        let color_scale: ColorScale = [
            hsla(0.0, 0.0, 0.0, 1.0),
            hsla(0.0, 0.0, 0.05, 1.0),
            hsla(0.0, 0.0, 0.1, 1.0),
            hsla(0.0, 0.0, 0.15, 1.0),
            hsla(0.0, 0.0, 0.2, 1.0),
            hsla(0.0, 0.0, 0.25, 1.0),
            hsla(0.0, 0.0, 0.3, 1.0),
            hsla(0.0, 0.0, 0.35, 1.0),
            hsla(0.0, 0.0, 0.4, 1.0),
            hsla(0.0, 0.0, 0.45, 1.0),
            hsla(0.0, 0.0, 0.5, 1.0),
            hsla(0.0, 0.0, 0.55, 1.0),
        ];

        let ui_color = UIColor::new("filled-element-background", color_scale, "Used for the background of filled elements, like buttons and checkboxes.");
        assert_eq!(ui_color.name, "filled-element-background");
        assert_eq!(ui_color.description, "Used for the background of filled elements, like buttons and checkboxes.");
    }

    #[test]
    fn test_ui_colors() {
        let mut colors = UIColors::new();
        let color = UIColor {
            name: "filled-element-background".to_string(),
            value: [hsla(0.0, 0.0, 0.0, 1.0); 12],
            description: "Used for the background of filled elements, like buttons and checkboxes.".to_string(),
        };
        colors.add(color.clone());
        assert_eq!(colors.get("filled-element-background"), Some(&color));
    }
}
