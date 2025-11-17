#[derive(Debug, Clone)]
#[repr(u8)]
pub enum IndevType {
    None = 0,
    Pointer,
    Keypad,
    Button,
    Encoder
}

#[derive(Debug, Clone)]
#[repr(u8)]
pub enum GestureDir {
    None = 0x00,
    Left = 1 << 0,
    Right = 1 << 1,
    Top = 1 << 2,
    Bottom = 1 << 3,
    Hor = Self::Left as u8 | Self::Right as u8,
    Ver = Self::Top as u8 | Self::Bottom as u8,
    All = Self::Hor as u8 | Self::Ver as u8
}

impl GestureDir {
    pub fn from_bits(bits: u8) -> Self {
        match bits {
            value if value == Self::None as u8 => Self::None,
            value if value == Self::Left as u8 => Self::Left,
            value if value == Self::Right as u8 => Self::Right,
            value if value == Self::Top as u8 => Self::Top,
            value if value == Self::Bottom as u8 => Self::Bottom,
            value if value == Self::Hor as u8 => Self::Hor,
            value if value == Self::Ver as u8 => Self::Ver,
            value if value == Self::All as u8 => Self::All,
            _ => Self::None,
        }
    }
}