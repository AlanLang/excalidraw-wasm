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
        let config_string = Rough::generator_rectangle(
            self.rect.start_x,
            self.rect.start_y,
            self.rect.end_x - self.rect.start_x,
            self.rect.end_y - self.rect.start_y,
        );
        [config_string].to_vec()
    }
}
