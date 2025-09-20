use std::{ffi::CString, ptr::null_mut};

use rust_lvgl_base::obj::{LvObj, LvObjPtr, Obj};
use rust_lvgl_macro::lvgl_obj;
use rust_lvgl_sys::{
    lv_msgbox_add_close_button, lv_msgbox_add_footer_button, lv_msgbox_add_text,
    lv_msgbox_add_title, lv_msgbox_close, lv_msgbox_create, lv_msgbox_get_content,
    lv_msgbox_get_footer, lv_msgbox_get_header, lv_msgbox_get_title,
};

use crate::widgets::{button::Button, label::Label};

#[lvgl_obj]
pub struct MsgBox {}

impl MsgBox {
    pub fn create(parent: Option<&dyn LvObjPtr>) -> Self {
        Self {
            _lv_obj_ptr: unsafe {
                lv_msgbox_create(parent.map(|t| t.as_ptr()).unwrap_or(null_mut()))
            },
        }
    }

    pub fn get_content(&self) -> Obj {
        Obj::from_raw(unsafe { lv_msgbox_get_content(self.as_ptr()) })
    }

    pub fn get_title(&self) -> Option<Obj> {
        let ptr = unsafe { lv_msgbox_get_title(self.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(Obj::from_raw(ptr))
        }
    }

    pub fn get_footer(&self) -> Option<Obj> {
        let ptr = unsafe { lv_msgbox_get_footer(self.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(Obj::from_raw(ptr))
        }
    }
    pub fn get_header(&self) -> Option<Obj> {
        let ptr = unsafe { lv_msgbox_get_header(self.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(Obj::from_raw(ptr))
        }
    }

    pub fn add_text(&mut self, text: Option<&str>) -> Label {
        Label::from_raw(match text {
            Some(text) => {
                let text = CString::new(text).unwrap();
                unsafe { lv_msgbox_add_text(self.as_mut(), text.as_ptr()) }
            }
            None => unsafe { lv_msgbox_add_text(self.as_mut(), null_mut()) },
        })
    }

    pub fn set_title(&mut self, text: Option<&str>) -> Label {
        Label::from_raw(match text {
            Some(text) => {
                let text = CString::new(text).unwrap();
                unsafe { lv_msgbox_add_title(self.as_mut(), text.as_ptr()) }
            }
            None => unsafe { lv_msgbox_add_title(self.as_mut(), null_mut()) },
        })
    }

    pub fn add_close_button(&mut self) -> Button {
        Button::from_raw(unsafe { lv_msgbox_add_close_button(self.as_mut()) })
    }

    // pub fn add_header_button(&mut self, icon: ImageSrc) -> Button {
    //     Button::from_raw(unsafe {
    //         lv_msgbox_add_header_button(
    //             self.as_mut(),

    //         )
    //     })
    // }

    pub fn add_footer_button(&mut self, text: Option<&str>) -> Button {
        let text = text.map(|text| CString::new(text).unwrap());
        Button::from_raw(unsafe {
            lv_msgbox_add_footer_button(
                self.as_mut(),
                text.map(|s| s.as_ptr()).unwrap_or(null_mut()),
            )
        })
    }

    pub fn close(self) {
        unsafe { lv_msgbox_close(self.as_ptr()) };
    }
}
