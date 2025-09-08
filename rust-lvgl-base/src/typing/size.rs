use rust_lvgl_sys::lv_pct;

#[derive(Clone)]
pub enum Length {
    Pct(i32),
    Pix(i32),
}

impl Length {
    pub fn val(self) -> i32 {
        match self {
            Length::Pct(val) => unsafe { lv_pct(val) },
            Length::Pix(val) => val,
        }
    }
}
