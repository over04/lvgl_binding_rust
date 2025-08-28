#[repr(u8)]
pub enum Flex {
    Column = 1 << 0,
    Wrap = 1 << 2,
    Reverse = 1 << 3,
}

#[repr(u8)]
pub enum FlexFlow {
    Row = 0,
    Column = Flex::Column as _,
    RowWrap = Flex::Wrap as u8,
    RowReverse = Flex::Reverse as u8,
    RowWrapReverse = Flex::Wrap as u8 | Flex::Reverse as u8,
    ColumnWrap = Flex::Column as u8 | Flex::Wrap as u8,
    ColumnReverse = Flex::Column as u8 | Flex::Reverse as u8,
    ColumnWrapReverse = Flex::Column as u8 | Flex::Wrap as u8 | Flex::Reverse as u8,
}

#[repr(u8)]
pub enum FlexAlign {
    Start,
    End,
    Center,
    SpaceEvenly,
    SpaceAround,
    SpaceBetween,
}
