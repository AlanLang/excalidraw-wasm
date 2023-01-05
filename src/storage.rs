use crate::model::AppData;

const KEY: &str = "excalidraw-clipboard";

pub fn save_data(app_data: &AppData) {
    let local_storage = web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .expect("user has not enabled localStorage");
    local_storage
        .set_item(KEY, &serde_json::to_string(app_data).unwrap())
        .unwrap();
}

pub fn read_data() -> Option<AppData> {
    let local_storage = web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .expect("user has not enabled localStorage");
    local_storage
        .get_item(KEY)
        .unwrap()
        .map(|data| serde_json::from_str(&data).unwrap())
}
