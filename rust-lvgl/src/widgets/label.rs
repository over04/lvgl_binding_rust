use alloc::ffi::CString;
use rust_lvgl_base::obj::{LvObjCreator, LvObjPtr};
use rust_lvgl_base::typing::color::Color;
use rust_lvgl_base::typing::style::StyleSelector;
use rust_lvgl_macro::lvgl_obj;
use rust_lvgl_sys::{lv_label_create, lv_label_set_text, lv_obj_set_style_text_color};

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
    pub fn set_style_text_color(&mut self, color: Color, style: StyleSelector) -> &mut Self {
        unsafe {
            lv_obj_set_style_text_color(self.as_mut(), color.val(), style.val)
        }
        self
    }
}
