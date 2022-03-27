use crate::types::GLubyte;
use libloading::Library;
use std::cmp::min;
use std::ffi::{c_void, CStr, OsStr};
use std::os::raw::c_char;
use std::ptr;
use std::str::Utf8Error;
use std::sync::Arc;
use crate::gl::feature::EntryGLFn;

use crate::gl::command::PFN_glGetString;
use crate::gl::enums::StringName;

#[cfg(target_os = "linux")]
use crate::glx::command::PFN_glXGetProcAddressARB;
#[cfg(target_os = "windows")]
use crate::wgl::command::PFN_wglGetProcAddress;

type PFN_LoadFunction = fn(&::std::ffi::CStr) -> *const c_void;

#[derive(Clone)]
pub struct LoadEntryPoint {
    #[cfg(target_os = "windows")]
    pub wglGetProcAddress: Option<PFN_wglGetProcAddress>,
    #[cfg(target_os = "linux")]
    pub glXGetProcAddressARB: Option<PFN_glXGetProcAddressARB>,
    pub get_version_string: PFN_glGetString,
}

impl LoadEntryPoint {
    pub unsafe fn load_checked<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        LoadEntryPoint {
            #[cfg(target_os = "windows")]
            wglGetProcAddress: Some({
                static WGL_GET_PROC_ADDRESS: &[u8] = b"wglGetProcAddress\0";
                let cname = CStr::from_bytes_with_nul_unchecked(WGL_GET_PROC_ADDRESS);
                let val = _f(cname);
                ::std::mem::transmute(val)
            }),
            #[cfg(target_os = "linux")]
            glXGetProcAddressARB: unsafe {
                static GLX_GET_PROC_ADDRESS: &[u8] = b"glXGetProcAddressARB\0";
                let cname = CStr::from_bytes_with_nul_unchecked(GLX_GET_PROC_ADDRESS);
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_version_string: unsafe {
                static GL_VERSION_STRING: &[u8] = b"glGetString\0";
                let cname = CStr::from_bytes_with_nul_unchecked(GL_VERSION_STRING);
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
        }
    }
}

#[derive(Clone)]
pub struct GLContext {
    pub(crate) entry: crate::gl::feature::EntryGLFn,

    #[cfg(feature = "loaded")]
    _lib_guard: Option<Arc<Library>>,
}

impl GLContext {
    unsafe fn _load_internal(load_fn: &LoadEntryPoint, guard: Option<Arc<Library>>) -> Self {
        let version =
            CStr::from_ptr((load_fn.get_version_string)(StringName::GL_VERSION) as *const c_char);
        let version_str = version.to_str().expect("failed to resolve version");
        let mut version = version_str.split('.');
        let major: u16 = version.next().unwrap().parse().unwrap();
        let minor: u16 = version.next().unwrap().parse().unwrap();

        let version_4_6 = (major > 4) || (major == 4 && minor >= 6);
        let version_4_5 = (major == 4 && minor >= 5);
        let version_4_4 = version_4_5 || (major == 4 && minor >= 4);
        let version_4_3 = version_4_4 || (major == 4 && minor >= 3);
        let version_4_2 = version_4_3 || (major == 4 && minor >= 2);
        let version_4_1 = version_4_2 || (major == 4 && minor >= 1);
        let version_4_0 = version_4_1 || (major == 4);
        let version_3_3 = version_4_0 || (major == 3 && minor >= 3);
        let version_3_2 = version_3_3 || (major == 3 && minor >= 2);
        let version_3_1 = version_3_2 || (major == 3 && minor >= 1);
        let version_3_0 = version_3_1 || (major == 3);
        let version_2_1 = version_3_0 || (major == 2 && minor >= 1);
        let version_2_0 = version_2_1 || (major == 2);
        let version_1_5 = version_2_0 || (major == 1 && minor >= 5);
        let version_1_4 = version_1_5 || (major == 1 && minor >= 4);
        let version_1_3 = version_1_4 || (major == 1 && minor >= 3);
        let version_1_2_1 = version_1_3;
        let version_1_2 = version_1_2_1 || (major == 1 && minor >= 2);
        let version_1_1 = version_1_2 || (major == 1 && minor >= 1);
        let version_1_0 = version_1_1 || (major == 1 && minor >= 0);

        let load_handler = |str: &std::ffi::CStr| -> *const c_void {
            #[cfg(target_os = "linux")]
            let val = (load_fn
                .glXGetProcAddressARB
                .expect("missing glXGetProcAddressARB"))(
                str.as_ptr() as *const u8
            );
            ::std::mem::transmute(val)
        };

        macro_rules! load_helper {
            ($load:expr, $x:ident, $handler_empty:expr, $handler_load:expr ) => {{
                if $load {
                    $x::load($handler_load)
                } else {
                    $x::load($handler_empty)
                }
            }};
        }

        Self {
            entry: EntryGLFn::load(load_handler),
            _lib_guard: guard,
        }
    }

    pub unsafe fn load(load_fn: &LoadEntryPoint) -> Self {
        Self::_load_internal(load_fn, None)
    }

    #[cfg(feature = "loaded")]
    pub unsafe fn load_from(path: impl AsRef<OsStr>) -> Self {
        let lib: Arc<Library> = Library::new(path).map(Arc::new).unwrap();

        let static_fn = LoadEntryPoint::load_checked(|name| {
            lib.get(name.to_bytes_with_nul())
                .map(|symbol| *symbol)
                .unwrap_or(ptr::null_mut())
        });

        Self::_load_internal(&static_fn, Some(lib))
    }
}
