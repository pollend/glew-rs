use crate::gl::feature::EntryGLFn;
use libloading::Library;
use std::ffi::{c_void, OsStr};
use std::ptr;
use std::sync::Arc;

// #[allow(non_camel_case_types)]
// type PFN_LoadFunction = fn(&::std::ffi::CStr) -> *const c_void;
//
// #[derive(Clone)]
// pub struct LoadEntryPoint {
//     pub glGetProcAddress: Option<PFN_LoadFunction>,
//     pub get_version_string: Option<PFN_glGetString>,
// }
//
// impl LoadEntryPoint {
//     pub unsafe fn load_checked<F>(mut _f: F) -> Self
//     where
//         F: FnMut(&::std::ffi::CStr) -> *const c_void,
//     {
//         LoadEntryPoint {
//             glGetProcAddress: Some(_f),
//             get_version_string: unsafe {
//                 static GL_VERSION_STRING: &[u8] = b"glGetString\0";
//                 let cname = CStr::from_bytes_with_nul_unchecked(GL_VERSION_STRING);
//                 let val = _f(cname);
//                 ::std::mem::transmute(val)
//             },
//         }
//     }
// }

#[derive(Clone)]
pub struct GLContext {
    pub(crate) entry: crate::gl::feature::EntryGLFn,

    #[cfg(feature = "loaded")]
    _lib_guard: Option<Arc<Library>>,
}

impl GLContext {
    // unsafe fn _load_internal<F>(mut load_fn: &F, guard: Option<Arc<Library>>) -> Self
    // where
    //     F: FnMut(&::std::ffi::CStr) -> *const c_void {
    //
    //     // let version = CStr::from_ptr((load_fn.get_version_string)(GL_VERSION) as *const c_char);
    //     // let version_str = version.to_str().expect("failed to resolve version");
    //     // let mut version = version_str.split('.');
    //     // let major: u16 = version.next().unwrap().parse().unwrap();
    //     // let minor: u16 = version.next().unwrap().parse().unwrap();
    //     //
    //     // let version_4_6 = (major > 4) || (major == 4 && minor >= 6);
    //     // let version_4_5 = (major == 4 && minor >= 5);
    //     // let version_4_4 = version_4_5 || (major == 4 && minor >= 4);
    //     // let version_4_3 = version_4_4 || (major == 4 && minor >= 3);
    //     // let version_4_2 = version_4_3 || (major == 4 && minor >= 2);
    //     // let version_4_1 = version_4_2 || (major == 4 && minor >= 1);
    //     // let version_4_0 = version_4_1 || (major == 4);
    //     // let version_3_3 = version_4_0 || (major == 3 && minor >= 3);
    //     // let version_3_2 = version_3_3 || (major == 3 && minor >= 2);
    //     // let version_3_1 = version_3_2 || (major == 3 && minor >= 1);
    //     // let version_3_0 = version_3_1 || (major == 3);
    //     // let version_2_1 = version_3_0 || (major == 2 && minor >= 1);
    //     // let version_2_0 = version_2_1 || (major == 2);
    //     // let version_1_5 = version_2_0 || (major == 1 && minor >= 5);
    //     // let version_1_4 = version_1_5 || (major == 1 && minor >= 4);
    //     // let version_1_3 = version_1_4 || (major == 1 && minor >= 3);
    //     // let version_1_2_1 = version_1_3;
    //     // let version_1_2 = version_1_2_1 || (major == 1 && minor >= 2);
    //     // let version_1_1 = version_1_2 || (major == 1 && minor >= 1);
    //     // let version_1_0 = version_1_1 || (major == 1 && minor >= 0);
    //     //
    //     // let load_handler = |str: &std::ffi::CStr| -> *const c_void {
    //     //     let val = (load_fn
    //     //         .glGetProcAddress
    //     //         .expect("missing glXGetProcAddressARB"))(
    //     //         str.as_ptr() as *const u8
    //     //     );
    //     //     ::std::mem::transmute(val)
    //     // };
    //     //
    //     // macro_rules! load_helper {
    //     //     ($load:expr, $x:ident, $handler_empty:expr, $handler_load:expr ) => {{
    //     //         if $load {
    //     //             $x::load($handler_load)
    //     //         } else {
    //     //             $x::load($handler_empty)
    //     //         }
    //     //     }};
    //     // }
    //
    //     Self {
    //         entry: EntryGLFn::load(&load_fn),
    //         _lib_guard: guard,
    //     }
    // }

    pub unsafe fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            entry: EntryGLFn::load(_f),
            _lib_guard: None,
        }
    }

    #[cfg(feature = "loaded")]
    pub unsafe fn load_from(path: impl AsRef<OsStr>) -> Self {
        let lib: Arc<Library> = Library::new(path).map(Arc::new).unwrap();

        Self {
            entry: EntryGLFn::load(|name| {
                lib.get(name.to_bytes_with_nul())
                    .map(|symbol| *symbol)
                    .unwrap_or(ptr::null_mut())
            }),
            _lib_guard: Some(lib),
        }
    }
}
