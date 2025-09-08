use rust_lvgl_base::obj::LvObjCreator;
use rust_lvgl_macro::lvgl_obj;
use rust_lvgl_sys::lv_button_create;

#[lvgl_obj]
pub struct Button {}

impl LvObjCreator for Button {
    fn create(parent: &dyn rust_lvgl_base::obj::LvObjPtr) -> Self {
        Self {
            _lv_obj_ptr: unsafe { lv_button_create(parent.as_ptr()) },
        }
    }
}
