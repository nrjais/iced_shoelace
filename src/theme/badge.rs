/// Badge variant types matching Shoelace design system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeVariant {
    #[default]
    Primary,
    Success,
    Neutral,
    Warning,
    Danger,
}
