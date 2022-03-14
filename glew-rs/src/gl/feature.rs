use crate::gl;
use crate::types::*;
use gl::bitflags::*;
use gl::command::*;
use gl::enums::*;
use std::ffi::c_void;
#[derive(Clone)]
pub struct EntryFnGL20 {
    pub glVertexAttrib1dv: crate::gl::command::PFN_glVertexAttrib1dv,
    pub glCompileShader: crate::gl::command::PFN_glCompileShader,
    pub glUniform3fv: crate::gl::command::PFN_glUniform3fv,
    pub glUniform2f: crate::gl::command::PFN_glUniform2f,
    pub glGetVertexAttribiv: crate::gl::command::PFN_glGetVertexAttribiv,
    pub glValidateProgram: crate::gl::command::PFN_glValidateProgram,
    pub glUniformMatrix2fv: crate::gl::command::PFN_glUniformMatrix2fv,
    pub glVertexAttrib1f: crate::gl::command::PFN_glVertexAttrib1f,
    pub glVertexAttrib4Nbv: crate::gl::command::PFN_glVertexAttrib4Nbv,
    pub glCreateProgram: crate::gl::command::PFN_glCreateProgram,
    pub glVertexAttrib4d: crate::gl::command::PFN_glVertexAttrib4d,
    pub glVertexAttrib4s: crate::gl::command::PFN_glVertexAttrib4s,
    pub glBlendEquationSeparate: crate::gl::command::PFN_glBlendEquationSeparate,
    pub glUniform2i: crate::gl::command::PFN_glUniform2i,
    pub glVertexAttrib4usv: crate::gl::command::PFN_glVertexAttrib4usv,
    pub glDisableVertexAttribArray: crate::gl::command::PFN_glDisableVertexAttribArray,
    pub glDeleteProgram: crate::gl::command::PFN_glDeleteProgram,
    pub glGetActiveAttrib: crate::gl::command::PFN_glGetActiveAttrib,
    pub glVertexAttrib2dv: crate::gl::command::PFN_glVertexAttrib2dv,
    pub glUniform2iv: crate::gl::command::PFN_glUniform2iv,
    pub glUniform4iv: crate::gl::command::PFN_glUniform4iv,
    pub glVertexAttrib3fv: crate::gl::command::PFN_glVertexAttrib3fv,
    pub glUniform4i: crate::gl::command::PFN_glUniform4i,
    pub glVertexAttribPointer: crate::gl::command::PFN_glVertexAttribPointer,
    pub glBindAttribLocation: crate::gl::command::PFN_glBindAttribLocation,
    pub glVertexAttrib3f: crate::gl::command::PFN_glVertexAttrib3f,
    pub glVertexAttrib4Niv: crate::gl::command::PFN_glVertexAttrib4Niv,
    pub glVertexAttrib2f: crate::gl::command::PFN_glVertexAttrib2f,
    pub glDetachShader: crate::gl::command::PFN_glDetachShader,
    pub glUniform1f: crate::gl::command::PFN_glUniform1f,
    pub glVertexAttrib4uiv: crate::gl::command::PFN_glVertexAttrib4uiv,
    pub glIsProgram: crate::gl::command::PFN_glIsProgram,
    pub glVertexAttrib2sv: crate::gl::command::PFN_glVertexAttrib2sv,
    pub glCreateShader: crate::gl::command::PFN_glCreateShader,
    pub glDeleteShader: crate::gl::command::PFN_glDeleteShader,
    pub glUseProgram: crate::gl::command::PFN_glUseProgram,
    pub glUniform3i: crate::gl::command::PFN_glUniform3i,
    pub glVertexAttrib1d: crate::gl::command::PFN_glVertexAttrib1d,
    pub glVertexAttrib4Nsv: crate::gl::command::PFN_glVertexAttrib4Nsv,
    pub glGetActiveUniform: crate::gl::command::PFN_glGetActiveUniform,
    pub glStencilMaskSeparate: crate::gl::command::PFN_glStencilMaskSeparate,
    pub glGetProgramiv: crate::gl::command::PFN_glGetProgramiv,
    pub glGetUniformfv: crate::gl::command::PFN_glGetUniformfv,
    pub glUniform3f: crate::gl::command::PFN_glUniform3f,
    pub glUniform2fv: crate::gl::command::PFN_glUniform2fv,
    pub glVertexAttrib1s: crate::gl::command::PFN_glVertexAttrib1s,
    pub glVertexAttrib4Nub: crate::gl::command::PFN_glVertexAttrib4Nub,
    pub glUniform1i: crate::gl::command::PFN_glUniform1i,
    pub glVertexAttrib1sv: crate::gl::command::PFN_glVertexAttrib1sv,
    pub glUniformMatrix3fv: crate::gl::command::PFN_glUniformMatrix3fv,
    pub glLinkProgram: crate::gl::command::PFN_glLinkProgram,
    pub glGetVertexAttribdv: crate::gl::command::PFN_glGetVertexAttribdv,
    pub glUniform4fv: crate::gl::command::PFN_glUniform4fv,
    pub glVertexAttrib4Nusv: crate::gl::command::PFN_glVertexAttrib4Nusv,
    pub glVertexAttrib3s: crate::gl::command::PFN_glVertexAttrib3s,
    pub glGetAttribLocation: crate::gl::command::PFN_glGetAttribLocation,
    pub glGetAttachedShaders: crate::gl::command::PFN_glGetAttachedShaders,
    pub glGetShaderInfoLog: crate::gl::command::PFN_glGetShaderInfoLog,
    pub glStencilOpSeparate: crate::gl::command::PFN_glStencilOpSeparate,
    pub glGetUniformLocation: crate::gl::command::PFN_glGetUniformLocation,
    pub glEnableVertexAttribArray: crate::gl::command::PFN_glEnableVertexAttribArray,
    pub glIsShader: crate::gl::command::PFN_glIsShader,
    pub glVertexAttrib4fv: crate::gl::command::PFN_glVertexAttrib4fv,
    pub glGetShaderSource: crate::gl::command::PFN_glGetShaderSource,
    pub glVertexAttrib3sv: crate::gl::command::PFN_glVertexAttrib3sv,
    pub glGetShaderiv: crate::gl::command::PFN_glGetShaderiv,
    pub glVertexAttrib4bv: crate::gl::command::PFN_glVertexAttrib4bv,
    pub glVertexAttrib4iv: crate::gl::command::PFN_glVertexAttrib4iv,
    pub glGetVertexAttribPointerv: crate::gl::command::PFN_glGetVertexAttribPointerv,
    pub glUniformMatrix4fv: crate::gl::command::PFN_glUniformMatrix4fv,
    pub glVertexAttrib1fv: crate::gl::command::PFN_glVertexAttrib1fv,
    pub glDrawBuffers: crate::gl::command::PFN_glDrawBuffers,
    pub glUniform4f: crate::gl::command::PFN_glUniform4f,
    pub glGetVertexAttribfv: crate::gl::command::PFN_glGetVertexAttribfv,
    pub glGetProgramInfoLog: crate::gl::command::PFN_glGetProgramInfoLog,
    pub glUniform1fv: crate::gl::command::PFN_glUniform1fv,
    pub glShaderSource: crate::gl::command::PFN_glShaderSource,
    pub glVertexAttrib2fv: crate::gl::command::PFN_glVertexAttrib2fv,
    pub glVertexAttrib4ubv: crate::gl::command::PFN_glVertexAttrib4ubv,
    pub glUniform1iv: crate::gl::command::PFN_glUniform1iv,
    pub glVertexAttrib4sv: crate::gl::command::PFN_glVertexAttrib4sv,
    pub glVertexAttrib2s: crate::gl::command::PFN_glVertexAttrib2s,
    pub glVertexAttrib3dv: crate::gl::command::PFN_glVertexAttrib3dv,
    pub glUniform3iv: crate::gl::command::PFN_glUniform3iv,
    pub glStencilFuncSeparate: crate::gl::command::PFN_glStencilFuncSeparate,
    pub glVertexAttrib2d: crate::gl::command::PFN_glVertexAttrib2d,
    pub glVertexAttrib3d: crate::gl::command::PFN_glVertexAttrib3d,
    pub glVertexAttrib4Nuiv: crate::gl::command::PFN_glVertexAttrib4Nuiv,
    pub glVertexAttrib4Nubv: crate::gl::command::PFN_glVertexAttrib4Nubv,
    pub glGetUniformiv: crate::gl::command::PFN_glGetUniformiv,
    pub glAttachShader: crate::gl::command::PFN_glAttachShader,
    pub glVertexAttrib4dv: crate::gl::command::PFN_glVertexAttrib4dv,
    pub glVertexAttrib4f: crate::gl::command::PFN_glVertexAttrib4f,
}
impl EntryFnGL20 {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            glVertexAttrib1dv: unsafe {
                unsafe extern "system" fn __glVertexAttrib1dv(_index: GLuint, _v: *mut GLdouble) {
                    panic!("Unable to load glVertexAttrib1dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib1dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib1dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCompileShader: unsafe {
                unsafe extern "system" fn __glCompileShader(_shader: GLuint) {
                    panic!("Unable to load glCompileShader")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCompileShader\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCompileShader
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform3fv: unsafe {
                unsafe extern "system" fn __glUniform3fv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLfloat,
                ) {
                    panic!("Unable to load glUniform3fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform3fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform3fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform2f: unsafe {
                unsafe extern "system" fn __glUniform2f(
                    _location: GLint,
                    _v0: GLfloat,
                    _v1: GLfloat,
                ) {
                    panic!("Unable to load glUniform2f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform2f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform2f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetVertexAttribiv: unsafe {
                unsafe extern "system" fn __glGetVertexAttribiv(
                    _index: GLuint,
                    _pname: VertexAttribPropertyARB,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetVertexAttribiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetVertexAttribiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetVertexAttribiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glValidateProgram: unsafe {
                unsafe extern "system" fn __glValidateProgram(_program: GLuint) {
                    panic!("Unable to load glValidateProgram")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glValidateProgram\0");
                let val = _f(cname);
                if val.is_null() {
                    __glValidateProgram
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniformMatrix2fv: unsafe {
                unsafe extern "system" fn __glUniformMatrix2fv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLfloat,
                ) {
                    panic!("Unable to load glUniformMatrix2fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix2fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformMatrix2fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib1f: unsafe {
                unsafe extern "system" fn __glVertexAttrib1f(_index: GLuint, _x: GLfloat) {
                    panic!("Unable to load glVertexAttrib1f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib1f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib1f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib4Nbv: unsafe {
                unsafe extern "system" fn __glVertexAttrib4Nbv(_index: GLuint, _v: *mut GLbyte) {
                    panic!("Unable to load glVertexAttrib4Nbv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib4Nbv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib4Nbv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCreateProgram: unsafe {
                unsafe extern "system" fn __glCreateProgram() -> GLuint {
                    panic!("Unable to load glCreateProgram")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCreateProgram\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCreateProgram
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib4d: unsafe {
                unsafe extern "system" fn __glVertexAttrib4d(
                    _index: GLuint,
                    _x: GLdouble,
                    _y: GLdouble,
                    _z: GLdouble,
                    _w: GLdouble,
                ) {
                    panic!("Unable to load glVertexAttrib4d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib4d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib4d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib4s: unsafe {
                unsafe extern "system" fn __glVertexAttrib4s(
                    _index: GLuint,
                    _x: GLshort,
                    _y: GLshort,
                    _z: GLshort,
                    _w: GLshort,
                ) {
                    panic!("Unable to load glVertexAttrib4s")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib4s\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib4s
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBlendEquationSeparate: unsafe {
                unsafe extern "system" fn __glBlendEquationSeparate(
                    _modeRGB: BlendEquationModeEXT,
                    _modeAlpha: BlendEquationModeEXT,
                ) {
                    panic!("Unable to load glBlendEquationSeparate")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBlendEquationSeparate\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBlendEquationSeparate
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform2i: unsafe {
                unsafe extern "system" fn __glUniform2i(_location: GLint, _v0: GLint, _v1: GLint) {
                    panic!("Unable to load glUniform2i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform2i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform2i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib4usv: unsafe {
                unsafe extern "system" fn __glVertexAttrib4usv(_index: GLuint, _v: *mut GLushort) {
                    panic!("Unable to load glVertexAttrib4usv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib4usv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib4usv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDisableVertexAttribArray: unsafe {
                unsafe extern "system" fn __glDisableVertexAttribArray(_index: GLuint) {
                    panic!("Unable to load glDisableVertexAttribArray")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glDisableVertexAttribArray\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glDisableVertexAttribArray
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDeleteProgram: unsafe {
                unsafe extern "system" fn __glDeleteProgram(_program: GLuint) {
                    panic!("Unable to load glDeleteProgram")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDeleteProgram\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDeleteProgram
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetActiveAttrib: unsafe {
                unsafe extern "system" fn __glGetActiveAttrib(
                    _program: GLuint,
                    _index: GLuint,
                    _bufSize: GLsizei,
                    _length: *mut GLsizei,
                    _size: *mut GLint,
                    _type: *mut AttributeType,
                    _name: *mut GLchar,
                ) {
                    panic!("Unable to load glGetActiveAttrib")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetActiveAttrib\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetActiveAttrib
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib2dv: unsafe {
                unsafe extern "system" fn __glVertexAttrib2dv(_index: GLuint, _v: *mut GLdouble) {
                    panic!("Unable to load glVertexAttrib2dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib2dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib2dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform2iv: unsafe {
                unsafe extern "system" fn __glUniform2iv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLint,
                ) {
                    panic!("Unable to load glUniform2iv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform2iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform2iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform4iv: unsafe {
                unsafe extern "system" fn __glUniform4iv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLint,
                ) {
                    panic!("Unable to load glUniform4iv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform4iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform4iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib3fv: unsafe {
                unsafe extern "system" fn __glVertexAttrib3fv(_index: GLuint, _v: *mut GLfloat) {
                    panic!("Unable to load glVertexAttrib3fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib3fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib3fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform4i: unsafe {
                unsafe extern "system" fn __glUniform4i(
                    _location: GLint,
                    _v0: GLint,
                    _v1: GLint,
                    _v2: GLint,
                    _v3: GLint,
                ) {
                    panic!("Unable to load glUniform4i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform4i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform4i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribPointer: unsafe {
                unsafe extern "system" fn __glVertexAttribPointer(
                    _index: GLuint,
                    _size: GLint,
                    _type: VertexAttribPointerType,
                    _normalized: Boolean,
                    _stride: GLsizei,
                    _pointer: *mut GLvoid,
                ) {
                    panic!("Unable to load glVertexAttribPointer")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribPointer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribPointer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBindAttribLocation: unsafe {
                unsafe extern "system" fn __glBindAttribLocation(
                    _program: GLuint,
                    _index: GLuint,
                    _name: GLchar,
                ) {
                    panic!("Unable to load glBindAttribLocation")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindAttribLocation\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindAttribLocation
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib3f: unsafe {
                unsafe extern "system" fn __glVertexAttrib3f(
                    _index: GLuint,
                    _x: GLfloat,
                    _y: GLfloat,
                    _z: GLfloat,
                ) {
                    panic!("Unable to load glVertexAttrib3f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib3f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib3f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib4Niv: unsafe {
                unsafe extern "system" fn __glVertexAttrib4Niv(_index: GLuint, _v: *mut GLint) {
                    panic!("Unable to load glVertexAttrib4Niv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib4Niv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib4Niv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib2f: unsafe {
                unsafe extern "system" fn __glVertexAttrib2f(
                    _index: GLuint,
                    _x: GLfloat,
                    _y: GLfloat,
                ) {
                    panic!("Unable to load glVertexAttrib2f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib2f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib2f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDetachShader: unsafe {
                unsafe extern "system" fn __glDetachShader(_program: GLuint, _shader: GLuint) {
                    panic!("Unable to load glDetachShader")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDetachShader\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDetachShader
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform1f: unsafe {
                unsafe extern "system" fn __glUniform1f(_location: GLint, _v0: GLfloat) {
                    panic!("Unable to load glUniform1f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform1f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform1f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib4uiv: unsafe {
                unsafe extern "system" fn __glVertexAttrib4uiv(_index: GLuint, _v: *mut GLuint) {
                    panic!("Unable to load glVertexAttrib4uiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib4uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib4uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glIsProgram: unsafe {
                unsafe extern "system" fn __glIsProgram(_program: GLuint) -> GLboolean {
                    panic!("Unable to load glIsProgram")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIsProgram\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIsProgram
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib2sv: unsafe {
                unsafe extern "system" fn __glVertexAttrib2sv(_index: GLuint, _v: *mut GLshort) {
                    panic!("Unable to load glVertexAttrib2sv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib2sv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib2sv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCreateShader: unsafe {
                unsafe extern "system" fn __glCreateShader(_type: ShaderType) -> GLuint {
                    panic!("Unable to load glCreateShader")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCreateShader\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCreateShader
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDeleteShader: unsafe {
                unsafe extern "system" fn __glDeleteShader(_shader: GLuint) {
                    panic!("Unable to load glDeleteShader")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDeleteShader\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDeleteShader
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUseProgram: unsafe {
                unsafe extern "system" fn __glUseProgram(_program: GLuint) {
                    panic!("Unable to load glUseProgram")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUseProgram\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUseProgram
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform3i: unsafe {
                unsafe extern "system" fn __glUniform3i(
                    _location: GLint,
                    _v0: GLint,
                    _v1: GLint,
                    _v2: GLint,
                ) {
                    panic!("Unable to load glUniform3i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform3i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform3i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib1d: unsafe {
                unsafe extern "system" fn __glVertexAttrib1d(_index: GLuint, _x: GLdouble) {
                    panic!("Unable to load glVertexAttrib1d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib1d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib1d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib4Nsv: unsafe {
                unsafe extern "system" fn __glVertexAttrib4Nsv(_index: GLuint, _v: *mut GLshort) {
                    panic!("Unable to load glVertexAttrib4Nsv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib4Nsv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib4Nsv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetActiveUniform: unsafe {
                unsafe extern "system" fn __glGetActiveUniform(
                    _program: GLuint,
                    _index: GLuint,
                    _bufSize: GLsizei,
                    _length: *mut GLsizei,
                    _size: *mut GLint,
                    _type: *mut UniformType,
                    _name: *mut GLchar,
                ) {
                    panic!("Unable to load glGetActiveUniform")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetActiveUniform\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetActiveUniform
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glStencilMaskSeparate: unsafe {
                unsafe extern "system" fn __glStencilMaskSeparate(
                    _face: StencilFaceDirection,
                    _mask: GLuint,
                ) {
                    panic!("Unable to load glStencilMaskSeparate")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glStencilMaskSeparate\0");
                let val = _f(cname);
                if val.is_null() {
                    __glStencilMaskSeparate
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetProgramiv: unsafe {
                unsafe extern "system" fn __glGetProgramiv(
                    _program: GLuint,
                    _pname: ProgramPropertyARB,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetProgramiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetProgramiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetProgramiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetUniformfv: unsafe {
                unsafe extern "system" fn __glGetUniformfv(
                    _program: GLuint,
                    _location: GLint,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load glGetUniformfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetUniformfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetUniformfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform3f: unsafe {
                unsafe extern "system" fn __glUniform3f(
                    _location: GLint,
                    _v0: GLfloat,
                    _v1: GLfloat,
                    _v2: GLfloat,
                ) {
                    panic!("Unable to load glUniform3f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform3f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform3f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform2fv: unsafe {
                unsafe extern "system" fn __glUniform2fv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLfloat,
                ) {
                    panic!("Unable to load glUniform2fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform2fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform2fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib1s: unsafe {
                unsafe extern "system" fn __glVertexAttrib1s(_index: GLuint, _x: GLshort) {
                    panic!("Unable to load glVertexAttrib1s")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib1s\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib1s
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib4Nub: unsafe {
                unsafe extern "system" fn __glVertexAttrib4Nub(
                    _index: GLuint,
                    _x: GLubyte,
                    _y: GLubyte,
                    _z: GLubyte,
                    _w: GLubyte,
                ) {
                    panic!("Unable to load glVertexAttrib4Nub")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib4Nub\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib4Nub
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform1i: unsafe {
                unsafe extern "system" fn __glUniform1i(_location: GLint, _v0: GLint) {
                    panic!("Unable to load glUniform1i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform1i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform1i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib1sv: unsafe {
                unsafe extern "system" fn __glVertexAttrib1sv(_index: GLuint, _v: *mut GLshort) {
                    panic!("Unable to load glVertexAttrib1sv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib1sv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib1sv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniformMatrix3fv: unsafe {
                unsafe extern "system" fn __glUniformMatrix3fv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLfloat,
                ) {
                    panic!("Unable to load glUniformMatrix3fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix3fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformMatrix3fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glLinkProgram: unsafe {
                unsafe extern "system" fn __glLinkProgram(_program: GLuint) {
                    panic!("Unable to load glLinkProgram")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLinkProgram\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLinkProgram
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetVertexAttribdv: unsafe {
                unsafe extern "system" fn __glGetVertexAttribdv(
                    _index: GLuint,
                    _pname: VertexAttribPropertyARB,
                    _params: *mut GLdouble,
                ) {
                    panic!("Unable to load glGetVertexAttribdv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetVertexAttribdv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetVertexAttribdv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform4fv: unsafe {
                unsafe extern "system" fn __glUniform4fv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLfloat,
                ) {
                    panic!("Unable to load glUniform4fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform4fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform4fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib4Nusv: unsafe {
                unsafe extern "system" fn __glVertexAttrib4Nusv(_index: GLuint, _v: *mut GLushort) {
                    panic!("Unable to load glVertexAttrib4Nusv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib4Nusv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib4Nusv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib3s: unsafe {
                unsafe extern "system" fn __glVertexAttrib3s(
                    _index: GLuint,
                    _x: GLshort,
                    _y: GLshort,
                    _z: GLshort,
                ) {
                    panic!("Unable to load glVertexAttrib3s")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib3s\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib3s
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetAttribLocation: unsafe {
                unsafe extern "system" fn __glGetAttribLocation(
                    _program: GLuint,
                    _name: GLchar,
                ) -> GLint {
                    panic!("Unable to load glGetAttribLocation")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetAttribLocation\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetAttribLocation
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetAttachedShaders: unsafe {
                unsafe extern "system" fn __glGetAttachedShaders(
                    _program: GLuint,
                    _maxCount: GLsizei,
                    _count: *mut GLsizei,
                    _shaders: *mut GLuint,
                ) {
                    panic!("Unable to load glGetAttachedShaders")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetAttachedShaders\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetAttachedShaders
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetShaderInfoLog: unsafe {
                unsafe extern "system" fn __glGetShaderInfoLog(
                    _shader: GLuint,
                    _bufSize: GLsizei,
                    _length: *mut GLsizei,
                    _infoLog: *mut GLchar,
                ) {
                    panic!("Unable to load glGetShaderInfoLog")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetShaderInfoLog\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetShaderInfoLog
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glStencilOpSeparate: unsafe {
                unsafe extern "system" fn __glStencilOpSeparate(
                    _face: StencilFaceDirection,
                    _sfail: StencilOp,
                    _dpfail: StencilOp,
                    _dppass: StencilOp,
                ) {
                    panic!("Unable to load glStencilOpSeparate")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glStencilOpSeparate\0");
                let val = _f(cname);
                if val.is_null() {
                    __glStencilOpSeparate
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetUniformLocation: unsafe {
                unsafe extern "system" fn __glGetUniformLocation(
                    _program: GLuint,
                    _name: GLchar,
                ) -> GLint {
                    panic!("Unable to load glGetUniformLocation")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetUniformLocation\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetUniformLocation
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glEnableVertexAttribArray: unsafe {
                unsafe extern "system" fn __glEnableVertexAttribArray(_index: GLuint) {
                    panic!("Unable to load glEnableVertexAttribArray")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEnableVertexAttribArray\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEnableVertexAttribArray
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glIsShader: unsafe {
                unsafe extern "system" fn __glIsShader(_shader: GLuint) -> GLboolean {
                    panic!("Unable to load glIsShader")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIsShader\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIsShader
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib4fv: unsafe {
                unsafe extern "system" fn __glVertexAttrib4fv(_index: GLuint, _v: *mut GLfloat) {
                    panic!("Unable to load glVertexAttrib4fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib4fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib4fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetShaderSource: unsafe {
                unsafe extern "system" fn __glGetShaderSource(
                    _shader: GLuint,
                    _bufSize: GLsizei,
                    _length: *mut GLsizei,
                    _source: *mut GLchar,
                ) {
                    panic!("Unable to load glGetShaderSource")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetShaderSource\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetShaderSource
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib3sv: unsafe {
                unsafe extern "system" fn __glVertexAttrib3sv(_index: GLuint, _v: *mut GLshort) {
                    panic!("Unable to load glVertexAttrib3sv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib3sv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib3sv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetShaderiv: unsafe {
                unsafe extern "system" fn __glGetShaderiv(
                    _shader: GLuint,
                    _pname: ShaderParameterName,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetShaderiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetShaderiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetShaderiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib4bv: unsafe {
                unsafe extern "system" fn __glVertexAttrib4bv(_index: GLuint, _v: *mut GLbyte) {
                    panic!("Unable to load glVertexAttrib4bv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib4bv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib4bv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib4iv: unsafe {
                unsafe extern "system" fn __glVertexAttrib4iv(_index: GLuint, _v: *mut GLint) {
                    panic!("Unable to load glVertexAttrib4iv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib4iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib4iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetVertexAttribPointerv: unsafe {
                unsafe extern "system" fn __glGetVertexAttribPointerv(
                    _index: GLuint,
                    _pname: VertexAttribPointerPropertyARB,
                    _pointer: *mut GLvoid,
                ) {
                    panic!("Unable to load glGetVertexAttribPointerv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetVertexAttribPointerv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetVertexAttribPointerv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniformMatrix4fv: unsafe {
                unsafe extern "system" fn __glUniformMatrix4fv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLfloat,
                ) {
                    panic!("Unable to load glUniformMatrix4fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix4fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformMatrix4fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib1fv: unsafe {
                unsafe extern "system" fn __glVertexAttrib1fv(_index: GLuint, _v: *mut GLfloat) {
                    panic!("Unable to load glVertexAttrib1fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib1fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib1fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDrawBuffers: unsafe {
                unsafe extern "system" fn __glDrawBuffers(_n: GLsizei, _bufs: *mut DrawBufferMode) {
                    panic!("Unable to load glDrawBuffers")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDrawBuffers\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDrawBuffers
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform4f: unsafe {
                unsafe extern "system" fn __glUniform4f(
                    _location: GLint,
                    _v0: GLfloat,
                    _v1: GLfloat,
                    _v2: GLfloat,
                    _v3: GLfloat,
                ) {
                    panic!("Unable to load glUniform4f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform4f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform4f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetVertexAttribfv: unsafe {
                unsafe extern "system" fn __glGetVertexAttribfv(
                    _index: GLuint,
                    _pname: VertexAttribPropertyARB,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load glGetVertexAttribfv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetVertexAttribfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetVertexAttribfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetProgramInfoLog: unsafe {
                unsafe extern "system" fn __glGetProgramInfoLog(
                    _program: GLuint,
                    _bufSize: GLsizei,
                    _length: *mut GLsizei,
                    _infoLog: *mut GLchar,
                ) {
                    panic!("Unable to load glGetProgramInfoLog")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetProgramInfoLog\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetProgramInfoLog
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform1fv: unsafe {
                unsafe extern "system" fn __glUniform1fv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLfloat,
                ) {
                    panic!("Unable to load glUniform1fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform1fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform1fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glShaderSource: unsafe {
                unsafe extern "system" fn __glShaderSource(
                    _shader: GLuint,
                    _count: GLsizei,
                    _string: *mut GLchar,
                    _length: *mut GLint,
                ) {
                    panic!("Unable to load glShaderSource")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glShaderSource\0");
                let val = _f(cname);
                if val.is_null() {
                    __glShaderSource
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib2fv: unsafe {
                unsafe extern "system" fn __glVertexAttrib2fv(_index: GLuint, _v: *mut GLfloat) {
                    panic!("Unable to load glVertexAttrib2fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib2fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib2fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib4ubv: unsafe {
                unsafe extern "system" fn __glVertexAttrib4ubv(_index: GLuint, _v: *mut GLubyte) {
                    panic!("Unable to load glVertexAttrib4ubv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib4ubv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib4ubv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform1iv: unsafe {
                unsafe extern "system" fn __glUniform1iv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLint,
                ) {
                    panic!("Unable to load glUniform1iv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform1iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform1iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib4sv: unsafe {
                unsafe extern "system" fn __glVertexAttrib4sv(_index: GLuint, _v: *mut GLshort) {
                    panic!("Unable to load glVertexAttrib4sv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib4sv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib4sv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib2s: unsafe {
                unsafe extern "system" fn __glVertexAttrib2s(
                    _index: GLuint,
                    _x: GLshort,
                    _y: GLshort,
                ) {
                    panic!("Unable to load glVertexAttrib2s")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib2s\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib2s
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib3dv: unsafe {
                unsafe extern "system" fn __glVertexAttrib3dv(_index: GLuint, _v: *mut GLdouble) {
                    panic!("Unable to load glVertexAttrib3dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib3dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib3dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform3iv: unsafe {
                unsafe extern "system" fn __glUniform3iv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLint,
                ) {
                    panic!("Unable to load glUniform3iv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform3iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform3iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glStencilFuncSeparate: unsafe {
                unsafe extern "system" fn __glStencilFuncSeparate(
                    _face: StencilFaceDirection,
                    _func: StencilFunction,
                    _ref: GLint,
                    _mask: GLuint,
                ) {
                    panic!("Unable to load glStencilFuncSeparate")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glStencilFuncSeparate\0");
                let val = _f(cname);
                if val.is_null() {
                    __glStencilFuncSeparate
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib2d: unsafe {
                unsafe extern "system" fn __glVertexAttrib2d(
                    _index: GLuint,
                    _x: GLdouble,
                    _y: GLdouble,
                ) {
                    panic!("Unable to load glVertexAttrib2d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib2d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib2d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib3d: unsafe {
                unsafe extern "system" fn __glVertexAttrib3d(
                    _index: GLuint,
                    _x: GLdouble,
                    _y: GLdouble,
                    _z: GLdouble,
                ) {
                    panic!("Unable to load glVertexAttrib3d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib3d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib3d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib4Nuiv: unsafe {
                unsafe extern "system" fn __glVertexAttrib4Nuiv(_index: GLuint, _v: *mut GLuint) {
                    panic!("Unable to load glVertexAttrib4Nuiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib4Nuiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib4Nuiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib4Nubv: unsafe {
                unsafe extern "system" fn __glVertexAttrib4Nubv(_index: GLuint, _v: *mut GLubyte) {
                    panic!("Unable to load glVertexAttrib4Nubv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib4Nubv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib4Nubv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetUniformiv: unsafe {
                unsafe extern "system" fn __glGetUniformiv(
                    _program: GLuint,
                    _location: GLint,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetUniformiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetUniformiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetUniformiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glAttachShader: unsafe {
                unsafe extern "system" fn __glAttachShader(_program: GLuint, _shader: GLuint) {
                    panic!("Unable to load glAttachShader")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glAttachShader\0");
                let val = _f(cname);
                if val.is_null() {
                    __glAttachShader
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib4dv: unsafe {
                unsafe extern "system" fn __glVertexAttrib4dv(_index: GLuint, _v: *mut GLdouble) {
                    panic!("Unable to load glVertexAttrib4dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib4dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib4dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttrib4f: unsafe {
                unsafe extern "system" fn __glVertexAttrib4f(
                    _index: GLuint,
                    _x: GLfloat,
                    _y: GLfloat,
                    _z: GLfloat,
                    _w: GLfloat,
                ) {
                    panic!("Unable to load glVertexAttrib4f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib4f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib4f
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[derive(Clone)]
pub struct EntryFnGL45 {
    pub glTextureParameterIiv: crate::gl::command::PFN_glTextureParameterIiv,
    pub glTextureStorage1D: crate::gl::command::PFN_glTextureStorage1D,
    pub glGetnUniformdv: crate::gl::command::PFN_glGetnUniformdv,
    pub glTextureStorage2DMultisample: crate::gl::command::PFN_glTextureStorage2DMultisample,
    pub glNamedFramebufferTexture: crate::gl::command::PFN_glNamedFramebufferTexture,
    pub glInvalidateNamedFramebufferSubData:
        crate::gl::command::PFN_glInvalidateNamedFramebufferSubData,
    pub glCreateTransformFeedbacks: crate::gl::command::PFN_glCreateTransformFeedbacks,
    pub glNamedFramebufferTextureLayer: crate::gl::command::PFN_glNamedFramebufferTextureLayer,
    pub glMemoryBarrierByRegion: crate::gl::command::PFN_glMemoryBarrierByRegion,
    pub glGetnMapdv: crate::gl::command::PFN_glGetnMapdv,
    pub glNamedBufferData: crate::gl::command::PFN_glNamedBufferData,
    pub glGetTransformFeedbacki64_v: crate::gl::command::PFN_glGetTransformFeedbacki64_v,
    pub glClearNamedFramebufferfi: crate::gl::command::PFN_glClearNamedFramebufferfi,
    pub glGetTextureLevelParameterfv: crate::gl::command::PFN_glGetTextureLevelParameterfv,
    pub glGetTransformFeedbacki_v: crate::gl::command::PFN_glGetTransformFeedbacki_v,
    pub glGetVertexArrayIndexediv: crate::gl::command::PFN_glGetVertexArrayIndexediv,
    pub glGetCompressedTextureSubImage: crate::gl::command::PFN_glGetCompressedTextureSubImage,
    pub glGetTextureParameterIiv: crate::gl::command::PFN_glGetTextureParameterIiv,
    pub glCopyTextureSubImage2D: crate::gl::command::PFN_glCopyTextureSubImage2D,
    pub glGetnUniformfv: crate::gl::command::PFN_glGetnUniformfv,
    pub glTransformFeedbackBufferRange: crate::gl::command::PFN_glTransformFeedbackBufferRange,
    pub glMapNamedBuffer: crate::gl::command::PFN_glMapNamedBuffer,
    pub glGetTextureParameterIuiv: crate::gl::command::PFN_glGetTextureParameterIuiv,
    pub glGetNamedFramebufferAttachmentParameteriv:
        crate::gl::command::PFN_glGetNamedFramebufferAttachmentParameteriv,
    pub glVertexArrayAttribIFormat: crate::gl::command::PFN_glVertexArrayAttribIFormat,
    pub glGetnMapfv: crate::gl::command::PFN_glGetnMapfv,
    pub glVertexArrayAttribFormat: crate::gl::command::PFN_glVertexArrayAttribFormat,
    pub glVertexArrayElementBuffer: crate::gl::command::PFN_glVertexArrayElementBuffer,
    pub glNamedFramebufferRenderbuffer: crate::gl::command::PFN_glNamedFramebufferRenderbuffer,
    pub glCopyNamedBufferSubData: crate::gl::command::PFN_glCopyNamedBufferSubData,
    pub glGetTextureImage: crate::gl::command::PFN_glGetTextureImage,
    pub glGetQueryBufferObjectui64v: crate::gl::command::PFN_glGetQueryBufferObjectui64v,
    pub glGetTextureParameteriv: crate::gl::command::PFN_glGetTextureParameteriv,
    pub glClearNamedBufferData: crate::gl::command::PFN_glClearNamedBufferData,
    pub glGetnPolygonStipple: crate::gl::command::PFN_glGetnPolygonStipple,
    pub glGetQueryBufferObjectuiv: crate::gl::command::PFN_glGetQueryBufferObjectuiv,
    pub glGetTextureSubImage: crate::gl::command::PFN_glGetTextureSubImage,
    pub glCreateVertexArrays: crate::gl::command::PFN_glCreateVertexArrays,
    pub glVertexArrayVertexBuffers: crate::gl::command::PFN_glVertexArrayVertexBuffers,
    pub glTextureBarrier: crate::gl::command::PFN_glTextureBarrier,
    pub glReadnPixels: crate::gl::command::PFN_glReadnPixels,
    pub glCompressedTextureSubImage3D: crate::gl::command::PFN_glCompressedTextureSubImage3D,
    pub glFlushMappedNamedBufferRange: crate::gl::command::PFN_glFlushMappedNamedBufferRange,
    pub glCompressedTextureSubImage2D: crate::gl::command::PFN_glCompressedTextureSubImage2D,
    pub glGetnSeparableFilter: crate::gl::command::PFN_glGetnSeparableFilter,
    pub glGetnUniformiv: crate::gl::command::PFN_glGetnUniformiv,
    pub glGetQueryBufferObjectiv: crate::gl::command::PFN_glGetQueryBufferObjectiv,
    pub glInvalidateNamedFramebufferData: crate::gl::command::PFN_glInvalidateNamedFramebufferData,
    pub glEnableVertexArrayAttrib: crate::gl::command::PFN_glEnableVertexArrayAttrib,
    pub glGetnColorTable: crate::gl::command::PFN_glGetnColorTable,
    pub glCreateSamplers: crate::gl::command::PFN_glCreateSamplers,
    pub glGetnPixelMapuiv: crate::gl::command::PFN_glGetnPixelMapuiv,
    pub glTextureParameterfv: crate::gl::command::PFN_glTextureParameterfv,
    pub glGetTextureParameterfv: crate::gl::command::PFN_glGetTextureParameterfv,
    pub glGetnTexImage: crate::gl::command::PFN_glGetnTexImage,
    pub glNamedRenderbufferStorage: crate::gl::command::PFN_glNamedRenderbufferStorage,
    pub glGetnPixelMapusv: crate::gl::command::PFN_glGetnPixelMapusv,
    pub glTextureParameteri: crate::gl::command::PFN_glTextureParameteri,
    pub glCreateProgramPipelines: crate::gl::command::PFN_glCreateProgramPipelines,
    pub glCreateRenderbuffers: crate::gl::command::PFN_glCreateRenderbuffers,
    pub glGetVertexArrayiv: crate::gl::command::PFN_glGetVertexArrayiv,
    pub glNamedRenderbufferStorageMultisample:
        crate::gl::command::PFN_glNamedRenderbufferStorageMultisample,
    pub glTextureStorage3DMultisample: crate::gl::command::PFN_glTextureStorage3DMultisample,
    pub glUnmapNamedBuffer: crate::gl::command::PFN_glUnmapNamedBuffer,
    pub glGetNamedBufferSubData: crate::gl::command::PFN_glGetNamedBufferSubData,
    pub glClearNamedFramebufferiv: crate::gl::command::PFN_glClearNamedFramebufferiv,
    pub glClearNamedBufferSubData: crate::gl::command::PFN_glClearNamedBufferSubData,
    pub glGetQueryBufferObjecti64v: crate::gl::command::PFN_glGetQueryBufferObjecti64v,
    pub glVertexArrayAttribLFormat: crate::gl::command::PFN_glVertexArrayAttribLFormat,
    pub glNamedFramebufferDrawBuffer: crate::gl::command::PFN_glNamedFramebufferDrawBuffer,
    pub glTextureSubImage2D: crate::gl::command::PFN_glTextureSubImage2D,
    pub glGetnMapiv: crate::gl::command::PFN_glGetnMapiv,
    pub glTransformFeedbackBufferBase: crate::gl::command::PFN_glTransformFeedbackBufferBase,
    pub glBindTextureUnit: crate::gl::command::PFN_glBindTextureUnit,
    pub glGetTransformFeedbackiv: crate::gl::command::PFN_glGetTransformFeedbackiv,
    pub glTextureParameterIuiv: crate::gl::command::PFN_glTextureParameterIuiv,
    pub glMapNamedBufferRange: crate::gl::command::PFN_glMapNamedBufferRange,
    pub glTextureBuffer: crate::gl::command::PFN_glTextureBuffer,
    pub glGetNamedBufferPointerv: crate::gl::command::PFN_glGetNamedBufferPointerv,
    pub glTextureParameterf: crate::gl::command::PFN_glTextureParameterf,
    pub glTextureSubImage1D: crate::gl::command::PFN_glTextureSubImage1D,
    pub glVertexArrayBindingDivisor: crate::gl::command::PFN_glVertexArrayBindingDivisor,
    pub glGetnCompressedTexImage: crate::gl::command::PFN_glGetnCompressedTexImage,
    pub glGetnHistogram: crate::gl::command::PFN_glGetnHistogram,
    pub glTextureBufferRange: crate::gl::command::PFN_glTextureBufferRange,
    pub glTextureStorage3D: crate::gl::command::PFN_glTextureStorage3D,
    pub glGetnMinmax: crate::gl::command::PFN_glGetnMinmax,
    pub glCheckNamedFramebufferStatus: crate::gl::command::PFN_glCheckNamedFramebufferStatus,
    pub glGetnUniformuiv: crate::gl::command::PFN_glGetnUniformuiv,
    pub glGetNamedBufferParameteri64v: crate::gl::command::PFN_glGetNamedBufferParameteri64v,
    pub glBlitNamedFramebuffer: crate::gl::command::PFN_glBlitNamedFramebuffer,
    pub glTextureStorage2D: crate::gl::command::PFN_glTextureStorage2D,
    pub glGetCompressedTextureImage: crate::gl::command::PFN_glGetCompressedTextureImage,
    pub glCompressedTextureSubImage1D: crate::gl::command::PFN_glCompressedTextureSubImage1D,
    pub glVertexArrayAttribBinding: crate::gl::command::PFN_glVertexArrayAttribBinding,
    pub glNamedFramebufferReadBuffer: crate::gl::command::PFN_glNamedFramebufferReadBuffer,
    pub glClearNamedFramebufferfv: crate::gl::command::PFN_glClearNamedFramebufferfv,
    pub glNamedBufferSubData: crate::gl::command::PFN_glNamedBufferSubData,
    pub glClearNamedFramebufferuiv: crate::gl::command::PFN_glClearNamedFramebufferuiv,
    pub glCopyTextureSubImage1D: crate::gl::command::PFN_glCopyTextureSubImage1D,
    pub glGetVertexArrayIndexed64iv: crate::gl::command::PFN_glGetVertexArrayIndexed64iv,
    pub glCreateBuffers: crate::gl::command::PFN_glCreateBuffers,
    pub glCreateFramebuffers: crate::gl::command::PFN_glCreateFramebuffers,
    pub glTextureSubImage3D: crate::gl::command::PFN_glTextureSubImage3D,
    pub glTextureParameteriv: crate::gl::command::PFN_glTextureParameteriv,
    pub glGetGraphicsResetStatus: crate::gl::command::PFN_glGetGraphicsResetStatus,
    pub glGetnConvolutionFilter: crate::gl::command::PFN_glGetnConvolutionFilter,
    pub glClipControl: crate::gl::command::PFN_glClipControl,
    pub glNamedFramebufferParameteri: crate::gl::command::PFN_glNamedFramebufferParameteri,
    pub glGetTextureLevelParameteriv: crate::gl::command::PFN_glGetTextureLevelParameteriv,
    pub glCopyTextureSubImage3D: crate::gl::command::PFN_glCopyTextureSubImage3D,
    pub glNamedBufferStorage: crate::gl::command::PFN_glNamedBufferStorage,
    pub glGetNamedBufferParameteriv: crate::gl::command::PFN_glGetNamedBufferParameteriv,
    pub glVertexArrayVertexBuffer: crate::gl::command::PFN_glVertexArrayVertexBuffer,
    pub glGetNamedRenderbufferParameteriv:
        crate::gl::command::PFN_glGetNamedRenderbufferParameteriv,
    pub glGetnPixelMapfv: crate::gl::command::PFN_glGetnPixelMapfv,
    pub glDisableVertexArrayAttrib: crate::gl::command::PFN_glDisableVertexArrayAttrib,
    pub glCreateQueries: crate::gl::command::PFN_glCreateQueries,
    pub glCreateTextures: crate::gl::command::PFN_glCreateTextures,
    pub glNamedFramebufferDrawBuffers: crate::gl::command::PFN_glNamedFramebufferDrawBuffers,
    pub glGetNamedFramebufferParameteriv: crate::gl::command::PFN_glGetNamedFramebufferParameteriv,
    pub glGenerateTextureMipmap: crate::gl::command::PFN_glGenerateTextureMipmap,
}
impl EntryFnGL45 {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            glTextureParameterIiv: unsafe {
                unsafe extern "system" fn __glTextureParameterIiv(
                    _texture: GLuint,
                    _pname: TextureParameterName,
                    _params: GLint,
                ) {
                    panic!("Unable to load glTextureParameterIiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTextureParameterIiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTextureParameterIiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTextureStorage1D: unsafe {
                unsafe extern "system" fn __glTextureStorage1D(
                    _texture: GLuint,
                    _levels: GLsizei,
                    _internalformat: SizedInternalFormat,
                    _width: GLsizei,
                ) {
                    panic!("Unable to load glTextureStorage1D")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTextureStorage1D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTextureStorage1D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetnUniformdv: unsafe {
                unsafe extern "system" fn __glGetnUniformdv(
                    _program: GLuint,
                    _location: GLint,
                    _bufSize: GLsizei,
                    _params: *mut GLdouble,
                ) {
                    panic!("Unable to load glGetnUniformdv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetnUniformdv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetnUniformdv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTextureStorage2DMultisample: unsafe {
                unsafe extern "system" fn __glTextureStorage2DMultisample(
                    _texture: GLuint,
                    _samples: GLsizei,
                    _internalformat: SizedInternalFormat,
                    _width: GLsizei,
                    _height: GLsizei,
                    _fixedsamplelocations: Boolean,
                ) {
                    panic!("Unable to load glTextureStorage2DMultisample")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glTextureStorage2DMultisample\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glTextureStorage2DMultisample
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glNamedFramebufferTexture: unsafe {
                unsafe extern "system" fn __glNamedFramebufferTexture(
                    _framebuffer: GLuint,
                    _attachment: FramebufferAttachment,
                    _texture: GLuint,
                    _level: GLint,
                ) {
                    panic!("Unable to load glNamedFramebufferTexture")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glNamedFramebufferTexture\0");
                let val = _f(cname);
                if val.is_null() {
                    __glNamedFramebufferTexture
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glInvalidateNamedFramebufferSubData: unsafe {
                unsafe extern "system" fn __glInvalidateNamedFramebufferSubData(
                    _framebuffer: GLuint,
                    _numAttachments: GLsizei,
                    _attachments: FramebufferAttachment,
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                ) {
                    panic!("Unable to load glInvalidateNamedFramebufferSubData")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glInvalidateNamedFramebufferSubData\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glInvalidateNamedFramebufferSubData
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCreateTransformFeedbacks: unsafe {
                unsafe extern "system" fn __glCreateTransformFeedbacks(
                    _n: GLsizei,
                    _ids: *mut GLuint,
                ) {
                    panic!("Unable to load glCreateTransformFeedbacks")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glCreateTransformFeedbacks\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glCreateTransformFeedbacks
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glNamedFramebufferTextureLayer: unsafe {
                unsafe extern "system" fn __glNamedFramebufferTextureLayer(
                    _framebuffer: GLuint,
                    _attachment: FramebufferAttachment,
                    _texture: GLuint,
                    _level: GLint,
                    _layer: GLint,
                ) {
                    panic!("Unable to load glNamedFramebufferTextureLayer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glNamedFramebufferTextureLayer\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glNamedFramebufferTextureLayer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMemoryBarrierByRegion: unsafe {
                unsafe extern "system" fn __glMemoryBarrierByRegion(_barriers: MemoryBarrierMask) {
                    panic!("Unable to load glMemoryBarrierByRegion")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMemoryBarrierByRegion\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMemoryBarrierByRegion
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetnMapdv: unsafe {
                unsafe extern "system" fn __glGetnMapdv(
                    _target: MapTarget,
                    _query: MapQuery,
                    _bufSize: GLsizei,
                    _v: *mut GLdouble,
                ) {
                    panic!("Unable to load glGetnMapdv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetnMapdv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetnMapdv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glNamedBufferData: unsafe {
                unsafe extern "system" fn __glNamedBufferData(
                    _buffer: GLuint,
                    _size: GLsizeiptr,
                    _data: GLvoid,
                    _usage: VertexBufferObjectUsage,
                ) {
                    panic!("Unable to load glNamedBufferData")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glNamedBufferData\0");
                let val = _f(cname);
                if val.is_null() {
                    __glNamedBufferData
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetTransformFeedbacki64_v: unsafe {
                unsafe extern "system" fn __glGetTransformFeedbacki64_v(
                    _xfb: GLuint,
                    _pname: TransformFeedbackPName,
                    _index: GLuint,
                    _param: GLint64,
                ) {
                    panic!("Unable to load glGetTransformFeedbacki64_v")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetTransformFeedbacki64_v\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetTransformFeedbacki64_v
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glClearNamedFramebufferfi: unsafe {
                unsafe extern "system" fn __glClearNamedFramebufferfi(
                    _framebuffer: GLuint,
                    _buffer: Buffer,
                    _drawbuffer: GLint,
                    _depth: GLfloat,
                    _stencil: GLint,
                ) {
                    panic!("Unable to load glClearNamedFramebufferfi")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClearNamedFramebufferfi\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClearNamedFramebufferfi
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetTextureLevelParameterfv: unsafe {
                unsafe extern "system" fn __glGetTextureLevelParameterfv(
                    _texture: GLuint,
                    _level: GLint,
                    _pname: GetTextureParameter,
                    _params: GLfloat,
                ) {
                    panic!("Unable to load glGetTextureLevelParameterfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetTextureLevelParameterfv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetTextureLevelParameterfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetTransformFeedbacki_v: unsafe {
                unsafe extern "system" fn __glGetTransformFeedbacki_v(
                    _xfb: GLuint,
                    _pname: TransformFeedbackPName,
                    _index: GLuint,
                    _param: GLint,
                ) {
                    panic!("Unable to load glGetTransformFeedbacki_v")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTransformFeedbacki_v\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTransformFeedbacki_v
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetVertexArrayIndexediv: unsafe {
                unsafe extern "system" fn __glGetVertexArrayIndexediv(
                    _vaobj: GLuint,
                    _index: GLuint,
                    _pname: VertexArrayPName,
                    _param: GLint,
                ) {
                    panic!("Unable to load glGetVertexArrayIndexediv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetVertexArrayIndexediv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetVertexArrayIndexediv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetCompressedTextureSubImage: unsafe {
                unsafe extern "system" fn __glGetCompressedTextureSubImage(
                    _texture: GLuint,
                    _level: GLint,
                    _xoffset: GLint,
                    _yoffset: GLint,
                    _zoffset: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                    _depth: GLsizei,
                    _bufSize: GLsizei,
                    _pixels: GLvoid,
                ) {
                    panic!("Unable to load glGetCompressedTextureSubImage")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetCompressedTextureSubImage\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetCompressedTextureSubImage
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetTextureParameterIiv: unsafe {
                unsafe extern "system" fn __glGetTextureParameterIiv(
                    _texture: GLuint,
                    _pname: GetTextureParameter,
                    _params: GLint,
                ) {
                    panic!("Unable to load glGetTextureParameterIiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTextureParameterIiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTextureParameterIiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCopyTextureSubImage2D: unsafe {
                unsafe extern "system" fn __glCopyTextureSubImage2D(
                    _texture: GLuint,
                    _level: GLint,
                    _xoffset: GLint,
                    _yoffset: GLint,
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                ) {
                    panic!("Unable to load glCopyTextureSubImage2D")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCopyTextureSubImage2D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCopyTextureSubImage2D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetnUniformfv: unsafe {
                unsafe extern "system" fn __glGetnUniformfv(
                    _program: GLuint,
                    _location: GLint,
                    _bufSize: GLsizei,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load glGetnUniformfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetnUniformfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetnUniformfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTransformFeedbackBufferRange: unsafe {
                unsafe extern "system" fn __glTransformFeedbackBufferRange(
                    _xfb: GLuint,
                    _index: GLuint,
                    _buffer: GLuint,
                    _offset: GLintptr,
                    _size: GLsizeiptr,
                ) {
                    panic!("Unable to load glTransformFeedbackBufferRange")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glTransformFeedbackBufferRange\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glTransformFeedbackBufferRange
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMapNamedBuffer: unsafe {
                unsafe extern "system" fn __glMapNamedBuffer(
                    _buffer: GLuint,
                    _access: BufferAccessARB,
                ) {
                    panic!("Unable to load glMapNamedBuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMapNamedBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMapNamedBuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetTextureParameterIuiv: unsafe {
                unsafe extern "system" fn __glGetTextureParameterIuiv(
                    _texture: GLuint,
                    _pname: GetTextureParameter,
                    _params: GLuint,
                ) {
                    panic!("Unable to load glGetTextureParameterIuiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTextureParameterIuiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTextureParameterIuiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetNamedFramebufferAttachmentParameteriv: unsafe {
                unsafe extern "system" fn __glGetNamedFramebufferAttachmentParameteriv(
                    _framebuffer: GLuint,
                    _attachment: FramebufferAttachment,
                    _pname: FramebufferAttachmentParameterName,
                    _params: GLint,
                ) {
                    panic!("Unable to load glGetNamedFramebufferAttachmentParameteriv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetNamedFramebufferAttachmentParameteriv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetNamedFramebufferAttachmentParameteriv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexArrayAttribIFormat: unsafe {
                unsafe extern "system" fn __glVertexArrayAttribIFormat(
                    _vaobj: GLuint,
                    _attribindex: GLuint,
                    _size: GLint,
                    _type: VertexAttribIType,
                    _relativeoffset: GLuint,
                ) {
                    panic!("Unable to load glVertexArrayAttribIFormat")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glVertexArrayAttribIFormat\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glVertexArrayAttribIFormat
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetnMapfv: unsafe {
                unsafe extern "system" fn __glGetnMapfv(
                    _target: MapTarget,
                    _query: MapQuery,
                    _bufSize: GLsizei,
                    _v: GLfloat,
                ) {
                    panic!("Unable to load glGetnMapfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetnMapfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetnMapfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexArrayAttribFormat: unsafe {
                unsafe extern "system" fn __glVertexArrayAttribFormat(
                    _vaobj: GLuint,
                    _attribindex: GLuint,
                    _size: GLint,
                    _type: VertexAttribType,
                    _normalized: Boolean,
                    _relativeoffset: GLuint,
                ) {
                    panic!("Unable to load glVertexArrayAttribFormat")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexArrayAttribFormat\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexArrayAttribFormat
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexArrayElementBuffer: unsafe {
                unsafe extern "system" fn __glVertexArrayElementBuffer(
                    _vaobj: GLuint,
                    _buffer: GLuint,
                ) {
                    panic!("Unable to load glVertexArrayElementBuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glVertexArrayElementBuffer\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glVertexArrayElementBuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glNamedFramebufferRenderbuffer: unsafe {
                unsafe extern "system" fn __glNamedFramebufferRenderbuffer(
                    _framebuffer: GLuint,
                    _attachment: FramebufferAttachment,
                    _renderbuffertarget: RenderbufferTarget,
                    _renderbuffer: GLuint,
                ) {
                    panic!("Unable to load glNamedFramebufferRenderbuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glNamedFramebufferRenderbuffer\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glNamedFramebufferRenderbuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCopyNamedBufferSubData: unsafe {
                unsafe extern "system" fn __glCopyNamedBufferSubData(
                    _readBuffer: GLuint,
                    _writeBuffer: GLuint,
                    _readOffset: GLintptr,
                    _writeOffset: GLintptr,
                    _size: GLsizeiptr,
                ) {
                    panic!("Unable to load glCopyNamedBufferSubData")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCopyNamedBufferSubData\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCopyNamedBufferSubData
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetTextureImage: unsafe {
                unsafe extern "system" fn __glGetTextureImage(
                    _texture: GLuint,
                    _level: GLint,
                    _format: PixelFormat,
                    _type: PixelType,
                    _bufSize: GLsizei,
                    _pixels: GLvoid,
                ) {
                    panic!("Unable to load glGetTextureImage")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTextureImage\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTextureImage
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetQueryBufferObjectui64v: unsafe {
                unsafe extern "system" fn __glGetQueryBufferObjectui64v(
                    _id: GLuint,
                    _buffer: GLuint,
                    _pname: QueryObjectParameterName,
                    _offset: GLintptr,
                ) {
                    panic!("Unable to load glGetQueryBufferObjectui64v")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetQueryBufferObjectui64v\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetQueryBufferObjectui64v
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetTextureParameteriv: unsafe {
                unsafe extern "system" fn __glGetTextureParameteriv(
                    _texture: GLuint,
                    _pname: GetTextureParameter,
                    _params: GLint,
                ) {
                    panic!("Unable to load glGetTextureParameteriv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTextureParameteriv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTextureParameteriv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glClearNamedBufferData: unsafe {
                unsafe extern "system" fn __glClearNamedBufferData(
                    _buffer: GLuint,
                    _internalformat: SizedInternalFormat,
                    _format: PixelFormat,
                    _type: PixelType,
                    _data: GLvoid,
                ) {
                    panic!("Unable to load glClearNamedBufferData")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClearNamedBufferData\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClearNamedBufferData
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetnPolygonStipple: unsafe {
                unsafe extern "system" fn __glGetnPolygonStipple(
                    _bufSize: GLsizei,
                    _pattern: *mut GLubyte,
                ) {
                    panic!("Unable to load glGetnPolygonStipple")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetnPolygonStipple\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetnPolygonStipple
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetQueryBufferObjectuiv: unsafe {
                unsafe extern "system" fn __glGetQueryBufferObjectuiv(
                    _id: GLuint,
                    _buffer: GLuint,
                    _pname: QueryObjectParameterName,
                    _offset: GLintptr,
                ) {
                    panic!("Unable to load glGetQueryBufferObjectuiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetQueryBufferObjectuiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetQueryBufferObjectuiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetTextureSubImage: unsafe {
                unsafe extern "system" fn __glGetTextureSubImage(
                    _texture: GLuint,
                    _level: GLint,
                    _xoffset: GLint,
                    _yoffset: GLint,
                    _zoffset: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                    _depth: GLsizei,
                    _format: PixelFormat,
                    _type: PixelType,
                    _bufSize: GLsizei,
                    _pixels: GLvoid,
                ) {
                    panic!("Unable to load glGetTextureSubImage")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTextureSubImage\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTextureSubImage
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCreateVertexArrays: unsafe {
                unsafe extern "system" fn __glCreateVertexArrays(
                    _n: GLsizei,
                    _arrays: *mut GLuint,
                ) {
                    panic!("Unable to load glCreateVertexArrays")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCreateVertexArrays\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCreateVertexArrays
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexArrayVertexBuffers: unsafe {
                unsafe extern "system" fn __glVertexArrayVertexBuffers(
                    _vaobj: GLuint,
                    _first: GLuint,
                    _count: GLsizei,
                    _buffers: GLuint,
                    _offsets: GLintptr,
                    _strides: GLsizei,
                ) {
                    panic!("Unable to load glVertexArrayVertexBuffers")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glVertexArrayVertexBuffers\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glVertexArrayVertexBuffers
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTextureBarrier: unsafe {
                unsafe extern "system" fn __glTextureBarrier() {
                    panic!("Unable to load glTextureBarrier")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTextureBarrier\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTextureBarrier
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glReadnPixels: unsafe {
                unsafe extern "system" fn __glReadnPixels(
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                    _format: PixelFormat,
                    _type: PixelType,
                    _bufSize: GLsizei,
                    _data: *mut GLvoid,
                ) {
                    panic!("Unable to load glReadnPixels")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glReadnPixels\0");
                let val = _f(cname);
                if val.is_null() {
                    __glReadnPixels
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCompressedTextureSubImage3D: unsafe {
                unsafe extern "system" fn __glCompressedTextureSubImage3D(
                    _texture: GLuint,
                    _level: GLint,
                    _xoffset: GLint,
                    _yoffset: GLint,
                    _zoffset: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                    _depth: GLsizei,
                    _format: InternalFormat,
                    _imageSize: GLsizei,
                    _data: GLvoid,
                ) {
                    panic!("Unable to load glCompressedTextureSubImage3D")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glCompressedTextureSubImage3D\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glCompressedTextureSubImage3D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glFlushMappedNamedBufferRange: unsafe {
                unsafe extern "system" fn __glFlushMappedNamedBufferRange(
                    _buffer: GLuint,
                    _offset: GLintptr,
                    _length: GLsizeiptr,
                ) {
                    panic!("Unable to load glFlushMappedNamedBufferRange")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glFlushMappedNamedBufferRange\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glFlushMappedNamedBufferRange
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCompressedTextureSubImage2D: unsafe {
                unsafe extern "system" fn __glCompressedTextureSubImage2D(
                    _texture: GLuint,
                    _level: GLint,
                    _xoffset: GLint,
                    _yoffset: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                    _format: InternalFormat,
                    _imageSize: GLsizei,
                    _data: GLvoid,
                ) {
                    panic!("Unable to load glCompressedTextureSubImage2D")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glCompressedTextureSubImage2D\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glCompressedTextureSubImage2D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetnSeparableFilter: unsafe {
                unsafe extern "system" fn __glGetnSeparableFilter(
                    _target: SeparableTarget,
                    _format: PixelFormat,
                    _type: PixelType,
                    _rowBufSize: GLsizei,
                    _row: *mut GLvoid,
                    _columnBufSize: GLsizei,
                    _column: *mut GLvoid,
                    _span: *mut GLvoid,
                ) {
                    panic!("Unable to load glGetnSeparableFilter")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetnSeparableFilter\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetnSeparableFilter
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetnUniformiv: unsafe {
                unsafe extern "system" fn __glGetnUniformiv(
                    _program: GLuint,
                    _location: GLint,
                    _bufSize: GLsizei,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetnUniformiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetnUniformiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetnUniformiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetQueryBufferObjectiv: unsafe {
                unsafe extern "system" fn __glGetQueryBufferObjectiv(
                    _id: GLuint,
                    _buffer: GLuint,
                    _pname: QueryObjectParameterName,
                    _offset: GLintptr,
                ) {
                    panic!("Unable to load glGetQueryBufferObjectiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetQueryBufferObjectiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetQueryBufferObjectiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glInvalidateNamedFramebufferData: unsafe {
                unsafe extern "system" fn __glInvalidateNamedFramebufferData(
                    _framebuffer: GLuint,
                    _numAttachments: GLsizei,
                    _attachments: FramebufferAttachment,
                ) {
                    panic!("Unable to load glInvalidateNamedFramebufferData")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glInvalidateNamedFramebufferData\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glInvalidateNamedFramebufferData
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glEnableVertexArrayAttrib: unsafe {
                unsafe extern "system" fn __glEnableVertexArrayAttrib(
                    _vaobj: GLuint,
                    _index: GLuint,
                ) {
                    panic!("Unable to load glEnableVertexArrayAttrib")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEnableVertexArrayAttrib\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEnableVertexArrayAttrib
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetnColorTable: unsafe {
                unsafe extern "system" fn __glGetnColorTable(
                    _target: ColorTableTarget,
                    _format: PixelFormat,
                    _type: PixelType,
                    _bufSize: GLsizei,
                    _table: *mut GLvoid,
                ) {
                    panic!("Unable to load glGetnColorTable")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetnColorTable\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetnColorTable
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCreateSamplers: unsafe {
                unsafe extern "system" fn __glCreateSamplers(_n: GLsizei, _samplers: *mut GLuint) {
                    panic!("Unable to load glCreateSamplers")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCreateSamplers\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCreateSamplers
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetnPixelMapuiv: unsafe {
                unsafe extern "system" fn __glGetnPixelMapuiv(
                    _map: PixelMap,
                    _bufSize: GLsizei,
                    _values: GLuint,
                ) {
                    panic!("Unable to load glGetnPixelMapuiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetnPixelMapuiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetnPixelMapuiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTextureParameterfv: unsafe {
                unsafe extern "system" fn __glTextureParameterfv(
                    _texture: GLuint,
                    _pname: TextureParameterName,
                    _param: GLfloat,
                ) {
                    panic!("Unable to load glTextureParameterfv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTextureParameterfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTextureParameterfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetTextureParameterfv: unsafe {
                unsafe extern "system" fn __glGetTextureParameterfv(
                    _texture: GLuint,
                    _pname: GetTextureParameter,
                    _params: GLfloat,
                ) {
                    panic!("Unable to load glGetTextureParameterfv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTextureParameterfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTextureParameterfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetnTexImage: unsafe {
                unsafe extern "system" fn __glGetnTexImage(
                    _target: TextureTarget,
                    _level: GLint,
                    _format: PixelFormat,
                    _type: PixelType,
                    _bufSize: GLsizei,
                    _pixels: *mut GLvoid,
                ) {
                    panic!("Unable to load glGetnTexImage")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetnTexImage\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetnTexImage
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glNamedRenderbufferStorage: unsafe {
                unsafe extern "system" fn __glNamedRenderbufferStorage(
                    _renderbuffer: GLuint,
                    _internalformat: InternalFormat,
                    _width: GLsizei,
                    _height: GLsizei,
                ) {
                    panic!("Unable to load glNamedRenderbufferStorage")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glNamedRenderbufferStorage\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glNamedRenderbufferStorage
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetnPixelMapusv: unsafe {
                unsafe extern "system" fn __glGetnPixelMapusv(
                    _map: PixelMap,
                    _bufSize: GLsizei,
                    _values: GLushort,
                ) {
                    panic!("Unable to load glGetnPixelMapusv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetnPixelMapusv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetnPixelMapusv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTextureParameteri: unsafe {
                unsafe extern "system" fn __glTextureParameteri(
                    _texture: GLuint,
                    _pname: TextureParameterName,
                    _param: GLint,
                ) {
                    panic!("Unable to load glTextureParameteri")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTextureParameteri\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTextureParameteri
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCreateProgramPipelines: unsafe {
                unsafe extern "system" fn __glCreateProgramPipelines(
                    _n: GLsizei,
                    _pipelines: *mut GLuint,
                ) {
                    panic!("Unable to load glCreateProgramPipelines")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCreateProgramPipelines\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCreateProgramPipelines
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCreateRenderbuffers: unsafe {
                unsafe extern "system" fn __glCreateRenderbuffers(
                    _n: GLsizei,
                    _renderbuffers: *mut GLuint,
                ) {
                    panic!("Unable to load glCreateRenderbuffers")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCreateRenderbuffers\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCreateRenderbuffers
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetVertexArrayiv: unsafe {
                unsafe extern "system" fn __glGetVertexArrayiv(
                    _vaobj: GLuint,
                    _pname: VertexArrayPName,
                    _param: GLint,
                ) {
                    panic!("Unable to load glGetVertexArrayiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetVertexArrayiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetVertexArrayiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glNamedRenderbufferStorageMultisample: unsafe {
                unsafe extern "system" fn __glNamedRenderbufferStorageMultisample(
                    _renderbuffer: GLuint,
                    _samples: GLsizei,
                    _internalformat: InternalFormat,
                    _width: GLsizei,
                    _height: GLsizei,
                ) {
                    panic!("Unable to load glNamedRenderbufferStorageMultisample")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glNamedRenderbufferStorageMultisample\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glNamedRenderbufferStorageMultisample
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTextureStorage3DMultisample: unsafe {
                unsafe extern "system" fn __glTextureStorage3DMultisample(
                    _texture: GLuint,
                    _samples: GLsizei,
                    _internalformat: SizedInternalFormat,
                    _width: GLsizei,
                    _height: GLsizei,
                    _depth: GLsizei,
                    _fixedsamplelocations: Boolean,
                ) {
                    panic!("Unable to load glTextureStorage3DMultisample")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glTextureStorage3DMultisample\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glTextureStorage3DMultisample
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUnmapNamedBuffer: unsafe {
                unsafe extern "system" fn __glUnmapNamedBuffer(_buffer: GLuint) -> GLboolean {
                    panic!("Unable to load glUnmapNamedBuffer")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUnmapNamedBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUnmapNamedBuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetNamedBufferSubData: unsafe {
                unsafe extern "system" fn __glGetNamedBufferSubData(
                    _buffer: GLuint,
                    _offset: GLintptr,
                    _size: GLsizeiptr,
                    _data: GLvoid,
                ) {
                    panic!("Unable to load glGetNamedBufferSubData")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetNamedBufferSubData\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetNamedBufferSubData
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glClearNamedFramebufferiv: unsafe {
                unsafe extern "system" fn __glClearNamedFramebufferiv(
                    _framebuffer: GLuint,
                    _buffer: Buffer,
                    _drawbuffer: GLint,
                    _value: GLint,
                ) {
                    panic!("Unable to load glClearNamedFramebufferiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClearNamedFramebufferiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClearNamedFramebufferiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glClearNamedBufferSubData: unsafe {
                unsafe extern "system" fn __glClearNamedBufferSubData(
                    _buffer: GLuint,
                    _internalformat: SizedInternalFormat,
                    _offset: GLintptr,
                    _size: GLsizeiptr,
                    _format: PixelFormat,
                    _type: PixelType,
                    _data: GLvoid,
                ) {
                    panic!("Unable to load glClearNamedBufferSubData")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClearNamedBufferSubData\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClearNamedBufferSubData
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetQueryBufferObjecti64v: unsafe {
                unsafe extern "system" fn __glGetQueryBufferObjecti64v(
                    _id: GLuint,
                    _buffer: GLuint,
                    _pname: QueryObjectParameterName,
                    _offset: GLintptr,
                ) {
                    panic!("Unable to load glGetQueryBufferObjecti64v")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetQueryBufferObjecti64v\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetQueryBufferObjecti64v
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexArrayAttribLFormat: unsafe {
                unsafe extern "system" fn __glVertexArrayAttribLFormat(
                    _vaobj: GLuint,
                    _attribindex: GLuint,
                    _size: GLint,
                    _type: VertexAttribLType,
                    _relativeoffset: GLuint,
                ) {
                    panic!("Unable to load glVertexArrayAttribLFormat")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glVertexArrayAttribLFormat\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glVertexArrayAttribLFormat
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glNamedFramebufferDrawBuffer: unsafe {
                unsafe extern "system" fn __glNamedFramebufferDrawBuffer(
                    _framebuffer: GLuint,
                    _buf: ColorBuffer,
                ) {
                    panic!("Unable to load glNamedFramebufferDrawBuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glNamedFramebufferDrawBuffer\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glNamedFramebufferDrawBuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTextureSubImage2D: unsafe {
                unsafe extern "system" fn __glTextureSubImage2D(
                    _texture: GLuint,
                    _level: GLint,
                    _xoffset: GLint,
                    _yoffset: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                    _format: PixelFormat,
                    _type: PixelType,
                    _pixels: GLvoid,
                ) {
                    panic!("Unable to load glTextureSubImage2D")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTextureSubImage2D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTextureSubImage2D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetnMapiv: unsafe {
                unsafe extern "system" fn __glGetnMapiv(
                    _target: MapTarget,
                    _query: MapQuery,
                    _bufSize: GLsizei,
                    _v: GLint,
                ) {
                    panic!("Unable to load glGetnMapiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetnMapiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetnMapiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTransformFeedbackBufferBase: unsafe {
                unsafe extern "system" fn __glTransformFeedbackBufferBase(
                    _xfb: GLuint,
                    _index: GLuint,
                    _buffer: GLuint,
                ) {
                    panic!("Unable to load glTransformFeedbackBufferBase")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glTransformFeedbackBufferBase\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glTransformFeedbackBufferBase
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBindTextureUnit: unsafe {
                unsafe extern "system" fn __glBindTextureUnit(_unit: GLuint, _texture: GLuint) {
                    panic!("Unable to load glBindTextureUnit")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindTextureUnit\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindTextureUnit
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetTransformFeedbackiv: unsafe {
                unsafe extern "system" fn __glGetTransformFeedbackiv(
                    _xfb: GLuint,
                    _pname: TransformFeedbackPName,
                    _param: GLint,
                ) {
                    panic!("Unable to load glGetTransformFeedbackiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTransformFeedbackiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTransformFeedbackiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTextureParameterIuiv: unsafe {
                unsafe extern "system" fn __glTextureParameterIuiv(
                    _texture: GLuint,
                    _pname: TextureParameterName,
                    _params: GLuint,
                ) {
                    panic!("Unable to load glTextureParameterIuiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTextureParameterIuiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTextureParameterIuiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMapNamedBufferRange: unsafe {
                unsafe extern "system" fn __glMapNamedBufferRange(
                    _buffer: GLuint,
                    _offset: GLintptr,
                    _length: GLsizeiptr,
                    _access: MapBufferAccessMask,
                ) {
                    panic!("Unable to load glMapNamedBufferRange")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMapNamedBufferRange\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMapNamedBufferRange
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTextureBuffer: unsafe {
                unsafe extern "system" fn __glTextureBuffer(
                    _texture: GLuint,
                    _internalformat: SizedInternalFormat,
                    _buffer: GLuint,
                ) {
                    panic!("Unable to load glTextureBuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTextureBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTextureBuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetNamedBufferPointerv: unsafe {
                unsafe extern "system" fn __glGetNamedBufferPointerv(
                    _buffer: GLuint,
                    _pname: BufferPointerNameARB,
                    _params: *mut GLvoid,
                ) {
                    panic!("Unable to load glGetNamedBufferPointerv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetNamedBufferPointerv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetNamedBufferPointerv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTextureParameterf: unsafe {
                unsafe extern "system" fn __glTextureParameterf(
                    _texture: GLuint,
                    _pname: TextureParameterName,
                    _param: GLfloat,
                ) {
                    panic!("Unable to load glTextureParameterf")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTextureParameterf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTextureParameterf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTextureSubImage1D: unsafe {
                unsafe extern "system" fn __glTextureSubImage1D(
                    _texture: GLuint,
                    _level: GLint,
                    _xoffset: GLint,
                    _width: GLsizei,
                    _format: PixelFormat,
                    _type: PixelType,
                    _pixels: GLvoid,
                ) {
                    panic!("Unable to load glTextureSubImage1D")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTextureSubImage1D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTextureSubImage1D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexArrayBindingDivisor: unsafe {
                unsafe extern "system" fn __glVertexArrayBindingDivisor(
                    _vaobj: GLuint,
                    _bindingindex: GLuint,
                    _divisor: GLuint,
                ) {
                    panic!("Unable to load glVertexArrayBindingDivisor")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glVertexArrayBindingDivisor\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glVertexArrayBindingDivisor
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetnCompressedTexImage: unsafe {
                unsafe extern "system" fn __glGetnCompressedTexImage(
                    _target: TextureTarget,
                    _lod: GLint,
                    _bufSize: GLsizei,
                    _pixels: *mut GLvoid,
                ) {
                    panic!("Unable to load glGetnCompressedTexImage")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetnCompressedTexImage\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetnCompressedTexImage
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetnHistogram: unsafe {
                unsafe extern "system" fn __glGetnHistogram(
                    _target: HistogramTarget,
                    _reset: Boolean,
                    _format: PixelFormat,
                    _type: PixelType,
                    _bufSize: GLsizei,
                    _values: *mut GLvoid,
                ) {
                    panic!("Unable to load glGetnHistogram")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetnHistogram\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetnHistogram
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTextureBufferRange: unsafe {
                unsafe extern "system" fn __glTextureBufferRange(
                    _texture: GLuint,
                    _internalformat: SizedInternalFormat,
                    _buffer: GLuint,
                    _offset: GLintptr,
                    _size: GLsizeiptr,
                ) {
                    panic!("Unable to load glTextureBufferRange")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTextureBufferRange\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTextureBufferRange
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTextureStorage3D: unsafe {
                unsafe extern "system" fn __glTextureStorage3D(
                    _texture: GLuint,
                    _levels: GLsizei,
                    _internalformat: SizedInternalFormat,
                    _width: GLsizei,
                    _height: GLsizei,
                    _depth: GLsizei,
                ) {
                    panic!("Unable to load glTextureStorage3D")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTextureStorage3D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTextureStorage3D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetnMinmax: unsafe {
                unsafe extern "system" fn __glGetnMinmax(
                    _target: MinmaxTarget,
                    _reset: Boolean,
                    _format: PixelFormat,
                    _type: PixelType,
                    _bufSize: GLsizei,
                    _values: *mut GLvoid,
                ) {
                    panic!("Unable to load glGetnMinmax")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetnMinmax\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetnMinmax
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCheckNamedFramebufferStatus: unsafe {
                unsafe extern "system" fn __glCheckNamedFramebufferStatus(
                    _framebuffer: GLuint,
                    _target: FramebufferTarget,
                ) -> GLenum {
                    panic!("Unable to load glCheckNamedFramebufferStatus")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glCheckNamedFramebufferStatus\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glCheckNamedFramebufferStatus
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetnUniformuiv: unsafe {
                unsafe extern "system" fn __glGetnUniformuiv(
                    _program: GLuint,
                    _location: GLint,
                    _bufSize: GLsizei,
                    _params: *mut GLuint,
                ) {
                    panic!("Unable to load glGetnUniformuiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetnUniformuiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetnUniformuiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetNamedBufferParameteri64v: unsafe {
                unsafe extern "system" fn __glGetNamedBufferParameteri64v(
                    _buffer: GLuint,
                    _pname: BufferPNameARB,
                    _params: GLint64,
                ) {
                    panic!("Unable to load glGetNamedBufferParameteri64v")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetNamedBufferParameteri64v\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetNamedBufferParameteri64v
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBlitNamedFramebuffer: unsafe {
                unsafe extern "system" fn __glBlitNamedFramebuffer(
                    _readFramebuffer: GLuint,
                    _drawFramebuffer: GLuint,
                    _srcX0: GLint,
                    _srcY0: GLint,
                    _srcX1: GLint,
                    _srcY1: GLint,
                    _dstX0: GLint,
                    _dstY0: GLint,
                    _dstX1: GLint,
                    _dstY1: GLint,
                    _mask: ClearBufferMask,
                    _filter: BlitFramebufferFilter,
                ) {
                    panic!("Unable to load glBlitNamedFramebuffer")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBlitNamedFramebuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBlitNamedFramebuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTextureStorage2D: unsafe {
                unsafe extern "system" fn __glTextureStorage2D(
                    _texture: GLuint,
                    _levels: GLsizei,
                    _internalformat: SizedInternalFormat,
                    _width: GLsizei,
                    _height: GLsizei,
                ) {
                    panic!("Unable to load glTextureStorage2D")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTextureStorage2D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTextureStorage2D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetCompressedTextureImage: unsafe {
                unsafe extern "system" fn __glGetCompressedTextureImage(
                    _texture: GLuint,
                    _level: GLint,
                    _bufSize: GLsizei,
                    _pixels: GLvoid,
                ) {
                    panic!("Unable to load glGetCompressedTextureImage")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetCompressedTextureImage\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetCompressedTextureImage
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCompressedTextureSubImage1D: unsafe {
                unsafe extern "system" fn __glCompressedTextureSubImage1D(
                    _texture: GLuint,
                    _level: GLint,
                    _xoffset: GLint,
                    _width: GLsizei,
                    _format: InternalFormat,
                    _imageSize: GLsizei,
                    _data: GLvoid,
                ) {
                    panic!("Unable to load glCompressedTextureSubImage1D")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glCompressedTextureSubImage1D\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glCompressedTextureSubImage1D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexArrayAttribBinding: unsafe {
                unsafe extern "system" fn __glVertexArrayAttribBinding(
                    _vaobj: GLuint,
                    _attribindex: GLuint,
                    _bindingindex: GLuint,
                ) {
                    panic!("Unable to load glVertexArrayAttribBinding")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glVertexArrayAttribBinding\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glVertexArrayAttribBinding
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glNamedFramebufferReadBuffer: unsafe {
                unsafe extern "system" fn __glNamedFramebufferReadBuffer(
                    _framebuffer: GLuint,
                    _src: ColorBuffer,
                ) {
                    panic!("Unable to load glNamedFramebufferReadBuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glNamedFramebufferReadBuffer\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glNamedFramebufferReadBuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glClearNamedFramebufferfv: unsafe {
                unsafe extern "system" fn __glClearNamedFramebufferfv(
                    _framebuffer: GLuint,
                    _buffer: Buffer,
                    _drawbuffer: GLint,
                    _value: GLfloat,
                ) {
                    panic!("Unable to load glClearNamedFramebufferfv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClearNamedFramebufferfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClearNamedFramebufferfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glNamedBufferSubData: unsafe {
                unsafe extern "system" fn __glNamedBufferSubData(
                    _buffer: GLuint,
                    _offset: GLintptr,
                    _size: GLsizeiptr,
                    _data: *mut GLvoid,
                ) {
                    panic!("Unable to load glNamedBufferSubData")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glNamedBufferSubData\0");
                let val = _f(cname);
                if val.is_null() {
                    __glNamedBufferSubData
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glClearNamedFramebufferuiv: unsafe {
                unsafe extern "system" fn __glClearNamedFramebufferuiv(
                    _framebuffer: GLuint,
                    _buffer: Buffer,
                    _drawbuffer: GLint,
                    _value: GLuint,
                ) {
                    panic!("Unable to load glClearNamedFramebufferuiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glClearNamedFramebufferuiv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glClearNamedFramebufferuiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCopyTextureSubImage1D: unsafe {
                unsafe extern "system" fn __glCopyTextureSubImage1D(
                    _texture: GLuint,
                    _level: GLint,
                    _xoffset: GLint,
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                ) {
                    panic!("Unable to load glCopyTextureSubImage1D")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCopyTextureSubImage1D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCopyTextureSubImage1D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetVertexArrayIndexed64iv: unsafe {
                unsafe extern "system" fn __glGetVertexArrayIndexed64iv(
                    _vaobj: GLuint,
                    _index: GLuint,
                    _pname: VertexArrayPName,
                    _param: GLint64,
                ) {
                    panic!("Unable to load glGetVertexArrayIndexed64iv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetVertexArrayIndexed64iv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetVertexArrayIndexed64iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCreateBuffers: unsafe {
                unsafe extern "system" fn __glCreateBuffers(_n: GLsizei, _buffers: *mut GLuint) {
                    panic!("Unable to load glCreateBuffers")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCreateBuffers\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCreateBuffers
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCreateFramebuffers: unsafe {
                unsafe extern "system" fn __glCreateFramebuffers(
                    _n: GLsizei,
                    _framebuffers: *mut GLuint,
                ) {
                    panic!("Unable to load glCreateFramebuffers")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCreateFramebuffers\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCreateFramebuffers
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTextureSubImage3D: unsafe {
                unsafe extern "system" fn __glTextureSubImage3D(
                    _texture: GLuint,
                    _level: GLint,
                    _xoffset: GLint,
                    _yoffset: GLint,
                    _zoffset: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                    _depth: GLsizei,
                    _format: PixelFormat,
                    _type: PixelType,
                    _pixels: GLvoid,
                ) {
                    panic!("Unable to load glTextureSubImage3D")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTextureSubImage3D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTextureSubImage3D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTextureParameteriv: unsafe {
                unsafe extern "system" fn __glTextureParameteriv(
                    _texture: GLuint,
                    _pname: TextureParameterName,
                    _param: GLint,
                ) {
                    panic!("Unable to load glTextureParameteriv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTextureParameteriv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTextureParameteriv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetGraphicsResetStatus: unsafe {
                unsafe extern "system" fn __glGetGraphicsResetStatus() -> GLenum {
                    panic!("Unable to load glGetGraphicsResetStatus")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetGraphicsResetStatus\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetGraphicsResetStatus
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetnConvolutionFilter: unsafe {
                unsafe extern "system" fn __glGetnConvolutionFilter(
                    _target: ConvolutionTarget,
                    _format: PixelFormat,
                    _type: PixelType,
                    _bufSize: GLsizei,
                    _image: *mut GLvoid,
                ) {
                    panic!("Unable to load glGetnConvolutionFilter")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetnConvolutionFilter\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetnConvolutionFilter
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glClipControl: unsafe {
                unsafe extern "system" fn __glClipControl(
                    _origin: ClipControlOrigin,
                    _depth: ClipControlDepth,
                ) {
                    panic!("Unable to load glClipControl")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClipControl\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClipControl
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glNamedFramebufferParameteri: unsafe {
                unsafe extern "system" fn __glNamedFramebufferParameteri(
                    _framebuffer: GLuint,
                    _pname: FramebufferParameterName,
                    _param: GLint,
                ) {
                    panic!("Unable to load glNamedFramebufferParameteri")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glNamedFramebufferParameteri\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glNamedFramebufferParameteri
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetTextureLevelParameteriv: unsafe {
                unsafe extern "system" fn __glGetTextureLevelParameteriv(
                    _texture: GLuint,
                    _level: GLint,
                    _pname: GetTextureParameter,
                    _params: GLint,
                ) {
                    panic!("Unable to load glGetTextureLevelParameteriv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetTextureLevelParameteriv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetTextureLevelParameteriv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCopyTextureSubImage3D: unsafe {
                unsafe extern "system" fn __glCopyTextureSubImage3D(
                    _texture: GLuint,
                    _level: GLint,
                    _xoffset: GLint,
                    _yoffset: GLint,
                    _zoffset: GLint,
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                ) {
                    panic!("Unable to load glCopyTextureSubImage3D")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCopyTextureSubImage3D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCopyTextureSubImage3D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glNamedBufferStorage: unsafe {
                unsafe extern "system" fn __glNamedBufferStorage(
                    _buffer: GLuint,
                    _size: GLsizeiptr,
                    _data: *mut GLvoid,
                    _flags: BufferStorageMask,
                ) {
                    panic!("Unable to load glNamedBufferStorage")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glNamedBufferStorage\0");
                let val = _f(cname);
                if val.is_null() {
                    __glNamedBufferStorage
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetNamedBufferParameteriv: unsafe {
                unsafe extern "system" fn __glGetNamedBufferParameteriv(
                    _buffer: GLuint,
                    _pname: BufferPNameARB,
                    _params: GLint,
                ) {
                    panic!("Unable to load glGetNamedBufferParameteriv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetNamedBufferParameteriv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetNamedBufferParameteriv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexArrayVertexBuffer: unsafe {
                unsafe extern "system" fn __glVertexArrayVertexBuffer(
                    _vaobj: GLuint,
                    _bindingindex: GLuint,
                    _buffer: GLuint,
                    _offset: GLintptr,
                    _stride: GLsizei,
                ) {
                    panic!("Unable to load glVertexArrayVertexBuffer")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexArrayVertexBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexArrayVertexBuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetNamedRenderbufferParameteriv: unsafe {
                unsafe extern "system" fn __glGetNamedRenderbufferParameteriv(
                    _renderbuffer: GLuint,
                    _pname: RenderbufferParameterName,
                    _params: GLint,
                ) {
                    panic!("Unable to load glGetNamedRenderbufferParameteriv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetNamedRenderbufferParameteriv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetNamedRenderbufferParameteriv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetnPixelMapfv: unsafe {
                unsafe extern "system" fn __glGetnPixelMapfv(
                    _map: PixelMap,
                    _bufSize: GLsizei,
                    _values: *mut GLfloat,
                ) {
                    panic!("Unable to load glGetnPixelMapfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetnPixelMapfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetnPixelMapfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDisableVertexArrayAttrib: unsafe {
                unsafe extern "system" fn __glDisableVertexArrayAttrib(
                    _vaobj: GLuint,
                    _index: GLuint,
                ) {
                    panic!("Unable to load glDisableVertexArrayAttrib")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glDisableVertexArrayAttrib\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glDisableVertexArrayAttrib
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCreateQueries: unsafe {
                unsafe extern "system" fn __glCreateQueries(
                    _target: QueryTarget,
                    _n: GLsizei,
                    _ids: *mut GLuint,
                ) {
                    panic!("Unable to load glCreateQueries")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCreateQueries\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCreateQueries
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCreateTextures: unsafe {
                unsafe extern "system" fn __glCreateTextures(
                    _target: TextureTarget,
                    _n: GLsizei,
                    _textures: *mut GLuint,
                ) {
                    panic!("Unable to load glCreateTextures")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCreateTextures\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCreateTextures
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glNamedFramebufferDrawBuffers: unsafe {
                unsafe extern "system" fn __glNamedFramebufferDrawBuffers(
                    _framebuffer: GLuint,
                    _n: GLsizei,
                    _bufs: ColorBuffer,
                ) {
                    panic!("Unable to load glNamedFramebufferDrawBuffers")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glNamedFramebufferDrawBuffers\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glNamedFramebufferDrawBuffers
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetNamedFramebufferParameteriv: unsafe {
                unsafe extern "system" fn __glGetNamedFramebufferParameteriv(
                    _framebuffer: GLuint,
                    _pname: GetFramebufferParameter,
                    _param: GLint,
                ) {
                    panic!("Unable to load glGetNamedFramebufferParameteriv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetNamedFramebufferParameteriv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetNamedFramebufferParameteriv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGenerateTextureMipmap: unsafe {
                unsafe extern "system" fn __glGenerateTextureMipmap(_texture: GLuint) {
                    panic!("Unable to load glGenerateTextureMipmap")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGenerateTextureMipmap\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGenerateTextureMipmap
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[derive(Clone)]
pub struct EntryFnGL30 {
    pub glEnablei: crate::gl::command::PFN_glEnablei,
    pub glDeleteRenderbuffers: crate::gl::command::PFN_glDeleteRenderbuffers,
    pub glGetRenderbufferParameteriv: crate::gl::command::PFN_glGetRenderbufferParameteriv,
    pub glGetFramebufferAttachmentParameteriv:
        crate::gl::command::PFN_glGetFramebufferAttachmentParameteriv,
    pub glUniform4ui: crate::gl::command::PFN_glUniform4ui,
    pub glUniform2uiv: crate::gl::command::PFN_glUniform2uiv,
    pub glBindBufferRange: crate::gl::command::PFN_glBindBufferRange,
    pub glFramebufferTextureLayer: crate::gl::command::PFN_glFramebufferTextureLayer,
    pub glDisablei: crate::gl::command::PFN_glDisablei,
    pub glVertexAttribI1iv: crate::gl::command::PFN_glVertexAttribI1iv,
    pub glDeleteVertexArrays: crate::gl::command::PFN_glDeleteVertexArrays,
    pub glGetVertexAttribIuiv: crate::gl::command::PFN_glGetVertexAttribIuiv,
    pub glBeginTransformFeedback: crate::gl::command::PFN_glBeginTransformFeedback,
    pub glVertexAttribI2uiv: crate::gl::command::PFN_glVertexAttribI2uiv,
    pub glVertexAttribI4bv: crate::gl::command::PFN_glVertexAttribI4bv,
    pub glBindVertexArray: crate::gl::command::PFN_glBindVertexArray,
    pub glEndConditionalRender: crate::gl::command::PFN_glEndConditionalRender,
    pub glGetTexParameterIuiv: crate::gl::command::PFN_glGetTexParameterIuiv,
    pub glBindFramebuffer: crate::gl::command::PFN_glBindFramebuffer,
    pub glVertexAttribI4usv: crate::gl::command::PFN_glVertexAttribI4usv,
    pub glVertexAttribI4ubv: crate::gl::command::PFN_glVertexAttribI4ubv,
    pub glIsVertexArray: crate::gl::command::PFN_glIsVertexArray,
    pub glUniform2ui: crate::gl::command::PFN_glUniform2ui,
    pub glVertexAttribI4i: crate::gl::command::PFN_glVertexAttribI4i,
    pub glBeginConditionalRender: crate::gl::command::PFN_glBeginConditionalRender,
    pub glGenerateMipmap: crate::gl::command::PFN_glGenerateMipmap,
    pub glVertexAttribI1ui: crate::gl::command::PFN_glVertexAttribI1ui,
    pub glIsRenderbuffer: crate::gl::command::PFN_glIsRenderbuffer,
    pub glTexParameterIuiv: crate::gl::command::PFN_glTexParameterIuiv,
    pub glFramebufferTexture3D: crate::gl::command::PFN_glFramebufferTexture3D,
    pub glClearBufferiv: crate::gl::command::PFN_glClearBufferiv,
    pub glRenderbufferStorageMultisample: crate::gl::command::PFN_glRenderbufferStorageMultisample,
    pub glVertexAttribI2iv: crate::gl::command::PFN_glVertexAttribI2iv,
    pub glVertexAttribI3ui: crate::gl::command::PFN_glVertexAttribI3ui,
    pub glVertexAttribI4ui: crate::gl::command::PFN_glVertexAttribI4ui,
    pub glGetFragDataLocation: crate::gl::command::PFN_glGetFragDataLocation,
    pub glTransformFeedbackVaryings: crate::gl::command::PFN_glTransformFeedbackVaryings,
    pub glRenderbufferStorage: crate::gl::command::PFN_glRenderbufferStorage,
    pub glVertexAttribI3i: crate::gl::command::PFN_glVertexAttribI3i,
    pub glVertexAttribI1uiv: crate::gl::command::PFN_glVertexAttribI1uiv,
    pub glMapBufferRange: crate::gl::command::PFN_glMapBufferRange,
    pub glFramebufferRenderbuffer: crate::gl::command::PFN_glFramebufferRenderbuffer,
    pub glGenVertexArrays: crate::gl::command::PFN_glGenVertexArrays,
    pub glIsEnabledi: crate::gl::command::PFN_glIsEnabledi,
    pub glDeleteFramebuffers: crate::gl::command::PFN_glDeleteFramebuffers,
    pub glColorMaski: crate::gl::command::PFN_glColorMaski,
    pub glBlitFramebuffer: crate::gl::command::PFN_glBlitFramebuffer,
    pub glGetIntegeri_v: crate::gl::command::PFN_glGetIntegeri_v,
    pub glUniform1ui: crate::gl::command::PFN_glUniform1ui,
    pub glCheckFramebufferStatus: crate::gl::command::PFN_glCheckFramebufferStatus,
    pub glClearBufferuiv: crate::gl::command::PFN_glClearBufferuiv,
    pub glVertexAttribI2i: crate::gl::command::PFN_glVertexAttribI2i,
    pub glVertexAttribI2ui: crate::gl::command::PFN_glVertexAttribI2ui,
    pub glUniform3uiv: crate::gl::command::PFN_glUniform3uiv,
    pub glBindRenderbuffer: crate::gl::command::PFN_glBindRenderbuffer,
    pub glTexParameterIiv: crate::gl::command::PFN_glTexParameterIiv,
    pub glClearBufferfv: crate::gl::command::PFN_glClearBufferfv,
    pub glGenRenderbuffers: crate::gl::command::PFN_glGenRenderbuffers,
    pub glFramebufferTexture1D: crate::gl::command::PFN_glFramebufferTexture1D,
    pub glUniform4uiv: crate::gl::command::PFN_glUniform4uiv,
    pub glClearBufferfi: crate::gl::command::PFN_glClearBufferfi,
    pub glVertexAttribI1i: crate::gl::command::PFN_glVertexAttribI1i,
    pub glUniform1uiv: crate::gl::command::PFN_glUniform1uiv,
    pub glGetVertexAttribIiv: crate::gl::command::PFN_glGetVertexAttribIiv,
    pub glBindBufferBase: crate::gl::command::PFN_glBindBufferBase,
    pub glIsFramebuffer: crate::gl::command::PFN_glIsFramebuffer,
    pub glEndTransformFeedback: crate::gl::command::PFN_glEndTransformFeedback,
    pub glVertexAttribIPointer: crate::gl::command::PFN_glVertexAttribIPointer,
    pub glVertexAttribI4uiv: crate::gl::command::PFN_glVertexAttribI4uiv,
    pub glGetTexParameterIiv: crate::gl::command::PFN_glGetTexParameterIiv,
    pub glGetStringi: crate::gl::command::PFN_glGetStringi,
    pub glVertexAttribI3iv: crate::gl::command::PFN_glVertexAttribI3iv,
    pub glVertexAttribI4iv: crate::gl::command::PFN_glVertexAttribI4iv,
    pub glGetBooleani_v: crate::gl::command::PFN_glGetBooleani_v,
    pub glBindFragDataLocation: crate::gl::command::PFN_glBindFragDataLocation,
    pub glVertexAttribI4sv: crate::gl::command::PFN_glVertexAttribI4sv,
    pub glUniform3ui: crate::gl::command::PFN_glUniform3ui,
    pub glClampColor: crate::gl::command::PFN_glClampColor,
    pub glGetUniformuiv: crate::gl::command::PFN_glGetUniformuiv,
    pub glFlushMappedBufferRange: crate::gl::command::PFN_glFlushMappedBufferRange,
    pub glGenFramebuffers: crate::gl::command::PFN_glGenFramebuffers,
    pub glFramebufferTexture2D: crate::gl::command::PFN_glFramebufferTexture2D,
    pub glGetTransformFeedbackVarying: crate::gl::command::PFN_glGetTransformFeedbackVarying,
    pub glVertexAttribI3uiv: crate::gl::command::PFN_glVertexAttribI3uiv,
}
impl EntryFnGL30 {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            glEnablei: unsafe {
                unsafe extern "system" fn __glEnablei(_target: EnableCap, _index: GLuint) {
                    panic!("Unable to load glEnablei")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEnablei\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEnablei
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDeleteRenderbuffers: unsafe {
                unsafe extern "system" fn __glDeleteRenderbuffers(
                    _n: GLsizei,
                    _renderbuffers: *mut GLuint,
                ) {
                    panic!("Unable to load glDeleteRenderbuffers")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDeleteRenderbuffers\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDeleteRenderbuffers
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetRenderbufferParameteriv: unsafe {
                unsafe extern "system" fn __glGetRenderbufferParameteriv(
                    _target: RenderbufferTarget,
                    _pname: RenderbufferParameterName,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetRenderbufferParameteriv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetRenderbufferParameteriv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetRenderbufferParameteriv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetFramebufferAttachmentParameteriv: unsafe {
                unsafe extern "system" fn __glGetFramebufferAttachmentParameteriv(
                    _target: FramebufferTarget,
                    _attachment: FramebufferAttachment,
                    _pname: FramebufferAttachmentParameterName,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetFramebufferAttachmentParameteriv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetFramebufferAttachmentParameteriv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetFramebufferAttachmentParameteriv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform4ui: unsafe {
                unsafe extern "system" fn __glUniform4ui(
                    _location: GLint,
                    _v0: GLuint,
                    _v1: GLuint,
                    _v2: GLuint,
                    _v3: GLuint,
                ) {
                    panic!("Unable to load glUniform4ui")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform4ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform4ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform2uiv: unsafe {
                unsafe extern "system" fn __glUniform2uiv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLuint,
                ) {
                    panic!("Unable to load glUniform2uiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform2uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform2uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBindBufferRange: unsafe {
                unsafe extern "system" fn __glBindBufferRange(
                    _target: BufferTargetARB,
                    _index: GLuint,
                    _buffer: GLuint,
                    _offset: GLintptr,
                    _size: GLsizeiptr,
                ) {
                    panic!("Unable to load glBindBufferRange")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindBufferRange\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindBufferRange
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glFramebufferTextureLayer: unsafe {
                unsafe extern "system" fn __glFramebufferTextureLayer(
                    _target: FramebufferTarget,
                    _attachment: FramebufferAttachment,
                    _texture: GLuint,
                    _level: GLint,
                    _layer: GLint,
                ) {
                    panic!("Unable to load glFramebufferTextureLayer")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFramebufferTextureLayer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFramebufferTextureLayer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDisablei: unsafe {
                unsafe extern "system" fn __glDisablei(_target: EnableCap, _index: GLuint) {
                    panic!("Unable to load glDisablei")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDisablei\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDisablei
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribI1iv: unsafe {
                unsafe extern "system" fn __glVertexAttribI1iv(_index: GLuint, _v: *mut GLint) {
                    panic!("Unable to load glVertexAttribI1iv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI1iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribI1iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDeleteVertexArrays: unsafe {
                unsafe extern "system" fn __glDeleteVertexArrays(
                    _n: GLsizei,
                    _arrays: *mut GLuint,
                ) {
                    panic!("Unable to load glDeleteVertexArrays")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDeleteVertexArrays\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDeleteVertexArrays
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetVertexAttribIuiv: unsafe {
                unsafe extern "system" fn __glGetVertexAttribIuiv(
                    _index: GLuint,
                    _pname: VertexAttribEnum,
                    _params: *mut GLuint,
                ) {
                    panic!("Unable to load glGetVertexAttribIuiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetVertexAttribIuiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetVertexAttribIuiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBeginTransformFeedback: unsafe {
                unsafe extern "system" fn __glBeginTransformFeedback(
                    _primitiveMode: PrimitiveType,
                ) {
                    panic!("Unable to load glBeginTransformFeedback")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBeginTransformFeedback\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBeginTransformFeedback
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribI2uiv: unsafe {
                unsafe extern "system" fn __glVertexAttribI2uiv(_index: GLuint, _v: *mut GLuint) {
                    panic!("Unable to load glVertexAttribI2uiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI2uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribI2uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribI4bv: unsafe {
                unsafe extern "system" fn __glVertexAttribI4bv(_index: GLuint, _v: *mut GLbyte) {
                    panic!("Unable to load glVertexAttribI4bv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI4bv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribI4bv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBindVertexArray: unsafe {
                unsafe extern "system" fn __glBindVertexArray(_array: GLuint) {
                    panic!("Unable to load glBindVertexArray")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindVertexArray\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindVertexArray
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glEndConditionalRender: unsafe {
                unsafe extern "system" fn __glEndConditionalRender() {
                    panic!("Unable to load glEndConditionalRender")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEndConditionalRender\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEndConditionalRender
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetTexParameterIuiv: unsafe {
                unsafe extern "system" fn __glGetTexParameterIuiv(
                    _target: TextureTarget,
                    _pname: GetTextureParameter,
                    _params: *mut GLuint,
                ) {
                    panic!("Unable to load glGetTexParameterIuiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTexParameterIuiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTexParameterIuiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBindFramebuffer: unsafe {
                unsafe extern "system" fn __glBindFramebuffer(
                    _target: FramebufferTarget,
                    _framebuffer: GLuint,
                ) {
                    panic!("Unable to load glBindFramebuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindFramebuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindFramebuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribI4usv: unsafe {
                unsafe extern "system" fn __glVertexAttribI4usv(_index: GLuint, _v: *mut GLushort) {
                    panic!("Unable to load glVertexAttribI4usv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI4usv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribI4usv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribI4ubv: unsafe {
                unsafe extern "system" fn __glVertexAttribI4ubv(_index: GLuint, _v: *mut GLubyte) {
                    panic!("Unable to load glVertexAttribI4ubv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI4ubv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribI4ubv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glIsVertexArray: unsafe {
                unsafe extern "system" fn __glIsVertexArray(_array: GLuint) -> GLboolean {
                    panic!("Unable to load glIsVertexArray")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIsVertexArray\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIsVertexArray
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform2ui: unsafe {
                unsafe extern "system" fn __glUniform2ui(
                    _location: GLint,
                    _v0: GLuint,
                    _v1: GLuint,
                ) {
                    panic!("Unable to load glUniform2ui")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform2ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform2ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribI4i: unsafe {
                unsafe extern "system" fn __glVertexAttribI4i(
                    _index: GLuint,
                    _x: GLint,
                    _y: GLint,
                    _z: GLint,
                    _w: GLint,
                ) {
                    panic!("Unable to load glVertexAttribI4i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI4i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribI4i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBeginConditionalRender: unsafe {
                unsafe extern "system" fn __glBeginConditionalRender(
                    _id: GLuint,
                    _mode: ConditionalRenderMode,
                ) {
                    panic!("Unable to load glBeginConditionalRender")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBeginConditionalRender\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBeginConditionalRender
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGenerateMipmap: unsafe {
                unsafe extern "system" fn __glGenerateMipmap(_target: TextureTarget) {
                    panic!("Unable to load glGenerateMipmap")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGenerateMipmap\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGenerateMipmap
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribI1ui: unsafe {
                unsafe extern "system" fn __glVertexAttribI1ui(_index: GLuint, _x: GLuint) {
                    panic!("Unable to load glVertexAttribI1ui")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI1ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribI1ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glIsRenderbuffer: unsafe {
                unsafe extern "system" fn __glIsRenderbuffer(_renderbuffer: GLuint) -> GLboolean {
                    panic!("Unable to load glIsRenderbuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIsRenderbuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIsRenderbuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexParameterIuiv: unsafe {
                unsafe extern "system" fn __glTexParameterIuiv(
                    _target: TextureTarget,
                    _pname: TextureParameterName,
                    _params: *mut GLuint,
                ) {
                    panic!("Unable to load glTexParameterIuiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexParameterIuiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexParameterIuiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glFramebufferTexture3D: unsafe {
                unsafe extern "system" fn __glFramebufferTexture3D(
                    _target: FramebufferTarget,
                    _attachment: FramebufferAttachment,
                    _textarget: TextureTarget,
                    _texture: GLuint,
                    _level: GLint,
                    _zoffset: GLint,
                ) {
                    panic!("Unable to load glFramebufferTexture3D")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFramebufferTexture3D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFramebufferTexture3D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glClearBufferiv: unsafe {
                unsafe extern "system" fn __glClearBufferiv(
                    _buffer: Buffer,
                    _drawbuffer: GLint,
                    _value: *mut GLint,
                ) {
                    panic!("Unable to load glClearBufferiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClearBufferiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClearBufferiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRenderbufferStorageMultisample: unsafe {
                unsafe extern "system" fn __glRenderbufferStorageMultisample(
                    _target: RenderbufferTarget,
                    _samples: GLsizei,
                    _internalformat: InternalFormat,
                    _width: GLsizei,
                    _height: GLsizei,
                ) {
                    panic!("Unable to load glRenderbufferStorageMultisample")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glRenderbufferStorageMultisample\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glRenderbufferStorageMultisample
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribI2iv: unsafe {
                unsafe extern "system" fn __glVertexAttribI2iv(_index: GLuint, _v: *mut GLint) {
                    panic!("Unable to load glVertexAttribI2iv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI2iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribI2iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribI3ui: unsafe {
                unsafe extern "system" fn __glVertexAttribI3ui(
                    _index: GLuint,
                    _x: GLuint,
                    _y: GLuint,
                    _z: GLuint,
                ) {
                    panic!("Unable to load glVertexAttribI3ui")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI3ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribI3ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribI4ui: unsafe {
                unsafe extern "system" fn __glVertexAttribI4ui(
                    _index: GLuint,
                    _x: GLuint,
                    _y: GLuint,
                    _z: GLuint,
                    _w: GLuint,
                ) {
                    panic!("Unable to load glVertexAttribI4ui")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI4ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribI4ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetFragDataLocation: unsafe {
                unsafe extern "system" fn __glGetFragDataLocation(
                    _program: GLuint,
                    _name: *mut GLchar,
                ) -> GLint {
                    panic!("Unable to load glGetFragDataLocation")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetFragDataLocation\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetFragDataLocation
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTransformFeedbackVaryings: unsafe {
                unsafe extern "system" fn __glTransformFeedbackVaryings(
                    _program: GLuint,
                    _count: GLsizei,
                    _varyings: *mut GLchar,
                    _bufferMode: TransformFeedbackBufferMode,
                ) {
                    panic!("Unable to load glTransformFeedbackVaryings")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glTransformFeedbackVaryings\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glTransformFeedbackVaryings
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRenderbufferStorage: unsafe {
                unsafe extern "system" fn __glRenderbufferStorage(
                    _target: RenderbufferTarget,
                    _internalformat: InternalFormat,
                    _width: GLsizei,
                    _height: GLsizei,
                ) {
                    panic!("Unable to load glRenderbufferStorage")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRenderbufferStorage\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRenderbufferStorage
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribI3i: unsafe {
                unsafe extern "system" fn __glVertexAttribI3i(
                    _index: GLuint,
                    _x: GLint,
                    _y: GLint,
                    _z: GLint,
                ) {
                    panic!("Unable to load glVertexAttribI3i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI3i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribI3i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribI1uiv: unsafe {
                unsafe extern "system" fn __glVertexAttribI1uiv(_index: GLuint, _v: *mut GLuint) {
                    panic!("Unable to load glVertexAttribI1uiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI1uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribI1uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMapBufferRange: unsafe {
                unsafe extern "system" fn __glMapBufferRange(
                    _target: BufferTargetARB,
                    _offset: GLintptr,
                    _length: GLsizeiptr,
                    _access: MapBufferAccessMask,
                ) {
                    panic!("Unable to load glMapBufferRange")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMapBufferRange\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMapBufferRange
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glFramebufferRenderbuffer: unsafe {
                unsafe extern "system" fn __glFramebufferRenderbuffer(
                    _target: FramebufferTarget,
                    _attachment: FramebufferAttachment,
                    _renderbuffertarget: RenderbufferTarget,
                    _renderbuffer: GLuint,
                ) {
                    panic!("Unable to load glFramebufferRenderbuffer")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFramebufferRenderbuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFramebufferRenderbuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGenVertexArrays: unsafe {
                unsafe extern "system" fn __glGenVertexArrays(_n: GLsizei, _arrays: *mut GLuint) {
                    panic!("Unable to load glGenVertexArrays")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGenVertexArrays\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGenVertexArrays
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glIsEnabledi: unsafe {
                unsafe extern "system" fn __glIsEnabledi(
                    _target: EnableCap,
                    _index: GLuint,
                ) -> GLboolean {
                    panic!("Unable to load glIsEnabledi")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIsEnabledi\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIsEnabledi
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDeleteFramebuffers: unsafe {
                unsafe extern "system" fn __glDeleteFramebuffers(
                    _n: GLsizei,
                    _framebuffers: *mut GLuint,
                ) {
                    panic!("Unable to load glDeleteFramebuffers")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDeleteFramebuffers\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDeleteFramebuffers
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColorMaski: unsafe {
                unsafe extern "system" fn __glColorMaski(
                    _index: GLuint,
                    _r: Boolean,
                    _g: Boolean,
                    _b: Boolean,
                    _a: Boolean,
                ) {
                    panic!("Unable to load glColorMaski")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColorMaski\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColorMaski
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBlitFramebuffer: unsafe {
                unsafe extern "system" fn __glBlitFramebuffer(
                    _srcX0: GLint,
                    _srcY0: GLint,
                    _srcX1: GLint,
                    _srcY1: GLint,
                    _dstX0: GLint,
                    _dstY0: GLint,
                    _dstX1: GLint,
                    _dstY1: GLint,
                    _mask: ClearBufferMask,
                    _filter: BlitFramebufferFilter,
                ) {
                    panic!("Unable to load glBlitFramebuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBlitFramebuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBlitFramebuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetIntegeri_v: unsafe {
                unsafe extern "system" fn __glGetIntegeri_v(
                    _target: GetPName,
                    _index: GLuint,
                    _data: *mut GLint,
                ) {
                    panic!("Unable to load glGetIntegeri_v")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetIntegeri_v\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetIntegeri_v
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform1ui: unsafe {
                unsafe extern "system" fn __glUniform1ui(_location: GLint, _v0: GLuint) {
                    panic!("Unable to load glUniform1ui")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform1ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform1ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCheckFramebufferStatus: unsafe {
                unsafe extern "system" fn __glCheckFramebufferStatus(
                    _target: FramebufferTarget,
                ) -> GLenum {
                    panic!("Unable to load glCheckFramebufferStatus")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCheckFramebufferStatus\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCheckFramebufferStatus
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glClearBufferuiv: unsafe {
                unsafe extern "system" fn __glClearBufferuiv(
                    _buffer: Buffer,
                    _drawbuffer: GLint,
                    _value: *mut GLuint,
                ) {
                    panic!("Unable to load glClearBufferuiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClearBufferuiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClearBufferuiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribI2i: unsafe {
                unsafe extern "system" fn __glVertexAttribI2i(
                    _index: GLuint,
                    _x: GLint,
                    _y: GLint,
                ) {
                    panic!("Unable to load glVertexAttribI2i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI2i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribI2i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribI2ui: unsafe {
                unsafe extern "system" fn __glVertexAttribI2ui(
                    _index: GLuint,
                    _x: GLuint,
                    _y: GLuint,
                ) {
                    panic!("Unable to load glVertexAttribI2ui")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI2ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribI2ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform3uiv: unsafe {
                unsafe extern "system" fn __glUniform3uiv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLuint,
                ) {
                    panic!("Unable to load glUniform3uiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform3uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform3uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBindRenderbuffer: unsafe {
                unsafe extern "system" fn __glBindRenderbuffer(
                    _target: RenderbufferTarget,
                    _renderbuffer: GLuint,
                ) {
                    panic!("Unable to load glBindRenderbuffer")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindRenderbuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindRenderbuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexParameterIiv: unsafe {
                unsafe extern "system" fn __glTexParameterIiv(
                    _target: TextureTarget,
                    _pname: TextureParameterName,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glTexParameterIiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexParameterIiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexParameterIiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glClearBufferfv: unsafe {
                unsafe extern "system" fn __glClearBufferfv(
                    _buffer: Buffer,
                    _drawbuffer: GLint,
                    _value: *mut GLfloat,
                ) {
                    panic!("Unable to load glClearBufferfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClearBufferfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClearBufferfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGenRenderbuffers: unsafe {
                unsafe extern "system" fn __glGenRenderbuffers(
                    _n: GLsizei,
                    _renderbuffers: *mut GLuint,
                ) {
                    panic!("Unable to load glGenRenderbuffers")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGenRenderbuffers\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGenRenderbuffers
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glFramebufferTexture1D: unsafe {
                unsafe extern "system" fn __glFramebufferTexture1D(
                    _target: FramebufferTarget,
                    _attachment: FramebufferAttachment,
                    _textarget: TextureTarget,
                    _texture: GLuint,
                    _level: GLint,
                ) {
                    panic!("Unable to load glFramebufferTexture1D")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFramebufferTexture1D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFramebufferTexture1D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform4uiv: unsafe {
                unsafe extern "system" fn __glUniform4uiv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLuint,
                ) {
                    panic!("Unable to load glUniform4uiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform4uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform4uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glClearBufferfi: unsafe {
                unsafe extern "system" fn __glClearBufferfi(
                    _buffer: Buffer,
                    _drawbuffer: GLint,
                    _depth: GLfloat,
                    _stencil: GLint,
                ) {
                    panic!("Unable to load glClearBufferfi")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClearBufferfi\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClearBufferfi
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribI1i: unsafe {
                unsafe extern "system" fn __glVertexAttribI1i(_index: GLuint, _x: GLint) {
                    panic!("Unable to load glVertexAttribI1i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI1i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribI1i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform1uiv: unsafe {
                unsafe extern "system" fn __glUniform1uiv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLuint,
                ) {
                    panic!("Unable to load glUniform1uiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform1uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform1uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetVertexAttribIiv: unsafe {
                unsafe extern "system" fn __glGetVertexAttribIiv(
                    _index: GLuint,
                    _pname: VertexAttribEnum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetVertexAttribIiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetVertexAttribIiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetVertexAttribIiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBindBufferBase: unsafe {
                unsafe extern "system" fn __glBindBufferBase(
                    _target: BufferTargetARB,
                    _index: GLuint,
                    _buffer: GLuint,
                ) {
                    panic!("Unable to load glBindBufferBase")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindBufferBase\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindBufferBase
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glIsFramebuffer: unsafe {
                unsafe extern "system" fn __glIsFramebuffer(_framebuffer: GLuint) -> GLboolean {
                    panic!("Unable to load glIsFramebuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIsFramebuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIsFramebuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glEndTransformFeedback: unsafe {
                unsafe extern "system" fn __glEndTransformFeedback() {
                    panic!("Unable to load glEndTransformFeedback")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEndTransformFeedback\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEndTransformFeedback
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribIPointer: unsafe {
                unsafe extern "system" fn __glVertexAttribIPointer(
                    _index: GLuint,
                    _size: GLint,
                    _type: VertexAttribIType,
                    _stride: GLsizei,
                    _pointer: *mut GLvoid,
                ) {
                    panic!("Unable to load glVertexAttribIPointer")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribIPointer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribIPointer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribI4uiv: unsafe {
                unsafe extern "system" fn __glVertexAttribI4uiv(_index: GLuint, _v: *mut GLuint) {
                    panic!("Unable to load glVertexAttribI4uiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI4uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribI4uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetTexParameterIiv: unsafe {
                unsafe extern "system" fn __glGetTexParameterIiv(
                    _target: TextureTarget,
                    _pname: GetTextureParameter,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetTexParameterIiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTexParameterIiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTexParameterIiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetStringi: unsafe {
                unsafe extern "system" fn __glGetStringi(
                    _name: StringName,
                    _index: GLuint,
                ) -> *const char {
                    panic!("Unable to load glGetStringi")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetStringi\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetStringi
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribI3iv: unsafe {
                unsafe extern "system" fn __glVertexAttribI3iv(_index: GLuint, _v: *mut GLint) {
                    panic!("Unable to load glVertexAttribI3iv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI3iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribI3iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribI4iv: unsafe {
                unsafe extern "system" fn __glVertexAttribI4iv(_index: GLuint, _v: *mut GLint) {
                    panic!("Unable to load glVertexAttribI4iv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI4iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribI4iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetBooleani_v: unsafe {
                unsafe extern "system" fn __glGetBooleani_v(
                    _target: BufferTargetARB,
                    _index: GLuint,
                    _data: *mut Boolean,
                ) {
                    panic!("Unable to load glGetBooleani_v")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetBooleani_v\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetBooleani_v
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBindFragDataLocation: unsafe {
                unsafe extern "system" fn __glBindFragDataLocation(
                    _program: GLuint,
                    _color: GLuint,
                    _name: *mut GLchar,
                ) {
                    panic!("Unable to load glBindFragDataLocation")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindFragDataLocation\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindFragDataLocation
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribI4sv: unsafe {
                unsafe extern "system" fn __glVertexAttribI4sv(_index: GLuint, _v: *mut GLshort) {
                    panic!("Unable to load glVertexAttribI4sv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI4sv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribI4sv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform3ui: unsafe {
                unsafe extern "system" fn __glUniform3ui(
                    _location: GLint,
                    _v0: GLuint,
                    _v1: GLuint,
                    _v2: GLuint,
                ) {
                    panic!("Unable to load glUniform3ui")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform3ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform3ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glClampColor: unsafe {
                unsafe extern "system" fn __glClampColor(
                    _target: ClampColorTargetARB,
                    _clamp: ClampColorModeARB,
                ) {
                    panic!("Unable to load glClampColor")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClampColor\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClampColor
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetUniformuiv: unsafe {
                unsafe extern "system" fn __glGetUniformuiv(
                    _program: GLuint,
                    _location: GLint,
                    _params: *mut GLuint,
                ) {
                    panic!("Unable to load glGetUniformuiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetUniformuiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetUniformuiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glFlushMappedBufferRange: unsafe {
                unsafe extern "system" fn __glFlushMappedBufferRange(
                    _target: BufferTargetARB,
                    _offset: GLintptr,
                    _length: GLsizeiptr,
                ) {
                    panic!("Unable to load glFlushMappedBufferRange")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFlushMappedBufferRange\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFlushMappedBufferRange
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGenFramebuffers: unsafe {
                unsafe extern "system" fn __glGenFramebuffers(
                    _n: GLsizei,
                    _framebuffers: *mut GLuint,
                ) {
                    panic!("Unable to load glGenFramebuffers")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGenFramebuffers\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGenFramebuffers
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glFramebufferTexture2D: unsafe {
                unsafe extern "system" fn __glFramebufferTexture2D(
                    _target: FramebufferTarget,
                    _attachment: FramebufferAttachment,
                    _textarget: TextureTarget,
                    _texture: GLuint,
                    _level: GLint,
                ) {
                    panic!("Unable to load glFramebufferTexture2D")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFramebufferTexture2D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFramebufferTexture2D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetTransformFeedbackVarying: unsafe {
                unsafe extern "system" fn __glGetTransformFeedbackVarying(
                    _program: GLuint,
                    _index: GLuint,
                    _bufSize: GLsizei,
                    _length: *mut GLsizei,
                    _size: *mut GLsizei,
                    _type: *mut AttributeType,
                    _name: *mut GLchar,
                ) {
                    panic!("Unable to load glGetTransformFeedbackVarying")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetTransformFeedbackVarying\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetTransformFeedbackVarying
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribI3uiv: unsafe {
                unsafe extern "system" fn __glVertexAttribI3uiv(_index: GLuint, _v: *mut GLuint) {
                    panic!("Unable to load glVertexAttribI3uiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI3uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribI3uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[derive(Clone)]
pub struct EntryFnGL12 {
    pub glDrawRangeElements: crate::gl::command::PFN_glDrawRangeElements,
    pub glTexImage3D: crate::gl::command::PFN_glTexImage3D,
    pub glTexSubImage3D: crate::gl::command::PFN_glTexSubImage3D,
    pub glCopyTexSubImage3D: crate::gl::command::PFN_glCopyTexSubImage3D,
}
impl EntryFnGL12 {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            glDrawRangeElements: unsafe {
                unsafe extern "system" fn __glDrawRangeElements(
                    _mode: PrimitiveType,
                    _start: GLuint,
                    _end: GLuint,
                    _count: GLsizei,
                    _type: DrawElementsType,
                    _indices: *mut GLvoid,
                ) {
                    panic!("Unable to load glDrawRangeElements")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDrawRangeElements\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDrawRangeElements
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexImage3D: unsafe {
                unsafe extern "system" fn __glTexImage3D(
                    _target: TextureTarget,
                    _level: GLint,
                    _internalformat: InternalFormat,
                    _width: GLsizei,
                    _height: GLsizei,
                    _depth: GLsizei,
                    _border: GLint,
                    _format: PixelFormat,
                    _type: PixelType,
                    _pixels: *mut GLvoid,
                ) {
                    panic!("Unable to load glTexImage3D")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexImage3D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexImage3D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexSubImage3D: unsafe {
                unsafe extern "system" fn __glTexSubImage3D(
                    _target: TextureTarget,
                    _level: GLint,
                    _xoffset: GLint,
                    _yoffset: GLint,
                    _zoffset: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                    _depth: GLsizei,
                    _format: PixelFormat,
                    _type: PixelType,
                    _pixels: *mut GLvoid,
                ) {
                    panic!("Unable to load glTexSubImage3D")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexSubImage3D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexSubImage3D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCopyTexSubImage3D: unsafe {
                unsafe extern "system" fn __glCopyTexSubImage3D(
                    _target: TextureTarget,
                    _level: GLint,
                    _xoffset: GLint,
                    _yoffset: GLint,
                    _zoffset: GLint,
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                ) {
                    panic!("Unable to load glCopyTexSubImage3D")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCopyTexSubImage3D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCopyTexSubImage3D
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[derive(Clone)]
pub struct EntryFnGL21 {
    pub glUniformMatrix2x3fv: crate::gl::command::PFN_glUniformMatrix2x3fv,
    pub glUniformMatrix3x4fv: crate::gl::command::PFN_glUniformMatrix3x4fv,
    pub glUniformMatrix4x3fv: crate::gl::command::PFN_glUniformMatrix4x3fv,
    pub glUniformMatrix4x2fv: crate::gl::command::PFN_glUniformMatrix4x2fv,
    pub glUniformMatrix3x2fv: crate::gl::command::PFN_glUniformMatrix3x2fv,
    pub glUniformMatrix2x4fv: crate::gl::command::PFN_glUniformMatrix2x4fv,
}
impl EntryFnGL21 {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            glUniformMatrix2x3fv: unsafe {
                unsafe extern "system" fn __glUniformMatrix2x3fv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLfloat,
                ) {
                    panic!("Unable to load glUniformMatrix2x3fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix2x3fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformMatrix2x3fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniformMatrix3x4fv: unsafe {
                unsafe extern "system" fn __glUniformMatrix3x4fv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLfloat,
                ) {
                    panic!("Unable to load glUniformMatrix3x4fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix3x4fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformMatrix3x4fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniformMatrix4x3fv: unsafe {
                unsafe extern "system" fn __glUniformMatrix4x3fv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLfloat,
                ) {
                    panic!("Unable to load glUniformMatrix4x3fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix4x3fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformMatrix4x3fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniformMatrix4x2fv: unsafe {
                unsafe extern "system" fn __glUniformMatrix4x2fv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLfloat,
                ) {
                    panic!("Unable to load glUniformMatrix4x2fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix4x2fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformMatrix4x2fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniformMatrix3x2fv: unsafe {
                unsafe extern "system" fn __glUniformMatrix3x2fv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLfloat,
                ) {
                    panic!("Unable to load glUniformMatrix3x2fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix3x2fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformMatrix3x2fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniformMatrix2x4fv: unsafe {
                unsafe extern "system" fn __glUniformMatrix2x4fv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLfloat,
                ) {
                    panic!("Unable to load glUniformMatrix2x4fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix2x4fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformMatrix2x4fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[derive(Clone)]
pub struct EntryFnGL15 {
    pub glDeleteQueries: crate::gl::command::PFN_glDeleteQueries,
    pub glGetBufferParameteriv: crate::gl::command::PFN_glGetBufferParameteriv,
    pub glGetQueryiv: crate::gl::command::PFN_glGetQueryiv,
    pub glGenBuffers: crate::gl::command::PFN_glGenBuffers,
    pub glEndQuery: crate::gl::command::PFN_glEndQuery,
    pub glGetBufferSubData: crate::gl::command::PFN_glGetBufferSubData,
    pub glBufferData: crate::gl::command::PFN_glBufferData,
    pub glBeginQuery: crate::gl::command::PFN_glBeginQuery,
    pub glIsBuffer: crate::gl::command::PFN_glIsBuffer,
    pub glDeleteBuffers: crate::gl::command::PFN_glDeleteBuffers,
    pub glGetQueryObjectiv: crate::gl::command::PFN_glGetQueryObjectiv,
    pub glBufferSubData: crate::gl::command::PFN_glBufferSubData,
    pub glUnmapBuffer: crate::gl::command::PFN_glUnmapBuffer,
    pub glBindBuffer: crate::gl::command::PFN_glBindBuffer,
    pub glGetQueryObjectuiv: crate::gl::command::PFN_glGetQueryObjectuiv,
    pub glIsQuery: crate::gl::command::PFN_glIsQuery,
    pub glMapBuffer: crate::gl::command::PFN_glMapBuffer,
    pub glGetBufferPointerv: crate::gl::command::PFN_glGetBufferPointerv,
    pub glGenQueries: crate::gl::command::PFN_glGenQueries,
}
impl EntryFnGL15 {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            glDeleteQueries: unsafe {
                unsafe extern "system" fn __glDeleteQueries(_n: GLsizei, _ids: *mut GLuint) {
                    panic!("Unable to load glDeleteQueries")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDeleteQueries\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDeleteQueries
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetBufferParameteriv: unsafe {
                unsafe extern "system" fn __glGetBufferParameteriv(
                    _target: BufferTargetARB,
                    _pname: BufferPNameARB,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetBufferParameteriv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetBufferParameteriv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetBufferParameteriv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetQueryiv: unsafe {
                unsafe extern "system" fn __glGetQueryiv(
                    _target: QueryTarget,
                    _pname: QueryParameterName,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetQueryiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetQueryiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetQueryiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGenBuffers: unsafe {
                unsafe extern "system" fn __glGenBuffers(_n: GLsizei, _buffers: *mut GLuint) {
                    panic!("Unable to load glGenBuffers")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGenBuffers\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGenBuffers
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glEndQuery: unsafe {
                unsafe extern "system" fn __glEndQuery(_target: QueryTarget) {
                    panic!("Unable to load glEndQuery")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEndQuery\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEndQuery
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetBufferSubData: unsafe {
                unsafe extern "system" fn __glGetBufferSubData(
                    _target: BufferTargetARB,
                    _offset: GLintptr,
                    _size: GLsizeiptr,
                    _data: *mut GLvoid,
                ) {
                    panic!("Unable to load glGetBufferSubData")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetBufferSubData\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetBufferSubData
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBufferData: unsafe {
                unsafe extern "system" fn __glBufferData(
                    _target: BufferTargetARB,
                    _size: GLsizeiptr,
                    _data: *mut GLvoid,
                    _usage: BufferUsageARB,
                ) {
                    panic!("Unable to load glBufferData")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBufferData\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBufferData
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBeginQuery: unsafe {
                unsafe extern "system" fn __glBeginQuery(_target: QueryTarget, _id: GLuint) {
                    panic!("Unable to load glBeginQuery")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBeginQuery\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBeginQuery
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glIsBuffer: unsafe {
                unsafe extern "system" fn __glIsBuffer(_buffer: GLuint) -> GLboolean {
                    panic!("Unable to load glIsBuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIsBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIsBuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDeleteBuffers: unsafe {
                unsafe extern "system" fn __glDeleteBuffers(_n: GLsizei, _buffers: *mut GLuint) {
                    panic!("Unable to load glDeleteBuffers")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDeleteBuffers\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDeleteBuffers
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetQueryObjectiv: unsafe {
                unsafe extern "system" fn __glGetQueryObjectiv(
                    _id: GLuint,
                    _pname: QueryObjectParameterName,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetQueryObjectiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetQueryObjectiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetQueryObjectiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBufferSubData: unsafe {
                unsafe extern "system" fn __glBufferSubData(
                    _target: BufferTargetARB,
                    _offset: GLintptr,
                    _size: GLsizeiptr,
                    _data: *mut GLvoid,
                ) {
                    panic!("Unable to load glBufferSubData")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBufferSubData\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBufferSubData
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUnmapBuffer: unsafe {
                unsafe extern "system" fn __glUnmapBuffer(_target: BufferTargetARB) -> GLboolean {
                    panic!("Unable to load glUnmapBuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUnmapBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUnmapBuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBindBuffer: unsafe {
                unsafe extern "system" fn __glBindBuffer(
                    _target: BufferTargetARB,
                    _buffer: GLuint,
                ) {
                    panic!("Unable to load glBindBuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindBuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetQueryObjectuiv: unsafe {
                unsafe extern "system" fn __glGetQueryObjectuiv(
                    _id: GLuint,
                    _pname: QueryObjectParameterName,
                    _params: *mut GLuint,
                ) {
                    panic!("Unable to load glGetQueryObjectuiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetQueryObjectuiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetQueryObjectuiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glIsQuery: unsafe {
                unsafe extern "system" fn __glIsQuery(_id: GLuint) -> GLboolean {
                    panic!("Unable to load glIsQuery")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIsQuery\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIsQuery
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMapBuffer: unsafe {
                unsafe extern "system" fn __glMapBuffer(
                    _target: BufferTargetARB,
                    _access: BufferAccessARB,
                ) {
                    panic!("Unable to load glMapBuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMapBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMapBuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetBufferPointerv: unsafe {
                unsafe extern "system" fn __glGetBufferPointerv(
                    _target: BufferTargetARB,
                    _pname: BufferPointerNameARB,
                    _params: *mut GLvoid,
                ) {
                    panic!("Unable to load glGetBufferPointerv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetBufferPointerv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetBufferPointerv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGenQueries: unsafe {
                unsafe extern "system" fn __glGenQueries(_n: GLsizei, _ids: *mut GLuint) {
                    panic!("Unable to load glGenQueries")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGenQueries\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGenQueries
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[derive(Clone)]
pub struct EntryFnGL41 {
    pub glProgramUniformMatrix2dv: crate::gl::command::PFN_glProgramUniformMatrix2dv,
    pub glGetProgramBinary: crate::gl::command::PFN_glGetProgramBinary,
    pub glViewportIndexedfv: crate::gl::command::PFN_glViewportIndexedfv,
    pub glShaderBinary: crate::gl::command::PFN_glShaderBinary,
    pub glProgramUniform2dv: crate::gl::command::PFN_glProgramUniform2dv,
    pub glProgramUniform3ui: crate::gl::command::PFN_glProgramUniform3ui,
    pub glProgramUniform4i: crate::gl::command::PFN_glProgramUniform4i,
    pub glProgramBinary: crate::gl::command::PFN_glProgramBinary,
    pub glProgramUniform1ui: crate::gl::command::PFN_glProgramUniform1ui,
    pub glProgramUniform2iv: crate::gl::command::PFN_glProgramUniform2iv,
    pub glViewportArrayv: crate::gl::command::PFN_glViewportArrayv,
    pub glDepthRangef: crate::gl::command::PFN_glDepthRangef,
    pub glProgramUniformMatrix4fv: crate::gl::command::PFN_glProgramUniformMatrix4fv,
    pub glVertexAttribL1d: crate::gl::command::PFN_glVertexAttribL1d,
    pub glProgramUniform4d: crate::gl::command::PFN_glProgramUniform4d,
    pub glGenProgramPipelines: crate::gl::command::PFN_glGenProgramPipelines,
    pub glBindProgramPipeline: crate::gl::command::PFN_glBindProgramPipeline,
    pub glProgramUniform2uiv: crate::gl::command::PFN_glProgramUniform2uiv,
    pub glProgramUniform3f: crate::gl::command::PFN_glProgramUniform3f,
    pub glVertexAttribL1dv: crate::gl::command::PFN_glVertexAttribL1dv,
    pub glIsProgramPipeline: crate::gl::command::PFN_glIsProgramPipeline,
    pub glVertexAttribL4dv: crate::gl::command::PFN_glVertexAttribL4dv,
    pub glProgramUniformMatrix3x2fv: crate::gl::command::PFN_glProgramUniformMatrix3x2fv,
    pub glProgramUniformMatrix4x3fv: crate::gl::command::PFN_glProgramUniformMatrix4x3fv,
    pub glProgramUniform1iv: crate::gl::command::PFN_glProgramUniform1iv,
    pub glProgramUniformMatrix3x4dv: crate::gl::command::PFN_glProgramUniformMatrix3x4dv,
    pub glProgramUniform2f: crate::gl::command::PFN_glProgramUniform2f,
    pub glVertexAttribL4d: crate::gl::command::PFN_glVertexAttribL4d,
    pub glGetProgramPipelineiv: crate::gl::command::PFN_glGetProgramPipelineiv,
    pub glProgramUniformMatrix3x4fv: crate::gl::command::PFN_glProgramUniformMatrix3x4fv,
    pub glProgramUniform3i: crate::gl::command::PFN_glProgramUniform3i,
    pub glScissorIndexedv: crate::gl::command::PFN_glScissorIndexedv,
    pub glProgramParameteri: crate::gl::command::PFN_glProgramParameteri,
    pub glProgramUniform1i: crate::gl::command::PFN_glProgramUniform1i,
    pub glProgramUniform4fv: crate::gl::command::PFN_glProgramUniform4fv,
    pub glProgramUniformMatrix4dv: crate::gl::command::PFN_glProgramUniformMatrix4dv,
    pub glProgramUniformMatrix2x3fv: crate::gl::command::PFN_glProgramUniformMatrix2x3fv,
    pub glProgramUniformMatrix3x2dv: crate::gl::command::PFN_glProgramUniformMatrix3x2dv,
    pub glDepthRangeIndexed: crate::gl::command::PFN_glDepthRangeIndexed,
    pub glProgramUniform1dv: crate::gl::command::PFN_glProgramUniform1dv,
    pub glProgramUniformMatrix2fv: crate::gl::command::PFN_glProgramUniformMatrix2fv,
    pub glProgramUniformMatrix3fv: crate::gl::command::PFN_glProgramUniformMatrix3fv,
    pub glProgramUniform1d: crate::gl::command::PFN_glProgramUniform1d,
    pub glProgramUniform3uiv: crate::gl::command::PFN_glProgramUniform3uiv,
    pub glProgramUniform2d: crate::gl::command::PFN_glProgramUniform2d,
    pub glGetShaderPrecisionFormat: crate::gl::command::PFN_glGetShaderPrecisionFormat,
    pub glProgramUniform4ui: crate::gl::command::PFN_glProgramUniform4ui,
    pub glVertexAttribL2dv: crate::gl::command::PFN_glVertexAttribL2dv,
    pub glScissorArrayv: crate::gl::command::PFN_glScissorArrayv,
    pub glProgramUniform1fv: crate::gl::command::PFN_glProgramUniform1fv,
    pub glProgramUniformMatrix2x3dv: crate::gl::command::PFN_glProgramUniformMatrix2x3dv,
    pub glClearDepthf: crate::gl::command::PFN_glClearDepthf,
    pub glProgramUniform3dv: crate::gl::command::PFN_glProgramUniform3dv,
    pub glGetFloati_v: crate::gl::command::PFN_glGetFloati_v,
    pub glGetDoublei_v: crate::gl::command::PFN_glGetDoublei_v,
    pub glDeleteProgramPipelines: crate::gl::command::PFN_glDeleteProgramPipelines,
    pub glProgramUniformMatrix4x2fv: crate::gl::command::PFN_glProgramUniformMatrix4x2fv,
    pub glGetVertexAttribLdv: crate::gl::command::PFN_glGetVertexAttribLdv,
    pub glProgramUniform1uiv: crate::gl::command::PFN_glProgramUniform1uiv,
    pub glProgramUniform2i: crate::gl::command::PFN_glProgramUniform2i,
    pub glProgramUniform3iv: crate::gl::command::PFN_glProgramUniform3iv,
    pub glProgramUniformMatrix2x4dv: crate::gl::command::PFN_glProgramUniformMatrix2x4dv,
    pub glProgramUniformMatrix4x3dv: crate::gl::command::PFN_glProgramUniformMatrix4x3dv,
    pub glGetProgramPipelineInfoLog: crate::gl::command::PFN_glGetProgramPipelineInfoLog,
    pub glProgramUniform2fv: crate::gl::command::PFN_glProgramUniform2fv,
    pub glCreateShaderProgramv: crate::gl::command::PFN_glCreateShaderProgramv,
    pub glProgramUniform4f: crate::gl::command::PFN_glProgramUniform4f,
    pub glProgramUniform3d: crate::gl::command::PFN_glProgramUniform3d,
    pub glActiveShaderProgram: crate::gl::command::PFN_glActiveShaderProgram,
    pub glProgramUniformMatrix3dv: crate::gl::command::PFN_glProgramUniformMatrix3dv,
    pub glProgramUniformMatrix2x4fv: crate::gl::command::PFN_glProgramUniformMatrix2x4fv,
    pub glVertexAttribLPointer: crate::gl::command::PFN_glVertexAttribLPointer,
    pub glViewportIndexedf: crate::gl::command::PFN_glViewportIndexedf,
    pub glDepthRangeArrayv: crate::gl::command::PFN_glDepthRangeArrayv,
    pub glUseProgramStages: crate::gl::command::PFN_glUseProgramStages,
    pub glProgramUniform4dv: crate::gl::command::PFN_glProgramUniform4dv,
    pub glReleaseShaderCompiler: crate::gl::command::PFN_glReleaseShaderCompiler,
    pub glProgramUniformMatrix4x2dv: crate::gl::command::PFN_glProgramUniformMatrix4x2dv,
    pub glVertexAttribL3d: crate::gl::command::PFN_glVertexAttribL3d,
    pub glProgramUniform3fv: crate::gl::command::PFN_glProgramUniform3fv,
    pub glVertexAttribL3dv: crate::gl::command::PFN_glVertexAttribL3dv,
    pub glScissorIndexed: crate::gl::command::PFN_glScissorIndexed,
    pub glProgramUniform2ui: crate::gl::command::PFN_glProgramUniform2ui,
    pub glVertexAttribL2d: crate::gl::command::PFN_glVertexAttribL2d,
    pub glProgramUniform4iv: crate::gl::command::PFN_glProgramUniform4iv,
    pub glValidateProgramPipeline: crate::gl::command::PFN_glValidateProgramPipeline,
    pub glProgramUniform4uiv: crate::gl::command::PFN_glProgramUniform4uiv,
    pub glProgramUniform1f: crate::gl::command::PFN_glProgramUniform1f,
}
impl EntryFnGL41 {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            glProgramUniformMatrix2dv: unsafe {
                unsafe extern "system" fn __glProgramUniformMatrix2dv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLdouble,
                ) {
                    panic!("Unable to load glProgramUniformMatrix2dv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniformMatrix2dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniformMatrix2dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetProgramBinary: unsafe {
                unsafe extern "system" fn __glGetProgramBinary(
                    _program: GLuint,
                    _bufSize: GLsizei,
                    _length: *mut GLsizei,
                    _binaryFormat: *mut GLenum,
                    _binary: *mut GLvoid,
                ) {
                    panic!("Unable to load glGetProgramBinary")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetProgramBinary\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetProgramBinary
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glViewportIndexedfv: unsafe {
                unsafe extern "system" fn __glViewportIndexedfv(_index: GLuint, _v: *mut GLfloat) {
                    panic!("Unable to load glViewportIndexedfv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glViewportIndexedfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glViewportIndexedfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glShaderBinary: unsafe {
                unsafe extern "system" fn __glShaderBinary(
                    _count: GLsizei,
                    _shaders: *mut GLuint,
                    _binaryFormat: ShaderBinaryFormat,
                    _binary: *mut GLvoid,
                    _length: GLsizei,
                ) {
                    panic!("Unable to load glShaderBinary")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glShaderBinary\0");
                let val = _f(cname);
                if val.is_null() {
                    __glShaderBinary
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform2dv: unsafe {
                unsafe extern "system" fn __glProgramUniform2dv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLdouble,
                ) {
                    panic!("Unable to load glProgramUniform2dv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform2dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform2dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform3ui: unsafe {
                unsafe extern "system" fn __glProgramUniform3ui(
                    _program: GLuint,
                    _location: GLint,
                    _v0: GLuint,
                    _v1: GLuint,
                    _v2: GLuint,
                ) {
                    panic!("Unable to load glProgramUniform3ui")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform3ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform3ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform4i: unsafe {
                unsafe extern "system" fn __glProgramUniform4i(
                    _program: GLuint,
                    _location: GLint,
                    _v0: GLint,
                    _v1: GLint,
                    _v2: GLint,
                    _v3: GLint,
                ) {
                    panic!("Unable to load glProgramUniform4i")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform4i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform4i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramBinary: unsafe {
                unsafe extern "system" fn __glProgramBinary(
                    _program: GLuint,
                    _binaryFormat: GLenum,
                    _binary: *mut GLvoid,
                    _length: GLsizei,
                ) {
                    panic!("Unable to load glProgramBinary")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramBinary\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramBinary
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform1ui: unsafe {
                unsafe extern "system" fn __glProgramUniform1ui(
                    _program: GLuint,
                    _location: GLint,
                    _v0: GLuint,
                ) {
                    panic!("Unable to load glProgramUniform1ui")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform1ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform1ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform2iv: unsafe {
                unsafe extern "system" fn __glProgramUniform2iv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLint,
                ) {
                    panic!("Unable to load glProgramUniform2iv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform2iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform2iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glViewportArrayv: unsafe {
                unsafe extern "system" fn __glViewportArrayv(
                    _first: GLuint,
                    _count: GLsizei,
                    _v: *mut GLfloat,
                ) {
                    panic!("Unable to load glViewportArrayv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glViewportArrayv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glViewportArrayv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDepthRangef: unsafe {
                unsafe extern "system" fn __glDepthRangef(_n: GLfloat, _f: GLfloat) {
                    panic!("Unable to load glDepthRangef")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDepthRangef\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDepthRangef
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniformMatrix4fv: unsafe {
                unsafe extern "system" fn __glProgramUniformMatrix4fv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLfloat,
                ) {
                    panic!("Unable to load glProgramUniformMatrix4fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniformMatrix4fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniformMatrix4fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribL1d: unsafe {
                unsafe extern "system" fn __glVertexAttribL1d(_index: GLuint, _x: GLdouble) {
                    panic!("Unable to load glVertexAttribL1d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribL1d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribL1d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform4d: unsafe {
                unsafe extern "system" fn __glProgramUniform4d(
                    _program: GLuint,
                    _location: GLint,
                    _v0: GLdouble,
                    _v1: GLdouble,
                    _v2: GLdouble,
                    _v3: GLdouble,
                ) {
                    panic!("Unable to load glProgramUniform4d")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform4d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform4d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGenProgramPipelines: unsafe {
                unsafe extern "system" fn __glGenProgramPipelines(
                    _n: GLsizei,
                    _pipelines: *mut GLuint,
                ) {
                    panic!("Unable to load glGenProgramPipelines")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGenProgramPipelines\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGenProgramPipelines
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBindProgramPipeline: unsafe {
                unsafe extern "system" fn __glBindProgramPipeline(_pipeline: GLuint) {
                    panic!("Unable to load glBindProgramPipeline")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindProgramPipeline\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindProgramPipeline
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform2uiv: unsafe {
                unsafe extern "system" fn __glProgramUniform2uiv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLuint,
                ) {
                    panic!("Unable to load glProgramUniform2uiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform2uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform2uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform3f: unsafe {
                unsafe extern "system" fn __glProgramUniform3f(
                    _program: GLuint,
                    _location: GLint,
                    _v0: GLfloat,
                    _v1: GLfloat,
                    _v2: GLfloat,
                ) {
                    panic!("Unable to load glProgramUniform3f")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform3f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform3f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribL1dv: unsafe {
                unsafe extern "system" fn __glVertexAttribL1dv(_index: GLuint, _v: *mut GLdouble) {
                    panic!("Unable to load glVertexAttribL1dv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribL1dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribL1dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glIsProgramPipeline: unsafe {
                unsafe extern "system" fn __glIsProgramPipeline(_pipeline: GLuint) -> GLboolean {
                    panic!("Unable to load glIsProgramPipeline")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIsProgramPipeline\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIsProgramPipeline
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribL4dv: unsafe {
                unsafe extern "system" fn __glVertexAttribL4dv(_index: GLuint, _v: *mut GLdouble) {
                    panic!("Unable to load glVertexAttribL4dv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribL4dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribL4dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniformMatrix3x2fv: unsafe {
                unsafe extern "system" fn __glProgramUniformMatrix3x2fv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLfloat,
                ) {
                    panic!("Unable to load glProgramUniformMatrix3x2fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glProgramUniformMatrix3x2fv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniformMatrix3x2fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniformMatrix4x3fv: unsafe {
                unsafe extern "system" fn __glProgramUniformMatrix4x3fv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLfloat,
                ) {
                    panic!("Unable to load glProgramUniformMatrix4x3fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glProgramUniformMatrix4x3fv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniformMatrix4x3fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform1iv: unsafe {
                unsafe extern "system" fn __glProgramUniform1iv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLint,
                ) {
                    panic!("Unable to load glProgramUniform1iv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform1iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform1iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniformMatrix3x4dv: unsafe {
                unsafe extern "system" fn __glProgramUniformMatrix3x4dv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLdouble,
                ) {
                    panic!("Unable to load glProgramUniformMatrix3x4dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glProgramUniformMatrix3x4dv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniformMatrix3x4dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform2f: unsafe {
                unsafe extern "system" fn __glProgramUniform2f(
                    _program: GLuint,
                    _location: GLint,
                    _v0: GLfloat,
                    _v1: GLfloat,
                ) {
                    panic!("Unable to load glProgramUniform2f")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform2f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform2f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribL4d: unsafe {
                unsafe extern "system" fn __glVertexAttribL4d(
                    _index: GLuint,
                    _x: GLdouble,
                    _y: GLdouble,
                    _z: GLdouble,
                    _w: GLdouble,
                ) {
                    panic!("Unable to load glVertexAttribL4d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribL4d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribL4d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetProgramPipelineiv: unsafe {
                unsafe extern "system" fn __glGetProgramPipelineiv(
                    _pipeline: GLuint,
                    _pname: PipelineParameterName,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetProgramPipelineiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetProgramPipelineiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetProgramPipelineiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniformMatrix3x4fv: unsafe {
                unsafe extern "system" fn __glProgramUniformMatrix3x4fv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLfloat,
                ) {
                    panic!("Unable to load glProgramUniformMatrix3x4fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glProgramUniformMatrix3x4fv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniformMatrix3x4fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform3i: unsafe {
                unsafe extern "system" fn __glProgramUniform3i(
                    _program: GLuint,
                    _location: GLint,
                    _v0: GLint,
                    _v1: GLint,
                    _v2: GLint,
                ) {
                    panic!("Unable to load glProgramUniform3i")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform3i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform3i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glScissorIndexedv: unsafe {
                unsafe extern "system" fn __glScissorIndexedv(_index: GLuint, _v: *mut GLint) {
                    panic!("Unable to load glScissorIndexedv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glScissorIndexedv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glScissorIndexedv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramParameteri: unsafe {
                unsafe extern "system" fn __glProgramParameteri(
                    _program: GLuint,
                    _pname: ProgramParameterPName,
                    _value: GLint,
                ) {
                    panic!("Unable to load glProgramParameteri")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramParameteri\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramParameteri
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform1i: unsafe {
                unsafe extern "system" fn __glProgramUniform1i(
                    _program: GLuint,
                    _location: GLint,
                    _v0: GLint,
                ) {
                    panic!("Unable to load glProgramUniform1i")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform1i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform1i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform4fv: unsafe {
                unsafe extern "system" fn __glProgramUniform4fv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLfloat,
                ) {
                    panic!("Unable to load glProgramUniform4fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform4fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform4fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniformMatrix4dv: unsafe {
                unsafe extern "system" fn __glProgramUniformMatrix4dv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLdouble,
                ) {
                    panic!("Unable to load glProgramUniformMatrix4dv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniformMatrix4dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniformMatrix4dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniformMatrix2x3fv: unsafe {
                unsafe extern "system" fn __glProgramUniformMatrix2x3fv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLfloat,
                ) {
                    panic!("Unable to load glProgramUniformMatrix2x3fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glProgramUniformMatrix2x3fv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniformMatrix2x3fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniformMatrix3x2dv: unsafe {
                unsafe extern "system" fn __glProgramUniformMatrix3x2dv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLdouble,
                ) {
                    panic!("Unable to load glProgramUniformMatrix3x2dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glProgramUniformMatrix3x2dv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniformMatrix3x2dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDepthRangeIndexed: unsafe {
                unsafe extern "system" fn __glDepthRangeIndexed(
                    _index: GLuint,
                    _n: GLdouble,
                    _f: GLdouble,
                ) {
                    panic!("Unable to load glDepthRangeIndexed")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDepthRangeIndexed\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDepthRangeIndexed
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform1dv: unsafe {
                unsafe extern "system" fn __glProgramUniform1dv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLdouble,
                ) {
                    panic!("Unable to load glProgramUniform1dv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform1dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform1dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniformMatrix2fv: unsafe {
                unsafe extern "system" fn __glProgramUniformMatrix2fv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLfloat,
                ) {
                    panic!("Unable to load glProgramUniformMatrix2fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniformMatrix2fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniformMatrix2fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniformMatrix3fv: unsafe {
                unsafe extern "system" fn __glProgramUniformMatrix3fv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLfloat,
                ) {
                    panic!("Unable to load glProgramUniformMatrix3fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniformMatrix3fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniformMatrix3fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform1d: unsafe {
                unsafe extern "system" fn __glProgramUniform1d(
                    _program: GLuint,
                    _location: GLint,
                    _v0: GLdouble,
                ) {
                    panic!("Unable to load glProgramUniform1d")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform1d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform1d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform3uiv: unsafe {
                unsafe extern "system" fn __glProgramUniform3uiv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLuint,
                ) {
                    panic!("Unable to load glProgramUniform3uiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform3uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform3uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform2d: unsafe {
                unsafe extern "system" fn __glProgramUniform2d(
                    _program: GLuint,
                    _location: GLint,
                    _v0: GLdouble,
                    _v1: GLdouble,
                ) {
                    panic!("Unable to load glProgramUniform2d")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform2d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform2d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetShaderPrecisionFormat: unsafe {
                unsafe extern "system" fn __glGetShaderPrecisionFormat(
                    _shadertype: ShaderType,
                    _precisiontype: PrecisionType,
                    _range: *mut GLint,
                    _precision: *mut GLint,
                ) {
                    panic!("Unable to load glGetShaderPrecisionFormat")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetShaderPrecisionFormat\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetShaderPrecisionFormat
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform4ui: unsafe {
                unsafe extern "system" fn __glProgramUniform4ui(
                    _program: GLuint,
                    _location: GLint,
                    _v0: GLuint,
                    _v1: GLuint,
                    _v2: GLuint,
                    _v3: GLuint,
                ) {
                    panic!("Unable to load glProgramUniform4ui")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform4ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform4ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribL2dv: unsafe {
                unsafe extern "system" fn __glVertexAttribL2dv(_index: GLuint, _v: *mut GLdouble) {
                    panic!("Unable to load glVertexAttribL2dv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribL2dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribL2dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glScissorArrayv: unsafe {
                unsafe extern "system" fn __glScissorArrayv(
                    _first: GLuint,
                    _count: GLsizei,
                    _v: *mut GLint,
                ) {
                    panic!("Unable to load glScissorArrayv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glScissorArrayv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glScissorArrayv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform1fv: unsafe {
                unsafe extern "system" fn __glProgramUniform1fv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLfloat,
                ) {
                    panic!("Unable to load glProgramUniform1fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform1fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform1fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniformMatrix2x3dv: unsafe {
                unsafe extern "system" fn __glProgramUniformMatrix2x3dv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLdouble,
                ) {
                    panic!("Unable to load glProgramUniformMatrix2x3dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glProgramUniformMatrix2x3dv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniformMatrix2x3dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glClearDepthf: unsafe {
                unsafe extern "system" fn __glClearDepthf(_d: GLfloat) {
                    panic!("Unable to load glClearDepthf")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClearDepthf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClearDepthf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform3dv: unsafe {
                unsafe extern "system" fn __glProgramUniform3dv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLdouble,
                ) {
                    panic!("Unable to load glProgramUniform3dv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform3dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform3dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetFloati_v: unsafe {
                unsafe extern "system" fn __glGetFloati_v(
                    _target: GetPName,
                    _index: GLuint,
                    _data: *mut GLfloat,
                ) {
                    panic!("Unable to load glGetFloati_v")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetFloati_v\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetFloati_v
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetDoublei_v: unsafe {
                unsafe extern "system" fn __glGetDoublei_v(
                    _target: GetPName,
                    _index: GLuint,
                    _data: *mut GLdouble,
                ) {
                    panic!("Unable to load glGetDoublei_v")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetDoublei_v\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetDoublei_v
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDeleteProgramPipelines: unsafe {
                unsafe extern "system" fn __glDeleteProgramPipelines(
                    _n: GLsizei,
                    _pipelines: *mut GLuint,
                ) {
                    panic!("Unable to load glDeleteProgramPipelines")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDeleteProgramPipelines\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDeleteProgramPipelines
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniformMatrix4x2fv: unsafe {
                unsafe extern "system" fn __glProgramUniformMatrix4x2fv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLfloat,
                ) {
                    panic!("Unable to load glProgramUniformMatrix4x2fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glProgramUniformMatrix4x2fv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniformMatrix4x2fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetVertexAttribLdv: unsafe {
                unsafe extern "system" fn __glGetVertexAttribLdv(
                    _index: GLuint,
                    _pname: VertexAttribEnum,
                    _params: *mut GLdouble,
                ) {
                    panic!("Unable to load glGetVertexAttribLdv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetVertexAttribLdv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetVertexAttribLdv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform1uiv: unsafe {
                unsafe extern "system" fn __glProgramUniform1uiv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLuint,
                ) {
                    panic!("Unable to load glProgramUniform1uiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform1uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform1uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform2i: unsafe {
                unsafe extern "system" fn __glProgramUniform2i(
                    _program: GLuint,
                    _location: GLint,
                    _v0: GLint,
                    _v1: GLint,
                ) {
                    panic!("Unable to load glProgramUniform2i")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform2i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform2i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform3iv: unsafe {
                unsafe extern "system" fn __glProgramUniform3iv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLint,
                ) {
                    panic!("Unable to load glProgramUniform3iv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform3iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform3iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniformMatrix2x4dv: unsafe {
                unsafe extern "system" fn __glProgramUniformMatrix2x4dv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLdouble,
                ) {
                    panic!("Unable to load glProgramUniformMatrix2x4dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glProgramUniformMatrix2x4dv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniformMatrix2x4dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniformMatrix4x3dv: unsafe {
                unsafe extern "system" fn __glProgramUniformMatrix4x3dv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLdouble,
                ) {
                    panic!("Unable to load glProgramUniformMatrix4x3dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glProgramUniformMatrix4x3dv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniformMatrix4x3dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetProgramPipelineInfoLog: unsafe {
                unsafe extern "system" fn __glGetProgramPipelineInfoLog(
                    _pipeline: GLuint,
                    _bufSize: GLsizei,
                    _length: *mut GLsizei,
                    _infoLog: *mut GLchar,
                ) {
                    panic!("Unable to load glGetProgramPipelineInfoLog")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetProgramPipelineInfoLog\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetProgramPipelineInfoLog
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform2fv: unsafe {
                unsafe extern "system" fn __glProgramUniform2fv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLfloat,
                ) {
                    panic!("Unable to load glProgramUniform2fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform2fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform2fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCreateShaderProgramv: unsafe {
                unsafe extern "system" fn __glCreateShaderProgramv(
                    _type: ShaderType,
                    _count: GLsizei,
                    _strings: *mut GLchar,
                ) -> GLuint {
                    panic!("Unable to load glCreateShaderProgramv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCreateShaderProgramv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCreateShaderProgramv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform4f: unsafe {
                unsafe extern "system" fn __glProgramUniform4f(
                    _program: GLuint,
                    _location: GLint,
                    _v0: GLfloat,
                    _v1: GLfloat,
                    _v2: GLfloat,
                    _v3: GLfloat,
                ) {
                    panic!("Unable to load glProgramUniform4f")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform4f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform4f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform3d: unsafe {
                unsafe extern "system" fn __glProgramUniform3d(
                    _program: GLuint,
                    _location: GLint,
                    _v0: GLdouble,
                    _v1: GLdouble,
                    _v2: GLdouble,
                ) {
                    panic!("Unable to load glProgramUniform3d")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform3d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform3d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glActiveShaderProgram: unsafe {
                unsafe extern "system" fn __glActiveShaderProgram(
                    _pipeline: GLuint,
                    _program: GLuint,
                ) {
                    panic!("Unable to load glActiveShaderProgram")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glActiveShaderProgram\0");
                let val = _f(cname);
                if val.is_null() {
                    __glActiveShaderProgram
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniformMatrix3dv: unsafe {
                unsafe extern "system" fn __glProgramUniformMatrix3dv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLdouble,
                ) {
                    panic!("Unable to load glProgramUniformMatrix3dv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniformMatrix3dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniformMatrix3dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniformMatrix2x4fv: unsafe {
                unsafe extern "system" fn __glProgramUniformMatrix2x4fv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLfloat,
                ) {
                    panic!("Unable to load glProgramUniformMatrix2x4fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glProgramUniformMatrix2x4fv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniformMatrix2x4fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribLPointer: unsafe {
                unsafe extern "system" fn __glVertexAttribLPointer(
                    _index: GLuint,
                    _size: GLint,
                    _type: VertexAttribLType,
                    _stride: GLsizei,
                    _pointer: *mut GLvoid,
                ) {
                    panic!("Unable to load glVertexAttribLPointer")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribLPointer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribLPointer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glViewportIndexedf: unsafe {
                unsafe extern "system" fn __glViewportIndexedf(
                    _index: GLuint,
                    _x: GLfloat,
                    _y: GLfloat,
                    _w: GLfloat,
                    _h: GLfloat,
                ) {
                    panic!("Unable to load glViewportIndexedf")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glViewportIndexedf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glViewportIndexedf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDepthRangeArrayv: unsafe {
                unsafe extern "system" fn __glDepthRangeArrayv(
                    _first: GLuint,
                    _count: GLsizei,
                    _v: *mut GLdouble,
                ) {
                    panic!("Unable to load glDepthRangeArrayv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDepthRangeArrayv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDepthRangeArrayv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUseProgramStages: unsafe {
                unsafe extern "system" fn __glUseProgramStages(
                    _pipeline: GLuint,
                    _stages: UseProgramStageMask,
                    _program: GLuint,
                ) {
                    panic!("Unable to load glUseProgramStages")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUseProgramStages\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUseProgramStages
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform4dv: unsafe {
                unsafe extern "system" fn __glProgramUniform4dv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLdouble,
                ) {
                    panic!("Unable to load glProgramUniform4dv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform4dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform4dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glReleaseShaderCompiler: unsafe {
                unsafe extern "system" fn __glReleaseShaderCompiler() {
                    panic!("Unable to load glReleaseShaderCompiler")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glReleaseShaderCompiler\0");
                let val = _f(cname);
                if val.is_null() {
                    __glReleaseShaderCompiler
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniformMatrix4x2dv: unsafe {
                unsafe extern "system" fn __glProgramUniformMatrix4x2dv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLdouble,
                ) {
                    panic!("Unable to load glProgramUniformMatrix4x2dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glProgramUniformMatrix4x2dv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniformMatrix4x2dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribL3d: unsafe {
                unsafe extern "system" fn __glVertexAttribL3d(
                    _index: GLuint,
                    _x: GLdouble,
                    _y: GLdouble,
                    _z: GLdouble,
                ) {
                    panic!("Unable to load glVertexAttribL3d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribL3d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribL3d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform3fv: unsafe {
                unsafe extern "system" fn __glProgramUniform3fv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLfloat,
                ) {
                    panic!("Unable to load glProgramUniform3fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform3fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform3fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribL3dv: unsafe {
                unsafe extern "system" fn __glVertexAttribL3dv(_index: GLuint, _v: *mut GLdouble) {
                    panic!("Unable to load glVertexAttribL3dv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribL3dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribL3dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glScissorIndexed: unsafe {
                unsafe extern "system" fn __glScissorIndexed(
                    _index: GLuint,
                    _left: GLint,
                    _bottom: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                ) {
                    panic!("Unable to load glScissorIndexed")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glScissorIndexed\0");
                let val = _f(cname);
                if val.is_null() {
                    __glScissorIndexed
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform2ui: unsafe {
                unsafe extern "system" fn __glProgramUniform2ui(
                    _program: GLuint,
                    _location: GLint,
                    _v0: GLuint,
                    _v1: GLuint,
                ) {
                    panic!("Unable to load glProgramUniform2ui")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform2ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform2ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribL2d: unsafe {
                unsafe extern "system" fn __glVertexAttribL2d(
                    _index: GLuint,
                    _x: GLdouble,
                    _y: GLdouble,
                ) {
                    panic!("Unable to load glVertexAttribL2d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribL2d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribL2d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform4iv: unsafe {
                unsafe extern "system" fn __glProgramUniform4iv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLint,
                ) {
                    panic!("Unable to load glProgramUniform4iv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform4iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform4iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glValidateProgramPipeline: unsafe {
                unsafe extern "system" fn __glValidateProgramPipeline(_pipeline: GLuint) {
                    panic!("Unable to load glValidateProgramPipeline")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glValidateProgramPipeline\0");
                let val = _f(cname);
                if val.is_null() {
                    __glValidateProgramPipeline
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform4uiv: unsafe {
                unsafe extern "system" fn __glProgramUniform4uiv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLuint,
                ) {
                    panic!("Unable to load glProgramUniform4uiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform4uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform4uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProgramUniform1f: unsafe {
                unsafe extern "system" fn __glProgramUniform1f(
                    _program: GLuint,
                    _location: GLint,
                    _v0: GLfloat,
                ) {
                    panic!("Unable to load glProgramUniform1f")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform1f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform1f
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[derive(Clone)]
pub struct EntryFnGL44 {
    pub glBindSamplers: crate::gl::command::PFN_glBindSamplers,
    pub glBufferStorage: crate::gl::command::PFN_glBufferStorage,
    pub glBindBuffersBase: crate::gl::command::PFN_glBindBuffersBase,
    pub glBindTextures: crate::gl::command::PFN_glBindTextures,
    pub glClearTexImage: crate::gl::command::PFN_glClearTexImage,
    pub glClearTexSubImage: crate::gl::command::PFN_glClearTexSubImage,
    pub glBindImageTextures: crate::gl::command::PFN_glBindImageTextures,
    pub glBindVertexBuffers: crate::gl::command::PFN_glBindVertexBuffers,
    pub glBindBuffersRange: crate::gl::command::PFN_glBindBuffersRange,
}
impl EntryFnGL44 {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            glBindSamplers: unsafe {
                unsafe extern "system" fn __glBindSamplers(
                    _first: GLuint,
                    _count: GLsizei,
                    _samplers: *mut GLuint,
                ) {
                    panic!("Unable to load glBindSamplers")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindSamplers\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindSamplers
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBufferStorage: unsafe {
                unsafe extern "system" fn __glBufferStorage(
                    _target: BufferStorageTarget,
                    _size: GLsizeiptr,
                    _data: *mut GLvoid,
                    _flags: BufferStorageMask,
                ) {
                    panic!("Unable to load glBufferStorage")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBufferStorage\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBufferStorage
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBindBuffersBase: unsafe {
                unsafe extern "system" fn __glBindBuffersBase(
                    _target: BufferTargetARB,
                    _first: GLuint,
                    _count: GLsizei,
                    _buffers: *mut GLuint,
                ) {
                    panic!("Unable to load glBindBuffersBase")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindBuffersBase\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindBuffersBase
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBindTextures: unsafe {
                unsafe extern "system" fn __glBindTextures(
                    _first: GLuint,
                    _count: GLsizei,
                    _textures: *mut GLuint,
                ) {
                    panic!("Unable to load glBindTextures")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindTextures\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindTextures
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glClearTexImage: unsafe {
                unsafe extern "system" fn __glClearTexImage(
                    _texture: GLuint,
                    _level: GLint,
                    _format: PixelFormat,
                    _type: PixelType,
                    _data: *mut GLvoid,
                ) {
                    panic!("Unable to load glClearTexImage")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClearTexImage\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClearTexImage
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glClearTexSubImage: unsafe {
                unsafe extern "system" fn __glClearTexSubImage(
                    _texture: GLuint,
                    _level: GLint,
                    _xoffset: GLint,
                    _yoffset: GLint,
                    _zoffset: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                    _depth: GLsizei,
                    _format: PixelFormat,
                    _type: PixelType,
                    _data: *mut GLvoid,
                ) {
                    panic!("Unable to load glClearTexSubImage")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClearTexSubImage\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClearTexSubImage
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBindImageTextures: unsafe {
                unsafe extern "system" fn __glBindImageTextures(
                    _first: GLuint,
                    _count: GLsizei,
                    _textures: *mut GLuint,
                ) {
                    panic!("Unable to load glBindImageTextures")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindImageTextures\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindImageTextures
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBindVertexBuffers: unsafe {
                unsafe extern "system" fn __glBindVertexBuffers(
                    _first: GLuint,
                    _count: GLsizei,
                    _buffers: *mut GLuint,
                    _offsets: *mut GLintptr,
                    _strides: *mut GLsizei,
                ) {
                    panic!("Unable to load glBindVertexBuffers")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindVertexBuffers\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindVertexBuffers
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBindBuffersRange: unsafe {
                unsafe extern "system" fn __glBindBuffersRange(
                    _target: BufferTargetARB,
                    _first: GLuint,
                    _count: GLsizei,
                    _buffers: *mut GLuint,
                    _offsets: *mut GLintptr,
                    _sizes: *mut GLsizeiptr,
                ) {
                    panic!("Unable to load glBindBuffersRange")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindBuffersRange\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindBuffersRange
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[derive(Clone)]
pub struct EntryFnGL42 {
    pub glDrawElementsInstancedBaseVertexBaseInstance:
        crate::gl::command::PFN_glDrawElementsInstancedBaseVertexBaseInstance,
    pub glDrawArraysInstancedBaseInstance:
        crate::gl::command::PFN_glDrawArraysInstancedBaseInstance,
    pub glGetActiveAtomicCounterBufferiv: crate::gl::command::PFN_glGetActiveAtomicCounterBufferiv,
    pub glMemoryBarrier: crate::gl::command::PFN_glMemoryBarrier,
    pub glTexStorage1D: crate::gl::command::PFN_glTexStorage1D,
    pub glTexStorage2D: crate::gl::command::PFN_glTexStorage2D,
    pub glTexStorage3D: crate::gl::command::PFN_glTexStorage3D,
    pub glDrawTransformFeedbackStreamInstanced:
        crate::gl::command::PFN_glDrawTransformFeedbackStreamInstanced,
    pub glDrawElementsInstancedBaseInstance:
        crate::gl::command::PFN_glDrawElementsInstancedBaseInstance,
    pub glBindImageTexture: crate::gl::command::PFN_glBindImageTexture,
    pub glGetInternalformativ: crate::gl::command::PFN_glGetInternalformativ,
    pub glDrawTransformFeedbackInstanced: crate::gl::command::PFN_glDrawTransformFeedbackInstanced,
}
impl EntryFnGL42 {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            glDrawElementsInstancedBaseVertexBaseInstance: unsafe {
                unsafe extern "system" fn __glDrawElementsInstancedBaseVertexBaseInstance(
                    _mode: PrimitiveType,
                    _count: GLsizei,
                    _type: DrawElementsType,
                    _indices: *mut GLvoid,
                    _instancecount: GLsizei,
                    _basevertex: GLint,
                    _baseinstance: GLuint,
                ) {
                    panic!("Unable to load glDrawElementsInstancedBaseVertexBaseInstance")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glDrawElementsInstancedBaseVertexBaseInstance\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glDrawElementsInstancedBaseVertexBaseInstance
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDrawArraysInstancedBaseInstance: unsafe {
                unsafe extern "system" fn __glDrawArraysInstancedBaseInstance(
                    _mode: PrimitiveType,
                    _first: GLint,
                    _count: GLsizei,
                    _instancecount: GLsizei,
                    _baseinstance: GLuint,
                ) {
                    panic!("Unable to load glDrawArraysInstancedBaseInstance")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glDrawArraysInstancedBaseInstance\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glDrawArraysInstancedBaseInstance
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetActiveAtomicCounterBufferiv: unsafe {
                unsafe extern "system" fn __glGetActiveAtomicCounterBufferiv(
                    _program: GLuint,
                    _bufferIndex: GLuint,
                    _pname: AtomicCounterBufferPName,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetActiveAtomicCounterBufferiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetActiveAtomicCounterBufferiv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetActiveAtomicCounterBufferiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMemoryBarrier: unsafe {
                unsafe extern "system" fn __glMemoryBarrier(_barriers: MemoryBarrierMask) {
                    panic!("Unable to load glMemoryBarrier")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMemoryBarrier\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMemoryBarrier
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexStorage1D: unsafe {
                unsafe extern "system" fn __glTexStorage1D(
                    _target: TextureTarget,
                    _levels: GLsizei,
                    _internalformat: SizedInternalFormat,
                    _width: GLsizei,
                ) {
                    panic!("Unable to load glTexStorage1D")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexStorage1D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexStorage1D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexStorage2D: unsafe {
                unsafe extern "system" fn __glTexStorage2D(
                    _target: TextureTarget,
                    _levels: GLsizei,
                    _internalformat: SizedInternalFormat,
                    _width: GLsizei,
                    _height: GLsizei,
                ) {
                    panic!("Unable to load glTexStorage2D")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexStorage2D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexStorage2D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexStorage3D: unsafe {
                unsafe extern "system" fn __glTexStorage3D(
                    _target: TextureTarget,
                    _levels: GLsizei,
                    _internalformat: SizedInternalFormat,
                    _width: GLsizei,
                    _height: GLsizei,
                    _depth: GLsizei,
                ) {
                    panic!("Unable to load glTexStorage3D")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexStorage3D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexStorage3D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDrawTransformFeedbackStreamInstanced: unsafe {
                unsafe extern "system" fn __glDrawTransformFeedbackStreamInstanced(
                    _mode: PrimitiveType,
                    _id: GLuint,
                    _stream: GLuint,
                    _instancecount: GLsizei,
                ) {
                    panic!("Unable to load glDrawTransformFeedbackStreamInstanced")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glDrawTransformFeedbackStreamInstanced\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glDrawTransformFeedbackStreamInstanced
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDrawElementsInstancedBaseInstance: unsafe {
                unsafe extern "system" fn __glDrawElementsInstancedBaseInstance(
                    _mode: PrimitiveType,
                    _count: GLsizei,
                    _type: PrimitiveType,
                    _indices: *mut GLvoid,
                    _instancecount: GLsizei,
                    _baseinstance: GLuint,
                ) {
                    panic!("Unable to load glDrawElementsInstancedBaseInstance")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glDrawElementsInstancedBaseInstance\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glDrawElementsInstancedBaseInstance
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBindImageTexture: unsafe {
                unsafe extern "system" fn __glBindImageTexture(
                    _unit: GLuint,
                    _texture: GLuint,
                    _level: GLint,
                    _layered: Boolean,
                    _layer: GLint,
                    _access: BufferAccessARB,
                    _format: InternalFormat,
                ) {
                    panic!("Unable to load glBindImageTexture")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindImageTexture\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindImageTexture
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetInternalformativ: unsafe {
                unsafe extern "system" fn __glGetInternalformativ(
                    _target: TextureTarget,
                    _internalformat: InternalFormat,
                    _pname: InternalFormatPName,
                    _count: GLsizei,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetInternalformativ")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetInternalformativ\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetInternalformativ
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDrawTransformFeedbackInstanced: unsafe {
                unsafe extern "system" fn __glDrawTransformFeedbackInstanced(
                    _mode: PrimitiveType,
                    _id: GLuint,
                    _instancecount: GLsizei,
                ) {
                    panic!("Unable to load glDrawTransformFeedbackInstanced")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glDrawTransformFeedbackInstanced\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glDrawTransformFeedbackInstanced
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[derive(Clone)]
pub struct EntryFnGL43 {
    pub glGetDebugMessageLog: crate::gl::command::PFN_glGetDebugMessageLog,
    pub glVertexBindingDivisor: crate::gl::command::PFN_glVertexBindingDivisor,
    pub glTextureView: crate::gl::command::PFN_glTextureView,
    pub glPopDebugGroup: crate::gl::command::PFN_glPopDebugGroup,
    pub glDispatchCompute: crate::gl::command::PFN_glDispatchCompute,
    pub glInvalidateSubFramebuffer: crate::gl::command::PFN_glInvalidateSubFramebuffer,
    pub glGetProgramResourceIndex: crate::gl::command::PFN_glGetProgramResourceIndex,
    pub glDebugMessageInsert: crate::gl::command::PFN_glDebugMessageInsert,
    pub glGetObjectPtrLabel: crate::gl::command::PFN_glGetObjectPtrLabel,
    pub glInvalidateTexSubImage: crate::gl::command::PFN_glInvalidateTexSubImage,
    pub glMultiDrawElementsIndirect: crate::gl::command::PFN_glMultiDrawElementsIndirect,
    pub glVertexAttribFormat: crate::gl::command::PFN_glVertexAttribFormat,
    pub glGetObjectLabel: crate::gl::command::PFN_glGetObjectLabel,
    pub glPushDebugGroup: crate::gl::command::PFN_glPushDebugGroup,
    pub glVertexAttribBinding: crate::gl::command::PFN_glVertexAttribBinding,
    pub glTexStorage3DMultisample: crate::gl::command::PFN_glTexStorage3DMultisample,
    pub glDebugMessageControl: crate::gl::command::PFN_glDebugMessageControl,
    pub glInvalidateBufferData: crate::gl::command::PFN_glInvalidateBufferData,
    pub glDebugMessageCallback: crate::gl::command::PFN_glDebugMessageCallback,
    pub glInvalidateTexImage: crate::gl::command::PFN_glInvalidateTexImage,
    pub glBindVertexBuffer: crate::gl::command::PFN_glBindVertexBuffer,
    pub glGetProgramResourceLocationIndex:
        crate::gl::command::PFN_glGetProgramResourceLocationIndex,
    pub glGetFramebufferParameteriv: crate::gl::command::PFN_glGetFramebufferParameteriv,
    pub glInvalidateFramebuffer: crate::gl::command::PFN_glInvalidateFramebuffer,
    pub glObjectPtrLabel: crate::gl::command::PFN_glObjectPtrLabel,
    pub glObjectLabel: crate::gl::command::PFN_glObjectLabel,
    pub glTexBufferRange: crate::gl::command::PFN_glTexBufferRange,
    pub glGetProgramResourceName: crate::gl::command::PFN_glGetProgramResourceName,
    pub glVertexAttribLFormat: crate::gl::command::PFN_glVertexAttribLFormat,
    pub glGetProgramResourceLocation: crate::gl::command::PFN_glGetProgramResourceLocation,
    pub glClearBufferData: crate::gl::command::PFN_glClearBufferData,
    pub glCopyImageSubData: crate::gl::command::PFN_glCopyImageSubData,
    pub glClearBufferSubData: crate::gl::command::PFN_glClearBufferSubData,
    pub glGetProgramResourceiv: crate::gl::command::PFN_glGetProgramResourceiv,
    pub glMultiDrawArraysIndirect: crate::gl::command::PFN_glMultiDrawArraysIndirect,
    pub glDispatchComputeIndirect: crate::gl::command::PFN_glDispatchComputeIndirect,
    pub glShaderStorageBlockBinding: crate::gl::command::PFN_glShaderStorageBlockBinding,
    pub glTexStorage2DMultisample: crate::gl::command::PFN_glTexStorage2DMultisample,
    pub glGetPointerv: crate::gl::command::PFN_glGetPointerv,
    pub glGetProgramInterfaceiv: crate::gl::command::PFN_glGetProgramInterfaceiv,
    pub glInvalidateBufferSubData: crate::gl::command::PFN_glInvalidateBufferSubData,
    pub glGetInternalformati64v: crate::gl::command::PFN_glGetInternalformati64v,
    pub glFramebufferParameteri: crate::gl::command::PFN_glFramebufferParameteri,
    pub glVertexAttribIFormat: crate::gl::command::PFN_glVertexAttribIFormat,
}
impl EntryFnGL43 {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            glGetDebugMessageLog: unsafe {
                unsafe extern "system" fn __glGetDebugMessageLog(
                    _count: GLuint,
                    _bufSize: GLsizei,
                    _sources: *mut DebugSource,
                    _types: *mut DebugType,
                    _ids: *mut GLuint,
                    _severities: *mut DebugSeverity,
                    _lengths: *mut GLsizei,
                    _messageLog: *mut GLchar,
                ) -> GLuint {
                    panic!("Unable to load glGetDebugMessageLog")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetDebugMessageLog\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetDebugMessageLog
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexBindingDivisor: unsafe {
                unsafe extern "system" fn __glVertexBindingDivisor(
                    _bindingindex: GLuint,
                    _divisor: GLuint,
                ) {
                    panic!("Unable to load glVertexBindingDivisor")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexBindingDivisor\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexBindingDivisor
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTextureView: unsafe {
                unsafe extern "system" fn __glTextureView(
                    _texture: GLuint,
                    _target: TextureTarget,
                    _origtexture: GLuint,
                    _internalformat: SizedInternalFormat,
                    _minlevel: GLuint,
                    _numlevels: GLuint,
                    _minlayer: GLuint,
                    _numlayers: GLuint,
                ) {
                    panic!("Unable to load glTextureView")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTextureView\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTextureView
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPopDebugGroup: unsafe {
                unsafe extern "system" fn __glPopDebugGroup() {
                    panic!("Unable to load glPopDebugGroup")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPopDebugGroup\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPopDebugGroup
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDispatchCompute: unsafe {
                unsafe extern "system" fn __glDispatchCompute(
                    _num_groups_x: GLuint,
                    _num_groups_y: GLuint,
                    _num_groups_z: GLuint,
                ) {
                    panic!("Unable to load glDispatchCompute")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDispatchCompute\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDispatchCompute
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glInvalidateSubFramebuffer: unsafe {
                unsafe extern "system" fn __glInvalidateSubFramebuffer(
                    _target: FramebufferTarget,
                    _numAttachments: GLsizei,
                    _attachments: *mut InvalidateFramebufferAttachment,
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                ) {
                    panic!("Unable to load glInvalidateSubFramebuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glInvalidateSubFramebuffer\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glInvalidateSubFramebuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetProgramResourceIndex: unsafe {
                unsafe extern "system" fn __glGetProgramResourceIndex(
                    _program: GLuint,
                    _programInterface: ProgramInterface,
                    _name: *mut GLchar,
                ) -> GLuint {
                    panic!("Unable to load glGetProgramResourceIndex")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetProgramResourceIndex\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetProgramResourceIndex
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDebugMessageInsert: unsafe {
                unsafe extern "system" fn __glDebugMessageInsert(
                    _source: DebugSource,
                    _type: DebugType,
                    _id: GLuint,
                    _severity: DebugSeverity,
                    _length: GLsizei,
                    _buf: *mut GLchar,
                ) {
                    panic!("Unable to load glDebugMessageInsert")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDebugMessageInsert\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDebugMessageInsert
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetObjectPtrLabel: unsafe {
                unsafe extern "system" fn __glGetObjectPtrLabel(
                    _ptr: GLvoid,
                    _bufSize: GLsizei,
                    _length: *mut GLsizei,
                    _label: *mut GLchar,
                ) {
                    panic!("Unable to load glGetObjectPtrLabel")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetObjectPtrLabel\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetObjectPtrLabel
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glInvalidateTexSubImage: unsafe {
                unsafe extern "system" fn __glInvalidateTexSubImage(
                    _texture: GLuint,
                    _level: GLint,
                    _xoffset: GLint,
                    _yoffset: GLint,
                    _zoffset: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                    _depth: GLsizei,
                ) {
                    panic!("Unable to load glInvalidateTexSubImage")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glInvalidateTexSubImage\0");
                let val = _f(cname);
                if val.is_null() {
                    __glInvalidateTexSubImage
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiDrawElementsIndirect: unsafe {
                unsafe extern "system" fn __glMultiDrawElementsIndirect(
                    _mode: PrimitiveType,
                    _type: DrawElementsType,
                    _indirect: *mut GLvoid,
                    _drawcount: GLsizei,
                    _stride: GLsizei,
                ) {
                    panic!("Unable to load glMultiDrawElementsIndirect")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glMultiDrawElementsIndirect\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glMultiDrawElementsIndirect
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribFormat: unsafe {
                unsafe extern "system" fn __glVertexAttribFormat(
                    _attribindex: GLuint,
                    _size: GLint,
                    _type: VertexAttribType,
                    _normalized: Boolean,
                    _relativeoffset: GLuint,
                ) {
                    panic!("Unable to load glVertexAttribFormat")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribFormat\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribFormat
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetObjectLabel: unsafe {
                unsafe extern "system" fn __glGetObjectLabel(
                    _identifier: ObjectIdentifier,
                    _name: GLuint,
                    _bufSize: GLsizei,
                    _length: *mut GLsizei,
                    _label: *mut GLchar,
                ) {
                    panic!("Unable to load glGetObjectLabel")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetObjectLabel\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetObjectLabel
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPushDebugGroup: unsafe {
                unsafe extern "system" fn __glPushDebugGroup(
                    _source: DebugSource,
                    _id: GLuint,
                    _length: GLsizei,
                    _message: *mut GLchar,
                ) {
                    panic!("Unable to load glPushDebugGroup")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPushDebugGroup\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPushDebugGroup
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribBinding: unsafe {
                unsafe extern "system" fn __glVertexAttribBinding(
                    _attribindex: GLuint,
                    _bindingindex: GLuint,
                ) {
                    panic!("Unable to load glVertexAttribBinding")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribBinding\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribBinding
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexStorage3DMultisample: unsafe {
                unsafe extern "system" fn __glTexStorage3DMultisample(
                    _target: TextureTarget,
                    _samples: GLsizei,
                    _internalformat: SizedInternalFormat,
                    _width: GLsizei,
                    _height: GLsizei,
                    _depth: GLsizei,
                    _fixedsamplelocations: Boolean,
                ) {
                    panic!("Unable to load glTexStorage3DMultisample")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexStorage3DMultisample\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexStorage3DMultisample
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDebugMessageControl: unsafe {
                unsafe extern "system" fn __glDebugMessageControl(
                    _source: DebugSource,
                    _type: DebugType,
                    _severity: DebugSeverity,
                    _count: GLsizei,
                    _ids: *mut GLuint,
                    _enabled: Boolean,
                ) {
                    panic!("Unable to load glDebugMessageControl")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDebugMessageControl\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDebugMessageControl
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glInvalidateBufferData: unsafe {
                unsafe extern "system" fn __glInvalidateBufferData(_buffer: GLuint) {
                    panic!("Unable to load glInvalidateBufferData")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glInvalidateBufferData\0");
                let val = _f(cname);
                if val.is_null() {
                    __glInvalidateBufferData
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDebugMessageCallback: unsafe {
                unsafe extern "system" fn __glDebugMessageCallback(
                    _callback: GLDEBUGPROC,
                    _userParam: GLvoid,
                ) {
                    panic!("Unable to load glDebugMessageCallback")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDebugMessageCallback\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDebugMessageCallback
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glInvalidateTexImage: unsafe {
                unsafe extern "system" fn __glInvalidateTexImage(_texture: GLuint, _level: GLint) {
                    panic!("Unable to load glInvalidateTexImage")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glInvalidateTexImage\0");
                let val = _f(cname);
                if val.is_null() {
                    __glInvalidateTexImage
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBindVertexBuffer: unsafe {
                unsafe extern "system" fn __glBindVertexBuffer(
                    _bindingindex: GLuint,
                    _buffer: GLuint,
                    _offset: GLintptr,
                    _stride: GLsizei,
                ) {
                    panic!("Unable to load glBindVertexBuffer")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindVertexBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindVertexBuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetProgramResourceLocationIndex: unsafe {
                unsafe extern "system" fn __glGetProgramResourceLocationIndex(
                    _program: GLuint,
                    _programInterface: ProgramInterface,
                    _name: *mut GLchar,
                ) -> GLint {
                    panic!("Unable to load glGetProgramResourceLocationIndex")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetProgramResourceLocationIndex\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetProgramResourceLocationIndex
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetFramebufferParameteriv: unsafe {
                unsafe extern "system" fn __glGetFramebufferParameteriv(
                    _target: FramebufferTarget,
                    _pname: FramebufferAttachmentParameterName,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetFramebufferParameteriv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetFramebufferParameteriv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetFramebufferParameteriv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glInvalidateFramebuffer: unsafe {
                unsafe extern "system" fn __glInvalidateFramebuffer(
                    _target: FramebufferTarget,
                    _numAttachments: GLsizei,
                    _attachments: *mut InvalidateFramebufferAttachment,
                ) {
                    panic!("Unable to load glInvalidateFramebuffer")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glInvalidateFramebuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glInvalidateFramebuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glObjectPtrLabel: unsafe {
                unsafe extern "system" fn __glObjectPtrLabel(
                    _ptr: GLvoid,
                    _length: GLsizei,
                    _label: *mut GLchar,
                ) {
                    panic!("Unable to load glObjectPtrLabel")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glObjectPtrLabel\0");
                let val = _f(cname);
                if val.is_null() {
                    __glObjectPtrLabel
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glObjectLabel: unsafe {
                unsafe extern "system" fn __glObjectLabel(
                    _identifier: ObjectIdentifier,
                    _name: GLuint,
                    _length: GLsizei,
                    _label: *mut GLchar,
                ) {
                    panic!("Unable to load glObjectLabel")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glObjectLabel\0");
                let val = _f(cname);
                if val.is_null() {
                    __glObjectLabel
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexBufferRange: unsafe {
                unsafe extern "system" fn __glTexBufferRange(
                    _target: TextureTarget,
                    _internalformat: SizedInternalFormat,
                    _buffer: GLuint,
                    _offset: GLintptr,
                    _size: GLsizeiptr,
                ) {
                    panic!("Unable to load glTexBufferRange")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexBufferRange\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexBufferRange
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetProgramResourceName: unsafe {
                unsafe extern "system" fn __glGetProgramResourceName(
                    _program: GLuint,
                    _programInterface: ProgramInterface,
                    _index: GLuint,
                    _bufSize: GLsizei,
                    _length: *mut GLsizei,
                    _name: *mut GLchar,
                ) {
                    panic!("Unable to load glGetProgramResourceName")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetProgramResourceName\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetProgramResourceName
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribLFormat: unsafe {
                unsafe extern "system" fn __glVertexAttribLFormat(
                    _attribindex: GLuint,
                    _size: GLint,
                    _type: VertexAttribLType,
                    _relativeoffset: GLuint,
                ) {
                    panic!("Unable to load glVertexAttribLFormat")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribLFormat\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribLFormat
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetProgramResourceLocation: unsafe {
                unsafe extern "system" fn __glGetProgramResourceLocation(
                    _program: GLuint,
                    _programInterface: ProgramInterface,
                    _name: *mut GLchar,
                ) -> GLint {
                    panic!("Unable to load glGetProgramResourceLocation")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetProgramResourceLocation\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetProgramResourceLocation
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glClearBufferData: unsafe {
                unsafe extern "system" fn __glClearBufferData(
                    _target: BufferStorageTarget,
                    _internalformat: SizedInternalFormat,
                    _format: PixelFormat,
                    _type: PixelType,
                    _data: *mut GLvoid,
                ) {
                    panic!("Unable to load glClearBufferData")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClearBufferData\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClearBufferData
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCopyImageSubData: unsafe {
                unsafe extern "system" fn __glCopyImageSubData(
                    _srcName: GLuint,
                    _srcTarget: CopyImageSubDataTarget,
                    _srcLevel: GLint,
                    _srcX: GLint,
                    _srcY: GLint,
                    _srcZ: GLint,
                    _dstName: GLuint,
                    _dstTarget: CopyImageSubDataTarget,
                    _dstLevel: GLint,
                    _dstX: GLint,
                    _dstY: GLint,
                    _dstZ: GLint,
                    _srcWidth: GLsizei,
                    _srcHeight: GLsizei,
                    _srcDepth: GLsizei,
                ) {
                    panic!("Unable to load glCopyImageSubData")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCopyImageSubData\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCopyImageSubData
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glClearBufferSubData: unsafe {
                unsafe extern "system" fn __glClearBufferSubData(
                    _target: BufferTargetARB,
                    _internalformat: SizedInternalFormat,
                    _offset: GLintptr,
                    _size: GLsizeiptr,
                    _format: PixelFormat,
                    _type: PixelType,
                    _data: *mut GLvoid,
                ) {
                    panic!("Unable to load glClearBufferSubData")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClearBufferSubData\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClearBufferSubData
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetProgramResourceiv: unsafe {
                unsafe extern "system" fn __glGetProgramResourceiv(
                    _program: GLuint,
                    _programInterface: ProgramInterface,
                    _index: GLuint,
                    _propCount: GLsizei,
                    _props: *mut ProgramResourceProperty,
                    _count: GLsizei,
                    _length: *mut GLsizei,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetProgramResourceiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetProgramResourceiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetProgramResourceiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiDrawArraysIndirect: unsafe {
                unsafe extern "system" fn __glMultiDrawArraysIndirect(
                    _mode: PrimitiveType,
                    _indirect: *mut GLvoid,
                    _drawcount: GLsizei,
                    _stride: GLsizei,
                ) {
                    panic!("Unable to load glMultiDrawArraysIndirect")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiDrawArraysIndirect\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiDrawArraysIndirect
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDispatchComputeIndirect: unsafe {
                unsafe extern "system" fn __glDispatchComputeIndirect(_indirect: GLintptr) {
                    panic!("Unable to load glDispatchComputeIndirect")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDispatchComputeIndirect\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDispatchComputeIndirect
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glShaderStorageBlockBinding: unsafe {
                unsafe extern "system" fn __glShaderStorageBlockBinding(
                    _program: GLuint,
                    _storageBlockIndex: GLuint,
                    _storageBlockBinding: GLuint,
                ) {
                    panic!("Unable to load glShaderStorageBlockBinding")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glShaderStorageBlockBinding\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glShaderStorageBlockBinding
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexStorage2DMultisample: unsafe {
                unsafe extern "system" fn __glTexStorage2DMultisample(
                    _target: TextureTarget,
                    _samples: GLsizei,
                    _internalformat: SizedInternalFormat,
                    _width: GLsizei,
                    _height: GLsizei,
                    _fixedsamplelocations: Boolean,
                ) {
                    panic!("Unable to load glTexStorage2DMultisample")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexStorage2DMultisample\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexStorage2DMultisample
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetPointerv: unsafe {
                unsafe extern "system" fn __glGetPointerv(
                    _pname: GetPointervPName,
                    _params: *mut GLvoid,
                ) {
                    panic!("Unable to load glGetPointerv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetPointerv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetPointerv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetProgramInterfaceiv: unsafe {
                unsafe extern "system" fn __glGetProgramInterfaceiv(
                    _program: GLuint,
                    _programInterface: ProgramInterface,
                    _pname: ProgramInterfacePName,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetProgramInterfaceiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetProgramInterfaceiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetProgramInterfaceiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glInvalidateBufferSubData: unsafe {
                unsafe extern "system" fn __glInvalidateBufferSubData(
                    _buffer: GLuint,
                    _offset: GLintptr,
                    _length: GLsizeiptr,
                ) {
                    panic!("Unable to load glInvalidateBufferSubData")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glInvalidateBufferSubData\0");
                let val = _f(cname);
                if val.is_null() {
                    __glInvalidateBufferSubData
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetInternalformati64v: unsafe {
                unsafe extern "system" fn __glGetInternalformati64v(
                    _target: TextureTarget,
                    _internalformat: InternalFormat,
                    _pname: InternalFormatPName,
                    _count: GLsizei,
                    _params: *mut GLint64,
                ) {
                    panic!("Unable to load glGetInternalformati64v")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetInternalformati64v\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetInternalformati64v
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glFramebufferParameteri: unsafe {
                unsafe extern "system" fn __glFramebufferParameteri(
                    _target: FramebufferTarget,
                    _pname: FramebufferParameterName,
                    _param: GLint,
                ) {
                    panic!("Unable to load glFramebufferParameteri")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFramebufferParameteri\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFramebufferParameteri
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribIFormat: unsafe {
                unsafe extern "system" fn __glVertexAttribIFormat(
                    _attribindex: GLuint,
                    _size: GLint,
                    _type: VertexAttribIType,
                    _relativeoffset: GLuint,
                ) {
                    panic!("Unable to load glVertexAttribIFormat")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribIFormat\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribIFormat
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[derive(Clone)]
pub struct EntryFnGL14 {
    pub glWindowPos3dv: crate::gl::command::PFN_glWindowPos3dv,
    pub glBlendEquation: crate::gl::command::PFN_glBlendEquation,
    pub glSecondaryColor3d: crate::gl::command::PFN_glSecondaryColor3d,
    pub glPointParameteriv: crate::gl::command::PFN_glPointParameteriv,
    pub glWindowPos2i: crate::gl::command::PFN_glWindowPos2i,
    pub glMultiDrawArrays: crate::gl::command::PFN_glMultiDrawArrays,
    pub glPointParameteri: crate::gl::command::PFN_glPointParameteri,
    pub glSecondaryColor3usv: crate::gl::command::PFN_glSecondaryColor3usv,
    pub glFogCoordfv: crate::gl::command::PFN_glFogCoordfv,
    pub glWindowPos3fv: crate::gl::command::PFN_glWindowPos3fv,
    pub glSecondaryColorPointer: crate::gl::command::PFN_glSecondaryColorPointer,
    pub glWindowPos3i: crate::gl::command::PFN_glWindowPos3i,
    pub glWindowPos3sv: crate::gl::command::PFN_glWindowPos3sv,
    pub glMultiDrawElements: crate::gl::command::PFN_glMultiDrawElements,
    pub glWindowPos2sv: crate::gl::command::PFN_glWindowPos2sv,
    pub glSecondaryColor3bv: crate::gl::command::PFN_glSecondaryColor3bv,
    pub glFogCoordf: crate::gl::command::PFN_glFogCoordf,
    pub glSecondaryColor3us: crate::gl::command::PFN_glSecondaryColor3us,
    pub glFogCoordPointer: crate::gl::command::PFN_glFogCoordPointer,
    pub glSecondaryColor3ubv: crate::gl::command::PFN_glSecondaryColor3ubv,
    pub glSecondaryColor3s: crate::gl::command::PFN_glSecondaryColor3s,
    pub glPointParameterfv: crate::gl::command::PFN_glPointParameterfv,
    pub glWindowPos3iv: crate::gl::command::PFN_glWindowPos3iv,
    pub glBlendFuncSeparate: crate::gl::command::PFN_glBlendFuncSeparate,
    pub glWindowPos2f: crate::gl::command::PFN_glWindowPos2f,
    pub glWindowPos3d: crate::gl::command::PFN_glWindowPos3d,
    pub glSecondaryColor3iv: crate::gl::command::PFN_glSecondaryColor3iv,
    pub glWindowPos2d: crate::gl::command::PFN_glWindowPos2d,
    pub glSecondaryColor3dv: crate::gl::command::PFN_glSecondaryColor3dv,
    pub glSecondaryColor3ui: crate::gl::command::PFN_glSecondaryColor3ui,
    pub glSecondaryColor3b: crate::gl::command::PFN_glSecondaryColor3b,
    pub glSecondaryColor3sv: crate::gl::command::PFN_glSecondaryColor3sv,
    pub glSecondaryColor3fv: crate::gl::command::PFN_glSecondaryColor3fv,
    pub glSecondaryColor3uiv: crate::gl::command::PFN_glSecondaryColor3uiv,
    pub glPointParameterf: crate::gl::command::PFN_glPointParameterf,
    pub glWindowPos3f: crate::gl::command::PFN_glWindowPos3f,
    pub glWindowPos3s: crate::gl::command::PFN_glWindowPos3s,
    pub glSecondaryColor3i: crate::gl::command::PFN_glSecondaryColor3i,
    pub glWindowPos2s: crate::gl::command::PFN_glWindowPos2s,
    pub glSecondaryColor3f: crate::gl::command::PFN_glSecondaryColor3f,
    pub glBlendColor: crate::gl::command::PFN_glBlendColor,
    pub glSecondaryColor3ub: crate::gl::command::PFN_glSecondaryColor3ub,
    pub glFogCoorddv: crate::gl::command::PFN_glFogCoorddv,
    pub glWindowPos2dv: crate::gl::command::PFN_glWindowPos2dv,
    pub glWindowPos2fv: crate::gl::command::PFN_glWindowPos2fv,
    pub glWindowPos2iv: crate::gl::command::PFN_glWindowPos2iv,
    pub glFogCoordd: crate::gl::command::PFN_glFogCoordd,
}
impl EntryFnGL14 {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            glWindowPos3dv: unsafe {
                unsafe extern "system" fn __glWindowPos3dv(_v: *mut GLdouble) {
                    panic!("Unable to load glWindowPos3dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glWindowPos3dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glWindowPos3dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBlendEquation: unsafe {
                unsafe extern "system" fn __glBlendEquation(_mode: BlendEquationModeEXT) {
                    panic!("Unable to load glBlendEquation")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBlendEquation\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBlendEquation
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glSecondaryColor3d: unsafe {
                unsafe extern "system" fn __glSecondaryColor3d(
                    _red: GLdouble,
                    _green: GLdouble,
                    _blue: GLdouble,
                ) {
                    panic!("Unable to load glSecondaryColor3d")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSecondaryColor3d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSecondaryColor3d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPointParameteriv: unsafe {
                unsafe extern "system" fn __glPointParameteriv(
                    _pname: PointParameterNameARB,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glPointParameteriv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPointParameteriv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPointParameteriv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glWindowPos2i: unsafe {
                unsafe extern "system" fn __glWindowPos2i(_x: GLint, _y: GLint) {
                    panic!("Unable to load glWindowPos2i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glWindowPos2i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glWindowPos2i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiDrawArrays: unsafe {
                unsafe extern "system" fn __glMultiDrawArrays(
                    _mode: PrimitiveType,
                    _first: *mut GLint,
                    _count: *mut GLsizei,
                    _drawcount: GLsizei,
                ) {
                    panic!("Unable to load glMultiDrawArrays")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiDrawArrays\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiDrawArrays
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPointParameteri: unsafe {
                unsafe extern "system" fn __glPointParameteri(
                    _pname: PointParameterNameARB,
                    _param: GLint,
                ) {
                    panic!("Unable to load glPointParameteri")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPointParameteri\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPointParameteri
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glSecondaryColor3usv: unsafe {
                unsafe extern "system" fn __glSecondaryColor3usv(_v: *mut GLushort) {
                    panic!("Unable to load glSecondaryColor3usv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSecondaryColor3usv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSecondaryColor3usv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glFogCoordfv: unsafe {
                unsafe extern "system" fn __glFogCoordfv(_coord: *mut GLfloat) {
                    panic!("Unable to load glFogCoordfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFogCoordfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFogCoordfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glWindowPos3fv: unsafe {
                unsafe extern "system" fn __glWindowPos3fv(_v: *mut GLfloat) {
                    panic!("Unable to load glWindowPos3fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glWindowPos3fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glWindowPos3fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glSecondaryColorPointer: unsafe {
                unsafe extern "system" fn __glSecondaryColorPointer(
                    _size: GLint,
                    _type: ColorPointerType,
                    _stride: GLsizei,
                    _pointer: *mut GLvoid,
                ) {
                    panic!("Unable to load glSecondaryColorPointer")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSecondaryColorPointer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSecondaryColorPointer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glWindowPos3i: unsafe {
                unsafe extern "system" fn __glWindowPos3i(_x: GLint, _y: GLint, _z: GLint) {
                    panic!("Unable to load glWindowPos3i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glWindowPos3i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glWindowPos3i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glWindowPos3sv: unsafe {
                unsafe extern "system" fn __glWindowPos3sv(_v: *mut GLshort) {
                    panic!("Unable to load glWindowPos3sv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glWindowPos3sv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glWindowPos3sv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiDrawElements: unsafe {
                unsafe extern "system" fn __glMultiDrawElements(
                    _mode: PrimitiveType,
                    _count: *mut GLsizei,
                    _type: DrawElementsType,
                    _indices: *mut GLvoid,
                    _drawcount: GLsizei,
                ) {
                    panic!("Unable to load glMultiDrawElements")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiDrawElements\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiDrawElements
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glWindowPos2sv: unsafe {
                unsafe extern "system" fn __glWindowPos2sv(_v: *mut GLshort) {
                    panic!("Unable to load glWindowPos2sv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glWindowPos2sv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glWindowPos2sv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glSecondaryColor3bv: unsafe {
                unsafe extern "system" fn __glSecondaryColor3bv(_v: *mut GLbyte) {
                    panic!("Unable to load glSecondaryColor3bv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSecondaryColor3bv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSecondaryColor3bv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glFogCoordf: unsafe {
                unsafe extern "system" fn __glFogCoordf(_coord: GLfloat) {
                    panic!("Unable to load glFogCoordf")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFogCoordf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFogCoordf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glSecondaryColor3us: unsafe {
                unsafe extern "system" fn __glSecondaryColor3us(
                    _red: GLushort,
                    _green: GLushort,
                    _blue: GLushort,
                ) {
                    panic!("Unable to load glSecondaryColor3us")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSecondaryColor3us\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSecondaryColor3us
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glFogCoordPointer: unsafe {
                unsafe extern "system" fn __glFogCoordPointer(
                    _type: FogPointerTypeEXT,
                    _stride: GLsizei,
                    _pointer: *mut GLvoid,
                ) {
                    panic!("Unable to load glFogCoordPointer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFogCoordPointer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFogCoordPointer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glSecondaryColor3ubv: unsafe {
                unsafe extern "system" fn __glSecondaryColor3ubv(_v: *mut GLubyte) {
                    panic!("Unable to load glSecondaryColor3ubv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSecondaryColor3ubv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSecondaryColor3ubv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glSecondaryColor3s: unsafe {
                unsafe extern "system" fn __glSecondaryColor3s(
                    _red: GLshort,
                    _green: GLshort,
                    _blue: GLshort,
                ) {
                    panic!("Unable to load glSecondaryColor3s")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSecondaryColor3s\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSecondaryColor3s
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPointParameterfv: unsafe {
                unsafe extern "system" fn __glPointParameterfv(
                    _pname: PointParameterNameARB,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load glPointParameterfv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPointParameterfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPointParameterfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glWindowPos3iv: unsafe {
                unsafe extern "system" fn __glWindowPos3iv(_v: *mut GLint) {
                    panic!("Unable to load glWindowPos3iv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glWindowPos3iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glWindowPos3iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBlendFuncSeparate: unsafe {
                unsafe extern "system" fn __glBlendFuncSeparate(
                    _sfactorRGB: BlendingFactor,
                    _dfactorRGB: BlendingFactor,
                    _sfactorAlpha: BlendingFactor,
                    _dfactorAlpha: BlendingFactor,
                ) {
                    panic!("Unable to load glBlendFuncSeparate")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBlendFuncSeparate\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBlendFuncSeparate
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glWindowPos2f: unsafe {
                unsafe extern "system" fn __glWindowPos2f(_x: GLfloat, _y: GLfloat) {
                    panic!("Unable to load glWindowPos2f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glWindowPos2f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glWindowPos2f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glWindowPos3d: unsafe {
                unsafe extern "system" fn __glWindowPos3d(
                    _x: GLdouble,
                    _y: GLdouble,
                    _z: GLdouble,
                ) {
                    panic!("Unable to load glWindowPos3d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glWindowPos3d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glWindowPos3d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glSecondaryColor3iv: unsafe {
                unsafe extern "system" fn __glSecondaryColor3iv(_v: *mut GLint) {
                    panic!("Unable to load glSecondaryColor3iv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSecondaryColor3iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSecondaryColor3iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glWindowPos2d: unsafe {
                unsafe extern "system" fn __glWindowPos2d(_x: GLdouble, _y: GLdouble) {
                    panic!("Unable to load glWindowPos2d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glWindowPos2d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glWindowPos2d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glSecondaryColor3dv: unsafe {
                unsafe extern "system" fn __glSecondaryColor3dv(_v: *mut GLdouble) {
                    panic!("Unable to load glSecondaryColor3dv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSecondaryColor3dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSecondaryColor3dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glSecondaryColor3ui: unsafe {
                unsafe extern "system" fn __glSecondaryColor3ui(
                    _red: GLuint,
                    _green: GLuint,
                    _blue: GLuint,
                ) {
                    panic!("Unable to load glSecondaryColor3ui")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSecondaryColor3ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSecondaryColor3ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glSecondaryColor3b: unsafe {
                unsafe extern "system" fn __glSecondaryColor3b(
                    _red: GLbyte,
                    _green: GLbyte,
                    _blue: GLbyte,
                ) {
                    panic!("Unable to load glSecondaryColor3b")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSecondaryColor3b\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSecondaryColor3b
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glSecondaryColor3sv: unsafe {
                unsafe extern "system" fn __glSecondaryColor3sv(_v: *mut GLshort) {
                    panic!("Unable to load glSecondaryColor3sv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSecondaryColor3sv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSecondaryColor3sv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glSecondaryColor3fv: unsafe {
                unsafe extern "system" fn __glSecondaryColor3fv(_v: *mut GLfloat) {
                    panic!("Unable to load glSecondaryColor3fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSecondaryColor3fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSecondaryColor3fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glSecondaryColor3uiv: unsafe {
                unsafe extern "system" fn __glSecondaryColor3uiv(_v: *mut GLuint) {
                    panic!("Unable to load glSecondaryColor3uiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSecondaryColor3uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSecondaryColor3uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPointParameterf: unsafe {
                unsafe extern "system" fn __glPointParameterf(
                    _pname: PointParameterNameARB,
                    _param: GLfloat,
                ) {
                    panic!("Unable to load glPointParameterf")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPointParameterf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPointParameterf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glWindowPos3f: unsafe {
                unsafe extern "system" fn __glWindowPos3f(_x: GLfloat, _y: GLfloat, _z: GLfloat) {
                    panic!("Unable to load glWindowPos3f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glWindowPos3f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glWindowPos3f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glWindowPos3s: unsafe {
                unsafe extern "system" fn __glWindowPos3s(_x: GLshort, _y: GLshort, _z: GLshort) {
                    panic!("Unable to load glWindowPos3s")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glWindowPos3s\0");
                let val = _f(cname);
                if val.is_null() {
                    __glWindowPos3s
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glSecondaryColor3i: unsafe {
                unsafe extern "system" fn __glSecondaryColor3i(
                    _red: GLint,
                    _green: GLint,
                    _blue: GLint,
                ) {
                    panic!("Unable to load glSecondaryColor3i")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSecondaryColor3i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSecondaryColor3i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glWindowPos2s: unsafe {
                unsafe extern "system" fn __glWindowPos2s(_x: GLshort, _y: GLshort) {
                    panic!("Unable to load glWindowPos2s")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glWindowPos2s\0");
                let val = _f(cname);
                if val.is_null() {
                    __glWindowPos2s
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glSecondaryColor3f: unsafe {
                unsafe extern "system" fn __glSecondaryColor3f(
                    _red: GLfloat,
                    _green: GLfloat,
                    _blue: GLfloat,
                ) {
                    panic!("Unable to load glSecondaryColor3f")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSecondaryColor3f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSecondaryColor3f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBlendColor: unsafe {
                unsafe extern "system" fn __glBlendColor(
                    _red: GLfloat,
                    _green: GLfloat,
                    _blue: GLfloat,
                    _alpha: GLfloat,
                ) {
                    panic!("Unable to load glBlendColor")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBlendColor\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBlendColor
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glSecondaryColor3ub: unsafe {
                unsafe extern "system" fn __glSecondaryColor3ub(
                    _red: GLubyte,
                    _green: GLubyte,
                    _blue: GLubyte,
                ) {
                    panic!("Unable to load glSecondaryColor3ub")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSecondaryColor3ub\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSecondaryColor3ub
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glFogCoorddv: unsafe {
                unsafe extern "system" fn __glFogCoorddv(_coord: *mut GLdouble) {
                    panic!("Unable to load glFogCoorddv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFogCoorddv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFogCoorddv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glWindowPos2dv: unsafe {
                unsafe extern "system" fn __glWindowPos2dv(_v: *mut GLdouble) {
                    panic!("Unable to load glWindowPos2dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glWindowPos2dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glWindowPos2dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glWindowPos2fv: unsafe {
                unsafe extern "system" fn __glWindowPos2fv(_v: *mut GLfloat) {
                    panic!("Unable to load glWindowPos2fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glWindowPos2fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glWindowPos2fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glWindowPos2iv: unsafe {
                unsafe extern "system" fn __glWindowPos2iv(_v: *mut GLint) {
                    panic!("Unable to load glWindowPos2iv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glWindowPos2iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glWindowPos2iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glFogCoordd: unsafe {
                unsafe extern "system" fn __glFogCoordd(_coord: GLdouble) {
                    panic!("Unable to load glFogCoordd")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFogCoordd\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFogCoordd
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[derive(Clone)]
pub struct EntryFnGL33 {
    pub glSamplerParameteri: crate::gl::command::PFN_glSamplerParameteri,
    pub glNormalP3ui: crate::gl::command::PFN_glNormalP3ui,
    pub glMultiTexCoordP4uiv: crate::gl::command::PFN_glMultiTexCoordP4uiv,
    pub glTexCoordP4uiv: crate::gl::command::PFN_glTexCoordP4uiv,
    pub glColorP3uiv: crate::gl::command::PFN_glColorP3uiv,
    pub glMultiTexCoordP3uiv: crate::gl::command::PFN_glMultiTexCoordP3uiv,
    pub glVertexP2uiv: crate::gl::command::PFN_glVertexP2uiv,
    pub glGetSamplerParameteriv: crate::gl::command::PFN_glGetSamplerParameteriv,
    pub glVertexP4ui: crate::gl::command::PFN_glVertexP4ui,
    pub glVertexAttribP2uiv: crate::gl::command::PFN_glVertexAttribP2uiv,
    pub glVertexAttribP4ui: crate::gl::command::PFN_glVertexAttribP4ui,
    pub glTexCoordP3uiv: crate::gl::command::PFN_glTexCoordP3uiv,
    pub glNormalP3uiv: crate::gl::command::PFN_glNormalP3uiv,
    pub glDeleteSamplers: crate::gl::command::PFN_glDeleteSamplers,
    pub glIsSampler: crate::gl::command::PFN_glIsSampler,
    pub glVertexAttribDivisor: crate::gl::command::PFN_glVertexAttribDivisor,
    pub glTexCoordP1ui: crate::gl::command::PFN_glTexCoordP1ui,
    pub glMultiTexCoordP3ui: crate::gl::command::PFN_glMultiTexCoordP3ui,
    pub glColorP3ui: crate::gl::command::PFN_glColorP3ui,
    pub glSamplerParameterf: crate::gl::command::PFN_glSamplerParameterf,
    pub glGenSamplers: crate::gl::command::PFN_glGenSamplers,
    pub glGetSamplerParameterIuiv: crate::gl::command::PFN_glGetSamplerParameterIuiv,
    pub glVertexAttribP2ui: crate::gl::command::PFN_glVertexAttribP2ui,
    pub glMultiTexCoordP1uiv: crate::gl::command::PFN_glMultiTexCoordP1uiv,
    pub glGetQueryObjecti64v: crate::gl::command::PFN_glGetQueryObjecti64v,
    pub glSamplerParameterIiv: crate::gl::command::PFN_glSamplerParameterIiv,
    pub glSamplerParameterIuiv: crate::gl::command::PFN_glSamplerParameterIuiv,
    pub glVertexP2ui: crate::gl::command::PFN_glVertexP2ui,
    pub glColorP4ui: crate::gl::command::PFN_glColorP4ui,
    pub glTexCoordP1uiv: crate::gl::command::PFN_glTexCoordP1uiv,
    pub glQueryCounter: crate::gl::command::PFN_glQueryCounter,
    pub glColorP4uiv: crate::gl::command::PFN_glColorP4uiv,
    pub glBindFragDataLocationIndexed: crate::gl::command::PFN_glBindFragDataLocationIndexed,
    pub glTexCoordP4ui: crate::gl::command::PFN_glTexCoordP4ui,
    pub glTexCoordP2uiv: crate::gl::command::PFN_glTexCoordP2uiv,
    pub glVertexP3ui: crate::gl::command::PFN_glVertexP3ui,
    pub glMultiTexCoordP4ui: crate::gl::command::PFN_glMultiTexCoordP4ui,
    pub glVertexAttribP1uiv: crate::gl::command::PFN_glVertexAttribP1uiv,
    pub glSamplerParameterfv: crate::gl::command::PFN_glSamplerParameterfv,
    pub glVertexP3uiv: crate::gl::command::PFN_glVertexP3uiv,
    pub glVertexAttribP1ui: crate::gl::command::PFN_glVertexAttribP1ui,
    pub glVertexAttribP4uiv: crate::gl::command::PFN_glVertexAttribP4uiv,
    pub glSecondaryColorP3uiv: crate::gl::command::PFN_glSecondaryColorP3uiv,
    pub glVertexAttribP3ui: crate::gl::command::PFN_glVertexAttribP3ui,
    pub glBindSampler: crate::gl::command::PFN_glBindSampler,
    pub glGetQueryObjectui64v: crate::gl::command::PFN_glGetQueryObjectui64v,
    pub glTexCoordP3ui: crate::gl::command::PFN_glTexCoordP3ui,
    pub glGetSamplerParameterfv: crate::gl::command::PFN_glGetSamplerParameterfv,
    pub glMultiTexCoordP2uiv: crate::gl::command::PFN_glMultiTexCoordP2uiv,
    pub glMultiTexCoordP1ui: crate::gl::command::PFN_glMultiTexCoordP1ui,
    pub glGetFragDataIndex: crate::gl::command::PFN_glGetFragDataIndex,
    pub glTexCoordP2ui: crate::gl::command::PFN_glTexCoordP2ui,
    pub glGetSamplerParameterIiv: crate::gl::command::PFN_glGetSamplerParameterIiv,
    pub glSamplerParameteriv: crate::gl::command::PFN_glSamplerParameteriv,
    pub glVertexAttribP3uiv: crate::gl::command::PFN_glVertexAttribP3uiv,
    pub glMultiTexCoordP2ui: crate::gl::command::PFN_glMultiTexCoordP2ui,
    pub glSecondaryColorP3ui: crate::gl::command::PFN_glSecondaryColorP3ui,
    pub glVertexP4uiv: crate::gl::command::PFN_glVertexP4uiv,
}
impl EntryFnGL33 {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            glSamplerParameteri: unsafe {
                unsafe extern "system" fn __glSamplerParameteri(
                    _sampler: GLuint,
                    _pname: SamplerParameterI,
                    _param: GLint,
                ) {
                    panic!("Unable to load glSamplerParameteri")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSamplerParameteri\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSamplerParameteri
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glNormalP3ui: unsafe {
                unsafe extern "system" fn __glNormalP3ui(
                    _type: NormalPointerType,
                    _coords: GLuint,
                ) {
                    panic!("Unable to load glNormalP3ui")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glNormalP3ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glNormalP3ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoordP4uiv: unsafe {
                unsafe extern "system" fn __glMultiTexCoordP4uiv(
                    _texture: TextureUnit,
                    _type: TexCoordPointerType,
                    _coords: *mut GLuint,
                ) {
                    panic!("Unable to load glMultiTexCoordP4uiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoordP4uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoordP4uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoordP4uiv: unsafe {
                unsafe extern "system" fn __glTexCoordP4uiv(
                    _type: TexCoordPointerType,
                    _coords: *mut GLuint,
                ) {
                    panic!("Unable to load glTexCoordP4uiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoordP4uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoordP4uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColorP3uiv: unsafe {
                unsafe extern "system" fn __glColorP3uiv(
                    _type: ColorPointerType,
                    _color: *mut GLuint,
                ) {
                    panic!("Unable to load glColorP3uiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColorP3uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColorP3uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoordP3uiv: unsafe {
                unsafe extern "system" fn __glMultiTexCoordP3uiv(
                    _texture: TextureUnit,
                    _type: TexCoordPointerType,
                    _coords: *mut GLuint,
                ) {
                    panic!("Unable to load glMultiTexCoordP3uiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoordP3uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoordP3uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexP2uiv: unsafe {
                unsafe extern "system" fn __glVertexP2uiv(
                    _type: VertexPointerType,
                    _value: *mut GLuint,
                ) {
                    panic!("Unable to load glVertexP2uiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexP2uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexP2uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetSamplerParameteriv: unsafe {
                unsafe extern "system" fn __glGetSamplerParameteriv(
                    _sampler: GLuint,
                    _pname: SamplerParameterI,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetSamplerParameteriv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetSamplerParameteriv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetSamplerParameteriv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexP4ui: unsafe {
                unsafe extern "system" fn __glVertexP4ui(_type: VertexPointerType, _value: GLuint) {
                    panic!("Unable to load glVertexP4ui")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexP4ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexP4ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribP2uiv: unsafe {
                unsafe extern "system" fn __glVertexAttribP2uiv(
                    _index: GLuint,
                    _type: VertexAttribPointerType,
                    _normalized: Boolean,
                    _value: *mut GLuint,
                ) {
                    panic!("Unable to load glVertexAttribP2uiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribP2uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribP2uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribP4ui: unsafe {
                unsafe extern "system" fn __glVertexAttribP4ui(
                    _index: GLuint,
                    _type: VertexAttribPointerType,
                    _normalized: Boolean,
                    _value: GLuint,
                ) {
                    panic!("Unable to load glVertexAttribP4ui")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribP4ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribP4ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoordP3uiv: unsafe {
                unsafe extern "system" fn __glTexCoordP3uiv(
                    _type: TexCoordPointerType,
                    _coords: *mut GLuint,
                ) {
                    panic!("Unable to load glTexCoordP3uiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoordP3uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoordP3uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glNormalP3uiv: unsafe {
                unsafe extern "system" fn __glNormalP3uiv(
                    _type: NormalPointerType,
                    _coords: *mut GLuint,
                ) {
                    panic!("Unable to load glNormalP3uiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glNormalP3uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glNormalP3uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDeleteSamplers: unsafe {
                unsafe extern "system" fn __glDeleteSamplers(
                    _count: GLsizei,
                    _samplers: *mut GLuint,
                ) {
                    panic!("Unable to load glDeleteSamplers")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDeleteSamplers\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDeleteSamplers
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glIsSampler: unsafe {
                unsafe extern "system" fn __glIsSampler(_sampler: GLuint) -> GLboolean {
                    panic!("Unable to load glIsSampler")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIsSampler\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIsSampler
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribDivisor: unsafe {
                unsafe extern "system" fn __glVertexAttribDivisor(
                    _index: GLuint,
                    _divisor: GLuint,
                ) {
                    panic!("Unable to load glVertexAttribDivisor")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribDivisor\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribDivisor
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoordP1ui: unsafe {
                unsafe extern "system" fn __glTexCoordP1ui(
                    _type: TexCoordPointerType,
                    _coords: GLuint,
                ) {
                    panic!("Unable to load glTexCoordP1ui")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoordP1ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoordP1ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoordP3ui: unsafe {
                unsafe extern "system" fn __glMultiTexCoordP3ui(
                    _texture: TextureUnit,
                    _type: TexCoordPointerType,
                    _coords: GLuint,
                ) {
                    panic!("Unable to load glMultiTexCoordP3ui")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoordP3ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoordP3ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColorP3ui: unsafe {
                unsafe extern "system" fn __glColorP3ui(_type: ColorPointerType, _color: GLuint) {
                    panic!("Unable to load glColorP3ui")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColorP3ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColorP3ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glSamplerParameterf: unsafe {
                unsafe extern "system" fn __glSamplerParameterf(
                    _sampler: GLuint,
                    _pname: SamplerParameterF,
                    _param: GLfloat,
                ) {
                    panic!("Unable to load glSamplerParameterf")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSamplerParameterf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSamplerParameterf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGenSamplers: unsafe {
                unsafe extern "system" fn __glGenSamplers(_count: GLsizei, _samplers: *mut GLuint) {
                    panic!("Unable to load glGenSamplers")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGenSamplers\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGenSamplers
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetSamplerParameterIuiv: unsafe {
                unsafe extern "system" fn __glGetSamplerParameterIuiv(
                    _sampler: GLuint,
                    _pname: SamplerParameterI,
                    _params: *mut GLuint,
                ) {
                    panic!("Unable to load glGetSamplerParameterIuiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetSamplerParameterIuiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetSamplerParameterIuiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribP2ui: unsafe {
                unsafe extern "system" fn __glVertexAttribP2ui(
                    _index: GLuint,
                    _type: VertexAttribPointerType,
                    _normalized: Boolean,
                    _value: GLuint,
                ) {
                    panic!("Unable to load glVertexAttribP2ui")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribP2ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribP2ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoordP1uiv: unsafe {
                unsafe extern "system" fn __glMultiTexCoordP1uiv(
                    _texture: TextureUnit,
                    _type: TexCoordPointerType,
                    _coords: *mut GLuint,
                ) {
                    panic!("Unable to load glMultiTexCoordP1uiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoordP1uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoordP1uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetQueryObjecti64v: unsafe {
                unsafe extern "system" fn __glGetQueryObjecti64v(
                    _id: GLuint,
                    _pname: QueryObjectParameterName,
                    _params: *mut GLint64,
                ) {
                    panic!("Unable to load glGetQueryObjecti64v")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetQueryObjecti64v\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetQueryObjecti64v
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glSamplerParameterIiv: unsafe {
                unsafe extern "system" fn __glSamplerParameterIiv(
                    _sampler: GLuint,
                    _pname: SamplerParameterI,
                    _param: *mut GLint,
                ) {
                    panic!("Unable to load glSamplerParameterIiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSamplerParameterIiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSamplerParameterIiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glSamplerParameterIuiv: unsafe {
                unsafe extern "system" fn __glSamplerParameterIuiv(
                    _sampler: GLuint,
                    _pname: SamplerParameterI,
                    _param: *mut GLuint,
                ) {
                    panic!("Unable to load glSamplerParameterIuiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSamplerParameterIuiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSamplerParameterIuiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexP2ui: unsafe {
                unsafe extern "system" fn __glVertexP2ui(_type: VertexPointerType, _value: GLuint) {
                    panic!("Unable to load glVertexP2ui")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexP2ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexP2ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColorP4ui: unsafe {
                unsafe extern "system" fn __glColorP4ui(_type: ColorPointerType, _color: GLuint) {
                    panic!("Unable to load glColorP4ui")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColorP4ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColorP4ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoordP1uiv: unsafe {
                unsafe extern "system" fn __glTexCoordP1uiv(
                    _type: TexCoordPointerType,
                    _coords: *mut GLuint,
                ) {
                    panic!("Unable to load glTexCoordP1uiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoordP1uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoordP1uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glQueryCounter: unsafe {
                unsafe extern "system" fn __glQueryCounter(
                    _id: GLuint,
                    _target: QueryCounterTarget,
                ) {
                    panic!("Unable to load glQueryCounter")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glQueryCounter\0");
                let val = _f(cname);
                if val.is_null() {
                    __glQueryCounter
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColorP4uiv: unsafe {
                unsafe extern "system" fn __glColorP4uiv(
                    _type: ColorPointerType,
                    _color: *mut GLuint,
                ) {
                    panic!("Unable to load glColorP4uiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColorP4uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColorP4uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBindFragDataLocationIndexed: unsafe {
                unsafe extern "system" fn __glBindFragDataLocationIndexed(
                    _program: GLuint,
                    _colorNumber: GLuint,
                    _index: GLuint,
                    _name: GLchar,
                ) {
                    panic!("Unable to load glBindFragDataLocationIndexed")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glBindFragDataLocationIndexed\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glBindFragDataLocationIndexed
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoordP4ui: unsafe {
                unsafe extern "system" fn __glTexCoordP4ui(
                    _type: TexCoordPointerType,
                    _coords: GLuint,
                ) {
                    panic!("Unable to load glTexCoordP4ui")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoordP4ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoordP4ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoordP2uiv: unsafe {
                unsafe extern "system" fn __glTexCoordP2uiv(
                    _type: TexCoordPointerType,
                    _coords: *mut GLuint,
                ) {
                    panic!("Unable to load glTexCoordP2uiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoordP2uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoordP2uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexP3ui: unsafe {
                unsafe extern "system" fn __glVertexP3ui(_type: VertexPointerType, _value: GLuint) {
                    panic!("Unable to load glVertexP3ui")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexP3ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexP3ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoordP4ui: unsafe {
                unsafe extern "system" fn __glMultiTexCoordP4ui(
                    _texture: TextureUnit,
                    _type: TexCoordPointerType,
                    _coords: GLuint,
                ) {
                    panic!("Unable to load glMultiTexCoordP4ui")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoordP4ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoordP4ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribP1uiv: unsafe {
                unsafe extern "system" fn __glVertexAttribP1uiv(
                    _index: GLuint,
                    _type: VertexAttribPointerType,
                    _normalized: Boolean,
                    _value: *mut GLuint,
                ) {
                    panic!("Unable to load glVertexAttribP1uiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribP1uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribP1uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glSamplerParameterfv: unsafe {
                unsafe extern "system" fn __glSamplerParameterfv(
                    _sampler: GLuint,
                    _pname: SamplerParameterF,
                    _param: *mut GLfloat,
                ) {
                    panic!("Unable to load glSamplerParameterfv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSamplerParameterfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSamplerParameterfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexP3uiv: unsafe {
                unsafe extern "system" fn __glVertexP3uiv(
                    _type: VertexPointerType,
                    _value: *mut GLuint,
                ) {
                    panic!("Unable to load glVertexP3uiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexP3uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexP3uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribP1ui: unsafe {
                unsafe extern "system" fn __glVertexAttribP1ui(
                    _index: GLuint,
                    _type: VertexAttribPointerType,
                    _normalized: Boolean,
                    _value: GLuint,
                ) {
                    panic!("Unable to load glVertexAttribP1ui")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribP1ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribP1ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribP4uiv: unsafe {
                unsafe extern "system" fn __glVertexAttribP4uiv(
                    _index: GLuint,
                    _type: VertexAttribPointerType,
                    _normalized: Boolean,
                    _value: *mut GLuint,
                ) {
                    panic!("Unable to load glVertexAttribP4uiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribP4uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribP4uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glSecondaryColorP3uiv: unsafe {
                unsafe extern "system" fn __glSecondaryColorP3uiv(
                    _type: ColorPointerType,
                    _color: *mut GLuint,
                ) {
                    panic!("Unable to load glSecondaryColorP3uiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSecondaryColorP3uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSecondaryColorP3uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribP3ui: unsafe {
                unsafe extern "system" fn __glVertexAttribP3ui(
                    _index: GLuint,
                    _type: VertexAttribPointerType,
                    _normalized: Boolean,
                    _value: GLuint,
                ) {
                    panic!("Unable to load glVertexAttribP3ui")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribP3ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribP3ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBindSampler: unsafe {
                unsafe extern "system" fn __glBindSampler(_unit: GLuint, _sampler: GLuint) {
                    panic!("Unable to load glBindSampler")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindSampler\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindSampler
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetQueryObjectui64v: unsafe {
                unsafe extern "system" fn __glGetQueryObjectui64v(
                    _id: GLuint,
                    _pname: QueryObjectParameterName,
                    _params: *mut GLuint64,
                ) {
                    panic!("Unable to load glGetQueryObjectui64v")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetQueryObjectui64v\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetQueryObjectui64v
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoordP3ui: unsafe {
                unsafe extern "system" fn __glTexCoordP3ui(
                    _type: TexCoordPointerType,
                    _coords: GLuint,
                ) {
                    panic!("Unable to load glTexCoordP3ui")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoordP3ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoordP3ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetSamplerParameterfv: unsafe {
                unsafe extern "system" fn __glGetSamplerParameterfv(
                    _sampler: GLuint,
                    _pname: SamplerParameterF,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load glGetSamplerParameterfv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetSamplerParameterfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetSamplerParameterfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoordP2uiv: unsafe {
                unsafe extern "system" fn __glMultiTexCoordP2uiv(
                    _texture: TextureUnit,
                    _type: TexCoordPointerType,
                    _coords: *mut GLuint,
                ) {
                    panic!("Unable to load glMultiTexCoordP2uiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoordP2uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoordP2uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoordP1ui: unsafe {
                unsafe extern "system" fn __glMultiTexCoordP1ui(
                    _texture: TextureUnit,
                    _type: TexCoordPointerType,
                    _coords: GLuint,
                ) {
                    panic!("Unable to load glMultiTexCoordP1ui")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoordP1ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoordP1ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetFragDataIndex: unsafe {
                unsafe extern "system" fn __glGetFragDataIndex(
                    _program: GLuint,
                    _name: GLchar,
                ) -> GLint {
                    panic!("Unable to load glGetFragDataIndex")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetFragDataIndex\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetFragDataIndex
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoordP2ui: unsafe {
                unsafe extern "system" fn __glTexCoordP2ui(
                    _type: TexCoordPointerType,
                    _coords: GLuint,
                ) {
                    panic!("Unable to load glTexCoordP2ui")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoordP2ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoordP2ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetSamplerParameterIiv: unsafe {
                unsafe extern "system" fn __glGetSamplerParameterIiv(
                    _sampler: GLuint,
                    _pname: SamplerParameterI,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetSamplerParameterIiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetSamplerParameterIiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetSamplerParameterIiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glSamplerParameteriv: unsafe {
                unsafe extern "system" fn __glSamplerParameteriv(
                    _sampler: GLuint,
                    _pname: SamplerParameterI,
                    _param: *mut GLint,
                ) {
                    panic!("Unable to load glSamplerParameteriv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSamplerParameteriv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSamplerParameteriv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexAttribP3uiv: unsafe {
                unsafe extern "system" fn __glVertexAttribP3uiv(
                    _index: GLuint,
                    _type: VertexAttribPointerType,
                    _normalized: Boolean,
                    _value: *mut GLuint,
                ) {
                    panic!("Unable to load glVertexAttribP3uiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribP3uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribP3uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoordP2ui: unsafe {
                unsafe extern "system" fn __glMultiTexCoordP2ui(
                    _texture: TextureUnit,
                    _type: TexCoordPointerType,
                    _coords: GLuint,
                ) {
                    panic!("Unable to load glMultiTexCoordP2ui")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoordP2ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoordP2ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glSecondaryColorP3ui: unsafe {
                unsafe extern "system" fn __glSecondaryColorP3ui(
                    _type: ColorPointerType,
                    _color: GLuint,
                ) {
                    panic!("Unable to load glSecondaryColorP3ui")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSecondaryColorP3ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSecondaryColorP3ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexP4uiv: unsafe {
                unsafe extern "system" fn __glVertexP4uiv(
                    _type: VertexPointerType,
                    _value: *mut GLuint,
                ) {
                    panic!("Unable to load glVertexP4uiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexP4uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexP4uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[derive(Clone)]
pub struct EntryFnGL13 {
    pub glMultiTexCoord3i: crate::gl::command::PFN_glMultiTexCoord3i,
    pub glCompressedTexImage3D: crate::gl::command::PFN_glCompressedTexImage3D,
    pub glLoadTransposeMatrixd: crate::gl::command::PFN_glLoadTransposeMatrixd,
    pub glMultTransposeMatrixd: crate::gl::command::PFN_glMultTransposeMatrixd,
    pub glCompressedTexSubImage1D: crate::gl::command::PFN_glCompressedTexSubImage1D,
    pub glMultiTexCoord1d: crate::gl::command::PFN_glMultiTexCoord1d,
    pub glMultiTexCoord2fv: crate::gl::command::PFN_glMultiTexCoord2fv,
    pub glMultiTexCoord2sv: crate::gl::command::PFN_glMultiTexCoord2sv,
    pub glMultiTexCoord4fv: crate::gl::command::PFN_glMultiTexCoord4fv,
    pub glMultiTexCoord3s: crate::gl::command::PFN_glMultiTexCoord3s,
    pub glClientActiveTexture: crate::gl::command::PFN_glClientActiveTexture,
    pub glMultiTexCoord2i: crate::gl::command::PFN_glMultiTexCoord2i,
    pub glMultiTexCoord3d: crate::gl::command::PFN_glMultiTexCoord3d,
    pub glMultiTexCoord1fv: crate::gl::command::PFN_glMultiTexCoord1fv,
    pub glLoadTransposeMatrixf: crate::gl::command::PFN_glLoadTransposeMatrixf,
    pub glMultiTexCoord4d: crate::gl::command::PFN_glMultiTexCoord4d,
    pub glMultiTexCoord1sv: crate::gl::command::PFN_glMultiTexCoord1sv,
    pub glMultiTexCoord4sv: crate::gl::command::PFN_glMultiTexCoord4sv,
    pub glMultiTexCoord3dv: crate::gl::command::PFN_glMultiTexCoord3dv,
    pub glMultiTexCoord3iv: crate::gl::command::PFN_glMultiTexCoord3iv,
    pub glMultiTexCoord4dv: crate::gl::command::PFN_glMultiTexCoord4dv,
    pub glCompressedTexSubImage2D: crate::gl::command::PFN_glCompressedTexSubImage2D,
    pub glCompressedTexImage2D: crate::gl::command::PFN_glCompressedTexImage2D,
    pub glSampleCoverage: crate::gl::command::PFN_glSampleCoverage,
    pub glMultTransposeMatrixf: crate::gl::command::PFN_glMultTransposeMatrixf,
    pub glMultiTexCoord1f: crate::gl::command::PFN_glMultiTexCoord1f,
    pub glMultiTexCoord2dv: crate::gl::command::PFN_glMultiTexCoord2dv,
    pub glGetCompressedTexImage: crate::gl::command::PFN_glGetCompressedTexImage,
    pub glMultiTexCoord2d: crate::gl::command::PFN_glMultiTexCoord2d,
    pub glMultiTexCoord2iv: crate::gl::command::PFN_glMultiTexCoord2iv,
    pub glMultiTexCoord4s: crate::gl::command::PFN_glMultiTexCoord4s,
    pub glMultiTexCoord1i: crate::gl::command::PFN_glMultiTexCoord1i,
    pub glMultiTexCoord2f: crate::gl::command::PFN_glMultiTexCoord2f,
    pub glMultiTexCoord3f: crate::gl::command::PFN_glMultiTexCoord3f,
    pub glMultiTexCoord3sv: crate::gl::command::PFN_glMultiTexCoord3sv,
    pub glMultiTexCoord3fv: crate::gl::command::PFN_glMultiTexCoord3fv,
    pub glMultiTexCoord1s: crate::gl::command::PFN_glMultiTexCoord1s,
    pub glCompressedTexImage1D: crate::gl::command::PFN_glCompressedTexImage1D,
    pub glMultiTexCoord4f: crate::gl::command::PFN_glMultiTexCoord4f,
    pub glCompressedTexSubImage3D: crate::gl::command::PFN_glCompressedTexSubImage3D,
    pub glMultiTexCoord1iv: crate::gl::command::PFN_glMultiTexCoord1iv,
    pub glMultiTexCoord2s: crate::gl::command::PFN_glMultiTexCoord2s,
    pub glActiveTexture: crate::gl::command::PFN_glActiveTexture,
    pub glMultiTexCoord1dv: crate::gl::command::PFN_glMultiTexCoord1dv,
    pub glMultiTexCoord4i: crate::gl::command::PFN_glMultiTexCoord4i,
    pub glMultiTexCoord4iv: crate::gl::command::PFN_glMultiTexCoord4iv,
}
impl EntryFnGL13 {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            glMultiTexCoord3i: unsafe {
                unsafe extern "system" fn __glMultiTexCoord3i(
                    _target: TextureUnit,
                    _s: GLint,
                    _t: GLint,
                    _r: GLint,
                ) {
                    panic!("Unable to load glMultiTexCoord3i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord3i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord3i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCompressedTexImage3D: unsafe {
                unsafe extern "system" fn __glCompressedTexImage3D(
                    _target: TextureTarget,
                    _level: GLint,
                    _internalformat: InternalFormat,
                    _width: GLsizei,
                    _height: GLsizei,
                    _depth: GLsizei,
                    _border: GLint,
                    _imageSize: GLsizei,
                    _data: *mut GLvoid,
                ) {
                    panic!("Unable to load glCompressedTexImage3D")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCompressedTexImage3D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCompressedTexImage3D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glLoadTransposeMatrixd: unsafe {
                unsafe extern "system" fn __glLoadTransposeMatrixd(_m: *mut GLdouble) {
                    panic!("Unable to load glLoadTransposeMatrixd")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLoadTransposeMatrixd\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLoadTransposeMatrixd
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultTransposeMatrixd: unsafe {
                unsafe extern "system" fn __glMultTransposeMatrixd(_m: *mut GLdouble) {
                    panic!("Unable to load glMultTransposeMatrixd")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultTransposeMatrixd\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultTransposeMatrixd
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCompressedTexSubImage1D: unsafe {
                unsafe extern "system" fn __glCompressedTexSubImage1D(
                    _target: TextureTarget,
                    _level: GLint,
                    _xoffset: GLint,
                    _width: GLsizei,
                    _format: InternalFormat,
                    _imageSize: GLsizei,
                    _data: *mut GLvoid,
                ) {
                    panic!("Unable to load glCompressedTexSubImage1D")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCompressedTexSubImage1D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCompressedTexSubImage1D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord1d: unsafe {
                unsafe extern "system" fn __glMultiTexCoord1d(_target: TextureUnit, _s: GLdouble) {
                    panic!("Unable to load glMultiTexCoord1d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord1d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord1d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord2fv: unsafe {
                unsafe extern "system" fn __glMultiTexCoord2fv(
                    _target: TextureUnit,
                    _v: *mut GLfloat,
                ) {
                    panic!("Unable to load glMultiTexCoord2fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord2fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord2fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord2sv: unsafe {
                unsafe extern "system" fn __glMultiTexCoord2sv(
                    _target: TextureUnit,
                    _v: *mut GLshort,
                ) {
                    panic!("Unable to load glMultiTexCoord2sv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord2sv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord2sv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord4fv: unsafe {
                unsafe extern "system" fn __glMultiTexCoord4fv(
                    _target: TextureUnit,
                    _v: *mut GLfloat,
                ) {
                    panic!("Unable to load glMultiTexCoord4fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord4fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord4fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord3s: unsafe {
                unsafe extern "system" fn __glMultiTexCoord3s(
                    _target: TextureUnit,
                    _s: GLshort,
                    _t: GLshort,
                    _r: GLshort,
                ) {
                    panic!("Unable to load glMultiTexCoord3s")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord3s\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord3s
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glClientActiveTexture: unsafe {
                unsafe extern "system" fn __glClientActiveTexture(_texture: TextureUnit) {
                    panic!("Unable to load glClientActiveTexture")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClientActiveTexture\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClientActiveTexture
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord2i: unsafe {
                unsafe extern "system" fn __glMultiTexCoord2i(
                    _target: TextureUnit,
                    _s: GLint,
                    _t: GLint,
                ) {
                    panic!("Unable to load glMultiTexCoord2i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord2i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord2i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord3d: unsafe {
                unsafe extern "system" fn __glMultiTexCoord3d(
                    _target: TextureUnit,
                    _s: GLdouble,
                    _t: GLdouble,
                    _r: GLdouble,
                ) {
                    panic!("Unable to load glMultiTexCoord3d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord3d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord3d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord1fv: unsafe {
                unsafe extern "system" fn __glMultiTexCoord1fv(
                    _target: TextureUnit,
                    _v: *mut GLfloat,
                ) {
                    panic!("Unable to load glMultiTexCoord1fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord1fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord1fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glLoadTransposeMatrixf: unsafe {
                unsafe extern "system" fn __glLoadTransposeMatrixf(_m: *mut GLfloat) {
                    panic!("Unable to load glLoadTransposeMatrixf")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLoadTransposeMatrixf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLoadTransposeMatrixf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord4d: unsafe {
                unsafe extern "system" fn __glMultiTexCoord4d(
                    _target: TextureUnit,
                    _s: GLdouble,
                    _t: GLdouble,
                    _r: GLdouble,
                    _q: GLdouble,
                ) {
                    panic!("Unable to load glMultiTexCoord4d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord4d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord4d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord1sv: unsafe {
                unsafe extern "system" fn __glMultiTexCoord1sv(
                    _target: TextureUnit,
                    _v: *mut GLshort,
                ) {
                    panic!("Unable to load glMultiTexCoord1sv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord1sv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord1sv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord4sv: unsafe {
                unsafe extern "system" fn __glMultiTexCoord4sv(
                    _target: TextureUnit,
                    _v: *mut GLshort,
                ) {
                    panic!("Unable to load glMultiTexCoord4sv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord4sv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord4sv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord3dv: unsafe {
                unsafe extern "system" fn __glMultiTexCoord3dv(
                    _target: TextureUnit,
                    _v: *mut GLdouble,
                ) {
                    panic!("Unable to load glMultiTexCoord3dv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord3dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord3dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord3iv: unsafe {
                unsafe extern "system" fn __glMultiTexCoord3iv(
                    _target: TextureUnit,
                    _v: *mut GLint,
                ) {
                    panic!("Unable to load glMultiTexCoord3iv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord3iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord3iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord4dv: unsafe {
                unsafe extern "system" fn __glMultiTexCoord4dv(
                    _target: TextureUnit,
                    _v: *mut GLdouble,
                ) {
                    panic!("Unable to load glMultiTexCoord4dv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord4dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord4dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCompressedTexSubImage2D: unsafe {
                unsafe extern "system" fn __glCompressedTexSubImage2D(
                    _target: TextureTarget,
                    _level: GLint,
                    _xoffset: GLint,
                    _yoffset: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                    _format: InternalFormat,
                    _imageSize: GLsizei,
                    _data: *mut GLvoid,
                ) {
                    panic!("Unable to load glCompressedTexSubImage2D")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCompressedTexSubImage2D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCompressedTexSubImage2D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCompressedTexImage2D: unsafe {
                unsafe extern "system" fn __glCompressedTexImage2D(
                    _target: TextureTarget,
                    _level: GLint,
                    _internalformat: InternalFormat,
                    _width: GLsizei,
                    _height: GLsizei,
                    _border: GLint,
                    _imageSize: GLsizei,
                    _data: *mut GLvoid,
                ) {
                    panic!("Unable to load glCompressedTexImage2D")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCompressedTexImage2D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCompressedTexImage2D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glSampleCoverage: unsafe {
                unsafe extern "system" fn __glSampleCoverage(_value: GLfloat, _invert: Boolean) {
                    panic!("Unable to load glSampleCoverage")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSampleCoverage\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSampleCoverage
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultTransposeMatrixf: unsafe {
                unsafe extern "system" fn __glMultTransposeMatrixf(_m: *mut GLfloat) {
                    panic!("Unable to load glMultTransposeMatrixf")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultTransposeMatrixf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultTransposeMatrixf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord1f: unsafe {
                unsafe extern "system" fn __glMultiTexCoord1f(_target: TextureUnit, _s: GLfloat) {
                    panic!("Unable to load glMultiTexCoord1f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord1f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord1f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord2dv: unsafe {
                unsafe extern "system" fn __glMultiTexCoord2dv(
                    _target: TextureUnit,
                    _v: *mut GLdouble,
                ) {
                    panic!("Unable to load glMultiTexCoord2dv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord2dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord2dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetCompressedTexImage: unsafe {
                unsafe extern "system" fn __glGetCompressedTexImage(
                    _target: TextureTarget,
                    _level: GLint,
                    _img: *mut GLvoid,
                ) {
                    panic!("Unable to load glGetCompressedTexImage")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetCompressedTexImage\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetCompressedTexImage
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord2d: unsafe {
                unsafe extern "system" fn __glMultiTexCoord2d(
                    _target: TextureUnit,
                    _s: GLdouble,
                    _t: GLdouble,
                ) {
                    panic!("Unable to load glMultiTexCoord2d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord2d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord2d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord2iv: unsafe {
                unsafe extern "system" fn __glMultiTexCoord2iv(
                    _target: TextureUnit,
                    _v: *mut GLint,
                ) {
                    panic!("Unable to load glMultiTexCoord2iv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord2iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord2iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord4s: unsafe {
                unsafe extern "system" fn __glMultiTexCoord4s(
                    _target: TextureUnit,
                    _s: GLshort,
                    _t: GLshort,
                    _r: GLshort,
                    _q: GLshort,
                ) {
                    panic!("Unable to load glMultiTexCoord4s")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord4s\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord4s
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord1i: unsafe {
                unsafe extern "system" fn __glMultiTexCoord1i(_target: TextureUnit, _s: GLint) {
                    panic!("Unable to load glMultiTexCoord1i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord1i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord1i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord2f: unsafe {
                unsafe extern "system" fn __glMultiTexCoord2f(
                    _target: TextureUnit,
                    _s: GLfloat,
                    _t: GLfloat,
                ) {
                    panic!("Unable to load glMultiTexCoord2f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord2f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord2f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord3f: unsafe {
                unsafe extern "system" fn __glMultiTexCoord3f(
                    _target: TextureUnit,
                    _s: GLfloat,
                    _t: GLfloat,
                    _r: GLfloat,
                ) {
                    panic!("Unable to load glMultiTexCoord3f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord3f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord3f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord3sv: unsafe {
                unsafe extern "system" fn __glMultiTexCoord3sv(
                    _target: TextureUnit,
                    _v: *mut GLshort,
                ) {
                    panic!("Unable to load glMultiTexCoord3sv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord3sv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord3sv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord3fv: unsafe {
                unsafe extern "system" fn __glMultiTexCoord3fv(
                    _target: TextureUnit,
                    _v: *mut GLfloat,
                ) {
                    panic!("Unable to load glMultiTexCoord3fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord3fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord3fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord1s: unsafe {
                unsafe extern "system" fn __glMultiTexCoord1s(_target: TextureUnit, _s: GLshort) {
                    panic!("Unable to load glMultiTexCoord1s")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord1s\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord1s
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCompressedTexImage1D: unsafe {
                unsafe extern "system" fn __glCompressedTexImage1D(
                    _target: TextureTarget,
                    _level: GLint,
                    _internalformat: InternalFormat,
                    _width: GLsizei,
                    _border: GLint,
                    _imageSize: GLsizei,
                    _data: *mut GLvoid,
                ) {
                    panic!("Unable to load glCompressedTexImage1D")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCompressedTexImage1D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCompressedTexImage1D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord4f: unsafe {
                unsafe extern "system" fn __glMultiTexCoord4f(
                    _target: TextureUnit,
                    _s: GLfloat,
                    _t: GLfloat,
                    _r: GLfloat,
                    _q: GLfloat,
                ) {
                    panic!("Unable to load glMultiTexCoord4f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord4f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord4f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCompressedTexSubImage3D: unsafe {
                unsafe extern "system" fn __glCompressedTexSubImage3D(
                    _target: TextureTarget,
                    _level: GLint,
                    _xoffset: GLint,
                    _yoffset: GLint,
                    _zoffset: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                    _depth: GLsizei,
                    _format: InternalFormat,
                    _imageSize: GLsizei,
                    _data: *mut GLvoid,
                ) {
                    panic!("Unable to load glCompressedTexSubImage3D")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCompressedTexSubImage3D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCompressedTexSubImage3D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord1iv: unsafe {
                unsafe extern "system" fn __glMultiTexCoord1iv(
                    _target: TextureUnit,
                    _v: *mut GLint,
                ) {
                    panic!("Unable to load glMultiTexCoord1iv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord1iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord1iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord2s: unsafe {
                unsafe extern "system" fn __glMultiTexCoord2s(
                    _target: TextureUnit,
                    _s: GLshort,
                    _t: GLshort,
                ) {
                    panic!("Unable to load glMultiTexCoord2s")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord2s\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord2s
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glActiveTexture: unsafe {
                unsafe extern "system" fn __glActiveTexture(_texture: TextureUnit) {
                    panic!("Unable to load glActiveTexture")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glActiveTexture\0");
                let val = _f(cname);
                if val.is_null() {
                    __glActiveTexture
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord1dv: unsafe {
                unsafe extern "system" fn __glMultiTexCoord1dv(
                    _target: TextureUnit,
                    _v: *mut GLdouble,
                ) {
                    panic!("Unable to load glMultiTexCoord1dv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord1dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord1dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord4i: unsafe {
                unsafe extern "system" fn __glMultiTexCoord4i(
                    _target: TextureUnit,
                    _s: GLint,
                    _t: GLint,
                    _r: GLint,
                    _q: GLint,
                ) {
                    panic!("Unable to load glMultiTexCoord4i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord4i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord4i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiTexCoord4iv: unsafe {
                unsafe extern "system" fn __glMultiTexCoord4iv(
                    _target: TextureUnit,
                    _v: *mut GLint,
                ) {
                    panic!("Unable to load glMultiTexCoord4iv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord4iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord4iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[derive(Clone)]
pub struct EntryFnGL32 {
    pub glDeleteSync: crate::gl::command::PFN_glDeleteSync,
    pub glClientWaitSync: crate::gl::command::PFN_glClientWaitSync,
    pub glTexImage3DMultisample: crate::gl::command::PFN_glTexImage3DMultisample,
    pub glFenceSync: crate::gl::command::PFN_glFenceSync,
    pub glDrawElementsInstancedBaseVertex:
        crate::gl::command::PFN_glDrawElementsInstancedBaseVertex,
    pub glGetBufferParameteri64v: crate::gl::command::PFN_glGetBufferParameteri64v,
    pub glMultiDrawElementsBaseVertex: crate::gl::command::PFN_glMultiDrawElementsBaseVertex,
    pub glGetInteger64v: crate::gl::command::PFN_glGetInteger64v,
    pub glSampleMaski: crate::gl::command::PFN_glSampleMaski,
    pub glDrawRangeElementsBaseVertex: crate::gl::command::PFN_glDrawRangeElementsBaseVertex,
    pub glGetInteger64i_v: crate::gl::command::PFN_glGetInteger64i_v,
    pub glTexImage2DMultisample: crate::gl::command::PFN_glTexImage2DMultisample,
    pub glDrawElementsBaseVertex: crate::gl::command::PFN_glDrawElementsBaseVertex,
    pub glGetMultisamplefv: crate::gl::command::PFN_glGetMultisamplefv,
    pub glFramebufferTexture: crate::gl::command::PFN_glFramebufferTexture,
    pub glProvokingVertex: crate::gl::command::PFN_glProvokingVertex,
    pub glIsSync: crate::gl::command::PFN_glIsSync,
    pub glGetSynciv: crate::gl::command::PFN_glGetSynciv,
    pub glWaitSync: crate::gl::command::PFN_glWaitSync,
}
impl EntryFnGL32 {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            glDeleteSync: unsafe {
                unsafe extern "system" fn __glDeleteSync(_sync: GLsync) {
                    panic!("Unable to load glDeleteSync")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDeleteSync\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDeleteSync
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glClientWaitSync: unsafe {
                unsafe extern "system" fn __glClientWaitSync(
                    _sync: GLsync,
                    _flags: SyncObjectMask,
                    _timeout: GLuint64,
                ) -> GLenum {
                    panic!("Unable to load glClientWaitSync")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClientWaitSync\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClientWaitSync
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexImage3DMultisample: unsafe {
                unsafe extern "system" fn __glTexImage3DMultisample(
                    _target: TextureTarget,
                    _samples: GLsizei,
                    _internalformat: InternalFormat,
                    _width: GLsizei,
                    _height: GLsizei,
                    _depth: GLsizei,
                    _fixedsamplelocations: Boolean,
                ) {
                    panic!("Unable to load glTexImage3DMultisample")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexImage3DMultisample\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexImage3DMultisample
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glFenceSync: unsafe {
                unsafe extern "system" fn __glFenceSync(
                    _condition: SyncCondition,
                    _flags: SyncBehaviorFlags,
                ) -> GLsync {
                    panic!("Unable to load glFenceSync")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFenceSync\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFenceSync
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDrawElementsInstancedBaseVertex: unsafe {
                unsafe extern "system" fn __glDrawElementsInstancedBaseVertex(
                    _mode: PrimitiveType,
                    _count: GLsizei,
                    _type: DrawElementsType,
                    _indices: *mut GLvoid,
                    _instancecount: GLsizei,
                    _basevertex: GLint,
                ) {
                    panic!("Unable to load glDrawElementsInstancedBaseVertex")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glDrawElementsInstancedBaseVertex\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glDrawElementsInstancedBaseVertex
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetBufferParameteri64v: unsafe {
                unsafe extern "system" fn __glGetBufferParameteri64v(
                    _target: BufferTargetARB,
                    _pname: BufferPNameARB,
                    _params: *mut GLint64,
                ) {
                    panic!("Unable to load glGetBufferParameteri64v")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetBufferParameteri64v\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetBufferParameteri64v
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiDrawElementsBaseVertex: unsafe {
                unsafe extern "system" fn __glMultiDrawElementsBaseVertex(
                    _mode: PrimitiveType,
                    _count: *mut GLsizei,
                    _type: DrawElementsType,
                    _indices: *mut GLvoid,
                    _drawcount: GLsizei,
                    _basevertex: *mut GLint,
                ) {
                    panic!("Unable to load glMultiDrawElementsBaseVertex")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glMultiDrawElementsBaseVertex\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glMultiDrawElementsBaseVertex
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetInteger64v: unsafe {
                unsafe extern "system" fn __glGetInteger64v(_pname: GetPName, _data: *mut GLint64) {
                    panic!("Unable to load glGetInteger64v")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetInteger64v\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetInteger64v
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glSampleMaski: unsafe {
                unsafe extern "system" fn __glSampleMaski(_maskNumber: GLuint, _mask: GLbitfield) {
                    panic!("Unable to load glSampleMaski")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSampleMaski\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSampleMaski
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDrawRangeElementsBaseVertex: unsafe {
                unsafe extern "system" fn __glDrawRangeElementsBaseVertex(
                    _mode: PrimitiveType,
                    _start: GLuint,
                    _end: GLuint,
                    _count: GLsizei,
                    _type: DrawElementsType,
                    _indices: *mut GLvoid,
                    _basevertex: GLint,
                ) {
                    panic!("Unable to load glDrawRangeElementsBaseVertex")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glDrawRangeElementsBaseVertex\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glDrawRangeElementsBaseVertex
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetInteger64i_v: unsafe {
                unsafe extern "system" fn __glGetInteger64i_v(
                    _target: GetPName,
                    _index: GLuint,
                    _data: *mut GLint64,
                ) {
                    panic!("Unable to load glGetInteger64i_v")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetInteger64i_v\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetInteger64i_v
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexImage2DMultisample: unsafe {
                unsafe extern "system" fn __glTexImage2DMultisample(
                    _target: TextureTarget,
                    _samples: GLsizei,
                    _internalformat: InternalFormat,
                    _width: GLsizei,
                    _height: GLsizei,
                    _fixedsamplelocations: Boolean,
                ) {
                    panic!("Unable to load glTexImage2DMultisample")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexImage2DMultisample\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexImage2DMultisample
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDrawElementsBaseVertex: unsafe {
                unsafe extern "system" fn __glDrawElementsBaseVertex(
                    _mode: PrimitiveType,
                    _count: GLsizei,
                    _type: DrawElementsType,
                    _indices: *mut GLvoid,
                    _basevertex: GLint,
                ) {
                    panic!("Unable to load glDrawElementsBaseVertex")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDrawElementsBaseVertex\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDrawElementsBaseVertex
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetMultisamplefv: unsafe {
                unsafe extern "system" fn __glGetMultisamplefv(
                    _pname: GetMultisamplePNameNV,
                    _index: GLuint,
                    _val: *mut GLfloat,
                ) {
                    panic!("Unable to load glGetMultisamplefv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetMultisamplefv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetMultisamplefv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glFramebufferTexture: unsafe {
                unsafe extern "system" fn __glFramebufferTexture(
                    _target: FramebufferTarget,
                    _attachment: FramebufferAttachment,
                    _texture: GLuint,
                    _level: GLint,
                ) {
                    panic!("Unable to load glFramebufferTexture")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFramebufferTexture\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFramebufferTexture
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glProvokingVertex: unsafe {
                unsafe extern "system" fn __glProvokingVertex(_mode: VertexProvokingMode) {
                    panic!("Unable to load glProvokingVertex")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProvokingVertex\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProvokingVertex
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glIsSync: unsafe {
                unsafe extern "system" fn __glIsSync(_sync: GLsync) -> GLboolean {
                    panic!("Unable to load glIsSync")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIsSync\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIsSync
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetSynciv: unsafe {
                unsafe extern "system" fn __glGetSynciv(
                    _sync: GLsync,
                    _pname: SyncParameterName,
                    _count: GLsizei,
                    _length: *mut GLsizei,
                    _values: *mut GLint,
                ) {
                    panic!("Unable to load glGetSynciv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetSynciv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetSynciv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glWaitSync: unsafe {
                unsafe extern "system" fn __glWaitSync(
                    _sync: GLsync,
                    _flags: SyncBehaviorFlags,
                    _timeout: GLuint64,
                ) {
                    panic!("Unable to load glWaitSync")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glWaitSync\0");
                let val = _f(cname);
                if val.is_null() {
                    __glWaitSync
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[derive(Clone)]
pub struct EntryFnGL11 {
    pub glDeleteTextures: crate::gl::command::PFN_glDeleteTextures,
    pub glVertexPointer: crate::gl::command::PFN_glVertexPointer,
    pub glIndexubv: crate::gl::command::PFN_glIndexubv,
    pub glCopyTexSubImage1D: crate::gl::command::PFN_glCopyTexSubImage1D,
    pub glArrayElement: crate::gl::command::PFN_glArrayElement,
    pub glGetPointerv: crate::gl::command::PFN_glGetPointerv,
    pub glCopyTexSubImage2D: crate::gl::command::PFN_glCopyTexSubImage2D,
    pub glPushClientAttrib: crate::gl::command::PFN_glPushClientAttrib,
    pub glDrawArrays: crate::gl::command::PFN_glDrawArrays,
    pub glColorPointer: crate::gl::command::PFN_glColorPointer,
    pub glInterleavedArrays: crate::gl::command::PFN_glInterleavedArrays,
    pub glIndexub: crate::gl::command::PFN_glIndexub,
    pub glTexSubImage2D: crate::gl::command::PFN_glTexSubImage2D,
    pub glPrioritizeTextures: crate::gl::command::PFN_glPrioritizeTextures,
    pub glIsTexture: crate::gl::command::PFN_glIsTexture,
    pub glCopyTexImage2D: crate::gl::command::PFN_glCopyTexImage2D,
    pub glBindTexture: crate::gl::command::PFN_glBindTexture,
    pub glGenTextures: crate::gl::command::PFN_glGenTextures,
    pub glAreTexturesResident: crate::gl::command::PFN_glAreTexturesResident,
    pub glDrawElements: crate::gl::command::PFN_glDrawElements,
    pub glCopyTexImage1D: crate::gl::command::PFN_glCopyTexImage1D,
    pub glTexSubImage1D: crate::gl::command::PFN_glTexSubImage1D,
    pub glTexCoordPointer: crate::gl::command::PFN_glTexCoordPointer,
    pub glIndexPointer: crate::gl::command::PFN_glIndexPointer,
    pub glEdgeFlagPointer: crate::gl::command::PFN_glEdgeFlagPointer,
    pub glNormalPointer: crate::gl::command::PFN_glNormalPointer,
    pub glEnableClientState: crate::gl::command::PFN_glEnableClientState,
    pub glPopClientAttrib: crate::gl::command::PFN_glPopClientAttrib,
    pub glPolygonOffset: crate::gl::command::PFN_glPolygonOffset,
    pub glDisableClientState: crate::gl::command::PFN_glDisableClientState,
}
impl EntryFnGL11 {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            glDeleteTextures: unsafe {
                unsafe extern "system" fn __glDeleteTextures(_n: GLsizei, _textures: *mut GLuint) {
                    panic!("Unable to load glDeleteTextures")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDeleteTextures\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDeleteTextures
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertexPointer: unsafe {
                unsafe extern "system" fn __glVertexPointer(
                    _size: GLint,
                    _type: VertexPointerType,
                    _stride: GLsizei,
                    _pointer: *mut GLvoid,
                ) {
                    panic!("Unable to load glVertexPointer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexPointer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexPointer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glIndexubv: unsafe {
                unsafe extern "system" fn __glIndexubv(_c: *mut GLubyte) {
                    panic!("Unable to load glIndexubv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIndexubv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIndexubv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCopyTexSubImage1D: unsafe {
                unsafe extern "system" fn __glCopyTexSubImage1D(
                    _target: TextureTarget,
                    _level: GLint,
                    _xoffset: GLint,
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                ) {
                    panic!("Unable to load glCopyTexSubImage1D")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCopyTexSubImage1D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCopyTexSubImage1D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glArrayElement: unsafe {
                unsafe extern "system" fn __glArrayElement(_i: GLint) {
                    panic!("Unable to load glArrayElement")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glArrayElement\0");
                let val = _f(cname);
                if val.is_null() {
                    __glArrayElement
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetPointerv: unsafe {
                unsafe extern "system" fn __glGetPointerv(
                    _pname: GetPointervPName,
                    _params: *mut GLvoid,
                ) {
                    panic!("Unable to load glGetPointerv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetPointerv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetPointerv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCopyTexSubImage2D: unsafe {
                unsafe extern "system" fn __glCopyTexSubImage2D(
                    _target: TextureTarget,
                    _level: GLint,
                    _xoffset: GLint,
                    _yoffset: GLint,
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                ) {
                    panic!("Unable to load glCopyTexSubImage2D")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCopyTexSubImage2D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCopyTexSubImage2D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPushClientAttrib: unsafe {
                unsafe extern "system" fn __glPushClientAttrib(_mask: ClientAttribMask) {
                    panic!("Unable to load glPushClientAttrib")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPushClientAttrib\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPushClientAttrib
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDrawArrays: unsafe {
                unsafe extern "system" fn __glDrawArrays(
                    _mode: PrimitiveType,
                    _first: GLint,
                    _count: GLsizei,
                ) {
                    panic!("Unable to load glDrawArrays")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDrawArrays\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDrawArrays
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColorPointer: unsafe {
                unsafe extern "system" fn __glColorPointer(
                    _size: GLint,
                    _type: ColorPointerType,
                    _stride: GLsizei,
                    _pointer: *mut GLvoid,
                ) {
                    panic!("Unable to load glColorPointer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColorPointer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColorPointer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glInterleavedArrays: unsafe {
                unsafe extern "system" fn __glInterleavedArrays(
                    _format: InterleavedArrayFormat,
                    _stride: GLsizei,
                    _pointer: *mut GLvoid,
                ) {
                    panic!("Unable to load glInterleavedArrays")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glInterleavedArrays\0");
                let val = _f(cname);
                if val.is_null() {
                    __glInterleavedArrays
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glIndexub: unsafe {
                unsafe extern "system" fn __glIndexub(_c: GLubyte) {
                    panic!("Unable to load glIndexub")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIndexub\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIndexub
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexSubImage2D: unsafe {
                unsafe extern "system" fn __glTexSubImage2D(
                    _target: TextureTarget,
                    _level: GLint,
                    _xoffset: GLint,
                    _yoffset: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                    _format: PixelFormat,
                    _type: PixelType,
                    _pixels: *mut GLvoid,
                ) {
                    panic!("Unable to load glTexSubImage2D")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexSubImage2D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexSubImage2D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPrioritizeTextures: unsafe {
                unsafe extern "system" fn __glPrioritizeTextures(
                    _n: GLsizei,
                    _textures: *mut GLuint,
                    _priorities: *mut GLfloat,
                ) {
                    panic!("Unable to load glPrioritizeTextures")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPrioritizeTextures\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPrioritizeTextures
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glIsTexture: unsafe {
                unsafe extern "system" fn __glIsTexture(_texture: GLuint) -> GLboolean {
                    panic!("Unable to load glIsTexture")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIsTexture\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIsTexture
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCopyTexImage2D: unsafe {
                unsafe extern "system" fn __glCopyTexImage2D(
                    _target: TextureTarget,
                    _level: GLint,
                    _internalformat: InternalFormat,
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                    _border: GLint,
                ) {
                    panic!("Unable to load glCopyTexImage2D")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCopyTexImage2D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCopyTexImage2D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBindTexture: unsafe {
                unsafe extern "system" fn __glBindTexture(
                    _target: TextureTarget,
                    _texture: GLuint,
                ) {
                    panic!("Unable to load glBindTexture")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindTexture\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindTexture
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGenTextures: unsafe {
                unsafe extern "system" fn __glGenTextures(_n: GLsizei, _textures: *mut GLuint) {
                    panic!("Unable to load glGenTextures")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGenTextures\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGenTextures
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glAreTexturesResident: unsafe {
                unsafe extern "system" fn __glAreTexturesResident(
                    _n: GLsizei,
                    _textures: *mut GLuint,
                    _residences: *mut Boolean,
                ) -> GLboolean {
                    panic!("Unable to load glAreTexturesResident")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glAreTexturesResident\0");
                let val = _f(cname);
                if val.is_null() {
                    __glAreTexturesResident
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDrawElements: unsafe {
                unsafe extern "system" fn __glDrawElements(
                    _mode: PrimitiveType,
                    _count: GLsizei,
                    _type: DrawElementsType,
                    _indices: *mut GLvoid,
                ) {
                    panic!("Unable to load glDrawElements")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDrawElements\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDrawElements
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCopyTexImage1D: unsafe {
                unsafe extern "system" fn __glCopyTexImage1D(
                    _target: TextureTarget,
                    _level: GLint,
                    _internalformat: InternalFormat,
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                    _border: GLint,
                ) {
                    panic!("Unable to load glCopyTexImage1D")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCopyTexImage1D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCopyTexImage1D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexSubImage1D: unsafe {
                unsafe extern "system" fn __glTexSubImage1D(
                    _target: TextureTarget,
                    _level: GLint,
                    _xoffset: GLint,
                    _width: GLsizei,
                    _format: PixelFormat,
                    _type: PixelType,
                    _pixels: *mut GLvoid,
                ) {
                    panic!("Unable to load glTexSubImage1D")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexSubImage1D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexSubImage1D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoordPointer: unsafe {
                unsafe extern "system" fn __glTexCoordPointer(
                    _size: GLint,
                    _type: TexCoordPointerType,
                    _stride: GLsizei,
                    _pointer: *mut GLvoid,
                ) {
                    panic!("Unable to load glTexCoordPointer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoordPointer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoordPointer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glIndexPointer: unsafe {
                unsafe extern "system" fn __glIndexPointer(
                    _type: IndexPointerType,
                    _stride: GLsizei,
                    _pointer: *mut GLvoid,
                ) {
                    panic!("Unable to load glIndexPointer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIndexPointer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIndexPointer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glEdgeFlagPointer: unsafe {
                unsafe extern "system" fn __glEdgeFlagPointer(
                    _stride: GLsizei,
                    _pointer: *mut GLvoid,
                ) {
                    panic!("Unable to load glEdgeFlagPointer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEdgeFlagPointer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEdgeFlagPointer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glNormalPointer: unsafe {
                unsafe extern "system" fn __glNormalPointer(
                    _type: NormalPointerType,
                    _stride: GLsizei,
                    _pointer: *mut GLvoid,
                ) {
                    panic!("Unable to load glNormalPointer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glNormalPointer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glNormalPointer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glEnableClientState: unsafe {
                unsafe extern "system" fn __glEnableClientState(_array: EnableCap) {
                    panic!("Unable to load glEnableClientState")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEnableClientState\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEnableClientState
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPopClientAttrib: unsafe {
                unsafe extern "system" fn __glPopClientAttrib() {
                    panic!("Unable to load glPopClientAttrib")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPopClientAttrib\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPopClientAttrib
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPolygonOffset: unsafe {
                unsafe extern "system" fn __glPolygonOffset(_factor: GLfloat, _units: GLfloat) {
                    panic!("Unable to load glPolygonOffset")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPolygonOffset\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPolygonOffset
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDisableClientState: unsafe {
                unsafe extern "system" fn __glDisableClientState(_array: EnableCap) {
                    panic!("Unable to load glDisableClientState")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDisableClientState\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDisableClientState
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[derive(Clone)]
pub struct EntryFnGL31 {
    pub glGetActiveUniformBlockiv: crate::gl::command::PFN_glGetActiveUniformBlockiv,
    pub glDrawArraysInstanced: crate::gl::command::PFN_glDrawArraysInstanced,
    pub glGetIntegeri_v: crate::gl::command::PFN_glGetIntegeri_v,
    pub glBindBufferRange: crate::gl::command::PFN_glBindBufferRange,
    pub glUniformBlockBinding: crate::gl::command::PFN_glUniformBlockBinding,
    pub glGetActiveUniformsiv: crate::gl::command::PFN_glGetActiveUniformsiv,
    pub glBindBufferBase: crate::gl::command::PFN_glBindBufferBase,
    pub glGetActiveUniformBlockName: crate::gl::command::PFN_glGetActiveUniformBlockName,
    pub glCopyBufferSubData: crate::gl::command::PFN_glCopyBufferSubData,
    pub glPrimitiveRestartIndex: crate::gl::command::PFN_glPrimitiveRestartIndex,
    pub glGetActiveUniformName: crate::gl::command::PFN_glGetActiveUniformName,
    pub glGetUniformIndices: crate::gl::command::PFN_glGetUniformIndices,
    pub glDrawElementsInstanced: crate::gl::command::PFN_glDrawElementsInstanced,
    pub glGetUniformBlockIndex: crate::gl::command::PFN_glGetUniformBlockIndex,
    pub glTexBuffer: crate::gl::command::PFN_glTexBuffer,
}
impl EntryFnGL31 {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            glGetActiveUniformBlockiv: unsafe {
                unsafe extern "system" fn __glGetActiveUniformBlockiv(
                    _program: GLuint,
                    _uniformBlockIndex: GLuint,
                    _pname: UniformBlockPName,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetActiveUniformBlockiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetActiveUniformBlockiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetActiveUniformBlockiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDrawArraysInstanced: unsafe {
                unsafe extern "system" fn __glDrawArraysInstanced(
                    _mode: PrimitiveType,
                    _first: GLint,
                    _count: GLsizei,
                    _instancecount: GLsizei,
                ) {
                    panic!("Unable to load glDrawArraysInstanced")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDrawArraysInstanced\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDrawArraysInstanced
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetIntegeri_v: unsafe {
                unsafe extern "system" fn __glGetIntegeri_v(
                    _target: GetPName,
                    _index: GLuint,
                    _data: *mut GLint,
                ) {
                    panic!("Unable to load glGetIntegeri_v")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetIntegeri_v\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetIntegeri_v
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBindBufferRange: unsafe {
                unsafe extern "system" fn __glBindBufferRange(
                    _target: BufferTargetARB,
                    _index: GLuint,
                    _buffer: GLuint,
                    _offset: GLintptr,
                    _size: GLsizeiptr,
                ) {
                    panic!("Unable to load glBindBufferRange")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindBufferRange\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindBufferRange
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniformBlockBinding: unsafe {
                unsafe extern "system" fn __glUniformBlockBinding(
                    _program: GLuint,
                    _uniformBlockIndex: GLuint,
                    _uniformBlockBinding: GLuint,
                ) {
                    panic!("Unable to load glUniformBlockBinding")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformBlockBinding\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformBlockBinding
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetActiveUniformsiv: unsafe {
                unsafe extern "system" fn __glGetActiveUniformsiv(
                    _program: GLuint,
                    _uniformCount: GLsizei,
                    _uniformIndices: *mut GLuint,
                    _pname: UniformPName,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetActiveUniformsiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetActiveUniformsiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetActiveUniformsiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBindBufferBase: unsafe {
                unsafe extern "system" fn __glBindBufferBase(
                    _target: BufferTargetARB,
                    _index: GLuint,
                    _buffer: GLuint,
                ) {
                    panic!("Unable to load glBindBufferBase")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindBufferBase\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindBufferBase
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetActiveUniformBlockName: unsafe {
                unsafe extern "system" fn __glGetActiveUniformBlockName(
                    _program: GLuint,
                    _uniformBlockIndex: GLuint,
                    _bufSize: GLsizei,
                    _length: *mut GLsizei,
                    _uniformBlockName: *mut GLchar,
                ) {
                    panic!("Unable to load glGetActiveUniformBlockName")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetActiveUniformBlockName\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetActiveUniformBlockName
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCopyBufferSubData: unsafe {
                unsafe extern "system" fn __glCopyBufferSubData(
                    _readTarget: CopyBufferSubDataTarget,
                    _writeTarget: CopyBufferSubDataTarget,
                    _readOffset: GLintptr,
                    _writeOffset: GLintptr,
                    _size: GLsizeiptr,
                ) {
                    panic!("Unable to load glCopyBufferSubData")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCopyBufferSubData\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCopyBufferSubData
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPrimitiveRestartIndex: unsafe {
                unsafe extern "system" fn __glPrimitiveRestartIndex(_index: GLuint) {
                    panic!("Unable to load glPrimitiveRestartIndex")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPrimitiveRestartIndex\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPrimitiveRestartIndex
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetActiveUniformName: unsafe {
                unsafe extern "system" fn __glGetActiveUniformName(
                    _program: GLuint,
                    _uniformIndex: GLuint,
                    _bufSize: GLsizei,
                    _length: *mut GLsizei,
                    _uniformName: *mut GLchar,
                ) {
                    panic!("Unable to load glGetActiveUniformName")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetActiveUniformName\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetActiveUniformName
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetUniformIndices: unsafe {
                unsafe extern "system" fn __glGetUniformIndices(
                    _program: GLuint,
                    _uniformCount: GLsizei,
                    _uniformNames: *mut GLchar,
                    _uniformIndices: *mut GLuint,
                ) {
                    panic!("Unable to load glGetUniformIndices")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetUniformIndices\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetUniformIndices
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDrawElementsInstanced: unsafe {
                unsafe extern "system" fn __glDrawElementsInstanced(
                    _mode: PrimitiveType,
                    _count: GLsizei,
                    _type: DrawElementsType,
                    _indices: *mut GLvoid,
                    _instancecount: GLsizei,
                ) {
                    panic!("Unable to load glDrawElementsInstanced")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDrawElementsInstanced\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDrawElementsInstanced
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetUniformBlockIndex: unsafe {
                unsafe extern "system" fn __glGetUniformBlockIndex(
                    _program: GLuint,
                    _uniformBlockName: *mut GLchar,
                ) -> GLuint {
                    panic!("Unable to load glGetUniformBlockIndex")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetUniformBlockIndex\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetUniformBlockIndex
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexBuffer: unsafe {
                unsafe extern "system" fn __glTexBuffer(
                    _target: TextureTarget,
                    _internalformat: SizedInternalFormat,
                    _buffer: GLuint,
                ) {
                    panic!("Unable to load glTexBuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexBuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[derive(Clone)]
pub struct EntryFnGL46 {
    pub glPolygonOffsetClamp: crate::gl::command::PFN_glPolygonOffsetClamp,
    pub glSpecializeShader: crate::gl::command::PFN_glSpecializeShader,
    pub glMultiDrawArraysIndirectCount: crate::gl::command::PFN_glMultiDrawArraysIndirectCount,
    pub glMultiDrawElementsIndirectCount: crate::gl::command::PFN_glMultiDrawElementsIndirectCount,
}
impl EntryFnGL46 {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            glPolygonOffsetClamp: unsafe {
                unsafe extern "system" fn __glPolygonOffsetClamp(
                    _factor: GLfloat,
                    _units: GLfloat,
                    _clamp: GLfloat,
                ) {
                    panic!("Unable to load glPolygonOffsetClamp")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPolygonOffsetClamp\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPolygonOffsetClamp
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glSpecializeShader: unsafe {
                unsafe extern "system" fn __glSpecializeShader(
                    _shader: GLuint,
                    _pEntryPoint: GLchar,
                    _numSpecializationConstants: GLuint,
                    _pConstantIndex: GLuint,
                    _pConstantValue: GLuint,
                ) {
                    panic!("Unable to load glSpecializeShader")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSpecializeShader\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSpecializeShader
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiDrawArraysIndirectCount: unsafe {
                unsafe extern "system" fn __glMultiDrawArraysIndirectCount(
                    _mode: PrimitiveType,
                    _indirect: GLvoid,
                    _drawcount: GLintptr,
                    _maxdrawcount: GLsizei,
                    _stride: GLsizei,
                ) {
                    panic!("Unable to load glMultiDrawArraysIndirectCount")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glMultiDrawArraysIndirectCount\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glMultiDrawArraysIndirectCount
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultiDrawElementsIndirectCount: unsafe {
                unsafe extern "system" fn __glMultiDrawElementsIndirectCount(
                    _mode: PrimitiveType,
                    _type: DrawElementsType,
                    _indirect: GLvoid,
                    _drawcount: GLintptr,
                    _maxdrawcount: GLsizei,
                    _stride: GLsizei,
                ) {
                    panic!("Unable to load glMultiDrawElementsIndirectCount")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glMultiDrawElementsIndirectCount\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glMultiDrawElementsIndirectCount
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[derive(Clone)]
pub struct EntryFnGL10 {
    pub glRasterPos3s: crate::gl::command::PFN_glRasterPos3s,
    pub glRasterPos3d: crate::gl::command::PFN_glRasterPos3d,
    pub glClipPlane: crate::gl::command::PFN_glClipPlane,
    pub glEndList: crate::gl::command::PFN_glEndList,
    pub glDeleteLists: crate::gl::command::PFN_glDeleteLists,
    pub glEvalMesh1: crate::gl::command::PFN_glEvalMesh1,
    pub glEvalCoord2f: crate::gl::command::PFN_glEvalCoord2f,
    pub glTranslated: crate::gl::command::PFN_glTranslated,
    pub glRasterPos2f: crate::gl::command::PFN_glRasterPos2f,
    pub glTexCoord2iv: crate::gl::command::PFN_glTexCoord2iv,
    pub glColor4iv: crate::gl::command::PFN_glColor4iv,
    pub glRectfv: crate::gl::command::PFN_glRectfv,
    pub glColor4dv: crate::gl::command::PFN_glColor4dv,
    pub glRasterPos3sv: crate::gl::command::PFN_glRasterPos3sv,
    pub glTexGeniv: crate::gl::command::PFN_glTexGeniv,
    pub glColor3us: crate::gl::command::PFN_glColor3us,
    pub glGetMapiv: crate::gl::command::PFN_glGetMapiv,
    pub glFogfv: crate::gl::command::PFN_glFogfv,
    pub glColor3d: crate::gl::command::PFN_glColor3d,
    pub glPixelStorei: crate::gl::command::PFN_glPixelStorei,
    pub glColor3fv: crate::gl::command::PFN_glColor3fv,
    pub glPixelTransferi: crate::gl::command::PFN_glPixelTransferi,
    pub glRasterPos2dv: crate::gl::command::PFN_glRasterPos2dv,
    pub glTexCoord2dv: crate::gl::command::PFN_glTexCoord2dv,
    pub glTranslatef: crate::gl::command::PFN_glTranslatef,
    pub glColor4us: crate::gl::command::PFN_glColor4us,
    pub glRasterPos2fv: crate::gl::command::PFN_glRasterPos2fv,
    pub glCallList: crate::gl::command::PFN_glCallList,
    pub glVertex4fv: crate::gl::command::PFN_glVertex4fv,
    pub glMap2d: crate::gl::command::PFN_glMap2d,
    pub glTexCoord1d: crate::gl::command::PFN_glTexCoord1d,
    pub glStencilFunc: crate::gl::command::PFN_glStencilFunc,
    pub glTexImage1D: crate::gl::command::PFN_glTexImage1D,
    pub glMapGrid2f: crate::gl::command::PFN_glMapGrid2f,
    pub glColor3dv: crate::gl::command::PFN_glColor3dv,
    pub glPushAttrib: crate::gl::command::PFN_glPushAttrib,
    pub glCopyPixels: crate::gl::command::PFN_glCopyPixels,
    pub glFrustum: crate::gl::command::PFN_glFrustum,
    pub glTexCoord3sv: crate::gl::command::PFN_glTexCoord3sv,
    pub glGetPixelMapfv: crate::gl::command::PFN_glGetPixelMapfv,
    pub glFogi: crate::gl::command::PFN_glFogi,
    pub glLightfv: crate::gl::command::PFN_glLightfv,
    pub glLoadMatrixd: crate::gl::command::PFN_glLoadMatrixd,
    pub glIndexf: crate::gl::command::PFN_glIndexf,
    pub glInitNames: crate::gl::command::PFN_glInitNames,
    pub glPixelMapuiv: crate::gl::command::PFN_glPixelMapuiv,
    pub glColor4uiv: crate::gl::command::PFN_glColor4uiv,
    pub glEvalPoint1: crate::gl::command::PFN_glEvalPoint1,
    pub glGetPixelMapuiv: crate::gl::command::PFN_glGetPixelMapuiv,
    pub glViewport: crate::gl::command::PFN_glViewport,
    pub glEvalCoord2d: crate::gl::command::PFN_glEvalCoord2d,
    pub glLightf: crate::gl::command::PFN_glLightf,
    pub glColor3i: crate::gl::command::PFN_glColor3i,
    pub glRasterPos3f: crate::gl::command::PFN_glRasterPos3f,
    pub glAlphaFunc: crate::gl::command::PFN_glAlphaFunc,
    pub glVertex2dv: crate::gl::command::PFN_glVertex2dv,
    pub glColor4d: crate::gl::command::PFN_glColor4d,
    pub glNormal3b: crate::gl::command::PFN_glNormal3b,
    pub glGetString: crate::gl::command::PFN_glGetString,
    pub glIsEnabled: crate::gl::command::PFN_glIsEnabled,
    pub glRectiv: crate::gl::command::PFN_glRectiv,
    pub glRasterPos2iv: crate::gl::command::PFN_glRasterPos2iv,
    pub glRasterPos4sv: crate::gl::command::PFN_glRasterPos4sv,
    pub glColor3iv: crate::gl::command::PFN_glColor3iv,
    pub glPopAttrib: crate::gl::command::PFN_glPopAttrib,
    pub glRasterPos4f: crate::gl::command::PFN_glRasterPos4f,
    pub glMap2f: crate::gl::command::PFN_glMap2f,
    pub glVertex3dv: crate::gl::command::PFN_glVertex3dv,
    pub glBlendFunc: crate::gl::command::PFN_glBlendFunc,
    pub glTexCoord2d: crate::gl::command::PFN_glTexCoord2d,
    pub glRotatef: crate::gl::command::PFN_glRotatef,
    pub glColor3bv: crate::gl::command::PFN_glColor3bv,
    pub glRectdv: crate::gl::command::PFN_glRectdv,
    pub glEdgeFlagv: crate::gl::command::PFN_glEdgeFlagv,
    pub glTexEnvf: crate::gl::command::PFN_glTexEnvf,
    pub glEdgeFlag: crate::gl::command::PFN_glEdgeFlag,
    pub glGetTexGenfv: crate::gl::command::PFN_glGetTexGenfv,
    pub glLoadIdentity: crate::gl::command::PFN_glLoadIdentity,
    pub glGetFloatv: crate::gl::command::PFN_glGetFloatv,
    pub glScalef: crate::gl::command::PFN_glScalef,
    pub glColorMask: crate::gl::command::PFN_glColorMask,
    pub glIndexdv: crate::gl::command::PFN_glIndexdv,
    pub glTexCoord3d: crate::gl::command::PFN_glTexCoord3d,
    pub glGetIntegerv: crate::gl::command::PFN_glGetIntegerv,
    pub glVertex2s: crate::gl::command::PFN_glVertex2s,
    pub glColor4usv: crate::gl::command::PFN_glColor4usv,
    pub glScissor: crate::gl::command::PFN_glScissor,
    pub glPolygonStipple: crate::gl::command::PFN_glPolygonStipple,
    pub glMapGrid1f: crate::gl::command::PFN_glMapGrid1f,
    pub glPolygonMode: crate::gl::command::PFN_glPolygonMode,
    pub glVertex3iv: crate::gl::command::PFN_glVertex3iv,
    pub glTexCoord2sv: crate::gl::command::PFN_glTexCoord2sv,
    pub glListBase: crate::gl::command::PFN_glListBase,
    pub glTexCoord4i: crate::gl::command::PFN_glTexCoord4i,
    pub glEvalCoord1fv: crate::gl::command::PFN_glEvalCoord1fv,
    pub glRasterPos4fv: crate::gl::command::PFN_glRasterPos4fv,
    pub glGetLightfv: crate::gl::command::PFN_glGetLightfv,
    pub glGetMaterialiv: crate::gl::command::PFN_glGetMaterialiv,
    pub glFinish: crate::gl::command::PFN_glFinish,
    pub glMaterialfv: crate::gl::command::PFN_glMaterialfv,
    pub glPixelMapusv: crate::gl::command::PFN_glPixelMapusv,
    pub glPixelStoref: crate::gl::command::PFN_glPixelStoref,
    pub glTexEnvi: crate::gl::command::PFN_glTexEnvi,
    pub glColor3sv: crate::gl::command::PFN_glColor3sv,
    pub glCallLists: crate::gl::command::PFN_glCallLists,
    pub glRasterPos2i: crate::gl::command::PFN_glRasterPos2i,
    pub glRasterPos2sv: crate::gl::command::PFN_glRasterPos2sv,
    pub glColor3ubv: crate::gl::command::PFN_glColor3ubv,
    pub glColor4sv: crate::gl::command::PFN_glColor4sv,
    pub glGetMaterialfv: crate::gl::command::PFN_glGetMaterialfv,
    pub glTexCoord2fv: crate::gl::command::PFN_glTexCoord2fv,
    pub glVertex4i: crate::gl::command::PFN_glVertex4i,
    pub glPushName: crate::gl::command::PFN_glPushName,
    pub glDrawPixels: crate::gl::command::PFN_glDrawPixels,
    pub glTexCoord1fv: crate::gl::command::PFN_glTexCoord1fv,
    pub glGetTexGendv: crate::gl::command::PFN_glGetTexGendv,
    pub glDepthFunc: crate::gl::command::PFN_glDepthFunc,
    pub glTexCoord1dv: crate::gl::command::PFN_glTexCoord1dv,
    pub glTexCoord1i: crate::gl::command::PFN_glTexCoord1i,
    pub glVertex4iv: crate::gl::command::PFN_glVertex4iv,
    pub glGetDoublev: crate::gl::command::PFN_glGetDoublev,
    pub glTexCoord3fv: crate::gl::command::PFN_glTexCoord3fv,
    pub glRasterPos4d: crate::gl::command::PFN_glRasterPos4d,
    pub glNormal3iv: crate::gl::command::PFN_glNormal3iv,
    pub glReadBuffer: crate::gl::command::PFN_glReadBuffer,
    pub glRasterPos2s: crate::gl::command::PFN_glRasterPos2s,
    pub glEnd: crate::gl::command::PFN_glEnd,
    pub glIndexi: crate::gl::command::PFN_glIndexi,
    pub glTexCoord1sv: crate::gl::command::PFN_glTexCoord1sv,
    pub glGenLists: crate::gl::command::PFN_glGenLists,
    pub glDepthRange: crate::gl::command::PFN_glDepthRange,
    pub glVertex3i: crate::gl::command::PFN_glVertex3i,
    pub glClearAccum: crate::gl::command::PFN_glClearAccum,
    pub glGetClipPlane: crate::gl::command::PFN_glGetClipPlane,
    pub glGetTexEnviv: crate::gl::command::PFN_glGetTexEnviv,
    pub glIsList: crate::gl::command::PFN_glIsList,
    pub glColor3usv: crate::gl::command::PFN_glColor3usv,
    pub glNormal3sv: crate::gl::command::PFN_glNormal3sv,
    pub glLightModeliv: crate::gl::command::PFN_glLightModeliv,
    pub glColor3s: crate::gl::command::PFN_glColor3s,
    pub glTexGendv: crate::gl::command::PFN_glTexGendv,
    pub glFlush: crate::gl::command::PFN_glFlush,
    pub glVertex3sv: crate::gl::command::PFN_glVertex3sv,
    pub glPopMatrix: crate::gl::command::PFN_glPopMatrix,
    pub glTexParameteri: crate::gl::command::PFN_glTexParameteri,
    pub glRasterPos4s: crate::gl::command::PFN_glRasterPos4s,
    pub glRecti: crate::gl::command::PFN_glRecti,
    pub glClearColor: crate::gl::command::PFN_glClearColor,
    pub glStencilOp: crate::gl::command::PFN_glStencilOp,
    pub glIndexiv: crate::gl::command::PFN_glIndexiv,
    pub glRenderMode: crate::gl::command::PFN_glRenderMode,
    pub glVertex2iv: crate::gl::command::PFN_glVertex2iv,
    pub glVertex4d: crate::gl::command::PFN_glVertex4d,
    pub glColorMaterial: crate::gl::command::PFN_glColorMaterial,
    pub glGetMapdv: crate::gl::command::PFN_glGetMapdv,
    pub glGetPolygonStipple: crate::gl::command::PFN_glGetPolygonStipple,
    pub glLoadName: crate::gl::command::PFN_glLoadName,
    pub glTexCoord3iv: crate::gl::command::PFN_glTexCoord3iv,
    pub glLogicOp: crate::gl::command::PFN_glLogicOp,
    pub glGetPixelMapusv: crate::gl::command::PFN_glGetPixelMapusv,
    pub glScaled: crate::gl::command::PFN_glScaled,
    pub glTexParameterf: crate::gl::command::PFN_glTexParameterf,
    pub glNormal3dv: crate::gl::command::PFN_glNormal3dv,
    pub glRasterPos3fv: crate::gl::command::PFN_glRasterPos3fv,
    pub glTexCoord4d: crate::gl::command::PFN_glTexCoord4d,
    pub glEvalCoord1f: crate::gl::command::PFN_glEvalCoord1f,
    pub glMaterialiv: crate::gl::command::PFN_glMaterialiv,
    pub glNewList: crate::gl::command::PFN_glNewList,
    pub glGetTexParameteriv: crate::gl::command::PFN_glGetTexParameteriv,
    pub glVertex3f: crate::gl::command::PFN_glVertex3f,
    pub glColor4ui: crate::gl::command::PFN_glColor4ui,
    pub glShadeModel: crate::gl::command::PFN_glShadeModel,
    pub glMap1d: crate::gl::command::PFN_glMap1d,
    pub glClearDepth: crate::gl::command::PFN_glClearDepth,
    pub glLightModelf: crate::gl::command::PFN_glLightModelf,
    pub glTexCoord2s: crate::gl::command::PFN_glTexCoord2s,
    pub glGetTexGeniv: crate::gl::command::PFN_glGetTexGeniv,
    pub glVertex2i: crate::gl::command::PFN_glVertex2i,
    pub glLoadMatrixf: crate::gl::command::PFN_glLoadMatrixf,
    pub glColor3b: crate::gl::command::PFN_glColor3b,
    pub glTexCoord1f: crate::gl::command::PFN_glTexCoord1f,
    pub glDisable: crate::gl::command::PFN_glDisable,
    pub glVertex3s: crate::gl::command::PFN_glVertex3s,
    pub glIndexs: crate::gl::command::PFN_glIndexs,
    pub glEnable: crate::gl::command::PFN_glEnable,
    pub glTexCoord4fv: crate::gl::command::PFN_glTexCoord4fv,
    pub glLightiv: crate::gl::command::PFN_glLightiv,
    pub glClearStencil: crate::gl::command::PFN_glClearStencil,
    pub glClearIndex: crate::gl::command::PFN_glClearIndex,
    pub glGetTexEnvfv: crate::gl::command::PFN_glGetTexEnvfv,
    pub glMultMatrixd: crate::gl::command::PFN_glMultMatrixd,
    pub glColor4fv: crate::gl::command::PFN_glColor4fv,
    pub glGetMapfv: crate::gl::command::PFN_glGetMapfv,
    pub glNormal3fv: crate::gl::command::PFN_glNormal3fv,
    pub glGetLightiv: crate::gl::command::PFN_glGetLightiv,
    pub glColor4bv: crate::gl::command::PFN_glColor4bv,
    pub glIndexMask: crate::gl::command::PFN_glIndexMask,
    pub glPixelZoom: crate::gl::command::PFN_glPixelZoom,
    pub glVertex4dv: crate::gl::command::PFN_glVertex4dv,
    pub glLightModelfv: crate::gl::command::PFN_glLightModelfv,
    pub glRectd: crate::gl::command::PFN_glRectd,
    pub glEvalCoord1d: crate::gl::command::PFN_glEvalCoord1d,
    pub glGetTexParameterfv: crate::gl::command::PFN_glGetTexParameterfv,
    pub glVertex3fv: crate::gl::command::PFN_glVertex3fv,
    pub glRasterPos3iv: crate::gl::command::PFN_glRasterPos3iv,
    pub glRects: crate::gl::command::PFN_glRects,
    pub glVertex2fv: crate::gl::command::PFN_glVertex2fv,
    pub glVertex4f: crate::gl::command::PFN_glVertex4f,
    pub glFogf: crate::gl::command::PFN_glFogf,
    pub glTexGenf: crate::gl::command::PFN_glTexGenf,
    pub glMapGrid2d: crate::gl::command::PFN_glMapGrid2d,
    pub glSelectBuffer: crate::gl::command::PFN_glSelectBuffer,
    pub glPushMatrix: crate::gl::command::PFN_glPushMatrix,
    pub glEvalMesh2: crate::gl::command::PFN_glEvalMesh2,
    pub glRectsv: crate::gl::command::PFN_glRectsv,
    pub glRectf: crate::gl::command::PFN_glRectf,
    pub glReadPixels: crate::gl::command::PFN_glReadPixels,
    pub glEvalPoint2: crate::gl::command::PFN_glEvalPoint2,
    pub glMultMatrixf: crate::gl::command::PFN_glMultMatrixf,
    pub glLineWidth: crate::gl::command::PFN_glLineWidth,
    pub glClear: crate::gl::command::PFN_glClear,
    pub glBitmap: crate::gl::command::PFN_glBitmap,
    pub glTexCoord4dv: crate::gl::command::PFN_glTexCoord4dv,
    pub glNormal3f: crate::gl::command::PFN_glNormal3f,
    pub glCullFace: crate::gl::command::PFN_glCullFace,
    pub glColor4f: crate::gl::command::PFN_glColor4f,
    pub glTexCoord1iv: crate::gl::command::PFN_glTexCoord1iv,
    pub glRasterPos4iv: crate::gl::command::PFN_glRasterPos4iv,
    pub glMaterialf: crate::gl::command::PFN_glMaterialf,
    pub glFeedbackBuffer: crate::gl::command::PFN_glFeedbackBuffer,
    pub glGetTexLevelParameterfv: crate::gl::command::PFN_glGetTexLevelParameterfv,
    pub glDepthMask: crate::gl::command::PFN_glDepthMask,
    pub glTexCoord1s: crate::gl::command::PFN_glTexCoord1s,
    pub glTexImage2D: crate::gl::command::PFN_glTexImage2D,
    pub glDrawBuffer: crate::gl::command::PFN_glDrawBuffer,
    pub glColor3ui: crate::gl::command::PFN_glColor3ui,
    pub glTexCoord3i: crate::gl::command::PFN_glTexCoord3i,
    pub glColor3uiv: crate::gl::command::PFN_glColor3uiv,
    pub glVertex2d: crate::gl::command::PFN_glVertex2d,
    pub glVertex4s: crate::gl::command::PFN_glVertex4s,
    pub glLighti: crate::gl::command::PFN_glLighti,
    pub glTexEnvfv: crate::gl::command::PFN_glTexEnvfv,
    pub glTexCoord3f: crate::gl::command::PFN_glTexCoord3f,
    pub glTexCoord4sv: crate::gl::command::PFN_glTexCoord4sv,
    pub glTexParameterfv: crate::gl::command::PFN_glTexParameterfv,
    pub glFogiv: crate::gl::command::PFN_glFogiv,
    pub glColor4ub: crate::gl::command::PFN_glColor4ub,
    pub glTexCoord2i: crate::gl::command::PFN_glTexCoord2i,
    pub glPixelTransferf: crate::gl::command::PFN_glPixelTransferf,
    pub glMap1f: crate::gl::command::PFN_glMap1f,
    pub glVertex4sv: crate::gl::command::PFN_glVertex4sv,
    pub glBegin: crate::gl::command::PFN_glBegin,
    pub glLightModeli: crate::gl::command::PFN_glLightModeli,
    pub glTexGend: crate::gl::command::PFN_glTexGend,
    pub glGetTexLevelParameteriv: crate::gl::command::PFN_glGetTexLevelParameteriv,
    pub glOrtho: crate::gl::command::PFN_glOrtho,
    pub glIndexfv: crate::gl::command::PFN_glIndexfv,
    pub glIndexd: crate::gl::command::PFN_glIndexd,
    pub glRotated: crate::gl::command::PFN_glRotated,
    pub glIndexsv: crate::gl::command::PFN_glIndexsv,
    pub glTexParameteriv: crate::gl::command::PFN_glTexParameteriv,
    pub glFrontFace: crate::gl::command::PFN_glFrontFace,
    pub glVertex2f: crate::gl::command::PFN_glVertex2f,
    pub glNormal3i: crate::gl::command::PFN_glNormal3i,
    pub glPassThrough: crate::gl::command::PFN_glPassThrough,
    pub glColor4ubv: crate::gl::command::PFN_glColor4ubv,
    pub glTexCoord4s: crate::gl::command::PFN_glTexCoord4s,
    pub glMapGrid1d: crate::gl::command::PFN_glMapGrid1d,
    pub glTexGenfv: crate::gl::command::PFN_glTexGenfv,
    pub glColor3f: crate::gl::command::PFN_glColor3f,
    pub glRasterPos3dv: crate::gl::command::PFN_glRasterPos3dv,
    pub glLineStipple: crate::gl::command::PFN_glLineStipple,
    pub glRasterPos3i: crate::gl::command::PFN_glRasterPos3i,
    pub glVertex3d: crate::gl::command::PFN_glVertex3d,
    pub glHint: crate::gl::command::PFN_glHint,
    pub glColor4i: crate::gl::command::PFN_glColor4i,
    pub glVertex2sv: crate::gl::command::PFN_glVertex2sv,
    pub glPointSize: crate::gl::command::PFN_glPointSize,
    pub glMateriali: crate::gl::command::PFN_glMateriali,
    pub glTexCoord2f: crate::gl::command::PFN_glTexCoord2f,
    pub glMatrixMode: crate::gl::command::PFN_glMatrixMode,
    pub glTexCoord4f: crate::gl::command::PFN_glTexCoord4f,
    pub glRasterPos4i: crate::gl::command::PFN_glRasterPos4i,
    pub glEvalCoord2fv: crate::gl::command::PFN_glEvalCoord2fv,
    pub glGetError: crate::gl::command::PFN_glGetError,
    pub glNormal3bv: crate::gl::command::PFN_glNormal3bv,
    pub glStencilMask: crate::gl::command::PFN_glStencilMask,
    pub glAccum: crate::gl::command::PFN_glAccum,
    pub glGetBooleanv: crate::gl::command::PFN_glGetBooleanv,
    pub glEvalCoord2dv: crate::gl::command::PFN_glEvalCoord2dv,
    pub glTexCoord3s: crate::gl::command::PFN_glTexCoord3s,
    pub glTexEnviv: crate::gl::command::PFN_glTexEnviv,
    pub glPopName: crate::gl::command::PFN_glPopName,
    pub glColor4s: crate::gl::command::PFN_glColor4s,
    pub glNormal3d: crate::gl::command::PFN_glNormal3d,
    pub glGetTexImage: crate::gl::command::PFN_glGetTexImage,
    pub glColor4b: crate::gl::command::PFN_glColor4b,
    pub glColor3ub: crate::gl::command::PFN_glColor3ub,
    pub glTexCoord3dv: crate::gl::command::PFN_glTexCoord3dv,
    pub glEvalCoord1dv: crate::gl::command::PFN_glEvalCoord1dv,
    pub glPixelMapfv: crate::gl::command::PFN_glPixelMapfv,
    pub glRasterPos4dv: crate::gl::command::PFN_glRasterPos4dv,
    pub glNormal3s: crate::gl::command::PFN_glNormal3s,
    pub glRasterPos2d: crate::gl::command::PFN_glRasterPos2d,
    pub glTexGeni: crate::gl::command::PFN_glTexGeni,
    pub glTexCoord4iv: crate::gl::command::PFN_glTexCoord4iv,
}
impl EntryFnGL10 {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            glRasterPos3s: unsafe {
                unsafe extern "system" fn __glRasterPos3s(_x: GLshort, _y: GLshort, _z: GLshort) {
                    panic!("Unable to load glRasterPos3s")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRasterPos3s\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRasterPos3s
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRasterPos3d: unsafe {
                unsafe extern "system" fn __glRasterPos3d(
                    _x: GLdouble,
                    _y: GLdouble,
                    _z: GLdouble,
                ) {
                    panic!("Unable to load glRasterPos3d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRasterPos3d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRasterPos3d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glClipPlane: unsafe {
                unsafe extern "system" fn __glClipPlane(
                    _plane: ClipPlaneName,
                    _equation: *mut GLdouble,
                ) {
                    panic!("Unable to load glClipPlane")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClipPlane\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClipPlane
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glEndList: unsafe {
                unsafe extern "system" fn __glEndList() {
                    panic!("Unable to load glEndList")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEndList\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEndList
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDeleteLists: unsafe {
                unsafe extern "system" fn __glDeleteLists(_list: GLuint, _range: GLsizei) {
                    panic!("Unable to load glDeleteLists")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDeleteLists\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDeleteLists
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glEvalMesh1: unsafe {
                unsafe extern "system" fn __glEvalMesh1(_mode: MeshMode1, _i1: GLint, _i2: GLint) {
                    panic!("Unable to load glEvalMesh1")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEvalMesh1\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEvalMesh1
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glEvalCoord2f: unsafe {
                unsafe extern "system" fn __glEvalCoord2f(_u: GLfloat, _v: GLfloat) {
                    panic!("Unable to load glEvalCoord2f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEvalCoord2f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEvalCoord2f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTranslated: unsafe {
                unsafe extern "system" fn __glTranslated(_x: GLdouble, _y: GLdouble, _z: GLdouble) {
                    panic!("Unable to load glTranslated")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTranslated\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTranslated
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRasterPos2f: unsafe {
                unsafe extern "system" fn __glRasterPos2f(_x: GLfloat, _y: GLfloat) {
                    panic!("Unable to load glRasterPos2f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRasterPos2f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRasterPos2f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord2iv: unsafe {
                unsafe extern "system" fn __glTexCoord2iv(_v: *mut GLint) {
                    panic!("Unable to load glTexCoord2iv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord2iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord2iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor4iv: unsafe {
                unsafe extern "system" fn __glColor4iv(_v: *mut GLint) {
                    panic!("Unable to load glColor4iv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor4iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor4iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRectfv: unsafe {
                unsafe extern "system" fn __glRectfv(_v1: *mut GLfloat, _v2: *mut GLfloat) {
                    panic!("Unable to load glRectfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRectfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRectfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor4dv: unsafe {
                unsafe extern "system" fn __glColor4dv(_v: *mut GLdouble) {
                    panic!("Unable to load glColor4dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor4dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor4dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRasterPos3sv: unsafe {
                unsafe extern "system" fn __glRasterPos3sv(_v: *mut GLshort) {
                    panic!("Unable to load glRasterPos3sv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRasterPos3sv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRasterPos3sv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexGeniv: unsafe {
                unsafe extern "system" fn __glTexGeniv(
                    _coord: TextureCoordName,
                    _pname: TextureGenParameter,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glTexGeniv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexGeniv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexGeniv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor3us: unsafe {
                unsafe extern "system" fn __glColor3us(
                    _red: GLushort,
                    _green: GLushort,
                    _blue: GLushort,
                ) {
                    panic!("Unable to load glColor3us")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor3us\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor3us
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetMapiv: unsafe {
                unsafe extern "system" fn __glGetMapiv(
                    _target: MapTarget,
                    _query: GetMapQuery,
                    _v: *mut GLint,
                ) {
                    panic!("Unable to load glGetMapiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetMapiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetMapiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glFogfv: unsafe {
                unsafe extern "system" fn __glFogfv(_pname: FogParameter, _params: *mut GLfloat) {
                    panic!("Unable to load glFogfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFogfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFogfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor3d: unsafe {
                unsafe extern "system" fn __glColor3d(
                    _red: GLdouble,
                    _green: GLdouble,
                    _blue: GLdouble,
                ) {
                    panic!("Unable to load glColor3d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor3d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor3d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPixelStorei: unsafe {
                unsafe extern "system" fn __glPixelStorei(
                    _pname: PixelStoreParameter,
                    _param: GLint,
                ) {
                    panic!("Unable to load glPixelStorei")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPixelStorei\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPixelStorei
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor3fv: unsafe {
                unsafe extern "system" fn __glColor3fv(_v: *mut GLfloat) {
                    panic!("Unable to load glColor3fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor3fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor3fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPixelTransferi: unsafe {
                unsafe extern "system" fn __glPixelTransferi(
                    _pname: PixelTransferParameter,
                    _param: GLint,
                ) {
                    panic!("Unable to load glPixelTransferi")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPixelTransferi\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPixelTransferi
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRasterPos2dv: unsafe {
                unsafe extern "system" fn __glRasterPos2dv(_v: *mut GLdouble) {
                    panic!("Unable to load glRasterPos2dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRasterPos2dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRasterPos2dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord2dv: unsafe {
                unsafe extern "system" fn __glTexCoord2dv(_v: *mut GLdouble) {
                    panic!("Unable to load glTexCoord2dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord2dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord2dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTranslatef: unsafe {
                unsafe extern "system" fn __glTranslatef(_x: GLfloat, _y: GLfloat, _z: GLfloat) {
                    panic!("Unable to load glTranslatef")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTranslatef\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTranslatef
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor4us: unsafe {
                unsafe extern "system" fn __glColor4us(
                    _red: GLushort,
                    _green: GLushort,
                    _blue: GLushort,
                    _alpha: GLushort,
                ) {
                    panic!("Unable to load glColor4us")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor4us\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor4us
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRasterPos2fv: unsafe {
                unsafe extern "system" fn __glRasterPos2fv(_v: *mut GLfloat) {
                    panic!("Unable to load glRasterPos2fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRasterPos2fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRasterPos2fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCallList: unsafe {
                unsafe extern "system" fn __glCallList(_list: GLuint) {
                    panic!("Unable to load glCallList")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCallList\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCallList
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertex4fv: unsafe {
                unsafe extern "system" fn __glVertex4fv(_v: *mut GLfloat) {
                    panic!("Unable to load glVertex4fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertex4fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertex4fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMap2d: unsafe {
                unsafe extern "system" fn __glMap2d(
                    _target: MapTarget,
                    _u1: GLdouble,
                    _u2: GLdouble,
                    _ustride: GLint,
                    _uorder: GLint,
                    _v1: GLdouble,
                    _v2: GLdouble,
                    _vstride: GLint,
                    _vorder: GLint,
                    _points: *mut GLdouble,
                ) {
                    panic!("Unable to load glMap2d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMap2d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMap2d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord1d: unsafe {
                unsafe extern "system" fn __glTexCoord1d(_s: GLdouble) {
                    panic!("Unable to load glTexCoord1d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord1d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord1d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glStencilFunc: unsafe {
                unsafe extern "system" fn __glStencilFunc(
                    _func: StencilFunction,
                    _ref: GLint,
                    _mask: GLuint,
                ) {
                    panic!("Unable to load glStencilFunc")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glStencilFunc\0");
                let val = _f(cname);
                if val.is_null() {
                    __glStencilFunc
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexImage1D: unsafe {
                unsafe extern "system" fn __glTexImage1D(
                    _target: TextureTarget,
                    _level: GLint,
                    _internalformat: InternalFormat,
                    _width: GLsizei,
                    _border: GLint,
                    _format: PixelFormat,
                    _type: PixelType,
                    _pixels: *mut GLvoid,
                ) {
                    panic!("Unable to load glTexImage1D")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexImage1D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexImage1D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMapGrid2f: unsafe {
                unsafe extern "system" fn __glMapGrid2f(
                    _un: GLint,
                    _u1: GLfloat,
                    _u2: GLfloat,
                    _vn: GLint,
                    _v1: GLfloat,
                    _v2: GLfloat,
                ) {
                    panic!("Unable to load glMapGrid2f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMapGrid2f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMapGrid2f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor3dv: unsafe {
                unsafe extern "system" fn __glColor3dv(_v: *mut GLdouble) {
                    panic!("Unable to load glColor3dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor3dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor3dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPushAttrib: unsafe {
                unsafe extern "system" fn __glPushAttrib(_mask: AttribMask) {
                    panic!("Unable to load glPushAttrib")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPushAttrib\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPushAttrib
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCopyPixels: unsafe {
                unsafe extern "system" fn __glCopyPixels(
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                    _type: PixelCopyType,
                ) {
                    panic!("Unable to load glCopyPixels")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCopyPixels\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCopyPixels
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glFrustum: unsafe {
                unsafe extern "system" fn __glFrustum(
                    _left: GLdouble,
                    _right: GLdouble,
                    _bottom: GLdouble,
                    _top: GLdouble,
                    _zNear: GLdouble,
                    _zFar: GLdouble,
                ) {
                    panic!("Unable to load glFrustum")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFrustum\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFrustum
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord3sv: unsafe {
                unsafe extern "system" fn __glTexCoord3sv(_v: *mut GLshort) {
                    panic!("Unable to load glTexCoord3sv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord3sv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord3sv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetPixelMapfv: unsafe {
                unsafe extern "system" fn __glGetPixelMapfv(_map: PixelMap, _values: *mut GLfloat) {
                    panic!("Unable to load glGetPixelMapfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetPixelMapfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetPixelMapfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glFogi: unsafe {
                unsafe extern "system" fn __glFogi(_pname: FogParameter, _param: GLint) {
                    panic!("Unable to load glFogi")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFogi\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFogi
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glLightfv: unsafe {
                unsafe extern "system" fn __glLightfv(
                    _light: LightName,
                    _pname: LightParameter,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load glLightfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLightfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLightfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glLoadMatrixd: unsafe {
                unsafe extern "system" fn __glLoadMatrixd(_m: *mut GLdouble) {
                    panic!("Unable to load glLoadMatrixd")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLoadMatrixd\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLoadMatrixd
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glIndexf: unsafe {
                unsafe extern "system" fn __glIndexf(_c: GLfloat) {
                    panic!("Unable to load glIndexf")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIndexf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIndexf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glInitNames: unsafe {
                unsafe extern "system" fn __glInitNames() {
                    panic!("Unable to load glInitNames")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glInitNames\0");
                let val = _f(cname);
                if val.is_null() {
                    __glInitNames
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPixelMapuiv: unsafe {
                unsafe extern "system" fn __glPixelMapuiv(
                    _map: PixelMap,
                    _mapsize: GLsizei,
                    _values: *mut GLuint,
                ) {
                    panic!("Unable to load glPixelMapuiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPixelMapuiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPixelMapuiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor4uiv: unsafe {
                unsafe extern "system" fn __glColor4uiv(_v: *mut GLuint) {
                    panic!("Unable to load glColor4uiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor4uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor4uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glEvalPoint1: unsafe {
                unsafe extern "system" fn __glEvalPoint1(_i: GLint) {
                    panic!("Unable to load glEvalPoint1")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEvalPoint1\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEvalPoint1
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetPixelMapuiv: unsafe {
                unsafe extern "system" fn __glGetPixelMapuiv(_map: PixelMap, _values: *mut GLuint) {
                    panic!("Unable to load glGetPixelMapuiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetPixelMapuiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetPixelMapuiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glViewport: unsafe {
                unsafe extern "system" fn __glViewport(
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                ) {
                    panic!("Unable to load glViewport")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glViewport\0");
                let val = _f(cname);
                if val.is_null() {
                    __glViewport
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glEvalCoord2d: unsafe {
                unsafe extern "system" fn __glEvalCoord2d(_u: GLdouble, _v: GLdouble) {
                    panic!("Unable to load glEvalCoord2d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEvalCoord2d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEvalCoord2d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glLightf: unsafe {
                unsafe extern "system" fn __glLightf(
                    _light: LightName,
                    _pname: LightParameter,
                    _param: GLfloat,
                ) {
                    panic!("Unable to load glLightf")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLightf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLightf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor3i: unsafe {
                unsafe extern "system" fn __glColor3i(_red: GLint, _green: GLint, _blue: GLint) {
                    panic!("Unable to load glColor3i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor3i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor3i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRasterPos3f: unsafe {
                unsafe extern "system" fn __glRasterPos3f(_x: GLfloat, _y: GLfloat, _z: GLfloat) {
                    panic!("Unable to load glRasterPos3f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRasterPos3f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRasterPos3f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glAlphaFunc: unsafe {
                unsafe extern "system" fn __glAlphaFunc(_func: AlphaFunction, _ref: GLfloat) {
                    panic!("Unable to load glAlphaFunc")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glAlphaFunc\0");
                let val = _f(cname);
                if val.is_null() {
                    __glAlphaFunc
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertex2dv: unsafe {
                unsafe extern "system" fn __glVertex2dv(_v: *mut GLdouble) {
                    panic!("Unable to load glVertex2dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertex2dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertex2dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor4d: unsafe {
                unsafe extern "system" fn __glColor4d(
                    _red: GLdouble,
                    _green: GLdouble,
                    _blue: GLdouble,
                    _alpha: GLdouble,
                ) {
                    panic!("Unable to load glColor4d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor4d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor4d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glNormal3b: unsafe {
                unsafe extern "system" fn __glNormal3b(_nx: GLbyte, _ny: GLbyte, _nz: GLbyte) {
                    panic!("Unable to load glNormal3b")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glNormal3b\0");
                let val = _f(cname);
                if val.is_null() {
                    __glNormal3b
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetString: unsafe {
                unsafe extern "system" fn __glGetString(_name: StringName) -> *const char {
                    panic!("Unable to load glGetString")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetString\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetString
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glIsEnabled: unsafe {
                unsafe extern "system" fn __glIsEnabled(_cap: EnableCap) -> GLboolean {
                    panic!("Unable to load glIsEnabled")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIsEnabled\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIsEnabled
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRectiv: unsafe {
                unsafe extern "system" fn __glRectiv(_v1: *mut GLint, _v2: *mut GLint) {
                    panic!("Unable to load glRectiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRectiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRectiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRasterPos2iv: unsafe {
                unsafe extern "system" fn __glRasterPos2iv(_v: *mut GLint) {
                    panic!("Unable to load glRasterPos2iv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRasterPos2iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRasterPos2iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRasterPos4sv: unsafe {
                unsafe extern "system" fn __glRasterPos4sv(_v: *mut GLshort) {
                    panic!("Unable to load glRasterPos4sv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRasterPos4sv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRasterPos4sv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor3iv: unsafe {
                unsafe extern "system" fn __glColor3iv(_v: *mut GLint) {
                    panic!("Unable to load glColor3iv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor3iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor3iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPopAttrib: unsafe {
                unsafe extern "system" fn __glPopAttrib() {
                    panic!("Unable to load glPopAttrib")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPopAttrib\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPopAttrib
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRasterPos4f: unsafe {
                unsafe extern "system" fn __glRasterPos4f(
                    _x: GLfloat,
                    _y: GLfloat,
                    _z: GLfloat,
                    _w: GLfloat,
                ) {
                    panic!("Unable to load glRasterPos4f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRasterPos4f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRasterPos4f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMap2f: unsafe {
                unsafe extern "system" fn __glMap2f(
                    _target: MapTarget,
                    _u1: GLfloat,
                    _u2: GLfloat,
                    _ustride: GLint,
                    _uorder: GLint,
                    _v1: GLfloat,
                    _v2: GLfloat,
                    _vstride: GLint,
                    _vorder: GLint,
                    _points: *mut GLfloat,
                ) {
                    panic!("Unable to load glMap2f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMap2f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMap2f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertex3dv: unsafe {
                unsafe extern "system" fn __glVertex3dv(_v: *mut GLdouble) {
                    panic!("Unable to load glVertex3dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertex3dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertex3dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBlendFunc: unsafe {
                unsafe extern "system" fn __glBlendFunc(
                    _sfactor: BlendingFactor,
                    _dfactor: BlendingFactor,
                ) {
                    panic!("Unable to load glBlendFunc")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBlendFunc\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBlendFunc
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord2d: unsafe {
                unsafe extern "system" fn __glTexCoord2d(_s: GLdouble, _t: GLdouble) {
                    panic!("Unable to load glTexCoord2d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord2d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord2d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRotatef: unsafe {
                unsafe extern "system" fn __glRotatef(
                    _angle: GLfloat,
                    _x: GLfloat,
                    _y: GLfloat,
                    _z: GLfloat,
                ) {
                    panic!("Unable to load glRotatef")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRotatef\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRotatef
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor3bv: unsafe {
                unsafe extern "system" fn __glColor3bv(_v: *mut GLbyte) {
                    panic!("Unable to load glColor3bv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor3bv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor3bv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRectdv: unsafe {
                unsafe extern "system" fn __glRectdv(_v1: *mut GLdouble, _v2: *mut GLdouble) {
                    panic!("Unable to load glRectdv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRectdv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRectdv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glEdgeFlagv: unsafe {
                unsafe extern "system" fn __glEdgeFlagv(_flag: *mut Boolean) {
                    panic!("Unable to load glEdgeFlagv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEdgeFlagv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEdgeFlagv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexEnvf: unsafe {
                unsafe extern "system" fn __glTexEnvf(
                    _target: TextureEnvTarget,
                    _pname: TextureEnvParameter,
                    _param: GLfloat,
                ) {
                    panic!("Unable to load glTexEnvf")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexEnvf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexEnvf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glEdgeFlag: unsafe {
                unsafe extern "system" fn __glEdgeFlag(_flag: Boolean) {
                    panic!("Unable to load glEdgeFlag")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEdgeFlag\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEdgeFlag
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetTexGenfv: unsafe {
                unsafe extern "system" fn __glGetTexGenfv(
                    _coord: TextureCoordName,
                    _pname: TextureGenParameter,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load glGetTexGenfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTexGenfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTexGenfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glLoadIdentity: unsafe {
                unsafe extern "system" fn __glLoadIdentity() {
                    panic!("Unable to load glLoadIdentity")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLoadIdentity\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLoadIdentity
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetFloatv: unsafe {
                unsafe extern "system" fn __glGetFloatv(_pname: GetPName, _data: *mut GLfloat) {
                    panic!("Unable to load glGetFloatv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetFloatv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetFloatv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glScalef: unsafe {
                unsafe extern "system" fn __glScalef(_x: GLfloat, _y: GLfloat, _z: GLfloat) {
                    panic!("Unable to load glScalef")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glScalef\0");
                let val = _f(cname);
                if val.is_null() {
                    __glScalef
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColorMask: unsafe {
                unsafe extern "system" fn __glColorMask(
                    _red: Boolean,
                    _green: Boolean,
                    _blue: Boolean,
                    _alpha: Boolean,
                ) {
                    panic!("Unable to load glColorMask")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColorMask\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColorMask
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glIndexdv: unsafe {
                unsafe extern "system" fn __glIndexdv(_c: *mut GLdouble) {
                    panic!("Unable to load glIndexdv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIndexdv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIndexdv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord3d: unsafe {
                unsafe extern "system" fn __glTexCoord3d(_s: GLdouble, _t: GLdouble, _r: GLdouble) {
                    panic!("Unable to load glTexCoord3d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord3d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord3d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetIntegerv: unsafe {
                unsafe extern "system" fn __glGetIntegerv(_pname: GetPName, _data: *mut GLint) {
                    panic!("Unable to load glGetIntegerv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetIntegerv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetIntegerv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertex2s: unsafe {
                unsafe extern "system" fn __glVertex2s(_x: GLshort, _y: GLshort) {
                    panic!("Unable to load glVertex2s")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertex2s\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertex2s
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor4usv: unsafe {
                unsafe extern "system" fn __glColor4usv(_v: *mut GLushort) {
                    panic!("Unable to load glColor4usv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor4usv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor4usv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glScissor: unsafe {
                unsafe extern "system" fn __glScissor(
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                ) {
                    panic!("Unable to load glScissor")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glScissor\0");
                let val = _f(cname);
                if val.is_null() {
                    __glScissor
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPolygonStipple: unsafe {
                unsafe extern "system" fn __glPolygonStipple(_mask: *mut GLubyte) {
                    panic!("Unable to load glPolygonStipple")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPolygonStipple\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPolygonStipple
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMapGrid1f: unsafe {
                unsafe extern "system" fn __glMapGrid1f(_un: GLint, _u1: GLfloat, _u2: GLfloat) {
                    panic!("Unable to load glMapGrid1f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMapGrid1f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMapGrid1f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPolygonMode: unsafe {
                unsafe extern "system" fn __glPolygonMode(_face: MaterialFace, _mode: PolygonMode) {
                    panic!("Unable to load glPolygonMode")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPolygonMode\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPolygonMode
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertex3iv: unsafe {
                unsafe extern "system" fn __glVertex3iv(_v: *mut GLint) {
                    panic!("Unable to load glVertex3iv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertex3iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertex3iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord2sv: unsafe {
                unsafe extern "system" fn __glTexCoord2sv(_v: *mut GLshort) {
                    panic!("Unable to load glTexCoord2sv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord2sv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord2sv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glListBase: unsafe {
                unsafe extern "system" fn __glListBase(_base: GLuint) {
                    panic!("Unable to load glListBase")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glListBase\0");
                let val = _f(cname);
                if val.is_null() {
                    __glListBase
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord4i: unsafe {
                unsafe extern "system" fn __glTexCoord4i(
                    _s: GLint,
                    _t: GLint,
                    _r: GLint,
                    _q: GLint,
                ) {
                    panic!("Unable to load glTexCoord4i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord4i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord4i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glEvalCoord1fv: unsafe {
                unsafe extern "system" fn __glEvalCoord1fv(_u: *mut GLfloat) {
                    panic!("Unable to load glEvalCoord1fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEvalCoord1fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEvalCoord1fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRasterPos4fv: unsafe {
                unsafe extern "system" fn __glRasterPos4fv(_v: *mut GLfloat) {
                    panic!("Unable to load glRasterPos4fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRasterPos4fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRasterPos4fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetLightfv: unsafe {
                unsafe extern "system" fn __glGetLightfv(
                    _light: LightName,
                    _pname: LightParameter,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load glGetLightfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetLightfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetLightfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetMaterialiv: unsafe {
                unsafe extern "system" fn __glGetMaterialiv(
                    _face: MaterialFace,
                    _pname: MaterialParameter,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetMaterialiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetMaterialiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetMaterialiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glFinish: unsafe {
                unsafe extern "system" fn __glFinish() {
                    panic!("Unable to load glFinish")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFinish\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFinish
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMaterialfv: unsafe {
                unsafe extern "system" fn __glMaterialfv(
                    _face: MaterialFace,
                    _pname: MaterialParameter,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load glMaterialfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMaterialfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMaterialfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPixelMapusv: unsafe {
                unsafe extern "system" fn __glPixelMapusv(
                    _map: PixelMap,
                    _mapsize: GLsizei,
                    _values: *mut GLushort,
                ) {
                    panic!("Unable to load glPixelMapusv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPixelMapusv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPixelMapusv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPixelStoref: unsafe {
                unsafe extern "system" fn __glPixelStoref(
                    _pname: PixelStoreParameter,
                    _param: GLfloat,
                ) {
                    panic!("Unable to load glPixelStoref")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPixelStoref\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPixelStoref
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexEnvi: unsafe {
                unsafe extern "system" fn __glTexEnvi(
                    _target: TextureEnvTarget,
                    _pname: TextureEnvParameter,
                    _param: GLint,
                ) {
                    panic!("Unable to load glTexEnvi")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexEnvi\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexEnvi
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor3sv: unsafe {
                unsafe extern "system" fn __glColor3sv(_v: *mut GLshort) {
                    panic!("Unable to load glColor3sv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor3sv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor3sv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCallLists: unsafe {
                unsafe extern "system" fn __glCallLists(
                    _n: GLsizei,
                    _type: ListNameType,
                    _lists: *mut GLvoid,
                ) {
                    panic!("Unable to load glCallLists")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCallLists\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCallLists
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRasterPos2i: unsafe {
                unsafe extern "system" fn __glRasterPos2i(_x: GLint, _y: GLint) {
                    panic!("Unable to load glRasterPos2i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRasterPos2i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRasterPos2i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRasterPos2sv: unsafe {
                unsafe extern "system" fn __glRasterPos2sv(_v: *mut GLshort) {
                    panic!("Unable to load glRasterPos2sv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRasterPos2sv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRasterPos2sv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor3ubv: unsafe {
                unsafe extern "system" fn __glColor3ubv(_v: *mut GLubyte) {
                    panic!("Unable to load glColor3ubv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor3ubv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor3ubv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor4sv: unsafe {
                unsafe extern "system" fn __glColor4sv(_v: *mut GLshort) {
                    panic!("Unable to load glColor4sv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor4sv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor4sv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetMaterialfv: unsafe {
                unsafe extern "system" fn __glGetMaterialfv(
                    _face: MaterialFace,
                    _pname: MaterialParameter,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load glGetMaterialfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetMaterialfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetMaterialfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord2fv: unsafe {
                unsafe extern "system" fn __glTexCoord2fv(_v: *mut GLfloat) {
                    panic!("Unable to load glTexCoord2fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord2fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord2fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertex4i: unsafe {
                unsafe extern "system" fn __glVertex4i(_x: GLint, _y: GLint, _z: GLint, _w: GLint) {
                    panic!("Unable to load glVertex4i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertex4i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertex4i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPushName: unsafe {
                unsafe extern "system" fn __glPushName(_name: GLuint) {
                    panic!("Unable to load glPushName")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPushName\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPushName
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDrawPixels: unsafe {
                unsafe extern "system" fn __glDrawPixels(
                    _width: GLsizei,
                    _height: GLsizei,
                    _format: PixelFormat,
                    _type: PixelType,
                    _pixels: *mut GLvoid,
                ) {
                    panic!("Unable to load glDrawPixels")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDrawPixels\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDrawPixels
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord1fv: unsafe {
                unsafe extern "system" fn __glTexCoord1fv(_v: *mut GLfloat) {
                    panic!("Unable to load glTexCoord1fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord1fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord1fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetTexGendv: unsafe {
                unsafe extern "system" fn __glGetTexGendv(
                    _coord: TextureCoordName,
                    _pname: TextureGenParameter,
                    _params: *mut GLdouble,
                ) {
                    panic!("Unable to load glGetTexGendv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTexGendv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTexGendv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDepthFunc: unsafe {
                unsafe extern "system" fn __glDepthFunc(_func: DepthFunction) {
                    panic!("Unable to load glDepthFunc")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDepthFunc\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDepthFunc
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord1dv: unsafe {
                unsafe extern "system" fn __glTexCoord1dv(_v: *mut GLdouble) {
                    panic!("Unable to load glTexCoord1dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord1dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord1dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord1i: unsafe {
                unsafe extern "system" fn __glTexCoord1i(_s: GLint) {
                    panic!("Unable to load glTexCoord1i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord1i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord1i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertex4iv: unsafe {
                unsafe extern "system" fn __glVertex4iv(_v: *mut GLint) {
                    panic!("Unable to load glVertex4iv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertex4iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertex4iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetDoublev: unsafe {
                unsafe extern "system" fn __glGetDoublev(_pname: GetPName, _data: *mut GLdouble) {
                    panic!("Unable to load glGetDoublev")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetDoublev\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetDoublev
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord3fv: unsafe {
                unsafe extern "system" fn __glTexCoord3fv(_v: *mut GLfloat) {
                    panic!("Unable to load glTexCoord3fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord3fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord3fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRasterPos4d: unsafe {
                unsafe extern "system" fn __glRasterPos4d(
                    _x: GLdouble,
                    _y: GLdouble,
                    _z: GLdouble,
                    _w: GLdouble,
                ) {
                    panic!("Unable to load glRasterPos4d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRasterPos4d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRasterPos4d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glNormal3iv: unsafe {
                unsafe extern "system" fn __glNormal3iv(_v: *mut GLint) {
                    panic!("Unable to load glNormal3iv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glNormal3iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glNormal3iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glReadBuffer: unsafe {
                unsafe extern "system" fn __glReadBuffer(_src: ReadBufferMode) {
                    panic!("Unable to load glReadBuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glReadBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glReadBuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRasterPos2s: unsafe {
                unsafe extern "system" fn __glRasterPos2s(_x: GLshort, _y: GLshort) {
                    panic!("Unable to load glRasterPos2s")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRasterPos2s\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRasterPos2s
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glEnd: unsafe {
                unsafe extern "system" fn __glEnd() {
                    panic!("Unable to load glEnd")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEnd\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEnd
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glIndexi: unsafe {
                unsafe extern "system" fn __glIndexi(_c: GLint) {
                    panic!("Unable to load glIndexi")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIndexi\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIndexi
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord1sv: unsafe {
                unsafe extern "system" fn __glTexCoord1sv(_v: *mut GLshort) {
                    panic!("Unable to load glTexCoord1sv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord1sv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord1sv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGenLists: unsafe {
                unsafe extern "system" fn __glGenLists(_range: GLsizei) -> GLuint {
                    panic!("Unable to load glGenLists")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGenLists\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGenLists
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDepthRange: unsafe {
                unsafe extern "system" fn __glDepthRange(_n: GLdouble, _f: GLdouble) {
                    panic!("Unable to load glDepthRange")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDepthRange\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDepthRange
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertex3i: unsafe {
                unsafe extern "system" fn __glVertex3i(_x: GLint, _y: GLint, _z: GLint) {
                    panic!("Unable to load glVertex3i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertex3i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertex3i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glClearAccum: unsafe {
                unsafe extern "system" fn __glClearAccum(
                    _red: GLfloat,
                    _green: GLfloat,
                    _blue: GLfloat,
                    _alpha: GLfloat,
                ) {
                    panic!("Unable to load glClearAccum")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClearAccum\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClearAccum
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetClipPlane: unsafe {
                unsafe extern "system" fn __glGetClipPlane(
                    _plane: ClipPlaneName,
                    _equation: *mut GLdouble,
                ) {
                    panic!("Unable to load glGetClipPlane")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetClipPlane\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetClipPlane
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetTexEnviv: unsafe {
                unsafe extern "system" fn __glGetTexEnviv(
                    _target: TextureEnvTarget,
                    _pname: TextureEnvParameter,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetTexEnviv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTexEnviv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTexEnviv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glIsList: unsafe {
                unsafe extern "system" fn __glIsList(_list: GLuint) -> GLboolean {
                    panic!("Unable to load glIsList")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIsList\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIsList
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor3usv: unsafe {
                unsafe extern "system" fn __glColor3usv(_v: *mut GLushort) {
                    panic!("Unable to load glColor3usv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor3usv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor3usv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glNormal3sv: unsafe {
                unsafe extern "system" fn __glNormal3sv(_v: *mut GLshort) {
                    panic!("Unable to load glNormal3sv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glNormal3sv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glNormal3sv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glLightModeliv: unsafe {
                unsafe extern "system" fn __glLightModeliv(
                    _pname: LightModelParameter,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glLightModeliv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLightModeliv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLightModeliv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor3s: unsafe {
                unsafe extern "system" fn __glColor3s(
                    _red: GLshort,
                    _green: GLshort,
                    _blue: GLshort,
                ) {
                    panic!("Unable to load glColor3s")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor3s\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor3s
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexGendv: unsafe {
                unsafe extern "system" fn __glTexGendv(
                    _coord: TextureCoordName,
                    _pname: TextureGenParameter,
                    _params: *mut GLdouble,
                ) {
                    panic!("Unable to load glTexGendv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexGendv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexGendv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glFlush: unsafe {
                unsafe extern "system" fn __glFlush() {
                    panic!("Unable to load glFlush")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFlush\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFlush
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertex3sv: unsafe {
                unsafe extern "system" fn __glVertex3sv(_v: *mut GLshort) {
                    panic!("Unable to load glVertex3sv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertex3sv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertex3sv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPopMatrix: unsafe {
                unsafe extern "system" fn __glPopMatrix() {
                    panic!("Unable to load glPopMatrix")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPopMatrix\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPopMatrix
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexParameteri: unsafe {
                unsafe extern "system" fn __glTexParameteri(
                    _target: TextureTarget,
                    _pname: TextureParameterName,
                    _param: GLint,
                ) {
                    panic!("Unable to load glTexParameteri")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexParameteri\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexParameteri
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRasterPos4s: unsafe {
                unsafe extern "system" fn __glRasterPos4s(
                    _x: GLshort,
                    _y: GLshort,
                    _z: GLshort,
                    _w: GLshort,
                ) {
                    panic!("Unable to load glRasterPos4s")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRasterPos4s\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRasterPos4s
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRecti: unsafe {
                unsafe extern "system" fn __glRecti(
                    _x1: GLint,
                    _y1: GLint,
                    _x2: GLint,
                    _y2: GLint,
                ) {
                    panic!("Unable to load glRecti")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRecti\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRecti
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glClearColor: unsafe {
                unsafe extern "system" fn __glClearColor(
                    _red: GLfloat,
                    _green: GLfloat,
                    _blue: GLfloat,
                    _alpha: GLfloat,
                ) {
                    panic!("Unable to load glClearColor")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClearColor\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClearColor
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glStencilOp: unsafe {
                unsafe extern "system" fn __glStencilOp(
                    _fail: StencilOp,
                    _zfail: StencilOp,
                    _zpass: StencilOp,
                ) {
                    panic!("Unable to load glStencilOp")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glStencilOp\0");
                let val = _f(cname);
                if val.is_null() {
                    __glStencilOp
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glIndexiv: unsafe {
                unsafe extern "system" fn __glIndexiv(_c: *mut GLint) {
                    panic!("Unable to load glIndexiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIndexiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIndexiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRenderMode: unsafe {
                unsafe extern "system" fn __glRenderMode(_mode: RenderingMode) -> GLint {
                    panic!("Unable to load glRenderMode")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRenderMode\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRenderMode
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertex2iv: unsafe {
                unsafe extern "system" fn __glVertex2iv(_v: *mut GLint) {
                    panic!("Unable to load glVertex2iv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertex2iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertex2iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertex4d: unsafe {
                unsafe extern "system" fn __glVertex4d(
                    _x: GLdouble,
                    _y: GLdouble,
                    _z: GLdouble,
                    _w: GLdouble,
                ) {
                    panic!("Unable to load glVertex4d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertex4d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertex4d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColorMaterial: unsafe {
                unsafe extern "system" fn __glColorMaterial(
                    _face: MaterialFace,
                    _mode: ColorMaterialParameter,
                ) {
                    panic!("Unable to load glColorMaterial")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColorMaterial\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColorMaterial
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetMapdv: unsafe {
                unsafe extern "system" fn __glGetMapdv(
                    _target: MapTarget,
                    _query: GetMapQuery,
                    _v: *mut GLdouble,
                ) {
                    panic!("Unable to load glGetMapdv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetMapdv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetMapdv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetPolygonStipple: unsafe {
                unsafe extern "system" fn __glGetPolygonStipple(_mask: *mut GLubyte) {
                    panic!("Unable to load glGetPolygonStipple")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetPolygonStipple\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetPolygonStipple
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glLoadName: unsafe {
                unsafe extern "system" fn __glLoadName(_name: GLuint) {
                    panic!("Unable to load glLoadName")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLoadName\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLoadName
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord3iv: unsafe {
                unsafe extern "system" fn __glTexCoord3iv(_v: *mut GLint) {
                    panic!("Unable to load glTexCoord3iv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord3iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord3iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glLogicOp: unsafe {
                unsafe extern "system" fn __glLogicOp(_opcode: LogicOp) {
                    panic!("Unable to load glLogicOp")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLogicOp\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLogicOp
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetPixelMapusv: unsafe {
                unsafe extern "system" fn __glGetPixelMapusv(
                    _map: PixelMap,
                    _values: *mut GLushort,
                ) {
                    panic!("Unable to load glGetPixelMapusv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetPixelMapusv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetPixelMapusv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glScaled: unsafe {
                unsafe extern "system" fn __glScaled(_x: GLdouble, _y: GLdouble, _z: GLdouble) {
                    panic!("Unable to load glScaled")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glScaled\0");
                let val = _f(cname);
                if val.is_null() {
                    __glScaled
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexParameterf: unsafe {
                unsafe extern "system" fn __glTexParameterf(
                    _target: TextureTarget,
                    _pname: TextureParameterName,
                    _param: GLfloat,
                ) {
                    panic!("Unable to load glTexParameterf")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexParameterf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexParameterf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glNormal3dv: unsafe {
                unsafe extern "system" fn __glNormal3dv(_v: *mut GLdouble) {
                    panic!("Unable to load glNormal3dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glNormal3dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glNormal3dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRasterPos3fv: unsafe {
                unsafe extern "system" fn __glRasterPos3fv(_v: *mut GLfloat) {
                    panic!("Unable to load glRasterPos3fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRasterPos3fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRasterPos3fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord4d: unsafe {
                unsafe extern "system" fn __glTexCoord4d(
                    _s: GLdouble,
                    _t: GLdouble,
                    _r: GLdouble,
                    _q: GLdouble,
                ) {
                    panic!("Unable to load glTexCoord4d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord4d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord4d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glEvalCoord1f: unsafe {
                unsafe extern "system" fn __glEvalCoord1f(_u: GLfloat) {
                    panic!("Unable to load glEvalCoord1f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEvalCoord1f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEvalCoord1f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMaterialiv: unsafe {
                unsafe extern "system" fn __glMaterialiv(
                    _face: MaterialFace,
                    _pname: MaterialParameter,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glMaterialiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMaterialiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMaterialiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glNewList: unsafe {
                unsafe extern "system" fn __glNewList(_list: GLuint, _mode: ListMode) {
                    panic!("Unable to load glNewList")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glNewList\0");
                let val = _f(cname);
                if val.is_null() {
                    __glNewList
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetTexParameteriv: unsafe {
                unsafe extern "system" fn __glGetTexParameteriv(
                    _target: TextureTarget,
                    _pname: GetTextureParameter,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetTexParameteriv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTexParameteriv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTexParameteriv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertex3f: unsafe {
                unsafe extern "system" fn __glVertex3f(_x: GLfloat, _y: GLfloat, _z: GLfloat) {
                    panic!("Unable to load glVertex3f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertex3f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertex3f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor4ui: unsafe {
                unsafe extern "system" fn __glColor4ui(
                    _red: GLuint,
                    _green: GLuint,
                    _blue: GLuint,
                    _alpha: GLuint,
                ) {
                    panic!("Unable to load glColor4ui")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor4ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor4ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glShadeModel: unsafe {
                unsafe extern "system" fn __glShadeModel(_mode: ShadingModel) {
                    panic!("Unable to load glShadeModel")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glShadeModel\0");
                let val = _f(cname);
                if val.is_null() {
                    __glShadeModel
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMap1d: unsafe {
                unsafe extern "system" fn __glMap1d(
                    _target: MapTarget,
                    _u1: GLdouble,
                    _u2: GLdouble,
                    _stride: GLint,
                    _order: GLint,
                    _points: *mut GLdouble,
                ) {
                    panic!("Unable to load glMap1d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMap1d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMap1d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glClearDepth: unsafe {
                unsafe extern "system" fn __glClearDepth(_depth: GLdouble) {
                    panic!("Unable to load glClearDepth")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClearDepth\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClearDepth
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glLightModelf: unsafe {
                unsafe extern "system" fn __glLightModelf(
                    _pname: LightModelParameter,
                    _param: GLfloat,
                ) {
                    panic!("Unable to load glLightModelf")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLightModelf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLightModelf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord2s: unsafe {
                unsafe extern "system" fn __glTexCoord2s(_s: GLshort, _t: GLshort) {
                    panic!("Unable to load glTexCoord2s")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord2s\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord2s
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetTexGeniv: unsafe {
                unsafe extern "system" fn __glGetTexGeniv(
                    _coord: TextureCoordName,
                    _pname: TextureGenParameter,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetTexGeniv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTexGeniv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTexGeniv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertex2i: unsafe {
                unsafe extern "system" fn __glVertex2i(_x: GLint, _y: GLint) {
                    panic!("Unable to load glVertex2i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertex2i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertex2i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glLoadMatrixf: unsafe {
                unsafe extern "system" fn __glLoadMatrixf(_m: *mut GLfloat) {
                    panic!("Unable to load glLoadMatrixf")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLoadMatrixf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLoadMatrixf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor3b: unsafe {
                unsafe extern "system" fn __glColor3b(_red: GLbyte, _green: GLbyte, _blue: GLbyte) {
                    panic!("Unable to load glColor3b")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor3b\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor3b
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord1f: unsafe {
                unsafe extern "system" fn __glTexCoord1f(_s: GLfloat) {
                    panic!("Unable to load glTexCoord1f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord1f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord1f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDisable: unsafe {
                unsafe extern "system" fn __glDisable(_cap: EnableCap) {
                    panic!("Unable to load glDisable")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDisable\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDisable
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertex3s: unsafe {
                unsafe extern "system" fn __glVertex3s(_x: GLshort, _y: GLshort, _z: GLshort) {
                    panic!("Unable to load glVertex3s")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertex3s\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertex3s
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glIndexs: unsafe {
                unsafe extern "system" fn __glIndexs(_c: GLshort) {
                    panic!("Unable to load glIndexs")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIndexs\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIndexs
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glEnable: unsafe {
                unsafe extern "system" fn __glEnable(_cap: EnableCap) {
                    panic!("Unable to load glEnable")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEnable\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEnable
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord4fv: unsafe {
                unsafe extern "system" fn __glTexCoord4fv(_v: *mut GLfloat) {
                    panic!("Unable to load glTexCoord4fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord4fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord4fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glLightiv: unsafe {
                unsafe extern "system" fn __glLightiv(
                    _light: LightName,
                    _pname: LightParameter,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glLightiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLightiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLightiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glClearStencil: unsafe {
                unsafe extern "system" fn __glClearStencil(_s: GLint) {
                    panic!("Unable to load glClearStencil")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClearStencil\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClearStencil
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glClearIndex: unsafe {
                unsafe extern "system" fn __glClearIndex(_c: GLfloat) {
                    panic!("Unable to load glClearIndex")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClearIndex\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClearIndex
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetTexEnvfv: unsafe {
                unsafe extern "system" fn __glGetTexEnvfv(
                    _target: TextureEnvTarget,
                    _pname: TextureEnvParameter,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load glGetTexEnvfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTexEnvfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTexEnvfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultMatrixd: unsafe {
                unsafe extern "system" fn __glMultMatrixd(_m: *mut GLdouble) {
                    panic!("Unable to load glMultMatrixd")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultMatrixd\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultMatrixd
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor4fv: unsafe {
                unsafe extern "system" fn __glColor4fv(_v: *mut GLfloat) {
                    panic!("Unable to load glColor4fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor4fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor4fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetMapfv: unsafe {
                unsafe extern "system" fn __glGetMapfv(
                    _target: MapTarget,
                    _query: GetMapQuery,
                    _v: *mut GLfloat,
                ) {
                    panic!("Unable to load glGetMapfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetMapfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetMapfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glNormal3fv: unsafe {
                unsafe extern "system" fn __glNormal3fv(_v: *mut GLfloat) {
                    panic!("Unable to load glNormal3fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glNormal3fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glNormal3fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetLightiv: unsafe {
                unsafe extern "system" fn __glGetLightiv(
                    _light: LightName,
                    _pname: LightParameter,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetLightiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetLightiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetLightiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor4bv: unsafe {
                unsafe extern "system" fn __glColor4bv(_v: *mut GLbyte) {
                    panic!("Unable to load glColor4bv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor4bv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor4bv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glIndexMask: unsafe {
                unsafe extern "system" fn __glIndexMask(_mask: GLuint) {
                    panic!("Unable to load glIndexMask")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIndexMask\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIndexMask
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPixelZoom: unsafe {
                unsafe extern "system" fn __glPixelZoom(_xfactor: GLfloat, _yfactor: GLfloat) {
                    panic!("Unable to load glPixelZoom")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPixelZoom\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPixelZoom
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertex4dv: unsafe {
                unsafe extern "system" fn __glVertex4dv(_v: *mut GLdouble) {
                    panic!("Unable to load glVertex4dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertex4dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertex4dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glLightModelfv: unsafe {
                unsafe extern "system" fn __glLightModelfv(
                    _pname: LightModelParameter,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load glLightModelfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLightModelfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLightModelfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRectd: unsafe {
                unsafe extern "system" fn __glRectd(
                    _x1: GLdouble,
                    _y1: GLdouble,
                    _x2: GLdouble,
                    _y2: GLdouble,
                ) {
                    panic!("Unable to load glRectd")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRectd\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRectd
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glEvalCoord1d: unsafe {
                unsafe extern "system" fn __glEvalCoord1d(_u: GLdouble) {
                    panic!("Unable to load glEvalCoord1d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEvalCoord1d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEvalCoord1d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetTexParameterfv: unsafe {
                unsafe extern "system" fn __glGetTexParameterfv(
                    _target: TextureTarget,
                    _pname: GetTextureParameter,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load glGetTexParameterfv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTexParameterfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTexParameterfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertex3fv: unsafe {
                unsafe extern "system" fn __glVertex3fv(_v: *mut GLfloat) {
                    panic!("Unable to load glVertex3fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertex3fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertex3fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRasterPos3iv: unsafe {
                unsafe extern "system" fn __glRasterPos3iv(_v: *mut GLint) {
                    panic!("Unable to load glRasterPos3iv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRasterPos3iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRasterPos3iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRects: unsafe {
                unsafe extern "system" fn __glRects(
                    _x1: GLshort,
                    _y1: GLshort,
                    _x2: GLshort,
                    _y2: GLshort,
                ) {
                    panic!("Unable to load glRects")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRects\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRects
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertex2fv: unsafe {
                unsafe extern "system" fn __glVertex2fv(_v: *mut GLfloat) {
                    panic!("Unable to load glVertex2fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertex2fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertex2fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertex4f: unsafe {
                unsafe extern "system" fn __glVertex4f(
                    _x: GLfloat,
                    _y: GLfloat,
                    _z: GLfloat,
                    _w: GLfloat,
                ) {
                    panic!("Unable to load glVertex4f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertex4f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertex4f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glFogf: unsafe {
                unsafe extern "system" fn __glFogf(_pname: FogParameter, _param: GLfloat) {
                    panic!("Unable to load glFogf")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFogf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFogf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexGenf: unsafe {
                unsafe extern "system" fn __glTexGenf(
                    _coord: TextureCoordName,
                    _pname: TextureGenParameter,
                    _param: GLfloat,
                ) {
                    panic!("Unable to load glTexGenf")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexGenf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexGenf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMapGrid2d: unsafe {
                unsafe extern "system" fn __glMapGrid2d(
                    _un: GLint,
                    _u1: GLdouble,
                    _u2: GLdouble,
                    _vn: GLint,
                    _v1: GLdouble,
                    _v2: GLdouble,
                ) {
                    panic!("Unable to load glMapGrid2d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMapGrid2d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMapGrid2d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glSelectBuffer: unsafe {
                unsafe extern "system" fn __glSelectBuffer(_size: GLsizei, _buffer: *mut GLuint) {
                    panic!("Unable to load glSelectBuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSelectBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSelectBuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPushMatrix: unsafe {
                unsafe extern "system" fn __glPushMatrix() {
                    panic!("Unable to load glPushMatrix")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPushMatrix\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPushMatrix
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glEvalMesh2: unsafe {
                unsafe extern "system" fn __glEvalMesh2(
                    _mode: MeshMode2,
                    _i1: GLint,
                    _i2: GLint,
                    _j1: GLint,
                    _j2: GLint,
                ) {
                    panic!("Unable to load glEvalMesh2")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEvalMesh2\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEvalMesh2
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRectsv: unsafe {
                unsafe extern "system" fn __glRectsv(_v1: *mut GLshort, _v2: *mut GLshort) {
                    panic!("Unable to load glRectsv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRectsv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRectsv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRectf: unsafe {
                unsafe extern "system" fn __glRectf(
                    _x1: GLfloat,
                    _y1: GLfloat,
                    _x2: GLfloat,
                    _y2: GLfloat,
                ) {
                    panic!("Unable to load glRectf")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRectf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRectf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glReadPixels: unsafe {
                unsafe extern "system" fn __glReadPixels(
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                    _format: PixelFormat,
                    _type: PixelType,
                    _pixels: *mut GLvoid,
                ) {
                    panic!("Unable to load glReadPixels")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glReadPixels\0");
                let val = _f(cname);
                if val.is_null() {
                    __glReadPixels
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glEvalPoint2: unsafe {
                unsafe extern "system" fn __glEvalPoint2(_i: GLint, _j: GLint) {
                    panic!("Unable to load glEvalPoint2")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEvalPoint2\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEvalPoint2
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMultMatrixf: unsafe {
                unsafe extern "system" fn __glMultMatrixf(_m: *mut GLfloat) {
                    panic!("Unable to load glMultMatrixf")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultMatrixf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultMatrixf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glLineWidth: unsafe {
                unsafe extern "system" fn __glLineWidth(_width: GLfloat) {
                    panic!("Unable to load glLineWidth")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLineWidth\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLineWidth
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glClear: unsafe {
                unsafe extern "system" fn __glClear(_mask: ClearBufferMask) {
                    panic!("Unable to load glClear")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClear\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClear
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBitmap: unsafe {
                unsafe extern "system" fn __glBitmap(
                    _width: GLsizei,
                    _height: GLsizei,
                    _xorig: GLfloat,
                    _yorig: GLfloat,
                    _xmove: GLfloat,
                    _ymove: GLfloat,
                    _bitmap: *mut GLubyte,
                ) {
                    panic!("Unable to load glBitmap")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBitmap\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBitmap
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord4dv: unsafe {
                unsafe extern "system" fn __glTexCoord4dv(_v: *mut GLdouble) {
                    panic!("Unable to load glTexCoord4dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord4dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord4dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glNormal3f: unsafe {
                unsafe extern "system" fn __glNormal3f(_nx: GLfloat, _ny: GLfloat, _nz: GLfloat) {
                    panic!("Unable to load glNormal3f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glNormal3f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glNormal3f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glCullFace: unsafe {
                unsafe extern "system" fn __glCullFace(_mode: CullFaceMode) {
                    panic!("Unable to load glCullFace")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCullFace\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCullFace
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor4f: unsafe {
                unsafe extern "system" fn __glColor4f(
                    _red: GLfloat,
                    _green: GLfloat,
                    _blue: GLfloat,
                    _alpha: GLfloat,
                ) {
                    panic!("Unable to load glColor4f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor4f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor4f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord1iv: unsafe {
                unsafe extern "system" fn __glTexCoord1iv(_v: *mut GLint) {
                    panic!("Unable to load glTexCoord1iv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord1iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord1iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRasterPos4iv: unsafe {
                unsafe extern "system" fn __glRasterPos4iv(_v: *mut GLint) {
                    panic!("Unable to load glRasterPos4iv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRasterPos4iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRasterPos4iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMaterialf: unsafe {
                unsafe extern "system" fn __glMaterialf(
                    _face: MaterialFace,
                    _pname: MaterialParameter,
                    _param: GLfloat,
                ) {
                    panic!("Unable to load glMaterialf")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMaterialf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMaterialf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glFeedbackBuffer: unsafe {
                unsafe extern "system" fn __glFeedbackBuffer(
                    _size: GLsizei,
                    _type: FeedbackType,
                    _buffer: *mut GLfloat,
                ) {
                    panic!("Unable to load glFeedbackBuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFeedbackBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFeedbackBuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetTexLevelParameterfv: unsafe {
                unsafe extern "system" fn __glGetTexLevelParameterfv(
                    _target: TextureTarget,
                    _level: GLint,
                    _pname: GetTextureParameter,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load glGetTexLevelParameterfv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTexLevelParameterfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTexLevelParameterfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDepthMask: unsafe {
                unsafe extern "system" fn __glDepthMask(_flag: Boolean) {
                    panic!("Unable to load glDepthMask")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDepthMask\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDepthMask
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord1s: unsafe {
                unsafe extern "system" fn __glTexCoord1s(_s: GLshort) {
                    panic!("Unable to load glTexCoord1s")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord1s\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord1s
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexImage2D: unsafe {
                unsafe extern "system" fn __glTexImage2D(
                    _target: TextureTarget,
                    _level: GLint,
                    _internalformat: InternalFormat,
                    _width: GLsizei,
                    _height: GLsizei,
                    _border: GLint,
                    _format: PixelFormat,
                    _type: PixelType,
                    _pixels: *mut GLvoid,
                ) {
                    panic!("Unable to load glTexImage2D")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexImage2D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexImage2D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDrawBuffer: unsafe {
                unsafe extern "system" fn __glDrawBuffer(_buf: DrawBufferMode) {
                    panic!("Unable to load glDrawBuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDrawBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDrawBuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor3ui: unsafe {
                unsafe extern "system" fn __glColor3ui(
                    _red: GLuint,
                    _green: GLuint,
                    _blue: GLuint,
                ) {
                    panic!("Unable to load glColor3ui")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor3ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor3ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord3i: unsafe {
                unsafe extern "system" fn __glTexCoord3i(_s: GLint, _t: GLint, _r: GLint) {
                    panic!("Unable to load glTexCoord3i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord3i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord3i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor3uiv: unsafe {
                unsafe extern "system" fn __glColor3uiv(_v: *mut GLuint) {
                    panic!("Unable to load glColor3uiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor3uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor3uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertex2d: unsafe {
                unsafe extern "system" fn __glVertex2d(_x: GLdouble, _y: GLdouble) {
                    panic!("Unable to load glVertex2d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertex2d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertex2d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertex4s: unsafe {
                unsafe extern "system" fn __glVertex4s(
                    _x: GLshort,
                    _y: GLshort,
                    _z: GLshort,
                    _w: GLshort,
                ) {
                    panic!("Unable to load glVertex4s")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertex4s\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertex4s
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glLighti: unsafe {
                unsafe extern "system" fn __glLighti(
                    _light: LightName,
                    _pname: LightParameter,
                    _param: GLint,
                ) {
                    panic!("Unable to load glLighti")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLighti\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLighti
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexEnvfv: unsafe {
                unsafe extern "system" fn __glTexEnvfv(
                    _target: TextureEnvTarget,
                    _pname: TextureEnvParameter,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load glTexEnvfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexEnvfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexEnvfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord3f: unsafe {
                unsafe extern "system" fn __glTexCoord3f(_s: GLfloat, _t: GLfloat, _r: GLfloat) {
                    panic!("Unable to load glTexCoord3f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord3f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord3f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord4sv: unsafe {
                unsafe extern "system" fn __glTexCoord4sv(_v: *mut GLshort) {
                    panic!("Unable to load glTexCoord4sv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord4sv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord4sv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexParameterfv: unsafe {
                unsafe extern "system" fn __glTexParameterfv(
                    _target: TextureTarget,
                    _pname: TextureParameterName,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load glTexParameterfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexParameterfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexParameterfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glFogiv: unsafe {
                unsafe extern "system" fn __glFogiv(_pname: FogParameter, _params: *mut GLint) {
                    panic!("Unable to load glFogiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFogiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFogiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor4ub: unsafe {
                unsafe extern "system" fn __glColor4ub(
                    _red: GLubyte,
                    _green: GLubyte,
                    _blue: GLubyte,
                    _alpha: GLubyte,
                ) {
                    panic!("Unable to load glColor4ub")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor4ub\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor4ub
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord2i: unsafe {
                unsafe extern "system" fn __glTexCoord2i(_s: GLint, _t: GLint) {
                    panic!("Unable to load glTexCoord2i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord2i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord2i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPixelTransferf: unsafe {
                unsafe extern "system" fn __glPixelTransferf(
                    _pname: PixelTransferParameter,
                    _param: GLfloat,
                ) {
                    panic!("Unable to load glPixelTransferf")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPixelTransferf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPixelTransferf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMap1f: unsafe {
                unsafe extern "system" fn __glMap1f(
                    _target: MapTarget,
                    _u1: GLfloat,
                    _u2: GLfloat,
                    _stride: GLint,
                    _order: GLint,
                    _points: *mut GLfloat,
                ) {
                    panic!("Unable to load glMap1f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMap1f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMap1f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertex4sv: unsafe {
                unsafe extern "system" fn __glVertex4sv(_v: *mut GLshort) {
                    panic!("Unable to load glVertex4sv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertex4sv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertex4sv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBegin: unsafe {
                unsafe extern "system" fn __glBegin(_mode: PrimitiveType) {
                    panic!("Unable to load glBegin")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBegin\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBegin
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glLightModeli: unsafe {
                unsafe extern "system" fn __glLightModeli(
                    _pname: LightModelParameter,
                    _param: GLint,
                ) {
                    panic!("Unable to load glLightModeli")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLightModeli\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLightModeli
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexGend: unsafe {
                unsafe extern "system" fn __glTexGend(
                    _coord: TextureCoordName,
                    _pname: TextureGenParameter,
                    _param: GLdouble,
                ) {
                    panic!("Unable to load glTexGend")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexGend\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexGend
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetTexLevelParameteriv: unsafe {
                unsafe extern "system" fn __glGetTexLevelParameteriv(
                    _target: TextureTarget,
                    _level: GLint,
                    _pname: GetTextureParameter,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetTexLevelParameteriv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTexLevelParameteriv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTexLevelParameteriv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glOrtho: unsafe {
                unsafe extern "system" fn __glOrtho(
                    _left: GLdouble,
                    _right: GLdouble,
                    _bottom: GLdouble,
                    _top: GLdouble,
                    _zNear: GLdouble,
                    _zFar: GLdouble,
                ) {
                    panic!("Unable to load glOrtho")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glOrtho\0");
                let val = _f(cname);
                if val.is_null() {
                    __glOrtho
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glIndexfv: unsafe {
                unsafe extern "system" fn __glIndexfv(_c: *mut GLfloat) {
                    panic!("Unable to load glIndexfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIndexfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIndexfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glIndexd: unsafe {
                unsafe extern "system" fn __glIndexd(_c: GLdouble) {
                    panic!("Unable to load glIndexd")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIndexd\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIndexd
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRotated: unsafe {
                unsafe extern "system" fn __glRotated(
                    _angle: GLdouble,
                    _x: GLdouble,
                    _y: GLdouble,
                    _z: GLdouble,
                ) {
                    panic!("Unable to load glRotated")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRotated\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRotated
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glIndexsv: unsafe {
                unsafe extern "system" fn __glIndexsv(_c: *mut GLshort) {
                    panic!("Unable to load glIndexsv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIndexsv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIndexsv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexParameteriv: unsafe {
                unsafe extern "system" fn __glTexParameteriv(
                    _target: TextureTarget,
                    _pname: TextureParameterName,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glTexParameteriv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexParameteriv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexParameteriv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glFrontFace: unsafe {
                unsafe extern "system" fn __glFrontFace(_mode: FrontFaceDirection) {
                    panic!("Unable to load glFrontFace")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFrontFace\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFrontFace
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertex2f: unsafe {
                unsafe extern "system" fn __glVertex2f(_x: GLfloat, _y: GLfloat) {
                    panic!("Unable to load glVertex2f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertex2f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertex2f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glNormal3i: unsafe {
                unsafe extern "system" fn __glNormal3i(_nx: GLint, _ny: GLint, _nz: GLint) {
                    panic!("Unable to load glNormal3i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glNormal3i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glNormal3i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPassThrough: unsafe {
                unsafe extern "system" fn __glPassThrough(_token: GLfloat) {
                    panic!("Unable to load glPassThrough")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPassThrough\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPassThrough
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor4ubv: unsafe {
                unsafe extern "system" fn __glColor4ubv(_v: *mut GLubyte) {
                    panic!("Unable to load glColor4ubv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor4ubv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor4ubv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord4s: unsafe {
                unsafe extern "system" fn __glTexCoord4s(
                    _s: GLshort,
                    _t: GLshort,
                    _r: GLshort,
                    _q: GLshort,
                ) {
                    panic!("Unable to load glTexCoord4s")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord4s\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord4s
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMapGrid1d: unsafe {
                unsafe extern "system" fn __glMapGrid1d(_un: GLint, _u1: GLdouble, _u2: GLdouble) {
                    panic!("Unable to load glMapGrid1d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMapGrid1d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMapGrid1d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexGenfv: unsafe {
                unsafe extern "system" fn __glTexGenfv(
                    _coord: TextureCoordName,
                    _pname: TextureGenParameter,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load glTexGenfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexGenfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexGenfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor3f: unsafe {
                unsafe extern "system" fn __glColor3f(
                    _red: GLfloat,
                    _green: GLfloat,
                    _blue: GLfloat,
                ) {
                    panic!("Unable to load glColor3f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor3f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor3f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRasterPos3dv: unsafe {
                unsafe extern "system" fn __glRasterPos3dv(_v: *mut GLdouble) {
                    panic!("Unable to load glRasterPos3dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRasterPos3dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRasterPos3dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glLineStipple: unsafe {
                unsafe extern "system" fn __glLineStipple(_factor: GLint, _pattern: GLushort) {
                    panic!("Unable to load glLineStipple")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLineStipple\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLineStipple
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRasterPos3i: unsafe {
                unsafe extern "system" fn __glRasterPos3i(_x: GLint, _y: GLint, _z: GLint) {
                    panic!("Unable to load glRasterPos3i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRasterPos3i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRasterPos3i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertex3d: unsafe {
                unsafe extern "system" fn __glVertex3d(_x: GLdouble, _y: GLdouble, _z: GLdouble) {
                    panic!("Unable to load glVertex3d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertex3d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertex3d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glHint: unsafe {
                unsafe extern "system" fn __glHint(_target: HintTarget, _mode: HintMode) {
                    panic!("Unable to load glHint")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glHint\0");
                let val = _f(cname);
                if val.is_null() {
                    __glHint
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor4i: unsafe {
                unsafe extern "system" fn __glColor4i(
                    _red: GLint,
                    _green: GLint,
                    _blue: GLint,
                    _alpha: GLint,
                ) {
                    panic!("Unable to load glColor4i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor4i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor4i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glVertex2sv: unsafe {
                unsafe extern "system" fn __glVertex2sv(_v: *mut GLshort) {
                    panic!("Unable to load glVertex2sv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertex2sv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertex2sv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPointSize: unsafe {
                unsafe extern "system" fn __glPointSize(_size: GLfloat) {
                    panic!("Unable to load glPointSize")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPointSize\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPointSize
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMateriali: unsafe {
                unsafe extern "system" fn __glMateriali(
                    _face: MaterialFace,
                    _pname: MaterialParameter,
                    _param: GLint,
                ) {
                    panic!("Unable to load glMateriali")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMateriali\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMateriali
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord2f: unsafe {
                unsafe extern "system" fn __glTexCoord2f(_s: GLfloat, _t: GLfloat) {
                    panic!("Unable to load glTexCoord2f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord2f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord2f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMatrixMode: unsafe {
                unsafe extern "system" fn __glMatrixMode(_mode: MatrixMode) {
                    panic!("Unable to load glMatrixMode")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMatrixMode\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMatrixMode
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord4f: unsafe {
                unsafe extern "system" fn __glTexCoord4f(
                    _s: GLfloat,
                    _t: GLfloat,
                    _r: GLfloat,
                    _q: GLfloat,
                ) {
                    panic!("Unable to load glTexCoord4f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord4f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord4f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRasterPos4i: unsafe {
                unsafe extern "system" fn __glRasterPos4i(
                    _x: GLint,
                    _y: GLint,
                    _z: GLint,
                    _w: GLint,
                ) {
                    panic!("Unable to load glRasterPos4i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRasterPos4i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRasterPos4i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glEvalCoord2fv: unsafe {
                unsafe extern "system" fn __glEvalCoord2fv(_u: *mut GLfloat) {
                    panic!("Unable to load glEvalCoord2fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEvalCoord2fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEvalCoord2fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetError: unsafe {
                unsafe extern "system" fn __glGetError() -> GLenum {
                    panic!("Unable to load glGetError")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetError\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetError
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glNormal3bv: unsafe {
                unsafe extern "system" fn __glNormal3bv(_v: *mut GLbyte) {
                    panic!("Unable to load glNormal3bv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glNormal3bv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glNormal3bv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glStencilMask: unsafe {
                unsafe extern "system" fn __glStencilMask(_mask: GLuint) {
                    panic!("Unable to load glStencilMask")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glStencilMask\0");
                let val = _f(cname);
                if val.is_null() {
                    __glStencilMask
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glAccum: unsafe {
                unsafe extern "system" fn __glAccum(_op: AccumOp, _value: GLfloat) {
                    panic!("Unable to load glAccum")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glAccum\0");
                let val = _f(cname);
                if val.is_null() {
                    __glAccum
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetBooleanv: unsafe {
                unsafe extern "system" fn __glGetBooleanv(_pname: GetPName, _data: *mut Boolean) {
                    panic!("Unable to load glGetBooleanv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetBooleanv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetBooleanv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glEvalCoord2dv: unsafe {
                unsafe extern "system" fn __glEvalCoord2dv(_u: *mut GLdouble) {
                    panic!("Unable to load glEvalCoord2dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEvalCoord2dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEvalCoord2dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord3s: unsafe {
                unsafe extern "system" fn __glTexCoord3s(_s: GLshort, _t: GLshort, _r: GLshort) {
                    panic!("Unable to load glTexCoord3s")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord3s\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord3s
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexEnviv: unsafe {
                unsafe extern "system" fn __glTexEnviv(
                    _target: TextureEnvTarget,
                    _pname: TextureEnvParameter,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glTexEnviv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexEnviv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexEnviv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPopName: unsafe {
                unsafe extern "system" fn __glPopName() {
                    panic!("Unable to load glPopName")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPopName\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPopName
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor4s: unsafe {
                unsafe extern "system" fn __glColor4s(
                    _red: GLshort,
                    _green: GLshort,
                    _blue: GLshort,
                    _alpha: GLshort,
                ) {
                    panic!("Unable to load glColor4s")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor4s\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor4s
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glNormal3d: unsafe {
                unsafe extern "system" fn __glNormal3d(
                    _nx: GLdouble,
                    _ny: GLdouble,
                    _nz: GLdouble,
                ) {
                    panic!("Unable to load glNormal3d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glNormal3d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glNormal3d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetTexImage: unsafe {
                unsafe extern "system" fn __glGetTexImage(
                    _target: TextureTarget,
                    _level: GLint,
                    _format: PixelFormat,
                    _type: PixelType,
                    _pixels: *mut GLvoid,
                ) {
                    panic!("Unable to load glGetTexImage")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTexImage\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTexImage
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor4b: unsafe {
                unsafe extern "system" fn __glColor4b(
                    _red: GLbyte,
                    _green: GLbyte,
                    _blue: GLbyte,
                    _alpha: GLbyte,
                ) {
                    panic!("Unable to load glColor4b")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor4b\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor4b
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glColor3ub: unsafe {
                unsafe extern "system" fn __glColor3ub(
                    _red: GLubyte,
                    _green: GLubyte,
                    _blue: GLubyte,
                ) {
                    panic!("Unable to load glColor3ub")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor3ub\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor3ub
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord3dv: unsafe {
                unsafe extern "system" fn __glTexCoord3dv(_v: *mut GLdouble) {
                    panic!("Unable to load glTexCoord3dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord3dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord3dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glEvalCoord1dv: unsafe {
                unsafe extern "system" fn __glEvalCoord1dv(_u: *mut GLdouble) {
                    panic!("Unable to load glEvalCoord1dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEvalCoord1dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEvalCoord1dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPixelMapfv: unsafe {
                unsafe extern "system" fn __glPixelMapfv(
                    _map: PixelMap,
                    _mapsize: GLsizei,
                    _values: *mut GLfloat,
                ) {
                    panic!("Unable to load glPixelMapfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPixelMapfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPixelMapfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRasterPos4dv: unsafe {
                unsafe extern "system" fn __glRasterPos4dv(_v: *mut GLdouble) {
                    panic!("Unable to load glRasterPos4dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRasterPos4dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRasterPos4dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glNormal3s: unsafe {
                unsafe extern "system" fn __glNormal3s(_nx: GLshort, _ny: GLshort, _nz: GLshort) {
                    panic!("Unable to load glNormal3s")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glNormal3s\0");
                let val = _f(cname);
                if val.is_null() {
                    __glNormal3s
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glRasterPos2d: unsafe {
                unsafe extern "system" fn __glRasterPos2d(_x: GLdouble, _y: GLdouble) {
                    panic!("Unable to load glRasterPos2d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRasterPos2d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRasterPos2d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexGeni: unsafe {
                unsafe extern "system" fn __glTexGeni(
                    _coord: TextureCoordName,
                    _pname: TextureGenParameter,
                    _param: GLint,
                ) {
                    panic!("Unable to load glTexGeni")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexGeni\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexGeni
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glTexCoord4iv: unsafe {
                unsafe extern "system" fn __glTexCoord4iv(_v: *mut GLint) {
                    panic!("Unable to load glTexCoord4iv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoord4iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoord4iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[derive(Clone)]
pub struct EntryFnGL40 {
    pub glGetQueryIndexediv: crate::gl::command::PFN_glGetQueryIndexediv,
    pub glGetSubroutineUniformLocation: crate::gl::command::PFN_glGetSubroutineUniformLocation,
    pub glGetSubroutineIndex: crate::gl::command::PFN_glGetSubroutineIndex,
    pub glGetActiveSubroutineName: crate::gl::command::PFN_glGetActiveSubroutineName,
    pub glBeginQueryIndexed: crate::gl::command::PFN_glBeginQueryIndexed,
    pub glUniform1dv: crate::gl::command::PFN_glUniform1dv,
    pub glGetActiveSubroutineUniformiv: crate::gl::command::PFN_glGetActiveSubroutineUniformiv,
    pub glEndQueryIndexed: crate::gl::command::PFN_glEndQueryIndexed,
    pub glUniform3d: crate::gl::command::PFN_glUniform3d,
    pub glPatchParameteri: crate::gl::command::PFN_glPatchParameteri,
    pub glBlendFunci: crate::gl::command::PFN_glBlendFunci,
    pub glBlendEquationi: crate::gl::command::PFN_glBlendEquationi,
    pub glUniform4d: crate::gl::command::PFN_glUniform4d,
    pub glUniformMatrix2x3dv: crate::gl::command::PFN_glUniformMatrix2x3dv,
    pub glResumeTransformFeedback: crate::gl::command::PFN_glResumeTransformFeedback,
    pub glBindTransformFeedback: crate::gl::command::PFN_glBindTransformFeedback,
    pub glUniformMatrix3x4dv: crate::gl::command::PFN_glUniformMatrix3x4dv,
    pub glPauseTransformFeedback: crate::gl::command::PFN_glPauseTransformFeedback,
    pub glDeleteTransformFeedbacks: crate::gl::command::PFN_glDeleteTransformFeedbacks,
    pub glUniform4dv: crate::gl::command::PFN_glUniform4dv,
    pub glDrawElementsIndirect: crate::gl::command::PFN_glDrawElementsIndirect,
    pub glBlendFuncSeparatei: crate::gl::command::PFN_glBlendFuncSeparatei,
    pub glUniformMatrix2x4dv: crate::gl::command::PFN_glUniformMatrix2x4dv,
    pub glUniform2dv: crate::gl::command::PFN_glUniform2dv,
    pub glDrawArraysIndirect: crate::gl::command::PFN_glDrawArraysIndirect,
    pub glGetUniformSubroutineuiv: crate::gl::command::PFN_glGetUniformSubroutineuiv,
    pub glUniform1d: crate::gl::command::PFN_glUniform1d,
    pub glUniform2d: crate::gl::command::PFN_glUniform2d,
    pub glMinSampleShading: crate::gl::command::PFN_glMinSampleShading,
    pub glUniform3dv: crate::gl::command::PFN_glUniform3dv,
    pub glGetProgramStageiv: crate::gl::command::PFN_glGetProgramStageiv,
    pub glGenTransformFeedbacks: crate::gl::command::PFN_glGenTransformFeedbacks,
    pub glBlendEquationSeparatei: crate::gl::command::PFN_glBlendEquationSeparatei,
    pub glUniformMatrix3dv: crate::gl::command::PFN_glUniformMatrix3dv,
    pub glUniformMatrix4dv: crate::gl::command::PFN_glUniformMatrix4dv,
    pub glUniformMatrix3x2dv: crate::gl::command::PFN_glUniformMatrix3x2dv,
    pub glUniformMatrix4x3dv: crate::gl::command::PFN_glUniformMatrix4x3dv,
    pub glGetUniformdv: crate::gl::command::PFN_glGetUniformdv,
    pub glGetActiveSubroutineUniformName: crate::gl::command::PFN_glGetActiveSubroutineUniformName,
    pub glUniformSubroutinesuiv: crate::gl::command::PFN_glUniformSubroutinesuiv,
    pub glIsTransformFeedback: crate::gl::command::PFN_glIsTransformFeedback,
    pub glUniformMatrix4x2dv: crate::gl::command::PFN_glUniformMatrix4x2dv,
    pub glPatchParameterfv: crate::gl::command::PFN_glPatchParameterfv,
    pub glUniformMatrix2dv: crate::gl::command::PFN_glUniformMatrix2dv,
    pub glDrawTransformFeedback: crate::gl::command::PFN_glDrawTransformFeedback,
    pub glDrawTransformFeedbackStream: crate::gl::command::PFN_glDrawTransformFeedbackStream,
}
impl EntryFnGL40 {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            glGetQueryIndexediv: unsafe {
                unsafe extern "system" fn __glGetQueryIndexediv(
                    _target: QueryTarget,
                    _index: GLuint,
                    _pname: QueryParameterName,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetQueryIndexediv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetQueryIndexediv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetQueryIndexediv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetSubroutineUniformLocation: unsafe {
                unsafe extern "system" fn __glGetSubroutineUniformLocation(
                    _program: GLuint,
                    _shadertype: ShaderType,
                    _name: GLchar,
                ) -> GLint {
                    panic!("Unable to load glGetSubroutineUniformLocation")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetSubroutineUniformLocation\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetSubroutineUniformLocation
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetSubroutineIndex: unsafe {
                unsafe extern "system" fn __glGetSubroutineIndex(
                    _program: GLuint,
                    _shadertype: ShaderType,
                    _name: GLchar,
                ) -> GLuint {
                    panic!("Unable to load glGetSubroutineIndex")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetSubroutineIndex\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetSubroutineIndex
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetActiveSubroutineName: unsafe {
                unsafe extern "system" fn __glGetActiveSubroutineName(
                    _program: GLuint,
                    _shadertype: ShaderType,
                    _index: GLuint,
                    _bufSize: GLsizei,
                    _length: *mut GLsizei,
                    _name: *mut GLchar,
                ) {
                    panic!("Unable to load glGetActiveSubroutineName")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetActiveSubroutineName\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetActiveSubroutineName
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBeginQueryIndexed: unsafe {
                unsafe extern "system" fn __glBeginQueryIndexed(
                    _target: QueryTarget,
                    _index: GLuint,
                    _id: GLuint,
                ) {
                    panic!("Unable to load glBeginQueryIndexed")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBeginQueryIndexed\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBeginQueryIndexed
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform1dv: unsafe {
                unsafe extern "system" fn __glUniform1dv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLdouble,
                ) {
                    panic!("Unable to load glUniform1dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform1dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform1dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetActiveSubroutineUniformiv: unsafe {
                unsafe extern "system" fn __glGetActiveSubroutineUniformiv(
                    _program: GLuint,
                    _shadertype: ShaderType,
                    _index: GLuint,
                    _pname: SubroutineParameterName,
                    _values: *mut GLint,
                ) {
                    panic!("Unable to load glGetActiveSubroutineUniformiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetActiveSubroutineUniformiv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetActiveSubroutineUniformiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glEndQueryIndexed: unsafe {
                unsafe extern "system" fn __glEndQueryIndexed(
                    _target: QueryTarget,
                    _index: GLuint,
                ) {
                    panic!("Unable to load glEndQueryIndexed")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEndQueryIndexed\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEndQueryIndexed
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform3d: unsafe {
                unsafe extern "system" fn __glUniform3d(
                    _location: GLint,
                    _x: GLdouble,
                    _y: GLdouble,
                    _z: GLdouble,
                ) {
                    panic!("Unable to load glUniform3d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform3d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform3d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPatchParameteri: unsafe {
                unsafe extern "system" fn __glPatchParameteri(
                    _pname: PatchParameterName,
                    _value: GLint,
                ) {
                    panic!("Unable to load glPatchParameteri")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPatchParameteri\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPatchParameteri
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBlendFunci: unsafe {
                unsafe extern "system" fn __glBlendFunci(
                    _buf: GLuint,
                    _src: BlendingFactor,
                    _dst: BlendingFactor,
                ) {
                    panic!("Unable to load glBlendFunci")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBlendFunci\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBlendFunci
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBlendEquationi: unsafe {
                unsafe extern "system" fn __glBlendEquationi(
                    _buf: GLuint,
                    _mode: BlendEquationModeEXT,
                ) {
                    panic!("Unable to load glBlendEquationi")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBlendEquationi\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBlendEquationi
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform4d: unsafe {
                unsafe extern "system" fn __glUniform4d(
                    _location: GLint,
                    _x: GLdouble,
                    _y: GLdouble,
                    _z: GLdouble,
                    _w: GLdouble,
                ) {
                    panic!("Unable to load glUniform4d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform4d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform4d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniformMatrix2x3dv: unsafe {
                unsafe extern "system" fn __glUniformMatrix2x3dv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLdouble,
                ) {
                    panic!("Unable to load glUniformMatrix2x3dv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix2x3dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformMatrix2x3dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glResumeTransformFeedback: unsafe {
                unsafe extern "system" fn __glResumeTransformFeedback() {
                    panic!("Unable to load glResumeTransformFeedback")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glResumeTransformFeedback\0");
                let val = _f(cname);
                if val.is_null() {
                    __glResumeTransformFeedback
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBindTransformFeedback: unsafe {
                unsafe extern "system" fn __glBindTransformFeedback(
                    _target: BindTransformFeedbackTarget,
                    _id: GLuint,
                ) {
                    panic!("Unable to load glBindTransformFeedback")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindTransformFeedback\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindTransformFeedback
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniformMatrix3x4dv: unsafe {
                unsafe extern "system" fn __glUniformMatrix3x4dv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLdouble,
                ) {
                    panic!("Unable to load glUniformMatrix3x4dv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix3x4dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformMatrix3x4dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPauseTransformFeedback: unsafe {
                unsafe extern "system" fn __glPauseTransformFeedback() {
                    panic!("Unable to load glPauseTransformFeedback")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPauseTransformFeedback\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPauseTransformFeedback
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDeleteTransformFeedbacks: unsafe {
                unsafe extern "system" fn __glDeleteTransformFeedbacks(
                    _n: GLsizei,
                    _ids: *mut GLuint,
                ) {
                    panic!("Unable to load glDeleteTransformFeedbacks")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glDeleteTransformFeedbacks\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glDeleteTransformFeedbacks
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform4dv: unsafe {
                unsafe extern "system" fn __glUniform4dv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLdouble,
                ) {
                    panic!("Unable to load glUniform4dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform4dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform4dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDrawElementsIndirect: unsafe {
                unsafe extern "system" fn __glDrawElementsIndirect(
                    _mode: PrimitiveType,
                    _type: DrawElementsType,
                    _indirect: GLvoid,
                ) {
                    panic!("Unable to load glDrawElementsIndirect")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDrawElementsIndirect\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDrawElementsIndirect
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBlendFuncSeparatei: unsafe {
                unsafe extern "system" fn __glBlendFuncSeparatei(
                    _buf: GLuint,
                    _srcRGB: BlendingFactor,
                    _dstRGB: BlendingFactor,
                    _srcAlpha: BlendingFactor,
                    _dstAlpha: BlendingFactor,
                ) {
                    panic!("Unable to load glBlendFuncSeparatei")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBlendFuncSeparatei\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBlendFuncSeparatei
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniformMatrix2x4dv: unsafe {
                unsafe extern "system" fn __glUniformMatrix2x4dv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLdouble,
                ) {
                    panic!("Unable to load glUniformMatrix2x4dv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix2x4dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformMatrix2x4dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform2dv: unsafe {
                unsafe extern "system" fn __glUniform2dv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLdouble,
                ) {
                    panic!("Unable to load glUniform2dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform2dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform2dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDrawArraysIndirect: unsafe {
                unsafe extern "system" fn __glDrawArraysIndirect(
                    _mode: PrimitiveType,
                    _indirect: GLvoid,
                ) {
                    panic!("Unable to load glDrawArraysIndirect")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDrawArraysIndirect\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDrawArraysIndirect
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetUniformSubroutineuiv: unsafe {
                unsafe extern "system" fn __glGetUniformSubroutineuiv(
                    _shadertype: ShaderType,
                    _location: GLint,
                    _params: *mut GLuint,
                ) {
                    panic!("Unable to load glGetUniformSubroutineuiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetUniformSubroutineuiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetUniformSubroutineuiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform1d: unsafe {
                unsafe extern "system" fn __glUniform1d(_location: GLint, _x: GLdouble) {
                    panic!("Unable to load glUniform1d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform1d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform1d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform2d: unsafe {
                unsafe extern "system" fn __glUniform2d(
                    _location: GLint,
                    _x: GLdouble,
                    _y: GLdouble,
                ) {
                    panic!("Unable to load glUniform2d")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform2d\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform2d
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glMinSampleShading: unsafe {
                unsafe extern "system" fn __glMinSampleShading(_value: GLfloat) {
                    panic!("Unable to load glMinSampleShading")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMinSampleShading\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMinSampleShading
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniform3dv: unsafe {
                unsafe extern "system" fn __glUniform3dv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *mut GLdouble,
                ) {
                    panic!("Unable to load glUniform3dv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform3dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform3dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetProgramStageiv: unsafe {
                unsafe extern "system" fn __glGetProgramStageiv(
                    _program: GLuint,
                    _shadertype: ShaderType,
                    _pname: ProgramStagePName,
                    _values: *mut GLint,
                ) {
                    panic!("Unable to load glGetProgramStageiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetProgramStageiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetProgramStageiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGenTransformFeedbacks: unsafe {
                unsafe extern "system" fn __glGenTransformFeedbacks(
                    _n: GLsizei,
                    _ids: *mut GLuint,
                ) {
                    panic!("Unable to load glGenTransformFeedbacks")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGenTransformFeedbacks\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGenTransformFeedbacks
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glBlendEquationSeparatei: unsafe {
                unsafe extern "system" fn __glBlendEquationSeparatei(
                    _buf: GLuint,
                    _modeRGB: BlendEquationModeEXT,
                    _modeAlpha: BlendEquationModeEXT,
                ) {
                    panic!("Unable to load glBlendEquationSeparatei")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBlendEquationSeparatei\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBlendEquationSeparatei
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniformMatrix3dv: unsafe {
                unsafe extern "system" fn __glUniformMatrix3dv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLdouble,
                ) {
                    panic!("Unable to load glUniformMatrix3dv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix3dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformMatrix3dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniformMatrix4dv: unsafe {
                unsafe extern "system" fn __glUniformMatrix4dv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLdouble,
                ) {
                    panic!("Unable to load glUniformMatrix4dv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix4dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformMatrix4dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniformMatrix3x2dv: unsafe {
                unsafe extern "system" fn __glUniformMatrix3x2dv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLdouble,
                ) {
                    panic!("Unable to load glUniformMatrix3x2dv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix3x2dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformMatrix3x2dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniformMatrix4x3dv: unsafe {
                unsafe extern "system" fn __glUniformMatrix4x3dv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLdouble,
                ) {
                    panic!("Unable to load glUniformMatrix4x3dv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix4x3dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformMatrix4x3dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetUniformdv: unsafe {
                unsafe extern "system" fn __glGetUniformdv(
                    _program: GLuint,
                    _location: GLint,
                    _params: *mut GLdouble,
                ) {
                    panic!("Unable to load glGetUniformdv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetUniformdv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetUniformdv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glGetActiveSubroutineUniformName: unsafe {
                unsafe extern "system" fn __glGetActiveSubroutineUniformName(
                    _program: GLuint,
                    _shadertype: ShaderType,
                    _index: GLuint,
                    _bufSize: GLsizei,
                    _length: *mut GLsizei,
                    _name: *mut GLchar,
                ) {
                    panic!("Unable to load glGetActiveSubroutineUniformName")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetActiveSubroutineUniformName\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetActiveSubroutineUniformName
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniformSubroutinesuiv: unsafe {
                unsafe extern "system" fn __glUniformSubroutinesuiv(
                    _shadertype: ShaderType,
                    _count: GLsizei,
                    _indices: *mut GLuint,
                ) {
                    panic!("Unable to load glUniformSubroutinesuiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformSubroutinesuiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformSubroutinesuiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glIsTransformFeedback: unsafe {
                unsafe extern "system" fn __glIsTransformFeedback(_id: GLuint) -> GLboolean {
                    panic!("Unable to load glIsTransformFeedback")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIsTransformFeedback\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIsTransformFeedback
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniformMatrix4x2dv: unsafe {
                unsafe extern "system" fn __glUniformMatrix4x2dv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLdouble,
                ) {
                    panic!("Unable to load glUniformMatrix4x2dv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix4x2dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformMatrix4x2dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glPatchParameterfv: unsafe {
                unsafe extern "system" fn __glPatchParameterfv(
                    _pname: PatchParameterName,
                    _values: *mut GLfloat,
                ) {
                    panic!("Unable to load glPatchParameterfv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPatchParameterfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPatchParameterfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glUniformMatrix2dv: unsafe {
                unsafe extern "system" fn __glUniformMatrix2dv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: Boolean,
                    _value: *mut GLdouble,
                ) {
                    panic!("Unable to load glUniformMatrix2dv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix2dv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformMatrix2dv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDrawTransformFeedback: unsafe {
                unsafe extern "system" fn __glDrawTransformFeedback(
                    _mode: PrimitiveType,
                    _id: GLuint,
                ) {
                    panic!("Unable to load glDrawTransformFeedback")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDrawTransformFeedback\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDrawTransformFeedback
                } else {
                    ::std::mem::transmute(val)
                }
            },
            glDrawTransformFeedbackStream: unsafe {
                unsafe extern "system" fn __glDrawTransformFeedbackStream(
                    _mode: PrimitiveType,
                    _id: GLuint,
                    _stream: GLuint,
                ) {
                    panic!("Unable to load glDrawTransformFeedbackStream")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glDrawTransformFeedbackStream\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glDrawTransformFeedbackStream
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
