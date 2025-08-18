#![cfg(feature = "sdl2")]

use crate::driver::{DisplayDriver, DisplayDriverPtr, IndevDriver};
use rust_lvgl_sys::{
    SDL_GetTicks, lv_display_t, lv_indev_t, lv_sdl_mouse_create, lv_sdl_window_create, lv_tick_inc,
    lv_timer_handler,
};

pub struct SDL2Display {
    display: *mut lv_display_t,
}

pub struct SDL2Mouth {
    indev: *mut lv_indev_t,
}

impl DisplayDriverPtr for SDL2Display {
    fn get_display(&self) -> *mut lv_display_t {
        self.display
    }
}
impl DisplayDriver<(i32, i32)> for SDL2Display {
    fn _create(val: (i32, i32)) -> Self {
        unsafe {
            Self {
                display: lv_sdl_window_create(val.0, val.1),
            }
        }
    }

    fn handle(&mut self) {
        unsafe {
            let mut last_tick = SDL_GetTicks();
            loop {
                let current_tick = SDL_GetTicks();
                lv_tick_inc(current_tick - last_tick);
                last_tick = current_tick;
                lv_timer_handler();
            }
        }
    }
}

impl IndevDriver<()> for SDL2Mouth {
    fn create(_: ()) -> Self {
        Self {
            indev: unsafe { lv_sdl_mouse_create() },
        }
    }

    fn get_indev(&self) -> *mut lv_indev_t {
        self.indev
    }
}
