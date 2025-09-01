use crate::obj::LvObjPtr;
pub trait LvObjLayout<'a>
where
    Self: LvObjPtr,
{
    fn create(obj: &'a mut dyn LvObjPtr) -> Self;
}

// pub struct LvObjLayoutAlter<T: LvObj>(LvObjAlter<T>);
//
// impl<T: LvObj> LvObjLayoutAlter<T> {
//     pub fn exit(&mut self) -> LvObjAlter<T> {
//         unsafe { (self as *mut LvObjLayoutAlter<T>).read().0 }
//     }
//
//     pub fn finish(&mut self) -> T {
//         self.exit().finish()
//     }
// }
