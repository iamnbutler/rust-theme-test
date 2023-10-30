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
}
