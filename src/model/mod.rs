use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;

use crate::{draw_scene::draw_scene, utils::hit_test::hit_test};

use self::{element::Element, rect::Rect, widget_kind::WidgetKind};

pub mod element;
pub mod rect;
pub mod widget_kind;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AppData {
    pub elements: Vec<Element>,
}

impl AppData {
    pub fn add_element(&mut self, element: Element) {
        self.elements.push(element);
    }

    pub fn create_element(&mut self, kind: WidgetKind) -> &Element {
        let element = Element::new(kind);
        self.elements.push(element);
        self.elements.last_mut().unwrap()
    }

    pub fn get_element_mut(&mut self, id: u32) -> Option<&mut Element> {
        self.elements.iter_mut().find(|e| e.id == id)
    }

    pub fn get_element_by_point_mut(&mut self, x: i32, y: i32) -> Option<&mut Element> {
        self.elements
            .iter_mut()
            .find(|element| hit_test(element, x, y))
    }

    pub fn get_element_by_point(&self, x: i32, y: i32) -> Option<&Element> {
        self.elements
            .iter()
            .find(|element| element.rect.is_in_point(x, y))
    }

    pub fn select_elements(&mut self, rect: Rect) {
        self.elements
            .iter_mut()
            .for_each(|element| element.set_selected(element.rect.is_inside(rect)));
    }

    pub fn clean(&mut self) {
        self.elements
            .retain(|element| element.kind != WidgetKind::Selection);
    }

    pub fn clean_selected_state(&mut self) {
        self.elements
            .iter_mut()
            .for_each(|element| element.set_selected(false));
    }

    pub fn select_element(&mut self, id: u32, add: bool) {
        self.elements.iter_mut().for_each(|element| {
            let need_select: bool = if add {
                element.id == id || element.is_selected
            } else {
                element.id == id
            };
            element.set_selected(need_select)
        });
    }

    pub fn move_selected_elements(&mut self, offset_x: i32, offset_y: i32) {
        self.elements
            .iter_mut()
            .filter(|element| element.is_selected)
            .for_each(|element| element.move_element(offset_x, offset_y));
    }

    pub fn move_all_elements(&mut self, offset_x: i32, offset_y: i32) {
        self.elements
            .iter_mut()
            .for_each(|element| element.move_element(offset_x, offset_y));
    }

    pub fn delete_selected_elements(&mut self) {
        self.elements.retain(|element| !element.is_selected);
    }

    pub fn select_all_elements(&mut self) {
        self.elements
            .iter_mut()
            .for_each(|element| element.set_selected(true));
    }

    pub fn draw(&self) {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let main_canvas = document
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("should cast to canvas");
        draw_scene(main_canvas, self);
    }
}
