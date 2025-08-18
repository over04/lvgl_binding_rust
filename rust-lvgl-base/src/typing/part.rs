#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Part {
    Main = 0x000000,
    ScrollBar = 0x010000,
    Indicator = 0x020000,
    Knob = 0x030000,
    Selected = 0x040000,
    Items = 0x050000,
    Cursor = 0x060000,
    CustomFirst = 0x080000,
    Any = 0x0F0000,
}
