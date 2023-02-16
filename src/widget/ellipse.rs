use crate::{model::rect::Rect, rough::Rough};

use super::shape::Shape;

#[derive(Debug, Clone)]
pub struct Ellipse {
    rect: Rect,
    item_stroke_color: String,
    item_bg_color: String,
}

impl Ellipse {
    pub fn new(rect: Rect, item_stroke_color: String, item_bg_color: String) -> Ellipse {
        Ellipse {
            rect,
            item_bg_color,
            item_stroke_color,
        }
    }
}

impl Shape for Ellipse {
    fn get_config(&self) -> Vec<String> {
        let config_string = Rough::generator_ellipse(
            self.rect.get_width() / 2,
            self.rect.get_height() / 2,
            self.rect.get_width(),
            self.rect.get_height(),
            self.item_stroke_color.clone(),
            self.item_bg_color.clone(),
        );
        [config_string].to_vec()
    }
}
