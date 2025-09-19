use std::ffi::{CStr, CString};

use rust_lvgl_base::obj::LvObjPtr;
use rust_lvgl_macro::lvgl_obj;
use rust_lvgl_sys::{
    lv_textarea_add_char, lv_textarea_add_text, lv_textarea_create, lv_textarea_get_text,
    lv_textarea_set_one_line, lv_textarea_set_password_mode, lv_textarea_set_text,
};

#[lvgl_obj]
pub struct TextArea {
    /// 分配一块内存空间给text存储
    text: CString,
}

impl TextArea {
    pub fn create(parent: &dyn LvObjPtr) -> Self {
        let mut this = Self {
            _lv_obj_ptr: unsafe { lv_textarea_create(parent.as_ptr()) },
            text: CString::new("").unwrap(),
        };
        unsafe { lv_textarea_set_text(this.as_mut(), this.text.as_ptr()) };
        this
    }

    pub fn set_password_mode(&mut self, enable: bool) -> &mut Self {
        unsafe {
            lv_textarea_set_password_mode(self.as_mut(), enable);
        }
        self
    }

    pub fn set_text(&mut self, text: &str) -> &mut Self {
        let text = CString::new(text).unwrap();
        unsafe {
            lv_textarea_set_text(self.as_mut(), text.as_ptr());
        }
        self.text = text;
        self
    }

    pub fn get_text(&self) -> String {
        unsafe {
            CStr::from_ptr(lv_textarea_get_text(self.as_ptr()))
                .to_str()
                .unwrap_or_default()
                .to_string()
        }
    }

    pub fn add_char(&mut self, ch: char) -> &mut Self {
        unsafe {
            lv_textarea_add_char(self.as_mut(), ch as _);
        }
        self
    }

    pub fn add_text(&mut self, text: &str) -> &mut Self {
        unsafe {
            let text = CString::new(text).unwrap();
            lv_textarea_add_text(self.as_mut(), text.as_ptr());
        }
        self
    }

    pub fn set_one_line(&mut self, enable: bool) -> &mut Self {
        unsafe {
            lv_textarea_set_one_line(self.as_mut(), enable);
        }
        self
    }
}
