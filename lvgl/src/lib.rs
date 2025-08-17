#![allow(unsafe_op_in_unsafe_fn)]

pub mod driver;
pub mod layer;
pub mod layout;
pub mod widgets;
pub mod base {
    pub use lvgl_base::*;
}
pub mod sys {
    pub use lvgl_sys::*;
}

pub mod macros {
    pub use lvgl_macro::*;
}

static mut IS_INIT: bool = false;
pub fn init() {
    unsafe {
        if !IS_INIT {
            lvgl_sys::lv_init();
            IS_INIT = true;
        }
    }
}
