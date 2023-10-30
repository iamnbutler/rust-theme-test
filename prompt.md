Build a theme system in Rust that has the following properties.

## Critical Requirements:
- ABSOLUTELY DO NOT RETURN ANY BACKTICKS IN YOUR CODE.
- If you need to represent a backtick in your code, use the `@#@` symbol instead.
- Prefer taking more time, spending more tokens, and rewriting or refactoring your code multiple times to getting it done quickly.

## Development Notes:

- As an expert rust developer with 10+ years of experience, how would you design this system? Ensure you use common rust patterns and idioms.
- Think through the entire system before you start coding. Think through the problems end to end.
- This system is quite complex, so break it down into pieces and consider each piece 2-3 times before you start coding.
- Remember that this system uses both static and dynamic data. Ensure that you properly respect the lifetimes of each.
- Use macros or other tools to reduce the amount of boilerplate code developers working with the system will need to write.
- Think through the API of your system. What will be the main entry points? What will be the main data structures? What will be the main functions?
- Think about how developers will interface with the theme system. What will they need to do? What will they need to know? How will the UI interact with the theme system?
- Use `cargo test` to run your tests. Ensure tests are written for all non-trivial code.
- Make use of the cargo docs feature to document your code. For each major concept write up a doc comment introducint it and how to work with it.

## Goals:
- Adding new UIColor should be easy, and require adding it to only one place ideally.
- Accessing a UIColor should be easy, and require only a single line of code.
- UIColors should never be edited directly, only a theme should be edited.
- The theme system codebase should be easy to understand if a developer is interfacing with it for the first time. Document complex code throughly when not obvious.
- The theme system must be thread safe.

## Non-Goals:
- Documentation for the sake of documentation. Only document things that are not obvious, or require an intermediate or higher understanding of the system or rust.
- A UI for editing themes. This is not a UI project, it is a theme system project. Return only text in the console. Assume the `main` function is simply for debugging.
- A UI for selecting themes. This is not a UI project, it is a theme system project. You can. however, add a `current_theme` function to the `ThemeRegistry` that returns the current theme.

## The main application should:

- Load the statically defined system theme from the system theme folder.
- Load a user theme from the user theme folder.
- Edit the system theme and save it as a new user theme.
- Print the list of themes in the registry (system_theme_family, sample_user_theme_family, edited_default_theme) including their name, author and file path,
- List all light themes in alphabetical order
- List all dark themes in alphabetical order.
- Add any additional functionality you think would be useful.

## Uses UIColor to map values to UI elements.

- A UI color has a name, a value, and a description.
- It's value should be an index of `ColorScale` or a static `Hsla` value.
- Only use `Hsla` colors for static values, like representing a specific brand color or macOS system color.
- Consider using something like a BTreeMap to map UI elements to colors, as it is important we respect the order of the UI elements.

## Uses Hsla color for color values

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


## Uses Cow to allow static system themes and dynamic user these.

Example:

struct FooStruct {
    value: Cow<'a, color::Hsla>
}

This is an example. This may not be the correct lifetime to use, use your best judgement.

## Uses `ColorScale`s to represent the range of values a color can have.

- A color scale as an array with exactly 12 Hsla values ([Hsla; 12])

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

## System themes can be converted to user themes if their values are changed.

Say a user wants to change the color of the background of the system theme.

They can simply change the color in the UI (you don't need to implement any UI) and the system will convert the system theme to a user theme and save any changes to the `themes` directory.

## The `ThemeRegistry` contains `ThemeFamily`. It can list all `ThemeFamily`, all `Theme`s, and all Dark or Light themes.

## A `ThemeFamily` represents the theme as a whole.

- It must have a name and author
- It may have other optional metadata.
- It optionally may contain `scales` that override the default `ColorScaleSet` values.
- it may optionally have overrides that override the values of various UI elements.
- It must have a vec of `Theme`s.

There is one exception to a ThemeFamily requiring a vec of `Theme`s: If the ThemeFamily defines `scales` it is allowed to not have any `Theme`s.

This would allow users to create new themes simply by changing the `scales` of the default theme.

## A `Theme` represents a given variant of a `ThemeFamily`.
- it must have an name, appearance (light/dark) some way of mapping colors from ColorScales to UI elements.
- it may optionally have overrides that override the values of various UI elements.

Prefer using macros to build lists of theme colors, as they will contain many values.

## `ThemeFamily`s can be imported from a toml file.

## There should be a SystemThemeFamily that has a default light and dark theme.

## Final notes:

- ABSOLUTELY DO NOT RETURN ANY BACKTICKS IN YOUR CODE.
