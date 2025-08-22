use crate::layout::LvObjLayout;
use crate::typing::align::Align;
use crate::typing::event::{
    EventCb, EventCbWithData, EventCode, EventData, event_handler_cb, event_handler_cb_with_data,
};
use crate::typing::flag::Flag;
use crate::typing::size::Length;
use alloc::boxed::Box;
use core::ffi::c_void;
use rust_lvgl_sys::{
    lv_obj_add_event_cb, lv_obj_add_flag, lv_obj_align, lv_obj_align_to, lv_obj_center,
    lv_obj_flag_t, lv_obj_flag_t_LV_OBJ_FLAG_HIDDEN, lv_obj_remove_flag, lv_obj_remove_style_all,
    lv_obj_set_align, lv_obj_set_flag, lv_obj_set_height, lv_obj_set_pos, lv_obj_set_size,
    lv_obj_set_width, lv_obj_set_x, lv_obj_set_y, lv_obj_t,
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

pub trait LvObj
where
    Self: LvObjPtr + LvObjEvent + LvObjEventData + Sized,
{
    fn create(parent: &dyn LvObjPtr) -> Self;
    fn from_raw(raw: *mut lv_obj_t) -> Self;

    fn set_width(&mut self, width: i32) -> &mut Self {
        unsafe {
            lv_obj_set_width(self.as_mut(), width);
        }
        self
    }
    fn set_height(&mut self, height: i32) -> &mut Self {
        unsafe {
            lv_obj_set_height(self.as_mut(), height);
        }
        self
    }

    fn set_size(&mut self, width: Length, height: Length) -> &mut Self {
        unsafe {
            lv_obj_set_size(self.as_mut(), width.value(), height.value());
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

    fn set_x(&mut self, x: i32) -> &mut Self {
        unsafe {
            lv_obj_set_x(self.as_mut(), x);
        }
        self
    }
    fn set_y(&mut self, y: i32) -> &mut Self {
        unsafe {
            lv_obj_set_y(self.as_mut(), y);
        }
        self
    }

    fn set_pos(&mut self, x: i32, y: i32) -> &mut Self {
        unsafe {
            lv_obj_set_pos(self.as_mut(), x, y);
        }
        self
    }

    fn layout<'a, T: LvObjLayout<'a>>(&'a mut self) -> T {
        T::create(self)
    }
}
