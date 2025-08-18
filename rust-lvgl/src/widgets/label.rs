use rust_lvgl_base::obj::LvObjPtr;
use rust_lvgl_macro::lvgl_obj;
use rust_lvgl_sys::lv_label_set_text;
use std::ffi::CString;

#[lvgl_obj]
pub struct Label {}

impl Label {
    pub fn set_text(&mut self, text: &str) -> &mut Self {
        unsafe {
            let text = CString::new(text).unwrap();
            lv_label_set_text(self.as_mut(), text.as_ptr());
        }
        self
    }
}
