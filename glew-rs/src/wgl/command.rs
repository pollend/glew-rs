use crate::types::*;
use crate::wgl;
use std::ffi::c_void;
use std::fmt;
use wgl::bitflags::*;
use wgl::enums::*;
use wgl::types::*;
#[allow(non_camel_case_types)]
pub type PFN_wglSwapLayerBuffers = unsafe extern "system" fn(_hdc: HDC, _fuFlags: UINT) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglGetExtensionsStringARB = unsafe extern "system" fn(_hdc: HDC) -> *const char;
#[allow(non_camel_case_types)]
pub type PFN_wglGetGammaTableI3D = unsafe extern "system" fn(
    _hDC: HDC,
    _iEntries: std::os::raw::c_int,
    _puRed: *mut USHORT,
    _puGreen: *mut USHORT,
    _puBlue: *mut USHORT,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglBindVideoImageNV = unsafe extern "system" fn(
    _hVideoDevice: HPVIDEODEV,
    _hPbuffer: HPBUFFERARB,
    _iVideoBuffer: std::os::raw::c_int,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglGetGenlockSampleRateI3D =
    unsafe extern "system" fn(_hDC: HDC, _uRate: *mut UINT) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglSendPbufferToVideoNV = unsafe extern "system" fn(
    _hPbuffer: HPBUFFERARB,
    _iBufferType: std::os::raw::c_int,
    _pulCounterPbuffer: *mut std::os::raw::c_ulong,
    _bBlock: BOOL,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglEnumGpusFromAffinityDCNV =
    unsafe extern "system" fn(_hAffinityDC: HDC, _iGpuIndex: UINT, _hGpu: *mut HGPUNV) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglChoosePixelFormatARB = unsafe extern "system" fn(
    _hdc: HDC,
    _piAttribIList: *const std::os::raw::c_int,
    _pfAttribFList: *const FLOAT,
    _nMaxFormats: UINT,
    _piFormats: *mut std::os::raw::c_int,
    _nNumFormats: *mut UINT,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglEnumerateVideoCaptureDevicesNV =
    unsafe extern "system" fn(_hDc: HDC, _phDeviceList: *mut HVIDEOINPUTDEVICENV) -> UINT;
#[allow(non_camel_case_types)]
pub type PFN_wglGetCurrentReadDCARB = unsafe extern "system" fn() -> HDC;
#[allow(non_camel_case_types)]
pub type PFN_wglQueryGenlockMaxSourceDelayI3D = unsafe extern "system" fn(
    _hDC: HDC,
    _uMaxLineDelay: *mut UINT,
    _uMaxPixelDelay: *mut UINT,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_SetPixelFormat = unsafe extern "system" fn(
    _hdc: HDC,
    _ipfd: std::os::raw::c_int,
    _ppfd: *const PIXELFORMATDESCRIPTOR,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglBlitContextFramebufferAMD = unsafe extern "system" fn(
    _dstCtx: HGLRC,
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
) -> VOID;
#[allow(non_camel_case_types)]
pub type PFN_wglChoosePixelFormatEXT = unsafe extern "system" fn(
    _hdc: HDC,
    _piAttribIList: *const std::os::raw::c_int,
    _pfAttribFList: *const FLOAT,
    _nMaxFormats: UINT,
    _piFormats: *mut std::os::raw::c_int,
    _nNumFormats: *mut UINT,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_SwapBuffers = unsafe extern "system" fn(_hdc: HDC) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglDescribeLayerPlane = unsafe extern "system" fn(
    _hDc: HDC,
    _pixelFormat: std::os::raw::c_int,
    _layerPlane: std::os::raw::c_int,
    _nBytes: UINT,
    _plpd: *const LAYERPLANEDESCRIPTOR,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglEndFrameTrackingI3D = unsafe extern "system" fn() -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglMakeContextCurrentARB =
    unsafe extern "system" fn(_hDrawDC: HDC, _hReadDC: HDC, _hglrc: HGLRC) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglGenlockSourceDelayI3D = unsafe extern "system" fn(_hDC: HDC, _uDelay: UINT) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglMakeCurrent = unsafe extern "system" fn(_hDc: HDC, _newContext: HGLRC) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglDestroyPbufferEXT = unsafe extern "system" fn(_hPbuffer: HPBUFFEREXT) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglIsEnabledGenlockI3D =
    unsafe extern "system" fn(_hDC: HDC, _pFlag: *mut BOOL) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglFreeMemoryNV = unsafe extern "system" fn(_pointer: *mut std::os::raw::c_void);
#[allow(non_camel_case_types)]
pub type PFN_wglReleaseTexImageARB =
    unsafe extern "system" fn(_hPbuffer: HPBUFFERARB, _iBuffer: std::os::raw::c_int) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglReleaseVideoCaptureDeviceNV =
    unsafe extern "system" fn(_hDc: HDC, _hDevice: HVIDEOINPUTDEVICENV) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglGetFrameUsageI3D = unsafe extern "system" fn(_pUsage: *mut f32) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglBindVideoCaptureDeviceNV =
    unsafe extern "system" fn(_uVideoSlot: UINT, _hDevice: HVIDEOINPUTDEVICENV) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglQuerySwapGroupNV =
    unsafe extern "system" fn(_hDC: HDC, _group: *mut GLuint, _barrier: *mut GLuint) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglCreateAssociatedContextAttribsAMD = unsafe extern "system" fn(
    _id: UINT,
    _hShareContext: HGLRC,
    _attribList: *const std::os::raw::c_int,
) -> HGLRC;
#[allow(non_camel_case_types)]
pub type PFN_wglEnumGpusNV =
    unsafe extern "system" fn(_iGpuIndex: UINT, _phGpu: *mut HGPUNV) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglEnableGenlockI3D = unsafe extern "system" fn(_hDC: HDC) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglEnumGpuDevicesNV = unsafe extern "system" fn(
    _hGpu: HGPUNV,
    _iDeviceIndex: UINT,
    _lpGpuDevice: PGPU_DEVICE,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglSetGammaTableParametersI3D = unsafe extern "system" fn(
    _hDC: HDC,
    _iAttribute: std::os::raw::c_int,
    _piValue: *const std::os::raw::c_int,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglGetVideoInfoNV = unsafe extern "system" fn(
    _hpVideoDevice: HPVIDEODEV,
    _pulCounterOutputPbuffer: *mut std::os::raw::c_ulong,
    _pulCounterOutputVideo: *mut std::os::raw::c_ulong,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglDestroyDisplayColorTableEXT = unsafe extern "system" fn(_id: GLushort) -> VOID;
#[allow(non_camel_case_types)]
pub type PFN_wglUseFontBitmaps =
    unsafe extern "system" fn(_hDC: HDC, _first: DWORD, _count: DWORD, _listBase: DWORD) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglSwapIntervalEXT = unsafe extern "system" fn(_interval: std::os::raw::c_int) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglMakeContextCurrentEXT =
    unsafe extern "system" fn(_hDrawDC: HDC, _hReadDC: HDC, _hglrc: HGLRC) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglBeginFrameTrackingI3D = unsafe extern "system" fn() -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglAllocateMemoryNV = unsafe extern "system" fn(
    _size: GLsizei,
    _readfreq: GLfloat,
    _writefreq: GLfloat,
    _priority: GLfloat,
) -> *mut std::os::raw::c_void;
#[allow(non_camel_case_types)]
pub type PFN_wglCreateContext = unsafe extern "system" fn(_hDc: HDC) -> HGLRC;
#[allow(non_camel_case_types)]
pub type PFN_GetPixelFormat = unsafe extern "system" fn(_hdc: HDC) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_wglGetCurrentDC = unsafe extern "system" fn() -> HDC;
#[allow(non_camel_case_types)]
pub type PFN_wglGetPixelFormatAttribivEXT = unsafe extern "system" fn(
    _hdc: HDC,
    _iPixelFormat: std::os::raw::c_int,
    _iLayerPlane: std::os::raw::c_int,
    _nAttributes: UINT,
    _piAttributes: *mut std::os::raw::c_int,
    _piValues: *mut std::os::raw::c_int,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglGetSwapIntervalEXT = unsafe extern "system" fn() -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_wglQueryFrameTrackingI3D = unsafe extern "system" fn(
    _pFrameCount: *mut DWORD,
    _pMissedFrames: *mut DWORD,
    _pLastMissedUsage: *mut f32,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglGetCurrentAssociatedContextAMD = unsafe extern "system" fn() -> HGLRC;
#[allow(non_camel_case_types)]
pub type PFN_wglDisableFrameLockI3D = unsafe extern "system" fn() -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglReleasePbufferDCARB =
    unsafe extern "system" fn(_hPbuffer: HPBUFFERARB, _hDC: HDC) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_wglSetPbufferAttribARB = unsafe extern "system" fn(
    _hPbuffer: HPBUFFERARB,
    _piAttribList: *const std::os::raw::c_int,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglCreateAssociatedContextAMD = unsafe extern "system" fn(_id: UINT) -> HGLRC;
#[allow(non_camel_case_types)]
pub type PFN_wglDisableGenlockI3D = unsafe extern "system" fn(_hDC: HDC) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglGenlockSourceI3D = unsafe extern "system" fn(_hDC: HDC, _uSource: UINT) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglGetPbufferDCEXT = unsafe extern "system" fn(_hPbuffer: HPBUFFEREXT) -> HDC;
#[allow(non_camel_case_types)]
pub type PFN_wglGetProcAddress = unsafe extern "system" fn(_lpszProc: LPCSTR) -> PROC;
#[allow(non_camel_case_types)]
pub type PFN_wglGenlockSourceEdgeI3D = unsafe extern "system" fn(_hDC: HDC, _uEdge: UINT) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglBindTexImageARB =
    unsafe extern "system" fn(_hPbuffer: HPBUFFERARB, _iBuffer: std::os::raw::c_int) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglQueryCurrentContextNV = unsafe extern "system" fn(
    _iAttribute: std::os::raw::c_int,
    _piValue: *mut std::os::raw::c_int,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglGetVideoDeviceNV = unsafe extern "system" fn(
    _hDC: HDC,
    _numDevices: std::os::raw::c_int,
    _hVideoDevice: *mut HPVIDEODEV,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglRestoreBufferRegionARB = unsafe extern "system" fn(
    _hRegion: HANDLE,
    _x: std::os::raw::c_int,
    _y: std::os::raw::c_int,
    _width: std::os::raw::c_int,
    _height: std::os::raw::c_int,
    _xSrc: std::os::raw::c_int,
    _ySrc: std::os::raw::c_int,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglUseFontOutlinesW = unsafe extern "system" fn(
    _hDC: HDC,
    _first: DWORD,
    _count: DWORD,
    _listBase: DWORD,
    _deviation: FLOAT,
    _extrusion: FLOAT,
    _format: std::os::raw::c_int,
    _lpgmf: LPGLYPHMETRICSFLOAT,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglBindSwapBarrierNV =
    unsafe extern "system" fn(_group: GLuint, _barrier: GLuint) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglGetPixelFormatAttribfvEXT = unsafe extern "system" fn(
    _hdc: HDC,
    _iPixelFormat: std::os::raw::c_int,
    _iLayerPlane: std::os::raw::c_int,
    _nAttributes: UINT,
    _piAttributes: *mut std::os::raw::c_int,
    _pfValues: *mut FLOAT,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglUseFontBitmapsA =
    unsafe extern "system" fn(_hDC: HDC, _first: DWORD, _count: DWORD, _listBase: DWORD) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglDestroyImageBufferI3D =
    unsafe extern "system" fn(_hDC: HDC, _pAddress: LPVOID) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglCreateBufferRegionARB =
    unsafe extern "system" fn(_hDC: HDC, _iLayerPlane: std::os::raw::c_int, _uType: UINT) -> HANDLE;
#[allow(non_camel_case_types)]
pub type PFN_wglQueryMaxSwapGroupsNV = unsafe extern "system" fn(
    _hDC: HDC,
    _maxGroups: *mut GLuint,
    _maxBarriers: *mut GLuint,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglGetGenlockSourceEdgeI3D =
    unsafe extern "system" fn(_hDC: HDC, _uEdge: *mut UINT) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglGetDefaultProcAddress = unsafe extern "system" fn(_lpszProc: LPCSTR) -> PROC;
#[allow(non_camel_case_types)]
pub type PFN_wglCreateDisplayColorTableEXT = unsafe extern "system" fn(_id: GLushort) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_wglEnableFrameLockI3D = unsafe extern "system" fn() -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglIsEnabledFrameLockI3D = unsafe extern "system" fn(_pFlag: *mut BOOL) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglCopyImageSubDataNV = unsafe extern "system" fn(
    _hSrcRC: HGLRC,
    _srcName: GLuint,
    _srcTarget: GLenum,
    _srcLevel: GLint,
    _srcX: GLint,
    _srcY: GLint,
    _srcZ: GLint,
    _hDstRC: HGLRC,
    _dstName: GLuint,
    _dstTarget: GLenum,
    _dstLevel: GLint,
    _dstX: GLint,
    _dstY: GLint,
    _dstZ: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_DescribePixelFormat = unsafe extern "system" fn(
    _hdc: HDC,
    _ipfd: std::os::raw::c_int,
    _cjpfd: UINT,
    _ppfd: *const PIXELFORMATDESCRIPTOR,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_wglDXCloseDeviceNV = unsafe extern "system" fn(_hDevice: HANDLE) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglDXOpenDeviceNV =
    unsafe extern "system" fn(_dxDevice: *mut std::os::raw::c_void) -> HANDLE;
#[allow(non_camel_case_types)]
pub type PFN_wglGetExtensionsStringEXT = unsafe extern "system" fn() -> *const char;
#[allow(non_camel_case_types)]
pub type PFN_wglGetGenlockSourceDelayI3D =
    unsafe extern "system" fn(_hDC: HDC, _uDelay: *mut UINT) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglResetFrameCountNV = unsafe extern "system" fn(_hDC: HDC) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglSetStereoEmitterState3DL =
    unsafe extern "system" fn(_hDC: HDC, _uState: UINT) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglReleasePbufferDCEXT =
    unsafe extern "system" fn(_hPbuffer: HPBUFFEREXT, _hDC: HDC) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_wglDXSetResourceShareHandleNV =
    unsafe extern "system" fn(_dxObject: *mut std::os::raw::c_void, _shareHandle: HANDLE) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglGetSyncValuesOML = unsafe extern "system" fn(
    _hdc: HDC,
    _ust: *mut INT64,
    _msc: *mut INT64,
    _sbc: *mut INT64,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglLockVideoCaptureDeviceNV =
    unsafe extern "system" fn(_hDc: HDC, _hDevice: HVIDEOINPUTDEVICENV) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglDeleteDCNV = unsafe extern "system" fn(_hdc: HDC) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglGetPbufferDCARB = unsafe extern "system" fn(_hPbuffer: HPBUFFERARB) -> HDC;
#[allow(non_camel_case_types)]
pub type PFN_wglDelayBeforeSwapNV = unsafe extern "system" fn(_hDC: HDC, _seconds: GLfloat) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglBindDisplayColorTableEXT = unsafe extern "system" fn(_id: GLushort) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_wglAssociateImageBufferEventsI3D = unsafe extern "system" fn(
    _hDC: HDC,
    _pEvent: *const HANDLE,
    _pAddress: *const LPVOID,
    _pSize: *const DWORD,
    _count: UINT,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_GetEnhMetaFilePixelFormat =
    unsafe extern "system" fn(_hemf: HENHMETAFILE, _ppfd: *const PIXELFORMATDESCRIPTOR) -> UINT;
#[allow(non_camel_case_types)]
pub type PFN_wglGetGPUIDsAMD = unsafe extern "system" fn(_maxCount: UINT, _ids: *mut UINT) -> UINT;
#[allow(non_camel_case_types)]
pub type PFN_wglRealizeLayerPalette = unsafe extern "system" fn(
    _hdc: HDC,
    _iLayerPlane: std::os::raw::c_int,
    _bRealize: BOOL,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglGetCurrentReadDCEXT = unsafe extern "system" fn() -> HDC;
#[allow(non_camel_case_types)]
pub type PFN_wglCreatePbufferEXT = unsafe extern "system" fn(
    _hDC: HDC,
    _iPixelFormat: std::os::raw::c_int,
    _iWidth: std::os::raw::c_int,
    _iHeight: std::os::raw::c_int,
    _piAttribList: *const std::os::raw::c_int,
) -> HPBUFFEREXT;
#[allow(non_camel_case_types)]
pub type PFN_wglGetMscRateOML =
    unsafe extern "system" fn(_hdc: HDC, _numerator: *mut INT32, _denominator: *mut INT32) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglDestroyPbufferARB = unsafe extern "system" fn(_hPbuffer: HPBUFFERARB) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglEnumerateVideoDevicesNV = unsafe extern "system" fn(
    _hDc: HDC,
    _phDeviceList: *mut HVIDEOOUTPUTDEVICENV,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_wglSwapLayerBuffersMscOML = unsafe extern "system" fn(
    _hdc: HDC,
    _fuPlanes: INT,
    _target_msc: INT64,
    _divisor: INT64,
    _remainder: INT64,
) -> INT64;
#[allow(non_camel_case_types)]
pub type PFN_wglGetPixelFormatAttribfvARB = unsafe extern "system" fn(
    _hdc: HDC,
    _iPixelFormat: std::os::raw::c_int,
    _iLayerPlane: std::os::raw::c_int,
    _nAttributes: UINT,
    _piAttributes: *const std::os::raw::c_int,
    _pfValues: *mut FLOAT,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglSetLayerPaletteEntries = unsafe extern "system" fn(
    _hdc: HDC,
    _iLayerPlane: std::os::raw::c_int,
    _iStart: std::os::raw::c_int,
    _cEntries: std::os::raw::c_int,
    _pcr: *const COLORREF,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_wglGetGenlockSourceI3D =
    unsafe extern "system" fn(_hDC: HDC, _uSource: *mut UINT) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglGetDigitalVideoParametersI3D = unsafe extern "system" fn(
    _hDC: HDC,
    _iAttribute: std::os::raw::c_int,
    _piValue: *mut std::os::raw::c_int,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglCreateContextAttribsARB = unsafe extern "system" fn(
    _hDC: HDC,
    _hShareContext: HGLRC,
    _attribList: *const std::os::raw::c_int,
) -> HGLRC;
#[allow(non_camel_case_types)]
pub type PFN_wglReleaseVideoImageNV =
    unsafe extern "system" fn(_hPbuffer: HPBUFFERARB, _iVideoBuffer: std::os::raw::c_int) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglSetDigitalVideoParametersI3D = unsafe extern "system" fn(
    _hDC: HDC,
    _iAttribute: std::os::raw::c_int,
    _piValue: *const std::os::raw::c_int,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglQueryFrameLockMasterI3D = unsafe extern "system" fn(_pFlag: *mut BOOL) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglJoinSwapGroupNV = unsafe extern "system" fn(_hDC: HDC, _group: GLuint) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglDeleteBufferRegionARB = unsafe extern "system" fn(_hRegion: HANDLE) -> VOID;
#[allow(non_camel_case_types)]
pub type PFN_wglDXUnlockObjectsNV =
    unsafe extern "system" fn(_hDevice: HANDLE, _count: GLint, _hObjects: *mut HANDLE) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglGetCurrentContext = unsafe extern "system" fn() -> HGLRC;
#[allow(non_camel_case_types)]
pub type PFN_wglLoadDisplayColorTableEXT =
    unsafe extern "system" fn(_table: *const GLushort, _length: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_wglQueryPbufferEXT = unsafe extern "system" fn(
    _hPbuffer: HPBUFFEREXT,
    _iAttribute: std::os::raw::c_int,
    _piValue: *mut std::os::raw::c_int,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglCreateAffinityDCNV = unsafe extern "system" fn(_phGpuList: *const HGPUNV) -> HDC;
#[allow(non_camel_case_types)]
pub type PFN_wglCreateLayerContext =
    unsafe extern "system" fn(_hDc: HDC, _level: std::os::raw::c_int) -> HGLRC;
#[allow(non_camel_case_types)]
pub type PFN_wglGetGammaTableParametersI3D = unsafe extern "system" fn(
    _hDC: HDC,
    _iAttribute: std::os::raw::c_int,
    _piValue: *mut std::os::raw::c_int,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglShareLists =
    unsafe extern "system" fn(_hrcSrvShare: HGLRC, _hrcSrvSource: HGLRC) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglUseFontBitmapsW =
    unsafe extern "system" fn(_hDC: HDC, _first: DWORD, _count: DWORD, _listBase: DWORD) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglUseFontOutlines = unsafe extern "system" fn(
    _hDC: HDC,
    _first: DWORD,
    _count: DWORD,
    _listBase: DWORD,
    _deviation: FLOAT,
    _extrusion: FLOAT,
    _format: std::os::raw::c_int,
    _lpgmf: LPGLYPHMETRICSFLOAT,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglGenlockSampleRateI3D = unsafe extern "system" fn(_hDC: HDC, _uRate: UINT) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglUseFontOutlinesA = unsafe extern "system" fn(
    _hDC: HDC,
    _first: DWORD,
    _count: DWORD,
    _listBase: DWORD,
    _deviation: FLOAT,
    _extrusion: FLOAT,
    _format: std::os::raw::c_int,
    _lpgmf: LPGLYPHMETRICSFLOAT,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglDeleteAssociatedContextAMD = unsafe extern "system" fn(_hglrc: HGLRC) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglSwapBuffersMscOML = unsafe extern "system" fn(
    _hdc: HDC,
    _target_msc: INT64,
    _divisor: INT64,
    _remainder: INT64,
) -> INT64;
#[allow(non_camel_case_types)]
pub type PFN_wglGetContextGPUIDAMD = unsafe extern "system" fn(_hglrc: HGLRC) -> UINT;
#[allow(non_camel_case_types)]
pub type PFN_wglQueryPbufferARB = unsafe extern "system" fn(
    _hPbuffer: HPBUFFERARB,
    _iAttribute: std::os::raw::c_int,
    _piValue: *mut std::os::raw::c_int,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglReleaseImageBufferEventsI3D =
    unsafe extern "system" fn(_hDC: HDC, _pAddress: *const LPVOID, _count: UINT) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_ChoosePixelFormat = unsafe extern "system" fn(
    _hDc: HDC,
    _pPfd: *const PIXELFORMATDESCRIPTOR,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_wglCreateImageBufferI3D =
    unsafe extern "system" fn(_hDC: HDC, _dwSize: DWORD, _uFlags: UINT) -> LPVOID;
#[allow(non_camel_case_types)]
pub type PFN_wglDXUnregisterObjectNV =
    unsafe extern "system" fn(_hDevice: HANDLE, _hObject: HANDLE) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglGetGPUInfoAMD = unsafe extern "system" fn(
    _id: UINT,
    _property: INT,
    _dataType: GLenum,
    _size: UINT,
    _data: *mut std::os::raw::c_void,
) -> INT;
#[allow(non_camel_case_types)]
pub type PFN_wglSetGammaTableI3D = unsafe extern "system" fn(
    _hDC: HDC,
    _iEntries: std::os::raw::c_int,
    _puRed: *const USHORT,
    _puGreen: *const USHORT,
    _puBlue: *const USHORT,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglWaitForMscOML = unsafe extern "system" fn(
    _hdc: HDC,
    _target_msc: INT64,
    _divisor: INT64,
    _remainder: INT64,
    _ust: *mut INT64,
    _msc: *mut INT64,
    _sbc: *mut INT64,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglDXLockObjectsNV =
    unsafe extern "system" fn(_hDevice: HANDLE, _count: GLint, _hObjects: *mut HANDLE) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglReleaseVideoDeviceNV = unsafe extern "system" fn(_hVideoDevice: HPVIDEODEV) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglGetPixelFormatAttribivARB = unsafe extern "system" fn(
    _hdc: HDC,
    _iPixelFormat: std::os::raw::c_int,
    _iLayerPlane: std::os::raw::c_int,
    _nAttributes: UINT,
    _piAttributes: *const std::os::raw::c_int,
    _piValues: *mut std::os::raw::c_int,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglQueryFrameCountNV =
    unsafe extern "system" fn(_hDC: HDC, _count: *mut GLuint) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglDXObjectAccessNV =
    unsafe extern "system" fn(_hObject: HANDLE, _access: GLenum) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglDXRegisterObjectNV = unsafe extern "system" fn(
    _hDevice: HANDLE,
    _dxObject: *mut std::os::raw::c_void,
    _name: GLuint,
    _type: GLenum,
    _access: GLenum,
) -> HANDLE;
#[allow(non_camel_case_types)]
pub type PFN_wglBindVideoDeviceNV = unsafe extern "system" fn(
    _hDc: HDC,
    _uVideoSlot: std::os::raw::c_uint,
    _hVideoDevice: HVIDEOOUTPUTDEVICENV,
    _piAttribList: *const std::os::raw::c_int,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglCopyContext =
    unsafe extern "system" fn(_hglrcSrc: HGLRC, _hglrcDst: HGLRC, _mask: UINT) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglDeleteContext = unsafe extern "system" fn(_oldContext: HGLRC) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglGetLayerPaletteEntries = unsafe extern "system" fn(
    _hdc: HDC,
    _iLayerPlane: std::os::raw::c_int,
    _iStart: std::os::raw::c_int,
    _cEntries: std::os::raw::c_int,
    _pcr: *const COLORREF,
) -> std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type PFN_wglQueryVideoCaptureDeviceNV = unsafe extern "system" fn(
    _hDc: HDC,
    _hDevice: HVIDEOINPUTDEVICENV,
    _iAttribute: std::os::raw::c_int,
    _piValue: *mut std::os::raw::c_int,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglMakeAssociatedContextCurrentAMD = unsafe extern "system" fn(_hglrc: HGLRC) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglWaitForSbcOML = unsafe extern "system" fn(
    _hdc: HDC,
    _target_sbc: INT64,
    _ust: *mut INT64,
    _msc: *mut INT64,
    _sbc: *mut INT64,
) -> BOOL;
#[allow(non_camel_case_types)]
pub type PFN_wglCreatePbufferARB = unsafe extern "system" fn(
    _hDC: HDC,
    _iPixelFormat: std::os::raw::c_int,
    _iWidth: std::os::raw::c_int,
    _iHeight: std::os::raw::c_int,
    _piAttribList: *const std::os::raw::c_int,
) -> HPBUFFERARB;
#[allow(non_camel_case_types)]
pub type PFN_wglSaveBufferRegionARB = unsafe extern "system" fn(
    _hRegion: HANDLE,
    _x: std::os::raw::c_int,
    _y: std::os::raw::c_int,
    _width: std::os::raw::c_int,
    _height: std::os::raw::c_int,
) -> BOOL;
