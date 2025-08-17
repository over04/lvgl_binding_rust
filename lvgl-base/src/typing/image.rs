use std::ffi::c_void;

pub enum ImageSrc {
    Path(String),
    Symbol(String),
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
