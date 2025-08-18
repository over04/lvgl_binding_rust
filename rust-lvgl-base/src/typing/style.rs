use crate::typing::part::Part;
use crate::typing::state::State;

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
    pub fn val(&self) -> u32 {
        match self {
            StyleSelectorVal::State(val) => *val as u32,
            StyleSelectorVal::Part(val) => *val as u32,
        }
    }
}

pub struct StyleSelector {
    pub val: u32,
}

impl StyleSelector {
    pub fn new() -> Self {
        Self { val: 0 }
    }

    pub fn add(&mut self, val: StyleSelectorVal) -> &mut Self {
        self.val |= val.val();
        self
    }

    pub fn del(&mut self, val: StyleSelectorVal) -> &mut Self {
        self.val &= !val.val();
        self
    }
}
