use std::ffi::c_void;
use std::ptr;
use libloading::Library;
use std::sync::Arc;
use crate::gl::context::LoadFn;
use crate::gl::feature::EntryFnGL45;

#[derive(Clone)]
pub struct GL45 {
    supported: bool,
    entry_gl45: crate::gl::feature::EntryFnGL45,
    // #[cfg(feature = "loaded")]
    // _lib_guard: Option<Arc<Library>>,
}

impl GL45 {
    // pub unsafe fn load(load_fn: &LoadFn) -> GL45 {
    //
    // }

    pub fn empty() -> GL45{
        let empty_load = |str: &std::ffi::CStr| -> *const c_void {
            ptr::null()
        };
        GL45 {
            supported: false,
            entry_gl45: EntryFnGL45::load(empty_load)
        }
    }

}
