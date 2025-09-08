#![cfg(feature = "evdev")]

use rust_lvgl_base::{driver::IndevDriver, typing::indev::IndevType};
use rust_lvgl_sys::{lv_evdev_create, lv_indev_t};
use std::ffi::CString;

pub struct EVDev {
    indev: *mut lv_indev_t,
    t: IndevType,
}

impl IndevDriver<(IndevType, &str)> for EVDev {
    fn from_raw(raw: *mut lv_indev_t, t: IndevType) -> Self {
        Self { indev: raw, t }
    }

    fn create(val: (IndevType, &str)) -> Self {
        let path = CString::new(val.1).unwrap();
        unsafe {
            let indev = lv_evdev_create(val.0.clone() as _, path.as_ptr());
            Self { indev, t: val.0 }
        }
    }

    fn get_indev(&self) -> *mut lv_indev_t {
        self.indev
    }

    fn get_type(&self) -> IndevType {
        self.t.clone()
    }
}
