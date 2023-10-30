use super::color_scale::ColorScale;

/// A set of color scales used in a theme.
///
/// Each set contains four color scales: `light`, `dark`, `light_alpha`, and `dark_alpha`.
/// The `light` and `dark` scales are used for solid colors, while the `light_alpha` and `dark_alpha` scales are used for transparent colors.
pub struct ColorScaleSet {
    pub name: String,
    pub light: ColorScale,
    pub dark: ColorScale,
    pub light_alpha: ColorScale,
    pub dark_alpha: ColorScale,
}

impl ColorScaleSet {
    /// Creates a new `ColorScaleSet`.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the color scale set.
    /// * `light` - The light color scale.
    /// * `dark` - The dark color scale.
    /// * `light_alpha` - The light alpha color scale.
    /// * `dark_alpha` - The dark alpha color scale.
    pub fn new(name: String, light: ColorScale, dark: ColorScale, light_alpha: ColorScale, dark_alpha: ColorScale) -> Self {
        ColorScaleSet {
            name,
            light,
            dark,
            light_alpha,
            dark_alpha,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::scale::color_scale::hsla;

    use super::*;

    #[test]
    fn test_color_scale_set() {
        let light = [hsla(0.0, 0.0, 1.0, 1.0); 12];
        let dark = [hsla(0.0, 0.0, 0.0, 1.0); 12];
        let light_alpha = [hsla(0.0, 0.0, 1.0, 0.5); 12];
        let dark_alpha = [hsla(0.0, 0.0, 0.0, 0.5); 12];

        let set = ColorScaleSet::new("test".to_string(), light, dark, light_alpha, dark_alpha);

        assert_eq!(set.name, "test");
        assert_eq!(set.light[0], hsla(0.0, 0.0, 1.0, 1.0));
        assert_eq!(set.dark[0], hsla(0.0, 0.0, 0.0, 1.0));
        assert_eq!(set.light_alpha[0], hsla(0.0, 0.0, 1.0, 0.5));
        assert_eq!(set.dark_alpha[0], hsla(0.0, 0.0, 0.0, 0.5));
    }
}
