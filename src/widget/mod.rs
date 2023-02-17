use crate::model::{rect::Rect, widget_kind::WidgetKind};

use self::{ellipse::Ellipse, rectangle::Rectangle, shape::Shape};

pub mod arrow;
pub mod ellipse;
pub mod rectangle;
pub mod selection;
pub mod shape;

pub fn create_widget(
    widget_type: WidgetKind,
    rect: Rect,
    item_stroke_color: String,
    item_bg_color: String,
) -> Box<dyn Shape> {
    match widget_type {
        WidgetKind::Rectangle => Box::new(Rectangle::new(rect, item_stroke_color, item_bg_color)),
        WidgetKind::Ellipse => Box::new(Ellipse::new(rect, item_stroke_color, item_bg_color)),
        WidgetKind::Arrow => Box::new(arrow::Arrow::new(rect, item_stroke_color, item_bg_color)),
        WidgetKind::Selection => Box::new(selection::Selection::new()),
        _ => unimplemented!(),
    }
}
