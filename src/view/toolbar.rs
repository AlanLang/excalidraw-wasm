use sycamore::prelude::*;

use crate::{model::widget_kind::WidgetKind, store::AppState};

#[component]
pub fn Toolbar<G: Html>(ctx: Scope) -> View<G> {
    // TODO：使用迭代器来减少下面的重复代码
    view! (ctx,
        div {
            ElementOption(
                text=WidgetKind::Rectangle.to_string(),
                kind=WidgetKind::Rectangle,
            )
            ElementOption(
                text=WidgetKind::Ellipse.to_string(),
                kind=WidgetKind::Ellipse,
            )
            ElementOption(
                text=WidgetKind::Arrow.to_string(),
                kind=WidgetKind::Arrow,
            )
            ElementOption(
                text=WidgetKind::Text.to_string(),
                kind=WidgetKind::Text,
            )
            ElementOption(
                text=WidgetKind::Selection.to_string(),
                kind=WidgetKind::Selection,
            )
        }
    )
}

#[derive(Prop)]
struct ElementOptionProps {
    text: String,
    kind: WidgetKind,
}

#[component]
fn ElementOption<G: Html>(ctx: Scope, props: ElementOptionProps) -> View<G> {
    let app_state = use_context::<AppState>(ctx);

    let on_selected = |kind: WidgetKind| {
        tracing::info!("kind down at ({:?})", kind);
        app_state.set_selected_kind(kind);
        app_state.clear_selection_elements();
    };

    let checked = create_signal(ctx, *app_state.selected_kind.get() == props.kind);
    create_effect(ctx, move || {
        checked.set(*app_state.selected_kind.get() == props.kind)
    });

    view! (ctx,
        label(class="cursor-pointer select-none") {
            input(
                class="cursor-pointer",
                type="radio",
                name="element",
                on:click=move |_| on_selected(props.kind),
                value=checked,
                bind:checked=checked
            )
            (props.text)
        }
    )
}
