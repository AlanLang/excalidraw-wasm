use self::{
    ellipse::Ellipse,
    rectangle::Rectangle,
    shape::{Rect, Shape},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

pub mod arrow;
pub mod ellipse;
pub mod rectangle;
pub mod selection;
pub mod shape;

pub fn create_widget(widget_type: WidgetKind, rect: Rect) -> Box<dyn Shape> {
    match widget_type {
        WidgetKind::Rectangle => Box::new(Rectangle::new(rect)),
        WidgetKind::Ellipse => Box::new(Ellipse::new(rect)),
        WidgetKind::Arrow => Box::new(arrow::Arrow::new(rect)),
        WidgetKind::Selection => Box::new(selection::Selection::new(rect)),
        _ => unimplemented!(),
    }
}
