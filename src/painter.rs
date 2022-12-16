use std::rc::Rc;

use gloo_utils::format::JsValueSerdeExt;
use sycamore::{
    prelude::NodeRef,
    reactive::RcSignal,
    web::{DomNode, Html},
};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::{store::Element, widget::WidgetKind};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = painter)]
    fn rectangle(x: i32, y: i32, w: i32, h: i32) -> String;

    #[wasm_bindgen(js_namespace = painter)]
    fn ellipse(center_x: i32, center_y: i32, w: i32, h: i32) -> String;

    #[wasm_bindgen(js_namespace = painter)]
    fn line(x1: f32, y1: f32, x2: f32, y2: f32) -> String;

    #[wasm_bindgen(js_namespace = painter)]
    fn draw(config_string: String);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Copy, Clone)]
pub struct Painter {}

fn get_context(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .expect("should get context")
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("should cast to context")
}

impl Painter {
    pub fn new() -> Painter {
        Painter {}
    }

    pub fn rectangle(&self, x: i32, y: i32, w: i32, h: i32) -> String {
        rectangle(x, y, w, h)
    }

    pub fn ellipse(&self, center_x: i32, center_y: i32, w: i32, h: i32) -> String {
        ellipse(center_x, center_y, w, h)
    }

    pub fn line(&self, x1: f32, y1: f32, x2: f32, y2: f32) -> String {
        line(x1, y1, x2, y2)
    }

    pub fn selection(&self, _x: i32, _y: i32, _w: i32, _h: i32) -> String {
        "".to_string()
    }

    pub fn draw_shape(&self, config_string: &String) {
        draw(config_string.to_string());
    }

    pub fn draw_selection(&self, canvas: &HtmlCanvasElement, element: &Rc<Element>) {
        let ctx = get_context(canvas);
        let rect = element.rect;
        let fill_style = ctx.fill_style();
        ctx.set_fill_style(&JsValue::from_str("rgba(0, 0, 255, 0.10)"));
        ctx.fill_rect(
            rect.start_x as f64,
            rect.start_y as f64,
            (rect.end_x - rect.start_x) as f64,
            (rect.end_y - rect.start_y) as f64,
        );
        ctx.stroke();
        ctx.set_fill_style(fill_style.as_ref());
    }

    pub fn draw_selection_border(&self, canvas: &HtmlCanvasElement, element: &Rc<Element>) {
        let ctx = get_context(canvas);
        let dash = JsValue::from_serde(&[8.0]).unwrap();
        let line_dash = ctx.get_line_dash();
        ctx.set_line_dash(&dash).unwrap();
        let margin = 4;
        let x = element.rect.start_x;
        let y = element.rect.start_y;
        let width = element.rect.end_x - element.rect.start_x;
        let height = element.rect.end_y - element.rect.start_y;
        ctx.stroke_rect(
            (x - margin).into(),
            (y - margin).into(),
            (width + margin * 2).into(),
            (height + margin * 2).into(),
        );
        ctx.set_line_dash(line_dash.as_ref()).unwrap();
    }

    pub fn draw_text(&self, canvas: &HtmlCanvasElement, element: &Rc<Element>) {
        let ctx = get_context(canvas);
        let font = ctx.font();
        ctx.set_font("normal 20px Virgil");
        let text = element.shape_string.get();
        let text = text.first();
        let text = match text {
            Some(text) => text,
            None => return,
        };
        let text_measure = ctx.measure_text(&text).unwrap();

        let x = element.rect.start_x;
        let y = element.rect.start_y + text_measure.font_bounding_box_ascent() as i32;

        ctx.fill_text(text.as_str(), x.into(), y.into()).unwrap();
        ctx.set_font(font.as_ref());
    }

    pub fn draw_elements<G: Html>(
        &self,
        canvas_ref: &NodeRef<G>,
        elements: Rc<Vec<RcSignal<Element>>>,
    ) {
        self.clear_canvas(canvas_ref);
        let html_canvas_element: HtmlCanvasElement = canvas_ref.get::<DomNode>().unchecked_into();

        elements.iter().for_each(|element| {
            let element = element.get();
            let shape_string = element.shape_string.get();
            if element.kind == WidgetKind::Text {
                self.draw_text(&html_canvas_element, &element);
            } else {
                shape_string.iter().for_each(|shape| {
                    self.draw_shape(shape);
                });
            }
            if element.kind == WidgetKind::Selection {
                self.draw_selection(&html_canvas_element, &element);
            }
            if element.is_selected {
                self.draw_selection_border(&html_canvas_element, &element)
            }
        });
    }

    pub fn clear_canvas<G: Html>(&self, canvas_ref: &NodeRef<G>) {
        let html_canvas_element: HtmlCanvasElement = canvas_ref.get::<DomNode>().unchecked_into();
        let ctx = html_canvas_element
            .get_context("2d")
            .expect("should get context")
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .expect("should cast to context");
        let canvas_width = html_canvas_element.width();
        let canvas_height = html_canvas_element.height();
        ctx.clear_rect(0.0, 0.0, canvas_width as f64, canvas_height as f64);
    }
}
