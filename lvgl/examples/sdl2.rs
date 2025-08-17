fn main() {
    #[cfg(feature = "sdl2")]
    {
        use lvgl::driver::sdl2::{SDL2Display, SDL2Mouth};
        use lvgl::driver::{DisplayDriver, IndevDriver};
        use lvgl::layer::LvObjLayer;
        use lvgl::widgets::obj::Obj;
        use lvgl_base::obj::{LvObj, LvObjEvent, LvObjEventData};
        use lvgl_base::typing::event::EventCode;
        let mut display = SDL2Display::create((480, 480));
        SDL2Mouth::create(());
        let mut a = Obj::create(&LvObjLayer::screen_active());
        LvObjEvent::on_event(&mut a, EventCode::Pressed, |event| {
            println!("{:?}", event);
        });
        let data = Box::leak(Box::new(1));
        unsafe {
            LvObjEventData::on_event(&mut a, EventCode::Released, data, |_, data| {
                println!("{}", *data);
            });
        }
        display.handle();
    }
}
