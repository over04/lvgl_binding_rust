use std::ffi::CString;

use rust_lvgl_base::{
    obj::{LvObjCreator, LvObjPtr},
    typing::size::Length,
};
use rust_lvgl_macro::lvgl_obj;
use rust_lvgl_sys::{lv_qrcode_create, lv_qrcode_set_size, lv_qrcode_update};

#[lvgl_obj]
pub struct QrCode {
    data: CString,
}

impl LvObjCreator for QrCode {
    fn create(parent: &dyn LvObjPtr) -> Self {
        Self {
            _lv_obj_ptr: unsafe { lv_qrcode_create(parent.as_ptr()) },
            ..Default::default()
        }
    }
}

impl QrCode {
    pub fn update(&mut self, data: &str) -> &mut Self {
        let data = CString::new(data).unwrap();
        unsafe {
            lv_qrcode_update(
                self.as_mut(),
                data.as_ptr() as *mut _,
                data.count_bytes() as _,
            )
        };
        self.data = data;
        self
    }

    pub fn set_qrcode_size(&mut self, size: Length) -> &mut Self {
        unsafe {
            lv_qrcode_set_size(self.as_mut(), size.val());
        }
        self
    }
}
