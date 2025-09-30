use std::ffi::CString;

use rust_lvgl_base::{obj::{LvObjCreator, LvObjPtr}, typing::{image::ImageAlign, style::BlendMode}};
use rust_lvgl_macro::lvgl_obj;
use rust_lvgl_sys::{
    lv_image_get_scale, lv_image_set_antialias, lv_image_set_blend_mode, lv_image_set_inner_align, lv_image_set_offset_x, lv_image_set_offset_y, lv_image_set_pivot, lv_image_set_scale, lv_image_set_scale_x, lv_image_set_scale_y, lv_lottie_create, lv_lottie_set_buffer, lv_lottie_set_src_file
};

#[lvgl_obj]
pub struct Lottie {
    buffer: Option<Box<[u8]>>,
    path: Option<CString>,
}

impl LvObjCreator for Lottie {
    fn create(parent: &dyn LvObjPtr) -> Self {
        Self {
            _lv_obj_ptr: unsafe { lv_lottie_create(parent.as_ptr()) },
            buffer: None,
            path: None,
        }
    }
}

impl Lottie {
    pub fn set_src_file(&mut self, path: &str) -> &mut Self {
        self.path = Some(CString::new(path).unwrap());
        unsafe {
            lv_lottie_set_src_file(self.as_ptr(), self.path.as_ref().unwrap().as_ptr());
        }
        self
    }

    pub fn set_buffer(&mut self, w: usize, h: usize, pixel_stride: u8) -> &mut Self {
        self.buffer =
            Some(vec![0; w as usize * h as usize * pixel_stride as usize].into_boxed_slice());
        unsafe {
            lv_lottie_set_buffer(
                self.as_ptr(),
                w as _,
                h as _,
                self.buffer.as_mut().unwrap().as_mut_ptr() as *mut _,
            )
        };
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
