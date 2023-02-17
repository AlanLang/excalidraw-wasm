use crate::{model::rect::Rect, rough::Rough};

use super::shape::Shape;

#[derive(Debug, Clone)]
pub struct Arrow {
    rect: Rect,
    item_stroke_color: String,
    item_bg_color: String,
}

impl Arrow {
    pub fn new(rect: Rect, item_stroke_color: String, item_bg_color: String) -> Arrow {
        Arrow {
            rect,
            item_stroke_color,
            item_bg_color,
        }
    }
}

impl Shape for Arrow {
    fn get_config(&self) -> Vec<String> {
        let (x1, y1, x2, y2, x3, y3, x4, y4) = self.get_lines(0, 0);
        let config1 = Rough::generator_line(
            x3,
            y3,
            x2,
            y2,
            self.item_stroke_color.clone(),
            self.item_bg_color.clone(),
        );
        let config2 = Rough::generator_line(
            x1,
            y1,
            x2,
            y2,
            self.item_stroke_color.clone(),
            self.item_bg_color.clone(),
        );
        let config3 = Rough::generator_line(
            x4,
            y4,
            x2,
            y2,
            self.item_stroke_color.clone(),
            self.item_bg_color.clone(),
        );
        [config1, config2, config3].to_vec()
    }
}

impl Arrow {
    pub fn get_lines(&self, x: i32, y: i32) -> (f32, f32, f32, f32, f32, f32, f32, f32) {
        let width = (self.rect.get_width()) as f32;
        let height = (self.rect.get_height()) as f32;

        let x1 = x as f32;
        let y1 = y as f32;
        let x2 = x1 + width;
        let y2 = y1 + height;

        let size = 30 as f32; // pixels
        let distance = ((x2 - x1).powf(2.0) as f32 + (y2 - y1).powf(2.0) as f32).sqrt();
        // Scale down the arrow until we hit a certain size so that it doesn't look weird
        let min_size = size.min(distance / 2.0);
        let xs = x2 - ((x2 - x1) / distance) * min_size;
        let ys = y2 - ((y2 - y1) / distance) * min_size;

        let angle = 20 as f32; // degrees
        let [x3, y3] = rotate(xs, ys, x2, y2, (-angle * std::f32::consts::PI) / 180.0);
        let [x4, y4] = rotate(xs, ys, x2, y2, (angle * std::f32::consts::PI) / 180.0);
        (x1, y1, x2, y2, x3, y3, x4, y4)
    }
}

fn rotate(x1: f32, y1: f32, x2: f32, y2: f32, angle: f32) -> [f32; 2] {
    let x = x1 - x2;
    let y = y1 - y2;

    let x3 = x * angle.cos() - y * angle.sin();
    let y3 = x * angle.sin() + y * angle.cos();

    [x3 + x2, y3 + y2]
}
