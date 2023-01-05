use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::KeyboardEvent;

pub fn add_event_listener(key: &str, handler: impl FnMut(KeyboardEvent) + 'static) {
    let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    let window = web_sys::window().expect("should have a window in this context");
    window
        .add_event_listener_with_callback(key, closure.as_ref().unchecked_ref())
        .unwrap();

    closure.forget();
}
