use alloc::vec;
use alloc::vec::Vec;
use rust_lvgl_base::layout::{LvObjLayout, LvObjLayoutPad};
use rust_lvgl_base::obj::{LvObj, LvObjPtr};
use rust_lvgl_base::typing::grid::{GridAlign, GridSize};
use rust_lvgl_sys::{
    LV_GRID_CONTENT, LV_GRID_TEMPLATE_LAST, lv_grid_fr, lv_layout_t_LV_LAYOUT_GRID,
    lv_obj_set_grid_align, lv_obj_set_grid_cell, lv_obj_set_grid_dsc_array, lv_obj_set_layout,
    lv_obj_t,
};

pub struct GridLayout<'a> {
    obj: &'a mut dyn LvObjPtr,
    column_dsc: *mut [i32],
    row_dsc: *mut [i32],
}

impl LvObjPtr for GridLayout<'_> {
    fn as_ptr(&self) -> *mut lv_obj_t {
        self.obj.as_ptr()
    }

    fn as_mut(&mut self) -> *mut lv_obj_t {
        self.obj.as_mut()
    }
}

impl<'a> LvObjLayout<'a> for GridLayout<'a> {
    fn create(obj: &'a mut dyn LvObjPtr) -> Self {
        unsafe {
            lv_obj_set_layout(obj.as_mut(), lv_layout_t_LV_LAYOUT_GRID);
            Self {
                obj,
                column_dsc: Box::leak(vec![LV_GRID_TEMPLATE_LAST as i32].into_boxed_slice()),
                row_dsc: Box::leak(vec![LV_GRID_TEMPLATE_LAST as i32].into_boxed_slice()),
            }
        }
    }
}

impl<'a> LvObjLayoutPad<'a> for GridLayout<'a> {}

impl GridLayout<'_> {
    fn grid_size_vec_to_dsc(v: &[GridSize]) -> Box<[i32]> {
        let mut v: Vec<_> = v
            .iter()
            .map(|x| match x {
                GridSize::Pixel(x) => *x,
                GridSize::Content => LV_GRID_CONTENT as i32,
                GridSize::FR(x) => unsafe { lv_grid_fr(*x) },
            })
            .collect();
        v.push(LV_GRID_TEMPLATE_LAST as i32);
        v.into_boxed_slice()
    }

    pub fn set_dsc(&mut self, column_dsc: &[GridSize], row_dsc: &[GridSize]) -> &mut Self {
        let (old_column_dsc, old_row_dsc) = (self.column_dsc, self.row_dsc);
        self.column_dsc = Box::leak(Self::grid_size_vec_to_dsc(column_dsc));
        self.row_dsc = Box::leak(Self::grid_size_vec_to_dsc(row_dsc));

        unsafe {
            lv_obj_set_grid_dsc_array(self.as_mut(), self.column_dsc as _, self.row_dsc as _);
            let _ = Box::from_raw(old_column_dsc);
            let _ = Box::from_raw(old_row_dsc);
        }
        self
    }

    pub fn set_align(&mut self, column_align: GridAlign, row_align: GridAlign) -> &mut Self {
        unsafe { lv_obj_set_grid_align(self.as_mut(), column_align as _, row_align as _) }
        self
    }

    pub fn create_grid_cell<T: LvObj>(
        &mut self,
        column_align: GridAlign,
        col_pos: i32,
        col_span: i32,
        row_align: GridAlign,
        row_pos: i32,
        row_pan: i32,
    ) -> T {
        let mut obj = T::create(self);
        unsafe {
            lv_obj_set_grid_cell(
                obj.as_mut(),
                column_align as _,
                col_pos,
                col_span,
                row_align as _,
                row_pos,
                row_pan,
            );
        }
        obj
    }
}
