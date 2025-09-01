use rust_lvgl_sys::{lv_snapshot_free, lv_snapshot_take};

use crate::{
    obj::LvObjPtr,
    typing::{color::ColorFormat, image::ImageDsc},
};

#[derive(Debug)]
pub struct Snapshot {
    pub image: ImageDsc,
}

impl Snapshot {
    /// 拍摄照片
    pub fn take(target: &dyn LvObjPtr, color_format: ColorFormat) -> Self {
        Self {
            image: ImageDsc {
                dsc: unsafe { lv_snapshot_take(target.as_ptr(), color_format as _) as _ },
            },
        }
    }
}

impl Drop for Snapshot {
    fn drop(&mut self) {
        unsafe {
            lv_snapshot_free(self.image.dsc);
        }
    }
}
