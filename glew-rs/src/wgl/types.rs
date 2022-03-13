use std::ffi::c_void;
use std::os::raw::c_ulong;

pub type HPVIDEODEV = *mut c_void;
pub type HPBUFFERARB = *mut c_void;
pub type HANDLE = *mut c_void;
pub type DWORD = c_ulong;
pub type LPGLYPHMETRICSFLOAT = *mut c_void;
pub type LAYERPLANEDESCRIPTOR = *mut c_void;
pub type HGPUNV = *mut c_void;
pub type PIXELFORMATDESCRIPTOR = *mut c_void;
pub type HVIDEOINPUTDEVICENV = *mut c_void;
pub type HPBUFFEREXT = *mut c_void;
pub type HGLRC = *mut c_void;
pub type COLORREF = *mut c_void;
pub type LPCSTR = *mut c_void;
pub type HVIDEOOUTPUTDEVICENV = *mut c_void;
pub type HENHMETAFILE = *mut c_void;
pub type PGPU_DEVICE = *mut c_void;
