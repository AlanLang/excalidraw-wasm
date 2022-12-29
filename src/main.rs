use lib::{
    painter::Painter,
    store::{AppState, ArrowDirection},
    view::{export::ExportTool, toolbar::Toolbar},
    widget::{create_widget, shape::Rect, WidgetKind},
};
use sycamore::prelude::*;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{FontFace, HtmlCanvasElement, KeyboardEvent, MouseEvent};

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    let _ = FontFace::new_with_str("Virgil".into(), "url(https://uploads.codesandbox.io/uploads/user/ed077012-e728-4a42-8395-cbd299149d62/AflB-FG_Virgil.ttf)")
        .unwrap()
        .load();

    sycamore::render(|ctx| view!(ctx, App()));
}

#[component]
fn App<'a, G: Html>(ctx: Scope<'a>) -> View<G> {
    let window = web_sys::window().expect("no global `window` exists");
    let window_width = window.inner_width().unwrap().as_f64().unwrap();
    let window_height = window.inner_height().unwrap().as_f64().unwrap();

    let canvas_ref: &NodeRef<G> = create_node_ref(ctx);
    let painter = Painter::new();

    let drawing_state = create_signal(ctx, (0, 0, 0));
    let is_dragging = create_signal(ctx, (false, 0, 0));

    let app_state = AppState {
        selected_kind: create_rc_signal(WidgetKind::Selection),
        elements: create_rc_signal(vec![]),
        export_config: create_rc_signal(Default::default()),
    };
    let app_state = provide_context(ctx, app_state);
    let is_mounted = create_signal(ctx, false);

    on_mount(ctx, || {
        let canvas: HtmlCanvasElement = canvas_ref.get::<DomNode>().unchecked_into();
        let canvas_ctx = canvas
            .get_context("2d")
            .expect("should get context")
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .expect("should cast to context");
        canvas_ctx.translate(0.5, 0.5).unwrap();

        let window = web_sys::window().expect("should have a window in this context");
        let app_state_cloned = app_state.clone();
        let handler = move |event: KeyboardEvent| {
            let step: i32 = event.shift_key().then(|| 10).unwrap_or(1);
            match event.key().as_str() {
                "Backspace" => app_state_cloned.delete_selected_elements(),
                "ArrowLeft" => app_state_cloned.move_selected_elements(ArrowDirection::Left, step),
                "ArrowRight" => {
                    app_state_cloned.move_selected_elements(ArrowDirection::Right, step)
                }
                "ArrowUp" => app_state_cloned.move_selected_elements(ArrowDirection::Up, step),
                "ArrowDown" => app_state_cloned.move_selected_elements(ArrowDirection::Down, step),
                _ => (),
            };
        };
        let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);

        window
            .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
            .unwrap();

        closure.forget();
        is_mounted.set(true);
    });

    create_effect(ctx, move || {
        if *is_mounted.get() {
            let elements = app_state.elements.get();
            painter.draw_elements(canvas_ref, elements);
        }
    });

    create_effect(ctx, move || {
        let window = web_sys::window().expect("should have a window in this context");
        let document = window.document().expect("should have a document on window");
        let (dragging, _, _) = *is_dragging.get();
        if dragging {
            document
                .document_element()
                .unwrap()
                .set_class_name("cursor-move");
        } else {
            document
                .document_element()
                .unwrap()
                .set_class_name("cursor-auto");
        }
    });

    view! (ctx,
        div {
            ExportTool()
            Toolbar()
            canvas(
                ref=canvas_ref,
                class="fixed top-15 left-0",
                width=window_width,
                height=window_height,
                id="canvas",
                on:mousedown= move |event|  {
                    let id = app_state.add_element();
                    let mouse_event = event.dyn_into::<MouseEvent>().unwrap();
                    let x = mouse_event.offset_x();
                    let y = mouse_event.offset_y();

                    if *app_state.selected_kind.get() == WidgetKind::Text {
                        let (rect, text) = get_text_info(canvas_ref,x,y);
                        if text == "" {
                            return;
                        }
                        app_state.update_element(id, rect, vec![text]);
                        app_state.set_element_selected(id, true);
                        return;
                    }
                    // tracing::info!("Mouse down at ({}, {})", x, y);

                    // 如果当前是选择模式，且鼠标在某个元素上，则准备进入拖动模式
                    if *app_state.selected_kind.get() == WidgetKind::Selection {
                        let elements = app_state.elements.get();
                        let point_in_some_element = elements.iter().any(|rc_element| {
                            let element = rc_element.get();
                            let rect = element.rect;
                            element.is_selected && rect.is_in_point(x,y)
                        });
                        if point_in_some_element {
                            is_dragging.set((true, x, y));
                        }
                    }

                    drawing_state.set((id, x, y));
                },
                on:mousemove= move |event| {
                    let (id, start_x, start_y) = *drawing_state.get();
                    let (dragging, d_x, d_y) = *is_dragging.get();
                    let mouse_event = event.dyn_into::<MouseEvent>().unwrap();
                    let x = mouse_event.offset_x();
                    let y = mouse_event.offset_y();

                    // 如果是拖动选中的组件
                    if dragging {
                        let offset_x = x - d_x;
                        let offset_y = y - d_y;
                        is_dragging.set((true, x, y));
                        app_state.move_elements(offset_x, offset_y);
                        return;
                    }


                    if id > 0 {

                        let widget = create_widget(*app_state.selected_kind.get(), Rect::new(start_x, start_y, x, y));
                        let config_string = widget.get_config();
                        if *app_state.selected_kind.get() == WidgetKind::Selection {
                            app_state.update_element(id, fix_rect(Rect::new(start_x, start_y, x, y)), config_string);
                        } else {
                            app_state.update_element(id, Rect::new(start_x, start_y, x, y), config_string);
                        }
                    }
                },
                on:mouseup= move |event| {
                    let (id, _, _) = *drawing_state.get();
                    drawing_state.set((0, 0, 0));
                    let mouse_event = event.dyn_into::<MouseEvent>().unwrap();
                    let x = mouse_event.offset_x();
                    let y = mouse_event.offset_y();
                    tracing::info!("Mouse up at ({}, {})", x, y);
                    let can_be_selected = *app_state.selected_kind.get() != WidgetKind::Selection;
                    app_state.set_element_selected(id, can_be_selected);
                    app_state.delete_selection_element();
                    app_state.set_selected_kind_default();
                    is_dragging.set((false, 0, 0));
                },
            )
        }
    )
}

pub fn get_text_info<G: Html>(canvas_ref: &NodeRef<G>, x: i32, y: i32) -> (Rect, String) {
    let canvas: HtmlCanvasElement = canvas_ref.get::<DomNode>().unchecked_into();
    let window = web_sys::window().expect("should have a window in this context");
    let ctx = canvas
        .get_context("2d")
        .expect("should get context")
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("should cast to context");
    let text = window
        .prompt_with_message("What text do you want?")
        .unwrap();
    let text = text.unwrap();
    let text_measure = ctx.measure_text(&text).unwrap();

    let height = text_measure.font_bounding_box_ascent() + text_measure.font_bounding_box_descent();
    let width = text_measure.width();
    let rect = Rect {
        start_x: x,
        start_y: y,
        end_x: x + width as i32,
        end_y: y + height as i32,
    };
    (rect, text)
}

fn fix_rect(rect: Rect) -> Rect {
    let start_x = rect.start_x.min(rect.end_x);
    let start_y = rect.start_y.min(rect.end_y);
    let end_x = rect.start_x.max(rect.end_x);
    let end_y = rect.start_y.max(rect.end_y);
    Rect {
        start_x,
        start_y,
        end_x,
        end_y,
    }
}
