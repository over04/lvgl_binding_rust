use rust_lvgl_base::layout::LvObjLayout;
use rust_lvgl_base::obj::LvObjPtr;
use rust_lvgl_base::typing::flex::{FlexAlign, FlexFlow};
use rust_lvgl_sys::{
    lv_layout_t_LV_LAYOUT_FLEX, lv_obj_set_flex_align, lv_obj_set_flex_flow, lv_obj_set_layout,
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

impl FlexLayout<'_> {
    pub fn set_flow(&mut self, flow: FlexFlow) -> &mut Self {
        unsafe { lv_obj_set_flex_flow(self.as_mut(), flow as _) }
        self
    }

    pub fn set_align(
        &mut self,
        main_place: FlexAlign,
        cross_place: FlexAlign,
        track_cross_place: FlexAlign,
    ) -> &mut Self {
        unsafe {
            lv_obj_set_flex_align(
                self.as_mut(),
                main_place as _,
                cross_place as _,
                track_cross_place as _,
            )
        }
        self
    }
}
