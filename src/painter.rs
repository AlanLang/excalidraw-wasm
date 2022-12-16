use std::rc::Rc;

use gloo_utils::format::JsValueSerdeExt;
use sycamore::{
    prelude::NodeRef,
    reactive::RcSignal,
    web::{DomNode, Html},
};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::{rough::Rough, store::Element, widget::WidgetKind};

fn get_context(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .expect("should get context")
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("should cast to context")
}

#[derive(Clone)]
pub struct Painter {}

impl Painter {
    pub fn new() -> Painter {
        Painter {}
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
        let ctx = get_context(&html_canvas_element);

        elements.iter().for_each(|element| {
            let element = element.get();
            let rect = element.rect;
            let shape_string = element.shape_string.get();
            if element.kind == WidgetKind::Text {
                self.draw_text(&html_canvas_element, &element);
            } else {
                ctx.translate(rect.start_x.into(), rect.start_y.into())
                    .unwrap();
                shape_string.iter().for_each(|shape| {
                    Rough::draw_shape(shape);
                });
                ctx.translate((-rect.start_x).into(), (-rect.start_y).into())
                    .unwrap();
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
