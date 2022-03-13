use std::sync::Arc;
use libloading::Library;

#[derive(Clone)]
pub struct GL41 {
    entry_gl45: crate::gl::feature::EntryFnGL41,
    #[cfg(feature = "loaded")]
    _lib_guard: Option<Arc<Library>>,
}
