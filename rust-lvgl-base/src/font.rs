use rust_lvgl_sys::lv_font_t;

pub struct Font {
    font: *mut lv_font_t,
}

impl Font {
    pub fn as_ptr(&self) -> *mut lv_font_t {
        self.font
    }

    pub fn as_mut(&mut self) -> *mut lv_font_t {
        self.font
    }
}

#[cfg(feature = "tiny_ttf")]
impl Font {
    pub fn create_file(path: &str, font_size: u16) -> Self {
        use alloc::ffi::CString;
        use rust_lvgl_sys::lv_tiny_ttf_create_file;
        let path = CString::new(path).unwrap();
        Self {
            font: unsafe { lv_tiny_ttf_create_file(path.as_ptr(), font_size as _) },
        }
    }
}
