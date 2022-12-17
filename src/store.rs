use std::sync::atomic::AtomicU32;

use sycamore::reactive::{create_rc_signal, RcSignal};

use crate::widget::{shape::Rect, WidgetKind};

static NEXT_ELEMENT_ID: std::sync::atomic::AtomicU32 = AtomicU32::new(1);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArrowDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone)]
pub struct Element {
    pub id: u32,
    pub is_selected: bool,
    pub kind: WidgetKind,
    pub rect: Rect,
    pub shape_string: RcSignal<Vec<String>>,
}

#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub selected_kind: RcSignal<WidgetKind>,
    pub elements: RcSignal<Vec<RcSignal<Element>>>,
}

impl AppState {
    pub fn set_selected_kind(&self, kind: WidgetKind) {
        self.selected_kind.set(kind);
    }

    pub fn set_selected_kind_default(&self) {
        self.selected_kind.set(WidgetKind::Selection)
    }

    pub fn add_element(&self) -> u32 {
        let id = NEXT_ELEMENT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.elements.modify().push(create_rc_signal(Element {
            id: id,
            is_selected: false,
            kind: *self.selected_kind.get(),
            rect: Rect::new(0, 0, 0, 0),
            shape_string: create_rc_signal(vec![]),
        }));
        id
    }

    pub fn update_element(&self, id: u32, rect: Rect, shape_string: Vec<String>) {
        let elements = self.elements.get();
        let index = elements
            .iter()
            .position(|element| element.get().id == id)
            .unwrap();
        let element = elements[index].get();
        elements[index].modify().rect = rect;
        elements[index].modify().shape_string.set(shape_string);

        if element.kind == WidgetKind::Selection {
            elements.iter().for_each(|re_element| {
                let element = re_element.get();
                if element.rect.is_inside(rect) {
                    re_element.modify().is_selected = true;
                } else {
                    re_element.modify().is_selected = false;
                }
            });
        }
    }

    pub fn delete_selection_element(&self) {
        self.elements
            .modify()
            .retain(|element| element.get().kind != WidgetKind::Selection);
    }

    pub fn delete_selected_elements(&self) {
        self.elements
            .modify()
            .retain(|element| !element.get().is_selected);
    }

    pub fn clear_selection_elements(&self) {
        let elements = self.elements.get();
        elements.iter().for_each(|re_element| {
            re_element.modify().is_selected = false;
        })
    }

    pub fn move_selected_elements(&self, arrow: ArrowDirection, step: i32) {
        tracing::info!("Moving selected elements");
        let elements = self.elements.get();
        elements.iter().for_each(|re_element| {
            let element = re_element.get();
            if element.is_selected {
                match arrow {
                    ArrowDirection::Left => {
                        re_element.modify().rect.start_x -= step;
                        re_element.modify().rect.end_x -= step;
                    }
                    ArrowDirection::Right => {
                        re_element.modify().rect.start_x += step;
                        re_element.modify().rect.end_x += step;
                    }
                    ArrowDirection::Up => {
                        re_element.modify().rect.start_y -= step;
                        re_element.modify().rect.end_y -= step;
                    }
                    ArrowDirection::Down => {
                        re_element.modify().rect.start_y += step;
                        re_element.modify().rect.end_y += step;
                    }
                }
            }
        });
    }
}
