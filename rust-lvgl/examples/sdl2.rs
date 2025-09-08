fn main() {
    #[cfg(feature = "sdl2")]
    {
        use rust_lvgl::base::driver::{DisplayDriver, DisplayDriverBase, IndevDriver};
        use rust_lvgl::driver::sdl2::{SDL2Display, SDL2Mouth};
        use rust_lvgl::layer::LvObjLayer;
        use rust_lvgl::widgets::anim::Anim;
        use rust_lvgl_base::obj::{LvObj, LvObjEvent, LvObjEventData};
        use rust_lvgl_base::obj::{LvObjCreator, Obj};
        use rust_lvgl_base::typing::event::EventCode;
        use rust_lvgl_base::typing::size::Length;

        let mut display = SDL2Display::create((480, 480));
        SDL2Mouth::create(());
        let mut a = Obj::create(&LvObjLayer::screen_active());
        a.center();
        LvObjEvent::on_event(&mut a, EventCode::Pressed, |event| {
            println!("{:?}", event);
        });
        let data = Box::leak(Box::new(1));
        let mut anim = Anim::create();
        anim.start(a.clone(), |val, obj| {
            obj.set_x(Length::Pix(val));
        });

        unsafe {
            LvObjEventData::on_event(&mut a, EventCode::Released, data, |_, data| {
                println!("{}", *data);
            });
        }
        loop {
            display.handle();
        }
        //
        //
        // unsafe {
        //     use std::{ffi::CString, ptr::null_mut};

        //     use rust_lvgl_sys::{
        //         lv_color_hex, lv_label_create, lv_label_set_text, lv_menu_cont_create,
        //         lv_menu_create, lv_menu_page_create, lv_menu_section_create,
        //         lv_menu_set_load_page_event, lv_menu_set_page, lv_menu_set_sidebar_page,
        //         lv_obj_create, lv_obj_set_size, lv_obj_set_style_bg_color, lv_pct,
        //         lv_screen_active,
        //     };

        //     let menu = lv_menu_create(lv_screen_active());
        //     lv_obj_set_size(menu, lv_pct(100), lv_pct(100));

        //     let main_page = lv_menu_page_create(menu, null_mut());
        //     let label0 = lv_label_create(main_page);

        //     let sidebar_page = lv_menu_page_create(menu, null_mut());
        //     lv_menu_set_sidebar_page(menu, sidebar_page);

        //     // lv_menu_set_page(menu, main_page);

        //     let section = lv_menu_section_create(sidebar_page);
        //     let cont1 = lv_menu_cont_create(section);
        //     let label1 = lv_label_create(cont1);
        //     let cont2 = lv_menu_cont_create(section);
        //     let label2 = lv_label_create(cont2);
        //     lv_label_set_text(label1, CString::new("123").unwrap().into_raw());
        //     lv_label_set_text(label2, CString::new("1234").unwrap().into_raw());
        //     lv_menu_set_load_page_event(menu, cont1, main_page);
        // }
    }
    #[cfg(all(feature = "fbdev", feature = "evdev"))]
    {
        use rust_lvgl::driver::evdev::EVDev;
        use rust_lvgl::driver::fbdev::FBDev;
        use rust_lvgl_base::driver::{DisplayDriver, DisplayDriverBase, IndevDriver};
        use rust_lvgl_base::typing::indev::IndevType;
        let mut display = FBDev::create("/dev/fb0");
        let mut indev = EVDev::create((IndevType::Pointer, "/dev/input/event0"));
        indev.set_display(&display);
        display.handle();
    }
}
