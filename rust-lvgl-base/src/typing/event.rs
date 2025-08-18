#![allow(unsafe_op_in_unsafe_fn)]

use core::ffi::c_void;
use core::fmt::Debug;
use rust_lvgl_sys::{lv_event_get_code, lv_event_get_user_data, lv_event_t};

pub type EventCbWithData<T> = fn(Event, *mut T);
pub type EventCb = fn(Event);

#[derive(Debug)]
#[repr(u32)]
pub enum EventCode {
    All = 0,
    Pressed,
    Pressing,
    PressLost,
    ShortClicked,
    SingleClicked,
    DoubleClicked,
    TripleClicked,
    LongPressed,
    LongPressedRepeat,
    Clicked,
    Released,
    ScrollBegin,
    ScrollThrowBegin,
    ScrollEnd,
    Scroll,
    Gesture,
    Key,
    Rotary,
    Focused,
    Defocused,
    Leave,
    HitTest,
    IndevReset,
    HoverOver,
    HoverLeave,
    CoverCheck,
    RefrExtDrawSize,
    DrawMainBegin,
    DrawMain,
    DrawMainEnd,
    DrawPostBegin,
    DrawPost,
    DrawPostEnd,
    DrawTaskAdded,
    ValueChanged,
    Insert,
    Refresh,
    Ready,
    Cancel,
    Create,
    Delete,
    ChildChanged,
    ChildCreated,
    ChildDeleted,
    ScreenUnloadStart,
    ScreenLoadStart,
    ScreenLoaded,
    ScreenUnloaded,
    SizeChanged,
    StyleChanged,
    LayoutChanged,
    GetSelfSize,
    InvalidateArea,
    ResolutionChanged,
    ColorFormatChanged,
    RefrRequest,
    RefrStart,
    RefrReady,
    RenderStart,
    RenderReady,
    FlushStart,
    FlushFinish,
    FlushWaitStart,
    FlushWaitFinish,
    Vsync,
    VsyncRequest,
    Last,
    Preprocess = 0x8000,
    MarkedDeleting = 0x10000,
}

pub struct Event {
    lv_event: *mut lv_event_t,
}

impl Event {
    pub fn from_raw(lv_event: *mut lv_event_t) -> Self {
        Self { lv_event }
    }

    pub fn get_event_code(&self) -> EventCode {
        unsafe { core::mem::transmute(lv_event_get_code(self.lv_event)) }
    }

    pub unsafe fn get_user_data(&self) -> *mut c_void {
        lv_event_get_user_data(self.lv_event)
    }
}

impl Debug for Event {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Event: {:?}", self.get_event_code())
    }
}

pub struct EventData<T> {
    pub cb: EventCbWithData<T>,
    pub data: *mut T,
}

pub unsafe extern "C" fn event_handler_cb_with_data<T>(event: *mut lv_event_t) {
    let event = Event::from_raw(event);
    let data = event.get_user_data() as *mut EventData<T>;
    let func = (*data).cb;
    func(event, (*data).data);
}

pub unsafe extern "C" fn event_handler_cb(event: *mut lv_event_t) {
    let event = Event::from_raw(event);
    let func: EventCb = core::mem::transmute(event.get_user_data());
    func(event);
}
