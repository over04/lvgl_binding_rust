pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
        }
    }

    pub fn val(&self) -> rust_lvgl_sys::lv_color_t {
        unsafe {
            rust_lvgl_sys::lv_color_hex(
                (self.r as u32) << 16 | (self.g as u32) << 8 | (self.b as u32),
            )
        }
    }
}

#[derive(Debug, Clone)]
pub enum Opacity {
    Val(u8),
    Pct(u8),
}

impl Opacity {
    pub fn val(self) -> u8 {
        match self {
            Opacity::Val(val) => val,
            Opacity::Pct(pct) => ((pct as f32 / 100.) * 255.) as u8,
        }
    }
}
