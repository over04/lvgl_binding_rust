use lvgl_sys::{
    lv_anim_path_bounce, lv_anim_path_cb_t, lv_anim_path_custom_bezier3, lv_anim_path_ease_in,
    lv_anim_path_ease_in_out, lv_anim_path_ease_out, lv_anim_path_linear, lv_anim_path_overshoot,
    lv_anim_path_step,
};

pub type AnimExecCb<V> = fn(i32, &mut V);
pub type AnimCompletedCb = fn();

pub enum AnimPath {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Bounce,
    Overshoot,
    Step,
    CustomBezier3,
}

pub enum AnimRepeat {
    Count(u32),
    Infinity,
}

impl AnimPath {
    pub fn to_anim_path_cb(self) -> lv_anim_path_cb_t {
        match self {
            AnimPath::Linear => Some(lv_anim_path_linear),
            AnimPath::EaseIn => Some(lv_anim_path_ease_in),
            AnimPath::EaseOut => Some(lv_anim_path_ease_out),
            AnimPath::EaseInOut => Some(lv_anim_path_ease_in_out),
            AnimPath::Bounce => Some(lv_anim_path_bounce),
            AnimPath::Overshoot => Some(lv_anim_path_overshoot),
            AnimPath::Step => Some(lv_anim_path_step),
            AnimPath::CustomBezier3 => Some(lv_anim_path_custom_bezier3),
        }
    }
}

#[derive(Default, Clone)]
pub struct AnimSetting {
    pub completed_cb: Option<AnimCompletedCb>,
}

pub struct AnimData<V> {
    pub exec_cb: AnimExecCb<V>,
    pub var: V,
    pub setting: *mut AnimSetting,
}

impl<V> Drop for AnimData<V> {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(self.setting) };
    }
}
