use rust_lvgl_base::obj::{LvObj, LvObjCreator, LvObjPtr};
use rust_lvgl_macro::lvgl_obj;
use rust_lvgl_sys::{
    lv_ime_pinyin_create, lv_ime_pinyin_get_cand_panel, lv_ime_pinyin_set_keyboard,
};

use crate::widgets::keyboard::Keyboard;

#[lvgl_obj]
pub struct Pinyin {}

impl LvObjCreator for Pinyin {
    fn create(parent: &dyn LvObjPtr) -> Self {
        Self {
            _lv_obj_ptr: unsafe { lv_ime_pinyin_create(parent.as_ptr()) },
        }
    }
}

impl Pinyin {
    pub fn set_keyboard(&mut self, keyboard: &Keyboard) -> &mut Self {
        unsafe {
            lv_ime_pinyin_set_keyboard(self.as_mut(), keyboard.as_ptr());
        }
        self
    }

    pub fn get_cand_panel(&self) -> PinyinCandPanel {
        PinyinCandPanel::from_raw(unsafe { lv_ime_pinyin_get_cand_panel(self.as_ptr()) })
    }
}

#[lvgl_obj]
pub struct PinyinCandPanel {}
