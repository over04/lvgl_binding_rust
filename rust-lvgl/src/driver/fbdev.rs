#![cfg(feature = "fbdev")]

use crate::driver::{DisplayDriver, DisplayDriverPtr};
use rust_lvgl_sys::{lv_display_t, lv_linux_fbdev_create, lv_linux_fbdev_set_file};
use std::ffi::CString;

pub struct FBDev {
    display: *mut lv_display_t,
}

impl DisplayDriverPtr for FBDev {
    fn get_display(&self) -> *mut lv_display_t {
        self.display
    }
}

impl DisplayDriver<&str> for FBDev {
    fn _create(val: &str) -> Self {
        let val = CString::new(val).unwrap();
        unsafe {
            let display = lv_linux_fbdev_create();
            lv_linux_fbdev_set_file(display, val.as_ptr());
            Self { display }
        }
    }
}