use lvgl_base::typing::anim::{AnimCompletedCb, AnimExecCb, AnimRepeat};
use lvgl_base::typing::anim::{AnimData, AnimSetting};
use lvgl_sys::{
    LV_ANIM_REPEAT_INFINITE, lv_anim_get_user_data, lv_anim_set_delay, lv_anim_set_duration,
    lv_anim_set_repeat_count, lv_anim_set_reverse_duration, lv_anim_set_user_data, lv_anim_set_var,
    lv_anim_start, lv_anim_t,
};
use std::ffi::c_void;

pub struct Anim {
    anim: lv_anim_t,
    setting: AnimSetting,
}

impl Anim {
    pub fn create() -> Self {
        Anim {
            anim: Default::default(),
            setting: AnimSetting::default(),
        }
    }

    pub fn on_completed(&mut self, cb: AnimCompletedCb) -> &mut Self {
        self.setting.completed_cb = Some(cb);
        self
    }

    pub fn duration(&mut self, duration: u32) -> &mut Self {
        unsafe {
            lv_anim_set_duration(&mut self.anim, duration);
        }
        self
    }

    pub fn reverse_duration(&mut self, duration: u32) -> &mut Self {
        unsafe {
            lv_anim_set_reverse_duration(&mut self.anim, duration);
        }
        self
    }

    pub fn delay(&mut self, delay: u32) -> &mut Self {
        unsafe {
            lv_anim_set_delay(&mut self.anim, delay);
        }
        self
    }

    pub fn repeat(&mut self, repeat: AnimRepeat) -> &mut Self {
        unsafe {
            match repeat {
                AnimRepeat::Count(count) => lv_anim_set_repeat_count(&mut self.anim, count),
                AnimRepeat::Infinity => {
                    lv_anim_set_repeat_count(&mut self.anim, LV_ANIM_REPEAT_INFINITE)
                }
            }
        }
        self
    }

    pub fn start<V>(&mut self, var: V, exec_cb: AnimExecCb<V>) {
        let setting = Box::leak(Box::new(self.setting.clone()));
        let data = Box::leak(Box::new(AnimData {
            exec_cb,
            var,
            setting,
        })) as *mut AnimData<_>;
        unsafe {
            self.anim.exec_cb = Some(anim_exec_cb::<V>);
            self.anim.completed_cb = Some(anim_completed_cb::<V>);
            lv_anim_set_user_data(&mut self.anim, data as _);
            lv_anim_set_var(&mut self.anim, data as _);
            lv_anim_start(&mut self.anim);
        }
    }
}

unsafe extern "C" fn anim_exec_cb<V>(var: *mut c_void, val: i32) {
    let data = &mut *(var as *mut AnimData<V>);
    (data.exec_cb)(val, &mut data.var);
}

unsafe extern "C" fn anim_completed_cb<V>(anim: *mut lv_anim_t) {
    let data = &mut *(lv_anim_get_user_data(anim) as *mut AnimData<V>);
    if let Some(completed_cb) = &(&*data.setting).completed_cb {
        completed_cb();
    }
    let _ = Box::from_raw(data);
}
