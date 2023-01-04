use lib::{
    model::{rect::Rect, widget_kind::WidgetKind, AppData},
    store::AppState,
    view::{export::ExportTool, toolbar::Toolbar},
    widget::create_widget,
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

    let drawing_state = create_signal(ctx, (0, 0, 0));
    let is_dragging = create_signal(ctx, (false, 0, 0));

    let app_state = AppState {
        selected_kind: create_rc_signal(WidgetKind::Selection),
        export_config: create_rc_signal(Default::default()),
        app_data: create_rc_signal(AppData::default()),
    };
    let app_state = provide_context(ctx, app_state);

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
                "ArrowLeft" => app_state_cloned.move_selected_elements(-step, 0),
                "ArrowRight" => app_state_cloned.move_selected_elements(step, 0),
                "ArrowUp" => app_state_cloned.move_selected_elements(0, -step),
                "ArrowDown" => app_state_cloned.move_selected_elements(0, step),
                _ => (),
            };
        };
        let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);

        window
            .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
            .unwrap();

        closure.forget();
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
                width=window_width,
                height=window_height,
                id="canvas",
                on:mousedown= move |event|  {
                    let mouse_event = event.dyn_into::<MouseEvent>().unwrap();
                    let x = mouse_event.offset_x();
                    let y = mouse_event.offset_y();
                    let selected_kind = *app_state.selected_kind.get();
                    let mut app_data = app_state.get_data();
                    let element = app_data.create_element(selected_kind);

                    let id = element.id;

                    if *app_state.selected_kind.get() == WidgetKind::Text {
                        let (rect, text) = get_text_info(canvas_ref,x,y);
                        if text == "" {
                            return;
                        }
                        if let Some(element) = app_data.get_element_mut(id) {
                            element.update_rect(rect.start_x, rect.start_y, rect.end_x, rect.end_y);
                            element.update_shape_string(vec![text]);
                            element.set_selected(true);
                        }
                        return;
                    }
                    // tracing::info!("Mouse down at ({}, {})", x, y);

                    // 如果当前是选择模式，且鼠标在某个元素上，则准备进入拖动模式
                    if *app_state.selected_kind.get() == WidgetKind::Selection {
                        let point_in_some_element = app_data.get_element_by_point(x,y);
                        if point_in_some_element.is_some() {
                            is_dragging.set((true, x, y));
                        }
                    } else {
                        app_data.clean_selected_state(); // 清理当前的选中状态
                    }

                    drawing_state.set((id, x, y));
                },
                on:mousemove= move |event| {
                    let (id, start_x, start_y) = *drawing_state.get();
                    let (dragging, d_x, d_y) = *is_dragging.get();
                    let mouse_event = event.dyn_into::<MouseEvent>().unwrap();
                    let x = mouse_event.offset_x();
                    let y = mouse_event.offset_y();
                    let mut app_data = app_state.get_data();

                    // 如果是拖动选中的组件
                    if dragging {
                        let offset_x = x - d_x;
                        let offset_y = y - d_y;
                        is_dragging.set((true, x, y));
                        app_data.move_selected_elements(offset_x, offset_y);
                        app_data.draw();
                        return;
                    }


                    if id > 0 {
                        let widget = create_widget(*app_state.selected_kind.get(), Rect::new(start_x, start_y, x, y));
                        let config_string = widget.get_config();
                        let rect = fix_rect(Rect::new(start_x, start_y, x, y));
                        if let Some(element) = app_data.get_element_mut(id) {
                            element.update_rect(rect.start_x, rect.start_y, rect.end_x, rect.end_y);
                            element.update_shape_string(config_string);
                            if *app_state.selected_kind.get() == WidgetKind::Selection {
                                app_data.select_elements(rect);
                            }
                        }
                        app_data.draw();
                        return;
                    }
                },
                on:mouseup= move |event| {
                    let (id, start_x, start_y) = *drawing_state.get();
                    let mouse_event = event.dyn_into::<MouseEvent>().unwrap();
                    let x = mouse_event.offset_x();
                    let y = mouse_event.offset_y();
                    let mut app_data = app_state.get_data();
                    app_data.clean();
                    let has_dragged = start_x != x || start_y != y;

                    // 如果是在绘制图形，则在绘制完毕后选中该图形
                    if *app_state.selected_kind.get() != WidgetKind::Selection {
                        app_data.clean_selected_state();
                        if let Some(element) = app_data.get_element_mut(id) {
                            element.set_selected(true);
                        }
                    }

                    if !has_dragged && *app_state.selected_kind.get() == WidgetKind::Selection {
                        if let Some(point_in_some_element) = app_data.get_element_by_point_mut(x,y) {
                            let id = point_in_some_element.id;
                            app_data.select_element(id);
                        } else {
                            app_data.clean_selected_state();
                        }
                    }

                    app_data.draw();

                    tracing::info!("Mouse up at ({}, {})", x, y);
                    app_state.set_selected_kind_default();
                    drawing_state.set((0, 0, 0));
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
