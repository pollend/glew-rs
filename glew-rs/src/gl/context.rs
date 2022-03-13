use std::ffi::c_void;
use crate::gl::gl45::GL45;
use crate::types::GLubyte;
use libloading::Library;
use std::sync::Arc;

use crate::gl::command::PFN_glGetString;
use crate::glx::command::PFN_glXGetProcAddressARB;

#[derive(Clone)]
pub struct LoadFn {
    pub get_proc_address: FnMut(&::std::ffi::CStr) -> *const c_void,
    pub get_version_string: PFN_glGetString,
}

#[derive(Clone)]
pub struct GLContext {
    pub gl45: GL45,
    // entry_gl41: crate::gl::feature::GLEntry41,
    // entry_gl44: crate::gl::feature::GL44,
    // entry_gl11: crate::gl::feature::GL11,
    // entry_gl40: crate::gl::feature::GL40,
    // entry_gl13: crate::gl::feature::GL13,
    // entry_gl33: crate::gl::feature::GL33,
    // entry_gl30: crate::gl::feature::GL30,
    // entry_gl10: crate::gl::feature::GL10,
    // entry_gl42: crate::gl::feature::GL42,
    // entry_gl14: crate::gl::feature::GL14,
    // entry_gl20: crate::gl::feature::GL20,
    // entry_gl21: crate::gl::feature::GL21,
    // entry_gl31: crate::gl::feature::GL31,
    // entry_gl32: crate::gl::feature::GL32,
    // entry_gl43: crate::gl::feature::GL43,
    // entry_gl15: crate::gl::feature::GL15,
    // entry_gl12: crate::gl::feature::GL12,
    // entry_gl46: crate::gl::feature::GL46,
    #[cfg(feature = "loaded")]
    _lib_guard: Option<Arc<Library>>,
}

impl GLContext {
    pub unsafe fn load(load_fn: &LoadFn) {

        // GLContext {
        //     gl45: GL45::load(load_fn),
        //     _lib_guard: None
        // }
    }

}


// fn test() {
//     let mut c: GLContext = GLContext{ gl45: (), _lib_guard: None }
//     c.gl45.method();
// }
