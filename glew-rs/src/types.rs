use std::ffi::c_void;
use std::os::raw::{c_char, c_double, c_float, c_int, c_long, c_uchar, c_uint, c_ulong, c_ushort};

pub type GLenum = c_uint;
pub type GLboolean = c_uchar;
pub type GLbitfield = c_uint;
pub type GLvoid = *mut c_void;
pub type GLbyte = i8;
pub type GLubyte = u8;
pub type GLshort = i16;
pub type GLushort = u16;
pub type GLhalfNV = u16;
pub type GLhalf = u16;
pub type GLint = c_int;
pub type GLuint = c_uint;
pub type GLclampx = i32;
pub type GLsizei = c_int;
pub type GLfloat = c_float;
pub type GLclampf = c_float;
pub type GLdouble = c_double;
pub type GLclampd = c_double;
pub type GLeglClientBufferEXT = *mut c_void;
pub type GLeglImageOES = *mut c_void;
pub type GLsync = *mut c_void;
pub type GLvdpauSurfaceNV = *mut c_void;
pub type GLchar = c_char;
pub type GLcharARB = c_char;

pub type HDC = c_int;

pub type CLContext = *mut c_void;
pub type CLEvent = *mut c_void;
pub type FLOAT = c_float;
pub type LPVOID = *mut c_void;
pub type GLVULKANPROCNV  = *mut c_void;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub type GLhandleARB = *mut c_void;
#[cfg(not(any(target_os = "macos", target_os = "ios")))]
pub type GLhandleARB = c_uint;

pub type GLhalfARB = u16;
pub type GLfixed = i32;
pub type GLintptr = isize;
pub type GLintptrARB = isize;
pub type GLsizeiptr = isize;
pub type GLsizeiptrARB = isize;
pub type GLint64 = i64;
pub type GLint64EXT = i64;
pub type GLuint64 = u64;
pub type GLuint64EXT = u64;

pub type USHORT = c_ushort;
pub type UINT = c_uint;
pub type INT = c_int;
pub type INT64 = c_long;
pub type BOOL = bool;
pub type INT32 = c_int;

pub type GLDEBUGPROC = fn(
    source: GLenum,
    _type: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *mut GLchar,
    userParam: *mut c_void,
);
pub type GLDEBUGPROCARB = fn(
    source: GLenum,
    _type: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *mut GLchar,
    userParam: *mut c_void,
);
pub type GLDEBUGPROCAMD = fn(
    source: GLenum,
    _type: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *mut GLchar,
    userParam: *mut c_void,
);
pub type GLDEBUGPROCKHR = fn(
    source: GLenum,
    _type: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *mut GLchar,
    userParam: *mut c_void,
);
