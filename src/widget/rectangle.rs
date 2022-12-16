use crate::rough::Rough;

use super::shape::{Rect, Shape};

#[derive(Copy, Clone)]
pub struct Rectangle {
    rect: Rect,
}

impl Rectangle {
    pub fn new(rect: Rect) -> Rectangle {
        Rectangle { rect: rect }
    }
}

impl Shape for Rectangle {
    fn get_config(&self) -> Vec<String> {
        let config_string =
            Rough::generator_rectangle(0, 0, self.rect.get_width(), self.rect.get_height());
        [config_string].to_vec()
    }
}
