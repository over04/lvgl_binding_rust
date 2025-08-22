use alloc::ffi::CString;
use rust_lvgl_base::obj::{LvObjCreator, LvObjPtr};
use rust_lvgl_macro::lvgl_obj;
use rust_lvgl_sys::{lv_label_create, lv_label_set_text};

#[lvgl_obj]
pub struct Label {}

impl LvObjCreator for Label {
    fn create(parent: &dyn LvObjPtr) -> Self {
        Self {
            _lv_obj_ptr: unsafe { lv_label_create(parent.as_ptr()) },
        }
    }
}

impl Label {
    pub fn set_text(&mut self, text: &str) -> &mut Self {
        unsafe {
            let text = CString::new(text).unwrap();
            lv_label_set_text(self.as_mut(), text.as_ptr());
        }
        self
    }
}
