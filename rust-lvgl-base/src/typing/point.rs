use rust_lvgl_sys::lv_point_t;

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl From<lv_point_t> for Point {
    fn from(value: lv_point_t) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
