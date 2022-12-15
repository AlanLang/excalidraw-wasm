use lib::{
    painter::Painter,
    store::AppState,
    view::toolbar::Toolbar,
    widget::{create_widget, shape::Rect, WidgetKind},
};
use sycamore::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::MouseEvent;
fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    sycamore::render(|ctx| view!(ctx, App()));
}

#[component]
fn App<G: Html>(ctx: BoundedScope) -> View<G> {
    let window = web_sys::window().expect("no global `window` exists");
    let window_width = window.inner_width().unwrap().as_f64().unwrap();
    let window_height = window.inner_height().unwrap().as_f64().unwrap();

    let canvas_ref: &NodeRef<G> = create_node_ref(ctx);
    let painter = Painter::new();

    let drawing_state = create_signal(ctx, (0, 0, 0));

    let app_state = AppState {
        selected_kind: create_rc_signal(WidgetKind::Rectangle),
        elements: create_rc_signal(vec![]),
    };
    let app_state = provide_context(ctx, app_state);

    // on_mount(ctx, move || {
    //     painter.rectangle(100, 100, 200, 200);
    //     painter.ellipse(100, 100, 200, 200);
    // });

    create_effect(ctx, move || {
        let elements = app_state.elements.get();
        if elements.is_empty() {
            return;
        }
        painter.draw_elements(canvas_ref, elements);
    });

    view! (ctx,
        div {
            Toolbar()
            canvas(
                ref=canvas_ref,
                class="fixed top-10 left-0",
                width=window_width,
                height=window_height,
                id="canvas",
                on:mousedown= move |event|  {
                    painter.clear_canvas(canvas_ref);
                    let mouse_event = event.dyn_into::<MouseEvent>().unwrap();
                    let x = mouse_event.offset_x();
                    let y = mouse_event.offset_y();
                    // tracing::info!("Mouse down at ({}, {})", x, y);
                    let id = app_state.add_element();
                    drawing_state.set((id, x, y));
                },
                on:mousemove= move |event| {
                    let (id, start_x, start_y) = *drawing_state.get();
                    if id > 0 {
                        let mouse_event = event.dyn_into::<MouseEvent>().unwrap();
                        let x = mouse_event.offset_x();
                        let y = mouse_event.offset_y();
                        let widget = create_widget(*app_state.selected_kind.get(), Rect::new(start_x, start_y, x, y));
                        let config_string = widget.get_config(&painter);
                        app_state.update_element(id, Rect::new(start_x, start_y, x, y), config_string);
                    }
                },
                on:mouseup= move |event| {
                    drawing_state.set((0, 0, 0));
                    let mouse_event = event.dyn_into::<MouseEvent>().unwrap();
                    let x = mouse_event.offset_x();
                    let y = mouse_event.offset_y();
                    tracing::info!("Mouse up at ({}, {})", x, y);
                }
            )
        }
    )
}
