Develop a Rust theme system with the following attributes:

Critical Requirements:

1. ABSOLUTELY DO NOT RETURN ANY BACKTICKS IN YOUR CODE. Replace backticks with `@#@`.
2. The goal is quality, not speed.
3. The system involves both static and dynamic data.
4. Commit your work often, not in a single commit.
5. Tests and documentation should be colocated with the appropriate code.
6. ANY USE OF A BACKTICK CHARACTER, GRAVE CHARACTER OR ASCII CODE 96 WILL RESULT IN IMMEDIATE REJECTTION AND TERMINATION.

Development Notes:

As an expert rust developer with 10+ years of experience, how would you design this system? Ensure you use common rust patterns and idioms.
Remember that this system uses both static and dynamic data. Ensure that you properly respect the lifetimes of each.

1. Plan before coding.
2. Systematically decompose the complex system.
3. Account for both static and dynamic data.
4. Use macros to lessen boilerplate code.
5. Plan the API. Define main entry points, data structures, and functions.
6. Foresee how developers will interact with the theme system.
7. Write tests and docs for major functions. They should be located in the same file as the function.
8. Write documention intro blocks for major concepts..

Goals:
1. Easily add or access UIColor.
2. UIColor can only be edited via a theme.
3. The theme codebase should be uncomplicated yet thread safe.

Non-Goals:
1. We document only non-obvious concepts.
2. Any UI.

The application will:

- Load system and user themes.
- Modify system theme and save as a fresh user theme.
- Display themes in the registry & their details.
- Alphabetically list all light/dark themes.

Elements will map to UIColors, with each color having a name, value, and description. Respect the UI elements' order; consider using BTreeMap.

Use Hsla for color values; static values like brand color should be in Hsla.

We use this struct and function for hsla:

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

The UIColor struct is how color values are mapped to UI elements.

- A UI color has a name, a value, and a description.
- It's value should be an index of `ColorScale` or a static `Hsla` value.
- Only use `Hsla` colors for static values, like representing a specific brand color or macOS system color.
- Consider using something like a BTreeMap to map UI elements to colors, as it is important we respect the order of the UI elements.

If needed, utalize Cow to avoid cloning strings and handle the dual static and dynamic data.

Use `ColorScale`s to represent the range of values a color can have.

- A color scale as an array with exactly 12 Hsla values ([Hsla; 12])

ColorScaleSet:

- A ColorScaleSet has exactly 4 ColorScale
- A ColorScaleSet has a name
- A color scale has documentation that explains the semantic meaning of each value in the scale, see above.
- The 4 ColorScale in a ColorScaleSet are named `light`, `dark`, `light_alpha` and `dark_alpha`.
- The `light` and `dark` scales are used for solid colors.
- The `light_alpha` and `dark_alpha` scales are used for transparent colors.

The system should have a `default` ColorScaleSet that has the following values:

- gray
- red
- green
- blue
- yellow

Themes can define their own ColorScaleSets, if not they use the default.

## Uses a `themes` folder to store themes.

Rough structure:

repo_root/
    src/
        themes/
            system/
                default/
                    index.rs (ThemeFamily)
                    light_theme.rs (Theme)
                    dark_theme.rs (Theme)
            user/
                one/
                    index.toml (ThemeFamily)
                    one_light.toml (Theme)
                    one_light_material.toml (Theme)
                    one_dark.toml (Theme)
                    one_darker.toml (Theme)

Notice the system themes are stored in rust files, and the user themes are stored in toml files.

The system-to-user theme conversion is possible upon value modification.

In the ThemeRegistry, a ThemeFamily represents the theme as a whole i.e., a name, author, optionally, scales and overrides, plus a vector of Themes. A ThemeFamily with defined scales need not have any Themes.

Theme has a name, appearance, and a method to map colors to UI elements, optionally, overrides. Macros can facilitate theme color listing.

You can import ThemeFamily from a toml file.

There will be a SystemThemeFamily with default light and dark themes.

Finally, don't use backticks in code.

Maintain clarity for a LLM.

ABSOLUTELY DO NOT RETURN ANY BACKTICKS IN YOUR CODE.
