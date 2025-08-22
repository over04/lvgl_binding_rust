use rust_lvgl_base::obj::{LvObjCreator, LvObjPtr};
use rust_lvgl_macro::lvgl_obj;
use rust_lvgl_sys::lv_obj_create;

#[lvgl_obj]
pub struct Obj {}

impl LvObjCreator for Obj {
    fn create(parent: &dyn LvObjPtr) -> Self {
        Self {
            _lv_obj_ptr: unsafe { lv_obj_create(parent.as_ptr()) },
        }
    }
}
