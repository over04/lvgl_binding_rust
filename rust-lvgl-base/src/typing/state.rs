#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum State {
    Default = 0x0000,
    Checked = 0x0001,
    Focused = 0x0002,
    FocusKey = 0x0004,
    Edited = 0x0008,
    Hovered = 0x0010,
    Pressed = 0x0020,
    Scrolled = 0x0040,
    Disabled = 0x0080,
    User1 = 0x1000,
    User2 = 0x2000,
    User3 = 0x4000,
    User4 = 0x8000,
    LvStateAny = 0xFFFF,
}
