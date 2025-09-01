use crate::typing::part::Part;
use crate::typing::state::State;

#[derive(Debug, Clone)]
pub enum StyleSelectorVal {
    State(State),
    Part(Part),
}

#[repr(u8)]
pub enum BlendMode {
    Normal = 0,
    Additive,
    Subtractive,
    Multiply,
    Difference,
}

impl StyleSelectorVal {
    pub const fn val(self) -> u32 {
        match self {
            StyleSelectorVal::State(val) => val as u32,
            StyleSelectorVal::Part(val) => val as u32,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StyleSelector {
    pub val: u32,
}

impl StyleSelector {
    pub const fn new() -> Self {
        Self { val: 0 }
    }

    pub const fn add(mut self, val: StyleSelectorVal) -> Self {
        self.val |= val.val();
        self
    }

    pub const fn del(mut self, val: StyleSelectorVal) -> Self {
        self.val &= !val.val();
        self
    }
}
