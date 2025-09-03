use std::{ffi::CString, ptr::null_mut};

use rust_lvgl_base::{
    obj::{LvObj, LvObjCreator, LvObjPtr},
    typing::image::ImageSrc,
};
use rust_lvgl_macro::lvgl_obj;
use rust_lvgl_sys::{lv_list_add_button, lv_list_create, lv_obj_create};

#[lvgl_obj]
pub struct List {}

impl LvObjCreator for List {
    fn create(parent: &dyn rust_lvgl_base::obj::LvObjPtr) -> Self {
        Self {
            _lv_obj_ptr: unsafe { lv_list_create(parent.as_ptr()) },
        }
    }
}

impl List {
    pub fn create_button(&mut self, icon: Option<ImageSrc>, text: &str) -> ListButton {
        let mut temp_src: Option<CString> = None;
        let temp_text = CString::new(text).unwrap();
        let ptr = match icon {
            Some(ImageSrc::Path(src)) => {
                temp_src = Some(CString::new(src.as_str()).unwrap());
                temp_src.as_ref().unwrap().as_ptr() as _
            }
            Some(ImageSrc::Symbol(src)) => src.as_ptr() as _,
            Some(ImageSrc::Ptr(ptr)) => ptr,
            Some(ImageSrc::ImageDsc(dsc)) => dsc.dsc as _,
            None => null_mut(),
        };
        let mut btn = ListButton::from_raw(unsafe {
            lv_list_add_button(self.as_ptr(), ptr, temp_text.as_ptr())
        });
        btn._temp_src = temp_src;
        btn._text = temp_text;
        btn
    }
}

#[lvgl_obj]
pub struct ListButton {
    _temp_src: Option<CString>,
    _text: CString,
}

impl LvObjCreator for ListButton {
    fn create(parent: &dyn LvObjPtr) -> Self {
        Self {
            _lv_obj_ptr: unsafe { lv_obj_create(parent.as_ptr()) },
            ..Default::default()
        }
    }
}
