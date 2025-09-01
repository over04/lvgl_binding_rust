use crate::layout::LvObjLayout;
use crate::typing::align::Align;
use crate::typing::color::{Color, Opacity};
use crate::typing::event::{
    EventCb, EventCbWithData, EventCode, EventData, event_handler_cb, event_handler_cb_with_data,
};
use crate::typing::flag::Flag;
use crate::typing::grid::GridAlign;
use crate::typing::scroll::{ScrollBarMode, ScrollSnap};
use crate::typing::size::Length;
use crate::typing::style::StyleSelector;
use alloc::boxed::Box;
use core::ffi::c_void;
use rust_lvgl_sys::{
    lv_obj_add_event_cb, lv_obj_add_flag, lv_obj_align, lv_obj_align_to, lv_obj_center,
    lv_obj_clean, lv_obj_flag_t, lv_obj_flag_t_LV_OBJ_FLAG_HIDDEN, lv_obj_get_child,
    lv_obj_get_height, lv_obj_get_width, lv_obj_get_x, lv_obj_get_y, lv_obj_move_to_index,
    lv_obj_remove_flag, lv_obj_remove_style_all, lv_obj_scroll_to_view, lv_obj_set_align,
    lv_obj_set_flag, lv_obj_set_flex_grow, lv_obj_set_grid_cell, lv_obj_set_height, lv_obj_set_pos,
    lv_obj_set_scroll_snap_x, lv_obj_set_scrollbar_mode, lv_obj_set_size,
    lv_obj_set_style_bg_color, lv_obj_set_style_bg_opa, lv_obj_set_style_opa,
    lv_obj_set_style_pad_bottom, lv_obj_set_style_pad_column, lv_obj_set_style_pad_left,
    lv_obj_set_style_pad_right, lv_obj_set_style_pad_row, lv_obj_set_style_pad_top,
    lv_obj_set_style_transform_pivot_x, lv_obj_set_style_transform_pivot_y, lv_obj_set_width,
    lv_obj_set_x, lv_obj_set_y, lv_obj_t,
};

pub trait LvObjPtr {
    fn as_ptr(&self) -> *mut lv_obj_t;
    fn as_mut(&mut self) -> *mut lv_obj_t;
}

pub trait LvObjEvent
where
    Self: LvObjPtr,
{
    fn on_event(&mut self, code: EventCode, cb: EventCb) -> &mut Self {
        unsafe {
            lv_obj_add_event_cb(
                self.as_mut(),
                Some(event_handler_cb),
                code as _,
                cb as *mut c_void,
            );
        }
        self
    }
}

pub trait LvObjEventData
where
    Self: LvObjPtr,
{
    /// data的生命周期由用户管理
    unsafe fn on_event<T>(
        &mut self,
        code: EventCode,
        data: *mut T,
        cb: EventCbWithData<T>,
    ) -> &mut Self {
        unsafe {
            let data = Box::leak(Box::new(EventData { cb, data })) as *mut _ as *mut c_void;
            lv_obj_add_event_cb(
                self.as_mut(),
                Some(event_handler_cb_with_data::<T>),
                code as _,
                data,
            );
        }
        self
    }
}

pub trait LvObjCreator {
    fn create(parent: &dyn LvObjPtr) -> Self;
}

