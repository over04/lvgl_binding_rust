use rust_lvgl_sys::{
    lv_flex_flow_t_LV_FLEX_FLOW_COLUMN, lv_flex_flow_t_LV_FLEX_FLOW_COLUMN_REVERSE,
    lv_flex_flow_t_LV_FLEX_FLOW_COLUMN_WRAP, lv_flex_flow_t_LV_FLEX_FLOW_COLUMN_WRAP_REVERSE,
    lv_flex_flow_t_LV_FLEX_FLOW_ROW, lv_flex_flow_t_LV_FLEX_FLOW_ROW_REVERSE,
    lv_flex_flow_t_LV_FLEX_FLOW_ROW_WRAP, lv_flex_flow_t_LV_FLEX_FLOW_ROW_WRAP_REVERSE,
};

#[repr(u8)]
pub enum FlexFlow {
    Row = lv_flex_flow_t_LV_FLEX_FLOW_ROW as _,
    COLUMN = lv_flex_flow_t_LV_FLEX_FLOW_COLUMN as _,
    RowWrap = lv_flex_flow_t_LV_FLEX_FLOW_ROW_WRAP as _,
    RowReverse = lv_flex_flow_t_LV_FLEX_FLOW_ROW_REVERSE as _,
    RowWrapReverse = lv_flex_flow_t_LV_FLEX_FLOW_ROW_WRAP_REVERSE as _,
    ColumnWrap = lv_flex_flow_t_LV_FLEX_FLOW_COLUMN_WRAP as _,
    ColumnReverse = lv_flex_flow_t_LV_FLEX_FLOW_COLUMN_REVERSE as _,
    ColumnWrapReverse = lv_flex_flow_t_LV_FLEX_FLOW_COLUMN_WRAP_REVERSE as _,
}
