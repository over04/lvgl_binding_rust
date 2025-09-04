use libc::usleep;
use rust_lvgl_sys::{
    lv_display_t, lv_indev_active, lv_indev_get_point, lv_indev_set_display, lv_indev_t,
    lv_point_t, lv_timer_handler,
};

use crate::typing::{indev::IndevType, point::Point};

static mut IS_INIT: bool = false;

pub trait DisplayDriverBase {
    fn get_display(&self) -> *mut lv_display_t;
    fn handle(&mut self) {
        loop {
            unsafe {
                usleep(lv_timer_handler());
            }
        }
    }
}

pub struct LvDisplayDriver;

impl LvDisplayDriver {
    pub fn init() {
        unsafe {
            if !IS_INIT {
                rust_lvgl_sys::lv_init();
                IS_INIT = true;
            }
        }
    }
}

pub trait DisplayDriver<T>
where
    Self: DisplayDriverBase + Sized,
{
    fn _create(val: T) -> Self;

    fn create(val: T) -> Self {
        LvDisplayDriver::init();
        Self::_create(val)
    }
}

pub struct LvIndevDriver;

impl LvIndevDriver {
    pub fn get_active(t: IndevType) -> IndevDriverObj {
        IndevDriverObj::from_raw(unsafe { lv_indev_active() }, t)
    }
}

pub trait IndevDriver<T> {
    fn create(val: T) -> Self;
    fn from_raw(raw: *mut lv_indev_t, t: IndevType) -> Self;
    fn get_indev(&self) -> *mut lv_indev_t;
    fn get_type(&self) -> IndevType;

    fn set_display(&mut self, display: &dyn DisplayDriverBase) -> &mut Self {
        unsafe {
            lv_indev_set_display(self.get_indev(), display.get_display());
        }
        self
    }

    fn get_point(&self) -> Option<Point> {
        match self.get_type() {
            IndevType::Pointer => unsafe {
                let mut point = lv_point_t::default();
                lv_indev_get_point(self.get_indev(), &mut point);
                Some(Point {
                    x: point.x,
                    y: point.y,
                })
            },
            _ => None,
        }
    }
}

pub struct IndevDriverObj {
    indev: *mut lv_indev_t,
    t: IndevType,
}

impl IndevDriver<(*mut lv_indev_t, IndevType)> for IndevDriverObj {
    fn create(val: (*mut lv_indev_t, IndevType)) -> Self {
        Self {
            indev: val.0,
            t: val.1,
        }
    }

    fn get_type(&self) -> IndevType {
        self.t.clone()
    }

    fn from_raw(raw: *mut lv_indev_t, t: IndevType) -> Self {
        Self { indev: raw, t }
    }

    fn get_indev(&self) -> *mut lv_indev_t {
        self.indev
    }
}
