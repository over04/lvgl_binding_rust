fn main() {
    #[cfg(feature = "sdl2")]
    {
        use rust_lvgl::base::driver::{DisplayDriver, IndevDriver};
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
        display.handle();
    }
    #[cfg(all(feature = "fbdev", feature = "evdev"))]
    {
        use lvgl::driver::DisplayDriver;
        use lvgl::driver::IndevDriver;
        use lvgl::driver::evdev::EVDev;
        use lvgl::driver::fbdev::FBDev;
        use rust_lvgl_base::typing::indev::IndevType;
        let display = FBDev::create("/dev/fb0");
        let mut indev = EVDev::create((LVIndevType::Pointer, "/dev/input/event0"));
        indev.set_display(&display);
        display.handle();
    }
}
