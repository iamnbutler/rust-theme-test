Build a theme system in Rust that has the following properties.

## Development Notes:

- As an expert rust developer with 10+ years of experience, how would you design this system? Ensure you use common rust patterns and idioms.
- Think through the entire system before you start coding. Think through the problems end to end.
- This system is quite complex, so break it down into pieces and consider each piece 2-3 times before you start coding.
- Use macros or other tools to reduce the amount of boilerplate code developers working with the system will need to write.
- Use `cargo test` to run your tests. Ensure tests are written for all non-trivial code.
- Make use of the cargo docs feature to document your code. For each major concept write up a doc comment introducint it and how to work with it.

## Structure of a module:

Each module you create should use a structure like this:

- `[file-name].rs`:
    - mods needed to import/export the module
    - Documentation for the module, this is more detailed than function documentation, outlining the purpose of the module, and it's responsibilities.
    - Module code
    - Tests for the module. These should be located in the same file as the code they are testing, not in a separate file or folder.

## Goals:
- Adding new UIColor should be easy, and require adding it to only one place ideally.
- Accessing a UIColor should be easy, and require only a single line of code.
- UIColors should never be edited directly, only a theme should be edited.
- The theme system codebase should be easy to understand if a developer is interfacing with it for the first time. Document complex code throughly when not obvious.

## UIColor: Map values to UI elements.

- A UI color has a name, a value, and a description.
- It's value should be an index of `ColorScale`

An example of a UIColor:

```rust
UIColor {
    name: "filled-element-background",
    value: ColorScale[4],
    description: "Used for the background of filled elements, like buttons and checkboxes."
}
```

## UIColors: A collection of UIColors.

- Consider using something like a BTreeMap, as it is important we respect the order of the UI elements.

Example:

```rust
UIColors {
    FilledElementBackground: UIColor {
        name: "filled-element-background",
        value: ColorScale[4],
        description: "Used for the background of filled elements, like buttons and checkboxes."
    },
    // etc
}
```

## Uses Hsla color for ColorScale values.

Assume the defintion of Hsla is:

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


## Uses `ColorScale`s to represent the range of values a color can have.

- A color scale as an array with exactly 12 Hsla values ([Hsla; 12])
- A light color scale starts from the lightest color and goes to the darkest color.
- A dark color scale starts from the darkest color and goes to the lightest color.

Example:

```rust
dark: [hsla(0.0,0.0,0.0,1.0), hsla(0.0,0.0,0.05,1.0, //etc]
```

## `ColorScaleSet`:

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

Themes may define their own `ColorScaleSet`s, but by default they should use the `default` ColorScaleSet.

Example:

```rust
ColorScaleSet {
    light: // ColorScaleSet,
    dark: // ColorScaleSet,
    light_alpha: // ColorScaleSet,
    dark_alpha: // ColorScaleSet,
}
```

## Requirements

- Return at least these two things:
- `ColorScaleSets` - a map of ColorScaleSets.
- `UIColors` - a map of UIColors, built from the ColorScaleSets.
