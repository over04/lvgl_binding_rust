pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const fn from_hex(hex: u32) -> Self {
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
    pub const fn val(self) -> u8 {
        match self {
            Opacity::Val(val) => val,
            Opacity::Pct(pct) => ((pct as f32 / 100.) * 255.) as u8,
        }
    }
}

#[repr(u32)]
pub enum ColorFormat {
    Unknown = 0,

    RAW = 0x01,
    RAWALPHA = 0x02,

    L8 = 0x06,
    I1 = 0x07,
    I2 = 0x08,
    I4 = 0x09,
    I8 = 0x0A,
    A8 = 0x0E,

    RGB565 = 0x12,
    ARGB8565 = 0x13,
    RGB565A8 = 0x14,
    AL88 = 0x15,
    RGB565SWAPPED = 0x1B,

    RGB888 = 0x0F,
    ARGB8888 = 0x10,
    XRGB8888 = 0x11,
    ARGB8888PREMULTIPLIED = 0x1A,

    A1 = 0x0B,
    A2 = 0x0C,
    A4 = 0x0D,
    ARGB1555 = 0x16,
    ARGB4444 = 0x17,
    ARGB2222 = 0x18,

    I420 = 0x20,
    I422 = 0x21,
    I444 = 0x22,
    I400 = 0x23,
    NV21 = 0x24,
    NV12 = 0x25,

    YUY2 = 0x26,
    UYVY = 0x27,

    NEMATsc4 = 0x30,
    NEMATSC6 = 0x31,
    NEMATSC6A = 0x32,
    NEMATSC6AP = 0x33,
    NEMATSC12 = 0x34,
    NEMATSC12A = 0x35,
}
