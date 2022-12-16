use sycamore::prelude::*;

use crate::{store::AppState, widget::WidgetKind};

#[component]
pub fn Toolbar<G: Html>(ctx: Scope) -> View<G> {
    let app_state = use_context::<AppState>(ctx);
    let value = *app_state.selected_kind.get();

    let on_selected = |kind: WidgetKind| {
        tracing::info!("kind down at ({:?})", kind);
        app_state.set_selected_kind(kind);
    };
    // TODO：使用迭代器来减少下面的重复代码
    view! (ctx,
        div {
            ElementOption(
                text=WidgetKind::Rectangle.to_string(),
                kind=WidgetKind::Rectangle,
                on_click= Box::new(on_selected),
                checked = value == WidgetKind::Rectangle
            )
            ElementOption(
                text=WidgetKind::Ellipse.to_string(),
                kind=WidgetKind::Ellipse,
                on_click= Box::new(on_selected),
                checked = value == WidgetKind::Ellipse
            )
            ElementOption(
                text=WidgetKind::Arrow.to_string(),
                kind=WidgetKind::Arrow,
                on_click= Box::new(on_selected),
                checked = value == WidgetKind::Arrow
            )
            ElementOption(
                text=WidgetKind::Text.to_string(),
                kind=WidgetKind::Text,
                on_click= Box::new(on_selected),
                checked = value == WidgetKind::Text
            )
            ElementOption(
                text=WidgetKind::Selection.to_string(),
                kind=WidgetKind::Selection,
                on_click= Box::new(on_selected),
                checked = value == WidgetKind::Selection
            )
        }
    )
}

#[derive(Prop)]
struct ElementOptionProps<'a> {
    checked: bool,
    text: String,
    kind: WidgetKind,
    on_click: Box<dyn Fn(WidgetKind) + 'a>,
}

#[component]
fn ElementOption<'a, G: Html>(ctx: Scope<'a>, props: ElementOptionProps<'a>) -> View<G> {
    let on_click = props.on_click;
    view! (ctx,
        label(class="cursor-pointer") {
            input(
                class="cursor-pointer",
                type="radio",
                name="element",
                value="circle",
                on:click=move |_| on_click(props.kind),
                checked=props.checked
            )
            (props.text)
        }
    )
}
