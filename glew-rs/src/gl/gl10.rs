use crate::gl::enums::*;
use crate::types::*;

pub trait GL10 {
    fn entry_gl10(&self) -> &crate::gl::feature::EntryFnGL10;

    unsafe fn glAccum(&self, _op: AccumOp, _value: GLfloat) {
        self.entry_gl10().glAccum(_op, _value);
    }
}
