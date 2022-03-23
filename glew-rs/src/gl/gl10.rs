use crate::gl::enums::*;
use crate::types::*;

pub trait GL10 {
    fn entry_gl10(&self) -> &crate::gl::feature::EntryFnGL10;
}
