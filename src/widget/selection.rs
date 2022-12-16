use super::shape::Shape;

#[derive(Debug, Clone, Copy)]
pub struct Selection {}

impl Selection {
    pub fn new() -> Selection {
        Selection {}
    }
}

impl Shape for Selection {
    fn get_config(&self) -> Vec<String> {
        [].to_vec()
    }
}
