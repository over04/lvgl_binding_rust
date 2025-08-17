use lvgl_base::layout::{LvObjLayout, LvObjLayoutPad};
use lvgl_base::obj::LvObjPtr;
use lvgl_sys::{lv_layout_t_LV_LAYOUT_GRID, lv_obj_set_layout, lv_obj_t};

pub struct GridLayout<'a> {
    obj: &'a mut dyn LvObjPtr,
}

impl LvObjPtr for GridLayout<'_> {
    fn as_ptr(&self) -> *mut lv_obj_t {
        self.obj.as_ptr()
    }

    fn as_mut(&mut self) -> *mut lv_obj_t {
        self.obj.as_mut()
    }
}

impl<'a> LvObjLayout<'a> for GridLayout<'a> {
    fn create(obj: &'a mut dyn LvObjPtr) -> Self {
        unsafe { lv_obj_set_layout(obj.as_mut(), lv_layout_t_LV_LAYOUT_GRID) }
        Self { obj }
    }
}

impl<'a> LvObjLayoutPad<'a> for GridLayout<'a> {}
