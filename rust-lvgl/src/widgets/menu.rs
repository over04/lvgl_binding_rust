use std::{ffi::CString, ptr::null_mut};

use libc::c_char;
use rust_lvgl_base::obj::{LvObj, LvObjCreator, LvObjPtr, Obj};
use rust_lvgl_macro::lvgl_obj;
use rust_lvgl_sys::{
    lv_menu_cont_create, lv_menu_create, lv_menu_get_main_header_back_button, lv_menu_page_create,
    lv_menu_section_create, lv_menu_set_load_page_event, lv_menu_set_page,
    lv_menu_set_sidebar_page,
};

use crate::widgets::button::Button;

pub trait LvMenuCreator
where
    Self: LvObjPtr,
{
    fn section_create(&self) -> MenuSection {
        MenuSection {
            _lv_obj_ptr: unsafe { lv_menu_section_create(self.as_ptr()) },
        }
    }

    fn cont_create(&self) -> MenuCont {
        MenuCont {
            _lv_obj_ptr: unsafe { lv_menu_cont_create(self.as_ptr()) },
        }
    }
}

#[lvgl_obj]
pub struct Menu {}

impl LvObjCreator for Menu {
    fn create(parent: &dyn LvObjPtr) -> Self {
        unsafe {
            Self {
                _lv_obj_ptr: lv_menu_create(parent.as_ptr()),
            }
        }
    }
}

impl LvMenuCreator for Menu {}

impl Menu {
    pub fn page_create(&self, title: Option<&str>) -> MenuPage {
        let title: Option<*mut c_char> = title.map(|f| CString::new(f).unwrap().into_raw());
        let page = MenuPage {
            _title: title,
            _lv_obj_ptr: unsafe { lv_menu_page_create(self.as_ptr(), title.unwrap_or(null_mut())) },
        };
        page
    }

    pub fn set_sidebar_page(&mut self, page: &MenuPage) -> &mut Self {
        unsafe {
            lv_menu_set_sidebar_page(self.as_mut(), page.as_ptr());
        }
        self
    }

    pub fn set_page(&mut self, page: &MenuPage) -> &mut Self {
        unsafe {
            lv_menu_set_page(self.as_mut(), page.as_ptr());
        }
        self
    }

    pub fn set_load_page_event(&mut self, obj: &mut dyn LvObjPtr, page: &MenuPage) -> &mut Self {
        unsafe {
            lv_menu_set_load_page_event(self.as_mut(), obj.as_mut(), page.as_ptr());
        }
        self
    }

    pub fn get_main_header_back_button(&self) -> Button {
        Button {
            _lv_obj_ptr: unsafe { lv_menu_get_main_header_back_button(self.as_ptr()) },
        }
    }

    pub fn get_main_header_cont_class(&self) -> Obj {
        Obj::from_raw(unsafe { lv_menu_get_main_header_back_button(self.as_ptr()) })
    }
}

#[lvgl_obj]
pub struct MenuPage {
    _title: Option<*mut c_char>,
}

impl LvMenuCreator for MenuPage {}

#[lvgl_obj]
pub struct MenuSection {}

impl LvMenuCreator for MenuSection {}

#[lvgl_obj]
pub struct MenuCont {}

impl LvMenuCreator for MenuCont {}
