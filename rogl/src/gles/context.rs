use crate::gles::feature::EntryGLESFn;
use libloading::Library;
use std::ffi::{c_void, OsStr};
use std::ptr;
use std::sync::Arc;

#[derive(Clone)]
pub struct GLESContext {
    pub(crate) entry: crate::gles::feature::EntryGLESFn,

    #[cfg(feature = "loaded")]
    _lib_guard: Option<Arc<Library>>,
}

impl GLESContext {
    pub unsafe fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            entry: EntryGLESFn::load(_f),
            _lib_guard: None,
        }
    }

    #[cfg(feature = "loaded")]
    pub unsafe fn load_from(path: impl AsRef<OsStr>) -> Self {
        let lib: Arc<Library> = Library::new(path).map(Arc::new).unwrap();

        Self {
            entry: EntryGLESFn::load(|name| {
                lib.get(name.to_bytes_with_nul())
                    .map(|symbol| *symbol)
                    .unwrap_or(ptr::null_mut())
            }),
            _lib_guard: Some(lib),
        }
    }
}
