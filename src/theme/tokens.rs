use iced::Color;

/// A color scale from 50 (darkest) to 950 (lightest)
#[derive(Debug, Clone, Copy)]
pub struct ColorScale {
    pub c50: Color,
    pub c100: Color,
    pub c200: Color,
    pub c300: Color,
    pub c400: Color,
    pub c500: Color,
    pub c600: Color,
    pub c700: Color,
    pub c800: Color,
    pub c900: Color,
    pub c950: Color,
}

/// Border radius tokens
#[derive(Debug, Clone, Copy)]
pub struct BorderRadius {
    pub small: f32,
    pub medium: f32,
    pub large: f32,
    pub x_large: f32,
}

/// Spacing tokens
#[derive(Debug, Clone, Copy)]
pub struct Spacing {
    pub x3_small: f32,
    pub x2_small: f32,
    pub x_small: f32,
    pub small: f32,
    pub medium: f32,
    pub large: f32,
    pub x_large: f32,
    pub x2_large: f32,
    pub x3_large: f32,
    pub x4_large: f32,
}

/// Transition duration tokens (in milliseconds)
#[derive(Debug, Clone, Copy)]
pub struct Transition {
    pub x_slow: u64,
    pub slow: u64,
    pub medium: u64,
    pub fast: u64,
    pub x_fast: u64,
}

/// Font size tokens
#[derive(Debug, Clone, Copy)]
pub struct FontSize {
    pub x2_small: f32,
    pub x_small: f32,
    pub small: f32,
    pub medium: f32,
    pub large: f32,
    pub x_large: f32,
    pub x2_large: f32,
    pub x3_large: f32,
    pub x4_large: f32,
}

/// Font weight tokens
#[derive(Debug, Clone, Copy)]
pub struct FontWeight {
    pub light: u16,
    pub normal: u16,
    pub semibold: u16,
    pub bold: u16,
}

/// Line height tokens
#[derive(Debug, Clone, Copy)]
pub struct LineHeight {
    pub denser: f32,
    pub dense: f32,
    pub normal: f32,
    pub loose: f32,
    pub looser: f32,
}

/// Input height tokens
#[derive(Debug, Clone, Copy)]
pub struct InputHeight {
    pub small: f32,
    pub medium: f32,
    pub large: f32,
}

/// Toggle size tokens
#[derive(Debug, Clone, Copy)]
pub struct ToggleSize {
    pub small: f32,
    pub medium: f32,
    pub large: f32,
}

/// Z-index tokens
#[derive(Debug, Clone, Copy)]
pub struct ZIndex {
    pub drawer: i32,
    pub dialog: i32,
    pub dropdown: i32,
    pub toast: i32,
    pub tooltip: i32,
}

/// Complete theme token set
#[derive(Debug, Clone, Copy)]
pub struct Tokens {
    // Color scales
    pub gray: ColorScale,
    pub red: ColorScale,
    pub orange: ColorScale,
    pub amber: ColorScale,
    pub yellow: ColorScale,
    pub lime: ColorScale,
    pub green: ColorScale,
    pub emerald: ColorScale,
    pub teal: ColorScale,
    pub cyan: ColorScale,
    pub sky: ColorScale,
    pub blue: ColorScale,
    pub indigo: ColorScale,
    pub violet: ColorScale,
    pub purple: ColorScale,
    pub fuchsia: ColorScale,
    pub pink: ColorScale,
    pub rose: ColorScale,

    // Semantic colors
    pub primary: ColorScale,
    pub success: ColorScale,
    pub warning: ColorScale,
    pub danger: ColorScale,
    pub neutral: ColorScale,

    // Special neutral colors
    pub neutral_0: Color,
    pub neutral_1000: Color,

    // Design tokens
    pub border_radius: BorderRadius,
    pub spacing: Spacing,
    pub transition: Transition,
    pub font_size: FontSize,
    pub font_weight: FontWeight,
    pub line_height: LineHeight,
    pub input_height: InputHeight,
    pub toggle_size: ToggleSize,
    pub z_index: ZIndex,
}
