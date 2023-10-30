use std::collections::BTreeMap;
use ui_color::UIColor;
use crate::{color_scale::{ColorScale, hsla, ColorScaleSet}, theme::{Theme, ThemeFamily}};

mod ui_color;
mod theme;
mod color_scale;



use anyhow::Result;


fn main() -> Result<(), anyhow::Error> {
    let mut ui_colors: BTreeMap<String, UIColor> = BTreeMap::new();

    let test_color_scale: ColorScale = [
        hsla(0.0, 0.0, 0.0, 1.0),
        hsla(0.0, 0.0, 0.0, 1.0),
        hsla(0.0, 0.0, 0.0, 1.0),
        hsla(0.0, 0.0, 0.0, 1.0),
        hsla(0.0, 0.0, 0.0, 1.0),
        hsla(0.0, 0.0, 0.0, 1.0),
        hsla(0.0, 0.0, 0.0, 1.0),
        hsla(0.0, 0.0, 0.0, 1.0),
        hsla(0.0, 0.0, 0.0, 1.0),
        hsla(0.0, 0.0, 0.0, 1.0),
        hsla(0.0, 0.0, 0.0, 1.0),
        hsla(0.0, 0.0, 0.0, 1.0)
    ];

    let test_color_scale_set = ColorScaleSet::new("neutral".into(), test_color_scale.clone(), test_color_scale.clone(), test_color_scale.clone(), test_color_scale.clone());

    // Initialize UIColors
    ui_colors.insert("filled-element-background".to_string(), UIColor::new("filled-element-background", ColorScale::default(), "Used for the background of filled elements, like buttons and checkboxes."));

    let mut theme = Theme::new();
    theme.add_ui_colors(ui_colors);

    // Create a new theme familt
    let theme_family = ThemeFamily::new(
        "Zed".into(),
        "Zed Industries".into()
    ).add_color_scale_set(test_color_scale_set).add_theme(theme.clone());

    // Access a UIColor
    let ui_color = theme.get_ui_color("filled-element-background").unwrap();
    println!("UIColor: {:?}", ui_color);

    // Access a ColorScaleSet
    let color_scale_set = theme_family.get_color_scale_set("neutral").unwrap();
    println!("ColorScaleSet: {:?}", color_scale_set);

    Ok(())
}
