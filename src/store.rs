use sycamore::reactive::{Modify, RcSignal};

use crate::model::{widget_kind::WidgetKind, AppData};

#[derive(Debug, Clone)]
pub struct ExportConfig {
    pub background: bool,
    pub visible_area_only: bool,
    pub padding: u32,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            background: false,
            visible_area_only: true,
            padding: 10,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub selected_kind: RcSignal<WidgetKind>,
    pub export_config: RcSignal<ExportConfig>,
    pub app_data: RcSignal<AppData>,
}

impl AppState {
    pub fn set_selected_kind(&self, kind: WidgetKind) {
        self.selected_kind.set(kind);
    }

    pub fn set_selected_kind_default(&self) {
        self.selected_kind.set(WidgetKind::Selection)
    }

    pub fn delete_selected_elements(&self) {
        let mut app_data = self.get_data();
        app_data.delete_selected_elements();
        app_data.draw();
    }

    pub fn clear_selection_elements(&self) {
        let mut app_data = self.get_data();
        app_data.clean_selected_state();
        app_data.draw();
    }

    /**
     * 移动选中的元素
     */
    pub fn move_selected_elements(&self, offset_x: i32, offset_y: i32) {
        let mut app_data = self.get_data();
        app_data.move_selected_elements(offset_x, offset_y);
        app_data.draw();
    }

    pub fn get_data(&self) -> Modify<AppData> {
        let app_data = self.app_data.modify();
        app_data
    }
}
