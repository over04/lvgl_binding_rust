#[derive(Debug, Clone)]
pub enum GridSize {
    Pixel(i32),
    Content,
    FR(u8),
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum GridAlign {
    Start,
    Center,
    End,
    Stretch,
    SpaceEvenly,
    SpaceAround,
    SpaceBetween,
}
