use std::fmt;
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GLXAttribute(pub(crate) u64);
impl GLXAttribute {
    pub const GLX_TRANSPARENT_TYPE: Self = Self(0x23);
    pub const GLX_DOUBLEBUFFER: Self = Self(5);
    pub const GLX_TRANSPARENT_BLUE_VALUE: Self = Self(0x27);
    pub const GLX_STENCIL_SIZE: Self = Self(13);
    pub const GLX_GREEN_SIZE: Self = Self(9);
    pub const GLX_TRANSPARENT_GREEN_VALUE_EXT: Self = Self(0x26);
    pub const GLX_TRANSPARENT_INDEX_VALUE_EXT: Self = Self(0x24);
    pub const GLX_RED_SIZE: Self = Self(8);
    pub const GLX_VISUAL_CAVEAT_EXT: Self = Self(0x20);
    pub const GLX_AUX_BUFFERS: Self = Self(7);
    pub const GLX_BUFFER_SIZE: Self = Self(2);
    pub const GLX_TRANSPARENT_RED_VALUE_EXT: Self = Self(0x25);
    pub const GLX_STEREO: Self = Self(6);
    pub const GLX_DEPTH_SIZE: Self = Self(12);
    pub const GLX_TRANSPARENT_RED_VALUE: Self = Self(0x25);
    pub const GLX_CONFIG_CAVEAT: Self = Self(0x20);
    pub const GLX_TRANSPARENT_ALPHA_VALUE: Self = Self(0x28);
    pub const GLX_TRANSPARENT_BLUE_VALUE_EXT: Self = Self(0x27);
    pub const GLX_ACCUM_ALPHA_SIZE: Self = Self(17);
    pub const GLX_TRANSPARENT_GREEN_VALUE: Self = Self(0x26);
    pub const GLX_RGBA: Self = Self(4);
    pub const GLX_TRANSPARENT_ALPHA_VALUE_EXT: Self = Self(0x28);
    pub const GLX_TRANSPARENT_TYPE_EXT: Self = Self(0x23);
    pub const GLX_ACCUM_BLUE_SIZE: Self = Self(16);
    pub const GLX_ACCUM_RED_SIZE: Self = Self(14);
    pub const GLX_USE_GL: Self = Self(1);
    pub const GLX_ACCUM_GREEN_SIZE: Self = Self(15);
    pub const GLX_X_VISUAL_TYPE_EXT: Self = Self(0x22);
    pub const GLX_LEVEL: Self = Self(3);
    pub const GLX_ALPHA_SIZE: Self = Self(11);
    pub const GLX_BLUE_SIZE: Self = Self(10);
    pub const GLX_TRANSPARENT_INDEX_VALUE: Self = Self(0x24);
    pub const GLX_X_VISUAL_TYPE: Self = Self(0x22);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpecialNumbers(pub(crate) u64);
impl SpecialNumbers {
    pub const GLX_CONTEXT_RELEASE_BEHAVIOR_NONE_ARB: Self = Self(0);
    pub const GLX_DONT_CARE: Self = Self(0xffff_ffff);
}
