use crate::{model::rect::Rect, rough::Rough};

use super::shape::Shape;

#[derive(Clone)]
pub struct Rectangle {
    rect: Rect,
    item_stroke_color: String,
    item_bg_color: String,
}

impl Rectangle {
    pub fn new(rect: Rect, item_stroke_color: String, item_bg_color: String) -> Rectangle {
        Rectangle {
            rect,
            item_stroke_color,
            item_bg_color,
        }
    }
}

impl Shape for Rectangle {
    fn get_config(&self) -> Vec<String> {
        let config_string = Rough::generator_rectangle(
            0,
            0,
            self.rect.get_width(),
            self.rect.get_height(),
            self.item_stroke_color.clone(),
            self.item_bg_color.clone(),
        );
        [config_string].to_vec()
    }
}
