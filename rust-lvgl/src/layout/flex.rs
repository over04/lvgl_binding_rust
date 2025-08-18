use rust_lvgl_base::layout::{LvObjLayout, LvObjLayoutPad};
use rust_lvgl_base::obj::LvObjPtr;
use rust_lvgl_base::typing::flex::FlexFlow;
use rust_lvgl_sys::{
    lv_layout_t_LV_LAYOUT_FLEX, lv_obj_set_flex_flow, lv_obj_set_flex_grow, lv_obj_set_layout,
    lv_obj_t,
};

pub struct FlexLayout<'a> {
    obj: &'a mut dyn LvObjPtr,
}

impl LvObjPtr for FlexLayout<'_> {
    fn as_ptr(&self) -> *mut lv_obj_t {
        self.obj.as_ptr()
    }

    fn as_mut(&mut self) -> *mut lv_obj_t {
        self.obj.as_mut()
    }
}

impl<'a> LvObjLayout<'a> for FlexLayout<'a> {
    fn create(obj: &'a mut dyn LvObjPtr) -> Self {
        unsafe { lv_obj_set_layout(obj.as_mut(), lv_layout_t_LV_LAYOUT_FLEX) }
        Self { obj }
    }
}

impl<'a> LvObjLayoutPad<'a> for FlexLayout<'a> {}

impl<'a> FlexLayout<'a> {
    pub fn set_flow(&mut self, flow: FlexFlow) -> &mut Self {
        unsafe {
            lv_obj_set_flex_flow(self.obj.as_mut(), flow as _);
        }
        self
    }

    pub fn set_grow(&mut self, grow: u8) -> &mut Self {
        unsafe {
            lv_obj_set_flex_grow(self.obj.as_mut(), grow as _);
        }
        self
    }
}
