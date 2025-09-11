use rust_lvgl_base::obj::{LvObj, LvObjPtr};
use rust_lvgl_macro::lvgl_obj;
use rust_lvgl_sys::{lv_keyboard_create, lv_keyboard_get_textarea, lv_keyboard_set_textarea};

use crate::widgets::textarea::TextArea;

#[lvgl_obj]
pub struct Keyboard {}

impl Keyboard {
    pub fn create(parent: &dyn LvObjPtr) -> Self {
        Self {
            _lv_obj_ptr: unsafe { lv_keyboard_create(parent.as_ptr()) },
        }
    }

    pub fn set_textarea(&mut self, textarea: &TextArea) -> &mut Self {
        unsafe { lv_keyboard_set_textarea(self.as_mut(), textarea.as_ptr()) };
        self
    }

    pub fn get_textarea(&self) -> Option<TextArea> {
        let ptr = unsafe { lv_keyboard_get_textarea(self.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(TextArea::from_raw(ptr))
        }
    }
}
