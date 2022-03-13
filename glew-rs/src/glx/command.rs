use crate::glx;
use crate::types::*;
use glx::bitflags::*;
use glx::enums::*;
use glx::types::*;
use std::ffi::c_void;
use std::fmt;
#[allow(non_camel_case_types)]
pub type PFN_glXMakeContextCurrent = unsafe extern "system" fn(
    _dpy: Display,
    _draw: GLXDrawable,
    _read: GLXDrawable,
    _ctx: GLXContext,
) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXNamedCopyBufferSubDataNV = unsafe extern "system" fn(
    _dpy: Display,
    _readCtx: GLXContext,
    _writeCtx: GLXContext,
    _readBuffer: GLuint,
    _writeBuffer: GLuint,
    _readOffset: GLintptr,
    _writeOffset: GLintptr,
    _size: GLsizeiptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glXReleaseVideoCaptureDeviceNV =
    unsafe extern "system" fn(_dpy: Display, _device: GLXVideoCaptureDeviceNV);
#[allow(non_camel_case_types)]
pub type PFN_glXBindTexImageEXT = unsafe extern "system" fn(
    _dpy: Display,
    _drawable: GLXDrawable,
    _buffer: GLvoid,
    _attrib_list: GLvoid,
);
#[allow(non_camel_case_types)]
pub type PFN_glXQueryRendererIntegerMESA = unsafe extern "system" fn(
    _dpy: Display,
    _screen: GLvoid,
    _renderer: GLvoid,
    _attribute: GLvoid,
    _value: GLvoid,
) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXGetFBConfigAttribSGIX = unsafe extern "system" fn(
    _dpy: Display,
    _config: GLXFBConfigSGIX,
    _attribute: GLvoid,
    _value: GLvoid,
);
#[allow(non_camel_case_types)]
pub type PFN_glXHyperpipeConfigSGIX = unsafe extern "system" fn(
    _dpy: Display,
    _networkId: GLvoid,
    _npipes: GLvoid,
    _cfg: GLXHyperpipeConfigSGIX,
    _hpId: GLvoid,
);
#[allow(non_camel_case_types)]
pub type PFN_glXCreateGLXPixmapMESA = unsafe extern "system" fn(
    _dpy: Display,
    _visual: XVisualInfo,
    _pixmap: Pixmap,
    _cmap: Colormap,
) -> GLXPixmap;
#[allow(non_camel_case_types)]
pub type PFN_glXUseXFont =
    unsafe extern "system" fn(_font: Font, _first: GLvoid, _count: GLvoid, _list: GLvoid);
#[allow(non_camel_case_types)]
pub type PFN_glXSwapBuffersMscOML = unsafe extern "system" fn(
    _dpy: Display,
    _drawable: GLXDrawable,
    _target_msc: u64,
    _divisor: u64,
    _remainder: u64,
) -> u64;
#[allow(non_camel_case_types)]
pub type PFN_glXDestroyWindow = unsafe extern "system" fn(_dpy: Display, _win: GLXWindow);
#[allow(non_camel_case_types)]
pub type PFN_glXWaitX = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glXMakeAssociatedContextCurrentAMD =
    unsafe extern "system" fn(_ctx: GLXContext) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXChooseFBConfig = unsafe extern "system" fn(
    _dpy: Display,
    _screen: GLvoid,
    _attrib_list: GLvoid,
    _nelements: GLvoid,
) -> GLXFBConfig;
#[allow(non_camel_case_types)]
pub type PFN_glXCopyBufferSubDataNV = unsafe extern "system" fn(
    _dpy: Display,
    _readCtx: GLXContext,
    _writeCtx: GLXContext,
    _readTarget: GLenum,
    _writeTarget: GLenum,
    _readOffset: GLintptr,
    _writeOffset: GLintptr,
    _size: GLsizeiptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glXGetCurrentReadDrawable = unsafe extern "system" fn() -> GLXDrawable;
#[allow(non_camel_case_types)]
pub type PFN_glXSelectEventSGIX =
    unsafe extern "system" fn(_dpy: Display, _drawable: GLXDrawable, _mask: GLvoid);
#[allow(non_camel_case_types)]
pub type PFN_glXGetCurrentDisplay = unsafe extern "system" fn() -> Display;
#[allow(non_camel_case_types)]
pub type PFN_glXFreeContextEXT = unsafe extern "system" fn(_dpy: Display, _context: GLXContext);
#[allow(non_camel_case_types)]
pub type PFN_glXQueryVideoCaptureDeviceNV = unsafe extern "system" fn(
    _dpy: Display,
    _device: GLXVideoCaptureDeviceNV,
    _attribute: GLvoid,
    _value: GLvoid,
);
#[allow(non_camel_case_types)]
pub type PFN_glXCreateGLXPixmapWithConfigSGIX = unsafe extern "system" fn(
    _dpy: Display,
    _config: GLXFBConfigSGIX,
    _pixmap: Pixmap,
) -> GLXPixmap;
#[allow(non_camel_case_types)]
pub type PFN_glXQueryFrameCountNV =
    unsafe extern "system" fn(_dpy: Display, _screen: GLvoid, _count: GLuint) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXJoinSwapGroupNV =
    unsafe extern "system" fn(_dpy: Display, _drawable: GLXDrawable, _group: GLuint) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXDestroyGLXPbufferSGIX =
    unsafe extern "system" fn(_dpy: Display, _pbuf: GLXPbufferSGIX);
#[allow(non_camel_case_types)]
pub type PFN_glXEnumerateVideoDevicesNV =
    unsafe extern "system" fn(_dpy: Display, _screen: GLvoid, _nelements: GLvoid);
#[allow(non_camel_case_types)]
pub type PFN_glXGetCurrentAssociatedContextAMD = unsafe extern "system" fn() -> GLXContext;
#[allow(non_camel_case_types)]
pub type PFN_glXQueryVersion =
    unsafe extern "system" fn(_dpy: Display, _maj: GLvoid, _min: GLvoid) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXSendPbufferToVideoNV = unsafe extern "system" fn(
    _dpy: Display,
    _pbuf: GLXPbuffer,
    _iBufferType: GLvoid,
    _pulCounterPbuffer: GLvoid,
    _bBlock: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glXQueryMaxSwapBarriersSGIX =
    unsafe extern "system" fn(_dpy: Display, _screen: GLvoid, _max: GLvoid) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXGetProcAddressARB =
    unsafe extern "system" fn(_procName: GLubyte) -> __GLXextFuncPtr;
#[allow(non_camel_case_types)]
pub type PFN_glXGetVisualFromFBConfig =
    unsafe extern "system" fn(_dpy: Display, _config: GLXFBConfig) -> XVisualInfo;
#[allow(non_camel_case_types)]
pub type PFN_glXGetCurrentContext = unsafe extern "system" fn() -> GLXContext;
#[allow(non_camel_case_types)]
pub type PFN_glXCreateWindow = unsafe extern "system" fn(
    _dpy: Display,
    _config: GLXFBConfig,
    _win: Window,
    _attrib_list: GLvoid,
) -> GLXWindow;
#[allow(non_camel_case_types)]
pub type PFN_glXQueryServerString =
    unsafe extern "system" fn(_dpy: Display, _screen: GLvoid, _name: GLvoid);
#[allow(non_camel_case_types)]
pub type PFN_glXReleaseVideoImageNV = unsafe extern "system" fn(_dpy: Display, _pbuf: GLXPbuffer);
#[allow(non_camel_case_types)]
pub type PFN_glXWaitForMscOML = unsafe extern "system" fn(
    _dpy: Display,
    _drawable: GLXDrawable,
    _target_msc: u64,
    _divisor: u64,
    _remainder: u64,
    _ust: u64,
    _msc: u64,
    _sbc: u64,
) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXCreateGLXPixmap =
    unsafe extern "system" fn(_dpy: Display, _visual: XVisualInfo, _pixmap: Pixmap) -> GLXPixmap;
#[allow(non_camel_case_types)]
pub type PFN_glXGetFBConfigAttrib = unsafe extern "system" fn(
    _dpy: Display,
    _config: GLXFBConfig,
    _attribute: GLvoid,
    _value: GLvoid,
);
#[allow(non_camel_case_types)]
pub type PFN_glXGetClientString = unsafe extern "system" fn(_dpy: Display, _name: GLvoid);
#[allow(non_camel_case_types)]
pub type PFN_glXGetConfig =
    unsafe extern "system" fn(_dpy: Display, _visual: XVisualInfo, _attrib: GLvoid, _value: GLvoid);
#[allow(non_camel_case_types)]
pub type PFN_glXQueryChannelDeltasSGIX = unsafe extern "system" fn(
    _display: Display,
    _screen: GLvoid,
    _channel: GLvoid,
    _x: GLvoid,
    _y: GLvoid,
    _w: GLvoid,
    _h: GLvoid,
);
#[allow(non_camel_case_types)]
pub type PFN_glXGetSelectedEventSGIX =
    unsafe extern "system" fn(_dpy: Display, _drawable: GLXDrawable, _mask: GLvoid);
#[allow(non_camel_case_types)]
pub type PFN_glXCushionSGI =
    unsafe extern "system" fn(_dpy: Display, _window: Window, _cushion: GLvoid);
#[allow(non_camel_case_types)]
pub type PFN_glXDestroyGLXPixmap = unsafe extern "system" fn(_dpy: Display, _pixmap: GLXPixmap);
#[allow(non_camel_case_types)]
pub type PFN_glXQueryGLXPbufferSGIX = unsafe extern "system" fn(
    _dpy: Display,
    _pbuf: GLXPbufferSGIX,
    _attribute: GLvoid,
    _value: GLvoid,
);
#[allow(non_camel_case_types)]
pub type PFN_glXCreateGLXVideoSourceSGIX = unsafe extern "system" fn(
    _display: Display,
    _screen: GLvoid,
    _server: VLServer,
    _path: VLPath,
    _nodeClass: GLvoid,
    _drainNode: VLNode,
) -> GLXVideoSourceSGIX;
#[allow(non_camel_case_types)]
pub type PFN_glXQueryHyperpipeBestAttribSGIX = unsafe extern "system" fn(
    _dpy: Display,
    _timeSlice: GLvoid,
    _attrib: GLvoid,
    _size: GLvoid,
    _attribList: GLvoid,
    _returnAttribList: GLvoid,
);
#[allow(non_camel_case_types)]
pub type PFN_glXSwapIntervalSGI = unsafe extern "system" fn(_interval: GLvoid);
#[allow(non_camel_case_types)]
pub type PFN_glXChooseFBConfigSGIX = unsafe extern "system" fn(
    _dpy: Display,
    _screen: GLvoid,
    _attrib_list: GLvoid,
    _nelements: GLvoid,
) -> GLXFBConfigSGIX;
#[allow(non_camel_case_types)]
pub type PFN_glXEnumerateVideoCaptureDevicesNV =
    unsafe extern "system" fn(
        _dpy: Display,
        _screen: GLvoid,
        _nelements: GLvoid,
    ) -> GLXVideoCaptureDeviceNV;
#[allow(non_camel_case_types)]
pub type PFN_glXSwapBuffers = unsafe extern "system" fn(_dpy: Display, _drawable: GLXDrawable);
#[allow(non_camel_case_types)]
pub type PFN_glXBindVideoDeviceNV = unsafe extern "system" fn(
    _dpy: Display,
    _video_slot: GLvoid,
    _video_device: GLvoid,
    _attrib_list: GLvoid,
);
#[allow(non_camel_case_types)]
pub type PFN_glXBindVideoImageNV = unsafe extern "system" fn(
    _dpy: Display,
    _VideoDevice: GLXVideoDeviceNV,
    _pbuf: GLXPbuffer,
    _iVideoBuffer: GLvoid,
);
#[allow(non_camel_case_types)]
pub type PFN_glXGetSelectedEvent =
    unsafe extern "system" fn(_dpy: Display, _draw: GLXDrawable, _event_mask: GLvoid);
#[allow(non_camel_case_types)]
pub type PFN_glXCopySubBufferMESA = unsafe extern "system" fn(
    _dpy: Display,
    _drawable: GLXDrawable,
    _x: GLvoid,
    _y: GLvoid,
    _width: GLvoid,
    _height: GLvoid,
);
#[allow(non_camel_case_types)]
pub type PFN_glXCreateGLXPbufferSGIX = unsafe extern "system" fn(
    _dpy: Display,
    _config: GLXFBConfigSGIX,
    _width: GLvoid,
    _height: GLvoid,
    _attrib_list: GLvoid,
) -> GLXPbufferSGIX;
#[allow(non_camel_case_types)]
pub type PFN_glXDestroyGLXVideoSourceSGIX =
    unsafe extern "system" fn(_dpy: Display, _glxvideosource: GLXVideoSourceSGIX);
#[allow(non_camel_case_types)]
pub type PFN_glXGetCurrentDisplayEXT = unsafe extern "system" fn() -> Display;
#[allow(non_camel_case_types)]
pub type PFN_glXQueryExtensionsString = unsafe extern "system" fn(_dpy: Display, _screen: GLvoid);
#[allow(non_camel_case_types)]
pub type PFN_glXBindSwapBarrierNV =
    unsafe extern "system" fn(_dpy: Display, _group: GLuint, _barrier: GLuint) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXGetVideoDeviceNV = unsafe extern "system" fn(
    _dpy: Display,
    _screen: GLvoid,
    _numVideoDevices: GLvoid,
    _pVideoDevice: GLXVideoDeviceNV,
);
#[allow(non_camel_case_types)]
pub type PFN_glXQueryCurrentRendererIntegerMESA =
    unsafe extern "system" fn(_attribute: GLvoid, _value: GLvoid) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXQueryContext =
    unsafe extern "system" fn(_dpy: Display, _ctx: GLXContext, _attribute: GLvoid, _value: GLvoid);
#[allow(non_camel_case_types)]
pub type PFN_glXBindChannelToWindowSGIX = unsafe extern "system" fn(
    _display: Display,
    _screen: GLvoid,
    _channel: GLvoid,
    _window: Window,
);
#[allow(non_camel_case_types)]
pub type PFN_glXMakeCurrentReadSGI = unsafe extern "system" fn(
    _dpy: Display,
    _draw: GLXDrawable,
    _read: GLXDrawable,
    _ctx: GLXContext,
) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXResetFrameCountNV =
    unsafe extern "system" fn(_dpy: Display, _screen: GLvoid) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXGetFBConfigFromVisualSGIX =
    unsafe extern "system" fn(_dpy: Display, _vis: XVisualInfo) -> GLXFBConfigSGIX;
#[allow(non_camel_case_types)]
pub type PFN_glXDestroyPixmap = unsafe extern "system" fn(_dpy: Display, _pixmap: GLXPixmap);
#[allow(non_camel_case_types)]
pub type PFN_glXCreatePbuffer = unsafe extern "system" fn(
    _dpy: Display,
    _config: GLXFBConfig,
    _attrib_list: GLvoid,
) -> GLXPbuffer;
#[allow(non_camel_case_types)]
pub type PFN_glXGetAGPOffsetMESA = unsafe extern "system" fn(_pointer: GLvoid);
#[allow(non_camel_case_types)]
pub type PFN_glXGetGPUInfoAMD = unsafe extern "system" fn(
    _id: GLvoid,
    _property: GLvoid,
    _dataType: GLenum,
    _size: GLvoid,
    _data: GLvoid,
);
#[allow(non_camel_case_types)]
pub type PFN_glXSwapIntervalMESA = unsafe extern "system" fn(_interval: GLvoid);
#[allow(non_camel_case_types)]
pub type PFN_glXGetGPUIDsAMD = unsafe extern "system" fn(_maxCount: GLvoid, _ids: GLvoid);
#[allow(non_camel_case_types)]
pub type PFN_glXQueryCurrentRendererStringMESA = unsafe extern "system" fn(_attribute: GLvoid);
#[allow(non_camel_case_types)]
pub type PFN_glXCreateAssociatedContextAMD =
    unsafe extern "system" fn(_id: GLvoid, _share_list: GLXContext) -> GLXContext;
#[allow(non_camel_case_types)]
pub type PFN_glXQueryHyperpipeConfigSGIX = unsafe extern "system" fn(
    _dpy: Display,
    _hpId: GLvoid,
    _npipes: GLvoid,
) -> GLXHyperpipeConfigSGIX;
#[allow(non_camel_case_types)]
pub type PFN_glXSwapIntervalEXT =
    unsafe extern "system" fn(_dpy: Display, _drawable: GLXDrawable, _interval: GLvoid);
#[allow(non_camel_case_types)]
pub type PFN_glXCreateAssociatedContextAttribsAMD = unsafe extern "system" fn(
    _id: GLvoid,
    _share_context: GLXContext,
    _attribList: GLvoid,
) -> GLXContext;
#[allow(non_camel_case_types)]
pub type PFN_glXGetVideoSyncSGI = unsafe extern "system" fn(_count: GLvoid);
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
pub type PFN_glXGetVideoInfoNV = unsafe extern "system" fn(
    _dpy: Display,
    _screen: GLvoid,
    _VideoDevice: GLXVideoDeviceNV,
    _pulCounterOutputPbuffer: GLvoid,
    _pulCounterOutputVideo: GLvoid,
);
#[allow(non_camel_case_types)]
pub type PFN_glXBindSwapBarrierSGIX =
    unsafe extern "system" fn(_dpy: Display, _drawable: GLXDrawable, _barrier: GLvoid);
#[allow(non_camel_case_types)]
pub type PFN_glXGetFBConfigs =
    unsafe extern "system" fn(_dpy: Display, _screen: GLvoid, _nelements: GLvoid) -> GLXFBConfig;
#[allow(non_camel_case_types)]
pub type PFN_glXCreateContextWithConfigSGIX = unsafe extern "system" fn(
    _dpy: Display,
    _config: GLXFBConfigSGIX,
    _render_type: GLvoid,
    _share_list: GLXContext,
    _direct: bool,
) -> GLXContext;
#[allow(non_camel_case_types)]
pub type PFN_glXQueryHyperpipeAttribSGIX = unsafe extern "system" fn(
    _dpy: Display,
    _timeSlice: GLvoid,
    _attrib: GLvoid,
    _size: GLvoid,
    _returnAttribList: GLvoid,
);
#[allow(non_camel_case_types)]
pub type PFN_glXQueryRendererStringMESA = unsafe extern "system" fn(
    _dpy: Display,
    _screen: GLvoid,
    _renderer: GLvoid,
    _attribute: GLvoid,
);
#[allow(non_camel_case_types)]
pub type PFN_glXSet3DfxModeMESA = unsafe extern "system" fn(_mode: GLint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glXAssociateDMPbufferSGIX = unsafe extern "system" fn(
    _dpy: Display,
    _pbuffer: GLXPbufferSGIX,
    _params: DMparams,
    _dmbuffer: DMbuffer,
) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXWaitForSbcOML = unsafe extern "system" fn(
    _dpy: Display,
    _drawable: GLXDrawable,
    _target_sbc: u64,
    _ust: u64,
    _msc: u64,
    _sbc: u64,
) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXQueryChannelRectSGIX = unsafe extern "system" fn(
    _display: Display,
    _screen: GLvoid,
    _channel: GLvoid,
    _dx: GLvoid,
    _dy: GLvoid,
    _dw: GLvoid,
    _dh: GLvoid,
);
#[allow(non_camel_case_types)]
pub type PFN_glXGetTransparentIndexSUN = unsafe extern "system" fn(
    _dpy: Display,
    _overlay: Window,
    _underlay: Window,
    _pTransparentIndex: GLvoid,
) -> Status;
#[allow(non_camel_case_types)]
pub type PFN_glXQueryHyperpipeNetworkSGIX =
    unsafe extern "system" fn(_dpy: Display, _npipes: GLvoid) -> GLXHyperpipeNetworkSGIX;
#[allow(non_camel_case_types)]
pub type PFN_glXChannelRectSGIX = unsafe extern "system" fn(
    _display: Display,
    _screen: GLvoid,
    _channel: GLvoid,
    _x: GLvoid,
    _y: GLvoid,
    _w: GLvoid,
    _h: GLvoid,
);
#[allow(non_camel_case_types)]
pub type PFN_glXDelayBeforeSwapNV =
    unsafe extern "system" fn(_dpy: Display, _drawable: GLXDrawable, _seconds: GLfloat) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXReleaseTexImageEXT =
    unsafe extern "system" fn(_dpy: Display, _drawable: GLXDrawable, _buffer: GLvoid);
#[allow(non_camel_case_types)]
pub type PFN_glXGetProcAddress = unsafe extern "system" fn(_procName: GLubyte) -> __GLXextFuncPtr;
#[allow(non_camel_case_types)]
pub type PFN_glXQueryDrawable = unsafe extern "system" fn(
    _dpy: Display,
    _draw: GLXDrawable,
    _attribute: GLvoid,
    _value: GLvoid,
);
#[allow(non_camel_case_types)]
pub type PFN_glXGetMscRateOML = unsafe extern "system" fn(
    _dpy: Display,
    _drawable: GLXDrawable,
    _numerator: u32,
    _denominator: u32,
) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXGetCurrentReadDrawableSGI = unsafe extern "system" fn() -> GLXDrawable;
#[allow(non_camel_case_types)]
pub type PFN_glXReleaseBuffersMESA =
    unsafe extern "system" fn(_dpy: Display, _drawable: GLXDrawable) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXReleaseVideoDeviceNV =
    unsafe extern "system" fn(_dpy: Display, _screen: GLvoid, _VideoDevice: GLXVideoDeviceNV);
#[allow(non_camel_case_types)]
pub type PFN_glXWaitGL = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glXCreateContextAttribsARB = unsafe extern "system" fn(
    _dpy: Display,
    _config: GLXFBConfig,
    _share_context: GLXContext,
    _direct: bool,
    _attrib_list: GLvoid,
) -> GLXContext;
#[allow(non_camel_case_types)]
pub type PFN_glXDestroyPbuffer = unsafe extern "system" fn(_dpy: Display, _pbuf: GLXPbuffer);
#[allow(non_camel_case_types)]
pub type PFN_glXDeleteAssociatedContextAMD = unsafe extern "system" fn(_ctx: GLXContext) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXGetContextIDEXT = unsafe extern "system" fn(_context: GLXContext) -> GLXContextID;
#[allow(non_camel_case_types)]
pub type PFN_glXBindVideoCaptureDeviceNV = unsafe extern "system" fn(
    _dpy: Display,
    _video_capture_slot: GLvoid,
    _device: GLXVideoCaptureDeviceNV,
);
#[allow(non_camel_case_types)]
pub type PFN_glXCreatePixmap = unsafe extern "system" fn(
    _dpy: Display,
    _config: GLXFBConfig,
    _pixmap: Pixmap,
    _attrib_list: GLvoid,
) -> GLXPixmap;
#[allow(non_camel_case_types)]
pub type PFN_glXDestroyContext = unsafe extern "system" fn(_dpy: Display, _ctx: GLXContext);
#[allow(non_camel_case_types)]
pub type PFN_glXCopyContext =
    unsafe extern "system" fn(_dpy: Display, _src: GLXContext, _dst: GLXContext, _mask: GLvoid);
#[allow(non_camel_case_types)]
pub type PFN_glXCreateContext = unsafe extern "system" fn(
    _dpy: Display,
    _vis: XVisualInfo,
    _shareList: GLXContext,
    _direct: bool,
) -> GLXContext;
#[allow(non_camel_case_types)]
pub type PFN_glXGetSyncValuesOML = unsafe extern "system" fn(
    _dpy: Display,
    _drawable: GLXDrawable,
    _ust: u64,
    _msc: u64,
    _sbc: u64,
) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXIsDirect = unsafe extern "system" fn(_dpy: Display, _ctx: GLXContext) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXQueryContextInfoEXT = unsafe extern "system" fn(
    _dpy: Display,
    _context: GLXContext,
    _attribute: GLvoid,
    _value: GLvoid,
);
#[allow(non_camel_case_types)]
pub type PFN_glXWaitVideoSyncSGI =
    unsafe extern "system" fn(_divisor: GLvoid, _remainder: GLvoid, _count: GLvoid);
#[allow(non_camel_case_types)]
pub type PFN_glXChannelRectSyncSGIX = unsafe extern "system" fn(
    _display: Display,
    _screen: GLvoid,
    _channel: GLvoid,
    _synctype: GLenum,
);
#[allow(non_camel_case_types)]
pub type PFN_glXLockVideoCaptureDeviceNV =
    unsafe extern "system" fn(_dpy: Display, _device: GLXVideoCaptureDeviceNV);
#[allow(non_camel_case_types)]
pub type PFN_glXJoinSwapGroupSGIX =
    unsafe extern "system" fn(_dpy: Display, _drawable: GLXDrawable, _member: GLXDrawable);
#[allow(non_camel_case_types)]
pub type PFN_glXCopyImageSubDataNV = unsafe extern "system" fn(
    _dpy: Display,
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
pub type PFN_glXGetSwapIntervalMESA = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glXChooseVisual =
    unsafe extern "system" fn(_dpy: Display, _screen: GLvoid, _attribList: GLvoid) -> XVisualInfo;
#[allow(non_camel_case_types)]
pub type PFN_glXQueryMaxSwapGroupsNV = unsafe extern "system" fn(
    _dpy: Display,
    _screen: GLvoid,
    _maxGroups: GLuint,
    _maxBarriers: GLuint,
) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXDestroyHyperpipeConfigSGIX =
    unsafe extern "system" fn(_dpy: Display, _hpId: GLvoid);
#[allow(non_camel_case_types)]
pub type PFN_glXGetCurrentDrawable = unsafe extern "system" fn() -> GLXDrawable;
#[allow(non_camel_case_types)]
pub type PFN_glXBindHyperpipeSGIX = unsafe extern "system" fn(_dpy: Display, _hpId: GLvoid);
#[allow(non_camel_case_types)]
pub type PFN_glXImportContextEXT =
    unsafe extern "system" fn(_dpy: Display, _contextID: GLXContextID) -> GLXContext;
#[allow(non_camel_case_types)]
pub type PFN_glXGetContextGPUIDAMD = unsafe extern "system" fn(_ctx: GLXContext);
#[allow(non_camel_case_types)]
pub type PFN_glXMakeCurrent =
    unsafe extern "system" fn(_dpy: Display, _drawable: GLXDrawable, _ctx: GLXContext) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXHyperpipeAttribSGIX = unsafe extern "system" fn(
    _dpy: Display,
    _timeSlice: GLvoid,
    _attrib: GLvoid,
    _size: GLvoid,
    _attribList: GLvoid,
);
#[allow(non_camel_case_types)]
pub type PFN_glXGetVisualFromFBConfigSGIX =
    unsafe extern "system" fn(_dpy: Display, _config: GLXFBConfigSGIX) -> XVisualInfo;
#[allow(non_camel_case_types)]
pub type PFN_glXQuerySwapGroupNV = unsafe extern "system" fn(
    _dpy: Display,
    _drawable: GLXDrawable,
    _group: GLuint,
    _barrier: GLuint,
) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXSelectEvent =
    unsafe extern "system" fn(_dpy: Display, _draw: GLXDrawable, _event_mask: GLvoid);
#[allow(non_camel_case_types)]
pub type PFN_glXQueryExtension =
    unsafe extern "system" fn(_dpy: Display, _errorb: GLvoid, _event: GLvoid) -> bool;
#[allow(non_camel_case_types)]
pub type PFN_glXCreateNewContext = unsafe extern "system" fn(
    _dpy: Display,
    _config: GLXFBConfig,
    _render_type: GLvoid,
    _share_list: GLXContext,
    _direct: bool,
) -> GLXContext;
