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
}
