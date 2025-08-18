use rust_lvgl_base::layout::LvObjLayout;
use rust_lvgl_base::obj::LvObjPtr;
use rust_lvgl_base::typing::align::Align;
use rust_lvgl_sys::{
    lv_layout_t_LV_LAYOUT_NONE, lv_obj_align, lv_obj_align_to, lv_obj_center, lv_obj_set_align,
    lv_obj_set_layout, lv_obj_set_pos, lv_obj_set_x, lv_obj_set_y, lv_obj_t,
};

pub struct DefaultLayout<'a> {
    obj: &'a mut dyn LvObjPtr,
}

impl LvObjPtr for DefaultLayout<'_> {
    fn as_ptr(&self) -> *mut lv_obj_t {
        self.obj.as_ptr()
    }

    fn as_mut(&mut self) -> *mut lv_obj_t {
        self.obj.as_mut()
    }
}

impl<'a> LvObjLayout<'a> for DefaultLayout<'a> {
    fn create(obj: &'a mut dyn LvObjPtr) -> Self {
        unsafe { lv_obj_set_layout(obj.as_mut(), lv_layout_t_LV_LAYOUT_NONE) }
        Self { obj }
    }
}

impl<'a> DefaultLayout<'a> {
    pub fn center(&mut self) -> &mut Self {
        unsafe { lv_obj_center(self.obj.as_mut()) }
        self
    }

    pub fn set_align(&mut self, align: Align) -> &mut Self {
        unsafe { lv_obj_set_align(self.obj.as_mut(), align as _) }
        self
    }

    /// 相当于set_align + set_pos
    pub fn align(&mut self, align: Align, x_ofs: i32, y_ofs: i32) -> &mut Self {
        unsafe { lv_obj_align(self.obj.as_mut(), align as _, x_ofs, y_ofs) }
        self
    }

    pub fn align_to(
        &mut self,
        base: &dyn LvObjPtr,
        align: Align,
        x_ofs: i32,
        y_ofs: i32,
    ) -> &mut Self {
        unsafe { lv_obj_align_to(self.obj.as_mut(), base.as_ptr(), align as _, x_ofs, y_ofs) }
        self
    }

    fn set_x(&mut self, x: i32) -> &mut Self {
        unsafe {
            lv_obj_set_x(self.obj.as_mut(), x);
        }
        self
    }
    fn set_y(&mut self, y: i32) -> &mut Self {
        unsafe {
            lv_obj_set_y(self.obj.as_mut(), y);
        }
        self
    }

    fn set_pos(&mut self, x: i32, y: i32) -> &mut Self {
        unsafe {
            lv_obj_set_pos(self.obj.as_mut(), x, y);
        }
        self
    }
}
