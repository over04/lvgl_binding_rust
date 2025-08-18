#![cfg(feature = "evdev")]
use crate::driver::IndevDriver;
use rust_lvgl_base::typing::indev::LVIndevType;
use rust_lvgl_sys::{lv_evdev_create, lv_indev_t};
use std::ffi::CString;

pub struct EVDev {
    indev: *mut lv_indev_t,
}

impl IndevDriver<(LVIndevType, &str)> for EVDev {
    fn create(val: (LVIndevType, &str)) -> Self {
        let path = CString::new(val.1).unwrap();
        unsafe {
            let indev = lv_evdev_create(val.0 as _, path.as_ptr());
            Self { indev }
        }
    }

    fn get_indev(&self) -> *mut lv_indev_t {
        self.indev
    }
}
