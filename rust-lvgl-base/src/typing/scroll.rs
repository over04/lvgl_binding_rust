#[repr(u8)]
pub enum ScrollBarMode {
    Off,
    On,
    Active,
    Auto,
}

#[repr(u8)]
pub enum ScrollSnap {
    None,
    Start,
    End,
    Center,
}