pub trait LvObj
where
    Self: LvObjPtr + LvObjEvent + LvObjEventData + LvObjCreator + Sized,
{
    fn from_raw(raw: *mut lv_obj_t) -> Self;
    // fn alter(self) -> LvObjAlter<Self> {
    //     LvObjAlter(self, Default::default())
    // }

    fn get_width(&self) -> i32 {
        unsafe { lv_obj_get_width(self.as_ptr()) }
    }

    fn set_width(&mut self, width: Length) -> &mut Self {
        unsafe {
            lv_obj_set_width(self.as_mut(), width.val());
        }
        self
    }

    fn set_height(&mut self, height: Length) -> &mut Self {
        unsafe {
            lv_obj_set_height(self.as_mut(), height.val());
        }
        self
    }

    fn get_height(&self) -> i32 {
        unsafe { lv_obj_get_height(self.as_ptr()) }
    }

    fn set_size(&mut self, width: Length, height: Length) -> &mut Self {
        unsafe {
            lv_obj_set_size(self.as_mut(), width.val(), height.val());
        }
        self
    }

    fn hidden(&mut self) -> &mut Self {
        unsafe {
            lv_obj_add_flag(self.as_mut(), lv_obj_flag_t_LV_OBJ_FLAG_HIDDEN);
        }
        self
    }

    fn display(&mut self) -> &mut Self {
        unsafe {
            lv_obj_remove_flag(self.as_mut(), lv_obj_flag_t_LV_OBJ_FLAG_HIDDEN);
        }
        self
    }

    fn add_flag(&mut self, flag: Flag) -> &mut Self {
        unsafe {
            lv_obj_add_flag(self.as_mut(), flag as lv_obj_flag_t);
        }
        self
    }

    fn remove_flag(&mut self, flag: Flag) -> &mut Self {
        unsafe {
            lv_obj_remove_flag(self.as_mut(), flag as lv_obj_flag_t);
        }
        self
    }

    fn set_flag(&mut self, flag: Flag, value: bool) -> &mut Self {
        unsafe {
            lv_obj_set_flag(self.as_mut(), flag as lv_obj_flag_t, value);
        }
        self
    }

    fn remove_style_all(&mut self) -> &mut Self {
        unsafe {
            lv_obj_remove_style_all(self.as_mut());
        }
        self
    }

    fn center(&mut self) -> &mut Self {
        unsafe { lv_obj_center(self.as_mut()) }
        self
    }

    fn set_align(&mut self, align: Align) -> &mut Self {
        unsafe { lv_obj_set_align(self.as_mut(), align as _) }
        self
    }

    /// 相当于set_align + set_pos
    fn align(&mut self, align: Align, x_ofs: i32, y_ofs: i32) -> &mut Self {
        unsafe { lv_obj_align(self.as_mut(), align as _, x_ofs, y_ofs) }
        self
    }

    fn align_to(&mut self, base: &dyn LvObjPtr, align: Align, x_ofs: i32, y_ofs: i32) -> &mut Self {
        unsafe { lv_obj_align_to(self.as_mut(), base.as_ptr(), align as _, x_ofs, y_ofs) }
        self
    }

    fn move_to_index(&mut self, idx: i32) -> &mut Self {
        unsafe {
            lv_obj_move_to_index(self.as_mut(), idx);
        }
        self
    }

    /// 清除此obj下所有的子元素
    fn clean(&mut self) -> &mut Self {
        unsafe {
            lv_obj_clean(self.as_mut());
        }
        self
    }

    fn get_x(&self) -> i32 {
        unsafe { lv_obj_get_x(self.as_ptr()) }
    }

    fn set_x(&mut self, x: Length) -> &mut Self {
        unsafe {
            lv_obj_set_x(self.as_mut(), x.val());
        }
        self
    }

    fn get_y(&self) -> i32 {
        unsafe { lv_obj_get_y(self.as_ptr()) }
    }

    fn set_y(&mut self, y: Length) -> &mut Self {
        unsafe {
            lv_obj_set_y(self.as_mut(), y.val());
        }
        self
    }

    fn set_pos(&mut self, x: Length, y: Length) -> &mut Self {
        unsafe {
            lv_obj_set_pos(self.as_mut(), x.val(), y.val());
        }
        self
    }

    fn set_flex_grow(&mut self, grow: u8) -> &mut Self {
        unsafe {
            lv_obj_set_flex_grow(self.as_mut(), grow);
        }
        self
    }

    fn layout<'a, T: LvObjLayout<'a>>(&'a mut self) -> T {
        T::create(self)
    }

    fn set_style_bg_opa(&mut self, opacity: Opacity, style: StyleSelector) -> &mut Self {
        unsafe { lv_obj_set_style_bg_opa(self.as_mut(), opacity.val(), style.val) }
        self
    }

    fn set_style_bg_color(&mut self, color: Color, style: StyleSelector) -> &mut Self {
        unsafe { lv_obj_set_style_bg_color(self.as_mut(), color.val(), style.val) }
        self
    }

    fn set_style_opa(&mut self, opacity: Opacity, style: StyleSelector) -> &mut Self {
        unsafe { lv_obj_set_style_opa(self.as_mut(), opacity.val(), style.val) }
        self
    }

    fn set_scrollbar_mode(&mut self, mode: ScrollBarMode) -> &mut Self {
        unsafe { lv_obj_set_scrollbar_mode(self.as_mut(), mode as _) }
        self
    }

    fn set_transform_pivot_x(&mut self, x: i32, style: StyleSelector) -> &mut Self {
        unsafe { lv_obj_set_style_transform_pivot_x(self.as_mut(), x, style.val) }
        self
    }

    fn set_transform_pivot_y(&mut self, y: i32, style: StyleSelector) -> &mut Self {
        unsafe { lv_obj_set_style_transform_pivot_y(self.as_mut(), y, style.val) }
        self
    }

    fn set_transform_pivot_pos(&mut self, x: i32, y: i32, style: StyleSelector) -> &mut Self {
        unsafe {
            lv_obj_set_style_transform_pivot_x(self.as_mut(), x, style.val);
            lv_obj_set_style_transform_pivot_y(self.as_mut(), y, style.val);
        }
        self
    }

    /// span未经测试
    fn set_grid_cell(
        &mut self,
        column_align: GridAlign,
        col_pos: Length,
        col_span: i32,
        row_align: GridAlign,
        row_pos: Length,
        row_span: i32,
    ) -> &mut Self {
        unsafe {
            lv_obj_set_grid_cell(
                self.as_mut(),
                column_align as _,
                col_pos.val(),
                col_span,
                row_align as _,
                row_pos.val(),
                row_span,
            )
        }
        self
    }

    fn set_scroll_snap_x(&mut self, snap: ScrollSnap) -> &mut Self {
        unsafe {
            lv_obj_set_scroll_snap_x(self.as_mut(), snap as _);
        }
        self
    }

    fn set_style_pad_column(&mut self, val: i32, selector: StyleSelector) -> &mut Self {
        unsafe { lv_obj_set_style_pad_column(self.as_mut(), val, selector.val) }
        self
    }

    fn set_style_pad_row(&mut self, val: i32, selector: StyleSelector) -> &mut Self {
        unsafe { lv_obj_set_style_pad_row(self.as_mut(), val, selector.val) }
        self
    }

    fn set_style_pad_left(&mut self, val: i32, selector: StyleSelector) -> &mut Self {
        unsafe { lv_obj_set_style_pad_left(self.as_mut(), val, selector.val) }
        self
    }

    fn set_style_pad_right(&mut self, val: i32, selector: StyleSelector) -> &mut Self {
        unsafe { lv_obj_set_style_pad_right(self.as_mut(), val, selector.val) }
        self
    }

    fn set_style_pad_top(&mut self, val: i32, selector: StyleSelector) -> &mut Self {
        unsafe { lv_obj_set_style_pad_top(self.as_mut(), val, selector.val) }
        self
    }

    fn set_style_pad_bottom(&mut self, val: i32, selector: StyleSelector) -> &mut Self {
        unsafe { lv_obj_set_style_pad_bottom(self.as_mut(), val, selector.val) }
        self
    }
}

