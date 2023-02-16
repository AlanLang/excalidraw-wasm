use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = painter)]
    fn rectangle(
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        item_stroke_color: String,
        item_bg_color: String,
    ) -> String;

    #[wasm_bindgen(js_namespace = painter)]
    fn ellipse(
        center_x: i32,
        center_y: i32,
        w: i32,
        h: i32,
        item_stroke_color: String,
        item_bg_color: String,
    ) -> String;

    #[wasm_bindgen(js_namespace = painter)]
    fn line(x1: f32, y1: f32, x2: f32, y2: f32) -> String;

    #[wasm_bindgen(js_namespace = painter)]
    fn draw(config_string: String);
}

pub struct Rough {}

impl Rough {
    pub fn generator_rectangle(
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        item_stroke_color: String,
        item_bg_color: String,
    ) -> String {
        rectangle(x, y, w, h, item_stroke_color, item_bg_color)
    }

    pub fn generator_ellipse(
        center_x: i32,
        center_y: i32,
        w: i32,
        h: i32,
        item_stroke_color: String,
        item_bg_color: String,
    ) -> String {
        ellipse(center_x, center_y, w, h, item_stroke_color, item_bg_color)
    }

    pub fn generator_line(x1: f32, y1: f32, x2: f32, y2: f32) -> String {
        line(x1, y1, x2, y2)
    }

    pub fn draw_shape(config_string: &String) {
        draw(config_string.to_string())
    }
}
