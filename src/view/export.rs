use crate::store::AppState;
use sycamore::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

#[component]
pub fn ExportTool<G: Html>(ctx: Scope) -> View<G> {
    let app_state = use_context::<AppState>(ctx);

    let input_value = create_signal(ctx, app_state.export_config.get().padding.to_string());

    create_effect(ctx, move || {
        let mut export_config = app_state.export_config.modify();
        export_config.padding = input_value.get().parse().unwrap_or(0);
    });

    view!(ctx, div(class="exportWrapper") {
        button(
            class="bg-blue-500 hover:bg-blue-700 text-white  py-1 px-1 rounded mx-2",
            on:click=move |_| {
                tracing::info!("export to png: {:?}", app_state.export_config.get());
                app_state.clear_selection_elements();
                export_as_png(&app_state);
            },
        ) {
            "Export to png"
        }
        label(class="mx-2 cursor-pointer select-none") {
            input(
                type="checkbox",
                checked=app_state.export_config.get().background,
                on:click=move |_| {
                    let mut export_config = app_state.export_config.modify();
                    export_config.background = !export_config.background;
                },
            )
            "background"
        }
        label(class="mx-2 cursor-pointer select-none") {
            input(
                type="checkbox",
                checked=app_state.export_config.get().visible_area_only,
                on:click=move |_| {
                    let mut export_config = app_state.export_config.modify();
                    export_config.visible_area_only = !export_config.visible_area_only;
                },
            )
            "visible area only"
        }
        label(class="mx-2 select-none") {
            "(padding:"
            input(
                class="border w-16",
                type="number",
                bind:value=input_value,
            )
            "px)"
        }
    })
}

fn export_as_png(app_state: &AppState) {
    let app_data = app_state.app_data.get();
    let export_config = app_state.export_config.get();
    let view_bg_color = app_state.view_bg_color.get();

    let mut sub_canvas_x1 = i32::MAX;
    let mut sub_canvas_x2 = 0;
    let mut sub_canvas_y1 = i32::MAX;
    let mut sub_canvas_y2 = 0;

    app_data.elements.iter().for_each(|element| {
        sub_canvas_x1 = sub_canvas_x1.min(element.rect.start_x.min(element.rect.end_x));
        sub_canvas_x2 = sub_canvas_x2.max(element.rect.start_x.max(element.rect.end_x));
        sub_canvas_y1 = sub_canvas_y1.min(element.rect.start_y.min(element.rect.end_y));
        sub_canvas_y2 = sub_canvas_y2.max(element.rect.start_y.max(element.rect.end_y));
    });

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let main_canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("should cast to canvas");

    if export_config.visible_area_only {
        let canvas = document
            .create_element("canvas")
            .expect("should create canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("should cast to canvas");
        canvas.set_attribute("style", "display: none").unwrap();
        let _ = document.body().unwrap().append_child(&canvas);
        let width = sub_canvas_x2 - sub_canvas_x1 + (export_config.padding * 2) as i32;
        let height = sub_canvas_y2 - sub_canvas_y1 + (export_config.padding * 2) as i32;
        let padding = export_config.padding as i32;

        canvas
            .set_attribute("width", format!("{}px", width).as_str())
            .unwrap();
        canvas
            .set_attribute("height", format!("{}px", height).as_str())
            .unwrap();
        let canvas_ctx = canvas
            .get_context("2d")
            .expect("should get context")
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .expect("should cast to context");

        if export_config.background {
            canvas_ctx.set_fill_style(&JsValue::from_str(view_bg_color.as_str()));
            canvas_ctx.fill_rect(0.0, 0.0, width as f64, height as f64);
        }

        let _ = canvas_ctx
            .draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &main_canvas,
                (sub_canvas_x1 - padding).into(),
                (sub_canvas_y1 - padding).into(),
                width.into(),
                height.into(),
                0.0,
                0.0,
                width.into(),
                height.into(),
            );
        export_png_file(canvas.to_data_url().unwrap().as_str());
        canvas.remove();
    } else {
        export_png_file(main_canvas.to_data_url().unwrap().as_str());
    }
}

fn export_png_file(url: &str) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let link = document
        .create_element("a")
        .expect("should create a")
        .dyn_into::<web_sys::HtmlAnchorElement>()
        .expect("should cast to a");
    link.set_attribute("download", "export.png").unwrap();
    link.set_attribute("href", url).unwrap();
    link.click();
    link.remove();
}
