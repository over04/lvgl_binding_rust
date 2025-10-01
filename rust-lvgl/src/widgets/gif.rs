use std::ffi::CString;

use rust_lvgl_base::{
    obj::{LvObjCreator, LvObjPtr},
    typing::{
        image::{ImageAlign, ImageSrc},
        style::BlendMode,
    },
};
use rust_lvgl_macro::lvgl_obj;
use rust_lvgl_sys::{
    lv_gif_create, lv_gif_set_src, lv_image_get_scale, lv_image_set_antialias,
    lv_image_set_blend_mode, lv_image_set_inner_align, lv_image_set_offset_x,
    lv_image_set_offset_y, lv_image_set_pivot, lv_image_set_scale, lv_image_set_scale_x,
    lv_image_set_scale_y,
};

#[lvgl_obj]
pub struct GIF {
    _temp_src: Option<CString>,
}

impl LvObjCreator for GIF {
    fn create(parent: &dyn LvObjPtr) -> Self {
        Self {
            _lv_obj_ptr: unsafe { lv_gif_create(parent.as_ptr()) },
            _temp_src: None,
        }
    }
}

impl GIF {
    pub fn set_src(&mut self, src: ImageSrc) -> &mut Self {
        match src {
            ImageSrc::Path(src) => {
                self._temp_src = Some(CString::new(src.as_str()).unwrap());
                unsafe {
                    lv_gif_set_src(
                        self.as_mut(),
                        self._temp_src.as_ref().unwrap().as_ptr() as _,
                    );
                }
            }
            ImageSrc::Symbol(src) => unsafe {
                lv_gif_set_src(self.as_mut(), src.as_ptr() as _);
            },
            ImageSrc::Ptr(ptr) => unsafe {
                lv_gif_set_src(self.as_mut(), ptr);
            },
            ImageSrc::ImageDsc(dsc) => unsafe {
                lv_gif_set_src(self.as_mut(), dsc.dsc as _);
            },
        }
        self
    }

    pub fn set_offset_x(&mut self, offset: i32) -> &mut Self {
        unsafe {
            lv_image_set_offset_x(self.as_mut(), offset);
        }
        self
    }

    pub fn set_offset_y(&mut self, offset: i32) -> &mut Self {
        unsafe {
            lv_image_set_offset_y(self.as_mut(), offset);
        }
        self
    }

    pub fn set_pivot(&mut self, x: i32, y: i32) -> &mut Self {
        unsafe {
            lv_image_set_pivot(self.as_mut(), x, y);
        }
        self
    }

    pub fn get_scale(&self) -> i32 {
        unsafe { lv_image_get_scale(self.as_ptr()) }
    }

    pub fn set_scale(&mut self, zoom: u32) -> &mut Self {
        unsafe {
            lv_image_set_scale(self.as_mut(), zoom);
        }
        self
    }

    pub fn set_scale_x(&mut self, zoom: u32) -> &mut Self {
        unsafe {
            lv_image_set_scale_x(self.as_mut(), zoom);
        }
        self
    }

    pub fn set_scale_y(&mut self, zoom: u32) -> &mut Self {
        unsafe {
            lv_image_set_scale_y(self.as_mut(), zoom);
        }
        self
    }

    /// Set the blend mode of an image.
    pub fn set_blend_mode(&mut self, mode: BlendMode) -> &mut Self {
        unsafe {
            lv_image_set_blend_mode(self.as_mut(), mode as _);
        }
        self
    }

    /// Enable/disable anti-aliasing for the transformations (rotate, zoom) or not. The quality is better with anti-aliasing looks better but slower.
    pub fn set_antialias(&mut self, antialias: bool) -> &mut Self {
        unsafe {
            lv_image_set_antialias(self.as_mut(), antialias as _);
        }
        self
    }

    /// Set the image object size mode.
    pub fn set_inner_align(&mut self, align: ImageAlign) -> &mut Self {
        unsafe {
            lv_image_set_inner_align(self.as_mut(), align as _);
        }
        self
    }
}
