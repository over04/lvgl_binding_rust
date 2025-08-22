// #![no_std]
#![allow(unsafe_op_in_unsafe_fn)]
extern crate alloc;

pub mod driver;
pub mod layer;
pub mod layout;
pub mod widgets;
pub mod base {
    pub use rust_lvgl_base::*;
}
pub mod sys {
    pub use rust_lvgl_sys::*;
}

pub mod macros {
    pub use rust_lvgl_macro::*;
}

static mut IS_INIT: bool = false;
pub fn init() {
    unsafe {
        if !IS_INIT {
            rust_lvgl_sys::lv_init();
            IS_INIT = true;
        }
    }
}
