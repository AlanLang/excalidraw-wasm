use crate::painter::Painter;

use super::shape::{Rect, Shape};

#[derive(Debug, Clone, Copy)]
pub struct Selection {
    rect: Rect,
}

impl Selection {
    pub fn new(rect: Rect) -> Selection {
        Selection { rect }
    }
}

impl Shape for Selection {
    fn get_config(&self, painter: &Painter) -> Vec<String> {
        let width = self.rect.end_x - self.rect.start_x;
        let height = self.rect.end_y - self.rect.start_y;

        let config_string = painter.selection(self.rect.start_x, self.rect.start_y, width, height);
        [config_string].to_vec()
    }
}
