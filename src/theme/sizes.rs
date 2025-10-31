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

// Border radius tokens (in pixels)
pub const BORDER_RADIUS: BorderRadius = BorderRadius {
    small: 3.0,    // 0.1875rem
    medium: 4.0,   // 0.25rem
    large: 8.0,    // 0.5rem
    x_large: 16.0, // 1rem
};

// Spacing tokens (in pixels)
pub const SPACING: Spacing = Spacing {
    x3_small: 2.0,  // 0.125rem
    x2_small: 4.0,  // 0.25rem
    x_small: 8.0,   // 0.5rem
    small: 12.0,    // 0.75rem
    medium: 16.0,   // 1rem
    large: 20.0,    // 1.25rem
    x_large: 28.0,  // 1.75rem
    x2_large: 36.0, // 2.25rem
    x3_large: 48.0, // 3rem
    x4_large: 72.0, // 4.5rem
};

// Font sizes (in pixels)
pub const FONT_SIZE: FontSize = FontSize {
    x2_small: 10.0, // 0.625rem
    x_small: 12.0,  // 0.75rem
    small: 14.0,    // 0.875rem
    medium: 16.0,   // 1rem
    large: 20.0,    // 1.25rem
    x_large: 24.0,  // 1.5rem
    x2_large: 36.0, // 2.25rem
    x3_large: 48.0, // 3rem
    x4_large: 72.0, // 4.5rem
};

// Font weights
pub const FONT_WEIGHT: FontWeight = FontWeight {
    light: 300,
    normal: 400,
    semibold: 500,
    bold: 700,
};

// Line heights
pub const LINE_HEIGHT: LineHeight = LineHeight {
    denser: 1.0,
    dense: 1.4,
    normal: 1.8,
    loose: 2.2,
    looser: 2.6,
};

// Input heights (in pixels)
pub const INPUT_HEIGHT: InputHeight = InputHeight {
    small: 30.0,  // 1.875rem
    medium: 40.0, // 2.5rem
    large: 50.0,  // 3.125rem
};

// Toggle sizes (in pixels)
pub const TOGGLE_SIZE: ToggleSize = ToggleSize {
    small: 14.0,  // 0.875rem
    medium: 18.0, // 1.125rem
    large: 22.0,  // 1.375rem
};

// Z-index values
pub const Z_INDEX: ZIndex = ZIndex {
    drawer: 700,
    dialog: 800,
    dropdown: 900,
    toast: 950,
    tooltip: 1000,
};
