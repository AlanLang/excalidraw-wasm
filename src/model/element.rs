use std::sync::atomic::AtomicU32;

use serde::{Deserialize, Serialize};

use super::{rect::Rect, widget_kind::WidgetKind};
static NEXT_ELEMENT_ID: std::sync::atomic::AtomicU32 = AtomicU32::new(1);

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Element {
    pub id: u32,
    pub is_selected: bool,
    pub kind: WidgetKind,
    pub rect: Rect,
    pub shape_string: Vec<String>,
}

impl Element {
    pub fn new(kind: WidgetKind) -> Self {
        let id = NEXT_ELEMENT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        Self {
            id: id,
            is_selected: false,
            kind: kind,
            rect: Rect::default(),
            shape_string: Vec::new(),
        }
    }

    pub fn update_rect(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.rect = Rect::new(x1, y1, x2, y2);
    }

    pub fn update_shape_string(&mut self, shape_string: Vec<String>) {
        self.shape_string = shape_string;
    }

    pub fn set_selected(&mut self, is_selected: bool) {
        self.is_selected = is_selected;
    }

    pub fn is_selected(&self) -> bool {
        self.is_selected
    }

    pub fn move_element(&mut self, x: i32, y: i32) {
        self.rect = Rect::new(
            self.rect.start_x + x,
            self.rect.start_y + y,
            self.rect.end_x + x,
            self.rect.end_y + y,
        );
    }

    pub fn from(element: &Element) -> Self {
        let id = NEXT_ELEMENT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        Self {
            id,
            is_selected: element.is_selected,
            kind: element.kind.clone(),
            rect: element.rect.clone(),
            shape_string: element.shape_string.clone(),
        }
    }
}
