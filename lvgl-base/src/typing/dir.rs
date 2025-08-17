use crate::typing::style::StyleSelectorVal;

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Dir {
    None = 0x00,
    LEFT = (1 << 0),
    RIGHT = (1 << 1),
    TOP = (1 << 2),
    BOTTOM = (1 << 3),
    HOR = (1 << 0) | (1 << 1),
    VER = (1 << 2) | (1 << 3),
    ALL = (1 << 4) - 1,
}

pub struct DirSelector {
    pub val: u32,
}
impl DirSelector {
    pub fn add(&mut self, val: Dir) -> &mut Self {
        self.val |= val as u32;
        self
    }

    pub fn del(&mut self, val: StyleSelectorVal) -> &mut Self {
        self.val &= !(val.val());
        self
    }
}
