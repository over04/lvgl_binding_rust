#[derive(Debug, Clone)]
#[repr(u8)]
pub enum IndevType {
    None = 0,
    Pointer,
    Keypad,
    Button,
    Encoder
}