use std::{ffi::CString, ptr::null_mut};

use rust_lvgl_base::obj::{LvObj, LvObjCreator, LvObjPtr};
use rust_lvgl_macro::lvgl_obj;
use rust_lvgl_sys::{
    lv_menu_cont_create, lv_menu_create, lv_menu_page_create, lv_menu_section_create,
    lv_menu_set_load_page_event, lv_menu_set_page, lv_menu_set_sidebar_page,
};

pub trait LvMenuCreator
where
    Self: LvObj,
{
    fn section_create(&self) -> MenuSection {
        MenuSection::from_raw(unsafe { lv_menu_section_create(self.as_ptr()) })
    }

    fn cont_create(&self) -> MenuCont {
        MenuCont::from_raw(unsafe { lv_menu_cont_create(self.as_ptr()) })
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
        let title = title.map(|f| CString::new(f).unwrap().into_raw());
        let mut page = MenuPage::from_raw(unsafe {
            lv_menu_page_create(self.as_ptr(), title.unwrap_or(null_mut()))
        });
        page.title = title;
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
}

#[lvgl_obj]
pub struct MenuPage {
    title: Option<*mut i8>,
}

impl LvObjCreator for MenuPage {
    fn create(parent: &dyn LvObjPtr) -> Self {
        unsafe {
            Self {
                _lv_obj_ptr: lv_menu_page_create(parent.as_ptr(), null_mut()),
                title: None,
            }
        }
    }
}

impl LvMenuCreator for MenuPage {}

impl MenuPage {
    pub fn cont_create(&self) -> MenuCont {
        MenuCont::from_raw(unsafe { lv_menu_cont_create(self.as_ptr()) })
    }
}

#[lvgl_obj]
pub struct MenuSection {}

impl LvMenuCreator for MenuSection {}

impl LvObjCreator for MenuSection {
    fn create(parent: &dyn LvObjPtr) -> Self {
        unsafe {
            Self {
                _lv_obj_ptr: lv_menu_section_create(parent.as_ptr()),
            }
        }
    }
}

#[lvgl_obj]
pub struct MenuCont {}

impl LvMenuCreator for MenuCont {}

impl LvObjCreator for MenuCont {
    fn create(parent: &dyn LvObjPtr) -> Self {
        unsafe {
            Self {
                _lv_obj_ptr: lv_menu_cont_create(parent.as_ptr()),
            }
        }
    }
}
