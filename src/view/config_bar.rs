use crate::store::AppState;
use sycamore::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

#[component]
pub fn ConfigBar<G: Html>(ctx: Scope) -> View<G> {
    view!(ctx, div(class="config-bar") {
        label(class="mx-2 inline-flex items-center cursor-pointer select-none") {
            input(
                type="color",
            )
            "view background color"
        }
        label(class="mx-2 inline-flex items-center cursor-pointer select-none") {
            input(
                type="color",
            )
            "item stroke color"
        }
        label(class="mx-2 inline-flex items-center cursor-pointer select-none") {
            input(
                type="color",
            )
            "item background color"
        }
    })
}
