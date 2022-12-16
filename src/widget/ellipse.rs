use crate::rough::Rough;

use super::shape::{Rect, Shape};

#[derive(Debug, Clone, Copy)]
pub struct Ellipse {
    rect: Rect,
}

impl Ellipse {
    pub fn new(rect: Rect) -> Ellipse {
        Ellipse { rect }
    }
}

impl Shape for Ellipse {
    fn get_config(&self) -> Vec<String> {
        let config_string = Rough::generator_ellipse(
            self.rect.get_width() / 2,
            self.rect.get_height() / 2,
            self.rect.get_width(),
            self.rect.get_height(),
        );
        [config_string].to_vec()
    }
}
