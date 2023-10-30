
/// Module for handling color scales
///
/// This module provides the `ColorScale` and `Hsla` types, which are used to represent
/// color scales and individual colors in the HSLA color space, respectively.

#[derive(Default, Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Hsla {
    pub h: f32,
    pub s: f32,
    pub l: f32,
    pub a: f32,
}

pub fn hsla(h: f32, s: f32, l: f32, a: f32) -> Hsla {
    Hsla {
        h: h.clamp(0., 1.),
        s: s.clamp(0., 1.),
        l: l.clamp(0., 1.),
        a: a.clamp(0., 1.),
    }
}

pub type ColorScale = [Hsla; 12];

/// A set of color scales used in a theme.
///
/// Each set contains four color scales: `light`, `dark`, `light_alpha`, and `dark_alpha`.
/// The `light` and `dark` scales are used for solid colors, while the `light_alpha` and `dark_alpha` scales are used for transparent colors.
#[derive(Debug, Clone, PartialEq)]
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
    use super::*;

    #[test]
    fn test_hsla() {
        let color = hsla(0.5, 0.5, 0.5, 0.5);
        assert_eq!(color, Hsla { h: 0.5, s: 0.5, l: 0.5, a: 0.5 });
    }

    #[test]
    fn test_hsla_clamping() {
        let color = hsla(1.5, -0.5, 0.5, 0.5);
        assert_eq!(color, Hsla { h: 1.0, s: 0.0, l: 0.5, a: 0.5 });
    }

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
