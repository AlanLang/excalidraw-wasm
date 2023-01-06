use crate::model::{element::Element, widget_kind::WidgetKind};

const LINE_THRESHOLD: f32 = 10.0;

pub fn hit_test(element: &Element, x: i32, y: i32) -> bool {
    match element.kind {
        WidgetKind::Rectangle => hit_test_by_rectangle(element, x, y),
        // There doesn't seem to be a closed form solution for the distance between
        // a point and an ellipse, let's assume it's a rectangle for now...
        WidgetKind::Ellipse => hit_test_by_rectangle(element, x, y),
        _ => unimplemented!(),
    }
}

fn hit_test_by_rectangle(element: &Element, x: i32, y: i32) -> bool {
    let x1: f32 = element.rect.start_x as f32;
    let x2 = element.rect.end_x as f32;
    let y1 = element.rect.start_y as f32;
    let y2 = element.rect.end_y as f32;
    let x = x as f32;
    let y = y as f32;
    // (x1, y1) --A-- (x2, y1)
    //    |D             |B
    // (x1, y2) --C-- (x2, y2)
    distance_between_point_and_segment(x, y, x1, y1, x2, y1) < LINE_THRESHOLD
        || distance_between_point_and_segment(x, y, x2, y1, x2, y2) < LINE_THRESHOLD
        || distance_between_point_and_segment(x, y, x1, y2, x2, y2) < LINE_THRESHOLD
        || distance_between_point_and_segment(x, y, x1, y1, x1, y2) < LINE_THRESHOLD
}

fn distance_between_point_and_segment(x: f32, y: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let a = x - x1;
    let b = y - y1;
    let c = x2 - x1;
    let d = y2 - y1;

    let dot = a * c + b * d;
    let len_sq = c * c + d * d;
    let mut param = -1.0;
    if len_sq != 0.0 {
        // in case of 0 length line
        param = dot / len_sq;
    }

    let xx: f32;
    let yy: f32;

    if param < 0.0 {
        xx = x1;
        yy = y1;
    } else if param > 1.0 {
        xx = x2;
        yy = y2;
    } else {
        xx = x1 + param * c;
        yy = y1 + param * d;
    }

    let dx = x - xx;
    let dy = y - yy;
    ((dx * dx + dy * dy) as f32).sqrt()
}
