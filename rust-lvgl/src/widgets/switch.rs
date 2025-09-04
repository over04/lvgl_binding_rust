use rust_lvgl_base::{
    obj::{LvObjCreator, LvObjPtr},
    typing::state::State,
};
use rust_lvgl_macro::lvgl_obj;
use rust_lvgl_sys::{lv_obj_has_state, lv_obj_set_state, lv_switch_create};

#[lvgl_obj]
pub struct Switch {}

impl LvObjCreator for Switch {
    fn create(parent: &dyn LvObjPtr) -> Self {
        Self {
            _lv_obj_ptr: unsafe { lv_switch_create(parent.as_ptr()) },
        }
    }
}

impl Switch {
    pub fn get_checked(&self) -> bool {
        unsafe { lv_obj_has_state(self.as_ptr(), State::Checked as _) }
    }

    pub fn set_checked(&mut self, cheked: bool) -> &mut Self {
        unsafe {
            lv_obj_set_state(self.as_mut(), State::Checked as _, cheked);
        }
        self
    }
}
