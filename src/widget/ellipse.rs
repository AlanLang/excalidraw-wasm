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
        let width = self.rect.end_x - self.rect.start_x;
        let height = self.rect.end_y - self.rect.start_y;

        let config_string = Rough::generator_ellipse(
            self.rect.start_x + width / 2,
            self.rect.start_y + height / 2,
            width,
            height,
        );
        [config_string].to_vec()
    }
}
