use std::collections::BTreeMap;

use crate::UIColor;

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
