pub mod api;

use crate::gl::context::GLContext;
use crate::gl::feature::EntryGLFn;
pub use api::*;

impl GL13 for GLContext {
    unsafe fn entry(&self) -> &EntryGLFn {
        &self.entry
    }
}
