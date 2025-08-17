use crate::obj::LvObjPtr;
use crate::typing::style::StyleSelector;
use lvgl_sys::{
    lv_obj_set_style_pad_bottom, lv_obj_set_style_pad_column, lv_obj_set_style_pad_left,
    lv_obj_set_style_pad_right, lv_obj_set_style_pad_row, lv_obj_set_style_pad_top,
};

pub trait LvObjLayout<'a>
where
    Self: LvObjPtr,
{
    fn create(obj: &'a mut dyn LvObjPtr) -> Self;
}

pub trait LvObjLayoutPad<'a>
where
    Self: LvObjLayout<'a>,
{
    fn set_pad_column(&'a mut self, val: i32, selector: StyleSelector) -> &'a mut Self {
        unsafe { lv_obj_set_style_pad_column(self.as_mut(), val, selector.val) }
        self
    }

    fn set_pad_row(&'a mut self, val: i32, selector: StyleSelector) -> &'a mut Self {
        unsafe { lv_obj_set_style_pad_row(self.as_mut(), val, selector.val) }
        self
    }

    fn set_pad_left(&'a mut self, val: i32, selector: StyleSelector) -> &'a mut Self {
        unsafe { lv_obj_set_style_pad_left(self.as_mut(), val, selector.val) }
        self
    }

    fn set_pad_right(&'a mut self, val: i32, selector: StyleSelector) -> &'a mut Self {
        unsafe { lv_obj_set_style_pad_right(self.as_mut(), val, selector.val) }
        self
    }

    fn set_pad_top(&'a mut self, val: i32, selector: StyleSelector) -> &'a mut Self {
        unsafe { lv_obj_set_style_pad_top(self.as_mut(), val, selector.val) }
        self
    }

    fn set_pad_bottom(&'a mut self, val: i32, selector: StyleSelector) -> &'a mut Self {
        unsafe { lv_obj_set_style_pad_bottom(self.as_mut(), val, selector.val) }
        self
    }
}
