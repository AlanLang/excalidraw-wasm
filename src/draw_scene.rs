use gloo_utils::format::JsValueSerdeExt;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::{
    model::{element::Element, widget_kind::WidgetKind, AppData},
    rough::Rough,
};

fn get_context(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .expect("should get context")
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("should cast to context")
}

pub fn draw_scene(canvas: HtmlCanvasElement, app_data: &AppData) {
    clear_canvas(&canvas);
    let ctx = get_context(&canvas);
    app_data.elements.iter().for_each(|element| {
        let rect = element.rect;
        if element.kind == WidgetKind::Text {
            draw_text(&canvas, &element);
        } else if element.kind == WidgetKind::Selection {
            draw_selection(&canvas, &element)
        } else {
            ctx.translate(rect.start_x.into(), rect.start_y.into())
                .unwrap();
            element.shape_string.iter().for_each(|shape| {
                Rough::draw_shape(shape);
            });
            ctx.translate((-rect.start_x).into(), (-rect.start_y).into())
                .unwrap();
        }
        if element.is_selected {
            draw_selection_border(&canvas, &element)
        }
    })
}

fn draw_text(canvas: &HtmlCanvasElement, element: &Element) {
    let ctx = get_context(canvas);
    let font = ctx.font();
    let fill_style = ctx.fill_style();
    ctx.set_fill_style(&JsValue::from_str(
        element.config.item_stroke_color.as_str(),
    ));
    ctx.set_font("normal 20px Virgil");
    let text = element.shape_string.clone();
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
    ctx.set_fill_style(fill_style.as_ref());
}

fn draw_selection(canvas: &HtmlCanvasElement, element: &Element) {
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
    ctx.set_fill_style(fill_style.as_ref());
}

fn draw_selection_border(canvas: &HtmlCanvasElement, element: &Element) {
    let ctx = get_context(canvas);
    let dash = JsValue::from_serde(&[8.0]).unwrap();
    let line_dash = ctx.get_line_dash();
    ctx.set_line_dash(&dash).unwrap();
    let margin = 4;
    let x = element.rect.start_x.min(element.rect.end_x);
    let y = element.rect.start_y.min(element.rect.end_y);
    let width = (element.rect.end_x - element.rect.start_x).abs();
    let height = (element.rect.end_y - element.rect.start_y).abs();
    ctx.stroke_rect(
        (x - margin).into(),
        (y - margin).into(),
        (width + margin * 2).into(),
        (height + margin * 2).into(),
    );
    ctx.set_line_dash(line_dash.as_ref()).unwrap();
}

fn clear_canvas(canvas: &HtmlCanvasElement) {
    let ctx = canvas
        .get_context("2d")
        .expect("should get context")
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("should cast to context");
    let canvas_width = canvas.width();
    let canvas_height = canvas.height();
    ctx.clear_rect(-0.5, -0.5, canvas_width as f64, canvas_height as f64);
}
