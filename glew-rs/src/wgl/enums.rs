use std::fmt;
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpecialNumbers(pub(crate) u64);
impl SpecialNumbers {
    pub const WGL_FONT_POLYGONS: Self = Self(1);
    pub const WGL_CONTEXT_RELEASE_BEHAVIOR_NONE_ARB: Self = Self(0);
    pub const WGL_FONT_LINES: Self = Self(0);
}
