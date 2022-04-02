pub mod api;
use crate::gles::context::GLESContext;
use crate::gles::feature::EntryGLESFn;
pub use api::*;

impl GLES20 for GLESContext {
    unsafe fn entry(&self) -> &EntryGLESFn {
        &self.entry
    }
}
