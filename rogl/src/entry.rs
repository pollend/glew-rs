use libloading::Library;
use std::ffi::OsStr;
use std::os::unix::raw::time_t;
use std::ptr;
use std::sync::Arc;

struct LoadFn {}

#[derive(Clone)]
pub struct Entry {
    // entry_gl45: crate::gl::feature::GL45,
    // entry_gl41: crate::gl::feature::GL41,
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

impl Entry {
    // pub unsafe fn load_from(path: impl AsRef<OsStr>) -> Result<Self, ()> {
    //     let lib = Library::new(path)
    //         .map_err(LoadingError::LibraryLoadFailure)
    //         .map(Arc::new)?;
    //
    //     let static_fn = vk::StaticFn::load_checked(|name| {
    //         lib.get(name.to_bytes_with_nul())
    //             .map(|symbol| *symbol)
    //             .unwrap_or(ptr::null_mut())
    //     })?;
    //
    //     Ok(Self {
    //         _lib_guard: Some(lib),
    //         ..Self::from_static_fn(static_fn)
    //     })
    // }
}
