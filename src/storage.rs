use crate::model::{element::Element, AppData};

const EXCALIDRAW_CLIPBOARD: &str = "excalidraw-clipboard";
const EXCALIDRAW_DATA: &str = "excalidraw-data";

pub fn save_data(app_data: &AppData) {
    let local_storage = web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .expect("user has not enabled localStorage");
    local_storage
        .set_item(EXCALIDRAW_DATA, &serde_json::to_string(app_data).unwrap())
        .unwrap();
}

pub fn read_data() -> Option<AppData> {
    let window = web_sys::window().expect("no global `window` exists");
    let local_storage = window
        .local_storage()
        .expect("should have a local storage")
        .expect("should have a local storage");
    let data = local_storage
        .get_item(EXCALIDRAW_DATA)
        .expect("should get item")
        .unwrap_or_default();
    if data.is_empty() {
        None
    } else {
        serde_json::from_str(&data).expect("should deserialize")
    }
}

pub fn save_elements_to_clipboard(elements: &Vec<&Element>) {
    let local_storage = web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .expect("user has not enabled localStorage");
    local_storage
        .set_item(
            EXCALIDRAW_CLIPBOARD,
            &serde_json::to_string(elements).unwrap(),
        )
        .unwrap();
}

pub fn read_elements_from_clipboard() -> Option<Vec<Element>> {
    let window = web_sys::window().expect("no global `window` exists");
    let local_storage = window
        .local_storage()
        .expect("should have a local storage")
        .expect("should have a local storage");
    let data = local_storage
        .get_item(EXCALIDRAW_CLIPBOARD)
        .expect("should get item")
        .unwrap_or_default();
    if data.is_empty() {
        None
    } else {
        serde_json::from_str(&data).expect("should deserialize")
    }
}
