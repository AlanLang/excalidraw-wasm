use crate::store::AppState;
use sycamore::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

#[component]
pub fn ConfigBar<G: Html>(ctx: Scope) -> View<G> {
    let app_state = use_context::<AppState>(ctx);

    view!(ctx, div(class="config-bar") {
        label(class="mx-2 inline-flex items-center cursor-pointer select-none") {
            input(
                type="color",
                bind:value=app_state.view_bg_color,
            )
            "view background color"
        }
        label(class="mx-2 inline-flex items-center cursor-pointer select-none") {
            input(
                type="color",
                bind:value=app_state.item_stroke_color,
            )
            "item stroke color"
        }
        label(class="mx-2 inline-flex items-center cursor-pointer select-none") {
            input(
                type="color",
                bind:value=app_state.item_bg_color,
            )
            "item background color"
        }
    })
}
