use crate::glx;
use crate::types::*;
use glx::bitflags::*;
use glx::enums::*;
use glx::types::*;
use std::ffi::c_void;
use std::fmt;
#[allow(non_camel_case_types)]
pub type PFN_glXCushionSGI =
    unsafe extern "system" fn(_dpy: *mut Display, _window: Window, _cushion: f32);
#[allow(non_camel_case_types)]
pub type PFN_glXQueryMaxSwapBarriersSGIX = unsafe extern "system" fn(
    _dpy: *mut Display,
    _screen: std::os::raw::c_int,
    _max: *mut std::os::raw::c_int,
) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXGetMscRateOML = unsafe extern "system" fn(
    _dpy: *mut Display,
    _drawable: GLXDrawable,
    _numerator: *mut u32,
    _denominator: *mut u32,
) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXQueryRendererIntegerMESA = unsafe extern "system" fn(
    _dpy: *mut Display,
    _screen: std::os::raw::c_int,
    _renderer: std::os::raw::c_int,
    _attribute: std::os::raw::c_int,
    _value: *mut std::os::raw::c_uint,
) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXGetSyncValuesOML = unsafe extern "system" fn(
    _dpy: *mut Display,
    _drawable: GLXDrawable,
    _ust: *mut u64,
    _msc: *mut u64,
    _sbc: *mut u64,
) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXGetGPUInfoAMD = unsafe extern "system" fn(
    _id: std::os::raw::c_uint,
    _property: std::os::raw::c_int,
    _dataType: GLenum,
    _size: std::os::raw::c_uint,
    _data: *mut std::os::raw::c_void,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXGetVideoSyncSGI =
    unsafe extern "system" fn(_count: *mut std::os::raw::c_uint) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXCreateGLXPixmapWithConfigSGIX = unsafe extern "system" fn(
    _dpy: *mut Display,
    _config: GLXFBConfigSGIX,
    _pixmap: Pixmap,
) -> GLXPixmap;
#[allow(non_camel_case_types)]
pub type PFN_glXGetCurrentContext = unsafe extern "system" fn() -> GLXContext;
#[allow(non_camel_case_types)]
pub type PFN_glXGetProcAddressARB =
    unsafe extern "system" fn(_procName: *const GLubyte) -> __GLXextFuncPtr;
#[allow(non_camel_case_types)]
pub type PFN_glXGetSelectedEvent = unsafe extern "system" fn(
    _dpy: *mut Display,
    _draw: GLXDrawable,
    _event_mask: *mut std::os::raw::c_ulong,
);
#[allow(non_camel_case_types)]
pub type PFN_glXGetVideoInfoNV = unsafe extern "system" fn(
    _dpy: *mut Display,
    _screen: std::os::raw::c_int,
    _VideoDevice: GLXVideoDeviceNV,
    _pulCounterOutputPbuffer: *mut std::os::raw::c_ulong,
    _pulCounterOutputVideo: *mut std::os::raw::c_ulong,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXQueryChannelRectSGIX = unsafe extern "system" fn(
    _display: *mut Display,
    _screen: std::os::raw::c_int,
    _channel: std::os::raw::c_int,
    _dx: *mut std::os::raw::c_int,
    _dy: *mut std::os::raw::c_int,
    _dw: *mut std::os::raw::c_int,
    _dh: *mut std::os::raw::c_int,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXReleaseVideoDeviceNV = unsafe extern "system" fn(
    _dpy: *mut Display,
    _screen: std::os::raw::c_int,
    _VideoDevice: GLXVideoDeviceNV,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXQueryCurrentRendererStringMESA =
    unsafe extern "system" fn(_attribute: std::os::raw::c_int) -> *const char;
#[allow(non_camel_case_types)]
pub type PFN_glXChannelRectSGIX = unsafe extern "system" fn(
    _display: *mut Display,
    _screen: std::os::raw::c_int,
    _channel: std::os::raw::c_int,
    _x: std::os::raw::c_int,
    _y: std::os::raw::c_int,
    _w: std::os::raw::c_int,
    _h: std::os::raw::c_int,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXSwapIntervalEXT = unsafe extern "system" fn(
    _dpy: *mut Display,
    _drawable: GLXDrawable,
    _interval: std::os::raw::c_int,
);
#[allow(non_camel_case_types)]
pub type PFN_glXSelectEvent = unsafe extern "system" fn(
    _dpy: *mut Display,
    _draw: GLXDrawable,
    _event_mask: std::os::raw::c_ulong,
);
#[allow(non_camel_case_types)]
pub type PFN_glXMakeCurrent =
    unsafe extern "system" fn(_dpy: *mut Display, _drawable: GLXDrawable, _ctx: GLXContext) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXGetCurrentDisplayEXT = unsafe extern "system" fn() -> *mut Display;
#[allow(non_camel_case_types)]
pub type PFN_glXGetConfig = unsafe extern "system" fn(
    _dpy: *mut Display,
    _visual: *mut XVisualInfo,
    _attrib: std::os::raw::c_int,
    _value: *mut std::os::raw::c_int,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXLockVideoCaptureDeviceNV =
    unsafe extern "system" fn(_dpy: *mut Display, _device: GLXVideoCaptureDeviceNV);
#[allow(non_camel_case_types)]
pub type PFN_glXCopyBufferSubDataNV = unsafe extern "system" fn(
    _dpy: *mut Display,
    _readCtx: GLXContext,
    _writeCtx: GLXContext,
    _readTarget: GLenum,
    _writeTarget: GLenum,
    _readOffset: GLintptr,
    _writeOffset: GLintptr,
    _size: GLsizeiptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glXQueryRendererStringMESA = unsafe extern "system" fn(
    _dpy: *mut Display,
    _screen: std::os::raw::c_int,
    _renderer: std::os::raw::c_int,
    _attribute: std::os::raw::c_int,
) -> *const char;
#[allow(non_camel_case_types)]
pub type PFN_glXBindVideoImageNV = unsafe extern "system" fn(
    _dpy: *mut Display,
    _VideoDevice: GLXVideoDeviceNV,
    _pbuf: GLXPbuffer,
    _iVideoBuffer: std::os::raw::c_int,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXQueryContextInfoEXT = unsafe extern "system" fn(
    _dpy: *mut Display,
    _context: GLXContext,
    _attribute: std::os::raw::c_int,
    _value: *mut std::os::raw::c_int,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXEnumerateVideoDevicesNV = unsafe extern "system" fn(
    _dpy: *mut Display,
    _screen: std::os::raw::c_int,
    _nelements: *mut std::os::raw::c_int,
) -> *mut std::os::raw::c_uint;
#[allow(non_camel_case_types)]
pub type PFN_glXQueryHyperpipeBestAttribSGIX = unsafe extern "system" fn(
    _dpy: *mut Display,
    _timeSlice: std::os::raw::c_int,
    _attrib: std::os::raw::c_int,
    _size: std::os::raw::c_int,
    _attribList: *mut std::os::raw::c_void,
    _returnAttribList: *mut std::os::raw::c_void,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXCreateContextAttribsARB = unsafe extern "system" fn(
    _dpy: *mut Display,
    _config: GLXFBConfig,
    _share_context: GLXContext,
    _direct: bool,
    _attrib_list: *const std::os::raw::c_int,
) -> GLXContext;
#[allow(non_camel_case_types)]
pub type PFN_glXCopySubBufferMESA = unsafe extern "system" fn(
    _dpy: *mut Display,
    _drawable: GLXDrawable,
    _x: std::os::raw::c_int,
    _y: std::os::raw::c_int,
    _width: std::os::raw::c_int,
    _height: std::os::raw::c_int,
);
#[allow(non_camel_case_types)]
pub type PFN_glXCreateContextWithConfigSGIX = unsafe extern "system" fn(
    _dpy: *mut Display,
    _config: GLXFBConfigSGIX,
    _render_type: std::os::raw::c_int,
    _share_list: GLXContext,
    _direct: bool,
) -> GLXContext;
#[allow(non_camel_case_types)]
pub type PFN_glXDestroyPixmap = unsafe extern "system" fn(_dpy: *mut Display, _pixmap: GLXPixmap);
#[allow(non_camel_case_types)]
pub type PFN_glXCreateContext = unsafe extern "system" fn(
    _dpy: *mut Display,
    _vis: *mut XVisualInfo,
    _shareList: GLXContext,
    _direct: bool,
) -> GLXContext;
#[allow(non_camel_case_types)]
pub type PFN_glXReleaseBuffersMESA =
    unsafe extern "system" fn(_dpy: *mut Display, _drawable: GLXDrawable) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXDestroyPbuffer = unsafe extern "system" fn(_dpy: *mut Display, _pbuf: GLXPbuffer);
#[allow(non_camel_case_types)]
pub type PFN_glXChooseFBConfigSGIX = unsafe extern "system" fn(
    _dpy: *mut Display,
    _screen: std::os::raw::c_int,
    _attrib_list: *mut std::os::raw::c_int,
    _nelements: *mut std::os::raw::c_int,
) -> *mut GLXFBConfigSGIX;
#[allow(non_camel_case_types)]
pub type PFN_glXGetCurrentReadDrawable = unsafe extern "system" fn() -> GLXDrawable;
#[allow(non_camel_case_types)]
pub type PFN_glXGetCurrentDrawable = unsafe extern "system" fn() -> GLXDrawable;
#[allow(non_camel_case_types)]
pub type PFN_glXChooseFBConfig = unsafe extern "system" fn(
    _dpy: *mut Display,
    _screen: std::os::raw::c_int,
    _attrib_list: *const std::os::raw::c_int,
    _nelements: *mut std::os::raw::c_int,
) -> *mut GLXFBConfig;
#[allow(non_camel_case_types)]
pub type PFN_glXGetAGPOffsetMESA =
    unsafe extern "system" fn(_pointer: *const std::os::raw::c_void) -> std::os::raw::c_uint;
#[allow(non_camel_case_types)]
pub type PFN_glXQuerySwapGroupNV = unsafe extern "system" fn(
    _dpy: *mut Display,
    _drawable: GLXDrawable,
    _group: *mut GLuint,
    _barrier: *mut GLuint,
) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXBindSwapBarrierNV =
    unsafe extern "system" fn(_dpy: *mut Display, _group: GLuint, _barrier: GLuint) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXDelayBeforeSwapNV = unsafe extern "system" fn(
    _dpy: *mut Display,
    _drawable: GLXDrawable,
    _seconds: GLfloat,
) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXSet3DfxModeMESA = unsafe extern "system" fn(_mode: GLint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glXHyperpipeAttribSGIX = unsafe extern "system" fn(
    _dpy: *mut Display,
    _timeSlice: std::os::raw::c_int,
    _attrib: std::os::raw::c_int,
    _size: std::os::raw::c_int,
    _attribList: *mut std::os::raw::c_void,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXCreateAssociatedContextAMD =
    unsafe extern "system" fn(_id: std::os::raw::c_uint, _share_list: GLXContext) -> GLXContext;
#[allow(non_camel_case_types)]
pub type PFN_glXQueryGLXPbufferSGIX = unsafe extern "system" fn(
    _dpy: *mut Display,
    _pbuf: GLXPbufferSGIX,
    _attribute: std::os::raw::c_int,
    _value: *mut std::os::raw::c_uint,
);
#[allow(non_camel_case_types)]
pub type PFN_glXWaitVideoSyncSGI = unsafe extern "system" fn(
    _divisor: std::os::raw::c_int,
    _remainder: std::os::raw::c_int,
    _count: *mut std::os::raw::c_uint,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXChannelRectSyncSGIX = unsafe extern "system" fn(
    _display: *mut Display,
    _screen: std::os::raw::c_int,
    _channel: std::os::raw::c_int,
    _synctype: GLenum,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXBindTexImageEXT = unsafe extern "system" fn(
    _dpy: *mut Display,
    _drawable: GLXDrawable,
    _buffer: std::os::raw::c_int,
    _attrib_list: *const std::os::raw::c_int,
);
#[allow(non_camel_case_types)]
pub type PFN_glXDestroyHyperpipeConfigSGIX = unsafe extern "system" fn(
    _dpy: *mut Display,
    _hpId: std::os::raw::c_int,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXCreateGLXPbufferSGIX = unsafe extern "system" fn(
    _dpy: *mut Display,
    _config: GLXFBConfigSGIX,
    _width: std::os::raw::c_uint,
    _height: std::os::raw::c_uint,
    _attrib_list: *mut std::os::raw::c_int,
) -> GLXPbufferSGIX;
#[allow(non_camel_case_types)]
pub type PFN_glXCreatePbuffer = unsafe extern "system" fn(
    _dpy: *mut Display,
    _config: GLXFBConfig,
    _attrib_list: *const std::os::raw::c_int,
) -> GLXPbuffer;
#[allow(non_camel_case_types)]
pub type PFN_glXBindSwapBarrierSGIX = unsafe extern "system" fn(
    _dpy: *mut Display,
    _drawable: GLXDrawable,
    _barrier: std::os::raw::c_int,
);
#[allow(non_camel_case_types)]
pub type PFN_glXQueryExtensionsString =
    unsafe extern "system" fn(_dpy: *mut Display, _screen: std::os::raw::c_int) -> *const char;
#[allow(non_camel_case_types)]
pub type PFN_glXGetVideoDeviceNV = unsafe extern "system" fn(
    _dpy: *mut Display,
    _screen: std::os::raw::c_int,
    _numVideoDevices: std::os::raw::c_int,
    _pVideoDevice: *mut GLXVideoDeviceNV,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXHyperpipeConfigSGIX = unsafe extern "system" fn(
    _dpy: *mut Display,
    _networkId: std::os::raw::c_int,
    _npipes: std::os::raw::c_int,
    _cfg: *mut GLXHyperpipeConfigSGIX,
    _hpId: *mut std::os::raw::c_int,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXQueryHyperpipeAttribSGIX = unsafe extern "system" fn(
    _dpy: *mut Display,
    _timeSlice: std::os::raw::c_int,
    _attrib: std::os::raw::c_int,
    _size: std::os::raw::c_int,
    _returnAttribList: *mut std::os::raw::c_void,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXQueryExtension = unsafe extern "system" fn(
    _dpy: *mut Display,
    _errorb: *mut std::os::raw::c_int,
    _event: *mut std::os::raw::c_int,
) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXDestroyGLXVideoSourceSGIX =
    unsafe extern "system" fn(_dpy: *mut Display, _glxvideosource: GLXVideoSourceSGIX);
#[allow(non_camel_case_types)]
pub type PFN_glXQueryVersion = unsafe extern "system" fn(
    _dpy: *mut Display,
    _maj: *mut std::os::raw::c_int,
    _min: *mut std::os::raw::c_int,
) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXCopyContext = unsafe extern "system" fn(
    _dpy: *mut Display,
    _src: GLXContext,
    _dst: GLXContext,
    _mask: std::os::raw::c_ulong,
);
#[allow(non_camel_case_types)]
pub type PFN_glXReleaseVideoCaptureDeviceNV =
    unsafe extern "system" fn(_dpy: *mut Display, _device: GLXVideoCaptureDeviceNV);
#[allow(non_camel_case_types)]
pub type PFN_glXSendPbufferToVideoNV = unsafe extern "system" fn(
    _dpy: *mut Display,
    _pbuf: GLXPbuffer,
    _iBufferType: std::os::raw::c_int,
    _pulCounterPbuffer: *mut std::os::raw::c_ulong,
    _bBlock: GLboolean,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXSwapIntervalMESA =
    unsafe extern "system" fn(_interval: std::os::raw::c_uint) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXJoinSwapGroupSGIX =
    unsafe extern "system" fn(_dpy: *mut Display, _drawable: GLXDrawable, _member: GLXDrawable);
#[allow(non_camel_case_types)]
pub type PFN_glXCreateNewContext = unsafe extern "system" fn(
    _dpy: *mut Display,
    _config: GLXFBConfig,
    _render_type: std::os::raw::c_int,
    _share_list: GLXContext,
    _direct: bool,
) -> GLXContext;
#[allow(non_camel_case_types)]
pub type PFN_glXReleaseTexImageEXT = unsafe extern "system" fn(
    _dpy: *mut Display,
    _drawable: GLXDrawable,
    _buffer: std::os::raw::c_int,
);
#[allow(non_camel_case_types)]
pub type PFN_glXQueryFrameCountNV = unsafe extern "system" fn(
    _dpy: *mut Display,
    _screen: std::os::raw::c_int,
    _count: *mut GLuint,
) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXGetFBConfigs = unsafe extern "system" fn(
    _dpy: *mut Display,
    _screen: std::os::raw::c_int,
    _nelements: *mut std::os::raw::c_int,
) -> *mut GLXFBConfig;
#[allow(non_camel_case_types)]
pub type PFN_glXJoinSwapGroupNV =
    unsafe extern "system" fn(_dpy: *mut Display, _drawable: GLXDrawable, _group: GLuint) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXQueryVideoCaptureDeviceNV = unsafe extern "system" fn(
    _dpy: *mut Display,
    _device: GLXVideoCaptureDeviceNV,
    _attribute: std::os::raw::c_int,
    _value: *mut std::os::raw::c_int,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXUseXFont = unsafe extern "system" fn(
    _font: Font,
    _first: std::os::raw::c_int,
    _count: std::os::raw::c_int,
    _list: std::os::raw::c_int,
);
#[allow(non_camel_case_types)]
pub type PFN_glXSwapIntervalSGI =
    unsafe extern "system" fn(_interval: std::os::raw::c_int) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXQueryHyperpipeNetworkSGIX =
    unsafe extern "system" fn(
        _dpy: *mut Display,
        _npipes: *mut std::os::raw::c_int,
    ) -> *mut GLXHyperpipeNetworkSGIX;
#[allow(non_camel_case_types)]
pub type PFN_glXGetContextGPUIDAMD =
    unsafe extern "system" fn(_ctx: GLXContext) -> std::os::raw::c_uint;
#[allow(non_camel_case_types)]
pub type PFN_glXGetFBConfigAttribSGIX = unsafe extern "system" fn(
    _dpy: *mut Display,
    _config: GLXFBConfigSGIX,
    _attribute: std::os::raw::c_int,
    _value: *mut std::os::raw::c_int,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXCopyImageSubDataNV = unsafe extern "system" fn(
    _dpy: *mut Display,
    _srcCtx: GLXContext,
    _srcName: GLuint,
    _srcTarget: GLenum,
    _srcLevel: GLint,
    _srcX: GLint,
    _srcY: GLint,
    _srcZ: GLint,
    _dstCtx: GLXContext,
    _dstName: GLuint,
    _dstTarget: GLenum,
    _dstLevel: GLint,
    _dstX: GLint,
    _dstY: GLint,
    _dstZ: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glXGetClientString =
    unsafe extern "system" fn(_dpy: *mut Display, _name: std::os::raw::c_int) -> *const char;
#[allow(non_camel_case_types)]
pub type PFN_glXCreatePixmap = unsafe extern "system" fn(
    _dpy: *mut Display,
    _config: GLXFBConfig,
    _pixmap: Pixmap,
    _attrib_list: *const std::os::raw::c_int,
) -> GLXPixmap;
#[allow(non_camel_case_types)]
pub type PFN_glXQueryHyperpipeConfigSGIX = unsafe extern "system" fn(
    _dpy: *mut Display,
    _hpId: std::os::raw::c_int,
    _npipes: *mut std::os::raw::c_int,
)
    -> *mut GLXHyperpipeConfigSGIX;
#[allow(non_camel_case_types)]
pub type PFN_glXNamedCopyBufferSubDataNV = unsafe extern "system" fn(
    _dpy: *mut Display,
    _readCtx: GLXContext,
    _writeCtx: GLXContext,
    _readBuffer: GLuint,
    _writeBuffer: GLuint,
    _readOffset: GLintptr,
    _writeOffset: GLintptr,
    _size: GLsizeiptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glXGetFBConfigAttrib = unsafe extern "system" fn(
    _dpy: *mut Display,
    _config: GLXFBConfig,
    _attribute: std::os::raw::c_int,
    _value: *mut std::os::raw::c_int,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXWaitForMscOML = unsafe extern "system" fn(
    _dpy: *mut Display,
    _drawable: GLXDrawable,
    _target_msc: u64,
    _divisor: u64,
    _remainder: u64,
    _ust: *mut u64,
    _msc: *mut u64,
    _sbc: *mut u64,
) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXQueryCurrentRendererIntegerMESA = unsafe extern "system" fn(
    _attribute: std::os::raw::c_int,
    _value: *mut std::os::raw::c_uint,
) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXBindVideoCaptureDeviceNV = unsafe extern "system" fn(
    _dpy: *mut Display,
    _video_capture_slot: std::os::raw::c_uint,
    _device: GLXVideoCaptureDeviceNV,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXWaitForSbcOML = unsafe extern "system" fn(
    _dpy: *mut Display,
    _drawable: GLXDrawable,
    _target_sbc: u64,
    _ust: *mut u64,
    _msc: *mut u64,
    _sbc: *mut u64,
) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXQueryChannelDeltasSGIX = unsafe extern "system" fn(
    _display: *mut Display,
    _screen: std::os::raw::c_int,
    _channel: std::os::raw::c_int,
    _x: *mut std::os::raw::c_int,
    _y: *mut std::os::raw::c_int,
    _w: *mut std::os::raw::c_int,
    _h: *mut std::os::raw::c_int,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXMakeAssociatedContextCurrentAMD =
    unsafe extern "system" fn(_ctx: GLXContext) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXBindChannelToWindowSGIX = unsafe extern "system" fn(
    _display: *mut Display,
    _screen: std::os::raw::c_int,
    _channel: std::os::raw::c_int,
    _window: Window,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXWaitGL = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glXIsDirect = unsafe extern "system" fn(_dpy: *mut Display, _ctx: GLXContext) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXGetSelectedEventSGIX = unsafe extern "system" fn(
    _dpy: *mut Display,
    _drawable: GLXDrawable,
    _mask: *mut std::os::raw::c_ulong,
);
#[allow(non_camel_case_types)]
pub type PFN_glXQueryMaxSwapGroupsNV = unsafe extern "system" fn(
    _dpy: *mut Display,
    _screen: std::os::raw::c_int,
    _maxGroups: *mut GLuint,
    _maxBarriers: *mut GLuint,
) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXSwapBuffers = unsafe extern "system" fn(_dpy: *mut Display, _drawable: GLXDrawable);
#[allow(non_camel_case_types)]
pub type PFN_glXGetCurrentAssociatedContextAMD = unsafe extern "system" fn() -> GLXContext;
#[allow(non_camel_case_types)]
pub type PFN_glXCreateGLXPixmapMESA = unsafe extern "system" fn(
    _dpy: *mut Display,
    _visual: *mut XVisualInfo,
    _pixmap: Pixmap,
    _cmap: Colormap,
) -> GLXPixmap;
#[allow(non_camel_case_types)]
pub type PFN_glXFreeContextEXT =
    unsafe extern "system" fn(_dpy: *mut Display, _context: GLXContext);
#[allow(non_camel_case_types)]
pub type PFN_glXCreateGLXVideoSourceSGIX = unsafe extern "system" fn(
    _display: *mut Display,
    _screen: std::os::raw::c_int,
    _server: VLServer,
    _path: VLPath,
    _nodeClass: std::os::raw::c_int,
    _drainNode: VLNode,
) -> GLXVideoSourceSGIX;
#[allow(non_camel_case_types)]
pub type PFN_glXMakeContextCurrent = unsafe extern "system" fn(
    _dpy: *mut Display,
    _draw: GLXDrawable,
    _read: GLXDrawable,
    _ctx: GLXContext,
) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXSelectEventSGIX = unsafe extern "system" fn(
    _dpy: *mut Display,
    _drawable: GLXDrawable,
    _mask: std::os::raw::c_ulong,
);
#[allow(non_camel_case_types)]
pub type PFN_glXGetGPUIDsAMD = unsafe extern "system" fn(
    _maxCount: std::os::raw::c_uint,
    _ids: *mut std::os::raw::c_uint,
) -> std::os::raw::c_uint;
#[allow(non_camel_case_types)]
pub type PFN_glXCreateWindow = unsafe extern "system" fn(
    _dpy: *mut Display,
    _config: GLXFBConfig,
    _win: Window,
    _attrib_list: *const std::os::raw::c_int,
) -> GLXWindow;
#[allow(non_camel_case_types)]
pub type PFN_glXDestroyGLXPbufferSGIX =
    unsafe extern "system" fn(_dpy: *mut Display, _pbuf: GLXPbufferSGIX);
#[allow(non_camel_case_types)]
pub type PFN_glXGetCurrentReadDrawableSGI = unsafe extern "system" fn() -> GLXDrawable;
#[allow(non_camel_case_types)]
pub type PFN_glXMakeCurrentReadSGI = unsafe extern "system" fn(
    _dpy: *mut Display,
    _draw: GLXDrawable,
    _read: GLXDrawable,
    _ctx: GLXContext,
) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXDestroyWindow = unsafe extern "system" fn(_dpy: *mut Display, _win: GLXWindow);
#[allow(non_camel_case_types)]
pub type PFN_glXCreateAssociatedContextAttribsAMD = unsafe extern "system" fn(
    _id: std::os::raw::c_uint,
    _share_context: GLXContext,
    _attribList: *const std::os::raw::c_int,
) -> GLXContext;
#[allow(non_camel_case_types)]
pub type PFN_glXReleaseVideoImageNV =
    unsafe extern "system" fn(_dpy: *mut Display, _pbuf: GLXPbuffer) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXDeleteAssociatedContextAMD = unsafe extern "system" fn(_ctx: GLXContext) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXGetContextIDEXT = unsafe extern "system" fn(_context: GLXContext) -> GLXContextID;
#[allow(non_camel_case_types)]
pub type PFN_glXWaitX = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glXChooseVisual = unsafe extern "system" fn(
    _dpy: *mut Display,
    _screen: std::os::raw::c_int,
    _attribList: *mut std::os::raw::c_int,
) -> *mut XVisualInfo;
#[allow(non_camel_case_types)]
pub type PFN_glXGetCurrentDisplay = unsafe extern "system" fn() -> *mut Display;
#[allow(non_camel_case_types)]
pub type PFN_glXGetVisualFromFBConfigSGIX =
    unsafe extern "system" fn(_dpy: *mut Display, _config: GLXFBConfigSGIX) -> *mut XVisualInfo;
#[allow(non_camel_case_types)]
pub type PFN_glXQueryServerString = unsafe extern "system" fn(
    _dpy: *mut Display,
    _screen: std::os::raw::c_int,
    _name: std::os::raw::c_int,
) -> *const char;
#[allow(non_camel_case_types)]
pub type PFN_glXCreateGLXPixmap = unsafe extern "system" fn(
    _dpy: *mut Display,
    _visual: *mut XVisualInfo,
    _pixmap: Pixmap,
) -> GLXPixmap;
#[allow(non_camel_case_types)]
pub type PFN_glXBindHyperpipeSGIX = unsafe extern "system" fn(
    _dpy: *mut Display,
    _hpId: std::os::raw::c_int,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXGetFBConfigFromVisualSGIX =
    unsafe extern "system" fn(_dpy: *mut Display, _vis: *mut XVisualInfo) -> GLXFBConfigSGIX;
#[allow(non_camel_case_types)]
pub type PFN_glXDestroyGLXPixmap =
    unsafe extern "system" fn(_dpy: *mut Display, _pixmap: GLXPixmap);
#[allow(non_camel_case_types)]
pub type PFN_glXGetTransparentIndexSUN = unsafe extern "system" fn(
    _dpy: *mut Display,
    _overlay: Window,
    _underlay: Window,
    _pTransparentIndex: *mut std::os::raw::c_ulong,
) -> Status;
#[allow(non_camel_case_types)]
pub type PFN_glXGetProcAddress =
    unsafe extern "system" fn(_procName: *const GLubyte) -> __GLXextFuncPtr;
#[allow(non_camel_case_types)]
pub type PFN_glXBlitContextFramebufferAMD = unsafe extern "system" fn(
    _dstCtx: GLXContext,
    _srcX0: GLint,
    _srcY0: GLint,
    _srcX1: GLint,
    _srcY1: GLint,
    _dstX0: GLint,
    _dstY0: GLint,
    _dstX1: GLint,
    _dstY1: GLint,
    _mask: GLbitfield,
    _filter: GLenum,
);
#[allow(non_camel_case_types)]
pub type PFN_glXGetVisualFromFBConfig =
    unsafe extern "system" fn(_dpy: *mut Display, _config: GLXFBConfig) -> *mut XVisualInfo;
#[allow(non_camel_case_types)]
pub type PFN_glXQueryContext = unsafe extern "system" fn(
    _dpy: *mut Display,
    _ctx: GLXContext,
    _attribute: std::os::raw::c_int,
    _value: *mut std::os::raw::c_int,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXSwapBuffersMscOML = unsafe extern "system" fn(
    _dpy: *mut Display,
    _drawable: GLXDrawable,
    _target_msc: u64,
    _divisor: u64,
    _remainder: u64,
) -> u64;
#[allow(non_camel_case_types)]
pub type PFN_glXImportContextEXT =
    unsafe extern "system" fn(_dpy: *mut Display, _contextID: GLXContextID) -> GLXContext;
#[allow(non_camel_case_types)]
pub type PFN_glXBindVideoDeviceNV = unsafe extern "system" fn(
    _dpy: *mut Display,
    _video_slot: std::os::raw::c_uint,
    _video_device: std::os::raw::c_uint,
    _attrib_list: *const std::os::raw::c_int,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXQueryDrawable = unsafe extern "system" fn(
    _dpy: *mut Display,
    _draw: GLXDrawable,
    _attribute: std::os::raw::c_int,
    _value: *mut std::os::raw::c_uint,
);
#[allow(non_camel_case_types)]
pub type PFN_glXDestroyContext = unsafe extern "system" fn(_dpy: *mut Display, _ctx: GLXContext);
#[allow(non_camel_case_types)]
pub type PFN_glXGetSwapIntervalMESA = unsafe extern "system" fn() -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_glXResetFrameCountNV =
    unsafe extern "system" fn(_dpy: *mut Display, _screen: std::os::raw::c_int) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXAssociateDMPbufferSGIX = unsafe extern "system" fn(
    _dpy: *mut Display,
    _pbuffer: GLXPbufferSGIX,
    _params: *mut DMparams,
    _dmbuffer: DMbuffer,
) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXEnumerateVideoCaptureDevicesNV =
    unsafe extern "system" fn(
        _dpy: *mut Display,
        _screen: std::os::raw::c_int,
        _nelements: *mut std::os::raw::c_int,
    ) -> *mut GLXVideoCaptureDeviceNV;
