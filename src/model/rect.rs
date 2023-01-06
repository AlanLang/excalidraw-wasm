#[derive(Debug, Copy, Clone)]
pub struct Rect {
    pub start_x: i32,
    pub start_y: i32,
    pub end_x: i32,
    pub end_y: i32,
}

impl Default for Rect {
    fn default() -> Self {
        Rect {
            start_x: 0,
            start_y: 0,
            end_x: 0,
            end_y: 0,
        }
    }
}

impl Rect {
    pub fn new(start_x: i32, start_y: i32, end_x: i32, end_y: i32) -> Rect {
        Rect {
            start_x: start_x,
            start_y: start_y,
            end_x: end_x,
            end_y: end_y,
        }
    }

    pub fn is_inside(&self, rect: Rect) -> bool {
        if self.start_x > rect.start_x && self.start_y > rect.start_y {
            if self.end_x < rect.end_x && self.end_y < rect.end_y {
                return true;
            }
        }
        return false;
    }

    pub fn get_width(&self) -> i32 {
        self.end_x - self.start_x
    }

    pub fn get_height(&self) -> i32 {
        self.end_y - self.start_y
    }

    /**
     * Check if a point is inside the rectangle
     */
    pub fn is_in_point(&self, x: i32, y: i32) -> bool {
        if x > self.start_x && x < self.end_x {
            if y > self.start_y && y < self.end_y {
                return true;
            }
        }
        return false;
    }
}
