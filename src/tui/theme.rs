use ratatui::style::{Color, Modifier, Style};

// Category colors.
pub const DEV_CACHE_COLOR: Color = Color::Cyan;
pub const PROJECT_ARTIFACT_COLOR: Color = Color::Green;
pub const SYSTEM_JUNK_COLOR: Color = Color::Yellow;
pub const APP_CACHE_COLOR: Color = Color::Magenta;
pub const EXTERNAL_COLOR: Color = Color::Blue;

// Risk colors.
pub const SAFE_COLOR: Color = Color::Green;
pub const CAUTION_COLOR: Color = Color::Yellow;
pub const DANGEROUS_COLOR: Color = Color::Red;

// UI elements.
pub const TITLE_STYLE: Style = Style::new().fg(Color::White).add_modifier(Modifier::BOLD);
pub const DIM_STYLE: Style = Style::new().fg(Color::DarkGray);
pub const HELP_STYLE: Style = Style::new().fg(Color::DarkGray);
pub const SIZE_STYLE: Style = Style::new().fg(Color::Cyan);
pub const PATH_STYLE: Style = Style::new().fg(Color::DarkGray);

// Symbols.
pub const CHECKBOX_ON: &str = "[x]";
pub const CHECKBOX_OFF: &str = "[ ]";
pub const CATEGORY_EXPANDED: &str = "v ";
pub const CATEGORY_COLLAPSED: &str = "> ";
pub const GROUP_EXPANDED: &str = "▾ ";
pub const GROUP_COLLAPSED: &str = "▸ ";
pub const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

use crate::rules::Category;

pub fn category_color(category: Category) -> Color {
    match category {
        Category::DevCache => DEV_CACHE_COLOR,
        Category::ProjectArtifact => PROJECT_ARTIFACT_COLOR,
        Category::SystemJunk => SYSTEM_JUNK_COLOR,
        Category::AppCache => APP_CACHE_COLOR,
        Category::External => EXTERNAL_COLOR,
    }
}
