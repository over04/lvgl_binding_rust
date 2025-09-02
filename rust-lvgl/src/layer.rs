use rust_lvgl_base::{obj::LvObj, obj::Obj};
use rust_lvgl_sys::{lv_layer_bottom, lv_layer_sys, lv_layer_top, lv_screen_active};

pub struct LvObjLayer;

impl LvObjLayer {
    pub fn screen_active() -> Obj {
        unsafe { Obj::from_raw(lv_screen_active()) }
    }

    pub fn top() -> Obj {
        unsafe { Obj::from_raw(lv_layer_top()) }
    }

    pub fn bottom() -> Obj {
        unsafe { Obj::from_raw(lv_layer_bottom()) }
    }

    pub fn sys() -> Obj {
        unsafe { Obj::from_raw(lv_layer_sys()) }
    }
}
