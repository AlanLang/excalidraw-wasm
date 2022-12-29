use sycamore::prelude::*;

use crate::store::AppState;

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
            on:click=|_| {
                tracing::info!("export to png: {:?}", app_state.export_config.get());
            },
        ) {
            "Export to png"
        }
        label(class="mx-2") {
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
        label(class="mx-2") {
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
        label(class="mx-2") {
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
