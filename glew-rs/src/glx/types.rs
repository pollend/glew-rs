use std::ffi::c_void;

// x window types
pub type Display = *mut c_void;
pub type VLServer = *mut c_void;
pub type VLPath = *mut c_void;
pub type GLXContext = *mut c_void;

// SGIX
pub type GLXVideoSourceSGIX = *mut c_void;
pub type GLXPbufferSGIX = *mut c_void;
pub type GLXFBConfigSGIX = *mut c_void;
pub type GLXHyperpipeConfigSGIX = *mut c_void;
pub type GLXHyperpipeNetworkSGIX = *mut c_void;

pub type XVisualInfo = *mut c_void;
pub type GLXFBConfig = *mut c_void;
pub type GLXDrawable = *mut c_void;
pub type Pixmap = *mut c_void;
pub type GLXVideoCaptureDeviceNV = *mut c_void;
pub type GLXPbuffer = *mut c_void;
pub type Colormap = *mut c_void;
pub type Window = *mut c_void;
pub type VLNode = *mut c_void;
pub type GLXVideoDeviceNV = *mut c_void;
pub type GLXWindow = *mut c_void;
pub type GLXContextID = *mut c_void;
pub type GLXPixmap = *mut c_void;
pub type Font = *mut c_void;
pub type DMbuffer = *mut c_void;
pub type DMparams = *mut c_void;
pub type __GLXextFuncPtr = *mut c_void;

pub type Status =
    fn(dpy: Display, overlay: Window, underlay: Window, pTransparentIndex: *const u64);
