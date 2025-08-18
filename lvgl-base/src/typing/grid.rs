pub enum GridSize {
    Pixel(i32),
    Content,
    FR(u8),
}

#[repr(u8)]
pub enum GridAlign {
    Start = 0,
    Center,
    End,
    Stretch,
    SpaceEvenly,
    SpaceAround,
    SpaceBetween,
}
