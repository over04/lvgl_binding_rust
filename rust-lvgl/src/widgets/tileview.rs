use rust_lvgl_base::obj::{LvObj, LvObjPtr};
use rust_lvgl_base::typing::dir::DirSelector;
use rust_lvgl_macro::lvgl_obj;
use rust_lvgl_sys::{lv_tileview_add_tile, lv_tileview_set_tile, lv_tileview_set_tile_by_index};

#[lvgl_obj]
pub struct TileView {}

impl TileView {
    pub fn create_tile<T: LvObj>(&mut self, col: u8, row: u8, dir: DirSelector) -> T {
        unsafe { T::from_raw(lv_tileview_add_tile(self.as_mut(), col, row, dir.val)) }
    }

    pub fn set_tile(&mut self, tile: &dyn LvObjPtr, anim_enabled: bool) -> &mut Self {
        unsafe {
            lv_tileview_set_tile(self.as_mut(), tile.as_ptr(), anim_enabled);
        }
        self
    }

    pub fn set_title_by_index(&mut self, col: u32, row: u32, anim_enabled: bool) -> &mut Self {
        unsafe {
            lv_tileview_set_tile_by_index(self.as_mut(), col, row, anim_enabled);
        }
        self
    }
}
