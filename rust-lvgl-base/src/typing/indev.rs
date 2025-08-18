#[repr(u8)]
pub enum LVIndevType {
    None = 0,
    Pointer,
    Keypad,
    Button,
    Encoder
}