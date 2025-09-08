#![cfg(feature = "sdl2")]

use rust_lvgl_base::{
    driver::{DisplayDriver, DisplayDriverBase, IndevDriver},
    typing::indev::IndevType,
};
use rust_lvgl_sys::{
    SDL_GetTicks, lv_display_t, lv_indev_t, lv_sdl_mouse_create, lv_sdl_window_create, lv_tick_inc,
    lv_timer_handler,
};

pub struct SDL2Display {
    display: *mut lv_display_t,
    last_tick: Option<u32>,
}

pub struct SDL2Mouth {
    indev: *mut lv_indev_t,
}

impl DisplayDriverBase for SDL2Display {
    fn get_display(&self) -> *mut lv_display_t {
        self.display
    }

    fn handle(&mut self) {
        unsafe {
            let current_tick = SDL_GetTicks();
            lv_tick_inc(current_tick - self.last_tick.unwrap_or(SDL_GetTicks()));
            lv_timer_handler();
            self.last_tick = Some(current_tick);
        }
    }
}
impl DisplayDriver<(i32, i32)> for SDL2Display {
    fn _create(val: (i32, i32)) -> Self {
        unsafe {
            Self {
                display: lv_sdl_window_create(val.0, val.1),
                last_tick: None,
            }
        }
    }
}

impl IndevDriver<()> for SDL2Mouth {
    fn from_raw(raw: *mut lv_indev_t, _: IndevType) -> Self {
        Self { indev: raw }
    }

    fn get_type(&self) -> IndevType {
        IndevType::Pointer
    }

    fn create(_: ()) -> Self {
        Self {
            indev: unsafe { lv_sdl_mouse_create() },
        }
    }

    fn get_indev(&self) -> *mut lv_indev_t {
        self.indev
    }
}
