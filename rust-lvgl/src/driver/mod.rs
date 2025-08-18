use crate::IS_INIT;
use libc::usleep;
use rust_lvgl_sys::{lv_display_t, lv_indev_set_display, lv_indev_t, lv_init, lv_timer_handler};

#[cfg(feature = "evdev")]
pub mod evdev;
#[cfg(feature = "fbdev")]
pub mod fbdev;
#[cfg(feature = "sdl2")]
pub mod sdl2;

pub trait DisplayDriverPtr {
    fn get_display(&self) -> *mut lv_display_t;
}

pub trait DisplayDriver<T>
where
    Self: DisplayDriverPtr + Sized,
{
    fn _create(val: T) -> Self;

    fn create(val: T) -> Self {
        unsafe {
            if !IS_INIT {
                lv_init();
            }
        }
        Self::_create(val)
    }

    fn handle(&mut self) {
        loop {
            unsafe {
                usleep(lv_timer_handler());
            }
        }
    }
}

pub trait IndevDriver<T> {
    fn create(val: T) -> Self;
    fn get_indev(&self) -> *mut lv_indev_t;

    fn set_display(&mut self, display: &dyn DisplayDriverPtr) -> &mut Self {
        unsafe {
            lv_indev_set_display(self.get_indev(), display.get_display());
        }
        self
    }
}
