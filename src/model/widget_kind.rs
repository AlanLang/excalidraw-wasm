use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WidgetKind {
    Rectangle,
    Ellipse,
    Arrow,
    Text,
    Selection,
}

impl std::fmt::Display for WidgetKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rectangle => write!(f, "Rectangle"),
            Self::Ellipse => write!(f, "Ellipse"),
            Self::Arrow => write!(f, "Arrow"),
            Self::Text => write!(f, "Text"),
            Self::Selection => write!(f, "Selection"),
        }
    }
}

impl Default for WidgetKind {
    fn default() -> Self {
        Self::Rectangle
    }
}
