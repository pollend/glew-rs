use crate::gl::context::LoadEntryPoint;
use crate::gl::enums::{PixelFormat, PixelType};
use crate::gl::feature::EntryFnGL45;
use crate::types::{GLint, GLsizei, GLuint, GLvoid};
use libloading::Library;
use std::ffi::c_void;
use std::ptr;
use std::sync::Arc;

pub trait GL45 {
    fn entry_gl45(&self) -> &crate::gl::feature::EntryFnGL45;

    // fn glTextureSubImage3D(
    //     &self,
    //     _texture: GLuint,
    //     _level: GLint,
    //     _xoffset: GLint,
    //     _yoffset: GLint,
    //     _zoffset: GLint,
    //     _width: GLsizei,
    //     _height: GLsizei,
    //     _depth: GLsizei,
    //     _format: PixelFormat,
    //     _type: PixelType,
    //     _pixels: GLvoid,
    // ) {
    //     unsafe {
    //         (self.entry_gl45().glTextureSubImage3D)(
    //             _texture, _level, _xoffset, _yoffset, _zoffset, _width, _height, _depth, _format,
    //             _type, _pixels,
    //         )
    //     }
    // }
}

// #[derive(Clone)]
// pub struct GL45 {
//     supported: bool,
//     entry_gl45: crate::gl::feature::EntryFnGL45,
//     // #[cfg(feature = "loaded")]
//     // _lib_guard: Option<Arc<Library>>,
// }

//
// impl GL45 {
//     pub unsafe fn load<F>(mut _f: F) -> GL45
//         where F: FnMut(&::std::ffi::CStr) -> *const c_void{
//         GL45 {
//             supported: false,
//             entry_gl45: EntryFnGL45::load(_f)
//         }
//     }
//
//     pub fn empty() -> GL45{
//         let empty_load = |str: &std::ffi::CStr| -> *const c_void {
//             ptr::null()
//         };
//         GL45 {
//             supported: false,
//             entry_gl45: EntryFnGL45::load(empty_load)
//         }
//     }
//
// }
