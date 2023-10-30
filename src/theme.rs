use std::{collections::BTreeMap, sync::RwLock, fs, path::{Path, PathBuf}};

use crate::{UIColor, ColorScaleSet};

/// The Theme struct represents a theme in the system.
/// It contains a collection of UIColors.
#[derive(Debug, Clone)]
pub struct Theme {
    pub ui_colors: BTreeMap<String, UIColor>,
}

impl Theme {
    /// Creates a new Theme.
    pub fn new() -> Self {
        Theme {
            ui_colors: BTreeMap::new(),
        }
    }

    /// Adds a UIColor to the theme.
    pub fn add_ui_color(&mut self, ui_color: UIColor) {
        self.ui_colors.insert(ui_color.name.clone(), ui_color);
    }

    pub fn add_ui_colors(&mut self, ui_colors: BTreeMap<String, UIColor>) {
        for (name, ui_color) in ui_colors {
            self.add_ui_color(ui_color);
        }
    }

    /// Gets a UIColor from the theme.
    pub fn get_ui_color(&self, name: &str) -> Option<&UIColor> {
        self.ui_colors.get(name)
    }
}

#[derive(Debug, Clone)]
pub struct ThemeFamily {
    pub name: String,
    pub author: String,
    pub color_scale_sets: Option<BTreeMap<String, ColorScaleSet>>,
    pub themes: Vec<Theme>,
}

impl ThemeFamily {
    pub fn new(name: String, author: String) -> Self {
        Self {
            name,
            author,
            color_scale_sets: None,
            themes: Vec::new(),
        }
    }

    pub fn with_color_scale_sets(mut self, color_scale_sets: BTreeMap<String, ColorScaleSet>) -> Self {
        self.color_scale_sets = Some(color_scale_sets);
        self
    }

    pub fn add_theme(mut self, theme: Theme) -> Self {
        self.themes.push(theme);
        self
    }

    pub fn add_color_scale_set(mut self, color_scale_set: ColorScaleSet) -> Self {
        if let Some(color_scale_sets) = &mut self.color_scale_sets {
            color_scale_sets.insert(color_scale_set.name.clone(), color_scale_set);
        } else {
            let mut color_scale_sets = BTreeMap::new();
            color_scale_sets.insert(color_scale_set.name.clone(), color_scale_set);
            self.color_scale_sets = Some(color_scale_sets);
        }
        self
    }

    pub fn get_color_scale_set(&self, name: &str) -> Option<&ColorScaleSet> {
        if let Some(color_scale_sets) = &self.color_scale_sets {
            color_scale_sets.get(name)
        } else {
            None
        }
    }
}

pub struct ThemeRegistry {
    themes: RwLock<BTreeMap<String, ThemeFamily>>,
}

impl ThemeRegistry {
    pub fn new() -> Self {
        ThemeRegistry {
            themes: RwLock::new(BTreeMap::new()),
        }
    }

    pub fn list_themes(&self) -> Vec<String> {
        let themes = self.themes.read().unwrap();
        themes.keys().cloned().collect()
    }
}


#[cfg(test)]
mod tests {
    use crate::hsla;

    use super::*;

    #[test]
    fn test_theme() {
        let mut theme = Theme::new();

        let ui_color = UIColor {
            name: "filled-element-background".to_string(),
            value: [hsla(0.0, 0.0, 0.0, 1.0); 12],
            description: "Used for the background of filled elements, like buttons and checkboxes.".to_string(),
        };
        theme.add_ui_color(ui_color);

        assert_eq!(theme.ui_colors.len(), 1);
    }
}
