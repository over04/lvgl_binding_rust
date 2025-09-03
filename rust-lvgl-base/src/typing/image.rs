use alloc::string::String;
use core::{ffi::c_void, fmt::Debug};
use rust_lvgl_sys::lv_image_dsc_t;

#[derive(Debug, Clone)]
pub enum ImageSrc {
    Path(String),
    Symbol(&'static [u8; 4]),
    ImageDsc(ImageDsc),
    Ptr(*mut c_void),
}

#[repr(u8)]
pub enum ImageAlign {
    Default = 0,
    TopLeft,
    TopMid,
    TopRight,
    BottomLeft,
    BottomMid,
    BottomRight,
    LeftMid,
    RightMid,
    Center,
    AutoTransform,
    /// Set X and Y scale to fill the Widget's area.
    Stretch,
    /// Tile image to fill Widget's area. Offset is applied to shift the tiling.
    Tile,
    /// The image keeps its aspect ratio, but is resized to the maximum size that fits within the Widget's area.
    Contain,
    Cover,
}

#[derive(Debug, Clone)]
pub struct ImageDsc {
    pub dsc: *mut lv_image_dsc_t,
}