pub struct ObjUtil;

impl ObjUtil {
    pub fn scroll_to_view(to: &dyn LvObjPtr, anim_en: bool) {
        unsafe {
            lv_obj_scroll_to_view(to.as_ptr(), anim_en);
        }
    }

    pub unsafe fn get_child<T: LvObj>(parent: &dyn LvObjPtr, idx: i32) -> T {
        T::from_raw(unsafe { lv_obj_get_child(parent.as_ptr(), idx) })
    }
}

// /// 防止在多线程的情况下使用
// pub struct LvObjAlter<T: LvObj>(T, PhantomData<*const ()>);
//
// impl<T> LvObjPtr for LvObjAlter<T>
// where
//     T: LvObj,
// {
//     fn as_ptr(&self) -> *mut lv_obj_t {
//         self.0.as_ptr()
//     }
//
//     fn as_mut(&mut self) -> *mut lv_obj_t {
//         self.0.as_mut()
//     }
// }

// impl<T> LvObjCreator for LvObjAlter<T>
// where
//     T: LvObj,
// {
//     fn create(parent: &dyn LvObjPtr) -> Self {
//         Self(T::create(parent), Default::default())
//     }
// }
//
// impl<T> LvObjEvent for LvObjAlter<T> where T: LvObj {}
// impl<T> LvObjEventData for LvObjAlter<T> where T: LvObj {}
//
// impl<T> LvObj for LvObjAlter<T>
// where
//     T: LvObj,
// {
//     fn from_raw(raw: *mut lv_obj_t) -> Self {
//         Self(T::from_raw(raw), Default::default())
//     }
// }
//
// impl<T: LvObj> LvObjAlter<T> {
//     pub fn finish(&mut self) -> T {
//         unsafe { (self as *mut LvObjAlter<T>).read().0 }
//     }
// }
