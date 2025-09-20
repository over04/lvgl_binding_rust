use rust_lvgl_base::obj::{LvObjCreator, LvObjPtr};
use rust_lvgl_macro::lvgl_obj;
use rust_lvgl_sys::{
    lv_slider_create, lv_slider_get_value, lv_slider_set_range, lv_slider_set_value,
};

#[lvgl_obj]
pub struct Slider {}

impl LvObjCreator for Slider {
    fn create(parent: &dyn rust_lvgl_base::obj::LvObjPtr) -> Self {
        Self {
            _lv_obj_ptr: unsafe { lv_slider_create(parent.as_ptr()) },
        }
    }
}

impl Slider {
    pub fn set_value(&mut self, val: i32, anim: bool) -> &mut Self {
        unsafe {
            lv_slider_set_value(self.as_mut(), val, anim);
        }
        self
    }

    pub fn get_value(&self) -> i32 {
        unsafe { lv_slider_get_value(self.as_ptr()) }
    }

    pub fn set_range(&mut self, min: i32, max: i32) -> &mut Self {
        unsafe {
            lv_slider_set_range(self.as_mut(), min, max);
        }
        self
    }
}
