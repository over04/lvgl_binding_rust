use crate::IS_INIT;
use libc::usleep;
use lvgl_sys::{lv_display_t, lv_indev_t, lv_init, lv_timer_handler};

pub mod sdl2;
pub trait DisplayDriver<T>
where
    Self: Sized,
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
    fn get_display(&self) -> *mut lv_display_t;

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
    fn get_display(&self) -> *mut lv_indev_t;
}
