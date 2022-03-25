use crate::gl;
use crate::types::*;
use gl::bitflags::*;
use gl::enums::*;
use std::ffi::c_void;
use std::fmt;
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4bvARB = unsafe extern "system" fn(_index: GLuint, _v: *const GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glProgramLocalParameter4fARB = unsafe extern "system" fn(
    _target: ProgramTarget,
    _index: GLuint,
    _x: GLfloat,
    _y: GLfloat,
    _z: GLfloat,
    _w: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColorP3uiv =
    unsafe extern "system" fn(_type: ColorPointerType, _color: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVideoCaptureStreamParameterdvNV = unsafe extern "system" fn(
    _video_capture_slot: GLuint,
    _stream: GLuint,
    _pname: GLenum,
    _params: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glDrawElements = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _count: GLsizei,
    _type: DrawElementsType,
    _indices: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord4xOES =
    unsafe extern "system" fn(_s: GLfixed, _t: GLfixed, _r: GLfixed, _q: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord1fARB = unsafe extern "system" fn(_target: TextureUnit, _s: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glFlushRasterSGIX = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glDrawArrays =
    unsafe extern "system" fn(_mode: PrimitiveType, _first: GLint, _count: GLsizei);
#[allow(non_camel_case_types)]
pub type PFN_glIsStateNV = unsafe extern "system" fn(_state: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glGetTextureSubImage = unsafe extern "system" fn(
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
    _pixels: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glScissorExclusiveNV =
    unsafe extern "system" fn(_x: GLint, _y: GLint, _width: GLsizei, _height: GLsizei);
#[allow(non_camel_case_types)]
pub type PFN_glMakeNamedBufferNonResidentNV = unsafe extern "system" fn(_buffer: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glCopyMultiTexImage2DEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _level: GLint,
    _internalformat: InternalFormat,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _border: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetDriverControlsQCOM =
    unsafe extern "system" fn(_num: *mut GLint, _size: GLsizei, _driverControls: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glIndexFuncEXT = unsafe extern "system" fn(_func: IndexFunctionEXT, _ref: GLclampf);
#[allow(non_camel_case_types)]
pub type PFN_glLineStipple = unsafe extern "system" fn(_factor: GLint, _pattern: GLushort);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform3ui64vARB = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord3xvOES = unsafe extern "system" fn(_coords: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glTransformFeedbackVaryings = unsafe extern "system" fn(
    _program: GLuint,
    _count: GLsizei,
    _varyings: *const *const GLchar,
    _bufferMode: TransformFeedbackBufferMode,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib1fvARB = unsafe extern "system" fn(_index: GLuint, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glNamedProgramLocalParameterI4uiEXT = unsafe extern "system" fn(
    _program: GLuint,
    _target: ProgramTarget,
    _index: GLuint,
    _x: GLuint,
    _y: GLuint,
    _z: GLuint,
    _w: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord3fvARB =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glColor3fVertex3fvSUN =
    unsafe extern "system" fn(_c: *const GLfloat, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glOrthof = unsafe extern "system" fn(
    _l: GLfloat,
    _r: GLfloat,
    _b: GLfloat,
    _t: GLfloat,
    _n: GLfloat,
    _f: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib2dNV =
    unsafe extern "system" fn(_index: GLuint, _x: GLdouble, _y: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream3fATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _x: GLfloat, _y: GLfloat, _z: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glUniform3f =
    unsafe extern "system" fn(_location: GLint, _v0: GLfloat, _v1: GLfloat, _v2: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glMulticastBlitFramebufferNV = unsafe extern "system" fn(
    _srcGpu: GLuint,
    _dstGpu: GLuint,
    _srcX0: GLint,
    _srcY0: GLint,
    _srcX1: GLint,
    _srcY1: GLint,
    _dstX0: GLint,
    _dstY0: GLint,
    _dstX1: GLint,
    _dstY1: GLint,
    _mask: ClearBufferMask,
    _filter: GLenum,
);
#[allow(non_camel_case_types)]
pub type PFN_glCreateShaderProgramvEXT = unsafe extern "system" fn(
    _type: ShaderType,
    _count: GLsizei,
    _strings: *mut *const GLchar,
) -> GLuint;
#[allow(non_camel_case_types)]
pub type PFN_glPrimitiveRestartNV = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glGetPixelMapusv = unsafe extern "system" fn(_map: PixelMap, _values: *mut GLushort);
#[allow(non_camel_case_types)]
pub type PFN_glProgramEnvParameter4dARB = unsafe extern "system" fn(
    _target: ProgramTarget,
    _index: GLuint,
    _x: GLdouble,
    _y: GLdouble,
    _z: GLdouble,
    _w: GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glElementPointerAPPLE =
    unsafe extern "system" fn(_type: ElementPointerTypeATI, _pointer: *const std::os::raw::c_void);
#[allow(non_camel_case_types)]
pub type PFN_glEGLImageTargetTexture2DOES =
    unsafe extern "system" fn(_target: GLenum, _image: GLeglImageOES);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib1sARB = unsafe extern "system" fn(_index: GLuint, _x: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glIsEnablediNV =
    unsafe extern "system" fn(_target: EnableCap, _index: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glUniformMatrix2x4dv = unsafe extern "system" fn(
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureRenderbufferEXT =
    unsafe extern "system" fn(_texture: GLuint, _target: TextureTarget, _renderbuffer: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos2fMESA = unsafe extern "system" fn(_x: GLfloat, _y: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glImageTransformParameterfHP = unsafe extern "system" fn(
    _target: ImageTransformTargetHP,
    _pname: ImageTransformPNameHP,
    _param: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform3uiEXT =
    unsafe extern "system" fn(_location: GLint, _v0: GLuint, _v1: GLuint, _v2: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiModeDrawArraysIBM = unsafe extern "system" fn(
    _mode: *const PrimitiveType,
    _first: *const GLint,
    _count: *const GLsizei,
    _primcount: GLsizei,
    _modestride: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glNamedRenderbufferStorageMultisample = unsafe extern "system" fn(
    _renderbuffer: GLuint,
    _samples: GLsizei,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos4sMESA =
    unsafe extern "system" fn(_x: GLshort, _y: GLshort, _z: GLshort, _w: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glImportSemaphoreWin32NameEXT = unsafe extern "system" fn(
    _semaphore: GLuint,
    _handleType: ExternalHandleType,
    _name: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glPopAttrib = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColorP3ui =
    unsafe extern "system" fn(_type: ColorPointerType, _color: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix3x4fv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos3dMESA =
    unsafe extern "system" fn(_x: GLdouble, _y: GLdouble, _z: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glGetPathTexGenivNV =
    unsafe extern "system" fn(_texCoordSet: TextureUnit, _pname: PathGenMode, _value: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform1ui64ARB =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _x: GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glColor3hvNV = unsafe extern "system" fn(_v: *const GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glVertex4iv = unsafe extern "system" fn(_v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream1dATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _x: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glBindImageTexture = unsafe extern "system" fn(
    _unit: GLuint,
    _texture: GLuint,
    _level: GLint,
    _layered: GLboolean,
    _layer: GLint,
    _access: BufferAccessARB,
    _format: InternalFormat,
);
#[allow(non_camel_case_types)]
pub type PFN_glDrawTransformFeedbackStreamInstanced = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _id: GLuint,
    _stream: GLuint,
    _instancecount: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramEnvParameterI4uivNV =
    unsafe extern "system" fn(_target: ProgramTarget, _index: GLuint, _params: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glColorSubTable = unsafe extern "system" fn(
    _target: ColorTableTarget,
    _start: GLsizei,
    _count: GLsizei,
    _format: PixelFormat,
    _type: PixelType,
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetTexLevelParameteriv = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _pname: GetTextureParameter,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glLoadTransposeMatrixfARB = unsafe extern "system" fn(_m: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glBindImageTextures =
    unsafe extern "system" fn(_first: GLuint, _count: GLsizei, _textures: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glBindShadingRateImageNV = unsafe extern "system" fn(_texture: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixMultTranspose3x3fNV =
    unsafe extern "system" fn(_matrixMode: GLenum, _m: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glSampleCoverage = unsafe extern "system" fn(_value: GLfloat, _invert: GLboolean);
#[allow(non_camel_case_types)]
pub type PFN_glPatchParameterfv =
    unsafe extern "system" fn(_pname: PatchParameterName, _values: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream2svATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _coords: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glGetInteger64i_v =
    unsafe extern "system" fn(_target: GetPName, _index: GLuint, _data: *mut GLint64);
#[allow(non_camel_case_types)]
pub type PFN_glGetInternalformatSampleivNV = unsafe extern "system" fn(
    _target: TextureTarget,
    _internalformat: InternalFormat,
    _samples: GLsizei,
    _pname: InternalFormatPName,
    _count: GLsizei,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord4hvNV =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glMemoryBarrierEXT = unsafe extern "system" fn(_barriers: MemoryBarrierMask);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord4xvOES = unsafe extern "system" fn(_coords: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glDrawTexfOES = unsafe extern "system" fn(
    _x: GLfloat,
    _y: GLfloat,
    _z: GLfloat,
    _width: GLfloat,
    _height: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetVariantArrayObjectivATI =
    unsafe extern "system" fn(_id: GLuint, _pname: ArrayObjectPNameATI, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix4fvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferTextureLayerDownsampleIMG = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _attachment: FramebufferAttachment,
    _texture: GLuint,
    _level: GLint,
    _layer: GLint,
    _xscale: GLint,
    _yscale: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glTessellationFactorAMD = unsafe extern "system" fn(_factor: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexArrayVertexBindingDivisorEXT =
    unsafe extern "system" fn(_vaobj: GLuint, _bindingindex: GLuint, _divisor: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetMapParameterfvNV =
    unsafe extern "system" fn(_target: EvalTargetNV, _pname: MapParameterNV, _params: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glIsRenderbufferOES = unsafe extern "system" fn(_renderbuffer: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glMap1xOES = unsafe extern "system" fn(
    _target: MapTarget,
    _u1: GLfixed,
    _u2: GLfixed,
    _stride: GLint,
    _order: GLint,
    _points: GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glNamedProgramLocalParametersI4ivEXT = unsafe extern "system" fn(
    _program: GLuint,
    _target: ProgramTarget,
    _index: GLuint,
    _count: GLsizei,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glPNTrianglesfATI =
    unsafe extern "system" fn(_pname: PNTrianglesPNameATI, _param: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glTexEnvxOES = unsafe extern "system" fn(
    _target: TextureEnvTarget,
    _pname: TextureEnvParameter,
    _param: GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib2svNV = unsafe extern "system" fn(_index: GLuint, _v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glCombinerInputNV = unsafe extern "system" fn(
    _stage: CombinerStageNV,
    _portion: CombinerPortionNV,
    _variable: CombinerVariableNV,
    _input: CombinerRegisterNV,
    _mapping: CombinerMappingNV,
    _componentUsage: CombinerComponentUsageNV,
);
#[allow(non_camel_case_types)]
pub type PFN_glDebugMessageCallbackAMD =
    unsafe extern "system" fn(_callback: GLDEBUGPROCAMD, _userParam: *mut std::os::raw::c_void);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform4dEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _x: GLdouble,
    _y: GLdouble,
    _z: GLdouble,
    _w: GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform2i64vARB =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLint64);
#[allow(non_camel_case_types)]
pub type PFN_glGetNamedBufferPointerv = unsafe extern "system" fn(
    _buffer: GLuint,
    _pname: BufferPointerNameARB,
    _params: *mut *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glVDPAUMapSurfacesNV =
    unsafe extern "system" fn(_numSurfaces: GLsizei, _surfaces: *const GLvdpauSurfaceNV);
#[allow(non_camel_case_types)]
pub type PFN_glLightModelx =
    unsafe extern "system" fn(_pname: LightModelParameter, _param: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glGetMultisamplefv =
    unsafe extern "system" fn(_pname: GetMultisamplePNameNV, _index: GLuint, _val: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetnMapdv = unsafe extern "system" fn(
    _target: MapTarget,
    _query: MapQuery,
    _bufSize: GLsizei,
    _v: *mut GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetCompressedTexImageARB = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _img: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetNamedRenderbufferParameterivEXT = unsafe extern "system" fn(
    _renderbuffer: GLuint,
    _pname: RenderbufferParameterName,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform2fv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glQueryResourceNV = unsafe extern "system" fn(
    _queryType: GLenum,
    _tagId: GLint,
    _count: GLuint,
    _buffer: *mut GLint,
) -> GLint;
#[allow(non_camel_case_types)]
pub type PFN_glUniform2iv =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib3dvARB = unsafe extern "system" fn(_index: GLuint, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glBindFragDataLocationIndexedEXT = unsafe extern "system" fn(
    _program: GLuint,
    _colorNumber: GLuint,
    _index: GLuint,
    _name: *const GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glActiveStencilFaceEXT = unsafe extern "system" fn(_face: StencilFaceDirection);
#[allow(non_camel_case_types)]
pub type PFN_glClientWaitSync =
    unsafe extern "system" fn(_sync: GLsync, _flags: SyncObjectMask, _timeout: GLuint64) -> GLenum;
#[allow(non_camel_case_types)]
pub type PFN_glFragmentLightModeliSGIX =
    unsafe extern "system" fn(_pname: FragmentLightModelParameterSGIX, _param: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetnPixelMapuivARB =
    unsafe extern "system" fn(_map: PixelMap, _bufSize: GLsizei, _values: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glBindFramebufferEXT =
    unsafe extern "system" fn(_target: FramebufferTarget, _framebuffer: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glCompileShader = unsafe extern "system" fn(_shader: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetHistogramParameterivEXT = unsafe extern "system" fn(
    _target: HistogramTargetEXT,
    _pname: GetHistogramParameterPNameEXT,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetnUniformui64vARB = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _bufSize: GLsizei,
    _params: *mut GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glIndexFormatNV = unsafe extern "system" fn(_type: GLenum, _stride: GLsizei);
#[allow(non_camel_case_types)]
pub type PFN_glMap2f = unsafe extern "system" fn(
    _target: MapTarget,
    _u1: GLfloat,
    _u2: GLfloat,
    _ustride: GLint,
    _uorder: GLint,
    _v1: GLfloat,
    _v2: GLfloat,
    _vstride: GLint,
    _vorder: GLint,
    _points: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetTexParameterIuivEXT = unsafe extern "system" fn(
    _target: TextureTarget,
    _pname: GetTextureParameter,
    _params: *mut GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glIndexMaterialEXT =
    unsafe extern "system" fn(_face: MaterialFace, _mode: IndexMaterialParameterEXT);
#[allow(non_camel_case_types)]
pub type PFN_glVertex3xOES = unsafe extern "system" fn(_x: GLfixed, _y: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glGetPointeri_vEXT = unsafe extern "system" fn(
    _pname: GLenum,
    _index: GLuint,
    _params: *mut *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glPolygonOffsetx = unsafe extern "system" fn(_factor: GLfixed, _units: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3usvEXT = unsafe extern "system" fn(_v: *const GLushort);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib1dvNV = unsafe extern "system" fn(_index: GLuint, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glBeginOcclusionQueryNV = unsafe extern "system" fn(_id: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetTransformFeedbackVarying = unsafe extern "system" fn(
    _program: GLuint,
    _index: GLuint,
    _bufSize: GLsizei,
    _length: *mut GLsizei,
    _size: *mut GLsizei,
    _type: *mut AttributeType,
    _name: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glPrimitiveBoundingBoxOES = unsafe extern "system" fn(
    _minX: GLfloat,
    _minY: GLfloat,
    _minZ: GLfloat,
    _minW: GLfloat,
    _maxX: GLfloat,
    _maxY: GLfloat,
    _maxZ: GLfloat,
    _maxW: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glScalex = unsafe extern "system" fn(_x: GLfixed, _y: GLfixed, _z: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glTexGenfOES = unsafe extern "system" fn(
    _coord: TextureCoordName,
    _pname: TextureGenParameter,
    _param: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glFreeObjectBufferATI = unsafe extern "system" fn(_buffer: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glEGLImageTargetTexStorageEXT =
    unsafe extern "system" fn(_target: GLenum, _image: GLeglImageOES, _attrib_list: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glEnableVariantClientStateEXT = unsafe extern "system" fn(_id: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferTexture3DEXT = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _attachment: FramebufferAttachment,
    _textarget: TextureTarget,
    _texture: GLuint,
    _level: GLint,
    _zoffset: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixLoadTransposedEXT =
    unsafe extern "system" fn(_mode: MatrixMode, _m: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glSetInvariantEXT =
    unsafe extern "system" fn(_id: GLuint, _type: ScalarType, _addr: *const std::os::raw::c_void);
#[allow(non_camel_case_types)]
pub type PFN_glMultiDrawElementsEXT = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _count: *const GLsizei,
    _type: DrawElementsType,
    _indices: *const *const std::os::raw::c_void,
    _primcount: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureSubImage2D = unsafe extern "system" fn(
    _texture: GLuint,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _format: PixelFormat,
    _type: PixelType,
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glCopyImageSubData = unsafe extern "system" fn(
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
);
#[allow(non_camel_case_types)]
pub type PFN_glSemaphoreParameterui64vEXT = unsafe extern "system" fn(
    _semaphore: GLuint,
    _pname: SemaphoreParameterName,
    _params: *const GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureParameteri =
    unsafe extern "system" fn(_texture: GLuint, _pname: TextureParameterName, _param: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glDrawTexxOES = unsafe extern "system" fn(
    _x: GLfixed,
    _y: GLfixed,
    _z: GLfixed,
    _width: GLfixed,
    _height: GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribDivisorEXT = unsafe extern "system" fn(_index: GLuint, _divisor: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glBufferPageCommitmentMemNV = unsafe extern "system" fn(
    _target: BufferStorageTarget,
    _offset: GLintptr,
    _size: GLsizeiptr,
    _memory: GLuint,
    _memOffset: GLuint64,
    _commit: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureBufferRangeEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _internalformat: SizedInternalFormat,
    _buffer: GLuint,
    _offset: GLintptr,
    _size: GLsizeiptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexArrayIntegeri_vEXT = unsafe extern "system" fn(
    _vaobj: GLuint,
    _index: GLuint,
    _pname: VertexArrayPName,
    _param: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertex4bvOES = unsafe extern "system" fn(_coords: *const GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord3bOES = unsafe extern "system" fn(_s: GLbyte, _t: GLbyte, _r: GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glUseShaderProgramEXT = unsafe extern "system" fn(_type: GLenum, _program: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGenPerfMonitorsAMD = unsafe extern "system" fn(_n: GLsizei, _monitors: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glUniform3ivARB =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL3d =
    unsafe extern "system" fn(_index: GLuint, _x: GLdouble, _y: GLdouble, _z: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glFinishFenceNV = unsafe extern "system" fn(_fence: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glReplacementCodeuiColor3fVertex3fSUN = unsafe extern "system" fn(
    _rc: GLuint,
    _r: GLfloat,
    _g: GLfloat,
    _b: GLfloat,
    _x: GLfloat,
    _y: GLfloat,
    _z: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glWriteMaskEXT = unsafe extern "system" fn(
    _res: GLuint,
    _in: GLuint,
    _outX: VertexShaderWriteMaskEXT,
    _outY: VertexShaderWriteMaskEXT,
    _outZ: VertexShaderWriteMaskEXT,
    _outW: VertexShaderWriteMaskEXT,
);
#[allow(non_camel_case_types)]
pub type PFN_glEndQuery = unsafe extern "system" fn(_target: QueryTarget);
#[allow(non_camel_case_types)]
pub type PFN_glGetBufferSubData = unsafe extern "system" fn(
    _target: BufferTargetARB,
    _offset: GLintptr,
    _size: GLsizeiptr,
    _data: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glClipControl =
    unsafe extern "system" fn(_origin: ClipControlOrigin, _depth: ClipControlDepth);
#[allow(non_camel_case_types)]
pub type PFN_glProgramParameters4dvNV = unsafe extern "system" fn(
    _target: VertexAttribEnumNV,
    _index: GLuint,
    _count: GLsizei,
    _v: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glIndexPointer = unsafe extern "system" fn(
    _type: IndexPointerType,
    _stride: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribs4ubvNV =
    unsafe extern "system" fn(_index: GLuint, _count: GLsizei, _v: *const GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos3d = unsafe extern "system" fn(_x: GLdouble, _y: GLdouble, _z: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glTextureParameterfv = unsafe extern "system" fn(
    _texture: GLuint,
    _pname: TextureParameterName,
    _param: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos3i = unsafe extern "system" fn(_x: GLint, _y: GLint, _z: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glTextureStorage1DEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: GLenum,
    _levels: GLsizei,
    _internalformat: SizedInternalFormat,
    _width: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib1fNV = unsafe extern "system" fn(_index: GLuint, _x: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGenProgramPipelines =
    unsafe extern "system" fn(_n: GLsizei, _pipelines: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMaterialfv = unsafe extern "system" fn(
    _face: MaterialFace,
    _pname: MaterialParameter,
    _params: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glCopyTexImage2D = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _internalformat: InternalFormat,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _border: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glFlush = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glFlushMappedNamedBufferRangeEXT =
    unsafe extern "system" fn(_buffer: GLuint, _offset: GLintptr, _length: GLsizeiptr);
#[allow(non_camel_case_types)]
pub type PFN_glGenQueriesARB = unsafe extern "system" fn(_n: GLsizei, _ids: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord1xOES = unsafe extern "system" fn(_texture: TextureUnit, _s: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glVertex2bOES = unsafe extern "system" fn(_x: GLbyte, _y: GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glGetVaryingLocationNV =
    unsafe extern "system" fn(_program: GLuint, _name: *const GLchar) -> GLint;
#[allow(non_camel_case_types)]
pub type PFN_glShaderOp1EXT =
    unsafe extern "system" fn(_op: VertexShaderOpEXT, _res: GLuint, _arg1: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glTexStorage1D = unsafe extern "system" fn(
    _target: TextureTarget,
    _levels: GLsizei,
    _internalformat: SizedInternalFormat,
    _width: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexStorageMem2DEXT = unsafe extern "system" fn(
    _target: TextureTarget,
    _levels: GLsizei,
    _internalFormat: SizedInternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _memory: GLuint,
    _offset: GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertex2sv = unsafe extern "system" fn(_v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glTexParameterIivEXT = unsafe extern "system" fn(
    _target: TextureTarget,
    _pname: TextureParameterName,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixIndexusvARB =
    unsafe extern "system" fn(_size: GLint, _indices: *const GLushort);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4NusvARB =
    unsafe extern "system" fn(_index: GLuint, _v: *const GLushort);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream4iATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _x: GLint, _y: GLint, _z: GLint, _w: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glWeightusvARB = unsafe extern "system" fn(_size: GLint, _weights: *const GLushort);
#[allow(non_camel_case_types)]
pub type PFN_glCopyTextureSubImage2D = unsafe extern "system" fn(
    _texture: GLuint,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetVkProcAddrNV = unsafe extern "system" fn(_name: *const GLchar) -> GLVULKANPROCNV;
#[allow(non_camel_case_types)]
pub type PFN_glProgramStringARB = unsafe extern "system" fn(
    _target: ProgramTarget,
    _format: ProgramFormat,
    _len: GLsizei,
    _string: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL4d = unsafe extern "system" fn(
    _index: GLuint,
    _x: GLdouble,
    _y: GLdouble,
    _z: GLdouble,
    _w: GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glIsOcclusionQueryNV = unsafe extern "system" fn(_id: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glProgramBufferParametersIuivNV = unsafe extern "system" fn(
    _target: ProgramTarget,
    _bindingIndex: GLuint,
    _wordIndex: GLuint,
    _count: GLsizei,
    _params: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glColorTable = unsafe extern "system" fn(
    _target: ColorTableTarget,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _format: PixelFormat,
    _type: PixelType,
    _table: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL4dvEXT = unsafe extern "system" fn(_index: GLuint, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream1svATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _coords: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferTexture1D = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _attachment: FramebufferAttachment,
    _textarget: TextureTarget,
    _texture: GLuint,
    _level: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glInvalidateBufferSubData =
    unsafe extern "system" fn(_buffer: GLuint, _offset: GLintptr, _length: GLsizeiptr);
#[allow(non_camel_case_types)]
pub type PFN_glUniform2ui64NV =
    unsafe extern "system" fn(_location: GLint, _x: GLuint64EXT, _y: GLuint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glResizeBuffersMESA = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glBeginPerfQueryINTEL = unsafe extern "system" fn(_queryHandle: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL2ui64vNV =
    unsafe extern "system" fn(_index: GLuint, _v: *const GLuint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glViewportIndexedfOES =
    unsafe extern "system" fn(_index: GLuint, _x: GLfloat, _y: GLfloat, _w: GLfloat, _h: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3b =
    unsafe extern "system" fn(_red: GLbyte, _green: GLbyte, _blue: GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glRasterSamplesEXT =
    unsafe extern "system" fn(_samples: GLuint, _fixedsamplelocations: GLboolean);
#[allow(non_camel_case_types)]
pub type PFN_glBindTextures =
    unsafe extern "system" fn(_first: GLuint, _count: GLsizei, _textures: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMapBufferARB = unsafe extern "system" fn(
    _target: BufferTargetARB,
    _access: BufferAccessARB,
) -> *mut std::os::raw::c_void;
#[allow(non_camel_case_types)]
pub type PFN_glUseProgramObjectARB = unsafe extern "system" fn(_programObj: GLhandleARB);
#[allow(non_camel_case_types)]
pub type PFN_glLineWidthxOES = unsafe extern "system" fn(_width: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glBindFragDataLocationEXT =
    unsafe extern "system" fn(_program: GLuint, _color: GLuint, _name: *const GLchar);
#[allow(non_camel_case_types)]
pub type PFN_glDepthBoundsEXT = unsafe extern "system" fn(_zmin: GLclampd, _zmax: GLclampd);
#[allow(non_camel_case_types)]
pub type PFN_glGetOcclusionQueryivNV = unsafe extern "system" fn(
    _id: GLuint,
    _pname: OcclusionQueryParameterNameNV,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glMapNamedBuffer = unsafe extern "system" fn(
    _buffer: GLuint,
    _access: BufferAccessARB,
) -> *mut std::os::raw::c_void;
#[allow(non_camel_case_types)]
pub type PFN_glVertex3d = unsafe extern "system" fn(_x: GLdouble, _y: GLdouble, _z: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteProgramPipelines =
    unsafe extern "system" fn(_n: GLsizei, _pipelines: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glViewportIndexedf =
    unsafe extern "system" fn(_index: GLuint, _x: GLfloat, _y: GLfloat, _w: GLfloat, _h: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord4sv =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteSync = unsafe extern "system" fn(_sync: GLsync);
#[allow(non_camel_case_types)]
pub type PFN_glGetUniformiv =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glTexBufferEXT = unsafe extern "system" fn(
    _target: TextureTarget,
    _internalformat: SizedInternalFormat,
    _buffer: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVariantivEXT = unsafe extern "system" fn(_id: GLuint, _addr: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord3f =
    unsafe extern "system" fn(_target: TextureUnit, _s: GLfloat, _t: GLfloat, _r: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoordP1uiv = unsafe extern "system" fn(
    _texture: TextureUnit,
    _type: TexCoordPointerType,
    _coords: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glRects =
    unsafe extern "system" fn(_x1: GLshort, _y1: GLshort, _x2: GLshort, _y2: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glShaderOp2EXT =
    unsafe extern "system" fn(_op: VertexShaderOpEXT, _res: GLuint, _arg1: GLuint, _arg2: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glUniformMatrix2x4fv = unsafe extern "system" fn(
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexArrayVertexOffsetEXT = unsafe extern "system" fn(
    _vaobj: GLuint,
    _buffer: GLuint,
    _size: GLint,
    _type: VertexPointerType,
    _stride: GLsizei,
    _offset: GLintptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteQueriesEXT = unsafe extern "system" fn(_n: GLsizei, _ids: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glFogxvOES = unsafe extern "system" fn(_pname: FogPName, _param: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glGetActiveVaryingNV = unsafe extern "system" fn(
    _program: GLuint,
    _index: GLuint,
    _bufSize: GLsizei,
    _length: *mut GLsizei,
    _size: *mut GLsizei,
    _type: *mut GLenum,
    _name: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glPixelMapx =
    unsafe extern "system" fn(_map: PixelMap, _size: GLint, _values: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glIndexubv = unsafe extern "system" fn(_c: *const GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glClientWaitSemaphoreui64NVX = unsafe extern "system" fn(
    _fenceObjectCount: GLsizei,
    _semaphoreArray: *const GLuint,
    _fenceValueArray: *const GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glShaderOp3EXT = unsafe extern "system" fn(
    _op: VertexShaderOpEXT,
    _res: GLuint,
    _arg1: GLuint,
    _arg2: GLuint,
    _arg3: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertex3s = unsafe extern "system" fn(_x: GLshort, _y: GLshort, _z: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glDrawTransformFeedbackInstancedEXT =
    unsafe extern "system" fn(_mode: PrimitiveType, _id: GLuint, _instancecount: GLsizei);
#[allow(non_camel_case_types)]
pub type PFN_glFragmentLightModelfSGIX =
    unsafe extern "system" fn(_pname: FragmentLightModelParameterSGIX, _param: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glLoadTransposeMatrixf = unsafe extern "system" fn(_m: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform3uiv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix2x3fv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord1i = unsafe extern "system" fn(_s: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord2bOES =
    unsafe extern "system" fn(_texture: TextureUnit, _s: GLbyte, _t: GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glConvolutionParameterfv = unsafe extern "system" fn(
    _target: ConvolutionTarget,
    _pname: ConvolutionParameterEXT,
    _params: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetTextureSamplerHandleIMG =
    unsafe extern "system" fn(_texture: GLuint, _sampler: GLuint) -> GLuint64;
#[allow(non_camel_case_types)]
pub type PFN_glPolygonOffsetEXT = unsafe extern "system" fn(_factor: GLfloat, _bias: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord2hNV = unsafe extern "system" fn(_s: GLhalfNV, _t: GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glMapTexture2DINTEL = unsafe extern "system" fn(
    _texture: GLuint,
    _level: GLint,
    _access: GLbitfield,
    _stride: *mut GLint,
    _layout: *mut GLenum,
) -> *mut std::os::raw::c_void;
#[allow(non_camel_case_types)]
pub type PFN_glCompressedTexSubImage2D = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _format: InternalFormat,
    _imageSize: GLsizei,
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4ubvNV = unsafe extern "system" fn(_index: GLuint, _v: *const GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI4ubvEXT = unsafe extern "system" fn(_index: GLuint, _v: *const GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glClearNamedFramebufferfi = unsafe extern "system" fn(
    _framebuffer: GLuint,
    _buffer: Buffer,
    _drawbuffer: GLint,
    _depth: GLfloat,
    _stencil: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glNormalPointer = unsafe extern "system" fn(
    _type: NormalPointerType,
    _stride: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glMap2d = unsafe extern "system" fn(
    _target: MapTarget,
    _u1: GLdouble,
    _u2: GLdouble,
    _ustride: GLint,
    _uorder: GLint,
    _v1: GLdouble,
    _v2: GLdouble,
    _vstride: GLint,
    _vorder: GLint,
    _points: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix2dvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glStencilOpSeparateATI = unsafe extern "system" fn(
    _face: StencilFaceDirection,
    _sfail: StencilOp,
    _dpfail: StencilOp,
    _dppass: StencilOp,
);
#[allow(non_camel_case_types)]
pub type PFN_glClearDepthf = unsafe extern "system" fn(_d: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glDrawTransformFeedbackEXT =
    unsafe extern "system" fn(_mode: PrimitiveType, _id: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glTextureSubImage1D = unsafe extern "system" fn(
    _texture: GLuint,
    _level: GLint,
    _xoffset: GLint,
    _width: GLsizei,
    _format: PixelFormat,
    _type: PixelType,
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribs2hvNV =
    unsafe extern "system" fn(_index: GLuint, _n: GLsizei, _v: *const GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glVertexWeightPointerEXT = unsafe extern "system" fn(
    _size: GLint,
    _type: VertexWeightPointerTypeEXT,
    _stride: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glFlushVertexArrayRangeAPPLE =
    unsafe extern "system" fn(_length: GLsizei, _pointer: *mut std::os::raw::c_void);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribP3uiv = unsafe extern "system" fn(
    _index: GLuint,
    _type: VertexAttribPointerType,
    _normalized: GLboolean,
    _value: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glIsVertexArrayAPPLE = unsafe extern "system" fn(_array: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glLGPUCopyImageSubDataNVX = unsafe extern "system" fn(
    _sourceGpu: GLuint,
    _destinationGpuMask: GLbitfield,
    _srcName: GLuint,
    _srcTarget: GLenum,
    _srcLevel: GLint,
    _srcX: GLint,
    _srxY: GLint,
    _srcZ: GLint,
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
pub type PFN_glTextureBarrier = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glViewportSwizzleNV = unsafe extern "system" fn(
    _index: GLuint,
    _swizzlex: GLenum,
    _swizzley: GLenum,
    _swizzlez: GLenum,
    _swizzlew: GLenum,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetnPolygonStipple =
    unsafe extern "system" fn(_bufSize: GLsizei, _pattern: *mut GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glGenBuffers = unsafe extern "system" fn(_n: GLsizei, _buffers: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetPerfMonitorCountersAMD = unsafe extern "system" fn(
    _group: GLuint,
    _numCounters: *mut GLint,
    _maxActiveCounters: *mut GLint,
    _counterSize: GLsizei,
    _counters: *mut GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetPathColorGenfvNV =
    unsafe extern "system" fn(_color: PathColor, _pname: PathGenMode, _value: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix4dvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glLoadMatrixd = unsafe extern "system" fn(_m: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glMultiDrawArraysIndirectAMD = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _indirect: *const std::os::raw::c_void,
    _primcount: GLsizei,
    _stride: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glFogCoordfEXT = unsafe extern "system" fn(_coord: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glDepthRangeIndexed =
    unsafe extern "system" fn(_index: GLuint, _n: GLdouble, _f: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glColorP4uiv =
    unsafe extern "system" fn(_type: ColorPointerType, _color: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glObjectPtrLabelKHR = unsafe extern "system" fn(
    _ptr: *const std::os::raw::c_void,
    _length: GLsizei,
    _label: *const GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glReplacementCodeuiTexCoord2fVertex3fSUN = unsafe extern "system" fn(
    _rc: GLuint,
    _s: GLfloat,
    _t: GLfloat,
    _x: GLfloat,
    _y: GLfloat,
    _z: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixLoadTransposefEXT =
    unsafe extern "system" fn(_mode: MatrixMode, _m: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glFogCoordfv = unsafe extern "system" fn(_coord: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glNormalStream3bATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _nx: GLbyte, _ny: GLbyte, _nz: GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glResetMemoryObjectParameterNV =
    unsafe extern "system" fn(_memory: GLuint, _pname: GLenum);
#[allow(non_camel_case_types)]
pub type PFN_glSamplerParameterIuivOES =
    unsafe extern "system" fn(_sampler: GLuint, _pname: SamplerParameterI, _param: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glColorSubTableEXT = unsafe extern "system" fn(
    _target: ColorTableTarget,
    _start: GLsizei,
    _count: GLsizei,
    _format: PixelFormat,
    _type: PixelType,
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetNamedStringARB = unsafe extern "system" fn(
    _namelen: GLint,
    _name: *const GLchar,
    _bufSize: GLsizei,
    _stringlen: *mut GLint,
    _string: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord2xvOES = unsafe extern "system" fn(_coords: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord4f =
    unsafe extern "system" fn(_s: GLfloat, _t: GLfloat, _r: GLfloat, _q: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glUniform3ui =
    unsafe extern "system" fn(_location: GLint, _v0: GLuint, _v1: GLuint, _v2: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glUniformMatrix4x3fv = unsafe extern "system" fn(
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib2dvNV = unsafe extern "system" fn(_index: GLuint, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4Nusv = unsafe extern "system" fn(_index: GLuint, _v: *const GLushort);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteTransformFeedbacksNV =
    unsafe extern "system" fn(_n: GLsizei, _ids: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetFragDataIndexEXT =
    unsafe extern "system" fn(_program: GLuint, _name: *const GLchar) -> GLint;
#[allow(non_camel_case_types)]
pub type PFN_glGetMultiTexGenfvEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _coord: TextureCoordName,
    _pname: TextureGenParameter,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetnPixelMapuiv =
    unsafe extern "system" fn(_map: PixelMap, _bufSize: GLsizei, _values: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glStencilClearTagEXT =
    unsafe extern "system" fn(_stencilTagBits: GLsizei, _stencilClearTag: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetDebugMessageLogKHR = unsafe extern "system" fn(
    _count: GLuint,
    _bufSize: GLsizei,
    _sources: *mut DebugSource,
    _types: *mut DebugType,
    _ids: *mut GLuint,
    _severities: *mut DebugSeverity,
    _lengths: *mut GLsizei,
    _messageLog: *mut GLchar,
) -> GLuint;
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI4sv = unsafe extern "system" fn(_index: GLuint, _v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glGetMultisamplefvNV =
    unsafe extern "system" fn(_pname: GetMultisamplePNameNV, _index: GLuint, _val: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetGraphicsResetStatusKHR = unsafe extern "system" fn() -> GLenum;
#[allow(non_camel_case_types)]
pub type PFN_glListDrawCommandsStatesClientNV = unsafe extern "system" fn(
    _list: GLuint,
    _segment: GLuint,
    _indirects: *mut *const std::os::raw::c_void,
    _sizes: *const GLsizei,
    _states: *const GLuint,
    _fbos: *const GLuint,
    _count: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glSampleMaskIndexedNV = unsafe extern "system" fn(_index: GLuint, _mask: GLbitfield);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord4dv = unsafe extern "system" fn(_v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribs3svNV =
    unsafe extern "system" fn(_index: GLuint, _count: GLsizei, _v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glMultiDrawElementsIndirectCount = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _type: DrawElementsType,
    _indirect: *const std::os::raw::c_void,
    _drawcount: GLintptr,
    _maxdrawcount: GLsizei,
    _stride: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexArrayBindingDivisor =
    unsafe extern "system" fn(_vaobj: GLuint, _bindingindex: GLuint, _divisor: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glUniform2ivARB =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glMulticastBufferSubDataNV = unsafe extern "system" fn(
    _gpuMask: GLbitfield,
    _buffer: GLuint,
    _offset: GLintptr,
    _size: GLsizeiptr,
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord2fNormal3fVertex3fSUN = unsafe extern "system" fn(
    _s: GLfloat,
    _t: GLfloat,
    _nx: GLfloat,
    _ny: GLfloat,
    _nz: GLfloat,
    _x: GLfloat,
    _y: GLfloat,
    _z: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glColor4d =
    unsafe extern "system" fn(_red: GLdouble, _green: GLdouble, _blue: GLdouble, _alpha: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexArrayFogCoordOffsetEXT = unsafe extern "system" fn(
    _vaobj: GLuint,
    _buffer: GLuint,
    _type: FogCoordinatePointerType,
    _stride: GLsizei,
    _offset: GLintptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glDrawPixels = unsafe extern "system" fn(
    _width: GLsizei,
    _height: GLsizei,
    _format: PixelFormat,
    _type: PixelType,
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glSharpenTexFuncSGIS =
    unsafe extern "system" fn(_target: TextureTarget, _n: GLsizei, _points: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glColor4ubVertex3fSUN = unsafe extern "system" fn(
    _r: GLubyte,
    _g: GLubyte,
    _b: GLubyte,
    _a: GLubyte,
    _x: GLfloat,
    _y: GLfloat,
    _z: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glBindMultiTextureEXT =
    unsafe extern "system" fn(_texunit: TextureUnit, _target: TextureTarget, _texture: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glNormalStream3ivATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _coords: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glTexParameterf = unsafe extern "system" fn(
    _target: TextureTarget,
    _pname: TextureParameterName,
    _param: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiDrawArraysEXT = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _first: *const GLint,
    _count: *const GLsizei,
    _primcount: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI2iv = unsafe extern "system" fn(_index: GLuint, _v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord4bvOES = unsafe extern "system" fn(_coords: *const GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glGetActiveSubroutineUniformName = unsafe extern "system" fn(
    _program: GLuint,
    _shadertype: ShaderType,
    _index: GLuint,
    _bufSize: GLsizei,
    _length: *mut GLsizei,
    _name: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexGenivOES = unsafe extern "system" fn(
    _coord: TextureCoordName,
    _pname: TextureGenParameter,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord4fVertex4fSUN = unsafe extern "system" fn(
    _s: GLfloat,
    _t: GLfloat,
    _p: GLfloat,
    _q: GLfloat,
    _x: GLfloat,
    _y: GLfloat,
    _z: GLfloat,
    _w: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glCombinerOutputNV = unsafe extern "system" fn(
    _stage: CombinerStageNV,
    _portion: CombinerPortionNV,
    _abOutput: CombinerRegisterNV,
    _cdOutput: CombinerRegisterNV,
    _sumOutput: CombinerRegisterNV,
    _scale: CombinerScaleNV,
    _bias: CombinerBiasNV,
    _abDotProduct: GLboolean,
    _cdDotProduct: GLboolean,
    _muxSum: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord4f = unsafe extern "system" fn(
    _target: TextureUnit,
    _s: GLfloat,
    _t: GLfloat,
    _r: GLfloat,
    _q: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteTextures = unsafe extern "system" fn(_n: GLsizei, _textures: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMemoryBarrierByRegion = unsafe extern "system" fn(_barriers: MemoryBarrierMask);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib1fARB = unsafe extern "system" fn(_index: GLuint, _x: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix2x3dv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramEnvParameterfvARB =
    unsafe extern "system" fn(_target: ProgramTarget, _index: GLuint, _params: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI4svEXT = unsafe extern "system" fn(_index: GLuint, _v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glGetnPolygonStippleARB =
    unsafe extern "system" fn(_bufSize: GLsizei, _pattern: *mut GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glGetFixedvOES = unsafe extern "system" fn(_pname: GetPName, _params: *mut GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glExtGetBufferPointervQCOM =
    unsafe extern "system" fn(_target: GLenum, _params: *mut *mut std::os::raw::c_void);
#[allow(non_camel_case_types)]
pub type PFN_glTextureImage2DEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _level: GLint,
    _internalformat: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _border: GLint,
    _format: PixelFormat,
    _type: PixelType,
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glLoadMatrixf = unsafe extern "system" fn(_m: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetLightfv =
    unsafe extern "system" fn(_light: LightName, _pname: LightParameter, _params: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glNamedBufferSubDataEXT = unsafe extern "system" fn(
    _buffer: GLuint,
    _offset: GLintptr,
    _size: GLsizeiptr,
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform4i = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _v0: GLint,
    _v1: GLint,
    _v2: GLint,
    _v3: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glRenderbufferStorageEXT = unsafe extern "system" fn(
    _target: RenderbufferTarget,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetUniformi64vNV =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _params: *mut GLint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glUnlockArraysEXT = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI4bvEXT = unsafe extern "system" fn(_index: GLuint, _v: *const GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glGetTexEnvxvOES = unsafe extern "system" fn(
    _target: TextureEnvTarget,
    _pname: TextureEnvParameter,
    _params: *mut GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glGenNamesAMD =
    unsafe extern "system" fn(_identifier: GLenum, _num: GLuint, _names: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord2hvNV =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glLightModelfv =
    unsafe extern "system" fn(_pname: LightModelParameter, _params: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetImageHandleARB = unsafe extern "system" fn(
    _texture: GLuint,
    _level: GLint,
    _layered: GLboolean,
    _layer: GLint,
    _format: PixelFormat,
) -> GLuint64;
#[allow(non_camel_case_types)]
pub type PFN_glMulticastViewportArrayvNVX =
    unsafe extern "system" fn(_gpu: GLuint, _first: GLuint, _count: GLsizei, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glFrameZoomSGIX = unsafe extern "system" fn(_factor: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glTranslatef = unsafe extern "system" fn(_x: GLfloat, _y: GLfloat, _z: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glDrawBuffer = unsafe extern "system" fn(_buf: DrawBufferMode);
#[allow(non_camel_case_types)]
pub type PFN_glSamplerParameterIivEXT =
    unsafe extern "system" fn(_sampler: GLuint, _pname: SamplerParameterI, _param: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glUniform2iARB = unsafe extern "system" fn(_location: GLint, _v0: GLint, _v1: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexArrayAttribFormat = unsafe extern "system" fn(
    _vaobj: GLuint,
    _attribindex: GLuint,
    _size: GLint,
    _type: VertexAttribType,
    _normalized: GLboolean,
    _relativeoffset: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glEnableClientStateIndexedEXT =
    unsafe extern "system" fn(_array: EnableCap, _index: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetQueryObjectuiv =
    unsafe extern "system" fn(_id: GLuint, _pname: QueryObjectParameterName, _params: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetUnsignedBytei_vEXT =
    unsafe extern "system" fn(_target: GLenum, _index: GLuint, _data: *mut GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexAttribPointerv = unsafe extern "system" fn(
    _index: GLuint,
    _pname: VertexAttribPointerPropertyARB,
    _pointer: *mut *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glPolygonMode = unsafe extern "system" fn(_face: MaterialFace, _mode: PolygonMode);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixIndexPointerARB = unsafe extern "system" fn(
    _size: GLint,
    _type: MatrixIndexPointerTypeARB,
    _stride: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform1i = unsafe extern "system" fn(_location: GLint, _v0: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glBlendBarrierNV = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glCopyTexImage2DEXT = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _internalformat: InternalFormat,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _border: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetnMapivARB = unsafe extern "system" fn(
    _target: MapTarget,
    _query: MapQuery,
    _bufSize: GLsizei,
    _v: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetQueryObjectivARB =
    unsafe extern "system" fn(_id: GLuint, _pname: QueryObjectParameterName, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteQueryResourceTagNV =
    unsafe extern "system" fn(_n: GLsizei, _tagIds: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glNormalStream3dATI = unsafe extern "system" fn(
    _stream: VertexStreamATI,
    _nx: GLdouble,
    _ny: GLdouble,
    _nz: GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform2fv =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord2s = unsafe extern "system" fn(_s: GLshort, _t: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glUniformMatrix3fv = unsafe extern "system" fn(
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glBindFragmentShaderATI = unsafe extern "system" fn(_id: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform4ui = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _v0: GLuint,
    _v1: GLuint,
    _v2: GLuint,
    _v3: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glPointParameterivNV =
    unsafe extern "system" fn(_pname: PointParameterNameARB, _params: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glCoverStrokePathInstancedNV = unsafe extern "system" fn(
    _numPaths: GLsizei,
    _pathNameType: PathElementType,
    _paths: *const std::os::raw::c_void,
    _pathBase: GLuint,
    _coverMode: PathCoverMode,
    _transformType: PathTransformType,
    _transformValues: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetnUniformfvKHR = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _bufSize: GLsizei,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexImage2DMultisample = unsafe extern "system" fn(
    _target: TextureTarget,
    _samples: GLsizei,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _fixedsamplelocations: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetSubroutineUniformLocation = unsafe extern "system" fn(
    _program: GLuint,
    _shadertype: ShaderType,
    _name: *const GLchar,
) -> GLint;
#[allow(non_camel_case_types)]
pub type PFN_glGetFragmentMaterialfvSGIX = unsafe extern "system" fn(
    _face: MaterialFace,
    _pname: MaterialParameter,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glIsVariantEnabledEXT =
    unsafe extern "system" fn(_id: GLuint, _cap: VariantCapEXT) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform3i64vNV = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLint64EXT,
);
#[allow(non_camel_case_types)]
pub type PFN_glWeightsvARB = unsafe extern "system" fn(_size: GLint, _weights: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glGetSamplerParameterIuiv =
    unsafe extern "system" fn(_sampler: GLuint, _pname: SamplerParameterI, _params: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetnUniformfvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _bufSize: GLsizei,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetnUniformi64vARB = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _bufSize: GLsizei,
    _params: *mut GLint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetTexEnviv = unsafe extern "system" fn(
    _target: TextureEnvTarget,
    _pname: TextureEnvParameter,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glIsBufferARB = unsafe extern "system" fn(_buffer: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glMinSampleShadingOES = unsafe extern "system" fn(_value: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glRenderbufferStorage = unsafe extern "system" fn(
    _target: RenderbufferTarget,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glShadingRateSampleOrderCustomNV =
    unsafe extern "system" fn(_rate: GLenum, _samples: GLuint, _locations: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramPipelineivEXT = unsafe extern "system" fn(
    _pipeline: GLuint,
    _pname: PipelineParameterName,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glClientActiveTexture = unsafe extern "system" fn(_texture: TextureUnit);
#[allow(non_camel_case_types)]
pub type PFN_glMultiDrawElementArrayAPPLE = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _first: *const GLint,
    _count: *const GLsizei,
    _primcount: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramEnvParametersI4ivNV = unsafe extern "system" fn(
    _target: ProgramTarget,
    _index: GLuint,
    _count: GLsizei,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoordPointervINTEL = unsafe extern "system" fn(
    _size: GLint,
    _type: VertexPointerType,
    _pointer: *mut *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glBlendEquationEXT = unsafe extern "system" fn(_mode: BlendEquationModeEXT);
#[allow(non_camel_case_types)]
pub type PFN_glUnmapObjectBufferATI = unsafe extern "system" fn(_buffer: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glCopyTextureImage2DEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _level: GLint,
    _internalformat: InternalFormat,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _border: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glAlphaFunc = unsafe extern "system" fn(_func: AlphaFunction, _ref: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glTexStorage3DEXT = unsafe extern "system" fn(
    _target: TextureTarget,
    _levels: GLsizei,
    _internalformat: SizedInternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glBufferStorageMemEXT = unsafe extern "system" fn(
    _target: BufferTargetARB,
    _size: GLsizeiptr,
    _memory: GLuint,
    _offset: GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteProgramsARB = unsafe extern "system" fn(_n: GLsizei, _programs: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramEnvParameterIuivNV =
    unsafe extern "system" fn(_target: ProgramTarget, _index: GLuint, _params: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glExtGetProgramsQCOM = unsafe extern "system" fn(
    _programs: *mut GLuint,
    _maxPrograms: GLint,
    _numPrograms: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetnConvolutionFilterARB = unsafe extern "system" fn(
    _target: ConvolutionTarget,
    _format: PixelFormat,
    _type: PixelType,
    _bufSize: GLsizei,
    _image: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexGenivEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _coord: TextureCoordName,
    _pname: TextureGenParameter,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVDPAURegisterOutputSurfaceNV = unsafe extern "system" fn(
    _vdpSurface: *const std::os::raw::c_void,
    _target: GLenum,
    _numTextureNames: GLsizei,
    _textureNames: *const GLuint,
) -> GLvdpauSurfaceNV;
#[allow(non_camel_case_types)]
pub type PFN_glUniformMatrix4fv = unsafe extern "system" fn(
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glPollAsyncSGIX = unsafe extern "system" fn(_markerp: *mut GLuint) -> GLint;
#[allow(non_camel_case_types)]
pub type PFN_glGetUniformSubroutineuiv =
    unsafe extern "system" fn(_shadertype: ShaderType, _location: GLint, _params: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib3s =
    unsafe extern "system" fn(_index: GLuint, _x: GLshort, _y: GLshort, _z: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream3dvATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _coords: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteNamedStringARB =
    unsafe extern "system" fn(_namelen: GLint, _name: *const GLchar);
#[allow(non_camel_case_types)]
pub type PFN_glGetTexLevelParameterxvOES = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _pname: GetTextureParameter,
    _params: *mut GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureStorageMem2DMultisampleEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _samples: GLsizei,
    _internalFormat: SizedInternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _fixedSampleLocations: GLboolean,
    _memory: GLuint,
    _offset: GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream4dATI = unsafe extern "system" fn(
    _stream: VertexStreamATI,
    _x: GLdouble,
    _y: GLdouble,
    _z: GLdouble,
    _w: GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos2d = unsafe extern "system" fn(_x: GLdouble, _y: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glBindVertexBuffers = unsafe extern "system" fn(
    _first: GLuint,
    _count: GLsizei,
    _buffers: *const GLuint,
    _offsets: *const GLintptr,
    _strides: *const GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetSyncivAPPLE = unsafe extern "system" fn(
    _sync: GLsync,
    _pname: SyncParameterName,
    _count: GLsizei,
    _length: *mut GLsizei,
    _values: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glColorPointerEXT = unsafe extern "system" fn(
    _size: GLint,
    _type: ColorPointerType,
    _stride: GLsizei,
    _count: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glDrawBuffersNV = unsafe extern "system" fn(_n: GLsizei, _bufs: *const GLenum);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI2i = unsafe extern "system" fn(_index: GLuint, _x: GLint, _y: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetLightiv =
    unsafe extern "system" fn(_light: LightName, _pname: LightParameter, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glWeightubvARB = unsafe extern "system" fn(_size: GLint, _weights: *const GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glFenceSync =
    unsafe extern "system" fn(_condition: SyncCondition, _flags: SyncBehaviorFlags) -> GLsync;
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform1uiv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniformMatrix2x3fvNV = unsafe extern "system" fn(
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexParameterxvOES = unsafe extern "system" fn(
    _target: TextureTarget,
    _pname: GetTextureParameter,
    _params: *const GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glCompileShaderIncludeARB = unsafe extern "system" fn(
    _shader: GLuint,
    _count: GLsizei,
    _path: *const *const GLchar,
    _length: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glPushMatrix = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glDepthRangex = unsafe extern "system" fn(_n: GLfixed, _f: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glEdgeFlag = unsafe extern "system" fn(_flag: GLboolean);
#[allow(non_camel_case_types)]
pub type PFN_glGetVideoi64vNV =
    unsafe extern "system" fn(_video_slot: GLuint, _pname: GLenum, _params: *mut GLint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glNamedBufferData = unsafe extern "system" fn(
    _buffer: GLuint,
    _size: GLsizeiptr,
    _data: *const std::os::raw::c_void,
    _usage: VertexBufferObjectUsage,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramVertexLimitNV =
    unsafe extern "system" fn(_target: ProgramTarget, _limit: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetVariantPointervEXT = unsafe extern "system" fn(
    _id: GLuint,
    _value: GetVariantValueEXT,
    _data: *mut *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexEnviv = unsafe extern "system" fn(
    _target: TextureEnvTarget,
    _pname: TextureEnvParameter,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI3uiEXT =
    unsafe extern "system" fn(_index: GLuint, _x: GLuint, _y: GLuint, _z: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribPointerARB = unsafe extern "system" fn(
    _index: GLuint,
    _size: GLint,
    _type: VertexAttribPointerType,
    _normalized: GLboolean,
    _stride: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform1iEXT =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _v0: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexP3ui = unsafe extern "system" fn(_type: VertexPointerType, _value: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetQueryObjectivEXT =
    unsafe extern "system" fn(_id: GLuint, _pname: QueryObjectParameterName, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glUniformMatrix2x3fv = unsafe extern "system" fn(
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glMaterialx =
    unsafe extern "system" fn(_face: MaterialFace, _pname: MaterialParameter, _param: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord3dvARB =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glAreTexturesResidentEXT = unsafe extern "system" fn(
    _n: GLsizei,
    _textures: *const GLuint,
    _residences: *mut GLboolean,
) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glDeleteVertexShaderEXT = unsafe extern "system" fn(_id: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glBindVertexShaderEXT = unsafe extern "system" fn(_id: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glColorTableEXT = unsafe extern "system" fn(
    _target: ColorTableTarget,
    _internalFormat: InternalFormat,
    _width: GLsizei,
    _format: PixelFormat,
    _type: PixelType,
    _table: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetMinmaxEXT = unsafe extern "system" fn(
    _target: MinmaxTargetEXT,
    _reset: GLboolean,
    _format: PixelFormat,
    _type: PixelType,
    _values: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetQueryObjectui64vEXT = unsafe extern "system" fn(
    _id: GLuint,
    _pname: QueryObjectParameterName,
    _params: *mut GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetnUniformuivARB = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _bufSize: GLsizei,
    _params: *mut GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI1uivEXT = unsafe extern "system" fn(_index: GLuint, _v: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glIsCommandListNV = unsafe extern "system" fn(_list: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glReplacementCodeubSUN = unsafe extern "system" fn(_code: GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL2ui64NV =
    unsafe extern "system" fn(_index: GLuint, _x: GLuint64EXT, _y: GLuint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glGetVideouivNV =
    unsafe extern "system" fn(_video_slot: GLuint, _pname: GLenum, _params: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetNamedBufferParameteri64v =
    unsafe extern "system" fn(_buffer: GLuint, _pname: BufferPNameARB, _params: *mut GLint64);
#[allow(non_camel_case_types)]
pub type PFN_glDisable = unsafe extern "system" fn(_cap: EnableCap);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord3hNV =
    unsafe extern "system" fn(_target: TextureUnit, _s: GLhalfNV, _t: GLhalfNV, _r: GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4ubNV =
    unsafe extern "system" fn(_index: GLuint, _x: GLubyte, _y: GLubyte, _z: GLubyte, _w: GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glColorTableParameterivSGI = unsafe extern "system" fn(
    _target: ColorTableTargetSGI,
    _pname: ColorTableParameterPNameSGI,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glOrthofOES = unsafe extern "system" fn(
    _l: GLfloat,
    _r: GLfloat,
    _b: GLfloat,
    _t: GLfloat,
    _n: GLfloat,
    _f: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetFirstPerfQueryIdINTEL = unsafe extern "system" fn(_queryId: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glUniformMatrix2x4fvNV = unsafe extern "system" fn(
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureParameterivEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _pname: TextureParameterName,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramResourceLocationIndexEXT = unsafe extern "system" fn(
    _program: GLuint,
    _programInterface: ProgramInterface,
    _name: *const GLchar,
) -> GLint;
#[allow(non_camel_case_types)]
pub type PFN_glVertexBlendEnviATI =
    unsafe extern "system" fn(_pname: VertexStreamATI, _param: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glBindFragDataLocationIndexed = unsafe extern "system" fn(
    _program: GLuint,
    _colorNumber: GLuint,
    _index: GLuint,
    _name: *const GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glBeginConditionalRenderNV =
    unsafe extern "system" fn(_id: GLuint, _mode: ConditionalRenderMode);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferFoveationConfigQCOM = unsafe extern "system" fn(
    _framebuffer: GLuint,
    _numLayers: GLuint,
    _focalPointsPerLayer: GLuint,
    _requestedFeatures: GLuint,
    _providedFeatures: *mut GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord1bOES = unsafe extern "system" fn(_s: GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glGetRenderbufferParameteriv = unsafe extern "system" fn(
    _target: RenderbufferTarget,
    _pname: RenderbufferParameterName,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4Nub =
    unsafe extern "system" fn(_index: GLuint, _x: GLubyte, _y: GLubyte, _z: GLubyte, _w: GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glNamedBufferStorageExternalEXT = unsafe extern "system" fn(
    _buffer: GLuint,
    _offset: GLintptr,
    _size: GLsizeiptr,
    _clientBuffer: GLeglClientBufferEXT,
    _flags: BufferStorageMask,
);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos3svMESA = unsafe extern "system" fn(_v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glNamedProgramLocalParametersI4uivEXT = unsafe extern "system" fn(
    _program: GLuint,
    _target: ProgramTarget,
    _index: GLuint,
    _count: GLsizei,
    _params: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramBinary = unsafe extern "system" fn(
    _program: GLuint,
    _bufSize: GLsizei,
    _length: *mut GLsizei,
    _binaryFormat: *mut GLenum,
    _binary: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glColor4ubVertex2fvSUN =
    unsafe extern "system" fn(_c: *const GLubyte, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glActiveShaderProgramEXT =
    unsafe extern "system" fn(_pipeline: GLuint, _program: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glPathGlyphIndexRangeNV = unsafe extern "system" fn(
    _fontTarget: GLenum,
    _fontName: *const std::os::raw::c_void,
    _fontStyle: PathFontStyle,
    _pathParameterTemplate: GLuint,
    _emScale: GLfloat,
    _baseAndCount: *mut GLuint,
) -> GLenum;
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramResourcefvNV = unsafe extern "system" fn(
    _program: GLuint,
    _programInterface: ProgramInterface,
    _index: GLuint,
    _propCount: GLsizei,
    _props: *const GLenum,
    _count: GLsizei,
    _length: *mut GLsizei,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3hvNV = unsafe extern "system" fn(_v: *const GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3dvEXT = unsafe extern "system" fn(_v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4Nsv = unsafe extern "system" fn(_index: GLuint, _v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4sNV =
    unsafe extern "system" fn(_index: GLuint, _x: GLshort, _y: GLshort, _z: GLshort, _w: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glNamedBufferPageCommitmentMemNV = unsafe extern "system" fn(
    _buffer: GLuint,
    _offset: GLintptr,
    _size: GLsizeiptr,
    _memory: GLuint,
    _memOffset: GLuint64,
    _commit: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glColor3ubv = unsafe extern "system" fn(_v: *const GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteTexturesEXT = unsafe extern "system" fn(_n: GLsizei, _textures: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glDepthRangedNV = unsafe extern "system" fn(_zNear: GLdouble, _zFar: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glNamedProgramLocalParameters4fvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _target: ProgramTarget,
    _index: GLuint,
    _count: GLsizei,
    _params: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glReplacementCodeuiTexCoord2fNormal3fVertex3fSUN = unsafe extern "system" fn(
    _rc: GLuint,
    _s: GLfloat,
    _t: GLfloat,
    _nx: GLfloat,
    _ny: GLfloat,
    _nz: GLfloat,
    _x: GLfloat,
    _y: GLfloat,
    _z: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glDisableIndexedEXT = unsafe extern "system" fn(_target: EnableCap, _index: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glFrustumfOES = unsafe extern "system" fn(
    _l: GLfloat,
    _r: GLfloat,
    _b: GLfloat,
    _t: GLfloat,
    _n: GLfloat,
    _f: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glResolveDepthValuesNV = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glUniform1uiv =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribIPointer = unsafe extern "system" fn(
    _index: GLuint,
    _size: GLint,
    _type: VertexAttribIType,
    _stride: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetnCompressedTexImageARB = unsafe extern "system" fn(
    _target: TextureTarget,
    _lod: GLint,
    _bufSize: GLsizei,
    _img: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glClearColorIuiEXT =
    unsafe extern "system" fn(_red: GLuint, _green: GLuint, _blue: GLuint, _alpha: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glBlendEquationiEXT =
    unsafe extern "system" fn(_buf: GLuint, _mode: BlendEquationModeEXT);
#[allow(non_camel_case_types)]
pub type PFN_glIsSync = unsafe extern "system" fn(_sync: GLsync) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos4fMESA =
    unsafe extern "system" fn(_x: GLfloat, _y: GLfloat, _z: GLfloat, _w: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord2sv =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glNamedProgramLocalParameter4dEXT = unsafe extern "system" fn(
    _program: GLuint,
    _target: ProgramTarget,
    _index: GLuint,
    _x: GLdouble,
    _y: GLdouble,
    _z: GLdouble,
    _w: GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexAttribArrayObjectfvATI =
    unsafe extern "system" fn(_index: GLuint, _pname: ArrayObjectPNameATI, _params: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glMulticastScissorArrayvNVX =
    unsafe extern "system" fn(_gpu: GLuint, _first: GLuint, _count: GLsizei, _v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glColorMaski = unsafe extern "system" fn(
    _index: GLuint,
    _r: GLboolean,
    _g: GLboolean,
    _b: GLboolean,
    _a: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glIndexfv = unsafe extern "system" fn(_c: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glBindVertexBuffer = unsafe extern "system" fn(
    _bindingindex: GLuint,
    _buffer: GLuint,
    _offset: GLintptr,
    _stride: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glDisableVertexArrayAttribEXT =
    unsafe extern "system" fn(_vaobj: GLuint, _index: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix4x3fvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glNormal3d = unsafe extern "system" fn(_nx: GLdouble, _ny: GLdouble, _nz: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glTexStorage3DMultisampleOES = unsafe extern "system" fn(
    _target: TextureTarget,
    _samples: GLsizei,
    _internalformat: SizedInternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _fixedsamplelocations: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertex3dv = unsafe extern "system" fn(_v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib1dNV = unsafe extern "system" fn(_index: GLuint, _x: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream4fvATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _coords: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glNormal3xvOES = unsafe extern "system" fn(_coords: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glUniform3ui64vARB =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glCompressedTexSubImage1D = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _xoffset: GLint,
    _width: GLsizei,
    _format: InternalFormat,
    _imageSize: GLsizei,
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform4fvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform3fv =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexSubImage1DEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _level: GLint,
    _xoffset: GLint,
    _width: GLsizei,
    _format: PixelFormat,
    _type: PixelType,
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexParameteriEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _pname: TextureParameterName,
    _param: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glScissorIndexedvOES = unsafe extern "system" fn(_index: GLuint, _v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glFrustumx = unsafe extern "system" fn(
    _l: GLfixed,
    _r: GLfixed,
    _b: GLfixed,
    _t: GLfixed,
    _n: GLfixed,
    _f: GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glIsQueryEXT = unsafe extern "system" fn(_id: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord2fv = unsafe extern "system" fn(_v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI4uiv = unsafe extern "system" fn(_index: GLuint, _v: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetMultiTexParameterivEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _pname: GetTextureParameter,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glColor3ub = unsafe extern "system" fn(_red: GLubyte, _green: GLubyte, _blue: GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glBindProgramNV = unsafe extern "system" fn(_target: VertexAttribEnumNV, _id: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord3bvOES =
    unsafe extern "system" fn(_texture: TextureUnit, _coords: *const GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glColor3fVertex3fSUN = unsafe extern "system" fn(
    _r: GLfloat,
    _g: GLfloat,
    _b: GLfloat,
    _x: GLfloat,
    _y: GLfloat,
    _z: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetConvolutionParameterfv = unsafe extern "system" fn(
    _target: ConvolutionTarget,
    _pname: ConvolutionParameterEXT,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramStringARB = unsafe extern "system" fn(
    _target: ProgramTarget,
    _pname: ProgramStringProperty,
    _string: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramiv =
    unsafe extern "system" fn(_program: GLuint, _pname: ProgramPropertyARB, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform3f = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _v0: GLfloat,
    _v1: GLfloat,
    _v2: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glBufferSubData = unsafe extern "system" fn(
    _target: BufferTargetARB,
    _offset: GLintptr,
    _size: GLsizeiptr,
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glPathStringNV = unsafe extern "system" fn(
    _path: GLuint,
    _format: PathStringFormat,
    _length: GLsizei,
    _pathString: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glAttachObjectARB =
    unsafe extern "system" fn(_containerObj: GLhandleARB, _obj: GLhandleARB);
#[allow(non_camel_case_types)]
pub type PFN_glDrawRangeElementArrayAPPLE = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _start: GLuint,
    _end: GLuint,
    _first: GLint,
    _count: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glDrawTransformFeedbackNV =
    unsafe extern "system" fn(_mode: PrimitiveType, _id: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glUniform4ui64vNV =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLuint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glDrawArraysInstancedBaseInstance = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _first: GLint,
    _count: GLsizei,
    _instancecount: GLsizei,
    _baseinstance: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertex4dv = unsafe extern "system" fn(_v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glEnableVertexArrayAttribEXT =
    unsafe extern "system" fn(_vaobj: GLuint, _index: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord4bOES =
    unsafe extern "system" fn(_s: GLbyte, _t: GLbyte, _r: GLbyte, _q: GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glNamedCopyBufferSubDataEXT = unsafe extern "system" fn(
    _readBuffer: GLuint,
    _writeBuffer: GLuint,
    _readOffset: GLintptr,
    _writeOffset: GLintptr,
    _size: GLsizeiptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glDepthMask = unsafe extern "system" fn(_flag: GLboolean);
#[allow(non_camel_case_types)]
pub type PFN_glBlendFuncSeparateiEXT = unsafe extern "system" fn(
    _buf: GLuint,
    _srcRGB: BlendingFactor,
    _dstRGB: BlendingFactor,
    _srcAlpha: BlendingFactor,
    _dstAlpha: BlendingFactor,
);
#[allow(non_camel_case_types)]
pub type PFN_glClearNamedFramebufferuiv = unsafe extern "system" fn(
    _framebuffer: GLuint,
    _buffer: Buffer,
    _drawbuffer: GLint,
    _value: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glSignalSemaphoreui64NVX = unsafe extern "system" fn(
    _signalGpu: GLuint,
    _fenceObjectCount: GLsizei,
    _semaphoreArray: *const GLuint,
    _fenceValueArray: *const GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glCopyConvolutionFilter1D = unsafe extern "system" fn(
    _target: ConvolutionTarget,
    _internalformat: InternalFormat,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glEndQueryARB = unsafe extern "system" fn(_target: QueryTarget);
#[allow(non_camel_case_types)]
pub type PFN_glPassTexCoordATI =
    unsafe extern "system" fn(_dst: GLuint, _coord: GLuint, _swizzle: SwizzleOpATI);
#[allow(non_camel_case_types)]
pub type PFN_glReplacementCodeuiTexCoord2fColor4fNormal3fVertex3fSUN =
    unsafe extern "system" fn(
        _rc: GLuint,
        _s: GLfloat,
        _t: GLfloat,
        _r: GLfloat,
        _g: GLfloat,
        _b: GLfloat,
        _a: GLfloat,
        _nx: GLfloat,
        _ny: GLfloat,
        _nz: GLfloat,
        _x: GLfloat,
        _y: GLfloat,
        _z: GLfloat,
    );
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4dARB = unsafe extern "system" fn(
    _index: GLuint,
    _x: GLdouble,
    _y: GLdouble,
    _z: GLdouble,
    _w: GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glBindLightParameterEXT =
    unsafe extern "system" fn(_light: LightName, _value: LightParameter) -> GLuint;
#[allow(non_camel_case_types)]
pub type PFN_glNormalStream3fATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _nx: GLfloat, _ny: GLfloat, _nz: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix3x4dvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glBufferDataARB = unsafe extern "system" fn(
    _target: BufferTargetARB,
    _size: GLsizeiptrARB,
    _data: *const std::os::raw::c_void,
    _usage: BufferUsageARB,
);
#[allow(non_camel_case_types)]
pub type PFN_glColorMaterial =
    unsafe extern "system" fn(_face: MaterialFace, _mode: ColorMaterialParameter);
#[allow(non_camel_case_types)]
pub type PFN_glDrawElementsInstancedBaseInstanceEXT = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _count: GLsizei,
    _type: PrimitiveType,
    _indices: *const std::os::raw::c_void,
    _instancecount: GLsizei,
    _baseinstance: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glDebugMessageCallbackKHR =
    unsafe extern "system" fn(_callback: GLDEBUGPROCKHR, _userParam: *const std::os::raw::c_void);
#[allow(non_camel_case_types)]
pub type PFN_glCreateTransformFeedbacks = unsafe extern "system" fn(_n: GLsizei, _ids: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glEnableIndexedEXT = unsafe extern "system" fn(_target: EnableCap, _index: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glCreateSemaphoresNV =
    unsafe extern "system" fn(_n: GLsizei, _semaphores: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetMaterialxv = unsafe extern "system" fn(
    _face: MaterialFace,
    _pname: MaterialParameter,
    _params: *mut GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord1bvOES =
    unsafe extern "system" fn(_texture: TextureUnit, _coords: *const GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix2x4dv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI4usvEXT =
    unsafe extern "system" fn(_index: GLuint, _v: *const GLushort);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord2iv = unsafe extern "system" fn(_target: TextureUnit, _v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glSamplerParameterIiv =
    unsafe extern "system" fn(_sampler: GLuint, _pname: SamplerParameterI, _param: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glColorMaskIndexedEXT = unsafe extern "system" fn(
    _index: GLuint,
    _r: GLboolean,
    _g: GLboolean,
    _b: GLboolean,
    _a: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramNamedParameter4dvNV = unsafe extern "system" fn(
    _id: GLuint,
    _len: GLsizei,
    _name: *const GLubyte,
    _v: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glViewportArrayv =
    unsafe extern "system" fn(_first: GLuint, _count: GLsizei, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib2hvNV = unsafe extern "system" fn(_index: GLuint, _v: *const GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glColorFragmentOp2ATI = unsafe extern "system" fn(
    _op: FragmentOpATI,
    _dst: GLuint,
    _dstMask: GLuint,
    _dstMod: GLuint,
    _arg1: GLuint,
    _arg1Rep: GLuint,
    _arg1Mod: GLuint,
    _arg2: GLuint,
    _arg2Rep: GLuint,
    _arg2Mod: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glEndFragmentShaderATI = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glGetConvolutionFilterEXT = unsafe extern "system" fn(
    _target: ConvolutionTargetEXT,
    _format: PixelFormat,
    _type: PixelType,
    _image: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glFogCoordhvNV = unsafe extern "system" fn(_fog: *const GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform3i64NV = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _x: GLint64EXT,
    _y: GLint64EXT,
    _z: GLint64EXT,
);
#[allow(non_camel_case_types)]
pub type PFN_glBindVertexArrayOES = unsafe extern "system" fn(_array: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetTexGenxvOES = unsafe extern "system" fn(
    _coord: TextureCoordName,
    _pname: TextureGenParameter,
    _params: *mut GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glMinSampleShadingARB = unsafe extern "system" fn(_value: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform1ui64NV =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _x: GLuint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glFrameTerminatorGREMEDY = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glReplacementCodeuivSUN = unsafe extern "system" fn(_code: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glTangent3svEXT = unsafe extern "system" fn(_v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glUniform3fvARB =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glUniformMatrix3dv = unsafe extern "system" fn(
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord3d = unsafe extern "system" fn(_s: GLdouble, _t: GLdouble, _r: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glTexBufferOES = unsafe extern "system" fn(
    _target: TextureTarget,
    _internalformat: SizedInternalFormat,
    _buffer: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetnUniformdv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _bufSize: GLsizei,
    _params: *mut GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glHistogramEXT = unsafe extern "system" fn(
    _target: HistogramTargetEXT,
    _width: GLsizei,
    _internalformat: InternalFormat,
    _sink: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL3i64NV =
    unsafe extern "system" fn(_index: GLuint, _x: GLint64EXT, _y: GLint64EXT, _z: GLint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream2dATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _x: GLdouble, _y: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glTexParameterx =
    unsafe extern "system" fn(_target: TextureTarget, _pname: GetTextureParameter, _param: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glCreateVertexArrays = unsafe extern "system" fn(_n: GLsizei, _arrays: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glColorMaskiOES = unsafe extern "system" fn(
    _index: GLuint,
    _r: GLboolean,
    _g: GLboolean,
    _b: GLboolean,
    _a: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform4uiv =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib1sv = unsafe extern "system" fn(_index: GLuint, _v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribs4svNV =
    unsafe extern "system" fn(_index: GLuint, _count: GLsizei, _v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glReadBufferNV = unsafe extern "system" fn(_mode: GLenum);
#[allow(non_camel_case_types)]
pub type PFN_glFrustumf = unsafe extern "system" fn(
    _l: GLfloat,
    _r: GLfloat,
    _b: GLfloat,
    _t: GLfloat,
    _n: GLfloat,
    _f: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glUnmapBufferOES = unsafe extern "system" fn(_target: GLenum) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib1dARB = unsafe extern "system" fn(_index: GLuint, _x: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glIsFramebufferOES = unsafe extern "system" fn(_framebuffer: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI1iv = unsafe extern "system" fn(_index: GLuint, _v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetnMapfv = unsafe extern "system" fn(
    _target: MapTarget,
    _query: MapQuery,
    _bufSize: GLsizei,
    _v: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord3xOES =
    unsafe extern "system" fn(_texture: TextureUnit, _s: GLfixed, _t: GLfixed, _r: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos2sv = unsafe extern "system" fn(_v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream2fATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _x: GLfloat, _y: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glBufferParameteriAPPLE =
    unsafe extern "system" fn(_target: GLenum, _pname: GLenum, _param: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glClearBufferfv =
    unsafe extern "system" fn(_buffer: Buffer, _drawbuffer: GLint, _value: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glBlendFuncSeparateINGR = unsafe extern "system" fn(
    _sfactorRGB: BlendingFactor,
    _dfactorRGB: BlendingFactor,
    _sfactorAlpha: BlendingFactor,
    _dfactorAlpha: BlendingFactor,
);
#[allow(non_camel_case_types)]
pub type PFN_glClearNamedBufferData = unsafe extern "system" fn(
    _buffer: GLuint,
    _internalformat: SizedInternalFormat,
    _format: PixelFormat,
    _type: PixelType,
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glColor4fv = unsafe extern "system" fn(_v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix4x2fv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetFixedv = unsafe extern "system" fn(_pname: GetPName, _params: *mut GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glSyncTextureINTEL = unsafe extern "system" fn(_texture: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4usvARB = unsafe extern "system" fn(_index: GLuint, _v: *const GLushort);
#[allow(non_camel_case_types)]
pub type PFN_glLoadTransposeMatrixxOES = unsafe extern "system" fn(_m: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glBindBufferRange = unsafe extern "system" fn(
    _target: BufferTargetARB,
    _index: GLuint,
    _buffer: GLuint,
    _offset: GLintptr,
    _size: GLsizeiptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glEvalCoord2xvOES = unsafe extern "system" fn(_coords: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glVertexP3uiv =
    unsafe extern "system" fn(_type: VertexPointerType, _value: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetQueryIndexediv = unsafe extern "system" fn(
    _target: QueryTarget,
    _index: GLuint,
    _pname: QueryParameterName,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform2uiEXT =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _v0: GLuint, _v1: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glTexImage2DMultisampleCoverageNV = unsafe extern "system" fn(
    _target: TextureTarget,
    _coverageSamples: GLsizei,
    _colorSamples: GLsizei,
    _internalFormat: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _fixedSampleLocations: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos4fvMESA = unsafe extern "system" fn(_v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glTexParameterIivOES = unsafe extern "system" fn(
    _target: TextureTarget,
    _pname: TextureParameterName,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform4i64vARB =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLint64);
#[allow(non_camel_case_types)]
pub type PFN_glTestFenceAPPLE = unsafe extern "system" fn(_fence: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord4fVertex4fvSUN =
    unsafe extern "system" fn(_tc: *const GLfloat, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glDepthBoundsdNV = unsafe extern "system" fn(_zmin: GLdouble, _zmax: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glMultTransposeMatrixdARB = unsafe extern "system" fn(_m: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI1uiEXT = unsafe extern "system" fn(_index: GLuint, _x: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glViewportPositionWScaleNV =
    unsafe extern "system" fn(_index: GLuint, _xcoeff: GLfloat, _ycoeff: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glUniformMatrix2dv = unsafe extern "system" fn(
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glColorFragmentOp1ATI = unsafe extern "system" fn(
    _op: FragmentOpATI,
    _dst: GLuint,
    _dstMask: GLuint,
    _dstMod: GLuint,
    _arg1: GLuint,
    _arg1Rep: GLuint,
    _arg1Mod: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetConvolutionParameterfvEXT = unsafe extern "system" fn(
    _target: ConvolutionTargetEXT,
    _pname: ConvolutionParameterEXT,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glMapGrid1d = unsafe extern "system" fn(_un: GLint, _u1: GLdouble, _u2: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexPointerListIBM = unsafe extern "system" fn(
    _size: GLint,
    _type: VertexPointerType,
    _stride: GLint,
    _pointer: *mut *const std::os::raw::c_void,
    _ptrstride: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glBindVideoCaptureStreamBufferNV = unsafe extern "system" fn(
    _video_capture_slot: GLuint,
    _stream: GLuint,
    _frame_region: GLenum,
    _offset: GLintptrARB,
);
#[allow(non_camel_case_types)]
pub type PFN_glFragmentMaterialivSGIX = unsafe extern "system" fn(
    _face: MaterialFace,
    _pname: MaterialParameter,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib2fARB =
    unsafe extern "system" fn(_index: GLuint, _x: GLfloat, _y: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glTextureParameterIuivEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _pname: TextureParameterName,
    _params: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetDoublei_v =
    unsafe extern "system" fn(_target: GetPName, _index: GLuint, _data: *mut GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glGetTextureLevelParameterfvEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _level: GLint,
    _pname: GetTextureParameter,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib3svNV = unsafe extern "system" fn(_index: GLuint, _v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glGetnConvolutionFilter = unsafe extern "system" fn(
    _target: ConvolutionTarget,
    _format: PixelFormat,
    _type: PixelType,
    _bufSize: GLsizei,
    _image: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glLoadMatrixx = unsafe extern "system" fn(_m: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glGenQueriesEXT = unsafe extern "system" fn(_n: GLsizei, _ids: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform4uiEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _v0: GLuint,
    _v1: GLuint,
    _v2: GLuint,
    _v3: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glIsTransformFeedback = unsafe extern "system" fn(_id: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glExtractComponentEXT =
    unsafe extern "system" fn(_res: GLuint, _src: GLuint, _num: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMakeTextureHandleNonResidentARB = unsafe extern "system" fn(_handle: GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glBlendFunciARB =
    unsafe extern "system" fn(_buf: GLuint, _src: BlendingFactor, _dst: BlendingFactor);
#[allow(non_camel_case_types)]
pub type PFN_glPushClientAttrib = unsafe extern "system" fn(_mask: ClientAttribMask);
#[allow(non_camel_case_types)]
pub type PFN_glGetAttribLocationARB =
    unsafe extern "system" fn(_programObj: GLhandleARB, _name: *const GLcharARB) -> GLint;
#[allow(non_camel_case_types)]
pub type PFN_glRotatexOES =
    unsafe extern "system" fn(_angle: GLfixed, _x: GLfixed, _y: GLfixed, _z: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glGetSamplerParameterfv =
    unsafe extern "system" fn(_sampler: GLuint, _pname: SamplerParameterF, _params: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib3sARB =
    unsafe extern "system" fn(_index: GLuint, _x: GLshort, _y: GLshort, _z: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream3sATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _x: GLshort, _y: GLshort, _z: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glViewportArrayvOES =
    unsafe extern "system" fn(_first: GLuint, _count: GLsizei, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glMinSampleShading = unsafe extern "system" fn(_value: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glCompressedTextureImage1DEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _level: GLint,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _border: GLint,
    _imageSize: GLsizei,
    _bits: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glDrawElementArrayAPPLE =
    unsafe extern "system" fn(_mode: PrimitiveType, _first: GLint, _count: GLsizei);
#[allow(non_camel_case_types)]
pub type PFN_glEndTransformFeedbackNV = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord1ivARB =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramLocalParameterI4uivNV =
    unsafe extern "system" fn(_target: ProgramTarget, _index: GLuint, _params: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glImportMemoryWin32HandleEXT = unsafe extern "system" fn(
    _memory: GLuint,
    _size: GLuint64,
    _handleType: ExternalHandleType,
    _handle: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexImage3D = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _internalformat: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _border: GLint,
    _format: PixelFormat,
    _type: PixelType,
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib2fv = unsafe extern "system" fn(_index: GLuint, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI2uiEXT =
    unsafe extern "system" fn(_index: GLuint, _x: GLuint, _y: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetShadingRateImagePaletteNV =
    unsafe extern "system" fn(_viewport: GLuint, _entry: GLuint, _rate: *mut GLenum);
#[allow(non_camel_case_types)]
pub type PFN_glCopyTexSubImage3DOES = unsafe extern "system" fn(
    _target: GLenum,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _zoffset: GLint,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glViewportIndexedfvNV = unsafe extern "system" fn(_index: GLuint, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib3fv = unsafe extern "system" fn(_index: GLuint, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glEndTransformFeedbackEXT = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferReadBufferEXT =
    unsafe extern "system" fn(_framebuffer: GLuint, _mode: ReadBufferMode);
#[allow(non_camel_case_types)]
pub type PFN_glLightModelf =
    unsafe extern "system" fn(_pname: LightModelParameter, _param: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos2dvMESA = unsafe extern "system" fn(_v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glNormalStream3iATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _nx: GLint, _ny: GLint, _nz: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord3d =
    unsafe extern "system" fn(_target: TextureUnit, _s: GLdouble, _t: GLdouble, _r: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glIsProgramNV = unsafe extern "system" fn(_id: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glTexturePageCommitmentMemNV = unsafe extern "system" fn(
    _texture: GLuint,
    _layer: GLint,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _zoffset: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _memory: GLuint,
    _offset: GLuint64,
    _commit: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glClientActiveVertexStreamATI = unsafe extern "system" fn(_stream: VertexStreamATI);
#[allow(non_camel_case_types)]
pub type PFN_glGenSamplers = unsafe extern "system" fn(_count: GLsizei, _samplers: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetColorTableParameterfv = unsafe extern "system" fn(
    _target: ColorTableTarget,
    _pname: GetColorTableParameterPNameSGI,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib3sv = unsafe extern "system" fn(_index: GLuint, _v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glGetTextureHandleARB = unsafe extern "system" fn(_texture: GLuint) -> GLuint64;
#[allow(non_camel_case_types)]
pub type PFN_glIsQuery = unsafe extern "system" fn(_id: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glTexImage2D = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _internalformat: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _border: GLint,
    _format: PixelFormat,
    _type: PixelType,
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glDisableiOES = unsafe extern "system" fn(_target: EnableCap, _index: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos4dv = unsafe extern "system" fn(_v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glDrawElementsInstancedNV = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _count: GLsizei,
    _type: PrimitiveType,
    _indices: *const std::os::raw::c_void,
    _primcount: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetQueryObjectuivARB =
    unsafe extern "system" fn(_id: GLuint, _pname: QueryObjectParameterName, _params: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord2fColor3fVertex3fSUN = unsafe extern "system" fn(
    _s: GLfloat,
    _t: GLfloat,
    _r: GLfloat,
    _g: GLfloat,
    _b: GLfloat,
    _x: GLfloat,
    _y: GLfloat,
    _z: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glStencilThenCoverStrokePathNV =
    unsafe extern "system" fn(_path: GLuint, _reference: GLint, _mask: GLuint, _coverMode: GLenum);
#[allow(non_camel_case_types)]
pub type PFN_glPushGroupMarkerEXT =
    unsafe extern "system" fn(_length: GLsizei, _marker: *const GLchar);
#[allow(non_camel_case_types)]
pub type PFN_glPresentFrameDualFillNV = unsafe extern "system" fn(
    _video_slot: GLuint,
    _minPresentTime: GLuint64EXT,
    _beginPresentTimeId: GLuint,
    _presentDurationId: GLuint,
    _type: GLenum,
    _target0: GLenum,
    _fill0: GLuint,
    _target1: GLenum,
    _fill1: GLuint,
    _target2: GLenum,
    _fill2: GLuint,
    _target3: GLenum,
    _fill3: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform4fv =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glDrawTexsOES = unsafe extern "system" fn(
    _x: GLshort,
    _y: GLshort,
    _z: GLshort,
    _width: GLshort,
    _height: GLshort,
);
#[allow(non_camel_case_types)]
pub type PFN_glElementPointerATI =
    unsafe extern "system" fn(_type: ElementPointerTypeATI, _pointer: *const std::os::raw::c_void);
#[allow(non_camel_case_types)]
pub type PFN_glGetSemaphoreParameterivNV = unsafe extern "system" fn(
    _semaphore: GLuint,
    _pname: SemaphoreParameterName,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetObjectLabelEXT = unsafe extern "system" fn(
    _type: GLenum,
    _object: GLuint,
    _bufSize: GLsizei,
    _length: *mut GLsizei,
    _label: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glCompressedTextureImage2DEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _level: GLint,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _border: GLint,
    _imageSize: GLsizei,
    _bits: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glLightModelxOES =
    unsafe extern "system" fn(_pname: LightModelParameter, _param: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glCopyNamedBufferSubData = unsafe extern "system" fn(
    _readBuffer: GLuint,
    _writeBuffer: GLuint,
    _readOffset: GLintptr,
    _writeOffset: GLintptr,
    _size: GLsizeiptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteFragmentShaderATI = unsafe extern "system" fn(_id: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glFogfv = unsafe extern "system" fn(_pname: FogParameter, _params: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glPathTexGenNV = unsafe extern "system" fn(
    _texCoordSet: PathColor,
    _genMode: PathGenMode,
    _components: GLint,
    _coeffs: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix4dv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform2ui = unsafe extern "system" fn(_location: GLint, _v0: GLuint, _v1: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glPointParameteri =
    unsafe extern "system" fn(_pname: PointParameterNameARB, _param: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glColor4dv = unsafe extern "system" fn(_v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glMinmaxEXT = unsafe extern "system" fn(
    _target: MinmaxTargetEXT,
    _internalformat: InternalFormat,
    _sink: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glNamedFramebufferTexture2DEXT = unsafe extern "system" fn(
    _framebuffer: GLuint,
    _attachment: FramebufferAttachment,
    _textarget: TextureTarget,
    _texture: GLuint,
    _level: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream1dvATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _coords: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glMapBufferRangeEXT = unsafe extern "system" fn(
    _target: BufferTargetARB,
    _offset: GLintptr,
    _length: GLsizeiptr,
    _access: MapBufferAccessMask,
) -> *mut std::os::raw::c_void;
#[allow(non_camel_case_types)]
pub type PFN_glGetHistogram = unsafe extern "system" fn(
    _target: HistogramTargetEXT,
    _reset: GLboolean,
    _format: PixelFormat,
    _type: PixelType,
    _values: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetTexImage = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _format: PixelFormat,
    _type: PixelType,
    _pixels: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3iv = unsafe extern "system" fn(_v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib3fvARB = unsafe extern "system" fn(_index: GLuint, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glDrawElementsInstancedBaseInstance = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _count: GLsizei,
    _type: PrimitiveType,
    _indices: *const std::os::raw::c_void,
    _instancecount: GLsizei,
    _baseinstance: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glClampColor =
    unsafe extern "system" fn(_target: ClampColorTargetARB, _clamp: ClampColorModeARB);
#[allow(non_camel_case_types)]
pub type PFN_glBlendParameteriNV = unsafe extern "system" fn(_pname: GLenum, _value: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramLocalParameterfvARB =
    unsafe extern "system" fn(_target: ProgramTarget, _index: GLuint, _params: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glNamedStringARB = unsafe extern "system" fn(
    _type: GLenum,
    _namelen: GLint,
    _name: *const GLchar,
    _stringlen: GLint,
    _string: *const GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform1f =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _v0: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glRotated =
    unsafe extern "system" fn(_angle: GLdouble, _x: GLdouble, _y: GLdouble, _z: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteTransformFeedbacks =
    unsafe extern "system" fn(_n: GLsizei, _ids: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glEndPerfQueryINTEL = unsafe extern "system" fn(_queryHandle: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glIsNamedBufferResidentNV = unsafe extern "system" fn(_buffer: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glPathDashArrayNV =
    unsafe extern "system" fn(_path: GLuint, _dashCount: GLsizei, _dashArray: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform2ui64vARB = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glPixelTexGenParameterfvSGIS =
    unsafe extern "system" fn(_pname: PixelTexGenParameterNameSGIS, _params: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexAttribPointervARB = unsafe extern "system" fn(
    _index: GLuint,
    _pname: VertexAttribPointerPropertyARB,
    _pointer: *mut *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glScissorArrayvOES =
    unsafe extern "system" fn(_first: GLuint, _count: GLsizei, _v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetTrackMatrixivNV = unsafe extern "system" fn(
    _target: VertexAttribEnumNV,
    _address: GLuint,
    _pname: VertexAttribEnumNV,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glWaitVkSemaphoreNV = unsafe extern "system" fn(_vkSemaphore: GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glPathGlyphsNV = unsafe extern "system" fn(
    _firstPathName: GLuint,
    _fontTarget: PathFontTarget,
    _fontName: *const std::os::raw::c_void,
    _fontStyle: PathFontStyle,
    _numGlyphs: GLsizei,
    _type: PathElementType,
    _charcodes: *const std::os::raw::c_void,
    _handleMissingGlyphs: PathHandleMissingGlyphs,
    _pathParameterTemplate: GLuint,
    _emScale: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glCallList = unsafe extern "system" fn(_list: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord4bvOES =
    unsafe extern "system" fn(_texture: TextureUnit, _coords: *const GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4dv = unsafe extern "system" fn(_index: GLuint, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glPrioritizeTexturesxOES =
    unsafe extern "system" fn(_n: GLsizei, _textures: *const GLuint, _priorities: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glExtGetFramebuffersQCOM = unsafe extern "system" fn(
    _framebuffers: *mut GLuint,
    _maxFramebuffers: GLint,
    _numFramebuffers: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramNamedParameter4fNV = unsafe extern "system" fn(
    _id: GLuint,
    _len: GLsizei,
    _name: *const GLubyte,
    _x: GLfloat,
    _y: GLfloat,
    _z: GLfloat,
    _w: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glClearColorx =
    unsafe extern "system" fn(_red: GLfixed, _green: GLfixed, _blue: GLfixed, _alpha: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glPixelTransformParameterivEXT = unsafe extern "system" fn(
    _target: PixelTransformTargetEXT,
    _pname: PixelTransformPNameEXT,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetTransformFeedbackVaryingNV =
    unsafe extern "system" fn(_program: GLuint, _index: GLuint, _location: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glCoverageOperationNV = unsafe extern "system" fn(_operation: GLenum);
#[allow(non_camel_case_types)]
pub type PFN_glVertexArrayIndexOffsetEXT = unsafe extern "system" fn(
    _vaobj: GLuint,
    _buffer: GLuint,
    _type: IndexPointerType,
    _stride: GLsizei,
    _offset: GLintptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glDrawElementsInstancedEXT = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _count: GLsizei,
    _type: DrawElementsType,
    _indices: *const std::os::raw::c_void,
    _primcount: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4NubvARB = unsafe extern "system" fn(_index: GLuint, _v: *const GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos2svARB = unsafe extern "system" fn(_v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glGenerateMipmapEXT = unsafe extern "system" fn(_target: TextureTarget);
#[allow(non_camel_case_types)]
pub type PFN_glCopyTexImage1D = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _internalformat: InternalFormat,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
    _border: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glPixelStorex =
    unsafe extern "system" fn(_pname: PixelStoreParameter, _param: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixRotatedEXT = unsafe extern "system" fn(
    _mode: MatrixMode,
    _angle: GLdouble,
    _x: GLdouble,
    _y: GLdouble,
    _z: GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetBufferPointervOES = unsafe extern "system" fn(
    _target: BufferTargetARB,
    _pname: BufferPointerNameARB,
    _params: *mut *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramEnvParameterI4iNV = unsafe extern "system" fn(
    _target: ProgramTarget,
    _index: GLuint,
    _x: GLint,
    _y: GLint,
    _z: GLint,
    _w: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexStorage2DEXT = unsafe extern "system" fn(
    _target: TextureTarget,
    _levels: GLsizei,
    _internalformat: SizedInternalFormat,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glColor4fNormal3fVertex3fvSUN =
    unsafe extern "system" fn(_c: *const GLfloat, _n: *const GLfloat, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetFloati_v =
    unsafe extern "system" fn(_target: GetPName, _index: GLuint, _data: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glFragmentLightiSGIX = unsafe extern "system" fn(
    _light: FragmentLightNameSGIX,
    _pname: FragmentLightParameterSGIX,
    _param: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetObjectPtrLabelKHR = unsafe extern "system" fn(
    _ptr: *const std::os::raw::c_void,
    _bufSize: GLsizei,
    _length: *mut GLsizei,
    _label: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetNamedFramebufferAttachmentParameteriv = unsafe extern "system" fn(
    _framebuffer: GLuint,
    _attachment: FramebufferAttachment,
    _pname: FramebufferAttachmentParameterName,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glLoadPaletteFromModelViewMatrixOES = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glNormalPointerListIBM = unsafe extern "system" fn(
    _type: NormalPointerType,
    _stride: GLint,
    _pointer: *mut *const std::os::raw::c_void,
    _ptrstride: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glImageTransformParameterfvHP = unsafe extern "system" fn(
    _target: ImageTransformTargetHP,
    _pname: ImageTransformPNameHP,
    _params: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord3iv = unsafe extern "system" fn(_target: TextureUnit, _v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glMapGrid2xOES =
    unsafe extern "system" fn(_n: GLint, _u1: GLfixed, _u2: GLfixed, _v1: GLfixed, _v2: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glTbufferMask3DFX = unsafe extern "system" fn(_mask: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glUseProgramStagesEXT =
    unsafe extern "system" fn(_pipeline: GLuint, _stages: UseProgramStageMask, _program: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib1svARB = unsafe extern "system" fn(_index: GLuint, _v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glUniform1uiEXT = unsafe extern "system" fn(_location: GLint, _v0: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL1ui64vARB =
    unsafe extern "system" fn(_index: GLuint, _v: *const GLuint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribs3hvNV =
    unsafe extern "system" fn(_index: GLuint, _n: GLsizei, _v: *const GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glWaitSyncAPPLE =
    unsafe extern "system" fn(_sync: GLsync, _flags: SyncBehaviorFlags, _timeout: GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteRenderbuffersEXT =
    unsafe extern "system" fn(_n: GLsizei, _renderbuffers: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetListParameterfvSGIX =
    unsafe extern "system" fn(_list: GLuint, _pname: ListParameterName, _params: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glNewObjectBufferATI = unsafe extern "system" fn(
    _size: GLsizei,
    _pointer: *const std::os::raw::c_void,
    _usage: ArrayObjectUsageATI,
) -> GLuint;
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferFetchBarrierEXT = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glColor3fv = unsafe extern "system" fn(_v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glCreateProgramPipelines =
    unsafe extern "system" fn(_n: GLsizei, _pipelines: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetPathSpacingNV = unsafe extern "system" fn(
    _pathListMode: PathListMode,
    _numPaths: GLsizei,
    _pathNameType: PathElementType,
    _paths: *const std::os::raw::c_void,
    _pathBase: GLuint,
    _advanceScale: GLfloat,
    _kerningScale: GLfloat,
    _transformType: PathTransformType,
    _returnedSpacing: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform1ui64vARB =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL2dEXT =
    unsafe extern "system" fn(_index: GLuint, _x: GLdouble, _y: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glIsEnabledIndexedEXT =
    unsafe extern "system" fn(_target: EnableCap, _index: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glDisablei = unsafe extern "system" fn(_target: EnableCap, _index: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMapVertexAttrib1dAPPLE = unsafe extern "system" fn(
    _index: GLuint,
    _size: GLuint,
    _u1: GLdouble,
    _u2: GLdouble,
    _stride: GLint,
    _order: GLint,
    _points: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetCombinerInputParameterfvNV = unsafe extern "system" fn(
    _stage: CombinerStageNV,
    _portion: CombinerPortionNV,
    _variable: CombinerVariableNV,
    _pname: CombinerParameterNV,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glIsTextureHandleResidentNV =
    unsafe extern "system" fn(_handle: GLuint64) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glTexImage3DOES = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _border: GLint,
    _format: PixelFormat,
    _type: PixelType,
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glStencilMaskSeparate =
    unsafe extern "system" fn(_face: StencilFaceDirection, _mask: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiDrawArraysIndirect = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _indirect: *const std::os::raw::c_void,
    _drawcount: GLsizei,
    _stride: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureStorage3DMultisampleEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: GLenum,
    _samples: GLsizei,
    _internalformat: SizedInternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _fixedsamplelocations: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glConvolutionParameterf = unsafe extern "system" fn(
    _target: ConvolutionTarget,
    _pname: ConvolutionParameterEXT,
    _params: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glIsFramebuffer = unsafe extern "system" fn(_framebuffer: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexArrayPointervEXT = unsafe extern "system" fn(
    _vaobj: GLuint,
    _pname: VertexArrayPName,
    _param: *mut *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glIsPointInFillPathNV =
    unsafe extern "system" fn(_path: GLuint, _mask: GLuint, _x: GLfloat, _y: GLfloat) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glReplacementCodeuiSUN = unsafe extern "system" fn(_code: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glBindTransformFeedbackNV =
    unsafe extern "system" fn(_target: BufferTargetARB, _id: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glClearColor =
    unsafe extern "system" fn(_red: GLfloat, _green: GLfloat, _blue: GLfloat, _alpha: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glClearStencil = unsafe extern "system" fn(_s: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glCompressedTexSubImage2DARB = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _format: InternalFormat,
    _imageSize: GLsizei,
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferTexture2D = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _attachment: FramebufferAttachment,
    _textarget: TextureTarget,
    _texture: GLuint,
    _level: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferTexture2DMultisampleEXT = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _attachment: FramebufferAttachment,
    _textarget: TextureTarget,
    _texture: GLuint,
    _level: GLint,
    _samples: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3uiEXT =
    unsafe extern "system" fn(_red: GLuint, _green: GLuint, _blue: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glReplacementCodeuiColor4fNormal3fVertex3fSUN = unsafe extern "system" fn(
    _rc: GLuint,
    _r: GLfloat,
    _g: GLfloat,
    _b: GLfloat,
    _a: GLfloat,
    _nx: GLfloat,
    _ny: GLfloat,
    _nz: GLfloat,
    _x: GLfloat,
    _y: GLfloat,
    _z: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos3ivARB = unsafe extern "system" fn(_v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord4sARB = unsafe extern "system" fn(
    _target: TextureUnit,
    _s: GLshort,
    _t: GLshort,
    _r: GLshort,
    _q: GLshort,
);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos4svMESA = unsafe extern "system" fn(_v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformHandleui64vNV = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _values: *const GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glCreateShader = unsafe extern "system" fn(_type: ShaderType) -> GLuint;
#[allow(non_camel_case_types)]
pub type PFN_glEnableVertexAttribArrayARB = unsafe extern "system" fn(_index: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glReadBufferIndexedEXT =
    unsafe extern "system" fn(_src: ReadBufferMode, _index: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glFogi = unsafe extern "system" fn(_pname: FogParameter, _param: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexArrayParameteriAPPLE =
    unsafe extern "system" fn(_pname: VertexArrayPNameAPPLE, _param: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glCurrentPaletteMatrixARB = unsafe extern "system" fn(_index: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetColorTable = unsafe extern "system" fn(
    _target: ColorTableTarget,
    _format: PixelFormat,
    _type: PixelType,
    _table: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetConvolutionParameterxvOES =
    unsafe extern "system" fn(_target: GLenum, _pname: GLenum, _params: *mut GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord4xOES = unsafe extern "system" fn(
    _texture: TextureUnit,
    _s: GLfixed,
    _t: GLfixed,
    _r: GLfixed,
    _q: GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixMultTransposefEXT =
    unsafe extern "system" fn(_mode: MatrixMode, _m: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix3fvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord3s =
    unsafe extern "system" fn(_target: TextureUnit, _s: GLshort, _t: GLshort, _r: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glUniform1f = unsafe extern "system" fn(_location: GLint, _v0: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glMultiDrawArraysIndirectEXT = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _indirect: *const std::os::raw::c_void,
    _drawcount: GLsizei,
    _stride: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos3sARB = unsafe extern "system" fn(_x: GLshort, _y: GLshort, _z: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos2fv = unsafe extern "system" fn(_v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glTransformFeedbackBufferBase =
    unsafe extern "system" fn(_xfb: GLuint, _index: GLuint, _buffer: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glBindRenderbufferEXT =
    unsafe extern "system" fn(_target: RenderbufferTarget, _renderbuffer: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGenVertexArrays = unsafe extern "system" fn(_n: GLsizei, _arrays: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVariantubvEXT = unsafe extern "system" fn(_id: GLuint, _addr: *const GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4NbvARB = unsafe extern "system" fn(_index: GLuint, _v: *const GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glEndList = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glGetTextureParameterIiv =
    unsafe extern "system" fn(_texture: GLuint, _pname: GetTextureParameter, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glNamedProgramStringEXT = unsafe extern "system" fn(
    _program: GLuint,
    _target: ProgramTarget,
    _format: ProgramFormat,
    _len: GLsizei,
    _string: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI4i =
    unsafe extern "system" fn(_index: GLuint, _x: GLint, _y: GLint, _z: GLint, _w: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glBinormal3fvEXT = unsafe extern "system" fn(_v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glEvalCoord2f = unsafe extern "system" fn(_u: GLfloat, _v: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glActiveTexture = unsafe extern "system" fn(_texture: TextureUnit);
#[allow(non_camel_case_types)]
pub type PFN_glGetnTexImageARB = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _format: PixelFormat,
    _type: PixelType,
    _bufSize: GLsizei,
    _img: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultMatrixx = unsafe extern "system" fn(_m: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform3fvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream2fvATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _coords: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glBindBufferBase =
    unsafe extern "system" fn(_target: BufferTargetARB, _index: GLuint, _buffer: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord4dvARB =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glGetPathCommandsNV =
    unsafe extern "system" fn(_path: GLuint, _commands: *mut GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3bvEXT = unsafe extern "system" fn(_v: *const GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glNamedRenderbufferStorage = unsafe extern "system" fn(
    _renderbuffer: GLuint,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureStorage2DEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: GLenum,
    _levels: GLsizei,
    _internalformat: SizedInternalFormat,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetFramebufferParameteriv = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _pname: FramebufferAttachmentParameterName,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord2svARB =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glCopyImageSubDataNV = unsafe extern "system" fn(
    _srcName: GLuint,
    _srcTarget: CopyBufferSubDataTarget,
    _srcLevel: GLint,
    _srcX: GLint,
    _srcY: GLint,
    _srcZ: GLint,
    _dstName: GLuint,
    _dstTarget: CopyBufferSubDataTarget,
    _dstLevel: GLint,
    _dstX: GLint,
    _dstY: GLint,
    _dstZ: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glCompressedTexImage3DOES = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _border: GLint,
    _imageSize: GLsizei,
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureSubImage2DEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _format: PixelFormat,
    _type: PixelType,
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniformMatrix4x3fvNV = unsafe extern "system" fn(
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glDrawElementsBaseVertexEXT = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _count: GLsizei,
    _type: DrawElementsType,
    _indices: *const std::os::raw::c_void,
    _basevertex: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexArrayPointeri_vEXT = unsafe extern "system" fn(
    _vaobj: GLuint,
    _index: GLuint,
    _pname: VertexArrayPName,
    _param: *mut *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetCompressedTextureSubImage = unsafe extern "system" fn(
    _texture: GLuint,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _zoffset: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _bufSize: GLsizei,
    _pixels: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glDrawElementsInstancedBaseVertexEXT = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _count: GLsizei,
    _type: DrawElementsType,
    _indices: *const std::os::raw::c_void,
    _instancecount: GLsizei,
    _basevertex: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform3i64NV =
    unsafe extern "system" fn(_location: GLint, _x: GLint64EXT, _y: GLint64EXT, _z: GLint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribP4ui = unsafe extern "system" fn(
    _index: GLuint,
    _type: VertexAttribPointerType,
    _normalized: GLboolean,
    _value: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexParameteri =
    unsafe extern "system" fn(_target: TextureTarget, _pname: TextureParameterName, _param: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glColor4bv = unsafe extern "system" fn(_v: *const GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glCompressedTextureSubImage1D = unsafe extern "system" fn(
    _texture: GLuint,
    _level: GLint,
    _xoffset: GLint,
    _width: GLsizei,
    _format: InternalFormat,
    _imageSize: GLsizei,
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glReplacementCodeuiTexCoord2fColor4fNormal3fVertex3fvSUN =
    unsafe extern "system" fn(
        _rc: *const GLuint,
        _tc: *const GLfloat,
        _c: *const GLfloat,
        _n: *const GLfloat,
        _v: *const GLfloat,
    );
#[allow(non_camel_case_types)]
pub type PFN_glGetShaderPrecisionFormat = unsafe extern "system" fn(
    _shadertype: ShaderType,
    _precisiontype: PrecisionType,
    _range: *mut GLint,
    _precision: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4dNV = unsafe extern "system" fn(
    _index: GLuint,
    _x: GLdouble,
    _y: GLdouble,
    _z: GLdouble,
    _w: GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glFlushMappedBufferRangeEXT =
    unsafe extern "system" fn(_target: BufferTargetARB, _offset: GLintptr, _length: GLsizeiptr);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos2fvMESA = unsafe extern "system" fn(_v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetClipPlanef =
    unsafe extern "system" fn(_plane: ClipPlaneName, _equation: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteShader = unsafe extern "system" fn(_shader: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glPixelTexGenSGIX = unsafe extern "system" fn(_mode: PixelTexGenModeSGIX);
#[allow(non_camel_case_types)]
pub type PFN_glGetUniformuiv =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _params: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGenProgramsNV = unsafe extern "system" fn(_n: GLsizei, _programs: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glPassThrough = unsafe extern "system" fn(_token: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glSetMultisamplefvAMD =
    unsafe extern "system" fn(_pname: GLenum, _index: GLuint, _val: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos2xvOES = unsafe extern "system" fn(_coords: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glTangent3bEXT = unsafe extern "system" fn(_tx: GLbyte, _ty: GLbyte, _tz: GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glImportSemaphoreFdEXT =
    unsafe extern "system" fn(_semaphore: GLuint, _handleType: ExternalHandleType, _fd: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord2fColor4ubVertex3fvSUN =
    unsafe extern "system" fn(_tc: *const GLfloat, _c: *const GLubyte, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoordP3ui =
    unsafe extern "system" fn(_type: TexCoordPointerType, _coords: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glUniform2d = unsafe extern "system" fn(_location: GLint, _x: GLdouble, _y: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glDepthRangeIndexedfOES =
    unsafe extern "system" fn(_index: GLuint, _n: GLfloat, _f: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glExtGetTexLevelParameterivQCOM = unsafe extern "system" fn(
    _texture: GLuint,
    _face: GLenum,
    _level: GLint,
    _pname: GLenum,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetTexFilterFuncSGIS = unsafe extern "system" fn(
    _target: TextureTarget,
    _filter: TextureFilterSGIS,
    _weights: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glMateriali =
    unsafe extern "system" fn(_face: MaterialFace, _pname: MaterialParameter, _param: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glTexBumpParameterfvATI =
    unsafe extern "system" fn(_pname: TexBumpParameterATI, _param: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVDPAUIsSurfaceNV =
    unsafe extern "system" fn(_surface: GLvdpauSurfaceNV) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord2dvARB =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexArrayVertexAttribIOffsetEXT = unsafe extern "system" fn(
    _vaobj: GLuint,
    _buffer: GLuint,
    _index: GLuint,
    _size: GLint,
    _type: VertexAttribType,
    _stride: GLsizei,
    _offset: GLintptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glSignalVkSemaphoreNV = unsafe extern "system" fn(_vkSemaphore: GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glGetArrayObjectivATI =
    unsafe extern "system" fn(_array: EnableCap, _pname: ArrayObjectPNameATI, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib2svARB = unsafe extern "system" fn(_index: GLuint, _v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord4s =
    unsafe extern "system" fn(_s: GLshort, _t: GLshort, _r: GLshort, _q: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glObjectLabel = unsafe extern "system" fn(
    _identifier: ObjectIdentifier,
    _name: GLuint,
    _length: GLsizei,
    _label: *const GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glRectdv = unsafe extern "system" fn(_v1: *const GLdouble, _v2: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glColor4f =
    unsafe extern "system" fn(_red: GLfloat, _green: GLfloat, _blue: GLfloat, _alpha: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glActiveVaryingNV = unsafe extern "system" fn(_program: GLuint, _name: *const GLchar);
#[allow(non_camel_case_types)]
pub type PFN_glCompressedMultiTexImage3DEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _level: GLint,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _border: GLint,
    _imageSize: GLsizei,
    _bits: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glIsMemoryObjectEXT = unsafe extern "system" fn(_memoryObject: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glGetDebugMessageLogARB = unsafe extern "system" fn(
    _count: GLuint,
    _bufSize: GLsizei,
    _sources: *mut DebugSource,
    _types: *mut DebugType,
    _ids: *mut GLuint,
    _severities: *mut DebugSeverity,
    _lengths: *mut GLsizei,
    _messageLog: *mut GLchar,
) -> GLuint;
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexAttribLdv =
    unsafe extern "system" fn(_index: GLuint, _pname: VertexAttribEnum, _params: *mut GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glPointParameterfEXT =
    unsafe extern "system" fn(_pname: PointParameterNameARB, _param: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glMulticastGetQueryObjectui64vNV =
    unsafe extern "system" fn(_gpu: GLuint, _id: GLuint, _pname: GLenum, _params: *mut GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteLists = unsafe extern "system" fn(_list: GLuint, _range: GLsizei);
#[allow(non_camel_case_types)]
pub type PFN_glSelectBuffer = unsafe extern "system" fn(_size: GLsizei, _buffer: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glTextureMaterialEXT =
    unsafe extern "system" fn(_face: MaterialFace, _mode: MaterialParameter);
#[allow(non_camel_case_types)]
pub type PFN_glUniform3dv =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glPathCommandsNV = unsafe extern "system" fn(
    _path: GLuint,
    _numCommands: GLsizei,
    _commands: *const GLubyte,
    _numCoords: GLsizei,
    _coordType: PathCoordType,
    _coords: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glNamedFramebufferParameteri = unsafe extern "system" fn(
    _framebuffer: GLuint,
    _pname: FramebufferParameterName,
    _param: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos3iMESA = unsafe extern "system" fn(_x: GLint, _y: GLint, _z: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetnMinmaxARB = unsafe extern "system" fn(
    _target: MinmaxTargetEXT,
    _reset: GLboolean,
    _format: PixelFormat,
    _type: PixelType,
    _bufSize: GLsizei,
    _values: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoordP1ui =
    unsafe extern "system" fn(_type: TexCoordPointerType, _coords: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexAttribfvARB = unsafe extern "system" fn(
    _index: GLuint,
    _pname: VertexAttribPropertyARB,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferTexture2DMultisampleIMG = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _attachment: FramebufferAttachment,
    _textarget: TextureTarget,
    _texture: GLuint,
    _level: GLint,
    _samples: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord3dv = unsafe extern "system" fn(_v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord2sARB =
    unsafe extern "system" fn(_target: TextureUnit, _s: GLshort, _t: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3d =
    unsafe extern "system" fn(_red: GLdouble, _green: GLdouble, _blue: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL4dEXT = unsafe extern "system" fn(
    _index: GLuint,
    _x: GLdouble,
    _y: GLdouble,
    _z: GLdouble,
    _w: GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetVariantArrayObjectfvATI =
    unsafe extern "system" fn(_id: GLuint, _pname: ArrayObjectPNameATI, _params: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord1iARB = unsafe extern "system" fn(_target: TextureUnit, _s: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix3x2fvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexSubImage4DSGIS = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _zoffset: GLint,
    _woffset: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _size4d: GLsizei,
    _format: PixelFormat,
    _type: PixelType,
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureImage3DMultisampleCoverageNV = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _coverageSamples: GLsizei,
    _colorSamples: GLsizei,
    _internalFormat: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _fixedSampleLocations: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream3iATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _x: GLint, _y: GLint, _z: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetQueryBufferObjectuiv = unsafe extern "system" fn(
    _id: GLuint,
    _buffer: GLuint,
    _pname: QueryObjectParameterName,
    _offset: GLintptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform1fEXT =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _v0: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glQueryObjectParameteruiAMD =
    unsafe extern "system" fn(_target: QueryTarget, _id: GLuint, _pname: GLenum, _param: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMulticastCopyBufferSubDataNV = unsafe extern "system" fn(
    _readGpu: GLuint,
    _writeGpuMask: GLbitfield,
    _readBuffer: GLuint,
    _writeBuffer: GLuint,
    _readOffset: GLintptr,
    _writeOffset: GLintptr,
    _size: GLsizeiptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glClearNamedFramebufferfv = unsafe extern "system" fn(
    _framebuffer: GLuint,
    _buffer: Buffer,
    _drawbuffer: GLint,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glEvalCoord1d = unsafe extern "system" fn(_u: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glIsEnabled = unsafe extern "system" fn(_cap: EnableCap) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glBlendEquation = unsafe extern "system" fn(_mode: BlendEquationModeEXT);
#[allow(non_camel_case_types)]
pub type PFN_glBufferStorage = unsafe extern "system" fn(
    _target: BufferStorageTarget,
    _size: GLsizeiptr,
    _data: *const std::os::raw::c_void,
    _flags: BufferStorageMask,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform4i64vARB = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord1d = unsafe extern "system" fn(_s: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glColor4i =
    unsafe extern "system" fn(_red: GLint, _green: GLint, _blue: GLint, _alpha: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glDrawElementsInstancedBaseVertexBaseInstance = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _count: GLsizei,
    _type: DrawElementsType,
    _indices: *const std::os::raw::c_void,
    _instancecount: GLsizei,
    _basevertex: GLint,
    _baseinstance: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glColorMaskiEXT = unsafe extern "system" fn(
    _index: GLuint,
    _r: GLboolean,
    _g: GLboolean,
    _b: GLboolean,
    _a: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glGenerateMipmapOES = unsafe extern "system" fn(_target: TextureTarget);
#[allow(non_camel_case_types)]
pub type PFN_glGetBooleanIndexedvEXT =
    unsafe extern "system" fn(_target: BufferTargetARB, _index: GLuint, _data: *mut GLboolean);
#[allow(non_camel_case_types)]
pub type PFN_glGetTextureSamplerHandleNV =
    unsafe extern "system" fn(_texture: GLuint, _sampler: GLuint) -> GLuint64;
#[allow(non_camel_case_types)]
pub type PFN_glDebugMessageEnableAMD = unsafe extern "system" fn(
    _category: GLenum,
    _severity: DebugSeverity,
    _count: GLsizei,
    _ids: *const GLuint,
    _enabled: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetMapAttribParameterivNV = unsafe extern "system" fn(
    _target: EvalTargetNV,
    _index: GLuint,
    _pname: MapAttribParameterNV,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixMult3x2fNV =
    unsafe extern "system" fn(_matrixMode: GLenum, _m: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord4fARB = unsafe extern "system" fn(
    _target: TextureUnit,
    _s: GLfloat,
    _t: GLfloat,
    _r: GLfloat,
    _q: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glCullParameterfvEXT =
    unsafe extern "system" fn(_pname: CullParameterEXT, _params: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glBindTextureUnit = unsafe extern "system" fn(_unit: GLuint, _texture: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glFeedbackBuffer =
    unsafe extern "system" fn(_size: GLsizei, _type: FeedbackType, _buffer: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glCopyTextureImage1DEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _level: GLint,
    _internalformat: InternalFormat,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
    _border: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferTextureOES = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _attachment: FramebufferAttachment,
    _texture: GLuint,
    _level: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetFloati_vEXT =
    unsafe extern "system" fn(_pname: GetPName, _index: GLuint, _params: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glListParameterfSGIX =
    unsafe extern "system" fn(_list: GLuint, _pname: ListParameterName, _param: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glPointParameterfARB =
    unsafe extern "system" fn(_pname: PointParameterNameARB, _param: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glColor3hNV =
    unsafe extern "system" fn(_red: GLhalfNV, _green: GLhalfNV, _blue: GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glGetInteger64v = unsafe extern "system" fn(_pname: GetPName, _data: *mut GLint64);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform3dvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos2s = unsafe extern "system" fn(_x: GLshort, _y: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glBindSampler = unsafe extern "system" fn(_unit: GLuint, _sampler: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glTessellationModeAMD = unsafe extern "system" fn(_mode: GLenum);
#[allow(non_camel_case_types)]
pub type PFN_glIndexub = unsafe extern "system" fn(_c: GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glPixelStorei = unsafe extern "system" fn(_pname: PixelStoreParameter, _param: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetActiveUniformsiv = unsafe extern "system" fn(
    _program: GLuint,
    _uniformCount: GLsizei,
    _uniformIndices: *const GLuint,
    _pname: UniformPName,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord1sv = unsafe extern "system" fn(_v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glTexImage3DMultisampleCoverageNV = unsafe extern "system" fn(
    _target: TextureTarget,
    _coverageSamples: GLsizei,
    _colorSamples: GLsizei,
    _internalFormat: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _fixedSampleLocations: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureParameteriv =
    unsafe extern "system" fn(_texture: GLuint, _pname: TextureParameterName, _param: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glUniformMatrix4x2dv = unsafe extern "system" fn(
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glCopyMultiTexSubImage2DEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glStartInstrumentsSGIX = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glUploadGpuMaskNVX = unsafe extern "system" fn(_mask: GLbitfield);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL3i64vNV =
    unsafe extern "system" fn(_index: GLuint, _v: *const GLint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribLFormatNV = unsafe extern "system" fn(
    _index: GLuint,
    _size: GLint,
    _type: VertexAttribLType,
    _stride: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteQueries = unsafe extern "system" fn(_n: GLsizei, _ids: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexAttribArrayObjectivATI =
    unsafe extern "system" fn(_index: GLuint, _pname: ArrayObjectPNameATI, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glRenderbufferStorageMultisampleAdvancedAMD = unsafe extern "system" fn(
    _target: RenderbufferTarget,
    _samples: GLsizei,
    _storageSamples: GLsizei,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexImage2DEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _level: GLint,
    _internalformat: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _border: GLint,
    _format: PixelFormat,
    _type: PixelType,
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glMakeBufferResidentNV = unsafe extern "system" fn(_target: GLenum, _access: GLenum);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribLPointerEXT = unsafe extern "system" fn(
    _index: GLuint,
    _size: GLint,
    _type: VertexAttribLType,
    _stride: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexEnvxv = unsafe extern "system" fn(
    _target: TextureEnvTarget,
    _pname: TextureEnvParameter,
    _params: *const GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetnPixelMapusv =
    unsafe extern "system" fn(_map: PixelMap, _bufSize: GLsizei, _values: *mut GLushort);
#[allow(non_camel_case_types)]
pub type PFN_glStencilFillPathNV =
    unsafe extern "system" fn(_path: GLuint, _fillMode: PathFillMode, _mask: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glEnableClientStateiEXT = unsafe extern "system" fn(_array: EnableCap, _index: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glPrimitiveRestartIndex = unsafe extern "system" fn(_index: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos2dvARB = unsafe extern "system" fn(_v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glTransformFeedbackVaryingsEXT = unsafe extern "system" fn(
    _program: GLuint,
    _count: GLsizei,
    _varyings: *const *const GLchar,
    _bufferMode: GLenum,
);
#[allow(non_camel_case_types)]
pub type PFN_glEvalCoord2d = unsafe extern "system" fn(_u: GLdouble, _v: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glSignalVkFenceNV = unsafe extern "system" fn(_vkFence: GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glBlendFunciOES =
    unsafe extern "system" fn(_buf: GLuint, _src: BlendingFactor, _dst: BlendingFactor);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribs1hvNV =
    unsafe extern "system" fn(_index: GLuint, _n: GLsizei, _v: *const GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glGetMemoryObjectParameterivEXT = unsafe extern "system" fn(
    _memoryObject: GLuint,
    _pname: MemoryObjectParameterName,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glColor4xOES =
    unsafe extern "system" fn(_red: GLfixed, _green: GLfixed, _blue: GLfixed, _alpha: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexParameterIuivEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _pname: TextureParameterName,
    _params: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord4hvNV = unsafe extern "system" fn(_v: *const GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glProvokingVertexEXT = unsafe extern "system" fn(_mode: VertexProvokingMode);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib2fNV =
    unsafe extern "system" fn(_index: GLuint, _x: GLfloat, _y: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glReplacementCodeusSUN = unsafe extern "system" fn(_code: GLushort);
#[allow(non_camel_case_types)]
pub type PFN_glPixelTransformParameteriEXT = unsafe extern "system" fn(
    _target: PixelTransformTargetEXT,
    _pname: PixelTransformPNameEXT,
    _param: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glBlendFuncSeparateIndexedAMD = unsafe extern "system" fn(
    _buf: GLuint,
    _srcRGB: BlendingFactor,
    _dstRGB: BlendingFactor,
    _srcAlpha: BlendingFactor,
    _dstAlpha: BlendingFactor,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexEnviEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureEnvTarget,
    _pname: TextureEnvParameter,
    _param: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glNamedFramebufferTextureFaceEXT = unsafe extern "system" fn(
    _framebuffer: GLuint,
    _attachment: FramebufferAttachment,
    _texture: GLuint,
    _level: GLint,
    _face: TextureTarget,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribP2uiv = unsafe extern "system" fn(
    _index: GLuint,
    _type: VertexAttribPointerType,
    _normalized: GLboolean,
    _value: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribs2dvNV =
    unsafe extern "system" fn(_index: GLuint, _count: GLsizei, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glPixelTransferxOES =
    unsafe extern "system" fn(_pname: PixelTransferParameter, _param: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos2svMESA = unsafe extern "system" fn(_v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glGetPointerv =
    unsafe extern "system" fn(_pname: GetPointervPName, _params: *mut *mut std::os::raw::c_void);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteQueriesARB = unsafe extern "system" fn(_n: GLsizei, _ids: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glPushClientAttribDefaultEXT = unsafe extern "system" fn(_mask: ClientAttribMask);
#[allow(non_camel_case_types)]
pub type PFN_glGetUniformfv =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _params: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glLoadIdentityDeformationMapSGIX = unsafe extern "system" fn(_mask: FfdMaskSGIX);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribP3ui = unsafe extern "system" fn(
    _index: GLuint,
    _type: VertexAttribPointerType,
    _normalized: GLboolean,
    _value: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glBeginVertexShaderEXT = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glBlendFuncSeparatei = unsafe extern "system" fn(
    _buf: GLuint,
    _srcRGB: BlendingFactor,
    _dstRGB: BlendingFactor,
    _srcAlpha: BlendingFactor,
    _dstAlpha: BlendingFactor,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord2fv =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glBlendFuncIndexedAMD =
    unsafe extern "system" fn(_buf: GLuint, _src: GLenum, _dst: GLenum);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord2fColor4ubVertex3fSUN = unsafe extern "system" fn(
    _s: GLfloat,
    _t: GLfloat,
    _r: GLubyte,
    _g: GLubyte,
    _b: GLubyte,
    _a: GLubyte,
    _x: GLfloat,
    _y: GLfloat,
    _z: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream1ivATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _coords: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glUniform1ivARB =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glLoadIdentity = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform4i64NV = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _x: GLint64EXT,
    _y: GLint64EXT,
    _z: GLint64EXT,
    _w: GLint64EXT,
);
#[allow(non_camel_case_types)]
pub type PFN_glCombinerParameterfvNV =
    unsafe extern "system" fn(_pname: CombinerParameterNV, _params: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord4fv = unsafe extern "system" fn(_v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib3dv = unsafe extern "system" fn(_index: GLuint, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteBuffersARB = unsafe extern "system" fn(_n: GLsizei, _buffers: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexArrayTexCoordOffsetEXT = unsafe extern "system" fn(
    _vaobj: GLuint,
    _buffer: GLuint,
    _size: GLint,
    _type: TexCoordPointerType,
    _stride: GLsizei,
    _offset: GLintptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glDrawRangeElements = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _start: GLuint,
    _end: GLuint,
    _count: GLsizei,
    _type: DrawElementsType,
    _indices: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glColor3uiv = unsafe extern "system" fn(_v: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glClearDepthdNV = unsafe extern "system" fn(_depth: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glGetMultiTexEnvivEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureEnvTarget,
    _pname: TextureEnvParameter,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform1dv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribs1dvNV =
    unsafe extern "system" fn(_index: GLuint, _count: GLsizei, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos2dv = unsafe extern "system" fn(_v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glNamedProgramLocalParameterI4iEXT = unsafe extern "system" fn(
    _program: GLuint,
    _target: ProgramTarget,
    _index: GLuint,
    _x: GLint,
    _y: GLint,
    _z: GLint,
    _w: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glAcquireKeyedMutexWin32EXT =
    unsafe extern "system" fn(_memory: GLuint, _key: GLuint64, _timeout: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glBindRenderbuffer =
    unsafe extern "system" fn(_target: RenderbufferTarget, _renderbuffer: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glCompressedTextureSubImage1DEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _level: GLint,
    _xoffset: GLint,
    _width: GLsizei,
    _format: InternalFormat,
    _imageSize: GLsizei,
    _bits: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform3ivEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glNamedBufferSubData = unsafe extern "system" fn(
    _buffer: GLuint,
    _offset: GLintptr,
    _size: GLsizeiptr,
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetAttachedShaders = unsafe extern "system" fn(
    _program: GLuint,
    _maxCount: GLsizei,
    _count: *mut GLsizei,
    _shaders: *mut GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramResourceName = unsafe extern "system" fn(
    _program: GLuint,
    _programInterface: ProgramInterface,
    _index: GLuint,
    _bufSize: GLsizei,
    _length: *mut GLsizei,
    _name: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glIsShader = unsafe extern "system" fn(_shader: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glWeightbvARB = unsafe extern "system" fn(_size: GLint, _weights: *const GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glBlitFramebufferANGLE = unsafe extern "system" fn(
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
);
#[allow(non_camel_case_types)]
pub type PFN_glGetInfoLogARB = unsafe extern "system" fn(
    _obj: GLhandleARB,
    _maxLength: GLsizei,
    _length: *mut GLsizei,
    _infoLog: *mut GLcharARB,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetInteger64vEXT = unsafe extern "system" fn(_pname: GetPName, _data: *mut GLint64);
#[allow(non_camel_case_types)]
pub type PFN_glAsyncMarkerSGIX = unsafe extern "system" fn(_marker: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glCreateProgram = unsafe extern "system" fn() -> GLuint;
#[allow(non_camel_case_types)]
pub type PFN_glPathParameterfNV =
    unsafe extern "system" fn(_path: GLuint, _pname: PathParameter, _value: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord2dv = unsafe extern "system" fn(_v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib1svNV = unsafe extern "system" fn(_index: GLuint, _v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos2i = unsafe extern "system" fn(_x: GLint, _y: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glClearDepth = unsafe extern "system" fn(_depth: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribs4hvNV =
    unsafe extern "system" fn(_index: GLuint, _n: GLsizei, _v: *const GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix3fv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glVideoCaptureStreamParameterivNV = unsafe extern "system" fn(
    _video_capture_slot: GLuint,
    _stream: GLuint,
    _pname: GLenum,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos3sMESA = unsafe extern "system" fn(_x: GLshort, _y: GLshort, _z: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glBeginTransformFeedbackNV = unsafe extern "system" fn(_primitiveMode: PrimitiveType);
#[allow(non_camel_case_types)]
pub type PFN_glFragmentMaterialiSGIX =
    unsafe extern "system" fn(_face: MaterialFace, _pname: MaterialParameter, _param: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glInvalidateTexSubImage = unsafe extern "system" fn(
    _texture: GLuint,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _zoffset: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glEvalCoord2xOES = unsafe extern "system" fn(_u: GLfixed, _v: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glColor3sv = unsafe extern "system" fn(_v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glFinalCombinerInputNV = unsafe extern "system" fn(
    _variable: CombinerVariableNV,
    _input: CombinerRegisterNV,
    _mapping: CombinerMappingNV,
    _componentUsage: CombinerComponentUsageNV,
);
#[allow(non_camel_case_types)]
pub type PFN_glDisableiEXT = unsafe extern "system" fn(_target: EnableCap, _index: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetNamedRenderbufferParameteriv = unsafe extern "system" fn(
    _renderbuffer: GLuint,
    _pname: RenderbufferParameterName,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3f =
    unsafe extern "system" fn(_red: GLfloat, _green: GLfloat, _blue: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glListParameterfvSGIX =
    unsafe extern "system" fn(_list: GLuint, _pname: ListParameterName, _params: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetVariantBooleanvEXT =
    unsafe extern "system" fn(_id: GLuint, _value: GetVariantValueEXT, _data: *mut GLboolean);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferTexture2DOES = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _attachment: FramebufferAttachment,
    _textarget: TextureTarget,
    _texture: GLuint,
    _level: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glColorTableParameteriv = unsafe extern "system" fn(
    _target: ColorTableTarget,
    _pname: ColorTableParameterPNameSGI,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetPointervEXT =
    unsafe extern "system" fn(_pname: GetPointervPName, _params: *mut *mut std::os::raw::c_void);
#[allow(non_camel_case_types)]
pub type PFN_glSampleMaskEXT = unsafe extern "system" fn(_value: GLclampf, _invert: GLboolean);
#[allow(non_camel_case_types)]
pub type PFN_glDrawTexfvOES = unsafe extern "system" fn(_coords: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glDrawTexxvOES = unsafe extern "system" fn(_coords: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glSetFenceAPPLE = unsafe extern "system" fn(_fence: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glTexParameterIuiv = unsafe extern "system" fn(
    _target: TextureTarget,
    _pname: TextureParameterName,
    _params: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glEnableVertexAttribAPPLE = unsafe extern "system" fn(_index: GLuint, _pname: GLenum);
#[allow(non_camel_case_types)]
pub type PFN_glGenRenderbuffersEXT =
    unsafe extern "system" fn(_n: GLsizei, _renderbuffers: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform4ui64NV = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _x: GLuint64EXT,
    _y: GLuint64EXT,
    _z: GLuint64EXT,
    _w: GLuint64EXT,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord3f = unsafe extern "system" fn(_s: GLfloat, _t: GLfloat, _r: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glTexStorageMem3DMultisampleEXT = unsafe extern "system" fn(
    _target: TextureTarget,
    _samples: GLsizei,
    _internalFormat: SizedInternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _fixedSampleLocations: GLboolean,
    _memory: GLuint,
    _offset: GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glTestObjectAPPLE =
    unsafe extern "system" fn(_object: ObjectTypeAPPLE, _name: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL2dvEXT = unsafe extern "system" fn(_index: GLuint, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glViewport =
    unsafe extern "system" fn(_x: GLint, _y: GLint, _width: GLsizei, _height: GLsizei);
#[allow(non_camel_case_types)]
pub type PFN_glGetPerfMonitorCounterDataAMD = unsafe extern "system" fn(
    _monitor: GLuint,
    _pname: GLenum,
    _dataSize: GLsizei,
    _data: *mut GLuint,
    _bytesWritten: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glIsVertexArrayOES = unsafe extern "system" fn(_array: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos3sv = unsafe extern "system" fn(_v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glCheckFramebufferStatusEXT =
    unsafe extern "system" fn(_target: FramebufferTarget) -> GLenum;
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexParameterfEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _pname: TextureParameterName,
    _param: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniformMatrix3x2fvNV = unsafe extern "system" fn(
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glCopyTexImage1DEXT = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _internalformat: InternalFormat,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
    _border: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetColorTableSGI = unsafe extern "system" fn(
    _target: ColorTableTargetSGI,
    _format: PixelFormat,
    _type: PixelType,
    _table: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetFinalCombinerInputParameterfvNV = unsafe extern "system" fn(
    _variable: CombinerVariableNV,
    _pname: CombinerParameterNV,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glObjectLabelKHR = unsafe extern "system" fn(
    _identifier: ObjectIdentifier,
    _name: GLuint,
    _length: GLsizei,
    _label: *const GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos2xOES = unsafe extern "system" fn(_x: GLfixed, _y: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramLocalParameterIuivNV =
    unsafe extern "system" fn(_target: ProgramTarget, _index: GLuint, _params: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiDrawArraysIndirectCount = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _indirect: *const std::os::raw::c_void,
    _drawcount: GLintptr,
    _maxdrawcount: GLsizei,
    _stride: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glEnableDriverControlQCOM = unsafe extern "system" fn(_driverControl: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glNamedFramebufferTexture = unsafe extern "system" fn(
    _framebuffer: GLuint,
    _attachment: FramebufferAttachment,
    _texture: GLuint,
    _level: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform3fEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _v0: GLfloat,
    _v1: GLfloat,
    _v2: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glDeformationMap3dSGIX = unsafe extern "system" fn(
    _target: FfdTargetSGIX,
    _u1: GLdouble,
    _u2: GLdouble,
    _ustride: GLint,
    _uorder: GLint,
    _v1: GLdouble,
    _v2: GLdouble,
    _vstride: GLint,
    _vorder: GLint,
    _w1: GLdouble,
    _w2: GLdouble,
    _wstride: GLint,
    _worder: GLint,
    _points: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform2uiv =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glSamplerParameteriv =
    unsafe extern "system" fn(_sampler: GLuint, _pname: SamplerParameterI, _param: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoordP3uiv = unsafe extern "system" fn(
    _texture: TextureUnit,
    _type: TexCoordPointerType,
    _coords: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniformMatrix2x3dv = unsafe extern "system" fn(
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glEvalPoint2 = unsafe extern "system" fn(_i: GLint, _j: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGenSemaphoresEXT = unsafe extern "system" fn(_n: GLsizei, _semaphores: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetPathTexGenfvNV =
    unsafe extern "system" fn(_texCoordSet: TextureUnit, _pname: PathGenMode, _value: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexAttribfvNV =
    unsafe extern "system" fn(_index: GLuint, _pname: VertexAttribEnumNV, _params: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glNormal3x = unsafe extern "system" fn(_nx: GLfixed, _ny: GLfixed, _nz: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glUniform2uiEXT =
    unsafe extern "system" fn(_location: GLint, _v0: GLuint, _v1: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glFlushMappedNamedBufferRange =
    unsafe extern "system" fn(_buffer: GLuint, _offset: GLintptr, _length: GLsizeiptr);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform3ui64ARB = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _x: GLuint64,
    _y: GLuint64,
    _z: GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glCopyTexSubImage3DEXT = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _zoffset: GLint,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glImportMemoryFdEXT = unsafe extern "system" fn(
    _memory: GLuint,
    _size: GLuint64,
    _handleType: ExternalHandleType,
    _fd: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos3fv = unsafe extern "system" fn(_v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetNamedFramebufferAttachmentParameterivEXT = unsafe extern "system" fn(
    _framebuffer: GLuint,
    _attachment: FramebufferAttachment,
    _pname: FramebufferAttachmentParameterName,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetObjectPtrLabel = unsafe extern "system" fn(
    _ptr: *const std::os::raw::c_void,
    _bufSize: GLsizei,
    _length: *mut GLsizei,
    _label: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glImageTransformParameterivHP = unsafe extern "system" fn(
    _target: ImageTransformTargetHP,
    _pname: ImageTransformPNameHP,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glPrimitiveBoundingBoxEXT = unsafe extern "system" fn(
    _minX: GLfloat,
    _minY: GLfloat,
    _minZ: GLfloat,
    _minW: GLfloat,
    _maxX: GLfloat,
    _maxY: GLfloat,
    _maxZ: GLfloat,
    _maxW: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glCopyColorTable = unsafe extern "system" fn(
    _target: ColorTableTarget,
    _internalformat: InternalFormat,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetFragmentLightfvSGIX = unsafe extern "system" fn(
    _light: FragmentLightNameSGIX,
    _pname: FragmentLightParameterSGIX,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform3i64ARB = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _x: GLint64,
    _y: GLint64,
    _z: GLint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform4f = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _v0: GLfloat,
    _v1: GLfloat,
    _v2: GLfloat,
    _v3: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexSubImage2D = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _format: PixelFormat,
    _type: PixelType,
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureStorage3DMultisample = unsafe extern "system" fn(
    _texture: GLuint,
    _samples: GLsizei,
    _internalformat: SizedInternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _fixedsamplelocations: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertex2xvOES = unsafe extern "system" fn(_coords: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream3ivATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _coords: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glCopyConvolutionFilter1DEXT = unsafe extern "system" fn(
    _target: ConvolutionTargetEXT,
    _internalformat: InternalFormat,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetNamedProgramLocalParameterfvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _target: ProgramTarget,
    _index: GLuint,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform4ui64vARB =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glVertexArrayRangeAPPLE =
    unsafe extern "system" fn(_length: GLsizei, _pointer: *mut std::os::raw::c_void);
#[allow(non_camel_case_types)]
pub type PFN_glBlitFramebuffer = unsafe extern "system" fn(
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
);
#[allow(non_camel_case_types)]
pub type PFN_glTexGenf = unsafe extern "system" fn(
    _coord: TextureCoordName,
    _pname: TextureGenParameter,
    _param: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glViewportIndexedfvOES = unsafe extern "system" fn(_index: GLuint, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribs1svNV =
    unsafe extern "system" fn(_index: GLuint, _count: GLsizei, _v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glGetNamedBufferSubData = unsafe extern "system" fn(
    _buffer: GLuint,
    _offset: GLintptr,
    _size: GLsizeiptr,
    _data: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glStencilOpSeparate = unsafe extern "system" fn(
    _face: StencilFaceDirection,
    _sfail: StencilOp,
    _dpfail: StencilOp,
    _dppass: StencilOp,
);
#[allow(non_camel_case_types)]
pub type PFN_glSetLocalConstantEXT =
    unsafe extern "system" fn(_id: GLuint, _type: ScalarType, _addr: *const std::os::raw::c_void);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteVertexArrays = unsafe extern "system" fn(_n: GLsizei, _arrays: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMulticastBarrierNV = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glGetnColorTable = unsafe extern "system" fn(
    _target: ColorTableTarget,
    _format: PixelFormat,
    _type: PixelType,
    _bufSize: GLsizei,
    _table: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI2iEXT = unsafe extern "system" fn(_index: GLuint, _x: GLint, _y: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glMulticastGetQueryObjectivNV =
    unsafe extern "system" fn(_gpu: GLuint, _id: GLuint, _pname: GLenum, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glQueryResourceTagNV =
    unsafe extern "system" fn(_tagId: GLint, _tagString: *const GLchar);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3ub =
    unsafe extern "system" fn(_red: GLubyte, _green: GLubyte, _blue: GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glMakeImageHandleNonResidentARB = unsafe extern "system" fn(_handle: GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexArrayIndexed64iv = unsafe extern "system" fn(
    _vaobj: GLuint,
    _index: GLuint,
    _pname: VertexArrayPName,
    _param: *mut GLint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureRangeAPPLE = unsafe extern "system" fn(
    _target: GLenum,
    _length: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertex4xOES = unsafe extern "system" fn(_x: GLfixed, _y: GLfixed, _z: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glSampleMaski = unsafe extern "system" fn(_maskNumber: GLuint, _mask: GLbitfield);
#[allow(non_camel_case_types)]
pub type PFN_glFogCoordf = unsafe extern "system" fn(_coord: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glIsNamedStringARB =
    unsafe extern "system" fn(_namelen: GLint, _name: *const GLchar) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glSpriteParameterfSGIX =
    unsafe extern "system" fn(_pname: SpriteParameterNameSGIX, _param: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetnMinmax = unsafe extern "system" fn(
    _target: MinmaxTarget,
    _reset: GLboolean,
    _format: PixelFormat,
    _type: PixelType,
    _bufSize: GLsizei,
    _values: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetPixelMapuiv = unsafe extern "system" fn(_map: PixelMap, _values: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glTexBuffer = unsafe extern "system" fn(
    _target: TextureTarget,
    _internalformat: SizedInternalFormat,
    _buffer: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetPerfMonitorCounterInfoAMD = unsafe extern "system" fn(
    _group: GLuint,
    _counter: GLuint,
    _pname: GLenum,
    _data: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteProgramPipelinesEXT =
    unsafe extern "system" fn(_n: GLsizei, _pipelines: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMinmax = unsafe extern "system" fn(
    _target: MinmaxTargetEXT,
    _internalformat: InternalFormat,
    _sink: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureSubImage1DEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _level: GLint,
    _xoffset: GLint,
    _width: GLsizei,
    _format: PixelFormat,
    _type: PixelType,
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL3ui64NV =
    unsafe extern "system" fn(_index: GLuint, _x: GLuint64EXT, _y: GLuint64EXT, _z: GLuint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoordP4uiv = unsafe extern "system" fn(
    _texture: TextureUnit,
    _type: TexCoordPointerType,
    _coords: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glEdgeFlagPointer =
    unsafe extern "system" fn(_stride: GLsizei, _pointer: *const std::os::raw::c_void);
#[allow(non_camel_case_types)]
pub type PFN_glGetBufferParameterui64vNV =
    unsafe extern "system" fn(_target: BufferTargetARB, _pname: GLenum, _params: *mut GLuint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glEvalCoord2dv = unsafe extern "system" fn(_u: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexArrayIndexediv = unsafe extern "system" fn(
    _vaobj: GLuint,
    _index: GLuint,
    _pname: VertexArrayPName,
    _param: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexArrayVertexBuffers = unsafe extern "system" fn(
    _vaobj: GLuint,
    _first: GLuint,
    _count: GLsizei,
    _buffers: *const GLuint,
    _offsets: *const GLintptr,
    _strides: *const GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glFogCoordhNV = unsafe extern "system" fn(_fog: GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glBlendBarrierKHR = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glBlendEquationSeparateIndexedAMD = unsafe extern "system" fn(
    _buf: GLuint,
    _modeRGB: BlendEquationModeEXT,
    _modeAlpha: BlendEquationModeEXT,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetMultiTexParameterfvEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _pname: GetTextureParameter,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultMatrixd = unsafe extern "system" fn(_m: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glScissor =
    unsafe extern "system" fn(_x: GLint, _y: GLint, _width: GLsizei, _height: GLsizei);
#[allow(non_camel_case_types)]
pub type PFN_glStencilMask = unsafe extern "system" fn(_mask: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glTexEnvf = unsafe extern "system" fn(
    _target: TextureEnvTarget,
    _pname: TextureEnvParameter,
    _param: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos4iv = unsafe extern "system" fn(_v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glPathCoverDepthFuncNV = unsafe extern "system" fn(_func: DepthFunction);
#[allow(non_camel_case_types)]
pub type PFN_glGetHistogramParameterfv = unsafe extern "system" fn(
    _target: HistogramTargetEXT,
    _pname: GetHistogramParameterPNameEXT,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glDrawTexsvOES = unsafe extern "system" fn(_coords: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glNormalStream3fvATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _coords: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4NubARB =
    unsafe extern "system" fn(_index: GLuint, _x: GLubyte, _y: GLubyte, _z: GLubyte, _w: GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glTexParameterxv = unsafe extern "system" fn(
    _target: TextureTarget,
    _pname: GetTextureParameter,
    _params: *const GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoordP1ui =
    unsafe extern "system" fn(_texture: TextureUnit, _type: TexCoordPointerType, _coords: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetRenderbufferParameterivOES = unsafe extern "system" fn(
    _target: RenderbufferTarget,
    _pname: RenderbufferParameterName,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glNormal3fVertex3fSUN = unsafe extern "system" fn(
    _nx: GLfloat,
    _ny: GLfloat,
    _nz: GLfloat,
    _x: GLfloat,
    _y: GLfloat,
    _z: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix2x4fvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexImage3DEXT = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _border: GLint,
    _format: PixelFormat,
    _type: PixelType,
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glBeginQueryEXT = unsafe extern "system" fn(_target: QueryTarget, _id: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glDrawMeshTasksIndirectNV = unsafe extern "system" fn(_indirect: GLintptr);
#[allow(non_camel_case_types)]
pub type PFN_glBlendFuncSeparateiOES = unsafe extern "system" fn(
    _buf: GLuint,
    _srcRGB: BlendingFactor,
    _dstRGB: BlendingFactor,
    _srcAlpha: BlendingFactor,
    _dstAlpha: BlendingFactor,
);
#[allow(non_camel_case_types)]
pub type PFN_glExtGetProgramBinarySourceQCOM = unsafe extern "system" fn(
    _program: GLuint,
    _shadertype: ShaderType,
    _source: *mut GLchar,
    _length: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glCopyBufferSubData = unsafe extern "system" fn(
    _readTarget: CopyBufferSubDataTarget,
    _writeTarget: CopyBufferSubDataTarget,
    _readOffset: GLintptr,
    _writeOffset: GLintptr,
    _size: GLsizeiptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glFogx = unsafe extern "system" fn(_pname: FogPName, _param: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glGetMinmaxParameterfv = unsafe extern "system" fn(
    _target: MinmaxTargetEXT,
    _pname: GetMinmaxParameterPNameEXT,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetTexGenfvOES = unsafe extern "system" fn(
    _coord: TextureCoordName,
    _pname: TextureGenParameter,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetnUniformfv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _bufSize: GLsizei,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glMulticastFramebufferSampleLocationsfvNV = unsafe extern "system" fn(
    _gpu: GLuint,
    _framebuffer: GLuint,
    _start: GLuint,
    _count: GLsizei,
    _v: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetTextureImage = unsafe extern "system" fn(
    _texture: GLuint,
    _level: GLint,
    _format: PixelFormat,
    _type: PixelType,
    _bufSize: GLsizei,
    _pixels: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord4hNV = unsafe extern "system" fn(
    _target: TextureUnit,
    _s: GLhalfNV,
    _t: GLhalfNV,
    _r: GLhalfNV,
    _q: GLhalfNV,
);
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos4fv = unsafe extern "system" fn(_v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetUniformBlockIndex =
    unsafe extern "system" fn(_program: GLuint, _uniformBlockName: *const GLchar) -> GLuint;
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3dv = unsafe extern "system" fn(_v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glEndQueryEXT = unsafe extern "system" fn(_target: QueryTarget);
#[allow(non_camel_case_types)]
pub type PFN_glGetPerfMonitorGroupsAMD =
    unsafe extern "system" fn(_numGroups: *mut GLint, _groupsSize: GLsizei, _groups: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramivARB = unsafe extern "system" fn(
    _target: ProgramTarget,
    _pname: ProgramPropertyARB,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetTextureHandleNV = unsafe extern "system" fn(_texture: GLuint) -> GLuint64;
#[allow(non_camel_case_types)]
pub type PFN_glLightfv =
    unsafe extern "system" fn(_light: LightName, _pname: LightParameter, _params: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glDebugMessageInsertARB = unsafe extern "system" fn(
    _source: DebugSource,
    _type: DebugType,
    _id: GLuint,
    _severity: DebugSeverity,
    _length: GLsizei,
    _buf: *const GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform3ui = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _v0: GLuint,
    _v1: GLuint,
    _v2: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glSpecializeShaderARB = unsafe extern "system" fn(
    _shader: GLuint,
    _pEntryPoint: *const GLchar,
    _numSpecializationConstants: GLuint,
    _pConstantIndex: *const GLuint,
    _pConstantValue: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexBufferEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _internalformat: InternalFormat,
    _buffer: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glPixelTransformParameterfvEXT = unsafe extern "system" fn(
    _target: PixelTransformTargetEXT,
    _pname: PixelTransformPNameEXT,
    _params: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord3iv = unsafe extern "system" fn(_v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos3dv = unsafe extern "system" fn(_v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glSamplePatternEXT = unsafe extern "system" fn(_pattern: SamplePatternEXT);
#[allow(non_camel_case_types)]
pub type PFN_glTextureAttachMemoryNV =
    unsafe extern "system" fn(_texture: GLuint, _memory: GLuint, _offset: GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glTextureImage3DEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _level: GLint,
    _internalformat: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _border: GLint,
    _format: PixelFormat,
    _type: PixelType,
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glTransformFeedbackBufferRange = unsafe extern "system" fn(
    _xfb: GLuint,
    _index: GLuint,
    _buffer: GLuint,
    _offset: GLintptr,
    _size: GLsizeiptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertex2hvNV = unsafe extern "system" fn(_v: *const GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glVertexArrayNormalOffsetEXT = unsafe extern "system" fn(
    _vaobj: GLuint,
    _buffer: GLuint,
    _type: NormalPointerType,
    _stride: GLsizei,
    _offset: GLintptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetDoubleIndexedvEXT =
    unsafe extern "system" fn(_target: GetPName, _index: GLuint, _data: *mut GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glGetUniformivARB =
    unsafe extern "system" fn(_programObj: GLhandleARB, _location: GLint, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform1i64NV =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _x: GLint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord4i = unsafe extern "system" fn(_s: GLint, _t: GLint, _r: GLint, _q: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexGenfEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _coord: TextureCoordName,
    _pname: TextureGenParameter,
    _param: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4fARB =
    unsafe extern "system" fn(_index: GLuint, _x: GLfloat, _y: GLfloat, _z: GLfloat, _w: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glMemoryObjectParameterivEXT = unsafe extern "system" fn(
    _memoryObject: GLuint,
    _pname: MemoryObjectParameterName,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glDepthRangexOES = unsafe extern "system" fn(_n: GLfixed, _f: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord1i = unsafe extern "system" fn(_target: TextureUnit, _s: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glStencilFuncSeparate = unsafe extern "system" fn(
    _face: StencilFaceDirection,
    _func: StencilFunction,
    _ref: GLint,
    _mask: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribParameteriAMD =
    unsafe extern "system" fn(_index: GLuint, _pname: GLenum, _param: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetMinmaxParameterivEXT = unsafe extern "system" fn(
    _target: MinmaxTargetEXT,
    _pname: GetMinmaxParameterPNameEXT,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream3fvATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _coords: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glNormalP3ui = unsafe extern "system" fn(_type: NormalPointerType, _coords: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glConservativeRasterParameterfNV =
    unsafe extern "system" fn(_pname: GLenum, _value: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glFlushMappedBufferRange =
    unsafe extern "system" fn(_target: BufferTargetARB, _offset: GLintptr, _length: GLsizeiptr);
#[allow(non_camel_case_types)]
pub type PFN_glNormal3dv = unsafe extern "system" fn(_v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixMult3x3fNV =
    unsafe extern "system" fn(_matrixMode: GLenum, _m: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glUniform2i64NV =
    unsafe extern "system" fn(_location: GLint, _x: GLint64EXT, _y: GLint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribLPointer = unsafe extern "system" fn(
    _index: GLuint,
    _size: GLint,
    _type: VertexAttribLType,
    _stride: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glConservativeRasterParameteriNV =
    unsafe extern "system" fn(_pname: GLenum, _param: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetConvolutionParameteriv = unsafe extern "system" fn(
    _target: ConvolutionTarget,
    _pname: ConvolutionParameterEXT,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetLocalConstantBooleanvEXT =
    unsafe extern "system" fn(_id: GLuint, _value: GetVariantValueEXT, _data: *mut GLboolean);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix4x3dvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetTexParameterfv = unsafe extern "system" fn(
    _target: TextureTarget,
    _pname: GetTextureParameter,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glDrawArraysInstancedNV = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _first: GLint,
    _count: GLsizei,
    _primcount: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glDrawElementsInstancedBaseVertexBaseInstanceEXT = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _count: GLsizei,
    _type: DrawElementsType,
    _indices: *const std::os::raw::c_void,
    _instancecount: GLsizei,
    _basevertex: GLint,
    _baseinstance: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glLineWidth = unsafe extern "system" fn(_width: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glProgramEnvParameterI4ivNV =
    unsafe extern "system" fn(_target: ProgramTarget, _index: GLuint, _params: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetActiveAttrib = unsafe extern "system" fn(
    _program: GLuint,
    _index: GLuint,
    _bufSize: GLsizei,
    _length: *mut GLsizei,
    _size: *mut GLint,
    _type: *mut AttributeType,
    _name: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3fvEXT = unsafe extern "system" fn(_v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glDrawElementsInstancedANGLE = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _count: GLsizei,
    _type: PrimitiveType,
    _indices: *const std::os::raw::c_void,
    _primcount: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glEvalCoord1dv = unsafe extern "system" fn(_u: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord1iv = unsafe extern "system" fn(_target: TextureUnit, _v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixTranslatedEXT =
    unsafe extern "system" fn(_mode: MatrixMode, _x: GLdouble, _y: GLdouble, _z: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glArrayObjectATI = unsafe extern "system" fn(
    _array: EnableCap,
    _size: GLint,
    _type: ScalarType,
    _stride: GLsizei,
    _buffer: GLuint,
    _offset: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glMap2xOES = unsafe extern "system" fn(
    _target: MapTarget,
    _u1: GLfixed,
    _u2: GLfixed,
    _ustride: GLint,
    _uorder: GLint,
    _v1: GLfixed,
    _v2: GLfixed,
    _vstride: GLint,
    _vorder: GLint,
    _points: GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glColor3s = unsafe extern "system" fn(_red: GLshort, _green: GLshort, _blue: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform4iv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glRenderbufferStorageMultisampleANGLE = unsafe extern "system" fn(
    _target: RenderbufferTarget,
    _samples: GLsizei,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glRenderbufferStorageMultisampleIMG = unsafe extern "system" fn(
    _target: RenderbufferTarget,
    _samples: GLsizei,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexEnvx = unsafe extern "system" fn(
    _target: TextureEnvTarget,
    _pname: TextureEnvParameter,
    _param: GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetTexParameterPointervAPPLE = unsafe extern "system" fn(
    _target: GLenum,
    _pname: GLenum,
    _params: *mut *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord1hvNV =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glGetLightxv =
    unsafe extern "system" fn(_light: LightName, _pname: LightParameter, _params: *mut GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glGetFragDataLocation =
    unsafe extern "system" fn(_program: GLuint, _name: *const GLchar) -> GLint;
#[allow(non_camel_case_types)]
pub type PFN_glMapGrid1xOES = unsafe extern "system" fn(_n: GLint, _u1: GLfixed, _u2: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glCompressedTextureSubImage2DEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _format: InternalFormat,
    _imageSize: GLsizei,
    _bits: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferRenderbufferOES = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _attachment: FramebufferAttachment,
    _renderbuffertarget: RenderbufferTarget,
    _renderbuffer: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glHistogram = unsafe extern "system" fn(
    _target: HistogramTargetEXT,
    _width: GLsizei,
    _internalformat: InternalFormat,
    _sink: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glConvolutionParameterivEXT = unsafe extern "system" fn(
    _target: ConvolutionTargetEXT,
    _pname: ConvolutionParameterEXT,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glEnableiOES = unsafe extern "system" fn(_target: EnableCap, _index: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferTextureEXT = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _attachment: FramebufferAttachment,
    _texture: GLuint,
    _level: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixFrustumEXT = unsafe extern "system" fn(
    _mode: MatrixMode,
    _left: GLdouble,
    _right: GLdouble,
    _bottom: GLdouble,
    _top: GLdouble,
    _zNear: GLdouble,
    _zFar: GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform2uivEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultMatrixxOES = unsafe extern "system" fn(_m: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord4i =
    unsafe extern "system" fn(_target: TextureUnit, _s: GLint, _t: GLint, _r: GLint, _q: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord2xOES =
    unsafe extern "system" fn(_texture: TextureUnit, _s: GLfixed, _t: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glNamedFramebufferDrawBuffer =
    unsafe extern "system" fn(_framebuffer: GLuint, _buf: ColorBuffer);
#[allow(non_camel_case_types)]
pub type PFN_glFrustum = unsafe extern "system" fn(
    _left: GLdouble,
    _right: GLdouble,
    _bottom: GLdouble,
    _top: GLdouble,
    _zNear: GLdouble,
    _zFar: GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glNormal3iv = unsafe extern "system" fn(_v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos3iv = unsafe extern "system" fn(_v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glReadnPixelsEXT = unsafe extern "system" fn(
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _format: PixelFormat,
    _type: PixelType,
    _bufSize: GLsizei,
    _data: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glRenderbufferStorageMultisampleNV = unsafe extern "system" fn(
    _target: RenderbufferTarget,
    _samples: GLsizei,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetCommandHeaderNV =
    unsafe extern "system" fn(_tokenID: GLenum, _size: GLuint) -> GLuint;
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColorFormatNV =
    unsafe extern "system" fn(_size: GLint, _type: ColorPointerType, _stride: GLsizei);
#[allow(non_camel_case_types)]
pub type PFN_glCompileShaderARB = unsafe extern "system" fn(_shaderObj: GLhandleARB);
#[allow(non_camel_case_types)]
pub type PFN_glGetFenceivNV =
    unsafe extern "system" fn(_fence: GLuint, _pname: FenceParameterNameNV, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glMaterialxOES =
    unsafe extern "system" fn(_face: MaterialFace, _pname: MaterialParameter, _param: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixPushEXT = unsafe extern "system" fn(_mode: MatrixMode);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform4iEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _v0: GLint,
    _v1: GLint,
    _v2: GLint,
    _v3: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetPathCoordsNV = unsafe extern "system" fn(_path: GLuint, _coords: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glTexStorage1DEXT = unsafe extern "system" fn(
    _target: TextureTarget,
    _levels: GLsizei,
    _internalformat: SizedInternalFormat,
    _width: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform2ui64vNV =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLuint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glConvolutionParameteriv = unsafe extern "system" fn(
    _target: ConvolutionTarget,
    _pname: ConvolutionParameterEXT,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramPathFragmentInputGenNV = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _genMode: GLenum,
    _components: GLint,
    _coeffs: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform3i64vNV =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glUniformBlockBinding = unsafe extern "system" fn(
    _program: GLuint,
    _uniformBlockIndex: GLuint,
    _uniformBlockBinding: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glColor3d =
    unsafe extern "system" fn(_red: GLdouble, _green: GLdouble, _blue: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glCheckFramebufferStatusOES =
    unsafe extern "system" fn(_target: FramebufferTarget) -> GLenum;
#[allow(non_camel_case_types)]
pub type PFN_glDetailTexFuncSGIS =
    unsafe extern "system" fn(_target: TextureTarget, _n: GLsizei, _points: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glProgramBinary = unsafe extern "system" fn(
    _program: GLuint,
    _binaryFormat: GLenum,
    _binary: *const std::os::raw::c_void,
    _length: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glDisableClientState = unsafe extern "system" fn(_array: EnableCap);
#[allow(non_camel_case_types)]
pub type PFN_glIsProgramPipeline = unsafe extern "system" fn(_pipeline: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glProgramLocalParameterI4uiNV = unsafe extern "system" fn(
    _target: ProgramTarget,
    _index: GLuint,
    _x: GLuint,
    _y: GLuint,
    _z: GLuint,
    _w: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glActiveShaderProgram = unsafe extern "system" fn(_pipeline: GLuint, _program: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetTexGendv = unsafe extern "system" fn(
    _coord: TextureCoordName,
    _pname: TextureGenParameter,
    _params: *mut GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3i =
    unsafe extern "system" fn(_red: GLint, _green: GLint, _blue: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix3x2dvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramLocalParameter4dvARB =
    unsafe extern "system" fn(_target: ProgramTarget, _index: GLuint, _params: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glTextureBufferRange = unsafe extern "system" fn(
    _texture: GLuint,
    _internalformat: SizedInternalFormat,
    _buffer: GLuint,
    _offset: GLintptr,
    _size: GLsizeiptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glDisableiNV = unsafe extern "system" fn(_target: EnableCap, _index: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetVideoCaptureStreamfvNV = unsafe extern "system" fn(
    _video_capture_slot: GLuint,
    _stream: GLuint,
    _pname: GLenum,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glPathCoordsNV = unsafe extern "system" fn(
    _path: GLuint,
    _numCoords: GLsizei,
    _coordType: PathCoordType,
    _coords: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glValidateProgramPipeline = unsafe extern "system" fn(_pipeline: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI4uivEXT = unsafe extern "system" fn(_index: GLuint, _v: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glTextureImage3DMultisampleNV = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _samples: GLsizei,
    _internalFormat: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _fixedSampleLocations: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glReplacementCodePointerSUN = unsafe extern "system" fn(
    _type: ReplacementCodeTypeSUN,
    _stride: GLsizei,
    _pointer: *mut *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glCopyTexSubImage1D = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _xoffset: GLint,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetMinmaxParameteriv = unsafe extern "system" fn(
    _target: MinmaxTargetEXT,
    _pname: GetMinmaxParameterPNameEXT,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetActiveUniformARB = unsafe extern "system" fn(
    _programObj: GLhandleARB,
    _index: GLuint,
    _maxLength: GLsizei,
    _length: *mut GLsizei,
    _size: *mut GLint,
    _type: *mut UniformType,
    _name: *mut GLcharARB,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetCombinerOutputParameterfvNV = unsafe extern "system" fn(
    _stage: CombinerStageNV,
    _portion: CombinerPortionNV,
    _pname: CombinerParameterNV,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL1d = unsafe extern "system" fn(_index: GLuint, _x: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glWeightuivARB = unsafe extern "system" fn(_size: GLint, _weights: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glUniform3uivEXT =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glTextureImage2DMultisampleNV = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _samples: GLsizei,
    _internalFormat: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _fixedSampleLocations: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetCompressedTexImage = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _img: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glBindBuffer = unsafe extern "system" fn(_target: BufferTargetARB, _buffer: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMaxShaderCompilerThreadsARB = unsafe extern "system" fn(_count: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glNamedFramebufferTexture1DEXT = unsafe extern "system" fn(
    _framebuffer: GLuint,
    _attachment: FramebufferAttachment,
    _textarget: TextureTarget,
    _texture: GLuint,
    _level: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glPixelTexGenParameteriSGIS =
    unsafe extern "system" fn(_pname: PixelTexGenParameterNameSGIS, _param: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glActiveTextureARB = unsafe extern "system" fn(_texture: TextureUnit);
#[allow(non_camel_case_types)]
pub type PFN_glImageTransformParameteriHP = unsafe extern "system" fn(
    _target: ImageTransformTargetHP,
    _pname: ImageTransformPNameHP,
    _param: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform2ivEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord2f =
    unsafe extern "system" fn(_target: TextureUnit, _s: GLfloat, _t: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetFramebufferParameterfvAMD = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _pname: FramebufferAttachmentParameterName,
    _numsamples: GLuint,
    _pixelindex: GLuint,
    _size: GLsizei,
    _values: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertex2i = unsafe extern "system" fn(_x: GLint, _y: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glPointParameterfvARB =
    unsafe extern "system" fn(_pname: PointParameterNameARB, _params: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glNamedBufferDataEXT = unsafe extern "system" fn(
    _buffer: GLuint,
    _size: GLsizeiptr,
    _data: *const std::os::raw::c_void,
    _usage: VertexBufferObjectUsage,
);
#[allow(non_camel_case_types)]
pub type PFN_glColor4ui =
    unsafe extern "system" fn(_red: GLuint, _green: GLuint, _blue: GLuint, _alpha: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL3ui64vNV =
    unsafe extern "system" fn(_index: GLuint, _v: *const GLuint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos2iARB = unsafe extern "system" fn(_x: GLint, _y: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glMapNamedBufferEXT = unsafe extern "system" fn(
    _buffer: GLuint,
    _access: BufferAccessARB,
) -> *mut std::os::raw::c_void;
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexAttribIuivEXT =
    unsafe extern "system" fn(_index: GLuint, _pname: VertexAttribEnum, _params: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glReplacementCodeuiVertex3fvSUN =
    unsafe extern "system" fn(_rc: *const GLuint, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glCombinerParameterfNV =
    unsafe extern "system" fn(_pname: CombinerParameterNV, _param: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glFragmentCoverageColorNV = unsafe extern "system" fn(_color: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glTexSubImage3DEXT = unsafe extern "system" fn(
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
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glEnableClientState = unsafe extern "system" fn(_array: EnableCap);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL1i64NV = unsafe extern "system" fn(_index: GLuint, _x: GLint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribP4uiv = unsafe extern "system" fn(
    _index: GLuint,
    _type: VertexAttribPointerType,
    _normalized: GLboolean,
    _value: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos3dvMESA = unsafe extern "system" fn(_v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform4i64vNV = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLint64EXT,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI1ui = unsafe extern "system" fn(_index: GLuint, _x: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glSubpixelPrecisionBiasNV = unsafe extern "system" fn(_xbits: GLuint, _ybits: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glReplacementCodeuiColor3fVertex3fvSUN =
    unsafe extern "system" fn(_rc: *const GLuint, _c: *const GLfloat, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glIndexd = unsafe extern "system" fn(_c: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferSampleLocationsfvNV = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _start: GLuint,
    _count: GLsizei,
    _v: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL4ui64NV = unsafe extern "system" fn(
    _index: GLuint,
    _x: GLuint64EXT,
    _y: GLuint64EXT,
    _z: GLuint64EXT,
    _w: GLuint64EXT,
);
#[allow(non_camel_case_types)]
pub type PFN_glMaterialiv = unsafe extern "system" fn(
    _face: MaterialFace,
    _pname: MaterialParameter,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glBitmapxOES = unsafe extern "system" fn(
    _width: GLsizei,
    _height: GLsizei,
    _xorig: GLfixed,
    _yorig: GLfixed,
    _xmove: GLfixed,
    _ymove: GLfixed,
    _bitmap: *const GLubyte,
);
#[allow(non_camel_case_types)]
pub type PFN_glPrioritizeTexturesEXT =
    unsafe extern "system" fn(_n: GLsizei, _textures: *const GLuint, _priorities: *const GLclampf);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord2xOES = unsafe extern "system" fn(_s: GLfixed, _t: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glGetCompressedTextureImageEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _lod: GLint,
    _img: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixLoadTranspose3x3fNV =
    unsafe extern "system" fn(_matrixMode: GLenum, _m: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glBeginFragmentShaderATI = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glBindTextureUnitParameterEXT = unsafe extern "system" fn(
    _unit: TextureUnit,
    _value: VertexShaderTextureUnitParameter,
) -> GLuint;
#[allow(non_camel_case_types)]
pub type PFN_glCopyMultiTexSubImage1DEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _level: GLint,
    _xoffset: GLint,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream4sATI = unsafe extern "system" fn(
    _stream: VertexStreamATI,
    _x: GLshort,
    _y: GLshort,
    _z: GLshort,
    _w: GLshort,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexParameterIiv = unsafe extern "system" fn(
    _target: TextureTarget,
    _pname: TextureParameterName,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glCopyColorSubTable = unsafe extern "system" fn(
    _target: ColorTableTarget,
    _start: GLsizei,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform1ui64vNV = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLuint64EXT,
);
#[allow(non_camel_case_types)]
pub type PFN_glBlendFuncSeparateEXT = unsafe extern "system" fn(
    _sfactorRGB: BlendingFactor,
    _dfactorRGB: BlendingFactor,
    _sfactorAlpha: BlendingFactor,
    _dfactorAlpha: BlendingFactor,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetPerfQueryInfoINTEL = unsafe extern "system" fn(
    _queryId: GLuint,
    _queryNameLength: GLuint,
    _queryName: *mut GLchar,
    _dataSize: *mut GLuint,
    _noCounters: *mut GLuint,
    _noInstances: *mut GLuint,
    _capsMask: *mut GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glStencilFunc =
    unsafe extern "system" fn(_func: StencilFunction, _ref: GLint, _mask: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glTangent3dvEXT = unsafe extern "system" fn(_v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glBinormal3dvEXT = unsafe extern "system" fn(_v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glGetCombinerOutputParameterivNV = unsafe extern "system" fn(
    _stage: CombinerStageNV,
    _portion: CombinerPortionNV,
    _pname: CombinerParameterNV,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform2dEXT =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _x: GLdouble, _y: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixMode = unsafe extern "system" fn(_mode: MatrixMode);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramInfoLog = unsafe extern "system" fn(
    _program: GLuint,
    _bufSize: GLsizei,
    _length: *mut GLsizei,
    _infoLog: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glPathFogGenNV = unsafe extern "system" fn(_genMode: PathGenMode);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform4ui64vNV = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLuint64EXT,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetNamedBufferSubDataEXT = unsafe extern "system" fn(
    _buffer: GLuint,
    _offset: GLintptr,
    _size: GLsizeiptr,
    _data: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetSamplerParameteriv =
    unsafe extern "system" fn(_sampler: GLuint, _pname: SamplerParameterI, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glInsertEventMarkerEXT =
    unsafe extern "system" fn(_length: GLsizei, _marker: *const GLchar);
#[allow(non_camel_case_types)]
pub type PFN_glMapParameterfvNV = unsafe extern "system" fn(
    _target: EvalTargetNV,
    _pname: MapParameterNV,
    _params: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform2i64vARB = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glIndexs = unsafe extern "system" fn(_c: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord1iv = unsafe extern "system" fn(_v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord4fColor4fNormal3fVertex4fSUN = unsafe extern "system" fn(
    _s: GLfloat,
    _t: GLfloat,
    _p: GLfloat,
    _q: GLfloat,
    _r: GLfloat,
    _g: GLfloat,
    _b: GLfloat,
    _a: GLfloat,
    _nx: GLfloat,
    _ny: GLfloat,
    _nz: GLfloat,
    _x: GLfloat,
    _y: GLfloat,
    _z: GLfloat,
    _w: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform1ui64NV = unsafe extern "system" fn(_location: GLint, _x: GLuint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI2ui = unsafe extern "system" fn(_index: GLuint, _x: GLuint, _y: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glBindProgramPipelineEXT = unsafe extern "system" fn(_pipeline: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetShaderSource = unsafe extern "system" fn(
    _shader: GLuint,
    _bufSize: GLsizei,
    _length: *mut GLsizei,
    _source: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexAttribLui64vNV =
    unsafe extern "system" fn(_index: GLuint, _pname: VertexAttribEnum, _params: *mut GLuint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glMapVertexAttrib2fAPPLE = unsafe extern "system" fn(
    _index: GLuint,
    _size: GLuint,
    _u1: GLfloat,
    _u2: GLfloat,
    _ustride: GLint,
    _uorder: GLint,
    _v1: GLfloat,
    _v2: GLfloat,
    _vstride: GLint,
    _vorder: GLint,
    _points: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetPathParameterfvNV =
    unsafe extern "system" fn(_path: GLuint, _pname: PathParameter, _value: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glStencilOp =
    unsafe extern "system" fn(_fail: StencilOp, _zfail: StencilOp, _zpass: StencilOp);
#[allow(non_camel_case_types)]
pub type PFN_glGetSamplerParameterIuivOES =
    unsafe extern "system" fn(_sampler: GLuint, _pname: SamplerParameterI, _params: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glAlphaToCoverageDitherControlNV = unsafe extern "system" fn(_mode: GLenum);
#[allow(non_camel_case_types)]
pub type PFN_glBeginConditionalRender =
    unsafe extern "system" fn(_id: GLuint, _mode: ConditionalRenderMode);
#[allow(non_camel_case_types)]
pub type PFN_glVertexArrayVertexAttribIFormatEXT = unsafe extern "system" fn(
    _vaobj: GLuint,
    _attribindex: GLuint,
    _size: GLint,
    _type: VertexAttribIType,
    _relativeoffset: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glCreateShaderObjectARB =
    unsafe extern "system" fn(_shaderType: ShaderType) -> GLhandleARB;
#[allow(non_camel_case_types)]
pub type PFN_glRectsv = unsafe extern "system" fn(_v1: *const GLshort, _v2: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glUniformMatrix4fvARB = unsafe extern "system" fn(
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL1i64vNV =
    unsafe extern "system" fn(_index: GLuint, _v: *const GLint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glGetAttribLocation =
    unsafe extern "system" fn(_program: GLuint, _name: *const GLchar) -> GLint;
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL3dEXT =
    unsafe extern "system" fn(_index: GLuint, _x: GLdouble, _y: GLdouble, _z: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexPointervINTEL = unsafe extern "system" fn(
    _size: GLint,
    _type: VertexPointerType,
    _pointer: *mut *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexGeniEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _coord: TextureCoordName,
    _pname: TextureGenParameter,
    _param: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glInterleavedArrays = unsafe extern "system" fn(
    _format: InterleavedArrayFormat,
    _stride: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureView = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _origtexture: GLuint,
    _internalformat: SizedInternalFormat,
    _minlevel: GLuint,
    _numlevels: GLuint,
    _minlayer: GLuint,
    _numlayers: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glBeginQueryIndexed =
    unsafe extern "system" fn(_target: QueryTarget, _index: GLuint, _id: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glEvalCoord1f = unsafe extern "system" fn(_u: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glPixelMapfv =
    unsafe extern "system" fn(_map: PixelMap, _mapsize: GLsizei, _values: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glPixelMapusv =
    unsafe extern "system" fn(_map: PixelMap, _mapsize: GLsizei, _values: *const GLushort);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos3sv = unsafe extern "system" fn(_v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glGetSharpenTexFuncSGIS =
    unsafe extern "system" fn(_target: TextureTarget, _points: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glMultiDrawElementsBaseVertexEXT = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _count: *const GLsizei,
    _type: DrawElementsType,
    _indices: *const *const std::os::raw::c_void,
    _drawcount: GLsizei,
    _basevertex: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixMultTransposedEXT =
    unsafe extern "system" fn(_mode: MatrixMode, _m: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glNamedFramebufferParameteriEXT = unsafe extern "system" fn(
    _framebuffer: GLuint,
    _pname: FramebufferParameterName,
    _param: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glClipPlanexIMG = unsafe extern "system" fn(_p: ClipPlaneName, _eqn: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord2xvOES =
    unsafe extern "system" fn(_texture: TextureUnit, _coords: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glBlendEquationOES = unsafe extern "system" fn(_mode: BlendEquationModeEXT);
#[allow(non_camel_case_types)]
pub type PFN_glGetStageIndexNV = unsafe extern "system" fn(_shadertype: ShaderType) -> GLushort;
#[allow(non_camel_case_types)]
pub type PFN_glMapGrid2d = unsafe extern "system" fn(
    _un: GLint,
    _u1: GLdouble,
    _u2: GLdouble,
    _vn: GLint,
    _v1: GLdouble,
    _v2: GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glShadingRateImageBarrierNV = unsafe extern "system" fn(_synchronize: GLboolean);
#[allow(non_camel_case_types)]
pub type PFN_glDrawBuffersATI =
    unsafe extern "system" fn(_n: GLsizei, _bufs: *const DrawBufferMode);
#[allow(non_camel_case_types)]
pub type PFN_glAlphaFragmentOp3ATI = unsafe extern "system" fn(
    _op: FragmentOpATI,
    _dst: GLuint,
    _dstMod: GLuint,
    _arg1: GLuint,
    _arg1Rep: GLuint,
    _arg1Mod: GLuint,
    _arg2: GLuint,
    _arg2Rep: GLuint,
    _arg2Mod: GLuint,
    _arg3: GLuint,
    _arg3Rep: GLuint,
    _arg3Mod: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetSamplerParameterIivOES =
    unsafe extern "system" fn(_sampler: GLuint, _pname: SamplerParameterI, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetTexGenivOES = unsafe extern "system" fn(
    _coord: TextureCoordName,
    _pname: TextureGenParameter,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glFinishFenceAPPLE = unsafe extern "system" fn(_fence: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetTransformFeedbackiv =
    unsafe extern "system" fn(_xfb: GLuint, _pname: TransformFeedbackPName, _param: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetActiveUniformBlockiv = unsafe extern "system" fn(
    _program: GLuint,
    _uniformBlockIndex: GLuint,
    _pname: UniformBlockPName,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetnUniformfvARB = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _bufSize: GLsizei,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glPixelTexGenParameterivSGIS =
    unsafe extern "system" fn(_pname: PixelTexGenParameterNameSGIS, _params: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetTextureHandleIMG = unsafe extern "system" fn(_texture: GLuint) -> GLuint64;
#[allow(non_camel_case_types)]
pub type PFN_glRectxvOES = unsafe extern "system" fn(_v1: *const GLfixed, _v2: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3fEXT =
    unsafe extern "system" fn(_red: GLfloat, _green: GLfloat, _blue: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glShaderBinary = unsafe extern "system" fn(
    _count: GLsizei,
    _shaders: *const GLuint,
    _binaryFormat: ShaderBinaryFormat,
    _binary: *const std::os::raw::c_void,
    _length: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniformHandleui64ARB = unsafe extern "system" fn(_location: GLint, _value: GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glVDPAUInitNV = unsafe extern "system" fn(
    _vdpDevice: *const std::os::raw::c_void,
    _getProcAddress: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glImportMemoryWin32NameEXT = unsafe extern "system" fn(
    _memory: GLuint,
    _size: GLuint64,
    _handleType: ExternalHandleType,
    _name: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glCombinerStageParameterfvNV = unsafe extern "system" fn(
    _stage: CombinerStageNV,
    _pname: CombinerParameterNV,
    _params: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetMultiTexGenivEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _coord: TextureCoordName,
    _pname: TextureGenParameter,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glFragmentLightfSGIX = unsafe extern "system" fn(
    _light: FragmentLightNameSGIX,
    _pname: FragmentLightParameterSGIX,
    _param: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetLocalConstantIntegervEXT =
    unsafe extern "system" fn(_id: GLuint, _value: GetVariantValueEXT, _data: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glIsEnabledi =
    unsafe extern "system" fn(_target: EnableCap, _index: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glFinish = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glGetMultiTexLevelParameterfvEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _level: GLint,
    _pname: GetTextureParameter,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetnUniformdvARB = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _bufSize: GLsizei,
    _params: *mut GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glIsProgramARB = unsafe extern "system" fn(_program: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord2iARB =
    unsafe extern "system" fn(_target: TextureUnit, _s: GLint, _t: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform3uiEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _v0: GLuint,
    _v1: GLuint,
    _v2: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glDrawTransformFeedbackStream =
    unsafe extern "system" fn(_mode: PrimitiveType, _id: GLuint, _stream: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glLighti =
    unsafe extern "system" fn(_light: LightName, _pname: LightParameter, _param: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord1hNV = unsafe extern "system" fn(_s: GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glMultiDrawElementsIndirectCountARB = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _type: DrawElementsType,
    _indirect: *const std::os::raw::c_void,
    _drawcount: GLintptr,
    _maxdrawcount: GLsizei,
    _stride: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexImage4DSGIS = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _size4d: GLsizei,
    _border: GLint,
    _format: PixelFormat,
    _type: PixelType,
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib3f =
    unsafe extern "system" fn(_index: GLuint, _x: GLfloat, _y: GLfloat, _z: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexArrayVertexBuffer = unsafe extern "system" fn(
    _vaobj: GLuint,
    _bindingindex: GLuint,
    _buffer: GLuint,
    _offset: GLintptr,
    _stride: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoordPointerEXT = unsafe extern "system" fn(
    _size: GLint,
    _type: TexCoordPointerType,
    _stride: GLsizei,
    _count: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib3fARB =
    unsafe extern "system" fn(_index: GLuint, _x: GLfloat, _y: GLfloat, _z: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos3f = unsafe extern "system" fn(_x: GLfloat, _y: GLfloat, _z: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform4fEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _v0: GLfloat,
    _v1: GLfloat,
    _v2: GLfloat,
    _v3: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribArrayObjectATI = unsafe extern "system" fn(
    _index: GLuint,
    _size: GLint,
    _type: VertexAttribPointerType,
    _normalized: GLboolean,
    _stride: GLsizei,
    _buffer: GLuint,
    _offset: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexP2uiv =
    unsafe extern "system" fn(_type: VertexPointerType, _value: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glPrimitiveRestartIndexNV = unsafe extern "system" fn(_index: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glExtIsProgramBinaryQCOM = unsafe extern "system" fn(_program: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glTextureBufferEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _internalformat: SizedInternalFormat,
    _buffer: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexArrayEdgeFlagOffsetEXT =
    unsafe extern "system" fn(_vaobj: GLuint, _buffer: GLuint, _stride: GLsizei, _offset: GLintptr);
#[allow(non_camel_case_types)]
pub type PFN_glNormal3hvNV = unsafe extern "system" fn(_v: *const GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glGetUniformBufferSizeEXT =
    unsafe extern "system" fn(_program: GLuint, _location: GLint) -> GLint;
#[allow(non_camel_case_types)]
pub type PFN_glLogicOp = unsafe extern "system" fn(_opcode: LogicOp);
#[allow(non_camel_case_types)]
pub type PFN_glObjectUnpurgeableAPPLE =
    unsafe extern "system" fn(_objectType: GLenum, _name: GLuint, _option: GLenum) -> GLenum;
#[allow(non_camel_case_types)]
pub type PFN_glUniformMatrix4x2fv = unsafe extern "system" fn(
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glCreateStatesNV = unsafe extern "system" fn(_n: GLsizei, _states: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform2ui64ARB =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _x: GLuint64, _y: GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glTextureSubImage3DEXT = unsafe extern "system" fn(
    _texture: GLuint,
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
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix2fvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glSelectPerfMonitorCountersAMD = unsafe extern "system" fn(
    _monitor: GLuint,
    _enable: GLboolean,
    _group: GLuint,
    _numCounters: GLint,
    _counterList: *mut GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix4fv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glReplacementCodeuiNormal3fVertex3fvSUN =
    unsafe extern "system" fn(_rc: *const GLuint, _n: *const GLfloat, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glTexParameterfv = unsafe extern "system" fn(
    _target: TextureTarget,
    _pname: TextureParameterName,
    _params: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertex2s = unsafe extern "system" fn(_x: GLshort, _y: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream1iATI = unsafe extern "system" fn(_stream: VertexStreamATI, _x: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVideoCaptureNV = unsafe extern "system" fn(
    _video_capture_slot: GLuint,
    _sequence_num: *mut GLuint,
    _capture_time: *mut GLuint64EXT,
) -> GLenum;
#[allow(non_camel_case_types)]
pub type PFN_glUniform3ui64NV =
    unsafe extern "system" fn(_location: GLint, _x: GLuint64EXT, _y: GLuint64EXT, _z: GLuint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI4ubv = unsafe extern "system" fn(_index: GLuint, _v: *const GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glDeletePathsNV = unsafe extern "system" fn(_path: GLuint, _range: GLsizei);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream4dvATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _coords: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexParameterfvEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _pname: TextureParameterName,
    _params: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glTranslatexOES = unsafe extern "system" fn(_x: GLfixed, _y: GLfixed, _z: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glUniformHandleui64vNV =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4Nubv = unsafe extern "system" fn(_index: GLuint, _v: *const GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glMapParameterivNV =
    unsafe extern "system" fn(_target: EvalTargetNV, _pname: MapParameterNV, _params: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream4fATI = unsafe extern "system" fn(
    _stream: VertexStreamATI,
    _x: GLfloat,
    _y: GLfloat,
    _z: GLfloat,
    _w: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetQueryObjecti64v =
    unsafe extern "system" fn(_id: GLuint, _pname: QueryObjectParameterName, _params: *mut GLint64);
#[allow(non_camel_case_types)]
pub type PFN_glPathGlyphIndexArrayNV = unsafe extern "system" fn(
    _firstPathName: GLuint,
    _fontTarget: GLenum,
    _fontName: *const std::os::raw::c_void,
    _fontStyle: PathFontStyle,
    _firstGlyphIndex: GLuint,
    _numGlyphs: GLsizei,
    _pathParameterTemplate: GLuint,
    _emScale: GLfloat,
) -> GLenum;
#[allow(non_camel_case_types)]
pub type PFN_glWeightPointerOES = unsafe extern "system" fn(
    _size: GLint,
    _type: GLenum,
    _stride: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glSemaphoreParameterivNV = unsafe extern "system" fn(
    _semaphore: GLuint,
    _pname: SemaphoreParameterName,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glPopName = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glBeginTransformFeedbackEXT = unsafe extern "system" fn(_primitiveMode: PrimitiveType);
#[allow(non_camel_case_types)]
pub type PFN_glEvaluateDepthValuesARB = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glGetIntegeri_vEXT =
    unsafe extern "system" fn(_target: GetPName, _index: GLuint, _data: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glAreTexturesResident = unsafe extern "system" fn(
    _n: GLsizei,
    _textures: *const GLuint,
    _residences: *mut GLboolean,
) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glNamedBufferAttachMemoryNV =
    unsafe extern "system" fn(_buffer: GLuint, _memory: GLuint, _offset: GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glMapNamedBufferRangeEXT = unsafe extern "system" fn(
    _buffer: GLuint,
    _offset: GLintptr,
    _length: GLsizeiptr,
    _access: MapBufferAccessMask,
) -> *mut std::os::raw::c_void;
#[allow(non_camel_case_types)]
pub type PFN_glProgramBinaryOES = unsafe extern "system" fn(
    _program: GLuint,
    _binaryFormat: GLenum,
    _binary: *const std::os::raw::c_void,
    _length: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib3dvNV = unsafe extern "system" fn(_index: GLuint, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glBindFramebufferOES =
    unsafe extern "system" fn(_target: FramebufferTarget, _framebuffer: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glCompressedTexImage1DARB = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _border: GLint,
    _imageSize: GLsizei,
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glNamedFramebufferDrawBuffers =
    unsafe extern "system" fn(_framebuffer: GLuint, _n: GLsizei, _bufs: *const ColorBuffer);
#[allow(non_camel_case_types)]
pub type PFN_glUniform1iARB = unsafe extern "system" fn(_location: GLint, _v0: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glUniformMatrix4x3dv = unsafe extern "system" fn(
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4iv = unsafe extern "system" fn(_index: GLuint, _v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glPopGroupMarkerEXT = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glDrawArraysInstancedBaseInstanceEXT = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _first: GLint,
    _count: GLsizei,
    _instancecount: GLsizei,
    _baseinstance: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetTexGenfv = unsafe extern "system" fn(
    _coord: TextureCoordName,
    _pname: TextureGenParameter,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixPopEXT = unsafe extern "system" fn(_mode: MatrixMode);
#[allow(non_camel_case_types)]
pub type PFN_glBlendEquationi =
    unsafe extern "system" fn(_buf: GLuint, _mode: BlendEquationModeEXT);
#[allow(non_camel_case_types)]
pub type PFN_glFogCoordFormatNV = unsafe extern "system" fn(_type: GLenum, _stride: GLsizei);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteObjectARB = unsafe extern "system" fn(_obj: GLhandleARB);
#[allow(non_camel_case_types)]
pub type PFN_glGetFramebufferAttachmentParameteriv = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _attachment: FramebufferAttachment,
    _pname: FramebufferAttachmentParameterName,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetVariantFloatvEXT =
    unsafe extern "system" fn(_id: GLuint, _value: GetVariantValueEXT, _data: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glIndexsv = unsafe extern "system" fn(_c: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord3bOES =
    unsafe extern "system" fn(_texture: TextureUnit, _s: GLbyte, _t: GLbyte, _r: GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord4iv = unsafe extern "system" fn(_target: TextureUnit, _v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glNamedRenderbufferStorageEXT = unsafe extern "system" fn(
    _renderbuffer: GLuint,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix2dv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord2bOES = unsafe extern "system" fn(_s: GLbyte, _t: GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glUnmapBufferARB = unsafe extern "system" fn(_target: BufferTargetARB) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos3s = unsafe extern "system" fn(_x: GLshort, _y: GLshort, _z: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribIFormat = unsafe extern "system" fn(
    _attribindex: GLuint,
    _size: GLint,
    _type: VertexAttribIType,
    _relativeoffset: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetObjectLabel = unsafe extern "system" fn(
    _identifier: ObjectIdentifier,
    _name: GLuint,
    _bufSize: GLsizei,
    _length: *mut GLsizei,
    _label: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glPatchParameteriOES =
    unsafe extern "system" fn(_pname: PatchParameterName, _value: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glWaitSemaphoreui64NVX = unsafe extern "system" fn(
    _waitGpu: GLuint,
    _fenceObjectCount: GLsizei,
    _semaphoreArray: *const GLuint,
    _fenceValueArray: *const GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glIsBufferResidentNV = unsafe extern "system" fn(_target: GLenum) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4ubv = unsafe extern "system" fn(_index: GLuint, _v: *const GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glBindAttribLocation =
    unsafe extern "system" fn(_program: GLuint, _index: GLuint, _name: *const GLchar);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3bv = unsafe extern "system" fn(_v: *const GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glTangentPointerEXT = unsafe extern "system" fn(
    _type: TangentPointerTypeEXT,
    _stride: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord2dARB =
    unsafe extern "system" fn(_target: TextureUnit, _s: GLdouble, _t: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glNamedBufferPageCommitmentARB = unsafe extern "system" fn(
    _buffer: GLuint,
    _offset: GLintptr,
    _size: GLsizeiptr,
    _commit: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glCreateCommandListsNV = unsafe extern "system" fn(_n: GLsizei, _lists: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetFramebufferParameterivEXT = unsafe extern "system" fn(
    _framebuffer: GLuint,
    _pname: GetFramebufferParameter,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetPathDashArrayNV =
    unsafe extern "system" fn(_path: GLuint, _dashArray: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glPauseTransformFeedback = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream3dATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _x: GLdouble, _y: GLdouble, _z: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexArrayVertexAttribLOffsetEXT = unsafe extern "system" fn(
    _vaobj: GLuint,
    _buffer: GLuint,
    _index: GLuint,
    _size: GLint,
    _type: VertexAttribLType,
    _stride: GLsizei,
    _offset: GLintptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glSeparableFilter2DEXT = unsafe extern "system" fn(
    _target: SeparableTargetEXT,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _format: PixelFormat,
    _type: PixelType,
    _row: *const std::os::raw::c_void,
    _column: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4fvNV = unsafe extern "system" fn(_index: GLuint, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glNormalPointerEXT = unsafe extern "system" fn(
    _type: NormalPointerType,
    _stride: GLsizei,
    _count: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexParameterIuivOES = unsafe extern "system" fn(
    _target: TextureTarget,
    _pname: TextureParameterName,
    _params: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform1i64NV = unsafe extern "system" fn(_location: GLint, _x: GLint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4sv = unsafe extern "system" fn(_index: GLuint, _v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glBlendFunc =
    unsafe extern "system" fn(_sfactor: BlendingFactor, _dfactor: BlendingFactor);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferSampleLocationsfvARB = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _start: GLuint,
    _count: GLsizei,
    _v: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glIsPointInStrokePathNV =
    unsafe extern "system" fn(_path: GLuint, _x: GLfloat, _y: GLfloat) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glDepthRangeIndexedfNV =
    unsafe extern "system" fn(_index: GLuint, _n: GLfloat, _f: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glColor4iv = unsafe extern "system" fn(_v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramNamedParameterfvNV = unsafe extern "system" fn(
    _id: GLuint,
    _len: GLsizei,
    _name: *const GLubyte,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glCreateSamplers = unsafe extern "system" fn(_n: GLsizei, _samplers: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribs1fvNV =
    unsafe extern "system" fn(_index: GLuint, _count: GLsizei, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glWeightivARB = unsafe extern "system" fn(_size: GLint, _weights: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetTextureParameterIivEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _pname: GetTextureParameter,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glPathParameterivNV =
    unsafe extern "system" fn(_path: GLuint, _pname: PathParameter, _value: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix2fv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glRequestResidentProgramsNV =
    unsafe extern "system" fn(_n: GLsizei, _programs: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord1s = unsafe extern "system" fn(_target: TextureUnit, _s: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glGetHistogramEXT = unsafe extern "system" fn(
    _target: HistogramTargetEXT,
    _reset: GLboolean,
    _format: PixelFormat,
    _type: PixelType,
    _values: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glPathStencilFuncNV =
    unsafe extern "system" fn(_func: StencilFunction, _ref: GLint, _mask: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glCompressedTextureSubImage2D = unsafe extern "system" fn(
    _texture: GLuint,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _format: InternalFormat,
    _imageSize: GLsizei,
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureFoveationParametersQCOM = unsafe extern "system" fn(
    _texture: GLuint,
    _layer: GLuint,
    _focalPoint: GLuint,
    _focalX: GLfloat,
    _focalY: GLfloat,
    _gainX: GLfloat,
    _gainY: GLfloat,
    _foveaArea: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetShadingRateSampleLocationivNV = unsafe extern "system" fn(
    _rate: GLenum,
    _samples: GLuint,
    _index: GLuint,
    _location: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferTextureMultisampleMultiviewOVR = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _attachment: FramebufferAttachment,
    _texture: GLuint,
    _level: GLint,
    _samples: GLsizei,
    _baseViewIndex: GLint,
    _numViews: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glClearNamedBufferDataEXT = unsafe extern "system" fn(
    _buffer: GLuint,
    _internalformat: SizedInternalFormat,
    _format: PixelFormat,
    _type: PixelType,
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexEnvxvOES = unsafe extern "system" fn(
    _target: TextureEnvTarget,
    _pname: TextureEnvParameter,
    _params: *const GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform4f = unsafe extern "system" fn(
    _location: GLint,
    _v0: GLfloat,
    _v1: GLfloat,
    _v2: GLfloat,
    _v3: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glBlendFunci =
    unsafe extern "system" fn(_buf: GLuint, _src: BlendingFactor, _dst: BlendingFactor);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord2bvOES =
    unsafe extern "system" fn(_texture: TextureUnit, _coords: *const GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4Nuiv = unsafe extern "system" fn(_index: GLuint, _v: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetMultiTexEnvfvEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureEnvTarget,
    _pname: TextureEnvParameter,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexStorage3DMultisample = unsafe extern "system" fn(
    _target: TextureTarget,
    _samples: GLsizei,
    _internalformat: SizedInternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _fixedsamplelocations: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform1iv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glExtGetTexturesQCOM = unsafe extern "system" fn(
    _textures: *mut GLuint,
    _maxTextures: GLint,
    _numTextures: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glCompressedTexSubImage3DARB = unsafe extern "system" fn(
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
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetVideoivNV =
    unsafe extern "system" fn(_video_slot: GLuint, _pname: GLenum, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramParameter4fvNV =
    unsafe extern "system" fn(_target: VertexAttribEnumNV, _index: GLuint, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glProgramParameter4fNV = unsafe extern "system" fn(
    _target: VertexAttribEnumNV,
    _index: GLuint,
    _x: GLfloat,
    _y: GLfloat,
    _z: GLfloat,
    _w: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord4s = unsafe extern "system" fn(
    _target: TextureUnit,
    _s: GLshort,
    _t: GLshort,
    _r: GLshort,
    _q: GLshort,
);
#[allow(non_camel_case_types)]
pub type PFN_glInstrumentsBufferSGIX =
    unsafe extern "system" fn(_size: GLsizei, _buffer: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGlobalAlphaFactoruiSUN = unsafe extern "system" fn(_factor: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform3uivEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramLocalParameter4dARB = unsafe extern "system" fn(
    _target: ProgramTarget,
    _index: GLuint,
    _x: GLdouble,
    _y: GLdouble,
    _z: GLdouble,
    _w: GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix3x2fv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix4x2dv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glPushName = unsafe extern "system" fn(_name: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos4sv = unsafe extern "system" fn(_v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glReleaseShaderCompiler = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glGenFencesNV = unsafe extern "system" fn(_n: GLsizei, _fences: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glFinishTextureSUNX = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glLinkProgram = unsafe extern "system" fn(_program: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixLoadIdentityEXT = unsafe extern "system" fn(_mode: MatrixMode);
#[allow(non_camel_case_types)]
pub type PFN_glReplacementCodeuiTexCoord2fNormal3fVertex3fvSUN = unsafe extern "system" fn(
    _rc: *const GLuint,
    _tc: *const GLfloat,
    _n: *const GLfloat,
    _v: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glBlitFramebufferEXT = unsafe extern "system" fn(
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
);
#[allow(non_camel_case_types)]
pub type PFN_glGetTexParameterIiv = unsafe extern "system" fn(
    _target: TextureTarget,
    _pname: GetTextureParameter,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix4x2fvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramBinaryOES = unsafe extern "system" fn(
    _program: GLuint,
    _bufSize: GLsizei,
    _length: *mut GLsizei,
    _binaryFormat: *mut GLenum,
    _binary: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glIsFenceAPPLE = unsafe extern "system" fn(_fence: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glShaderSourceARB = unsafe extern "system" fn(
    _shaderObj: GLhandleARB,
    _count: GLsizei,
    _string: *mut *const GLcharARB,
    _length: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord2fNormal3fVertex3fvSUN =
    unsafe extern "system" fn(_tc: *const GLfloat, _n: *const GLfloat, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord2i = unsafe extern "system" fn(_s: GLint, _t: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetQueryiv = unsafe extern "system" fn(
    _target: QueryTarget,
    _pname: QueryParameterName,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureParameteriEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _pname: TextureParameterName,
    _param: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureStorageSparseAMD = unsafe extern "system" fn(
    _texture: GLuint,
    _target: GLenum,
    _internalFormat: SizedInternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _layers: GLsizei,
    _flags: TextureStorageMaskAMD,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform1fvARB =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glMultTransposeMatrixf = unsafe extern "system" fn(_m: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glUnmapNamedBuffer = unsafe extern "system" fn(_buffer: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib3hvNV = unsafe extern "system" fn(_index: GLuint, _v: *const GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glGetSamplerParameterIivEXT =
    unsafe extern "system" fn(_sampler: GLuint, _pname: SamplerParameterI, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream1fvATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _coords: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexWeighthNV = unsafe extern "system" fn(_weight: GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform1ui64vARB = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4NsvARB = unsafe extern "system" fn(_index: GLuint, _v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord3fv = unsafe extern "system" fn(_v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord1sv =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glGetBufferSubDataARB = unsafe extern "system" fn(
    _target: BufferTargetARB,
    _offset: GLintptrARB,
    _size: GLsizeiptrARB,
    _data: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glClearDepthx = unsafe extern "system" fn(_depth: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform1i =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _v0: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos2iMESA = unsafe extern "system" fn(_x: GLint, _y: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glTexStorageMem1DEXT = unsafe extern "system" fn(
    _target: TextureTarget,
    _levels: GLsizei,
    _internalFormat: SizedInternalFormat,
    _width: GLsizei,
    _memory: GLuint,
    _offset: GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramLocalParameterIivNV =
    unsafe extern "system" fn(_target: ProgramTarget, _index: GLuint, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferParameteriMESA = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _pname: FramebufferParameterName,
    _param: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetNamedBufferParameterivEXT =
    unsafe extern "system" fn(_buffer: GLuint, _pname: BufferPNameARB, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexArrayAttribLFormat = unsafe extern "system" fn(
    _vaobj: GLuint,
    _attribindex: GLuint,
    _size: GLint,
    _type: VertexAttribLType,
    _relativeoffset: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glColor3bv = unsafe extern "system" fn(_v: *const GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glEndConditionalRenderNVX = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glIndexdv = unsafe extern "system" fn(_c: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoordP2ui =
    unsafe extern "system" fn(_texture: TextureUnit, _type: TexCoordPointerType, _coords: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glShadingRateImagePaletteNV = unsafe extern "system" fn(
    _viewport: GLuint,
    _first: GLuint,
    _count: GLsizei,
    _rates: *const GLenum,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniformui64vNV =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLuint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glPixelMapuiv =
    unsafe extern "system" fn(_map: PixelMap, _mapsize: GLsizei, _values: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glFogf = unsafe extern "system" fn(_pname: FogParameter, _param: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform3dEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _x: GLdouble,
    _y: GLdouble,
    _z: GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glFragmentMaterialfSGIX =
    unsafe extern "system" fn(_face: MaterialFace, _pname: MaterialParameter, _param: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteVertexArraysOES =
    unsafe extern "system" fn(_n: GLsizei, _arrays: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glExecuteProgramNV =
    unsafe extern "system" fn(_target: VertexAttribEnumNV, _id: GLuint, _params: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glFogCoorddvEXT = unsafe extern "system" fn(_coord: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4dvARB = unsafe extern "system" fn(_index: GLuint, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glDispatchComputeGroupSizeARB = unsafe extern "system" fn(
    _num_groups_x: GLuint,
    _num_groups_y: GLuint,
    _num_groups_z: GLuint,
    _group_size_x: GLuint,
    _group_size_y: GLuint,
    _group_size_z: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoordPointer = unsafe extern "system" fn(
    _size: GLint,
    _type: TexCoordPointerType,
    _stride: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glUseProgramStages =
    unsafe extern "system" fn(_pipeline: GLuint, _stages: UseProgramStageMask, _program: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGenFramebuffersOES =
    unsafe extern "system" fn(_n: GLsizei, _framebuffers: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glIsEnablediOES =
    unsafe extern "system" fn(_target: EnableCap, _index: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexEnvfvEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureEnvTarget,
    _pname: TextureEnvParameter,
    _params: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glClear = unsafe extern "system" fn(_mask: ClearBufferMask);
#[allow(non_camel_case_types)]
pub type PFN_glGetnUniformuiv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _bufSize: GLsizei,
    _params: *mut GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glDeformationMap3fSGIX = unsafe extern "system" fn(
    _target: FfdTargetSGIX,
    _u1: GLfloat,
    _u2: GLfloat,
    _ustride: GLint,
    _uorder: GLint,
    _v1: GLfloat,
    _v2: GLfloat,
    _vstride: GLint,
    _vorder: GLint,
    _w1: GLfloat,
    _w2: GLfloat,
    _wstride: GLint,
    _worder: GLint,
    _points: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glMap1d = unsafe extern "system" fn(
    _target: MapTarget,
    _u1: GLdouble,
    _u2: GLdouble,
    _stride: GLint,
    _order: GLint,
    _points: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glGenQueryResourceTagNV = unsafe extern "system" fn(_n: GLsizei, _tagIds: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4hvNV = unsafe extern "system" fn(_index: GLuint, _v: *const GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glPixelTransformParameterfEXT = unsafe extern "system" fn(
    _target: PixelTransformTargetEXT,
    _pname: PixelTransformPNameEXT,
    _param: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI4iv = unsafe extern "system" fn(_index: GLuint, _v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glLGPUNamedBufferSubDataNVX = unsafe extern "system" fn(
    _gpuMask: GLbitfield,
    _buffer: GLuint,
    _offset: GLintptr,
    _size: GLsizeiptr,
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glIsPathNV = unsafe extern "system" fn(_path: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glUniform1ui = unsafe extern "system" fn(_location: GLint, _v0: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGlobalAlphaFactorubSUN = unsafe extern "system" fn(_factor: GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos2sv = unsafe extern "system" fn(_v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glGetObjectParameterivAPPLE = unsafe extern "system" fn(
    _objectType: GLenum,
    _name: GLuint,
    _pname: GLenum,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glRenderbufferStorageMultisample = unsafe extern "system" fn(
    _target: RenderbufferTarget,
    _samples: GLsizei,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glCompressedMultiTexImage2DEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _level: GLint,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _border: GLint,
    _imageSize: GLsizei,
    _bits: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord3sv =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glGetnTexImage = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _format: PixelFormat,
    _type: PixelType,
    _bufSize: GLsizei,
    _pixels: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetColorTableEXT = unsafe extern "system" fn(
    _target: ColorTableTarget,
    _format: PixelFormat,
    _type: PixelType,
    _data: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetPixelMapxv =
    unsafe extern "system" fn(_map: PixelMap, _size: GLint, _values: *mut GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord3hvNV =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glGetTextureLevelParameterfv = unsafe extern "system" fn(
    _texture: GLuint,
    _level: GLint,
    _pname: GetTextureParameter,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetInternalformativ = unsafe extern "system" fn(
    _target: TextureTarget,
    _internalformat: InternalFormat,
    _pname: InternalFormatPName,
    _count: GLsizei,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform4fv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord4d =
    unsafe extern "system" fn(_s: GLdouble, _t: GLdouble, _r: GLdouble, _q: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3ubEXT =
    unsafe extern "system" fn(_red: GLubyte, _green: GLubyte, _blue: GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glUniformHandleui64NV = unsafe extern "system" fn(_location: GLint, _value: GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformui64NV =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _value: GLuint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glUniformSubroutinesuiv =
    unsafe extern "system" fn(_shadertype: ShaderType, _count: GLsizei, _indices: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMulticastWaitSyncNV =
    unsafe extern "system" fn(_signalGpu: GLuint, _waitGpuMask: GLbitfield);
#[allow(non_camel_case_types)]
pub type PFN_glHint = unsafe extern "system" fn(_target: HintTarget, _mode: HintMode);
#[allow(non_camel_case_types)]
pub type PFN_glReplacementCodeubvSUN = unsafe extern "system" fn(_code: *const GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glVertexArrayColorOffsetEXT = unsafe extern "system" fn(
    _vaobj: GLuint,
    _buffer: GLuint,
    _size: GLint,
    _type: ColorPointerType,
    _stride: GLsizei,
    _offset: GLintptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glEGLImageTargetRenderbufferStorageOES =
    unsafe extern "system" fn(_target: GLenum, _image: GLeglImageOES);
#[allow(non_camel_case_types)]
pub type PFN_glColor4fNormal3fVertex3fSUN = unsafe extern "system" fn(
    _r: GLfloat,
    _g: GLfloat,
    _b: GLfloat,
    _a: GLfloat,
    _nx: GLfloat,
    _ny: GLfloat,
    _nz: GLfloat,
    _x: GLfloat,
    _y: GLfloat,
    _z: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGlobalAlphaFactorbSUN = unsafe extern "system" fn(_factor: GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform3fv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertex4s =
    unsafe extern "system" fn(_x: GLshort, _y: GLshort, _z: GLshort, _w: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glUniform1iv =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3ubvEXT = unsafe extern "system" fn(_v: *const GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform1i64vARB = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glClearTexImageEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _level: GLint,
    _format: PixelFormat,
    _type: PixelType,
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3uivEXT = unsafe extern "system" fn(_v: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetGraphicsResetStatusARB = unsafe extern "system" fn() -> GLenum;
#[allow(non_camel_case_types)]
pub type PFN_glBindVideoCaptureStreamTextureNV = unsafe extern "system" fn(
    _video_capture_slot: GLuint,
    _stream: GLuint,
    _frame_region: GLenum,
    _target: GLenum,
    _texture: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetInvariantFloatvEXT =
    unsafe extern "system" fn(_id: GLuint, _value: GetVariantValueEXT, _data: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramResourceLocation = unsafe extern "system" fn(
    _program: GLuint,
    _programInterface: ProgramInterface,
    _name: *const GLchar,
) -> GLint;
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexSubImage2DEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _format: PixelFormat,
    _type: PixelType,
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexParameterIuivEXT = unsafe extern "system" fn(
    _target: TextureTarget,
    _pname: TextureParameterName,
    _params: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform4ivARB =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4fvARB = unsafe extern "system" fn(_index: GLuint, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glViewportIndexedfNV =
    unsafe extern "system" fn(_index: GLuint, _x: GLfloat, _y: GLfloat, _w: GLfloat, _h: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glNormalFormatNV = unsafe extern "system" fn(_type: GLenum, _stride: GLsizei);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord4dv =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glTextureParameterIuiv = unsafe extern "system" fn(
    _texture: GLuint,
    _pname: TextureParameterName,
    _params: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glFlushVertexArrayRangeNV = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glInvalidateNamedFramebufferData = unsafe extern "system" fn(
    _framebuffer: GLuint,
    _numAttachments: GLsizei,
    _attachments: *const FramebufferAttachment,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniformBufferEXT =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _buffer: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glBlendFuncSeparateOES = unsafe extern "system" fn(
    _srcRGB: BlendingFactor,
    _dstRGB: BlendingFactor,
    _srcAlpha: BlendingFactor,
    _dstAlpha: BlendingFactor,
);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferFoveationParametersQCOM = unsafe extern "system" fn(
    _framebuffer: GLuint,
    _layer: GLuint,
    _focalPoint: GLuint,
    _focalX: GLfloat,
    _focalY: GLfloat,
    _gainX: GLfloat,
    _gainY: GLfloat,
    _foveaArea: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glDebugMessageInsertKHR = unsafe extern "system" fn(
    _source: DebugSource,
    _type: DebugType,
    _id: GLuint,
    _severity: DebugSeverity,
    _length: GLsizei,
    _buf: *const GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glApplyTextureEXT = unsafe extern "system" fn(_mode: LightTextureModeEXT);
#[allow(non_camel_case_types)]
pub type PFN_glMultMatrixf = unsafe extern "system" fn(_m: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glCopyTexSubImage1DEXT = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _xoffset: GLint,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteCommandListsNV = unsafe extern "system" fn(_n: GLsizei, _lists: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glPointParameterfv =
    unsafe extern "system" fn(_pname: PointParameterNameARB, _params: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoordP2ui =
    unsafe extern "system" fn(_type: TexCoordPointerType, _coords: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glBegin = unsafe extern "system" fn(_mode: PrimitiveType);
#[allow(non_camel_case_types)]
pub type PFN_glClipPlanexOES =
    unsafe extern "system" fn(_plane: ClipPlaneName, _equation: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glCreateQueries =
    unsafe extern "system" fn(_target: QueryTarget, _n: GLsizei, _ids: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glDrawRangeElementsBaseVertex = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _start: GLuint,
    _end: GLuint,
    _count: GLsizei,
    _type: DrawElementsType,
    _indices: *const std::os::raw::c_void,
    _basevertex: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetUniformuivEXT =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _params: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glInvalidateBufferData = unsafe extern "system" fn(_buffer: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glEvalCoord1fv = unsafe extern "system" fn(_u: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetUniformfvARB =
    unsafe extern "system" fn(_programObj: GLhandleARB, _location: GLint, _params: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glColorTableParameterfvSGI = unsafe extern "system" fn(
    _target: ColorTableTargetSGI,
    _pname: ColorTableParameterPNameSGI,
    _params: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glEdgeFlagPointerListIBM =
    unsafe extern "system" fn(_stride: GLint, _pointer: *mut *const GLboolean, _ptrstride: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetMapAttribParameterfvNV = unsafe extern "system" fn(
    _target: EvalTargetNV,
    _index: GLuint,
    _pname: MapAttribParameterNV,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetImageTransformParameterfvHP = unsafe extern "system" fn(
    _target: ImageTransformTargetHP,
    _pname: ImageTransformPNameHP,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glMakeTextureHandleResidentARB = unsafe extern "system" fn(_handle: GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixMultdEXT = unsafe extern "system" fn(_mode: MatrixMode, _m: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glGenerateTextureMipmap = unsafe extern "system" fn(_texture: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glBindSamplers =
    unsafe extern "system" fn(_first: GLuint, _count: GLsizei, _samplers: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord2i =
    unsafe extern "system" fn(_target: TextureUnit, _s: GLint, _t: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glPolygonOffset = unsafe extern "system" fn(_factor: GLfloat, _units: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glCheckFramebufferStatus =
    unsafe extern "system" fn(_target: FramebufferTarget) -> GLenum;
#[allow(non_camel_case_types)]
pub type PFN_glClearBufferfi = unsafe extern "system" fn(
    _buffer: Buffer,
    _drawbuffer: GLint,
    _depth: GLfloat,
    _stencil: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glDrawRangeElementsBaseVertexEXT = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _start: GLuint,
    _end: GLuint,
    _count: GLsizei,
    _type: DrawElementsType,
    _indices: *const std::os::raw::c_void,
    _basevertex: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetTransformFeedbacki_v = unsafe extern "system" fn(
    _xfb: GLuint,
    _pname: TransformFeedbackPName,
    _index: GLuint,
    _param: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramEnvParameters4fvEXT = unsafe extern "system" fn(
    _target: ProgramTarget,
    _index: GLuint,
    _count: GLsizei,
    _params: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramParameteri =
    unsafe extern "system" fn(_program: GLuint, _pname: ProgramParameterPName, _value: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramStringNV =
    unsafe extern "system" fn(_id: GLuint, _pname: VertexAttribEnumNV, _program: *mut GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform4ui64ARB = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _x: GLuint64,
    _y: GLuint64,
    _z: GLuint64,
    _w: GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix2x3dvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform1d = unsafe extern "system" fn(_location: GLint, _x: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glUniform4dv =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glUnmapBuffer = unsafe extern "system" fn(_target: BufferTargetARB) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glDisableVertexArrayAttrib = unsafe extern "system" fn(_vaobj: GLuint, _index: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertex2f = unsafe extern "system" fn(_x: GLfloat, _y: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexArrayBindVertexBufferEXT = unsafe extern "system" fn(
    _vaobj: GLuint,
    _bindingindex: GLuint,
    _buffer: GLuint,
    _offset: GLintptr,
    _stride: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI2uiv = unsafe extern "system" fn(_index: GLuint, _v: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glDrawElementsInstancedBaseVertexOES = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _count: GLsizei,
    _type: DrawElementsType,
    _indices: *const std::os::raw::c_void,
    _instancecount: GLsizei,
    _basevertex: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexAttribdvNV =
    unsafe extern "system" fn(_index: GLuint, _pname: VertexAttribEnumNV, _params: *mut GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI3uivEXT = unsafe extern "system" fn(_index: GLuint, _v: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetInternalformati64v = unsafe extern "system" fn(
    _target: TextureTarget,
    _internalformat: InternalFormat,
    _pname: InternalFormatPName,
    _count: GLsizei,
    _params: *mut GLint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramPipelineInfoLog = unsafe extern "system" fn(
    _pipeline: GLuint,
    _bufSize: GLsizei,
    _length: *mut GLsizei,
    _infoLog: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetRenderbufferParameterivEXT = unsafe extern "system" fn(
    _target: RenderbufferTarget,
    _pname: RenderbufferParameterName,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureStorageMem1DEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _levels: GLsizei,
    _internalFormat: SizedInternalFormat,
    _width: GLsizei,
    _memory: GLuint,
    _offset: GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glDrawArraysInstancedEXT = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _start: GLint,
    _count: GLsizei,
    _primcount: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramLocalParametersI4uivNV = unsafe extern "system" fn(
    _target: ProgramTarget,
    _index: GLuint,
    _count: GLsizei,
    _params: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL4dv = unsafe extern "system" fn(_index: GLuint, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexP4uiv =
    unsafe extern "system" fn(_type: VertexPointerType, _value: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glWeightfvARB = unsafe extern "system" fn(_size: GLint, _weights: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI1iEXT = unsafe extern "system" fn(_index: GLuint, _x: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glNamedBufferStorageEXT = unsafe extern "system" fn(
    _buffer: GLuint,
    _size: GLsizeiptr,
    _data: *const std::os::raw::c_void,
    _flags: BufferStorageMask,
);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3svEXT = unsafe extern "system" fn(_v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glStencilThenCoverFillPathNV =
    unsafe extern "system" fn(_path: GLuint, _fillMode: GLenum, _mask: GLuint, _coverMode: GLenum);
#[allow(non_camel_case_types)]
pub type PFN_glGetMultiTexParameterIivEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _pname: GetTextureParameter,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glEndConditionalRender = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glIsObjectBufferATI = unsafe extern "system" fn(_buffer: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glProgramLocalParameters4fvEXT = unsafe extern "system" fn(
    _target: ProgramTarget,
    _index: GLuint,
    _count: GLsizei,
    _params: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetConvolutionFilter = unsafe extern "system" fn(
    _target: ConvolutionTarget,
    _format: PixelFormat,
    _type: PixelType,
    _image: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexAttachMemoryNV =
    unsafe extern "system" fn(_target: TextureTarget, _memory: GLuint, _offset: GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glTexGenfvOES = unsafe extern "system" fn(
    _coord: TextureCoordName,
    _pname: TextureGenParameter,
    _params: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glNormal3bv = unsafe extern "system" fn(_v: *const GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4Niv = unsafe extern "system" fn(_index: GLuint, _v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glClampColorARB =
    unsafe extern "system" fn(_target: ClampColorTargetARB, _clamp: ClampColorModeARB);
#[allow(non_camel_case_types)]
pub type PFN_glNamedFramebufferRenderbuffer = unsafe extern "system" fn(
    _framebuffer: GLuint,
    _attachment: FramebufferAttachment,
    _renderbuffertarget: RenderbufferTarget,
    _renderbuffer: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glSampleMaskSGIS = unsafe extern "system" fn(_value: GLclampf, _invert: GLboolean);
#[allow(non_camel_case_types)]
pub type PFN_glEndPerfMonitorAMD = unsafe extern "system" fn(_monitor: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform4uiv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetAttachedObjectsARB = unsafe extern "system" fn(
    _containerObj: GLhandleARB,
    _maxCount: GLsizei,
    _count: *mut GLsizei,
    _obj: *mut GLhandleARB,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribDivisorNV = unsafe extern "system" fn(_index: GLuint, _divisor: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glUniformMatrix3fvARB = unsafe extern "system" fn(
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteProgramsNV = unsafe extern "system" fn(_n: GLsizei, _programs: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform2f =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _v0: GLfloat, _v1: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetSubroutineIndex = unsafe extern "system" fn(
    _program: GLuint,
    _shadertype: ShaderType,
    _name: *const GLchar,
) -> GLuint;
#[allow(non_camel_case_types)]
pub type PFN_glCopyTextureSubImage3DEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _zoffset: GLint,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexStorage2D = unsafe extern "system" fn(
    _target: TextureTarget,
    _levels: GLsizei,
    _internalformat: SizedInternalFormat,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib2sv = unsafe extern "system" fn(_index: GLuint, _v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL4i64NV = unsafe extern "system" fn(
    _index: GLuint,
    _x: GLint64EXT,
    _y: GLint64EXT,
    _z: GLint64EXT,
    _w: GLint64EXT,
);
#[allow(non_camel_case_types)]
pub type PFN_glDebugMessageInsert = unsafe extern "system" fn(
    _source: DebugSource,
    _type: DebugType,
    _id: GLuint,
    _severity: DebugSeverity,
    _length: GLsizei,
    _buf: *const GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glClipPlane =
    unsafe extern "system" fn(_plane: ClipPlaneName, _equation: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord1xvOES = unsafe extern "system" fn(_coords: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glGetTextureParameteriv =
    unsafe extern "system" fn(_texture: GLuint, _pname: GetTextureParameter, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexPointer = unsafe extern "system" fn(
    _size: GLint,
    _type: VertexPointerType,
    _stride: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glColor3dv = unsafe extern "system" fn(_v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glGetPathParameterivNV =
    unsafe extern "system" fn(_path: GLuint, _pname: PathParameter, _value: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetPixelTransformParameterfvEXT =
    unsafe extern "system" fn(_target: GLenum, _pname: GLenum, _params: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glColor3ui = unsafe extern "system" fn(_red: GLuint, _green: GLuint, _blue: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetDoublei_vEXT =
    unsafe extern "system" fn(_pname: GetPName, _index: GLuint, _params: *mut GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glGetSynciv = unsafe extern "system" fn(
    _sync: GLsync,
    _pname: SyncParameterName,
    _count: GLsizei,
    _length: *mut GLsizei,
    _values: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord2fVertex3fSUN =
    unsafe extern "system" fn(_s: GLfloat, _t: GLfloat, _x: GLfloat, _y: GLfloat, _z: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glTexSubImage3DOES = unsafe extern "system" fn(
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
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetTexEnvxv = unsafe extern "system" fn(
    _target: TextureEnvTarget,
    _pname: TextureEnvParameter,
    _params: *mut GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glBufferAddressRangeNV = unsafe extern "system" fn(
    _pname: GLenum,
    _index: GLuint,
    _address: GLuint64EXT,
    _length: GLsizeiptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glBlendEquationiARB =
    unsafe extern "system" fn(_buf: GLuint, _mode: BlendEquationModeEXT);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexGendvEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _coord: TextureCoordName,
    _pname: TextureGenParameter,
    _params: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glColor3i = unsafe extern "system" fn(_red: GLint, _green: GLint, _blue: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribIPointerEXT = unsafe extern "system" fn(
    _index: GLuint,
    _size: GLint,
    _type: VertexAttribIType,
    _stride: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetUniformi64vARB =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _params: *mut GLint64);
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexAttribivNV =
    unsafe extern "system" fn(_index: GLuint, _pname: VertexAttribEnumNV, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetInvariantIntegervEXT =
    unsafe extern "system" fn(_id: GLuint, _value: GetVariantValueEXT, _data: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glIsFenceNV = unsafe extern "system" fn(_fence: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glBufferPageCommitmentARB = unsafe extern "system" fn(
    _target: GLenum,
    _offset: GLintptr,
    _size: GLsizeiptr,
    _commit: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glColor4ub =
    unsafe extern "system" fn(_red: GLubyte, _green: GLubyte, _blue: GLubyte, _alpha: GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glBindRenderbufferOES =
    unsafe extern "system" fn(_target: RenderbufferTarget, _renderbuffer: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glEndVertexShaderEXT = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glColor4x =
    unsafe extern "system" fn(_red: GLfixed, _green: GLfixed, _blue: GLfixed, _alpha: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glCreateBuffers = unsafe extern "system" fn(_n: GLsizei, _buffers: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetMultiTexLevelParameterivEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _level: GLint,
    _pname: GetTextureParameter,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform3ui64NV = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _x: GLuint64EXT,
    _y: GLuint64EXT,
    _z: GLuint64EXT,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord2d =
    unsafe extern "system" fn(_target: TextureUnit, _s: GLdouble, _t: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glAlphaFuncx = unsafe extern "system" fn(_func: AlphaFunction, _ref: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos2dMESA = unsafe extern "system" fn(_x: GLdouble, _y: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramInterfaceiv = unsafe extern "system" fn(
    _program: GLuint,
    _programInterface: ProgramInterface,
    _pname: ProgramInterfacePName,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord3i =
    unsafe extern "system" fn(_target: TextureUnit, _s: GLint, _t: GLint, _r: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3iEXT =
    unsafe extern "system" fn(_red: GLint, _green: GLint, _blue: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertex3hNV = unsafe extern "system" fn(_x: GLhalfNV, _y: GLhalfNV, _z: GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glBlitNamedFramebuffer = unsafe extern "system" fn(
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
);
#[allow(non_camel_case_types)]
pub type PFN_glTexBufferARB = unsafe extern "system" fn(
    _target: TextureTarget,
    _internalformat: SizedInternalFormat,
    _buffer: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glConvolutionFilter2DEXT = unsafe extern "system" fn(
    _target: ConvolutionTargetEXT,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _format: PixelFormat,
    _type: PixelType,
    _image: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniformMatrix4dv = unsafe extern "system" fn(
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glIndexiv = unsafe extern "system" fn(_c: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glLGPUInterlockNVX = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glNamedFramebufferSamplePositionsfvAMD = unsafe extern "system" fn(
    _framebuffer: GLuint,
    _numsamples: GLuint,
    _pixelindex: GLuint,
    _values: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glDrawRangeElementsEXT = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _start: GLuint,
    _end: GLuint,
    _count: GLsizei,
    _type: DrawElementsType,
    _indices: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferParameteri = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _pname: FramebufferParameterName,
    _param: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glIndexPointerEXT = unsafe extern "system" fn(
    _type: IndexPointerType,
    _stride: GLsizei,
    _count: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetPerfQueryIdByNameINTEL =
    unsafe extern "system" fn(_queryName: *mut GLchar, _queryId: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glBindAttribLocationARB =
    unsafe extern "system" fn(_programObj: GLhandleARB, _index: GLuint, _name: *const GLcharARB);
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos3xvOES = unsafe extern "system" fn(_coords: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glCopyColorTableSGI = unsafe extern "system" fn(
    _target: ColorTableTargetSGI,
    _internalformat: InternalFormat,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetQueryivEXT = unsafe extern "system" fn(
    _target: QueryTarget,
    _pname: QueryParameterName,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform1i64vNV =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4NuivARB = unsafe extern "system" fn(_index: GLuint, _v: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glTexBufferRangeEXT = unsafe extern "system" fn(
    _target: TextureTarget,
    _internalformat: SizedInternalFormat,
    _buffer: GLuint,
    _offset: GLintptr,
    _size: GLsizeiptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glDrawRangeElementArrayATI =
    unsafe extern "system" fn(_mode: PrimitiveType, _start: GLuint, _end: GLuint, _count: GLsizei);
#[allow(non_camel_case_types)]
pub type PFN_glGetObjectParameterfvARB =
    unsafe extern "system" fn(_obj: GLhandleARB, _pname: GLenum, _params: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glSampleCoverageARB = unsafe extern "system" fn(_value: GLfloat, _invert: GLboolean);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixIndexubvARB =
    unsafe extern "system" fn(_size: GLint, _indices: *const GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glAsyncCopyBufferSubDataNVX = unsafe extern "system" fn(
    _waitSemaphoreCount: GLsizei,
    _waitSemaphoreArray: *const GLuint,
    _fenceValueArray: *const GLuint64,
    _readGpu: GLuint,
    _writeGpuMask: GLbitfield,
    _readBuffer: GLuint,
    _writeBuffer: GLuint,
    _readOffset: GLintptr,
    _writeOffset: GLintptr,
    _size: GLsizeiptr,
    _signalSemaphoreCount: GLsizei,
    _signalSemaphoreArray: *const GLuint,
    _signalValueArray: *const GLuint64,
) -> GLuint;
#[allow(non_camel_case_types)]
pub type PFN_glNamedProgramLocalParameter4dvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _target: ProgramTarget,
    _index: GLuint,
    _params: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord4ivARB =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glCompressedTexImage2DARB = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _border: GLint,
    _imageSize: GLsizei,
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glQueryCounter = unsafe extern "system" fn(_id: GLuint, _target: QueryCounterTarget);
#[allow(non_camel_case_types)]
pub type PFN_glUniform2f = unsafe extern "system" fn(_location: GLint, _v0: GLfloat, _v1: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glUniformMatrix4x2fvNV = unsafe extern "system" fn(
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetDebugMessageLog = unsafe extern "system" fn(
    _count: GLuint,
    _bufSize: GLsizei,
    _sources: *mut DebugSource,
    _types: *mut DebugType,
    _ids: *mut GLuint,
    _severities: *mut DebugSeverity,
    _lengths: *mut GLsizei,
    _messageLog: *mut GLchar,
) -> GLuint;
#[allow(non_camel_case_types)]
pub type PFN_glGetFinalCombinerInputParameterivNV = unsafe extern "system" fn(
    _variable: CombinerVariableNV,
    _pname: CombinerParameterNV,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glIsVertexArray = unsafe extern "system" fn(_array: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glUniform4ui =
    unsafe extern "system" fn(_location: GLint, _v0: GLuint, _v1: GLuint, _v2: GLuint, _v3: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glCopyTextureSubImage2DEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glTangent3iEXT = unsafe extern "system" fn(_tx: GLint, _ty: GLint, _tz: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glCopyTextureSubImage1D = unsafe extern "system" fn(
    _texture: GLuint,
    _level: GLint,
    _xoffset: GLint,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glActiveProgramEXT = unsafe extern "system" fn(_program: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glFogiv = unsafe extern "system" fn(_pname: FogParameter, _params: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4NivARB = unsafe extern "system" fn(_index: GLuint, _v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetHistogramParameterfvEXT = unsafe extern "system" fn(
    _target: HistogramTargetEXT,
    _pname: GetHistogramParameterPNameEXT,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glIsSemaphoreEXT = unsafe extern "system" fn(_semaphore: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glColor4usv = unsafe extern "system" fn(_v: *const GLushort);
#[allow(non_camel_case_types)]
pub type PFN_glDisableVariantClientStateEXT = unsafe extern "system" fn(_id: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glNamedFramebufferRenderbufferEXT = unsafe extern "system" fn(
    _framebuffer: GLuint,
    _attachment: FramebufferAttachment,
    _renderbuffertarget: RenderbufferTarget,
    _renderbuffer: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetNamedStringivARB = unsafe extern "system" fn(
    _namelen: GLint,
    _name: *const GLchar,
    _pname: GLenum,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL1ui64ARB = unsafe extern "system" fn(_index: GLuint, _x: GLuint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glGetActiveUniformBlockName = unsafe extern "system" fn(
    _program: GLuint,
    _uniformBlockIndex: GLuint,
    _bufSize: GLsizei,
    _length: *mut GLsizei,
    _uniformBlockName: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexArrayAttribIFormat = unsafe extern "system" fn(
    _vaobj: GLuint,
    _attribindex: GLuint,
    _size: GLint,
    _type: VertexAttribIType,
    _relativeoffset: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribP1uiv = unsafe extern "system" fn(
    _index: GLuint,
    _type: VertexAttribPointerType,
    _normalized: GLboolean,
    _value: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glLockArraysEXT = unsafe extern "system" fn(_first: GLint, _count: GLsizei);
#[allow(non_camel_case_types)]
pub type PFN_glTextureViewEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _origtexture: GLuint,
    _internalformat: SizedInternalFormat,
    _minlevel: GLuint,
    _numlevels: GLuint,
    _minlayer: GLuint,
    _numlayers: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL1ui64vNV =
    unsafe extern "system" fn(_index: GLuint, _v: *const GLuint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glViewportArrayvNV =
    unsafe extern "system" fn(_first: GLuint, _count: GLsizei, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord1bvOES = unsafe extern "system" fn(_coords: *const GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glUniformMatrix2fv = unsafe extern "system" fn(
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord3iARB =
    unsafe extern "system" fn(_target: TextureUnit, _s: GLint, _t: GLint, _r: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glUniform4uiEXT =
    unsafe extern "system" fn(_location: GLint, _v0: GLuint, _v1: GLuint, _v2: GLuint, _v3: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexP2ui = unsafe extern "system" fn(_type: VertexPointerType, _value: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glUpdateObjectBufferATI = unsafe extern "system" fn(
    _buffer: GLuint,
    _offset: GLuint,
    _size: GLsizei,
    _pointer: *const std::os::raw::c_void,
    _preserve: PreserveModeATI,
);
#[allow(non_camel_case_types)]
pub type PFN_glDepthRangeArraydvNV =
    unsafe extern "system" fn(_first: GLuint, _count: GLsizei, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glGlobalAlphaFactorusSUN = unsafe extern "system" fn(_factor: GLushort);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferTextureLayerARB = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _attachment: FramebufferAttachment,
    _texture: GLuint,
    _level: GLint,
    _layer: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glFrustumxOES = unsafe extern "system" fn(
    _l: GLfixed,
    _r: GLfixed,
    _b: GLfixed,
    _t: GLfixed,
    _n: GLfixed,
    _f: GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glCullParameterdvEXT =
    unsafe extern "system" fn(_pname: CullParameterEXT, _params: *mut GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glClientActiveTextureARB = unsafe extern "system" fn(_texture: TextureUnit);
#[allow(non_camel_case_types)]
pub type PFN_glScalef = unsafe extern "system" fn(_x: GLfloat, _y: GLfloat, _z: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetPerfMonitorGroupStringAMD = unsafe extern "system" fn(
    _group: GLuint,
    _bufSize: GLsizei,
    _length: *mut GLsizei,
    _groupString: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glNamedFramebufferSampleLocationsfvNV = unsafe extern "system" fn(
    _framebuffer: GLuint,
    _start: GLuint,
    _count: GLsizei,
    _v: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetNamedFramebufferParameteriv = unsafe extern "system" fn(
    _framebuffer: GLuint,
    _pname: GetFramebufferParameter,
    _param: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glCopyTexSubImage3D = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _zoffset: GLint,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixOrthoEXT = unsafe extern "system" fn(
    _mode: MatrixMode,
    _left: GLdouble,
    _right: GLdouble,
    _bottom: GLdouble,
    _top: GLdouble,
    _zNear: GLdouble,
    _zFar: GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureStorage1D = unsafe extern "system" fn(
    _texture: GLuint,
    _levels: GLsizei,
    _internalformat: SizedInternalFormat,
    _width: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI4usv = unsafe extern "system" fn(_index: GLuint, _v: *const GLushort);
#[allow(non_camel_case_types)]
pub type PFN_glClipPlanefOES =
    unsafe extern "system" fn(_plane: ClipPlaneName, _equation: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetGraphicsResetStatus = unsafe extern "system" fn() -> GLenum;
#[allow(non_camel_case_types)]
pub type PFN_glProgramParameter4dNV = unsafe extern "system" fn(
    _target: VertexAttribEnumNV,
    _index: GLuint,
    _x: GLdouble,
    _y: GLdouble,
    _z: GLdouble,
    _w: GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glBeginVideoCaptureNV = unsafe extern "system" fn(_video_capture_slot: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferTextureLayer = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _attachment: FramebufferAttachment,
    _texture: GLuint,
    _level: GLint,
    _layer: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glCopyImageSubDataOES = unsafe extern "system" fn(
    _srcName: GLuint,
    _srcTarget: CopyBufferSubDataTarget,
    _srcLevel: GLint,
    _srcX: GLint,
    _srcY: GLint,
    _srcZ: GLint,
    _dstName: GLuint,
    _dstTarget: CopyBufferSubDataTarget,
    _dstLevel: GLint,
    _dstX: GLint,
    _dstY: GLint,
    _dstZ: GLint,
    _srcWidth: GLsizei,
    _srcHeight: GLsizei,
    _srcDepth: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glExtTexObjectStateOverrideiQCOM =
    unsafe extern "system" fn(_target: GLenum, _pname: GLenum, _param: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferDrawBufferEXT =
    unsafe extern "system" fn(_framebuffer: GLuint, _mode: DrawBufferMode);
#[allow(non_camel_case_types)]
pub type PFN_glGetBufferParameterivARB = unsafe extern "system" fn(
    _target: BufferTargetARB,
    _pname: BufferPNameARB,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetQueryivARB = unsafe extern "system" fn(
    _target: QueryTarget,
    _pname: QueryParameterName,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetShaderSourceARB = unsafe extern "system" fn(
    _obj: GLhandleARB,
    _maxLength: GLsizei,
    _length: *mut GLsizei,
    _source: *mut GLcharARB,
);
#[allow(non_camel_case_types)]
pub type PFN_glGlobalAlphaFactorfSUN = unsafe extern "system" fn(_factor: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glMultiModeDrawElementsIBM = unsafe extern "system" fn(
    _mode: *const PrimitiveType,
    _count: *const GLsizei,
    _type: DrawElementsType,
    _indices: *const *const std::os::raw::c_void,
    _primcount: GLsizei,
    _modestride: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform2fEXT =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _v0: GLfloat, _v1: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glSwizzleEXT = unsafe extern "system" fn(
    _res: GLuint,
    _in: GLuint,
    _outX: VertexShaderCoordOutEXT,
    _outY: VertexShaderCoordOutEXT,
    _outZ: VertexShaderCoordOutEXT,
    _outW: VertexShaderCoordOutEXT,
);
#[allow(non_camel_case_types)]
pub type PFN_glTangent3sEXT = unsafe extern "system" fn(_tx: GLshort, _ty: GLshort, _tz: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord1fv = unsafe extern "system" fn(_v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetQueryObjectui64v = unsafe extern "system" fn(
    _id: GLuint,
    _pname: QueryObjectParameterName,
    _params: *mut GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexStorage3D = unsafe extern "system" fn(
    _target: TextureTarget,
    _levels: GLsizei,
    _internalformat: SizedInternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glEnableVertexArrayEXT = unsafe extern "system" fn(_vaobj: GLuint, _array: EnableCap);
#[allow(non_camel_case_types)]
pub type PFN_glDrawElementsBaseVertexOES = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _count: GLsizei,
    _type: DrawElementsType,
    _indices: *const std::os::raw::c_void,
    _basevertex: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetVideoCaptureStreamivNV = unsafe extern "system" fn(
    _video_capture_slot: GLuint,
    _stream: GLuint,
    _pname: GLenum,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord3hNV = unsafe extern "system" fn(_s: GLhalfNV, _t: GLhalfNV, _r: GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glDrawBuffersEXT = unsafe extern "system" fn(_n: GLsizei, _bufs: *const GLenum);
#[allow(non_camel_case_types)]
pub type PFN_glPolygonOffsetxOES = unsafe extern "system" fn(_factor: GLfixed, _units: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glMultiDrawArraysIndirectBindlessCountNV = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _indirect: *const std::os::raw::c_void,
    _drawCount: GLsizei,
    _maxDrawCount: GLsizei,
    _stride: GLsizei,
    _vertexBufferCount: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glBlendEquationSeparateiEXT = unsafe extern "system" fn(
    _buf: GLuint,
    _modeRGB: BlendEquationModeEXT,
    _modeAlpha: BlendEquationModeEXT,
);
#[allow(non_camel_case_types)]
pub type PFN_glFogxv = unsafe extern "system" fn(_pname: FogPName, _param: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glBufferStorageEXT = unsafe extern "system" fn(
    _target: BufferStorageTarget,
    _size: GLsizeiptr,
    _data: *const std::os::raw::c_void,
    _flags: BufferStorageMask,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform4dv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glApplyFramebufferAttachmentCMAAINTEL = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glDrawArraysInstancedARB = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _first: GLint,
    _count: GLsizei,
    _primcount: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glSampleMapATI =
    unsafe extern "system" fn(_dst: GLuint, _interp: GLuint, _swizzle: SwizzleOpATI);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos3ivMESA = unsafe extern "system" fn(_v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glClearAccumxOES =
    unsafe extern "system" fn(_red: GLfixed, _green: GLfixed, _blue: GLfixed, _alpha: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glSpriteParameteriSGIX =
    unsafe extern "system" fn(_pname: SpriteParameterNameSGIX, _param: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord4bOES = unsafe extern "system" fn(
    _texture: TextureUnit,
    _s: GLbyte,
    _t: GLbyte,
    _r: GLbyte,
    _q: GLbyte,
);
#[allow(non_camel_case_types)]
pub type PFN_glRecti = unsafe extern "system" fn(_x1: GLint, _y1: GLint, _x2: GLint, _y2: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glDisableVertexAttribArrayARB = unsafe extern "system" fn(_index: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glBindTextureEXT = unsafe extern "system" fn(_target: TextureTarget, _texture: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glDetachShader = unsafe extern "system" fn(_program: GLuint, _shader: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix3x4fvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glCompressedTexImage3D = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _border: GLint,
    _imageSize: GLsizei,
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glEndConditionalRenderNV = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4uivARB = unsafe extern "system" fn(_index: GLuint, _v: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiDrawRangeElementArrayAPPLE = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _start: GLuint,
    _end: GLuint,
    _first: *const GLint,
    _count: *const GLsizei,
    _primcount: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glMaterialxvOES = unsafe extern "system" fn(
    _face: MaterialFace,
    _pname: MaterialParameter,
    _param: *const GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glScissorIndexedNV = unsafe extern "system" fn(
    _index: GLuint,
    _left: GLint,
    _bottom: GLint,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform3ui64vNV =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLuint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexAttribLdvEXT =
    unsafe extern "system" fn(_index: GLuint, _pname: VertexAttribEnum, _params: *mut GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glProgramBufferParametersIivNV = unsafe extern "system" fn(
    _target: ProgramTarget,
    _bindingIndex: GLuint,
    _wordIndex: GLuint,
    _count: GLsizei,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord4hNV =
    unsafe extern "system" fn(_s: GLhalfNV, _t: GLhalfNV, _r: GLhalfNV, _q: GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glUniform1ui64ARB = unsafe extern "system" fn(_location: GLint, _x: GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glBindTexGenParameterEXT = unsafe extern "system" fn(
    _unit: TextureUnit,
    _coord: TextureCoordName,
    _value: TextureGenParameter,
) -> GLuint;
#[allow(non_camel_case_types)]
pub type PFN_glPointParameterfSGIS =
    unsafe extern "system" fn(_pname: PointParameterNameARB, _param: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glRectd =
    unsafe extern "system" fn(_x1: GLdouble, _y1: GLdouble, _x2: GLdouble, _y2: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glGetMultiTexImageEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _level: GLint,
    _format: PixelFormat,
    _type: PixelType,
    _pixels: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord4d = unsafe extern "system" fn(
    _target: TextureUnit,
    _s: GLdouble,
    _t: GLdouble,
    _r: GLdouble,
    _q: GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexGeni =
    unsafe extern "system" fn(_coord: TextureCoordName, _pname: TextureGenParameter, _param: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos2f = unsafe extern "system" fn(_x: GLfloat, _y: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos3xOES = unsafe extern "system" fn(_x: GLfixed, _y: GLfixed, _z: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glFragmentMaterialfvSGIX = unsafe extern "system" fn(
    _face: MaterialFace,
    _pname: MaterialParameter,
    _params: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glClipPlanefIMG = unsafe extern "system" fn(_p: ClipPlaneName, _eqn: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glBlendBarrier = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glColor3f = unsafe extern "system" fn(_red: GLfloat, _green: GLfloat, _blue: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glConvolutionParameterxvOES = unsafe extern "system" fn(
    _target: ConvolutionTargetEXT,
    _pname: ConvolutionParameterEXT,
    _params: *const GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform4ivEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glMaterialxv = unsafe extern "system" fn(
    _face: MaterialFace,
    _pname: MaterialParameter,
    _param: *const GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glFeedbackBufferxOES =
    unsafe extern "system" fn(_n: GLsizei, _type: GLenum, _buffer: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glDrawArraysInstancedANGLE = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _first: GLint,
    _count: GLsizei,
    _primcount: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glNormal3fv = unsafe extern "system" fn(_v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glReferencePlaneSGIX = unsafe extern "system" fn(_equation: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glIsRenderbufferEXT = unsafe extern "system" fn(_renderbuffer: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos2sARB = unsafe extern "system" fn(_x: GLshort, _y: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform1ivEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGenVertexArraysAPPLE = unsafe extern "system" fn(_n: GLsizei, _arrays: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glColor4s =
    unsafe extern "system" fn(_red: GLshort, _green: GLshort, _blue: GLshort, _alpha: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glHintPGI = unsafe extern "system" fn(_target: HintTargetPGI, _mode: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glDrawCommandsStatesAddressNV = unsafe extern "system" fn(
    _indirects: *const GLuint64,
    _sizes: *const GLsizei,
    _states: *const GLuint,
    _fbos: *const GLuint,
    _count: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glDrawElementsInstancedBaseVertex = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _count: GLsizei,
    _type: DrawElementsType,
    _indices: *const std::os::raw::c_void,
    _instancecount: GLsizei,
    _basevertex: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glDrawMeshTasksNV = unsafe extern "system" fn(_first: GLuint, _count: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiDrawArraysIndirectBindlessNV = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _indirect: *const std::os::raw::c_void,
    _drawCount: GLsizei,
    _stride: GLsizei,
    _vertexBufferCount: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glNormalStream3bvATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _coords: *const GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glOrtho = unsafe extern "system" fn(
    _left: GLdouble,
    _right: GLdouble,
    _bottom: GLdouble,
    _top: GLdouble,
    _zNear: GLdouble,
    _zFar: GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glPointParameterxOES =
    unsafe extern "system" fn(_pname: PointParameterNameARB, _param: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glVDPAUUnregisterSurfaceNV = unsafe extern "system" fn(_surface: GLvdpauSurfaceNV);
#[allow(non_camel_case_types)]
pub type PFN_glGenLists = unsafe extern "system" fn(_range: GLsizei) -> GLuint;
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform1fvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glNamedFramebufferTexture3DEXT = unsafe extern "system" fn(
    _framebuffer: GLuint,
    _attachment: FramebufferAttachment,
    _textarget: TextureTarget,
    _texture: GLuint,
    _level: GLint,
    _zoffset: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetObjectLabelKHR = unsafe extern "system" fn(
    _identifier: GLenum,
    _name: GLuint,
    _bufSize: GLsizei,
    _length: *mut GLsizei,
    _label: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glBinormalPointerEXT = unsafe extern "system" fn(
    _type: BinormalPointerTypeEXT,
    _stride: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glRenderGpuMaskNV = unsafe extern "system" fn(_mask: GLbitfield);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3usEXT =
    unsafe extern "system" fn(_red: GLushort, _green: GLushort, _blue: GLushort);
#[allow(non_camel_case_types)]
pub type PFN_glTexSubImage1DEXT = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _xoffset: GLint,
    _width: GLsizei,
    _format: PixelFormat,
    _type: PixelType,
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetnColorTableARB = unsafe extern "system" fn(
    _target: ColorTableTarget,
    _format: PixelFormat,
    _type: PixelType,
    _bufSize: GLsizei,
    _table: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glBindProgramPipeline = unsafe extern "system" fn(_pipeline: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetnMapfvARB = unsafe extern "system" fn(
    _target: MapTarget,
    _query: MapQuery,
    _bufSize: GLsizei,
    _v: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glCompressedTexSubImage1DARB = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _xoffset: GLint,
    _width: GLsizei,
    _format: InternalFormat,
    _imageSize: GLsizei,
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glRectfv = unsafe extern "system" fn(_v1: *const GLfloat, _v2: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4ubvARB = unsafe extern "system" fn(_index: GLuint, _v: *const GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI1i = unsafe extern "system" fn(_index: GLuint, _x: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glNormalP3uiv =
    unsafe extern "system" fn(_type: NormalPointerType, _coords: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glDepthRangeArrayfvNV =
    unsafe extern "system" fn(_first: GLuint, _count: GLsizei, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glNormalStream3svATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _coords: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glLightModelxv =
    unsafe extern "system" fn(_pname: LightModelParameter, _param: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glGetTexParameteriv = unsafe extern "system" fn(
    _target: TextureTarget,
    _pname: GetTextureParameter,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glEnableiNV = unsafe extern "system" fn(_target: EnableCap, _index: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGenerateTextureMipmapEXT =
    unsafe extern "system" fn(_texture: GLuint, _target: TextureTarget);
#[allow(non_camel_case_types)]
pub type PFN_glExtGetShadersQCOM =
    unsafe extern "system" fn(_shaders: *mut GLuint, _maxShaders: GLint, _numShaders: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glRenderbufferStorageOES = unsafe extern "system" fn(
    _target: RenderbufferTarget,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glBinormal3iEXT = unsafe extern "system" fn(_bx: GLint, _by: GLint, _bz: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glBlendFuncSeparateiARB = unsafe extern "system" fn(
    _buf: GLuint,
    _srcRGB: BlendingFactor,
    _dstRGB: BlendingFactor,
    _srcAlpha: BlendingFactor,
    _dstAlpha: BlendingFactor,
);
#[allow(non_camel_case_types)]
pub type PFN_glFragmentColorMaterialSGIX =
    unsafe extern "system" fn(_face: MaterialFace, _mode: MaterialParameter);
#[allow(non_camel_case_types)]
pub type PFN_glLabelObjectEXT = unsafe extern "system" fn(
    _type: GLenum,
    _object: GLuint,
    _length: GLsizei,
    _label: *const GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glRenderbufferStorageMultisampleCoverageNV = unsafe extern "system" fn(
    _target: RenderbufferTarget,
    _coverageSamples: GLsizei,
    _colorSamples: GLsizei,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glCallCommandListNV = unsafe extern "system" fn(_list: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glColorP3ui = unsafe extern "system" fn(_type: ColorPointerType, _color: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramNamedParameter4fvNV = unsafe extern "system" fn(
    _id: GLuint,
    _len: GLsizei,
    _name: *const GLubyte,
    _v: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexBumpParameterivATI =
    unsafe extern "system" fn(_pname: TexBumpParameterATI, _param: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glTextureLightEXT = unsafe extern "system" fn(_pname: LightTexturePNameEXT);
#[allow(non_camel_case_types)]
pub type PFN_glInterpolatePathsNV = unsafe extern "system" fn(
    _resultPath: GLuint,
    _pathA: GLuint,
    _pathB: GLuint,
    _weight: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib2dARB =
    unsafe extern "system" fn(_index: GLuint, _x: GLdouble, _y: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glWeightdvARB = unsafe extern "system" fn(_size: GLint, _weights: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3bEXT =
    unsafe extern "system" fn(_red: GLbyte, _green: GLbyte, _blue: GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib2f = unsafe extern "system" fn(_index: GLuint, _x: GLfloat, _y: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferTexture2DEXT = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _attachment: FramebufferAttachment,
    _textarget: TextureTarget,
    _texture: GLuint,
    _level: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL1dEXT = unsafe extern "system" fn(_index: GLuint, _x: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4svNV = unsafe extern "system" fn(_index: GLuint, _v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord3sARB =
    unsafe extern "system" fn(_target: TextureUnit, _s: GLshort, _t: GLshort, _r: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glGetCompressedMultiTexImageEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _lod: GLint,
    _img: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform3dv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glEvalMesh1 = unsafe extern "system" fn(_mode: MeshMode1, _i1: GLint, _i2: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glTexEnvi = unsafe extern "system" fn(
    _target: TextureEnvTarget,
    _pname: TextureEnvParameter,
    _param: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGenVertexShadersEXT = unsafe extern "system" fn(_range: GLuint) -> GLuint;
#[allow(non_camel_case_types)]
pub type PFN_glResetMinmaxEXT = unsafe extern "system" fn(_target: MinmaxTargetEXT);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix2x3fvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertex3fv = unsafe extern "system" fn(_v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glUniformHandleui64vARB =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glGetActiveAttribARB = unsafe extern "system" fn(
    _programObj: GLhandleARB,
    _index: GLuint,
    _maxLength: GLsizei,
    _length: *mut GLsizei,
    _size: *mut GLint,
    _type: *mut AttributeType,
    _name: *mut GLcharARB,
);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteOcclusionQueriesNV =
    unsafe extern "system" fn(_n: GLsizei, _ids: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glDrawVkImageNV = unsafe extern "system" fn(
    _vkImage: GLuint64,
    _sampler: GLuint,
    _x0: GLfloat,
    _y0: GLfloat,
    _x1: GLfloat,
    _y1: GLfloat,
    _z: GLfloat,
    _s0: GLfloat,
    _t0: GLfloat,
    _s1: GLfloat,
    _t1: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribP2ui = unsafe extern "system" fn(
    _index: GLuint,
    _type: VertexAttribPointerType,
    _normalized: GLboolean,
    _value: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord3i = unsafe extern "system" fn(_s: GLint, _t: GLint, _r: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glUniformMatrix3x4dv = unsafe extern "system" fn(
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoordP4ui =
    unsafe extern "system" fn(_texture: TextureUnit, _type: TexCoordPointerType, _coords: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glBindBufferOffsetEXT = unsafe extern "system" fn(
    _target: BufferTargetARB,
    _index: GLuint,
    _buffer: GLuint,
    _offset: GLintptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramSubroutineParametersuivNV =
    unsafe extern "system" fn(_target: GLenum, _count: GLsizei, _params: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glClearIndex = unsafe extern "system" fn(_c: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetHistogramParameteriv = unsafe extern "system" fn(
    _target: HistogramTargetEXT,
    _pname: GetHistogramParameterPNameEXT,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniformMatrix2fvARB = unsafe extern "system" fn(
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glScissorIndexedvNV = unsafe extern "system" fn(_index: GLuint, _v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetDriverControlStringQCOM = unsafe extern "system" fn(
    _driverControl: GLuint,
    _bufSize: GLsizei,
    _length: *mut GLsizei,
    _driverControlString: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetUnsignedBytevEXT =
    unsafe extern "system" fn(_pname: GetPName, _data: *mut GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glColor3usv = unsafe extern "system" fn(_v: *const GLushort);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramEnvParameterdvARB =
    unsafe extern "system" fn(_target: ProgramTarget, _index: GLuint, _params: *mut GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3ui =
    unsafe extern "system" fn(_red: GLuint, _green: GLuint, _blue: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glPointParameteriNV =
    unsafe extern "system" fn(_pname: PointParameterNameARB, _param: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord3xvOES =
    unsafe extern "system" fn(_texture: TextureUnit, _coords: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glDisableVertexAttribAPPLE = unsafe extern "system" fn(_index: GLuint, _pname: GLenum);
#[allow(non_camel_case_types)]
pub type PFN_glClipPlanef = unsafe extern "system" fn(_p: ClipPlaneName, _eqn: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL1ui64NV = unsafe extern "system" fn(_index: GLuint, _x: GLuint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glFinishAsyncSGIX = unsafe extern "system" fn(_markerp: *mut GLuint) -> GLint;
#[allow(non_camel_case_types)]
pub type PFN_glMakeImageHandleResidentNV =
    unsafe extern "system" fn(_handle: GLuint64, _access: GLenum);
#[allow(non_camel_case_types)]
pub type PFN_glGetNamedBufferParameteriv =
    unsafe extern "system" fn(_buffer: GLuint, _pname: BufferPNameARB, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetQueryBufferObjectiv = unsafe extern "system" fn(
    _id: GLuint,
    _buffer: GLuint,
    _pname: QueryObjectParameterName,
    _offset: GLintptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glDrawElementsInstancedARB = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _count: GLsizei,
    _type: DrawElementsType,
    _indices: *const std::os::raw::c_void,
    _primcount: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoordP2uiv = unsafe extern "system" fn(
    _texture: TextureUnit,
    _type: TexCoordPointerType,
    _coords: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramLocalParameterI4iNV = unsafe extern "system" fn(
    _target: ProgramTarget,
    _index: GLuint,
    _x: GLint,
    _y: GLint,
    _z: GLint,
    _w: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetPointerIndexedvEXT = unsafe extern "system" fn(
    _target: GLenum,
    _index: GLuint,
    _data: *mut *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform3ui64ARB =
    unsafe extern "system" fn(_location: GLint, _x: GLuint64, _y: GLuint64, _z: GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glConvolutionParameteriEXT = unsafe extern "system" fn(
    _target: ConvolutionTargetEXT,
    _pname: ConvolutionParameterEXT,
    _params: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glNormal3fVertex3fvSUN =
    unsafe extern "system" fn(_n: *const GLfloat, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixRotatefEXT = unsafe extern "system" fn(
    _mode: MatrixMode,
    _angle: GLfloat,
    _x: GLfloat,
    _y: GLfloat,
    _z: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glCoverageModulationTableNV =
    unsafe extern "system" fn(_n: GLsizei, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos4ivMESA = unsafe extern "system" fn(_v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetUniformui64vNV =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _params: *mut GLuint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord2fARB =
    unsafe extern "system" fn(_target: TextureUnit, _s: GLfloat, _t: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4uiv = unsafe extern "system" fn(_index: GLuint, _v: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glFogCoordPointerListIBM = unsafe extern "system" fn(
    _type: FogPointerTypeIBM,
    _stride: GLint,
    _pointer: *mut *const std::os::raw::c_void,
    _ptrstride: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glColor3xvOES = unsafe extern "system" fn(_components: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glArrayElement = unsafe extern "system" fn(_i: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glCompressedMultiTexSubImage3DEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
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
    _bits: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetClipPlanexOES =
    unsafe extern "system" fn(_plane: ClipPlaneName, _equation: *mut GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glBindVertexArrayAPPLE = unsafe extern "system" fn(_array: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glCoverFillPathInstancedNV = unsafe extern "system" fn(
    _numPaths: GLsizei,
    _pathNameType: PathElementType,
    _paths: *const std::os::raw::c_void,
    _pathBase: GLuint,
    _coverMode: PathCoverMode,
    _transformType: PathTransformType,
    _transformValues: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glColor4uiv = unsafe extern "system" fn(_v: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glCullFace = unsafe extern "system" fn(_mode: CullFaceMode);
#[allow(non_camel_case_types)]
pub type PFN_glGetLocalConstantFloatvEXT =
    unsafe extern "system" fn(_id: GLuint, _value: GetVariantValueEXT, _data: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetArrayObjectfvATI = unsafe extern "system" fn(
    _array: EnableCap,
    _pname: ArrayObjectPNameATI,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramResourceIndex = unsafe extern "system" fn(
    _program: GLuint,
    _programInterface: ProgramInterface,
    _name: *const GLchar,
) -> GLuint;
#[allow(non_camel_case_types)]
pub type PFN_glGetShaderiv =
    unsafe extern "system" fn(_shader: GLuint, _pname: ShaderParameterName, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGlobalAlphaFactoriSUN = unsafe extern "system" fn(_factor: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glLightx =
    unsafe extern "system" fn(_light: LightName, _pname: LightParameter, _param: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord3dv =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glNamedProgramLocalParameter4fvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _target: ProgramTarget,
    _index: GLuint,
    _params: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glPointParameterfvSGIS =
    unsafe extern "system" fn(_pname: PointParameterNameARB, _params: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGenTransformFeedbacks = unsafe extern "system" fn(_n: GLsizei, _ids: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glPolygonModeNV = unsafe extern "system" fn(_face: MaterialFace, _mode: PolygonMode);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform3d = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _v0: GLdouble,
    _v1: GLdouble,
    _v2: GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glDrawArraysEXT =
    unsafe extern "system" fn(_mode: PrimitiveType, _first: GLint, _count: GLsizei);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColorPointerEXT = unsafe extern "system" fn(
    _size: GLint,
    _type: ColorPointerType,
    _stride: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glWeightPointerARB = unsafe extern "system" fn(
    _size: GLint,
    _type: WeightPointerTypeARB,
    _stride: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glBinormal3bEXT = unsafe extern "system" fn(_bx: GLbyte, _by: GLbyte, _bz: GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glBinormal3svEXT = unsafe extern "system" fn(_v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glDrawCommandsNV = unsafe extern "system" fn(
    _primitiveMode: GLenum,
    _buffer: GLuint,
    _indirects: *const GLintptr,
    _sizes: *const GLsizei,
    _count: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glInvalidateSubFramebuffer = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _numAttachments: GLsizei,
    _attachments: *const InvalidateFramebufferAttachment,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glFogCoordPointerEXT = unsafe extern "system" fn(
    _type: FogPointerTypeEXT,
    _stride: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetFragmentMaterialivSGIX =
    unsafe extern "system" fn(_face: MaterialFace, _pname: MaterialParameter, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetIntegeri_v =
    unsafe extern "system" fn(_target: GetPName, _index: GLuint, _data: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glAttachShader = unsafe extern "system" fn(_program: GLuint, _shader: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetMemoryObjectDetachedResourcesuivNV = unsafe extern "system" fn(
    _memory: GLuint,
    _pname: GLenum,
    _first: GLint,
    _count: GLsizei,
    _params: *mut GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetObjectBufferivATI =
    unsafe extern "system" fn(_buffer: GLuint, _pname: ArrayObjectPNameATI, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetPixelTransformParameterivEXT =
    unsafe extern "system" fn(_target: GLenum, _pname: GLenum, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixIndexPointerOES = unsafe extern "system" fn(
    _size: GLint,
    _type: MatrixIndexPointerTypeARB,
    _stride: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glConvolutionParameterxOES = unsafe extern "system" fn(
    _target: ConvolutionTargetEXT,
    _pname: ConvolutionParameterEXT,
    _param: GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glBinormal3bvEXT = unsafe extern "system" fn(_v: *const GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glGetActiveUniform = unsafe extern "system" fn(
    _program: GLuint,
    _index: GLuint,
    _bufSize: GLsizei,
    _length: *mut GLsizei,
    _size: *mut GLint,
    _type: *mut UniformType,
    _name: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiDrawElementsIndirectBindlessNV = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _type: DrawElementsType,
    _indirect: *const std::os::raw::c_void,
    _drawCount: GLsizei,
    _stride: GLsizei,
    _vertexBufferCount: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexImage1DEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _level: GLint,
    _internalformat: GLint,
    _width: GLsizei,
    _border: GLint,
    _format: PixelFormat,
    _type: PixelType,
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord3svARB =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glNamedFramebufferTextureLayer = unsafe extern "system" fn(
    _framebuffer: GLuint,
    _attachment: FramebufferAttachment,
    _texture: GLuint,
    _level: GLint,
    _layer: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glDrawTransformFeedbackInstanced =
    unsafe extern "system" fn(_mode: PrimitiveType, _id: GLuint, _instancecount: GLsizei);
#[allow(non_camel_case_types)]
pub type PFN_glTexStorageMem3DEXT = unsafe extern "system" fn(
    _target: TextureTarget,
    _levels: GLsizei,
    _internalFormat: SizedInternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _memory: GLuint,
    _offset: GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glDebugMessageCallback =
    unsafe extern "system" fn(_callback: GLDEBUGPROC, _userParam: *const std::os::raw::c_void);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4fv = unsafe extern "system" fn(_index: GLuint, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos2iv = unsafe extern "system" fn(_v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib1d = unsafe extern "system" fn(_index: GLuint, _x: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glColor4b =
    unsafe extern "system" fn(_red: GLbyte, _green: GLbyte, _blue: GLbyte, _alpha: GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glCoverStrokePathNV =
    unsafe extern "system" fn(_path: GLuint, _coverMode: PathCoverMode);
#[allow(non_camel_case_types)]
pub type PFN_glEvalMesh2 =
    unsafe extern "system" fn(_mode: MeshMode2, _i1: GLint, _i2: GLint, _j1: GLint, _j2: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glAreProgramsResidentNV = unsafe extern "system" fn(
    _n: GLsizei,
    _programs: *const GLuint,
    _residences: *mut GLboolean,
) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glGetNamedProgramLocalParameterIivEXT = unsafe extern "system" fn(
    _program: GLuint,
    _target: ProgramTarget,
    _index: GLuint,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetPolygonStipple = unsafe extern "system" fn(_mask: *mut GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glPixelTransferf =
    unsafe extern "system" fn(_pname: PixelTransferParameter, _param: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetMaterialiv =
    unsafe extern "system" fn(_face: MaterialFace, _pname: MaterialParameter, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord1dvARB =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform2d =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _v0: GLdouble, _v1: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glGetInvariantBooleanvEXT =
    unsafe extern "system" fn(_id: GLuint, _value: GetVariantValueEXT, _data: *mut GLboolean);
#[allow(non_camel_case_types)]
pub type PFN_glGetIntegerui64i_vNV =
    unsafe extern "system" fn(_value: GLenum, _index: GLuint, _result: *mut GLuint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord1dARB = unsafe extern "system" fn(_target: TextureUnit, _s: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribs4fvNV =
    unsafe extern "system" fn(_index: GLuint, _count: GLsizei, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glBeginQueryARB = unsafe extern "system" fn(_target: QueryTarget, _id: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glPopDebugGroup = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glDeleteStatesNV = unsafe extern "system" fn(_n: GLsizei, _states: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform1uiEXT =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _v0: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glBlitFramebufferNV = unsafe extern "system" fn(
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
);
#[allow(non_camel_case_types)]
pub type PFN_glDrawArraysInstanced = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _first: GLint,
    _count: GLsizei,
    _instancecount: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteBuffers = unsafe extern "system" fn(_n: GLsizei, _buffers: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glCoverageModulationNV = unsafe extern "system" fn(_components: GLenum);
#[allow(non_camel_case_types)]
pub type PFN_glGetImageTransformParameterivHP = unsafe extern "system" fn(
    _target: ImageTransformTargetHP,
    _pname: ImageTransformPNameHP,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glEGLImageTargetTextureStorageEXT =
    unsafe extern "system" fn(_texture: GLuint, _image: GLeglImageOES, _attrib_list: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glDisableVertexArrayEXT = unsafe extern "system" fn(_vaobj: GLuint, _array: EnableCap);
#[allow(non_camel_case_types)]
pub type PFN_glGetQueryObjectuivEXT =
    unsafe extern "system" fn(_id: GLuint, _pname: QueryObjectParameterName, _params: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix3dv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glFragmentLightfvSGIX = unsafe extern "system" fn(
    _light: FragmentLightNameSGIX,
    _pname: FragmentLightParameterSGIX,
    _params: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos4xvOES = unsafe extern "system" fn(_coords: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glGetnCompressedTexImage = unsafe extern "system" fn(
    _target: TextureTarget,
    _lod: GLint,
    _bufSize: GLsizei,
    _pixels: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glRotatef =
    unsafe extern "system" fn(_angle: GLfloat, _x: GLfloat, _y: GLfloat, _z: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI3ivEXT = unsafe extern "system" fn(_index: GLuint, _v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetVideoCaptureivNV =
    unsafe extern "system" fn(_video_capture_slot: GLuint, _pname: GLenum, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glReadnPixelsKHR = unsafe extern "system" fn(
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _format: PixelFormat,
    _type: PixelType,
    _bufSize: GLsizei,
    _data: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexArrayElementBuffer =
    unsafe extern "system" fn(_vaobj: GLuint, _buffer: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetInteger64vAPPLE =
    unsafe extern "system" fn(_pname: GetPName, _params: *mut GLint64);
#[allow(non_camel_case_types)]
pub type PFN_glGetObjectParameterivARB =
    unsafe extern "system" fn(_obj: GLhandleARB, _pname: GLenum, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetInstrumentsSGIX = unsafe extern "system" fn() -> GLint;
#[allow(non_camel_case_types)]
pub type PFN_glGetSamplerParameterIiv =
    unsafe extern "system" fn(_sampler: GLuint, _pname: SamplerParameterI, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord3fv =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glMulticastCopyImageSubDataNV = unsafe extern "system" fn(
    _srcGpu: GLuint,
    _dstGpuMask: GLbitfield,
    _srcName: GLuint,
    _srcTarget: GLenum,
    _srcLevel: GLint,
    _srcX: GLint,
    _srcY: GLint,
    _srcZ: GLint,
    _dstName: GLuint,
    _dstTarget: GLenum,
    _dstLevel: GLint,
    _dstX: GLint,
    _dstY: GLint,
    _dstZ: GLint,
    _srcWidth: GLsizei,
    _srcHeight: GLsizei,
    _srcDepth: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glCompressedMultiTexSubImage1DEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _level: GLint,
    _xoffset: GLint,
    _width: GLsizei,
    _format: InternalFormat,
    _imageSize: GLsizei,
    _bits: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glObjectPtrLabel = unsafe extern "system" fn(
    _ptr: *const std::os::raw::c_void,
    _length: GLsizei,
    _label: *const GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform4i64ARB = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _x: GLint64,
    _y: GLint64,
    _z: GLint64,
    _w: GLint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetCompressedTextureImage = unsafe extern "system" fn(
    _texture: GLuint,
    _level: GLint,
    _bufSize: GLsizei,
    _pixels: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glCompressedTexImage2D = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _border: GLint,
    _imageSize: GLsizei,
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glSampleCoveragexOES = unsafe extern "system" fn(_value: GLclampx, _invert: GLboolean);
#[allow(non_camel_case_types)]
pub type PFN_glTextureStorage2D = unsafe extern "system" fn(
    _texture: GLuint,
    _levels: GLsizei,
    _internalformat: SizedInternalFormat,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glReadnPixelsARB = unsafe extern "system" fn(
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _format: PixelFormat,
    _type: PixelType,
    _bufSize: GLsizei,
    _data: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetOcclusionQueryuivNV = unsafe extern "system" fn(
    _id: GLuint,
    _pname: OcclusionQueryParameterNameNV,
    _params: *mut GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glCreateShaderProgramv = unsafe extern "system" fn(
    _type: ShaderType,
    _count: GLsizei,
    _strings: *const *const GLchar,
) -> GLuint;
#[allow(non_camel_case_types)]
pub type PFN_glExtrapolateTex2DQCOM =
    unsafe extern "system" fn(_src1: GLuint, _src2: GLuint, _output: GLuint, _scaleFactor: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glTranslatex = unsafe extern "system" fn(_x: GLfixed, _y: GLfixed, _z: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glUniform2uivEXT =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glCommandListSegmentsNV = unsafe extern "system" fn(_list: GLuint, _segments: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glDebugMessageControl = unsafe extern "system" fn(
    _source: DebugSource,
    _type: DebugType,
    _severity: DebugSeverity,
    _count: GLsizei,
    _ids: *const GLuint,
    _enabled: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiDrawElementsIndirect = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _type: DrawElementsType,
    _indirect: *const std::os::raw::c_void,
    _drawcount: GLsizei,
    _stride: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetFloatv = unsafe extern "system" fn(_pname: GetPName, _data: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glDrawElementsBaseVertex = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _count: GLsizei,
    _type: DrawElementsType,
    _indices: *const std::os::raw::c_void,
    _basevertex: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramPipelineiv = unsafe extern "system" fn(
    _pipeline: GLuint,
    _pname: PipelineParameterName,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetTransformFeedbacki64_v = unsafe extern "system" fn(
    _xfb: GLuint,
    _pname: TransformFeedbackPName,
    _index: GLuint,
    _param: *mut GLint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glPassThroughxOES = unsafe extern "system" fn(_token: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glMulticastGetQueryObjecti64vNV =
    unsafe extern "system" fn(_gpu: GLuint, _id: GLuint, _pname: GLenum, _params: *mut GLint64);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord2bvOES = unsafe extern "system" fn(_coords: *const GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glNamedFramebufferReadBuffer =
    unsafe extern "system" fn(_framebuffer: GLuint, _src: ColorBuffer);
#[allow(non_camel_case_types)]
pub type PFN_glCreatePerfQueryINTEL =
    unsafe extern "system" fn(_queryId: GLuint, _queryHandle: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetBooleanv = unsafe extern "system" fn(_pname: GetPName, _data: *mut GLboolean);
#[allow(non_camel_case_types)]
pub type PFN_glGetMapiv =
    unsafe extern "system" fn(_target: MapTarget, _query: GetMapQuery, _v: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetNamedProgramivEXT = unsafe extern "system" fn(
    _program: GLuint,
    _target: ProgramTarget,
    _pname: ProgramPropertyARB,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixTranslatefEXT =
    unsafe extern "system" fn(_mode: MatrixMode, _x: GLfloat, _y: GLfloat, _z: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetMultiTexParameterIuivEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _pname: GetTextureParameter,
    _params: *mut GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glColor3us =
    unsafe extern "system" fn(_red: GLushort, _green: GLushort, _blue: GLushort);
#[allow(non_camel_case_types)]
pub type PFN_glFogCoorddEXT = unsafe extern "system" fn(_coord: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glMapGrid1f = unsafe extern "system" fn(_un: GLint, _u1: GLfloat, _u2: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glMaxShaderCompilerThreadsKHR = unsafe extern "system" fn(_count: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoordP3ui =
    unsafe extern "system" fn(_texture: TextureUnit, _type: TexCoordPointerType, _coords: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord2fColor3fVertex3fvSUN =
    unsafe extern "system" fn(_tc: *const GLfloat, _c: *const GLfloat, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glAlphaFuncQCOM = unsafe extern "system" fn(_func: GLenum, _ref: GLclampf);
#[allow(non_camel_case_types)]
pub type PFN_glInvalidateFramebuffer = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _numAttachments: GLsizei,
    _attachments: *const InvalidateFramebufferAttachment,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureBuffer = unsafe extern "system" fn(
    _texture: GLuint,
    _internalformat: SizedInternalFormat,
    _buffer: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureParameterf =
    unsafe extern "system" fn(_texture: GLuint, _pname: TextureParameterName, _param: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glClearNamedBufferSubData = unsafe extern "system" fn(
    _buffer: GLuint,
    _internalformat: SizedInternalFormat,
    _offset: GLintptr,
    _size: GLsizeiptr,
    _format: PixelFormat,
    _type: PixelType,
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetTexParameterxvOES = unsafe extern "system" fn(
    _target: TextureTarget,
    _pname: GetTextureParameter,
    _params: *mut GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glIsTextureHandleResidentARB =
    unsafe extern "system" fn(_handle: GLuint64) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glDrawRangeElementsBaseVertexOES = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _start: GLuint,
    _end: GLuint,
    _count: GLsizei,
    _type: DrawElementsType,
    _indices: *const std::os::raw::c_void,
    _basevertex: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord4iv = unsafe extern "system" fn(_v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetQueryObjectiv =
    unsafe extern "system" fn(_id: GLuint, _pname: QueryObjectParameterName, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glTexEstimateMotionQCOM =
    unsafe extern "system" fn(_ref: GLuint, _target: GLuint, _output: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteAsyncMarkersSGIX = unsafe extern "system" fn(_marker: GLuint, _range: GLsizei);
#[allow(non_camel_case_types)]
pub type PFN_glSamplerParameterIuivEXT =
    unsafe extern "system" fn(_sampler: GLuint, _pname: SamplerParameterI, _param: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glUniform1i64ARB = unsafe extern "system" fn(_location: GLint, _x: GLint64);
#[allow(non_camel_case_types)]
pub type PFN_glUniformMatrix3x4fvNV = unsafe extern "system" fn(
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib1hNV = unsafe extern "system" fn(_index: GLuint, _x: GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glResumeTransformFeedback = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4Nbv = unsafe extern "system" fn(_index: GLuint, _v: *const GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI3i =
    unsafe extern "system" fn(_index: GLuint, _x: GLint, _y: GLint, _z: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribs3fvNV =
    unsafe extern "system" fn(_index: GLuint, _count: GLsizei, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexAttribfv = unsafe extern "system" fn(
    _index: GLuint,
    _pname: VertexAttribPropertyARB,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glReplacementCodeusvSUN = unsafe extern "system" fn(_code: *const GLushort);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord2hvNV = unsafe extern "system" fn(_v: *const GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glUniform4ui64NV = unsafe extern "system" fn(
    _location: GLint,
    _x: GLuint64EXT,
    _y: GLuint64EXT,
    _z: GLuint64EXT,
    _w: GLuint64EXT,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexArrayVertexAttribDivisorEXT =
    unsafe extern "system" fn(_vaobj: GLuint, _index: GLuint, _divisor: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glEdgeFlagFormatNV = unsafe extern "system" fn(_stride: GLsizei);
#[allow(non_camel_case_types)]
pub type PFN_glCompressedMultiTexSubImage2DEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _format: InternalFormat,
    _imageSize: GLsizei,
    _bits: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glEvalCoord2fv = unsafe extern "system" fn(_u: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteFencesAPPLE = unsafe extern "system" fn(_n: GLsizei, _fences: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glColor3b = unsafe extern "system" fn(_red: GLbyte, _green: GLbyte, _blue: GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexAttribIiv =
    unsafe extern "system" fn(_index: GLuint, _pname: VertexAttribEnum, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glPixelStoref =
    unsafe extern "system" fn(_pname: PixelStoreParameter, _param: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glConvolutionParameteri = unsafe extern "system" fn(
    _target: ConvolutionTarget,
    _pname: ConvolutionParameterEXT,
    _params: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord2f = unsafe extern "system" fn(_s: GLfloat, _t: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glPathColorGenNV = unsafe extern "system" fn(
    _color: PathColor,
    _genMode: PathGenMode,
    _colorFormat: PathColorFormat,
    _coeffs: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform3i64vARB =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLint64);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos2i = unsafe extern "system" fn(_x: GLint, _y: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos4iMESA =
    unsafe extern "system" fn(_x: GLint, _y: GLint, _z: GLint, _w: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glTextureColorMaskSGIS = unsafe extern "system" fn(
    _red: GLboolean,
    _green: GLboolean,
    _blue: GLboolean,
    _alpha: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glTransformFeedbackStreamAttribsNV = unsafe extern "system" fn(
    _count: GLsizei,
    _attribs: *const GLint,
    _nbuffers: GLsizei,
    _bufstreams: *const GLint,
    _bufferMode: GLenum,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertex2hNV = unsafe extern "system" fn(_x: GLhalfNV, _y: GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glGetFramebufferParameterivMESA = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _pname: FramebufferAttachmentParameterName,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream2iATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _x: GLint, _y: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glCreateMemoryObjectsEXT =
    unsafe extern "system" fn(_n: GLsizei, _memoryObjects: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glDebugMessageInsertAMD = unsafe extern "system" fn(
    _category: GLenum,
    _severity: DebugSeverity,
    _id: GLuint,
    _length: GLsizei,
    _buf: *const GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexP4ui = unsafe extern "system" fn(_type: VertexPointerType, _value: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetNamedFramebufferParameterfvAMD = unsafe extern "system" fn(
    _framebuffer: GLuint,
    _pname: GLenum,
    _numsamples: GLuint,
    _pixelindex: GLuint,
    _size: GLsizei,
    _values: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramParameterfvNV = unsafe extern "system" fn(
    _target: VertexAttribEnumNV,
    _index: GLuint,
    _pname: VertexAttribEnumNV,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGenFragmentShadersATI = unsafe extern "system" fn(_range: GLuint) -> GLuint;
#[allow(non_camel_case_types)]
pub type PFN_glMultiDrawElements = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _count: *const GLsizei,
    _type: DrawElementsType,
    _indices: *const *const std::os::raw::c_void,
    _drawcount: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertex4sv = unsafe extern "system" fn(_v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glDrawElementsInstanced = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _count: GLsizei,
    _type: DrawElementsType,
    _indices: *const std::os::raw::c_void,
    _instancecount: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glQueryMatrixxOES =
    unsafe extern "system" fn(_mantissa: *mut GLfixed, _exponent: *mut GLint) -> GLbitfield;
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos2f = unsafe extern "system" fn(_x: GLfloat, _y: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glMapBufferRange = unsafe extern "system" fn(
    _target: BufferTargetARB,
    _offset: GLintptr,
    _length: GLsizeiptr,
    _access: MapBufferAccessMask,
) -> *mut std::os::raw::c_void;
#[allow(non_camel_case_types)]
pub type PFN_glUniformHandleui64IMG = unsafe extern "system" fn(_location: GLint, _value: GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glGenPathsNV = unsafe extern "system" fn(_range: GLsizei) -> GLuint;
#[allow(non_camel_case_types)]
pub type PFN_glCompressedTextureSubImage3DEXT = unsafe extern "system" fn(
    _texture: GLuint,
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
    _bits: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glPopMatrix = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glUniform4iv =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixScaledEXT =
    unsafe extern "system" fn(_mode: MatrixMode, _x: GLdouble, _y: GLdouble, _z: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteFencesNV = unsafe extern "system" fn(_n: GLsizei, _fences: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetActiveSubroutineName = unsafe extern "system" fn(
    _program: GLuint,
    _shadertype: ShaderType,
    _index: GLuint,
    _bufSize: GLsizei,
    _length: *mut GLsizei,
    _name: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glListParameterivSGIX =
    unsafe extern "system" fn(_list: GLuint, _pname: ListParameterName, _params: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib3fNV =
    unsafe extern "system" fn(_index: GLuint, _x: GLfloat, _y: GLfloat, _z: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glValidateProgramPipelineEXT = unsafe extern "system" fn(_pipeline: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glFogCoordfvEXT = unsafe extern "system" fn(_coord: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glPathStencilDepthOffsetNV =
    unsafe extern "system" fn(_factor: GLfloat, _units: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glWaitSemaphoreEXT = unsafe extern "system" fn(
    _semaphore: GLuint,
    _numBufferBarriers: GLuint,
    _buffers: *const GLuint,
    _numTextureBarriers: GLuint,
    _textures: *const GLuint,
    _srcLayouts: *const TextureLayout,
);
#[allow(non_camel_case_types)]
pub type PFN_glEdgeFlagPointerEXT =
    unsafe extern "system" fn(_stride: GLsizei, _count: GLsizei, _pointer: *const GLboolean);
#[allow(non_camel_case_types)]
pub type PFN_glShadingRateSampleOrderNV = unsafe extern "system" fn(_order: GLenum);
#[allow(non_camel_case_types)]
pub type PFN_glTagSampleBufferSGIX = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glVertex3f = unsafe extern "system" fn(_x: GLfloat, _y: GLfloat, _z: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord3sv = unsafe extern "system" fn(_v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glVariantArrayObjectATI = unsafe extern "system" fn(
    _id: GLuint,
    _type: ScalarType,
    _stride: GLsizei,
    _buffer: GLuint,
    _offset: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glEnableiEXT = unsafe extern "system" fn(_target: EnableCap, _index: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertex3iv = unsafe extern "system" fn(_v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib3hNV =
    unsafe extern "system" fn(_index: GLuint, _x: GLhalfNV, _y: GLhalfNV, _z: GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glMultTransposeMatrixxOES = unsafe extern "system" fn(_m: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glIndexi = unsafe extern "system" fn(_c: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glLineWidthx = unsafe extern "system" fn(_width: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glBindBufferOffsetNV = unsafe extern "system" fn(
    _target: BufferTargetARB,
    _index: GLuint,
    _buffer: GLuint,
    _offset: GLintptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glPresentFrameKeyedNV = unsafe extern "system" fn(
    _video_slot: GLuint,
    _minPresentTime: GLuint64EXT,
    _beginPresentTimeId: GLuint,
    _presentDurationId: GLuint,
    _type: GLenum,
    _target0: GLenum,
    _fill0: GLuint,
    _key0: GLuint,
    _target1: GLenum,
    _fill1: GLuint,
    _key1: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGenSymbolsEXT = unsafe extern "system" fn(
    _datatype: DataTypeEXT,
    _storagetype: VertexShaderStorageTypeEXT,
    _range: ParameterRangeEXT,
    _components: GLuint,
) -> GLuint;
#[allow(non_camel_case_types)]
pub type PFN_glClearTexSubImageEXT = unsafe extern "system" fn(
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
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord1fvARB =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord4xvOES =
    unsafe extern "system" fn(_texture: TextureUnit, _coords: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glGetPerfMonitorCounterStringAMD = unsafe extern "system" fn(
    _group: GLuint,
    _counter: GLuint,
    _bufSize: GLsizei,
    _length: *mut GLsizei,
    _counterString: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glPolygonStipple = unsafe extern "system" fn(_mask: *const GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glUniform1ui64vNV =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLuint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glSamplerParameterIivOES =
    unsafe extern "system" fn(_sampler: GLuint, _pname: SamplerParameterI, _param: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiDrawElementsIndirectEXT = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _type: DrawElementsType,
    _indirect: *const std::os::raw::c_void,
    _drawcount: GLsizei,
    _stride: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glPointParameteriv =
    unsafe extern "system" fn(_pname: PointParameterNameARB, _params: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetMaterialxOES =
    unsafe extern "system" fn(_face: MaterialFace, _pname: MaterialParameter, _param: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glGetColorTableParameteriv = unsafe extern "system" fn(
    _target: ColorTableTarget,
    _pname: GetColorTableParameterPNameSGI,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexImage3DEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _level: GLint,
    _internalformat: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _border: GLint,
    _format: PixelFormat,
    _type: PixelType,
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformui64vNV = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLuint64EXT,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord2iv = unsafe extern "system" fn(_v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL1dvEXT = unsafe extern "system" fn(_index: GLuint, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glNormal3sv = unsafe extern "system" fn(_v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glCopyTextureSubImage1DEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _level: GLint,
    _xoffset: GLint,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glScissorIndexed = unsafe extern "system" fn(
    _index: GLuint,
    _left: GLint,
    _bottom: GLint,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glTranslated = unsafe extern "system" fn(_x: GLdouble, _y: GLdouble, _z: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glShaderStorageBlockBinding = unsafe extern "system" fn(
    _program: GLuint,
    _storageBlockIndex: GLuint,
    _storageBlockBinding: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3ubv = unsafe extern "system" fn(_v: *const GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glSignalSemaphoreEXT = unsafe extern "system" fn(
    _semaphore: GLuint,
    _numBufferBarriers: GLuint,
    _buffers: *const GLuint,
    _numTextureBarriers: GLuint,
    _textures: *const GLuint,
    _dstLayouts: *const TextureLayout,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform2i64NV =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _x: GLint64EXT, _y: GLint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoordFormatNV =
    unsafe extern "system" fn(_size: GLint, _type: GLenum, _stride: GLsizei);
#[allow(non_camel_case_types)]
pub type PFN_glGetTransformFeedbackVaryingEXT = unsafe extern "system" fn(
    _program: GLuint,
    _index: GLuint,
    _bufSize: GLsizei,
    _length: *mut GLsizei,
    _size: *mut GLsizei,
    _type: *mut AttributeType,
    _name: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureStorageMem3DMultisampleEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _samples: GLsizei,
    _internalFormat: SizedInternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _fixedSampleLocations: GLboolean,
    _memory: GLuint,
    _offset: GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos2ivMESA = unsafe extern "system" fn(_v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetFramebufferAttachmentParameterivEXT = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _attachment: FramebufferAttachment,
    _pname: FramebufferAttachmentParameterName,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform4uivEXT =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetColorTableParameterfvSGI = unsafe extern "system" fn(
    _target: ColorTableTargetSGI,
    _pname: GetColorTableParameterPNameSGI,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetImageHandleNV = unsafe extern "system" fn(
    _texture: GLuint,
    _level: GLint,
    _layered: GLboolean,
    _layer: GLint,
    _format: PixelFormat,
) -> GLuint64;
#[allow(non_camel_case_types)]
pub type PFN_glMakeBufferNonResidentNV = unsafe extern "system" fn(_target: GLenum);
#[allow(non_camel_case_types)]
pub type PFN_glDeletePerfQueryINTEL = unsafe extern "system" fn(_queryHandle: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetFragDataIndex =
    unsafe extern "system" fn(_program: GLuint, _name: *const GLchar) -> GLint;
#[allow(non_camel_case_types)]
pub type PFN_glRectiv = unsafe extern "system" fn(_v1: *const GLint, _v2: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glStencilFillPathInstancedNV = unsafe extern "system" fn(
    _numPaths: GLsizei,
    _pathNameType: PathElementType,
    _paths: *const std::os::raw::c_void,
    _pathBase: GLuint,
    _fillMode: PathFillMode,
    _mask: GLuint,
    _transformType: PathTransformType,
    _transformValues: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord3bvOES = unsafe extern "system" fn(_coords: *const GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoordP4uiv =
    unsafe extern "system" fn(_type: TexCoordPointerType, _coords: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribFormat = unsafe extern "system" fn(
    _attribindex: GLuint,
    _size: GLint,
    _type: VertexAttribType,
    _normalized: GLboolean,
    _relativeoffset: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glCoverFillPathNV =
    unsafe extern "system" fn(_path: GLuint, _coverMode: PathCoverMode);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramStageiv = unsafe extern "system" fn(
    _program: GLuint,
    _shadertype: ShaderType,
    _pname: ProgramStagePName,
    _values: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL4ui64vNV =
    unsafe extern "system" fn(_index: GLuint, _v: *const GLuint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribs2svNV =
    unsafe extern "system" fn(_index: GLuint, _count: GLsizei, _v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribs2fvNV =
    unsafe extern "system" fn(_index: GLuint, _count: GLsizei, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glBindMaterialParameterEXT =
    unsafe extern "system" fn(_face: MaterialFace, _value: MaterialParameter) -> GLuint;
#[allow(non_camel_case_types)]
pub type PFN_glGenTextures = unsafe extern "system" fn(_n: GLsizei, _textures: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glBufferData = unsafe extern "system" fn(
    _target: BufferTargetARB,
    _size: GLsizeiptr,
    _data: *const std::os::raw::c_void,
    _usage: BufferUsageARB,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetCombinerInputParameterivNV = unsafe extern "system" fn(
    _stage: CombinerStageNV,
    _portion: CombinerPortionNV,
    _variable: CombinerVariableNV,
    _pname: CombinerParameterNV,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glCopyBufferSubDataNV = unsafe extern "system" fn(
    _readTarget: CopyBufferSubDataTarget,
    _writeTarget: CopyBufferSubDataTarget,
    _readOffset: GLintptr,
    _writeOffset: GLintptr,
    _size: GLsizeiptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glReplacementCodeuiTexCoord2fVertex3fvSUN =
    unsafe extern "system" fn(_rc: *const GLuint, _tc: *const GLfloat, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetError = unsafe extern "system" fn() -> GLenum;
#[allow(non_camel_case_types)]
pub type PFN_glColor4hvNV = unsafe extern "system" fn(_v: *const GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glGetnUniformivEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _bufSize: GLsizei,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glIsTexture = unsafe extern "system" fn(_texture: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glSamplerParameterfv =
    unsafe extern "system" fn(_sampler: GLuint, _pname: SamplerParameterF, _param: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribP1ui = unsafe extern "system" fn(
    _index: GLuint,
    _type: VertexAttribPointerType,
    _normalized: GLboolean,
    _value: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glColorMask = unsafe extern "system" fn(
    _red: GLboolean,
    _green: GLboolean,
    _blue: GLboolean,
    _alpha: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureParameterIiv = unsafe extern "system" fn(
    _texture: GLuint,
    _pname: TextureParameterName,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetUniformOffsetEXT =
    unsafe extern "system" fn(_program: GLuint, _location: GLint) -> GLintptr;
#[allow(non_camel_case_types)]
pub type PFN_glGlobalAlphaFactorsSUN = unsafe extern "system" fn(_factor: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glFrontFace = unsafe extern "system" fn(_mode: FrontFaceDirection);
#[allow(non_camel_case_types)]
pub type PFN_glVariantdvEXT = unsafe extern "system" fn(_id: GLuint, _addr: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord2fColor4fNormal3fVertex3fSUN = unsafe extern "system" fn(
    _s: GLfloat,
    _t: GLfloat,
    _r: GLfloat,
    _g: GLfloat,
    _b: GLfloat,
    _a: GLfloat,
    _nx: GLfloat,
    _ny: GLfloat,
    _nz: GLfloat,
    _x: GLfloat,
    _y: GLfloat,
    _z: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteVertexArraysAPPLE =
    unsafe extern "system" fn(_n: GLsizei, _arrays: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetBooleani_v =
    unsafe extern "system" fn(_target: BufferTargetARB, _index: GLuint, _data: *mut GLboolean);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteProgram = unsafe extern "system" fn(_program: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix2x4dvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glEndOcclusionQueryNV = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexAttribLui64vARB =
    unsafe extern "system" fn(_index: GLuint, _pname: VertexAttribEnum, _params: *mut GLuint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform2i64ARB =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _x: GLint64, _y: GLint64);
#[allow(non_camel_case_types)]
pub type PFN_glGetPerfQueryDataINTEL = unsafe extern "system" fn(
    _queryHandle: GLuint,
    _flags: GLuint,
    _dataSize: GLsizei,
    _data: *mut std::os::raw::c_void,
    _bytesWritten: *mut GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertex4bOES =
    unsafe extern "system" fn(_x: GLbyte, _y: GLbyte, _z: GLbyte, _w: GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI3iEXT =
    unsafe extern "system" fn(_index: GLuint, _x: GLint, _y: GLint, _z: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glBindTexture = unsafe extern "system" fn(_target: TextureTarget, _texture: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glPixelDataRangeNV = unsafe extern "system" fn(
    _target: PixelDataRangeTargetNV,
    _length: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetTexEnvfv = unsafe extern "system" fn(
    _target: TextureEnvTarget,
    _pname: TextureEnvParameter,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribs3dvNV =
    unsafe extern "system" fn(_index: GLuint, _count: GLsizei, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos3d = unsafe extern "system" fn(_x: GLdouble, _y: GLdouble, _z: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glGetTextureParameterIuiv =
    unsafe extern "system" fn(_texture: GLuint, _pname: GetTextureParameter, _params: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glBitmap = unsafe extern "system" fn(
    _width: GLsizei,
    _height: GLsizei,
    _xorig: GLfloat,
    _yorig: GLfloat,
    _xmove: GLfloat,
    _ymove: GLfloat,
    _bitmap: *const GLubyte,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetBufferParameteriv = unsafe extern "system" fn(
    _target: BufferTargetARB,
    _pname: BufferPNameARB,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glTestFenceNV = unsafe extern "system" fn(_fence: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glGetUniformui64vARB =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _params: *mut GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib3sNV =
    unsafe extern "system" fn(_index: GLuint, _x: GLshort, _y: GLshort, _z: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glPrimitiveBoundingBoxARB = unsafe extern "system" fn(
    _minX: GLfloat,
    _minY: GLfloat,
    _minZ: GLfloat,
    _minW: GLfloat,
    _maxX: GLfloat,
    _maxY: GLfloat,
    _maxZ: GLfloat,
    _maxW: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos4xOES =
    unsafe extern "system" fn(_x: GLfixed, _y: GLfixed, _z: GLfixed, _w: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix2x4fv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos3fMESA = unsafe extern "system" fn(_x: GLfloat, _y: GLfloat, _z: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glBindProgramARB = unsafe extern "system" fn(_target: ProgramTarget, _program: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glIsImageHandleResidentARB = unsafe extern "system" fn(_handle: GLuint64) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glMapObjectBufferATI =
    unsafe extern "system" fn(_buffer: GLuint) -> *mut std::os::raw::c_void;
#[allow(non_camel_case_types)]
pub type PFN_glEnable = unsafe extern "system" fn(_cap: EnableCap);
#[allow(non_camel_case_types)]
pub type PFN_glTexPageCommitmentMemNV = unsafe extern "system" fn(
    _target: TextureTarget,
    _layer: GLint,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _zoffset: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _memory: GLuint,
    _offset: GLuint64,
    _commit: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixIndexuivARB = unsafe extern "system" fn(_size: GLint, _indices: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertex3sv = unsafe extern "system" fn(_v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glVertex2bvOES = unsafe extern "system" fn(_coords: *const GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glFinishObjectAPPLE =
    unsafe extern "system" fn(_object: ObjectTypeAPPLE, _name: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glIsQueryARB = unsafe extern "system" fn(_id: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glGetConvolutionParameterivEXT = unsafe extern "system" fn(
    _target: ConvolutionTargetEXT,
    _pname: ConvolutionParameterEXT,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetNextPerfQueryIdINTEL =
    unsafe extern "system" fn(_queryId: GLuint, _nextQueryId: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glDrawArraysIndirect =
    unsafe extern "system" fn(_mode: PrimitiveType, _indirect: *const std::os::raw::c_void);
#[allow(non_camel_case_types)]
pub type PFN_glGetTextureImageEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _level: GLint,
    _format: PixelFormat,
    _type: PixelType,
    _pixels: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexPageCommitmentARB = unsafe extern "system" fn(
    _target: GLenum,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _zoffset: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _commit: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteRenderbuffersOES =
    unsafe extern "system" fn(_n: GLsizei, _renderbuffers: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glPathSubCoordsNV = unsafe extern "system" fn(
    _path: GLuint,
    _coordStart: GLsizei,
    _numCoords: GLsizei,
    _coordType: PathCoordType,
    _coords: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform3iv =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertex4d =
    unsafe extern "system" fn(_x: GLdouble, _y: GLdouble, _z: GLdouble, _w: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos2s = unsafe extern "system" fn(_x: GLshort, _y: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glUniform2fARB =
    unsafe extern "system" fn(_location: GLint, _v0: GLfloat, _v1: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glDrawTransformFeedback = unsafe extern "system" fn(_mode: PrimitiveType, _id: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glDebugMessageCallbackARB =
    unsafe extern "system" fn(_callback: GLDEBUGPROCARB, _userParam: *const std::os::raw::c_void);
#[allow(non_camel_case_types)]
pub type PFN_glTexImage3DMultisample = unsafe extern "system" fn(
    _target: TextureTarget,
    _samples: GLsizei,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _fixedsamplelocations: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glMulticastViewportPositionWScaleNVX =
    unsafe extern "system" fn(_gpu: GLuint, _index: GLuint, _xcoeff: GLfloat, _ycoeff: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glCopyImageSubDataEXT = unsafe extern "system" fn(
    _srcName: GLuint,
    _srcTarget: CopyBufferSubDataTarget,
    _srcLevel: GLint,
    _srcX: GLint,
    _srcY: GLint,
    _srcZ: GLint,
    _dstName: GLuint,
    _dstTarget: CopyBufferSubDataTarget,
    _dstLevel: GLint,
    _dstX: GLint,
    _dstY: GLint,
    _dstZ: GLint,
    _srcWidth: GLsizei,
    _srcHeight: GLsizei,
    _srcDepth: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glIsList = unsafe extern "system" fn(_list: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glExtGetTexSubImageQCOM = unsafe extern "system" fn(
    _target: GLenum,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _zoffset: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _format: PixelFormat,
    _type: PixelType,
    _texels: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexGenxOES = unsafe extern "system" fn(
    _coord: TextureCoordName,
    _pname: TextureGenParameter,
    _param: GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4dvNV = unsafe extern "system" fn(_index: GLuint, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glLightxv =
    unsafe extern "system" fn(_light: LightName, _pname: LightParameter, _params: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glNewList = unsafe extern "system" fn(_list: GLuint, _mode: ListMode);
#[allow(non_camel_case_types)]
pub type PFN_glMakeTextureHandleNonResidentNV = unsafe extern "system" fn(_handle: GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glGetnUniformiv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _bufSize: GLsizei,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glNamedFramebufferTextureLayerEXT = unsafe extern "system" fn(
    _framebuffer: GLuint,
    _attachment: FramebufferAttachment,
    _texture: GLuint,
    _level: GLint,
    _layer: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glNamedProgramLocalParameterI4ivEXT = unsafe extern "system" fn(
    _program: GLuint,
    _target: ProgramTarget,
    _index: GLuint,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformHandleui64vIMG = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _values: *const GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexBufferRangeOES = unsafe extern "system" fn(
    _target: TextureTarget,
    _internalformat: SizedInternalFormat,
    _buffer: GLuint,
    _offset: GLintptr,
    _size: GLsizeiptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glLightf =
    unsafe extern "system" fn(_light: LightName, _pname: LightParameter, _param: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord2dv =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib1f = unsafe extern "system" fn(_index: GLuint, _x: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribDivisor = unsafe extern "system" fn(_index: GLuint, _divisor: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glBufferAttachMemoryNV =
    unsafe extern "system" fn(_target: BufferTargetARB, _memory: GLuint, _offset: GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glGetMinmax = unsafe extern "system" fn(
    _target: MinmaxTargetEXT,
    _reset: GLboolean,
    _format: PixelFormat,
    _type: PixelType,
    _values: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glPathParameteriNV =
    unsafe extern "system" fn(_path: GLuint, _pname: PathParameter, _value: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glDrawElementsIndirect = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _type: DrawElementsType,
    _indirect: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glUnmapNamedBufferEXT = unsafe extern "system" fn(_buffer: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glVertexArraySecondaryColorOffsetEXT = unsafe extern "system" fn(
    _vaobj: GLuint,
    _buffer: GLuint,
    _size: GLint,
    _type: ColorPointerType,
    _stride: GLsizei,
    _offset: GLintptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4d = unsafe extern "system" fn(
    _index: GLuint,
    _x: GLdouble,
    _y: GLdouble,
    _z: GLdouble,
    _w: GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetListParameterivSGIX =
    unsafe extern "system" fn(_list: GLuint, _pname: ListParameterName, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetnUniformivKHR = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _bufSize: GLsizei,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glIndexf = unsafe extern "system" fn(_c: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glCreateProgramObjectARB = unsafe extern "system" fn() -> GLhandleARB;
#[allow(non_camel_case_types)]
pub type PFN_glExtGetBuffersQCOM =
    unsafe extern "system" fn(_buffers: *mut GLuint, _maxBuffers: GLint, _numBuffers: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glDepthRangeArrayfvOES =
    unsafe extern "system" fn(_first: GLuint, _count: GLsizei, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos2iv = unsafe extern "system" fn(_v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetObjectBufferfvATI =
    unsafe extern "system" fn(_buffer: GLuint, _pname: ArrayObjectPNameATI, _params: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glSpecializeShader = unsafe extern "system" fn(
    _shader: GLuint,
    _pEntryPoint: *const GLchar,
    _numSpecializationConstants: GLuint,
    _pConstantIndex: *const GLuint,
    _pConstantValue: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos4s =
    unsafe extern "system" fn(_x: GLshort, _y: GLshort, _z: GLshort, _w: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexAttribLi64vNV =
    unsafe extern "system" fn(_index: GLuint, _pname: VertexAttribEnum, _params: *mut GLint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos2dARB = unsafe extern "system" fn(_x: GLdouble, _y: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glReleaseKeyedMutexWin32EXT =
    unsafe extern "system" fn(_memory: GLuint, _key: GLuint64) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramParameterdvNV = unsafe extern "system" fn(
    _target: VertexAttribEnumNV,
    _index: GLuint,
    _pname: VertexAttribEnumNV,
    _params: *mut GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glClearColorIiEXT =
    unsafe extern "system" fn(_red: GLint, _green: GLint, _blue: GLint, _alpha: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glStateCaptureNV = unsafe extern "system" fn(_state: GLuint, _mode: GLenum);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform2i64vNV = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLint64EXT,
);
#[allow(non_camel_case_types)]
pub type PFN_glMap1f = unsafe extern "system" fn(
    _target: MapTarget,
    _u1: GLfloat,
    _u2: GLfloat,
    _stride: GLint,
    _order: GLint,
    _points: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glIsNameAMD =
    unsafe extern "system" fn(_identifier: GLenum, _name: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glPrimitiveBoundingBox = unsafe extern "system" fn(
    _minX: GLfloat,
    _minY: GLfloat,
    _minZ: GLfloat,
    _minW: GLfloat,
    _maxX: GLfloat,
    _maxY: GLfloat,
    _maxZ: GLfloat,
    _maxW: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexStorageMem2DMultisampleEXT = unsafe extern "system" fn(
    _target: TextureTarget,
    _samples: GLsizei,
    _internalFormat: SizedInternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _fixedSampleLocations: GLboolean,
    _memory: GLuint,
    _offset: GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4hNV = unsafe extern "system" fn(
    _index: GLuint,
    _x: GLhalfNV,
    _y: GLhalfNV,
    _z: GLhalfNV,
    _w: GLhalfNV,
);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos3fARB = unsafe extern "system" fn(_x: GLfloat, _y: GLfloat, _z: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribPointerNV = unsafe extern "system" fn(
    _index: GLuint,
    _fsize: GLint,
    _type: VertexAttribEnumNV,
    _stride: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glStencilThenCoverFillPathInstancedNV = unsafe extern "system" fn(
    _numPaths: GLsizei,
    _pathNameType: GLenum,
    _paths: *const std::os::raw::c_void,
    _pathBase: GLuint,
    _fillMode: GLenum,
    _mask: GLuint,
    _coverMode: GLenum,
    _transformType: GLenum,
    _transformValues: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glBlendColorEXT =
    unsafe extern "system" fn(_red: GLfloat, _green: GLfloat, _blue: GLfloat, _alpha: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glDrawBuffers = unsafe extern "system" fn(_n: GLsizei, _bufs: *const DrawBufferMode);
#[allow(non_camel_case_types)]
pub type PFN_glCreateProgressFenceNVX = unsafe extern "system" fn() -> GLuint;
#[allow(non_camel_case_types)]
pub type PFN_glRectxOES =
    unsafe extern "system" fn(_x1: GLfixed, _y1: GLfixed, _x2: GLfixed, _y2: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glPathMemoryGlyphIndexArrayNV = unsafe extern "system" fn(
    _firstPathName: GLuint,
    _fontTarget: GLenum,
    _fontSize: GLsizeiptr,
    _fontData: *const std::os::raw::c_void,
    _faceIndex: GLsizei,
    _firstGlyphIndex: GLuint,
    _numGlyphs: GLsizei,
    _pathParameterTemplate: GLuint,
    _emScale: GLfloat,
) -> GLenum;
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexAttribdv = unsafe extern "system" fn(
    _index: GLuint,
    _pname: VertexAttribPropertyARB,
    _params: *mut GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos2d = unsafe extern "system" fn(_x: GLdouble, _y: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexArrayVertexAttribBindingEXT =
    unsafe extern "system" fn(_vaobj: GLuint, _attribindex: GLuint, _bindingindex: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glDrawTexivOES = unsafe extern "system" fn(_coords: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribLFormat = unsafe extern "system" fn(
    _attribindex: GLuint,
    _size: GLint,
    _type: VertexAttribLType,
    _relativeoffset: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glInvalidateNamedFramebufferSubData = unsafe extern "system" fn(
    _framebuffer: GLuint,
    _numAttachments: GLsizei,
    _attachments: *const FramebufferAttachment,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glPointSizePointerOES = unsafe extern "system" fn(
    _type: GLenum,
    _stride: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGenFramebuffersEXT =
    unsafe extern "system" fn(_n: GLsizei, _framebuffers: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glCompressedTextureSubImage3D = unsafe extern "system" fn(
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
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetFloatIndexedvEXT =
    unsafe extern "system" fn(_target: GetPName, _index: GLuint, _data: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glResetMinmax = unsafe extern "system" fn(_target: MinmaxTargetEXT);
#[allow(non_camel_case_types)]
pub type PFN_glReplacementCodeuiNormal3fVertex3fSUN = unsafe extern "system" fn(
    _rc: GLuint,
    _nx: GLfloat,
    _ny: GLfloat,
    _nz: GLfloat,
    _x: GLfloat,
    _y: GLfloat,
    _z: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetBufferPointervARB = unsafe extern "system" fn(
    _target: BufferTargetARB,
    _pname: BufferPointerNameARB,
    _params: *mut *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform2ui64vARB =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glBindBufferBaseEXT =
    unsafe extern "system" fn(_target: BufferTargetARB, _index: GLuint, _buffer: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetQueryBufferObjectui64v = unsafe extern "system" fn(
    _id: GLuint,
    _buffer: GLuint,
    _pname: QueryObjectParameterName,
    _offset: GLintptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glLoadTransposeMatrixd = unsafe extern "system" fn(_m: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glAsyncCopyImageSubDataNVX = unsafe extern "system" fn(
    _waitSemaphoreCount: GLsizei,
    _waitSemaphoreArray: *const GLuint,
    _waitValueArray: *const GLuint64,
    _srcGpu: GLuint,
    _dstGpuMask: GLbitfield,
    _srcName: GLuint,
    _srcTarget: GLenum,
    _srcLevel: GLint,
    _srcX: GLint,
    _srcY: GLint,
    _srcZ: GLint,
    _dstName: GLuint,
    _dstTarget: GLenum,
    _dstLevel: GLint,
    _dstX: GLint,
    _dstY: GLint,
    _dstZ: GLint,
    _srcWidth: GLsizei,
    _srcHeight: GLsizei,
    _srcDepth: GLsizei,
    _signalSemaphoreCount: GLsizei,
    _signalSemaphoreArray: *const GLuint,
    _signalValueArray: *const GLuint64,
) -> GLuint;
#[allow(non_camel_case_types)]
pub type PFN_glGetTexParameterIivEXT = unsafe extern "system" fn(
    _target: TextureTarget,
    _pname: GetTextureParameter,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glTangent3fvEXT = unsafe extern "system" fn(_v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glCopyTexSubImage2DEXT = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glBindImageTextureEXT = unsafe extern "system" fn(
    _index: GLuint,
    _texture: GLuint,
    _level: GLint,
    _layered: GLboolean,
    _layer: GLint,
    _access: BufferAccessARB,
    _format: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glClearColorxOES =
    unsafe extern "system" fn(_red: GLfixed, _green: GLfixed, _blue: GLfixed, _alpha: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glIsProgramPipelineEXT = unsafe extern "system" fn(_pipeline: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glBindBuffersRange = unsafe extern "system" fn(
    _target: BufferTargetARB,
    _first: GLuint,
    _count: GLsizei,
    _buffers: *const GLuint,
    _offsets: *const GLintptr,
    _sizes: *const GLsizeiptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord2fvARB =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glPathGlyphRangeNV = unsafe extern "system" fn(
    _firstPathName: GLuint,
    _fontTarget: PathFontTarget,
    _fontName: *const std::os::raw::c_void,
    _fontStyle: PathFontStyle,
    _firstGlyph: GLuint,
    _numGlyphs: GLsizei,
    _handleMissingGlyphs: PathHandleMissingGlyphs,
    _pathParameterTemplate: GLuint,
    _emScale: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramBufferParametersfvNV = unsafe extern "system" fn(
    _target: ProgramTarget,
    _bindingIndex: GLuint,
    _wordIndex: GLuint,
    _count: GLsizei,
    _params: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform2ui =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _v0: GLuint, _v1: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glColorP3uiv =
    unsafe extern "system" fn(_type: ColorPointerType, _color: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glFogFuncSGIS = unsafe extern "system" fn(_n: GLsizei, _points: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetActiveAtomicCounterBufferiv = unsafe extern "system" fn(
    _program: GLuint,
    _bufferIndex: GLuint,
    _pname: AtomicCounterBufferPName,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib1s = unsafe extern "system" fn(_index: GLuint, _x: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteFramebuffersOES =
    unsafe extern "system" fn(_n: GLsizei, _framebuffers: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoordP1uiv =
    unsafe extern "system" fn(_type: TexCoordPointerType, _coords: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glEndQueryIndexed = unsafe extern "system" fn(_target: QueryTarget, _index: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glIsEnablediEXT =
    unsafe extern "system" fn(_target: EnableCap, _index: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glLightModelxvOES =
    unsafe extern "system" fn(_pname: LightModelParameter, _param: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord4iARB =
    unsafe extern "system" fn(_target: TextureUnit, _s: GLint, _t: GLint, _r: GLint, _q: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferTextureARB = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _attachment: FramebufferAttachment,
    _texture: GLuint,
    _level: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glLightiv =
    unsafe extern "system" fn(_light: LightName, _pname: LightParameter, _params: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexEnvfEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureEnvTarget,
    _pname: TextureEnvParameter,
    _param: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform4d = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _v0: GLdouble,
    _v1: GLdouble,
    _v2: GLdouble,
    _v3: GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glBlendEquationSeparatei = unsafe extern "system" fn(
    _buf: GLuint,
    _modeRGB: BlendEquationModeEXT,
    _modeAlpha: BlendEquationModeEXT,
);
#[allow(non_camel_case_types)]
pub type PFN_glRotatex =
    unsafe extern "system" fn(_angle: GLfixed, _x: GLfixed, _y: GLfixed, _z: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord4fvARB =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform1i64ARB =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _x: GLint64);
#[allow(non_camel_case_types)]
pub type PFN_glSamplerParameteri =
    unsafe extern "system" fn(_sampler: GLuint, _pname: SamplerParameterI, _param: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glScissorArrayvNV =
    unsafe extern "system" fn(_first: GLuint, _count: GLsizei, _v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glTangent3ivEXT = unsafe extern "system" fn(_v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glEnd = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glIsBuffer = unsafe extern "system" fn(_buffer: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glGetQueryBufferObjecti64v = unsafe extern "system" fn(
    _id: GLuint,
    _buffer: GLuint,
    _pname: QueryObjectParameterName,
    _offset: GLintptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetTexBumpParameterfvATI =
    unsafe extern "system" fn(_pname: GetTexBumpParameterATI, _param: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3hNV =
    unsafe extern "system" fn(_red: GLhalfNV, _green: GLhalfNV, _blue: GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord3s = unsafe extern "system" fn(_s: GLshort, _t: GLshort, _r: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glTexEstimateMotionRegionsQCOM =
    unsafe extern "system" fn(_ref: GLuint, _target: GLuint, _output: GLuint, _mask: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glUniform3iARB =
    unsafe extern "system" fn(_location: GLint, _v0: GLint, _v1: GLint, _v2: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glAccum = unsafe extern "system" fn(_op: AccumOp, _value: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexArrayVertexAttribLFormatEXT = unsafe extern "system" fn(
    _vaobj: GLuint,
    _attribindex: GLuint,
    _size: GLint,
    _type: VertexAttribLType,
    _relativeoffset: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib1dvARB = unsafe extern "system" fn(_index: GLuint, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glDrawMeshArraysSUN = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _first: GLint,
    _count: GLsizei,
    _width: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glIsAsyncMarkerSGIX = unsafe extern "system" fn(_marker: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glCompressedTexImage1D = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _border: GLint,
    _imageSize: GLsizei,
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord1bOES = unsafe extern "system" fn(_texture: TextureUnit, _s: GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glIndexPointerListIBM = unsafe extern "system" fn(
    _type: IndexPointerType,
    _stride: GLint,
    _pointer: *mut *const std::os::raw::c_void,
    _ptrstride: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glFenceSyncAPPLE =
    unsafe extern "system" fn(_condition: SyncCondition, _flags: SyncBehaviorFlags) -> GLsync;
#[allow(non_camel_case_types)]
pub type PFN_glNamedRenderbufferStorageMultisampleAdvancedAMD = unsafe extern "system" fn(
    _renderbuffer: GLuint,
    _samples: GLsizei,
    _storageSamples: GLsizei,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureNormalEXT = unsafe extern "system" fn(_mode: TextureNormalModeEXT);
#[allow(non_camel_case_types)]
pub type PFN_glTextureStorage3D = unsafe extern "system" fn(
    _texture: GLuint,
    _levels: GLsizei,
    _internalformat: SizedInternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform2ui64ARB =
    unsafe extern "system" fn(_location: GLint, _x: GLuint64, _y: GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glUniformMatrix3x2fv = unsafe extern "system" fn(
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glEvalCoord1xOES = unsafe extern "system" fn(_u: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glVertex2d = unsafe extern "system" fn(_x: GLdouble, _y: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib3fvNV = unsafe extern "system" fn(_index: GLuint, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream2ivATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _coords: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos3dvARB = unsafe extern "system" fn(_v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos3svARB = unsafe extern "system" fn(_v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexParameterivEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _pname: TextureParameterName,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glTrackMatrixNV = unsafe extern "system" fn(
    _target: VertexAttribEnumNV,
    _address: GLuint,
    _matrix: VertexAttribEnumNV,
    _transform: VertexAttribEnumNV,
);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3sEXT =
    unsafe extern "system" fn(_red: GLshort, _green: GLshort, _blue: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glTexPageCommitmentEXT = unsafe extern "system" fn(
    _target: GLenum,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _zoffset: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _commit: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glResumeTransformFeedbackNV = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glBeginConditionalRenderNVX = unsafe extern "system" fn(_id: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetCombinerStageParameterfvNV = unsafe extern "system" fn(
    _stage: CombinerStageNV,
    _pname: CombinerParameterNV,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glDepthRangef = unsafe extern "system" fn(_n: GLfloat, _f: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glDisableClientStateIndexedEXT =
    unsafe extern "system" fn(_array: EnableCap, _index: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3s =
    unsafe extern "system" fn(_red: GLshort, _green: GLshort, _blue: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glDepthFunc = unsafe extern "system" fn(_func: DepthFunction);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixLoad3x3fNV =
    unsafe extern "system" fn(_matrixMode: GLenum, _m: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glNormal3hNV = unsafe extern "system" fn(_nx: GLhalfNV, _ny: GLhalfNV, _nz: GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glMapVertexAttrib1fAPPLE = unsafe extern "system" fn(
    _index: GLuint,
    _size: GLuint,
    _u1: GLfloat,
    _u2: GLfloat,
    _stride: GLint,
    _order: GLint,
    _points: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGenAsyncMarkersSGIX = unsafe extern "system" fn(_range: GLsizei) -> GLuint;
#[allow(non_camel_case_types)]
pub type PFN_glGetQueryObjecti64vEXT =
    unsafe extern "system" fn(_id: GLuint, _pname: QueryObjectParameterName, _params: *mut GLint64);
#[allow(non_camel_case_types)]
pub type PFN_glLightModeliv =
    unsafe extern "system" fn(_pname: LightModelParameter, _params: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glSeparableFilter2D = unsafe extern "system" fn(
    _target: SeparableTargetEXT,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _format: PixelFormat,
    _type: PixelType,
    _row: *const std::os::raw::c_void,
    _column: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glPauseTransformFeedbackNV = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glPointSizex = unsafe extern "system" fn(_size: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glPushAttrib = unsafe extern "system" fn(_mask: AttribMask);
#[allow(non_camel_case_types)]
pub type PFN_glTextureSubImage3D = unsafe extern "system" fn(
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
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI4uiEXT =
    unsafe extern "system" fn(_index: GLuint, _x: GLuint, _y: GLuint, _z: GLuint, _w: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMapControlPointsNV = unsafe extern "system" fn(
    _target: EvalTargetNV,
    _index: GLuint,
    _type: MapTypeNV,
    _ustride: GLsizei,
    _vstride: GLsizei,
    _uorder: GLint,
    _vorder: GLint,
    _packed: GLboolean,
    _points: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib3dNV =
    unsafe extern "system" fn(_index: GLuint, _x: GLdouble, _y: GLdouble, _z: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glPixelTransferi =
    unsafe extern "system" fn(_pname: PixelTransferParameter, _param: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glDrawBuffersIndexedEXT =
    unsafe extern "system" fn(_n: GLint, _location: *const GLenum, _indices: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glDiscardFramebufferEXT = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _numAttachments: GLsizei,
    _attachments: *const InvalidateFramebufferAttachment,
);
#[allow(non_camel_case_types)]
pub type PFN_glClipControlEXT = unsafe extern "system" fn(_origin: GLenum, _depth: GLenum);
#[allow(non_camel_case_types)]
pub type PFN_glGetnPixelMapfv =
    unsafe extern "system" fn(_map: PixelMap, _bufSize: GLsizei, _values: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform2dvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform1i64vARB =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLint64);
#[allow(non_camel_case_types)]
pub type PFN_glVariantbvEXT = unsafe extern "system" fn(_id: GLuint, _addr: *const GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glTexGeniv = unsafe extern "system" fn(
    _coord: TextureCoordName,
    _pname: TextureGenParameter,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib2fvNV = unsafe extern "system" fn(_index: GLuint, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4usv = unsafe extern "system" fn(_index: GLuint, _v: *const GLushort);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL1dv = unsafe extern "system" fn(_index: GLuint, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glBufferStorageExternalEXT = unsafe extern "system" fn(
    _target: GLenum,
    _offset: GLintptr,
    _size: GLsizeiptr,
    _clientBuffer: GLeglClientBufferEXT,
    _flags: BufferStorageMask,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetUniformLocation =
    unsafe extern "system" fn(_program: GLuint, _name: *const GLchar) -> GLint;
#[allow(non_camel_case_types)]
pub type PFN_glBindFramebuffer =
    unsafe extern "system" fn(_target: FramebufferTarget, _framebuffer: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord4dARB = unsafe extern "system" fn(
    _target: TextureUnit,
    _s: GLdouble,
    _t: GLdouble,
    _r: GLdouble,
    _q: GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream3svATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _coords: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord1f = unsafe extern "system" fn(_s: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glBlendEquationSeparateOES =
    unsafe extern "system" fn(_modeRGB: BlendEquationModeEXT, _modeAlpha: BlendEquationModeEXT);
#[allow(non_camel_case_types)]
pub type PFN_glConvolutionFilter1DEXT = unsafe extern "system" fn(
    _target: ConvolutionTargetEXT,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _format: PixelFormat,
    _type: PixelType,
    _image: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glFogxOES = unsafe extern "system" fn(_pname: FogPName, _param: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glPointParameterxv =
    unsafe extern "system" fn(_pname: PointParameterNameARB, _params: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glTexGeniOES =
    unsafe extern "system" fn(_coord: TextureCoordName, _pname: TextureGenParameter, _param: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glConvolutionFilter2D = unsafe extern "system" fn(
    _target: ConvolutionTarget,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _format: PixelFormat,
    _type: PixelType,
    _image: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetClipPlanex =
    unsafe extern "system" fn(_plane: ClipPlaneName, _equation: *mut GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream4ivATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _coords: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetPathMetricsNV = unsafe extern "system" fn(
    _metricQueryMask: PathMetricMask,
    _numPaths: GLsizei,
    _pathNameType: PathElementType,
    _paths: *const std::os::raw::c_void,
    _pathBase: GLuint,
    _stride: GLsizei,
    _metrics: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos3fvARB = unsafe extern "system" fn(_v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glWindowRectanglesEXT =
    unsafe extern "system" fn(_mode: GLenum, _count: GLsizei, _box: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glDepthRangefOES = unsafe extern "system" fn(_n: GLclampf, _f: GLclampf);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferRenderbufferEXT = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _attachment: FramebufferAttachment,
    _renderbuffertarget: RenderbufferTarget,
    _renderbuffer: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos3dv = unsafe extern "system" fn(_v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glIsVertexAttribEnabledAPPLE =
    unsafe extern "system" fn(_index: GLuint, _pname: GLenum) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glCopyColorSubTableEXT = unsafe extern "system" fn(
    _target: ColorTableTarget,
    _start: GLsizei,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteMemoryObjectsEXT =
    unsafe extern "system" fn(_n: GLsizei, _memoryObjects: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMaterialf =
    unsafe extern "system" fn(_face: MaterialFace, _pname: MaterialParameter, _param: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glInitNames = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_TexStorageAttribs3DEXT = unsafe extern "system" fn(
    _target: TextureTarget,
    _levels: GLsizei,
    _internalformat: SizedInternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _attrib_list: *const std::os::raw::c_int,
);
#[allow(non_camel_case_types)]
pub type PFN_glColorPointervINTEL = unsafe extern "system" fn(
    _size: GLint,
    _type: VertexPointerType,
    _pointer: *mut *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramResourceiv = unsafe extern "system" fn(
    _program: GLuint,
    _programInterface: ProgramInterface,
    _index: GLuint,
    _propCount: GLsizei,
    _props: *const ProgramResourceProperty,
    _count: GLsizei,
    _length: *mut GLsizei,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexAttribivARB =
    unsafe extern "system" fn(_index: GLuint, _pname: VertexAttribPropertyARB, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetnSeparableFilterARB = unsafe extern "system" fn(
    _target: SeparableTargetEXT,
    _format: PixelFormat,
    _type: PixelType,
    _rowBufSize: GLsizei,
    _row: *mut std::os::raw::c_void,
    _columnBufSize: GLsizei,
    _column: *mut std::os::raw::c_void,
    _span: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform4uivEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos4d =
    unsafe extern "system" fn(_x: GLdouble, _y: GLdouble, _z: GLdouble, _w: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glGetMultiTexGendvEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _coord: TextureCoordName,
    _pname: TextureGenParameter,
    _params: *mut GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetUniformIndices = unsafe extern "system" fn(
    _program: GLuint,
    _uniformCount: GLsizei,
    _uniformNames: *const *const GLchar,
    _uniformIndices: *mut GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGenProgramsARB = unsafe extern "system" fn(_n: GLsizei, _programs: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3us =
    unsafe extern "system" fn(_red: GLushort, _green: GLushort, _blue: GLushort);
#[allow(non_camel_case_types)]
pub type PFN_glBlendColor =
    unsafe extern "system" fn(_red: GLfloat, _green: GLfloat, _blue: GLfloat, _alpha: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glClearDepthfOES = unsafe extern "system" fn(_depth: GLclampf);
#[allow(non_camel_case_types)]
pub type PFN_glGetColorTableParameterivSGI = unsafe extern "system" fn(
    _target: ColorTableTargetSGI,
    _pname: GetColorTableParameterPNameSGI,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetNamedBufferParameterui64vNV =
    unsafe extern "system" fn(_buffer: GLuint, _pname: BufferPNameARB, _params: *mut GLuint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramSubroutineParameteruivNV =
    unsafe extern "system" fn(_target: GLenum, _index: GLuint, _param: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glStencilThenCoverStrokePathInstancedNV = unsafe extern "system" fn(
    _numPaths: GLsizei,
    _pathNameType: GLenum,
    _paths: *const std::os::raw::c_void,
    _pathBase: GLuint,
    _reference: GLint,
    _mask: GLuint,
    _coverMode: GLenum,
    _transformType: GLenum,
    _transformValues: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glAccumxOES = unsafe extern "system" fn(_op: GLenum, _value: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord1hvNV = unsafe extern "system" fn(_v: *const GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glTexSubImage3D = unsafe extern "system" fn(
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
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureStorage2DMultisampleEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _samples: GLsizei,
    _internalformat: SizedInternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _fixedsamplelocations: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexArrayVertexAttribOffsetEXT = unsafe extern "system" fn(
    _vaobj: GLuint,
    _buffer: GLuint,
    _index: GLuint,
    _size: GLint,
    _type: VertexAttribPointerType,
    _normalized: GLboolean,
    _stride: GLsizei,
    _offset: GLintptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribDivisorARB = unsafe extern "system" fn(_index: GLuint, _divisor: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetTexParameterIuivOES = unsafe extern "system" fn(
    _target: TextureTarget,
    _pname: GetTextureParameter,
    _params: *mut GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord4svARB =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glFlushStaticDataIBM = unsafe extern "system" fn(_target: GLenum);
#[allow(non_camel_case_types)]
pub type PFN_glTextureStorageMem3DEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _levels: GLsizei,
    _internalFormat: SizedInternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _memory: GLuint,
    _offset: GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glRenderbufferStorageMultisampleAPPLE = unsafe extern "system" fn(
    _target: RenderbufferTarget,
    _samples: GLsizei,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glNamedProgramLocalParameter4fEXT = unsafe extern "system" fn(
    _program: GLuint,
    _target: ProgramTarget,
    _index: GLuint,
    _x: GLfloat,
    _y: GLfloat,
    _z: GLfloat,
    _w: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glCoverageMaskNV = unsafe extern "system" fn(_mask: GLboolean);
#[allow(non_camel_case_types)]
pub type PFN_glGetTextureParameterfvEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _pname: GetTextureParameter,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform2i =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _v0: GLint, _v1: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glMemoryBarrier = unsafe extern "system" fn(_barriers: MemoryBarrierMask);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3sv = unsafe extern "system" fn(_v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glVertexArrayAttribBinding =
    unsafe extern "system" fn(_vaobj: GLuint, _attribindex: GLuint, _bindingindex: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib2fvARB = unsafe extern "system" fn(_index: GLuint, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib3d =
    unsafe extern "system" fn(_index: GLuint, _x: GLdouble, _y: GLdouble, _z: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glFogCoordd = unsafe extern "system" fn(_coord: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexWeightfvEXT = unsafe extern "system" fn(_weight: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferSamplePositionsfvAMD = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _numsamples: GLuint,
    _pixelindex: GLuint,
    _values: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glAlphaFuncxOES = unsafe extern "system" fn(_func: AlphaFunction, _ref: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos4dvMESA = unsafe extern "system" fn(_v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glColorFormatNV =
    unsafe extern "system" fn(_size: GLint, _type: GLenum, _stride: GLsizei);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform2ui64vNV = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLuint64EXT,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI4ivEXT = unsafe extern "system" fn(_index: GLuint, _v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetMapfv =
    unsafe extern "system" fn(_target: MapTarget, _query: GetMapQuery, _v: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVDPAUUnmapSurfacesNV =
    unsafe extern "system" fn(_numSurface: GLsizei, _surfaces: *const GLvdpauSurfaceNV);
#[allow(non_camel_case_types)]
pub type PFN_glEvalCoord1xvOES = unsafe extern "system" fn(_coords: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glUniform1fv =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glMapNamedBufferRange = unsafe extern "system" fn(
    _buffer: GLuint,
    _offset: GLintptr,
    _length: GLsizeiptr,
    _access: MapBufferAccessMask,
) -> *mut std::os::raw::c_void;
#[allow(non_camel_case_types)]
pub type PFN_glIsSampler = unsafe extern "system" fn(_sampler: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glBinormal3sEXT = unsafe extern "system" fn(_bx: GLshort, _by: GLshort, _bz: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexAttribPointervNV = unsafe extern "system" fn(
    _index: GLuint,
    _pname: VertexAttribEnumNV,
    _pointer: *mut *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramPipelineInfoLogEXT = unsafe extern "system" fn(
    _pipeline: GLuint,
    _bufSize: GLsizei,
    _length: *mut GLsizei,
    _infoLog: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetnMapiv = unsafe extern "system" fn(
    _target: MapTarget,
    _query: MapQuery,
    _bufSize: GLsizei,
    _v: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform2fvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform2i = unsafe extern "system" fn(_location: GLint, _v0: GLint, _v1: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetStringi =
    unsafe extern "system" fn(_name: StringName, _index: GLuint) -> *const GLubyte;
#[allow(non_camel_case_types)]
pub type PFN_glGetTexBumpParameterivATI =
    unsafe extern "system" fn(_pname: GetTexBumpParameterATI, _param: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetTextureParameterIuivEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _pname: GetTextureParameter,
    _params: *mut GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform3uiv =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glListParameteriSGIX =
    unsafe extern "system" fn(_list: GLuint, _pname: ListParameterName, _param: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4s =
    unsafe extern "system" fn(_index: GLuint, _x: GLshort, _y: GLshort, _z: GLshort, _w: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glInvalidateTexImage = unsafe extern "system" fn(_texture: GLuint, _level: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glBlendFunciEXT =
    unsafe extern "system" fn(_buf: GLuint, _src: BlendingFactor, _dst: BlendingFactor);
#[allow(non_camel_case_types)]
pub type PFN_glVertex2iv = unsafe extern "system" fn(_v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glUniformui64NV = unsafe extern "system" fn(_location: GLint, _value: GLuint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glBlendEquationIndexedAMD =
    unsafe extern "system" fn(_buf: GLuint, _mode: BlendEquationModeEXT);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix3x2dv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord2d = unsafe extern "system" fn(_s: GLdouble, _t: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glPrioritizeTextures =
    unsafe extern "system" fn(_n: GLsizei, _textures: *const GLuint, _priorities: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos2ivARB = unsafe extern "system" fn(_v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertex2xOES = unsafe extern "system" fn(_x: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos2fvARB = unsafe extern "system" fn(_v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glClearTexSubImage = unsafe extern "system" fn(
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
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexFilterFuncSGIS = unsafe extern "system" fn(
    _target: TextureTarget,
    _filter: TextureFilterSGIS,
    _n: GLsizei,
    _weights: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glAlphaFragmentOp2ATI = unsafe extern "system" fn(
    _op: FragmentOpATI,
    _dst: GLuint,
    _dstMod: GLuint,
    _arg1: GLuint,
    _arg1Rep: GLuint,
    _arg1Mod: GLuint,
    _arg2: GLuint,
    _arg2Rep: GLuint,
    _arg2Mod: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertex4xvOES = unsafe extern "system" fn(_coords: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glGetPerfCounterInfoINTEL = unsafe extern "system" fn(
    _queryId: GLuint,
    _counterId: GLuint,
    _counterNameLength: GLuint,
    _counterName: *mut GLchar,
    _counterDescLength: GLuint,
    _counterDesc: *mut GLchar,
    _counterOffset: *mut GLuint,
    _counterDataSize: *mut GLuint,
    _counterTypeEnum: *mut GLuint,
    _counterDataTypeEnum: *mut GLuint,
    _rawCounterMaxValue: *mut GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glBlendEquationSeparateiARB = unsafe extern "system" fn(
    _buf: GLuint,
    _modeRGB: BlendEquationModeEXT,
    _modeAlpha: BlendEquationModeEXT,
);
#[allow(non_camel_case_types)]
pub type PFN_glDispatchComputeIndirect = unsafe extern "system" fn(_indirect: GLintptr);
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexArrayiv =
    unsafe extern "system" fn(_vaobj: GLuint, _pname: VertexArrayPName, _param: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glUniformMatrix3x2dv = unsafe extern "system" fn(
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetMaterialxvOES = unsafe extern "system" fn(
    _face: MaterialFace,
    _pname: MaterialParameter,
    _params: *mut GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertex4i = unsafe extern "system" fn(_x: GLint, _y: GLint, _z: GLint, _w: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetDetailTexFuncSGIS =
    unsafe extern "system" fn(_target: TextureTarget, _points: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glMultTransposeMatrixd = unsafe extern "system" fn(_m: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord2fColor4fNormal3fVertex3fvSUN = unsafe extern "system" fn(
    _tc: *const GLfloat,
    _c: *const GLfloat,
    _n: *const GLfloat,
    _v: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureViewOES = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _origtexture: GLuint,
    _internalformat: SizedInternalFormat,
    _minlevel: GLuint,
    _numlevels: GLuint,
    _minlayer: GLuint,
    _numlayers: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib1sNV = unsafe extern "system" fn(_index: GLuint, _x: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoordP2uiv =
    unsafe extern "system" fn(_type: TexCoordPointerType, _coords: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glDisableDriverControlQCOM = unsafe extern "system" fn(_driverControl: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixLoad3x2fNV =
    unsafe extern "system" fn(_matrixMode: GLenum, _m: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glTexturePageCommitmentEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _zoffset: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _commit: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetnUniformivARB = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _bufSize: GLsizei,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos4dMESA =
    unsafe extern "system" fn(_x: GLdouble, _y: GLdouble, _z: GLdouble, _w: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glGetTranslatedShaderSourceANGLE = unsafe extern "system" fn(
    _shader: GLuint,
    _bufSize: GLsizei,
    _length: *mut GLsizei,
    _source: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord1sARB = unsafe extern "system" fn(_target: TextureUnit, _s: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform1fv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glSpriteParameterfvSGIX =
    unsafe extern "system" fn(_pname: SpriteParameterNameSGIX, _params: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glUniform4iARB =
    unsafe extern "system" fn(_location: GLint, _v0: GLint, _v1: GLint, _v2: GLint, _v3: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertex3xvOES = unsafe extern "system" fn(_coords: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glBeginPerfMonitorAMD = unsafe extern "system" fn(_monitor: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glPixelZoomxOES = unsafe extern "system" fn(_xfactor: GLfixed, _yfactor: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glUniform4i64ARB =
    unsafe extern "system" fn(_location: GLint, _x: GLint64, _y: GLint64, _z: GLint64, _w: GLint64);
#[allow(non_camel_case_types)]
pub type PFN_glSamplerParameterIuiv =
    unsafe extern "system" fn(_sampler: GLuint, _pname: SamplerParameterI, _param: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glCreateShaderProgramEXT =
    unsafe extern "system" fn(_type: ShaderType, _string: *const GLchar) -> GLuint;
#[allow(non_camel_case_types)]
pub type PFN_glGetTexLevelParameterfv = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _pname: GetTextureParameter,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexBufferRange = unsafe extern "system" fn(
    _target: TextureTarget,
    _internalformat: SizedInternalFormat,
    _buffer: GLuint,
    _offset: GLintptr,
    _size: GLsizeiptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetPathColorGenivNV =
    unsafe extern "system" fn(_color: PathColor, _pname: PathGenMode, _value: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertex4f =
    unsafe extern "system" fn(_x: GLfloat, _y: GLfloat, _z: GLfloat, _w: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGenTexturesEXT = unsafe extern "system" fn(_n: GLsizei, _textures: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertex3i = unsafe extern "system" fn(_x: GLint, _y: GLint, _z: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos4i = unsafe extern "system" fn(_x: GLint, _y: GLint, _z: GLint, _w: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib3svARB = unsafe extern "system" fn(_index: GLuint, _v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glDrawTextureNV = unsafe extern "system" fn(
    _texture: GLuint,
    _sampler: GLuint,
    _x0: GLfloat,
    _y0: GLfloat,
    _x1: GLfloat,
    _y1: GLfloat,
    _z: GLfloat,
    _s0: GLfloat,
    _t0: GLfloat,
    _s1: GLfloat,
    _t1: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord2s =
    unsafe extern "system" fn(_target: TextureUnit, _s: GLshort, _t: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glBinormal3ivEXT = unsafe extern "system" fn(_v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glColorP4ui = unsafe extern "system" fn(_type: ColorPointerType, _color: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glIsProgram = unsafe extern "system" fn(_program: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexRenderbufferEXT =
    unsafe extern "system" fn(_texunit: TextureUnit, _target: TextureTarget, _renderbuffer: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glColor4sv = unsafe extern "system" fn(_v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glGetActiveUniformName = unsafe extern "system" fn(
    _program: GLuint,
    _uniformIndex: GLuint,
    _bufSize: GLsizei,
    _length: *mut GLsizei,
    _uniformName: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformHandleui64NV =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _value: GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glUniform3d =
    unsafe extern "system" fn(_location: GLint, _x: GLdouble, _y: GLdouble, _z: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream2sATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _x: GLshort, _y: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glClipPlanex =
    unsafe extern "system" fn(_plane: ClipPlaneName, _equation: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glTexParameteriv = unsafe extern "system" fn(
    _target: TextureTarget,
    _pname: TextureParameterName,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glAlphaFragmentOp1ATI = unsafe extern "system" fn(
    _op: FragmentOpATI,
    _dst: GLuint,
    _dstMod: GLuint,
    _arg1: GLuint,
    _arg1Rep: GLuint,
    _arg1Mod: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribFormatNV = unsafe extern "system" fn(
    _index: GLuint,
    _size: GLint,
    _type: VertexAttribType,
    _normalized: GLboolean,
    _stride: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL2d =
    unsafe extern "system" fn(_index: GLuint, _x: GLdouble, _y: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glUniform2fvARB =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramEnvParameterIivNV =
    unsafe extern "system" fn(_target: ProgramTarget, _index: GLuint, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glEndVideoCaptureNV = unsafe extern "system" fn(_video_capture_slot: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexAttribIivEXT =
    unsafe extern "system" fn(_index: GLuint, _pname: VertexAttribEnum, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferTexture3DOES = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _attachment: FramebufferAttachment,
    _textarget: TextureTarget,
    _texture: GLuint,
    _level: GLint,
    _zoffset: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glClearBufferuiv =
    unsafe extern "system" fn(_buffer: Buffer, _drawbuffer: GLint, _value: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glEndTransformFeedback = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glOrthox = unsafe extern "system" fn(
    _l: GLfixed,
    _r: GLfixed,
    _b: GLfixed,
    _t: GLfixed,
    _n: GLfixed,
    _f: GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glPNTrianglesiATI =
    unsafe extern "system" fn(_pname: PNTrianglesPNameATI, _param: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform4dvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glColor4xvOES = unsafe extern "system" fn(_components: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glGetPathLengthNV = unsafe extern "system" fn(
    _path: GLuint,
    _startSegment: GLsizei,
    _numSegments: GLsizei,
) -> GLfloat;
#[allow(non_camel_case_types)]
pub type PFN_glGetNamedBufferPointervEXT = unsafe extern "system" fn(
    _buffer: GLuint,
    _pname: BufferPointerNameARB,
    _params: *mut *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord1dv =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glUniform4ui64ARB = unsafe extern "system" fn(
    _location: GLint,
    _x: GLuint64,
    _y: GLuint64,
    _z: GLuint64,
    _w: GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib2hNV =
    unsafe extern "system" fn(_index: GLuint, _x: GLhalfNV, _y: GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glDebugMessageControlKHR = unsafe extern "system" fn(
    _source: DebugSource,
    _type: DebugType,
    _severity: DebugSeverity,
    _count: GLsizei,
    _ids: *const GLuint,
    _enabled: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord1s = unsafe extern "system" fn(_s: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glGetVariantIntegervEXT =
    unsafe extern "system" fn(_id: GLuint, _value: GetVariantValueEXT, _data: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glBindBufferRangeEXT = unsafe extern "system" fn(
    _target: BufferTargetARB,
    _index: GLuint,
    _buffer: GLuint,
    _offset: GLintptr,
    _size: GLsizeiptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glGenTransformFeedbacksNV = unsafe extern "system" fn(_n: GLsizei, _ids: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib3dARB =
    unsafe extern "system" fn(_index: GLuint, _x: GLdouble, _y: GLdouble, _z: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glPointSizexOES = unsafe extern "system" fn(_size: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glShadingRateQCOM = unsafe extern "system" fn(_rate: ShadingRateQCOM);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream1fATI = unsafe extern "system" fn(_stream: VertexStreamATI, _x: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos2fARB = unsafe extern "system" fn(_x: GLfloat, _y: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glCreateTextures =
    unsafe extern "system" fn(_target: TextureTarget, _n: GLsizei, _textures: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetnSeparableFilter = unsafe extern "system" fn(
    _target: SeparableTarget,
    _format: PixelFormat,
    _type: PixelType,
    _rowBufSize: GLsizei,
    _row: *mut std::os::raw::c_void,
    _columnBufSize: GLsizei,
    _column: *mut std::os::raw::c_void,
    _span: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3usv = unsafe extern "system" fn(_v: *const GLushort);
#[allow(non_camel_case_types)]
pub type PFN_glUniform3i64ARB =
    unsafe extern "system" fn(_location: GLint, _x: GLint64, _y: GLint64, _z: GLint64);
#[allow(non_camel_case_types)]
pub type PFN_glGetIntegerIndexedvEXT =
    unsafe extern "system" fn(_target: GetPName, _index: GLuint, _data: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glTexGend = unsafe extern "system" fn(
    _coord: TextureCoordName,
    _pname: TextureGenParameter,
    _param: GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexGenfv = unsafe extern "system" fn(
    _coord: TextureCoordName,
    _pname: TextureGenParameter,
    _params: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexArrayIntegervEXT =
    unsafe extern "system" fn(_vaobj: GLuint, _pname: VertexArrayPName, _param: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glIsFramebufferEXT = unsafe extern "system" fn(_framebuffer: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform1i64vNV = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLint64EXT,
);
#[allow(non_camel_case_types)]
pub type PFN_glClearDepthxOES = unsafe extern "system" fn(_depth: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glVariantfvEXT = unsafe extern "system" fn(_id: GLuint, _addr: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexGendEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _coord: TextureCoordName,
    _pname: TextureGenParameter,
    _param: GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glVDPAUGetSurfaceivNV = unsafe extern "system" fn(
    _surface: GLvdpauSurfaceNV,
    _pname: GLenum,
    _count: GLsizei,
    _length: *mut GLsizei,
    _values: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetLightxOES =
    unsafe extern "system" fn(_light: LightName, _pname: LightParameter, _params: *mut GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glMakeImageHandleNonResidentNV = unsafe extern "system" fn(_handle: GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glReplacementCodeuiColor4ubVertex3fvSUN =
    unsafe extern "system" fn(_rc: *const GLuint, _c: *const GLubyte, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib1dv = unsafe extern "system" fn(_index: GLuint, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramResourceLocationIndex = unsafe extern "system" fn(
    _program: GLuint,
    _programInterface: ProgramInterface,
    _name: *const GLchar,
) -> GLint;
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord4fv =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI2ivEXT = unsafe extern "system" fn(_index: GLuint, _v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glClearTexImage = unsafe extern "system" fn(
    _texture: GLuint,
    _level: GLint,
    _format: PixelFormat,
    _type: PixelType,
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform1dvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glVDPAUSurfaceAccessNV =
    unsafe extern "system" fn(_surface: GLvdpauSurfaceNV, _access: GLenum);
#[allow(non_camel_case_types)]
pub type PFN_glGetFloati_vNV =
    unsafe extern "system" fn(_target: GetPName, _index: GLuint, _data: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glEdgeFlagv = unsafe extern "system" fn(_flag: *const GLboolean);
#[allow(non_camel_case_types)]
pub type PFN_glMulticastGetQueryObjectuivNV =
    unsafe extern "system" fn(_gpu: GLuint, _id: GLuint, _pname: GLenum, _params: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glNamedBufferStorage = unsafe extern "system" fn(
    _buffer: GLuint,
    _size: GLsizeiptr,
    _data: *const std::os::raw::c_void,
    _flags: BufferStorageMask,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexImage1D = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _internalformat: GLint,
    _width: GLsizei,
    _border: GLint,
    _format: PixelFormat,
    _type: PixelType,
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteNamesAMD =
    unsafe extern "system" fn(_identifier: GLenum, _num: GLuint, _names: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI3ui =
    unsafe extern "system" fn(_index: GLuint, _x: GLuint, _y: GLuint, _z: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL3dvEXT = unsafe extern "system" fn(_index: GLuint, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glNamedFramebufferTextureEXT = unsafe extern "system" fn(
    _framebuffer: GLuint,
    _attachment: FramebufferAttachment,
    _texture: GLuint,
    _level: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glWeightPathsNV = unsafe extern "system" fn(
    _resultPath: GLuint,
    _numPaths: GLsizei,
    _paths: *const GLuint,
    _weights: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glPatchParameteri =
    unsafe extern "system" fn(_pname: PatchParameterName, _value: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos3f = unsafe extern "system" fn(_x: GLfloat, _y: GLfloat, _z: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glColor4ubVertex3fvSUN =
    unsafe extern "system" fn(_c: *const GLubyte, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteFramebuffersEXT =
    unsafe extern "system" fn(_n: GLsizei, _framebuffers: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glBindBufferBaseNV =
    unsafe extern "system" fn(_target: BufferTargetARB, _index: GLuint, _buffer: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glBinormal3dEXT =
    unsafe extern "system" fn(_bx: GLdouble, _by: GLdouble, _bz: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glPointParameterfvEXT =
    unsafe extern "system" fn(_pname: PointParameterNameARB, _params: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glProgramParameteriEXT =
    unsafe extern "system" fn(_program: GLuint, _pname: ProgramParameterPName, _value: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glTextureBarrierNV = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glVertex2dv = unsafe extern "system" fn(_v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glGetnHistogramARB = unsafe extern "system" fn(
    _target: HistogramTargetEXT,
    _reset: GLboolean,
    _format: PixelFormat,
    _type: PixelType,
    _bufSize: GLsizei,
    _values: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGenBuffersARB = unsafe extern "system" fn(_n: GLsizei, _buffers: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glNamedBufferPageCommitmentEXT = unsafe extern "system" fn(
    _buffer: GLuint,
    _offset: GLintptr,
    _size: GLsizeiptr,
    _commit: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform2dv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixMultfEXT = unsafe extern "system" fn(_mode: MatrixMode, _m: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetSeparableFilter = unsafe extern "system" fn(
    _target: SeparableTargetEXT,
    _format: PixelFormat,
    _type: PixelType,
    _row: *mut std::os::raw::c_void,
    _column: *mut std::os::raw::c_void,
    _span: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glLoadTransposeMatrixdARB = unsafe extern "system" fn(_m: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexGenfvEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _coord: TextureCoordName,
    _pname: TextureGenParameter,
    _params: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glLightModeli = unsafe extern "system" fn(_pname: LightModelParameter, _param: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetBufferPointerv = unsafe extern "system" fn(
    _target: BufferTargetARB,
    _pname: BufferPointerNameARB,
    _params: *mut *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glIsImageHandleResidentNV = unsafe extern "system" fn(_handle: GLuint64) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glClearPixelLocalStorageuiEXT =
    unsafe extern "system" fn(_offset: GLsizei, _n: GLsizei, _values: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glUniform1fARB = unsafe extern "system" fn(_location: GLint, _v0: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glCurrentPaletteMatrixOES = unsafe extern "system" fn(_matrixpaletteindex: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGlobalAlphaFactordSUN = unsafe extern "system" fn(_factor: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4bv = unsafe extern "system" fn(_index: GLuint, _v: *const GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4svARB = unsafe extern "system" fn(_index: GLuint, _v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glColor4hNV =
    unsafe extern "system" fn(_red: GLhalfNV, _green: GLhalfNV, _blue: GLhalfNV, _alpha: GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteFramebuffers =
    unsafe extern "system" fn(_n: GLsizei, _framebuffers: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib1hvNV = unsafe extern "system" fn(_index: GLuint, _v: *const GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glOrthoxOES = unsafe extern "system" fn(
    _l: GLfixed,
    _r: GLfixed,
    _b: GLfixed,
    _t: GLfixed,
    _n: GLfixed,
    _f: GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL4i64vNV =
    unsafe extern "system" fn(_index: GLuint, _v: *const GLint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glVertex2fv = unsafe extern "system" fn(_v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib2sNV =
    unsafe extern "system" fn(_index: GLuint, _x: GLshort, _y: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL2i64vNV =
    unsafe extern "system" fn(_index: GLuint, _v: *const GLint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glLightxOES =
    unsafe extern "system" fn(_light: LightName, _pname: LightParameter, _param: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos3fvMESA = unsafe extern "system" fn(_v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glCompressedTexSubImage3DOES = unsafe extern "system" fn(
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
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glUseProgram = unsafe extern "system" fn(_program: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetShaderInfoLog = unsafe extern "system" fn(
    _shader: GLuint,
    _bufSize: GLsizei,
    _length: *mut GLsizei,
    _infoLog: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glMakeTextureHandleResidentNV = unsafe extern "system" fn(_handle: GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glProgramParameters4fvNV = unsafe extern "system" fn(
    _target: VertexAttribEnumNV,
    _index: GLuint,
    _count: GLsizei,
    _v: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord1dv = unsafe extern "system" fn(_v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glProgramEnvParameterI4uiNV = unsafe extern "system" fn(
    _target: ProgramTarget,
    _index: GLuint,
    _x: GLuint,
    _y: GLuint,
    _z: GLuint,
    _w: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glSamplerParameterf =
    unsafe extern "system" fn(_sampler: GLuint, _pname: SamplerParameterF, _param: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_TexStorageAttribs2DEXT = unsafe extern "system" fn(
    _target: TextureTarget,
    _levels: GLsizei,
    _internalformat: SizedInternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _attrib_list: *const std::os::raw::c_int,
);
#[allow(non_camel_case_types)]
pub type PFN_glListBase = unsafe extern "system" fn(_base: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glPointParameterx =
    unsafe extern "system" fn(_pname: PointParameterNameARB, _param: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glStopInstrumentsSGIX = unsafe extern "system" fn(_marker: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoordP4ui =
    unsafe extern "system" fn(_type: TexCoordPointerType, _coords: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glBindBufferRangeNV = unsafe extern "system" fn(
    _target: BufferTargetARB,
    _index: GLuint,
    _buffer: GLuint,
    _offset: GLintptr,
    _size: GLsizeiptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glColorPointer = unsafe extern "system" fn(
    _size: GLint,
    _type: ColorPointerType,
    _stride: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetFramebufferPixelLocalStorageSizeEXT =
    unsafe extern "system" fn(_target: GLuint) -> GLsizei;
#[allow(non_camel_case_types)]
pub type PFN_glGetTexParameterxv = unsafe extern "system" fn(
    _target: TextureTarget,
    _pname: GetTextureParameter,
    _params: *mut GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glPolygonOffsetClamp =
    unsafe extern "system" fn(_factor: GLfloat, _units: GLfloat, _clamp: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferTexture1DEXT = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _attachment: FramebufferAttachment,
    _textarget: TextureTarget,
    _texture: GLuint,
    _level: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetClipPlane =
    unsafe extern "system" fn(_plane: ClipPlaneName, _equation: *mut GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glUniform1dv =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glDetachObjectARB =
    unsafe extern "system" fn(_containerObj: GLhandleARB, _attachedObj: GLhandleARB);
#[allow(non_camel_case_types)]
pub type PFN_glBindTransformFeedback =
    unsafe extern "system" fn(_target: BindTransformFeedbackTarget, _id: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferTexture2DDownsampleIMG = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _attachment: FramebufferAttachment,
    _textarget: TextureTarget,
    _texture: GLuint,
    _level: GLint,
    _xscale: GLint,
    _yscale: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferTexture3D = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _attachment: FramebufferAttachment,
    _textarget: TextureTarget,
    _texture: GLuint,
    _level: GLint,
    _zoffset: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetNamedProgramLocalParameterIuivEXT = unsafe extern "system" fn(
    _program: GLuint,
    _target: ProgramTarget,
    _index: GLuint,
    _params: *mut GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glLinkProgramARB = unsafe extern "system" fn(_programObj: GLhandleARB);
#[allow(non_camel_case_types)]
pub type PFN_glLoadName = unsafe extern "system" fn(_name: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetUniformLocationARB =
    unsafe extern "system" fn(_programObj: GLhandleARB, _name: *const GLcharARB) -> GLint;
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord1f = unsafe extern "system" fn(_target: TextureUnit, _s: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetBufferParameteri64v = unsafe extern "system" fn(
    _target: BufferTargetARB,
    _pname: BufferPNameARB,
    _params: *mut GLint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetnMapdvARB = unsafe extern "system" fn(
    _target: MapTarget,
    _query: MapQuery,
    _bufSize: GLsizei,
    _v: *mut GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord1xvOES =
    unsafe extern "system" fn(_texture: TextureUnit, _coords: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexSubImage3DEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
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
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramEnvParameter4fARB = unsafe extern "system" fn(
    _target: ProgramTarget,
    _index: GLuint,
    _x: GLfloat,
    _y: GLfloat,
    _z: GLfloat,
    _w: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteSemaphoresEXT =
    unsafe extern "system" fn(_n: GLsizei, _semaphores: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glLightxvOES =
    unsafe extern "system" fn(_light: LightName, _pname: LightParameter, _params: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glScissorArrayv =
    unsafe extern "system" fn(_first: GLuint, _count: GLsizei, _v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glIsSyncAPPLE = unsafe extern "system" fn(_sync: GLsync) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glScissorExclusiveArrayvNV =
    unsafe extern "system" fn(_first: GLuint, _count: GLsizei, _v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetSemaphoreParameterui64vEXT = unsafe extern "system" fn(
    _semaphore: GLuint,
    _pname: SemaphoreParameterName,
    _params: *mut GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoordP3uiv =
    unsafe extern "system" fn(_type: TexCoordPointerType, _coords: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glPathSubCommandsNV = unsafe extern "system" fn(
    _path: GLuint,
    _commandStart: GLsizei,
    _commandsToDelete: GLsizei,
    _numCommands: GLsizei,
    _commands: *const GLubyte,
    _numCoords: GLsizei,
    _coordType: PathCoordType,
    _coords: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexGenxvOES = unsafe extern "system" fn(
    _coord: TextureCoordName,
    _pname: TextureGenParameter,
    _params: *const GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexStorageSparseAMD = unsafe extern "system" fn(
    _target: TextureTarget,
    _internalFormat: SizedInternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _layers: GLsizei,
    _flags: TextureStorageMaskAMD,
);
#[allow(non_camel_case_types)]
pub type PFN_glClearNamedFramebufferiv = unsafe extern "system" fn(
    _framebuffer: GLuint,
    _buffer: Buffer,
    _drawbuffer: GLint,
    _value: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform2i64vNV =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord4fColor4fNormal3fVertex4fvSUN = unsafe extern "system" fn(
    _tc: *const GLfloat,
    _c: *const GLfloat,
    _n: *const GLfloat,
    _v: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib2dvARB = unsafe extern "system" fn(_index: GLuint, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4sARB =
    unsafe extern "system" fn(_index: GLuint, _x: GLshort, _y: GLshort, _z: GLshort, _w: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glConvolutionParameterfEXT = unsafe extern "system" fn(
    _target: ConvolutionTargetEXT,
    _pname: ConvolutionParameterEXT,
    _params: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL2dv = unsafe extern "system" fn(_index: GLuint, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glWaitSync =
    unsafe extern "system" fn(_sync: GLsync, _flags: SyncBehaviorFlags, _timeout: GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glGetNamedProgramLocalParameterdvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _target: ProgramTarget,
    _index: GLuint,
    _params: *mut GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramEnvParametersI4uivNV = unsafe extern "system" fn(
    _target: ProgramTarget,
    _index: GLuint,
    _count: GLsizei,
    _params: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform2iv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glNamedRenderbufferStorageMultisampleEXT = unsafe extern "system" fn(
    _renderbuffer: GLuint,
    _samples: GLsizei,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetFragmentLightivSGIX = unsafe extern "system" fn(
    _light: FragmentLightNameSGIX,
    _pname: FragmentLightParameterSGIX,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glColor4ubv = unsafe extern "system" fn(_v: *const GLubyte);
#[allow(non_camel_case_types)]
pub type PFN_glGetFloati_vOES =
    unsafe extern "system" fn(_target: GetPName, _index: GLuint, _data: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glNormal3i = unsafe extern "system" fn(_nx: GLint, _ny: GLint, _nz: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetUniformdv =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _params: *mut GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glValidateProgramARB = unsafe extern "system" fn(_programObj: GLhandleARB);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribs4dvNV =
    unsafe extern "system" fn(_index: GLuint, _count: GLsizei, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexBlendARB = unsafe extern "system" fn(_count: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glDebugMessageControlARB = unsafe extern "system" fn(
    _source: DebugSource,
    _type: DebugType,
    _severity: DebugSeverity,
    _count: GLsizei,
    _ids: *const GLuint,
    _enabled: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos3iARB = unsafe extern "system" fn(_x: GLint, _y: GLint, _z: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexArrayRangeNV =
    unsafe extern "system" fn(_length: GLsizei, _pointer: *const std::os::raw::c_void);
#[allow(non_camel_case_types)]
pub type PFN_glCombinerParameteriNV =
    unsafe extern "system" fn(_pname: CombinerParameterNV, _param: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glCopyConvolutionFilter2D = unsafe extern "system" fn(
    _target: ConvolutionTarget,
    _internalformat: InternalFormat,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetPixelMapfv = unsafe extern "system" fn(_map: PixelMap, _values: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord2hNV =
    unsafe extern "system" fn(_target: TextureUnit, _s: GLhalfNV, _t: GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glVertexBlendEnvfATI =
    unsafe extern "system" fn(_pname: VertexStreamATI, _param: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glIsTransformFeedbackNV = unsafe extern "system" fn(_id: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3dEXT =
    unsafe extern "system" fn(_red: GLdouble, _green: GLdouble, _blue: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glTexParameterxOES =
    unsafe extern "system" fn(_target: TextureTarget, _pname: GetTextureParameter, _param: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glVertexFormatNV =
    unsafe extern "system" fn(_size: GLint, _type: VertexPointerType, _stride: GLsizei);
#[allow(non_camel_case_types)]
pub type PFN_glCombinerParameterivNV =
    unsafe extern "system" fn(_pname: CombinerParameterNV, _params: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib1fv = unsafe extern "system" fn(_index: GLuint, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetFramebufferAttachmentParameterivOES = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _attachment: FramebufferAttachment,
    _pname: FramebufferAttachmentParameterName,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformHandleui64vARB = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _values: *const GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexEnvivEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureEnvTarget,
    _pname: TextureEnvParameter,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform2ui64NV =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _x: GLuint64EXT, _y: GLuint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glUniform4i64NV = unsafe extern "system" fn(
    _location: GLint,
    _x: GLint64EXT,
    _y: GLint64EXT,
    _z: GLint64EXT,
    _w: GLint64EXT,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetPointervKHR =
    unsafe extern "system" fn(_pname: GLenum, _params: *mut *mut std::os::raw::c_void);
#[allow(non_camel_case_types)]
pub type PFN_glPollInstrumentsSGIX = unsafe extern "system" fn(_marker_p: *mut GLint) -> GLint;
#[allow(non_camel_case_types)]
pub type PFN_glProgramLocalParametersI4ivNV = unsafe extern "system" fn(
    _target: ProgramTarget,
    _index: GLuint,
    _count: GLsizei,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVariantuivEXT = unsafe extern "system" fn(_id: GLuint, _addr: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertex3hvNV = unsafe extern "system" fn(_v: *const GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glTexSubImage1D = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _xoffset: GLint,
    _width: GLsizei,
    _format: PixelFormat,
    _type: PixelType,
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribBinding =
    unsafe extern "system" fn(_attribindex: GLuint, _bindingindex: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glBindParameterEXT =
    unsafe extern "system" fn(_value: VertexShaderParameterEXT) -> GLuint;
#[allow(non_camel_case_types)]
pub type PFN_glShaderSource = unsafe extern "system" fn(
    _shader: GLuint,
    _count: GLsizei,
    _string: *const *const GLchar,
    _length: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glObjectPurgeableAPPLE =
    unsafe extern "system" fn(_objectType: GLenum, _name: GLuint, _option: GLenum) -> GLenum;
#[allow(non_camel_case_types)]
pub type PFN_glArrayElementEXT = unsafe extern "system" fn(_i: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glReplacementCodeuiColor4fNormal3fVertex3fvSUN = unsafe extern "system" fn(
    _rc: *const GLuint,
    _c: *const GLfloat,
    _n: *const GLfloat,
    _v: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord1d = unsafe extern "system" fn(_target: TextureUnit, _s: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord3xOES = unsafe extern "system" fn(_s: GLfixed, _t: GLfixed, _r: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glTextureStorageMem2DEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _levels: GLsizei,
    _internalFormat: SizedInternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _memory: GLuint,
    _offset: GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform4i =
    unsafe extern "system" fn(_location: GLint, _v0: GLint, _v1: GLint, _v2: GLint, _v3: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib2dv = unsafe extern "system" fn(_index: GLuint, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glIglooInterfaceSGIX =
    unsafe extern "system" fn(_pname: GLenum, _params: *const std::os::raw::c_void);
#[allow(non_camel_case_types)]
pub type PFN_glPointSize = unsafe extern "system" fn(_size: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexArrayMultiTexCoordOffsetEXT = unsafe extern "system" fn(
    _vaobj: GLuint,
    _buffer: GLuint,
    _texunit: GLenum,
    _size: GLint,
    _type: TexCoordPointerType,
    _stride: GLsizei,
    _offset: GLintptr,
);
#[allow(non_camel_case_types)]
pub type PFN_glEvalPoint1 = unsafe extern "system" fn(_i: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferTextureMultiviewOVR = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _attachment: FramebufferAttachment,
    _texture: GLuint,
    _level: GLint,
    _baseViewIndex: GLint,
    _numViews: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetMaterialfv = unsafe extern "system" fn(
    _face: MaterialFace,
    _pname: MaterialParameter,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI2uivEXT = unsafe extern "system" fn(_index: GLuint, _v: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexPointerEXT = unsafe extern "system" fn(
    _size: GLint,
    _type: VertexPointerType,
    _stride: GLsizei,
    _count: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glIndexMask = unsafe extern "system" fn(_mask: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiDrawElementsIndirectAMD = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _type: DrawElementsType,
    _indirect: *const std::os::raw::c_void,
    _primcount: GLsizei,
    _stride: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glVariantPointerEXT = unsafe extern "system" fn(
    _id: GLuint,
    _type: ScalarType,
    _stride: GLuint,
    _addr: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glStencilStrokePathNV =
    unsafe extern "system" fn(_path: GLuint, _reference: GLint, _mask: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glUniformHandleui64vIMG =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glBlendEquationSeparate =
    unsafe extern "system" fn(_modeRGB: BlendEquationModeEXT, _modeAlpha: BlendEquationModeEXT);
#[allow(non_camel_case_types)]
pub type PFN_glTexRenderbufferNV =
    unsafe extern "system" fn(_target: TextureTarget, _renderbuffer: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertex3bvOES = unsafe extern "system" fn(_coords: *const GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glVertex4fv = unsafe extern "system" fn(_v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos3iv = unsafe extern "system" fn(_v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord1fv =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glPointAlongPathNV = unsafe extern "system" fn(
    _path: GLuint,
    _startSegment: GLsizei,
    _numSegments: GLsizei,
    _distance: GLfloat,
    _x: *mut GLfloat,
    _y: *mut GLfloat,
    _tangentX: *mut GLfloat,
    _tangentY: *mut GLfloat,
) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord4x = unsafe extern "system" fn(
    _texture: TextureUnit,
    _s: GLfixed,
    _t: GLfixed,
    _r: GLfixed,
    _q: GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureStorage2DMultisample = unsafe extern "system" fn(
    _texture: GLuint,
    _samples: GLsizei,
    _internalformat: SizedInternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _fixedsamplelocations: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glPolygonOffsetClampEXT =
    unsafe extern "system" fn(_factor: GLfloat, _units: GLfloat, _clamp: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glSampleCoveragex = unsafe extern "system" fn(_value: GLclampx, _invert: GLboolean);
#[allow(non_camel_case_types)]
pub type PFN_glColorTableParameterfv = unsafe extern "system" fn(
    _target: ColorTableTarget,
    _pname: ColorTableParameterPNameSGI,
    _params: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glEnablei = unsafe extern "system" fn(_target: EnableCap, _index: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferDrawBuffersEXT =
    unsafe extern "system" fn(_framebuffer: GLuint, _n: GLsizei, _bufs: *const DrawBufferMode);
#[allow(non_camel_case_types)]
pub type PFN_glBindBufferARB = unsafe extern "system" fn(_target: BufferTargetARB, _buffer: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetTexParameterIuiv = unsafe extern "system" fn(
    _target: TextureTarget,
    _pname: GetTextureParameter,
    _params: *mut GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glRenderbufferStorageMultisampleEXT = unsafe extern "system" fn(
    _target: RenderbufferTarget,
    _samples: GLsizei,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferTextureFaceEXT = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _attachment: FramebufferAttachment,
    _texture: GLuint,
    _level: GLint,
    _face: TextureTarget,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiDrawArraysIndirectCountARB = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _indirect: *const std::os::raw::c_void,
    _drawcount: GLintptr,
    _maxdrawcount: GLsizei,
    _stride: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramNamedParameter4dNV = unsafe extern "system" fn(
    _id: GLuint,
    _len: GLsizei,
    _name: *const GLubyte,
    _x: GLdouble,
    _y: GLdouble,
    _z: GLdouble,
    _w: GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glSamplePatternSGIS = unsafe extern "system" fn(_pattern: SamplePatternSGIS);
#[allow(non_camel_case_types)]
pub type PFN_glIsTextureEXT = unsafe extern "system" fn(_texture: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glSetFenceNV = unsafe extern "system" fn(_fence: GLuint, _condition: FenceConditionNV);
#[allow(non_camel_case_types)]
pub type PFN_glValidateProgram = unsafe extern "system" fn(_program: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib2sARB =
    unsafe extern "system" fn(_index: GLuint, _x: GLshort, _y: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glVertex4hNV =
    unsafe extern "system" fn(_x: GLhalfNV, _y: GLhalfNV, _z: GLhalfNV, _w: GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribDivisorANGLE =
    unsafe extern "system" fn(_index: GLuint, _divisor: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribIFormatNV = unsafe extern "system" fn(
    _index: GLuint,
    _size: GLint,
    _type: VertexAttribIType,
    _stride: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetFragDataLocationEXT =
    unsafe extern "system" fn(_program: GLuint, _name: *const GLchar) -> GLint;
#[allow(non_camel_case_types)]
pub type PFN_glScaled = unsafe extern "system" fn(_x: GLdouble, _y: GLdouble, _z: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4fNV =
    unsafe extern "system" fn(_index: GLuint, _x: GLfloat, _y: GLfloat, _z: GLfloat, _w: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glColor3iv = unsafe extern "system" fn(_v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetFogFuncSGIS = unsafe extern "system" fn(_points: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glMultTransposeMatrixfARB = unsafe extern "system" fn(_m: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glNormal3f = unsafe extern "system" fn(_nx: GLfloat, _ny: GLfloat, _nz: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetVideoui64vNV =
    unsafe extern "system" fn(_video_slot: GLuint, _pname: GLenum, _params: *mut GLuint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramLocalParameterdvARB =
    unsafe extern "system" fn(_target: ProgramTarget, _index: GLuint, _params: *mut GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glRenderMode = unsafe extern "system" fn(_mode: RenderingMode) -> GLint;
#[allow(non_camel_case_types)]
pub type PFN_glBindFragDataLocation =
    unsafe extern "system" fn(_program: GLuint, _color: GLuint, _name: *const GLchar);
#[allow(non_camel_case_types)]
pub type PFN_glCreateRenderbuffers =
    unsafe extern "system" fn(_n: GLsizei, _renderbuffers: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetNamedFramebufferParameterivEXT = unsafe extern "system" fn(
    _framebuffer: GLuint,
    _pname: GetFramebufferParameter,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glLoadMatrixxOES = unsafe extern "system" fn(_m: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glBlendEquationiOES =
    unsafe extern "system" fn(_buf: GLuint, _mode: BlendEquationModeEXT);
#[allow(non_camel_case_types)]
pub type PFN_glShadeModel = unsafe extern "system" fn(_mode: ShadingModel);
#[allow(non_camel_case_types)]
pub type PFN_glGenRenderbuffers =
    unsafe extern "system" fn(_n: GLsizei, _renderbuffers: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexAttribIuiv =
    unsafe extern "system" fn(_index: GLuint, _pname: VertexAttribEnum, _params: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexAttribdvARB = unsafe extern "system" fn(
    _index: GLuint,
    _pname: VertexAttribPropertyARB,
    _params: *mut GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord2ivARB =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramLocalParameterI4ivNV =
    unsafe extern "system" fn(_target: ProgramTarget, _index: GLuint, _params: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI4bv = unsafe extern "system" fn(_index: GLuint, _v: *const GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glNamedFramebufferSampleLocationsfvARB = unsafe extern "system" fn(
    _framebuffer: GLuint,
    _start: GLuint,
    _count: GLsizei,
    _v: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glCopyTextureLevelsAPPLE = unsafe extern "system" fn(
    _destinationTexture: GLuint,
    _sourceTexture: GLuint,
    _sourceBaseLevel: GLint,
    _sourceLevelCount: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiDrawArrays = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _first: *const GLint,
    _count: *const GLsizei,
    _drawcount: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord3dARB =
    unsafe extern "system" fn(_target: TextureUnit, _s: GLdouble, _t: GLdouble, _r: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glCompressedTexImage3DARB = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _border: GLint,
    _imageSize: GLsizei,
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGenerateMultiTexMipmapEXT =
    unsafe extern "system" fn(_texunit: TextureUnit, _target: TextureTarget);
#[allow(non_camel_case_types)]
pub type PFN_glDeformSGIX = unsafe extern "system" fn(_mask: FfdMaskSGIX);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord3fARB =
    unsafe extern "system" fn(_target: TextureUnit, _s: GLfloat, _t: GLfloat, _r: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glPushDebugGroupKHR = unsafe extern "system" fn(
    _source: DebugSource,
    _id: GLuint,
    _length: GLsizei,
    _message: *const GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiDrawElementsIndirectBindlessCountNV = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _type: DrawElementsType,
    _indirect: *const std::os::raw::c_void,
    _drawCount: GLsizei,
    _maxDrawCount: GLsizei,
    _stride: GLsizei,
    _vertexBufferCount: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos3s = unsafe extern "system" fn(_x: GLshort, _y: GLshort, _z: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glTangent3bvEXT = unsafe extern "system" fn(_v: *const GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glTexStorage2DMultisample = unsafe extern "system" fn(
    _target: TextureTarget,
    _samples: GLsizei,
    _internalformat: SizedInternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _fixedsamplelocations: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformHandleui64IMG =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _value: GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glDrawElementArrayATI =
    unsafe extern "system" fn(_mode: PrimitiveType, _count: GLsizei);
#[allow(non_camel_case_types)]
pub type PFN_glScissorIndexedOES = unsafe extern "system" fn(
    _index: GLuint,
    _left: GLint,
    _bottom: GLint,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3uiv = unsafe extern "system" fn(_v: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glTextureStorage3DEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: GLenum,
    _levels: GLsizei,
    _internalformat: SizedInternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetMapParameterivNV =
    unsafe extern "system" fn(_target: EvalTargetNV, _pname: MapParameterNV, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiDrawMeshTasksIndirectNV =
    unsafe extern "system" fn(_indirect: GLintptr, _drawcount: GLsizei, _stride: GLsizei);
#[allow(non_camel_case_types)]
pub type PFN_glUniform1uivEXT =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI3iv = unsafe extern "system" fn(_index: GLuint, _v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glBindVertexArray = unsafe extern "system" fn(_array: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glBlendEquationSeparateEXT =
    unsafe extern "system" fn(_modeRGB: BlendEquationModeEXT, _modeAlpha: BlendEquationModeEXT);
#[allow(non_camel_case_types)]
pub type PFN_glGenFramebuffers = unsafe extern "system" fn(_n: GLsizei, _framebuffers: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetColorTableParameterfvEXT = unsafe extern "system" fn(
    _target: ColorTableTarget,
    _pname: GetColorTableParameterPNameSGI,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glClearBufferSubData = unsafe extern "system" fn(
    _target: BufferTargetARB,
    _internalformat: SizedInternalFormat,
    _offset: GLintptr,
    _size: GLsizeiptr,
    _format: PixelFormat,
    _type: PixelType,
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glDisableVertexAttribArray = unsafe extern "system" fn(_index: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4ivARB = unsafe extern "system" fn(_index: GLuint, _v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL3dv = unsafe extern "system" fn(_index: GLuint, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVideoCaptureStreamParameterfvNV = unsafe extern "system" fn(
    _video_capture_slot: GLuint,
    _stream: GLuint,
    _pname: GLenum,
    _params: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glBinormal3fEXT = unsafe extern "system" fn(_bx: GLfloat, _by: GLfloat, _bz: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glBlendFuncSeparate = unsafe extern "system" fn(
    _sfactorRGB: BlendingFactor,
    _dfactorRGB: BlendingFactor,
    _sfactorAlpha: BlendingFactor,
    _dfactorAlpha: BlendingFactor,
);
#[allow(non_camel_case_types)]
pub type PFN_glDispatchCompute =
    unsafe extern "system" fn(_num_groups_x: GLuint, _num_groups_y: GLuint, _num_groups_z: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetHistogramParameterxvOES = unsafe extern "system" fn(
    _target: HistogramTargetEXT,
    _pname: GetHistogramParameterPNameEXT,
    _params: *mut GLfixed,
);
#[allow(non_camel_case_types)]
pub type PFN_glTangent3fEXT = unsafe extern "system" fn(_tx: GLfloat, _ty: GLfloat, _tz: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVDPAURegisterVideoSurfaceWithPictureStructureNV =
    unsafe extern "system" fn(
        _vdpSurface: *const std::os::raw::c_void,
        _target: GLenum,
        _numTextureNames: GLsizei,
        _textureNames: *const GLuint,
        _isFrameStructure: GLboolean,
    ) -> GLvdpauSurfaceNV;
#[allow(non_camel_case_types)]
pub type PFN_glMatrixLoadfEXT = unsafe extern "system" fn(_mode: MatrixMode, _m: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glColorTableSGI = unsafe extern "system" fn(
    _target: ColorTableTargetSGI,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _format: PixelFormat,
    _type: PixelType,
    _table: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetTextureParameterivEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _pname: GetTextureParameter,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glImportSyncEXT = unsafe extern "system" fn(
    _external_sync_type: GLenum,
    _external_sync: GLintptr,
    _flags: GLbitfield,
) -> GLsync;
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform3ui64vNV = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLuint64EXT,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureParameterfEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _pname: TextureParameterName,
    _param: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferTexture = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _attachment: FramebufferAttachment,
    _texture: GLuint,
    _level: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGenProgramPipelinesEXT =
    unsafe extern "system" fn(_n: GLsizei, _pipelines: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramLocalParameter4fvARB =
    unsafe extern "system" fn(_target: ProgramTarget, _index: GLuint, _params: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform1uivEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glTangent3dEXT =
    unsafe extern "system" fn(_tx: GLdouble, _ty: GLdouble, _tz: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glTexEnvfv = unsafe extern "system" fn(
    _target: TextureEnvTarget,
    _pname: TextureEnvParameter,
    _params: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glCheckNamedFramebufferStatus =
    unsafe extern "system" fn(_framebuffer: GLuint, _target: FramebufferTarget) -> GLenum;
#[allow(non_camel_case_types)]
pub type PFN_glIsRenderbuffer = unsafe extern "system" fn(_renderbuffer: GLuint) -> GLboolean;
#[allow(non_camel_case_types)]
pub type PFN_glResolveMultisampleFramebufferAPPLE = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glSetFragmentShaderConstantATI =
    unsafe extern "system" fn(_dst: GLuint, _value: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glTransformFeedbackAttribsNV =
    unsafe extern "system" fn(_count: GLsizei, _attribs: *const GLint, _bufferMode: GLenum);
#[allow(non_camel_case_types)]
pub type PFN_glGetDebugMessageLogAMD = unsafe extern "system" fn(
    _count: GLuint,
    _bufSize: GLsizei,
    _categories: *mut GLenum,
    _severities: *mut GLuint,
    _ids: *mut GLuint,
    _lengths: *mut GLsizei,
    _message: *mut GLchar,
) -> GLuint;
#[allow(non_camel_case_types)]
pub type PFN_glDrawCommandsStatesNV = unsafe extern "system" fn(
    _buffer: GLuint,
    _indirects: *const GLintptr,
    _sizes: *const GLsizei,
    _states: *const GLuint,
    _fbos: *const GLuint,
    _count: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glNamedRenderbufferStorageMultisampleCoverageEXT = unsafe extern "system" fn(
    _renderbuffer: GLuint,
    _coverageSamples: GLsizei,
    _colorSamples: GLsizei,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glUnmapTexture2DINTEL = unsafe extern "system" fn(_texture: GLuint, _level: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord3ivARB =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glResetHistogram = unsafe extern "system" fn(_target: HistogramTargetEXT);
#[allow(non_camel_case_types)]
pub type PFN_glVariantsvEXT = unsafe extern "system" fn(_id: GLuint, _addr: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glGetTexParameterIivOES = unsafe extern "system" fn(
    _target: TextureTarget,
    _pname: GetTextureParameter,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetMinmaxParameterfvEXT = unsafe extern "system" fn(
    _target: MinmaxTargetEXT,
    _pname: GetMinmaxParameterPNameEXT,
    _params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetTexGeniv = unsafe extern "system" fn(
    _coord: TextureCoordName,
    _pname: TextureGenParameter,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGenOcclusionQueriesNV = unsafe extern "system" fn(_n: GLsizei, _ids: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glTextureImage2DMultisampleCoverageNV = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _coverageSamples: GLsizei,
    _colorSamples: GLsizei,
    _internalFormat: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _fixedSampleLocations: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFN_glClearNamedBufferSubDataEXT = unsafe extern "system" fn(
    _buffer: GLuint,
    _internalformat: SizedInternalFormat,
    _offset: GLsizeiptr,
    _size: GLsizeiptr,
    _format: PixelFormat,
    _type: PixelType,
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glStartTilingQCOM = unsafe extern "system" fn(
    _x: GLuint,
    _y: GLuint,
    _width: GLuint,
    _height: GLuint,
    _preserveMask: BufferBitQCOM,
);
#[allow(non_camel_case_types)]
pub type PFN_glVDPAUFiniNV = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI1ivEXT = unsafe extern "system" fn(_index: GLuint, _v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glPointParameterf =
    unsafe extern "system" fn(_pname: PointParameterNameARB, _param: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribL2i64NV =
    unsafe extern "system" fn(_index: GLuint, _x: GLint64EXT, _y: GLint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glLightEnviSGIX =
    unsafe extern "system" fn(_pname: LightEnvParameterSGIX, _param: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform1dEXT =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _x: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glGetNamedProgramStringEXT = unsafe extern "system" fn(
    _program: GLuint,
    _target: ProgramTarget,
    _pname: ProgramStringProperty,
    _string: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribPointer = unsafe extern "system" fn(
    _index: GLuint,
    _size: GLint,
    _type: VertexAttribPointerType,
    _normalized: GLboolean,
    _stride: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexWeightfEXT = unsafe extern "system" fn(_weight: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glDeletePerfMonitorsAMD =
    unsafe extern "system" fn(_n: GLsizei, _monitors: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glNormal3b = unsafe extern "system" fn(_nx: GLbyte, _ny: GLbyte, _nz: GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glReadnPixels = unsafe extern "system" fn(
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _format: PixelFormat,
    _type: PixelType,
    _bufSize: GLsizei,
    _data: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI1uiv = unsafe extern "system" fn(_index: GLuint, _v: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream4svATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _coords: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glGetSeparableFilterEXT = unsafe extern "system" fn(
    _target: SeparableTargetEXT,
    _format: PixelFormat,
    _type: PixelType,
    _row: *mut std::os::raw::c_void,
    _column: *mut std::os::raw::c_void,
    _span: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glLoadProgramNV = unsafe extern "system" fn(
    _target: VertexAttribEnumNV,
    _id: GLuint,
    _len: GLsizei,
    _program: *const GLubyte,
);
#[allow(non_camel_case_types)]
pub type PFN_glProvokingVertex = unsafe extern "system" fn(_mode: VertexProvokingMode);
#[allow(non_camel_case_types)]
pub type PFN_glStencilFuncSeparateATI = unsafe extern "system" fn(
    _frontfunc: StencilFunction,
    _backfunc: StencilFunction,
    _ref: GLint,
    _mask: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetIntegerv = unsafe extern "system" fn(_pname: GetPName, _data: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glCompressedTextureImage3DEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _level: GLint,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _height: GLsizei,
    _depth: GLsizei,
    _border: GLint,
    _imageSize: GLsizei,
    _bits: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glMapBuffer = unsafe extern "system" fn(
    _target: BufferTargetARB,
    _access: BufferAccessARB,
) -> *mut std::os::raw::c_void;
#[allow(non_camel_case_types)]
pub type PFN_glImportSemaphoreWin32HandleEXT = unsafe extern "system" fn(
    _semaphore: GLuint,
    _handleType: ExternalHandleType,
    _handle: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexWeighthvNV = unsafe extern "system" fn(_weight: *const GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos2dv = unsafe extern "system" fn(_v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos2sMESA = unsafe extern "system" fn(_x: GLshort, _y: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glGenVertexArraysOES = unsafe extern "system" fn(_n: GLsizei, _arrays: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glDrawBuffersARB =
    unsafe extern "system" fn(_n: GLsizei, _bufs: *const DrawBufferMode);
#[allow(non_camel_case_types)]
pub type PFN_glGetnHistogram = unsafe extern "system" fn(
    _target: HistogramTarget,
    _reset: GLboolean,
    _format: PixelFormat,
    _type: PixelType,
    _bufSize: GLsizei,
    _values: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiDrawMeshTasksIndirectCountNV = unsafe extern "system" fn(
    _indirect: GLintptr,
    _drawcount: GLintptr,
    _maxdrawcount: GLsizei,
    _stride: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3ivEXT = unsafe extern "system" fn(_v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glClearBufferData = unsafe extern "system" fn(
    _target: BufferStorageTarget,
    _internalformat: SizedInternalFormat,
    _format: PixelFormat,
    _type: PixelType,
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glCopyConvolutionFilter2DEXT = unsafe extern "system" fn(
    _target: ConvolutionTargetEXT,
    _internalformat: InternalFormat,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glNamedProgramLocalParameterI4uivEXT = unsafe extern "system" fn(
    _program: GLuint,
    _target: ProgramTarget,
    _index: GLuint,
    _params: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform2dv =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glReadPixels = unsafe extern "system" fn(
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _format: PixelFormat,
    _type: PixelType,
    _pixels: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform4fARB = unsafe extern "system" fn(
    _location: GLint,
    _v0: GLfloat,
    _v1: GLfloat,
    _v2: GLfloat,
    _v3: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetActiveSubroutineUniformiv = unsafe extern "system" fn(
    _program: GLuint,
    _shadertype: ShaderType,
    _index: GLuint,
    _pname: SubroutineParameterName,
    _values: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniformMatrix3x4fv = unsafe extern "system" fn(
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetTextureLevelParameteriv = unsafe extern "system" fn(
    _texture: GLuint,
    _level: GLint,
    _pname: GetTextureParameter,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glClearBufferiv =
    unsafe extern "system" fn(_buffer: Buffer, _drawbuffer: GLint, _value: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glDisableClientStateiEXT =
    unsafe extern "system" fn(_array: EnableCap, _index: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos3fv = unsafe extern "system" fn(_v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glBeginTransformFeedback = unsafe extern "system" fn(_primitiveMode: PrimitiveType);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferFetchBarrierQCOM = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferTextureFaceARB = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _attachment: FramebufferAttachment,
    _texture: GLuint,
    _level: GLint,
    _face: TextureTarget,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetVertexAttribiv =
    unsafe extern "system" fn(_index: GLuint, _pname: VertexAttribPropertyARB, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glPixelZoom = unsafe extern "system" fn(_xfactor: GLfloat, _yfactor: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glCopyTextureSubImage3D = unsafe extern "system" fn(
    _texture: GLuint,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _zoffset: GLint,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glCreateFramebuffers =
    unsafe extern "system" fn(_n: GLsizei, _framebuffers: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetPixelTexGenParameterivSGIS =
    unsafe extern "system" fn(_pname: PixelTexGenParameterNameSGIS, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramEnvParameter4fvARB =
    unsafe extern "system" fn(_target: ProgramTarget, _index: GLuint, _params: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix3dvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix3x4dv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glNormal3s = unsafe extern "system" fn(_nx: GLshort, _ny: GLshort, _nz: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glConvolutionFilter1D = unsafe extern "system" fn(
    _target: ConvolutionTarget,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _format: PixelFormat,
    _type: PixelType,
    _image: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glCopyPixels = unsafe extern "system" fn(
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _type: PixelCopyType,
);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferPixelLocalStorageSizeEXT =
    unsafe extern "system" fn(_target: GLuint, _size: GLsizei);
#[allow(non_camel_case_types)]
pub type PFN_glGetColorTableParameterivEXT = unsafe extern "system" fn(
    _target: ColorTableTarget,
    _pname: GetColorTableParameterPNameSGI,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiDrawElementsBaseVertex = unsafe extern "system" fn(
    _mode: PrimitiveType,
    _count: *const GLsizei,
    _type: DrawElementsType,
    _indices: *const *const std::os::raw::c_void,
    _drawcount: GLsizei,
    _basevertex: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glStencilStrokePathInstancedNV = unsafe extern "system" fn(
    _numPaths: GLsizei,
    _pathNameType: PathElementType,
    _paths: *const std::os::raw::c_void,
    _pathBase: GLuint,
    _reference: GLint,
    _mask: GLuint,
    _transformType: PathTransformType,
    _transformValues: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glTransformFeedbackVaryingsNV = unsafe extern "system" fn(
    _program: GLuint,
    _count: GLsizei,
    _locations: *const GLint,
    _bufferMode: GLenum,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform4d = unsafe extern "system" fn(
    _location: GLint,
    _x: GLdouble,
    _y: GLdouble,
    _z: GLdouble,
    _w: GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord2sv = unsafe extern "system" fn(_v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glVertex4hvNV = unsafe extern "system" fn(_v: *const GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glExtGetRenderbuffersQCOM = unsafe extern "system" fn(
    _renderbuffers: *mut GLuint,
    _maxRenderbuffers: GLint,
    _numRenderbuffers: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glCompileCommandListNV = unsafe extern "system" fn(_list: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteSyncAPPLE = unsafe extern "system" fn(_sync: GLsync);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord1xOES = unsafe extern "system" fn(_s: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glGenRenderbuffersOES =
    unsafe extern "system" fn(_n: GLsizei, _renderbuffers: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramParameteriARB =
    unsafe extern "system" fn(_program: GLuint, _pname: ProgramParameterPName, _value: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord1svARB =
    unsafe extern "system" fn(_target: TextureUnit, _v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glDepthRange = unsafe extern "system" fn(_n: GLdouble, _f: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glGenFencesAPPLE = unsafe extern "system" fn(_n: GLsizei, _fences: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixScalefEXT =
    unsafe extern "system" fn(_mode: MatrixMode, _x: GLfloat, _y: GLfloat, _z: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glReadBuffer = unsafe extern "system" fn(_src: ReadBufferMode);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoord1hNV = unsafe extern "system" fn(_target: TextureUnit, _s: GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glPixelTexGenParameterfSGIS =
    unsafe extern "system" fn(_pname: PixelTexGenParameterNameSGIS, _param: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glUniform4i64vNV =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glClientWaitSyncAPPLE =
    unsafe extern "system" fn(_sync: GLsync, _flags: SyncObjectMask, _timeout: GLuint64) -> GLenum;
#[allow(non_camel_case_types)]
pub type PFN_glCopyTexSubImage2D = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetnPixelMapfvARB =
    unsafe extern "system" fn(_map: PixelMap, _bufSize: GLsizei, _values: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glClearAccum =
    unsafe extern "system" fn(_red: GLfloat, _green: GLfloat, _blue: GLfloat, _alpha: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glFogCoordPointer = unsafe extern "system" fn(
    _type: FogPointerTypeEXT,
    _stride: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetDoublev = unsafe extern "system" fn(_pname: GetPName, _data: *mut GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glGetLightxvOES =
    unsafe extern "system" fn(_light: LightName, _pname: LightParameter, _params: *mut GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glGetVideoCaptureStreamdvNV = unsafe extern "system" fn(
    _video_capture_slot: GLuint,
    _stream: GLuint,
    _pname: GLenum,
    _params: *mut GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramivNV =
    unsafe extern "system" fn(_id: GLuint, _pname: VertexAttribEnumNV, _params: *mut GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetnUniformuivKHR = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _bufSize: GLsizei,
    _params: *mut GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glMapGrid2f = unsafe extern "system" fn(
    _un: GLint,
    _u1: GLfloat,
    _u2: GLfloat,
    _vn: GLint,
    _v1: GLfloat,
    _v2: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glNormal3xOES = unsafe extern "system" fn(_nx: GLfixed, _ny: GLfixed, _nz: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glNormalStream3sATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _nx: GLshort, _ny: GLshort, _nz: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glTexGendv = unsafe extern "system" fn(
    _coord: TextureCoordName,
    _pname: TextureGenParameter,
    _params: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glTextureParameterfvEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _pname: TextureParameterName,
    _params: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform3fARB =
    unsafe extern "system" fn(_location: GLint, _v0: GLfloat, _v1: GLfloat, _v2: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glIndexxOES = unsafe extern "system" fn(_component: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glMakeNamedBufferResidentNV =
    unsafe extern "system" fn(_buffer: GLuint, _access: GLenum);
#[allow(non_camel_case_types)]
pub type PFN_glGetPixelTexGenParameterfvSGIS =
    unsafe extern "system" fn(_pname: PixelTexGenParameterNameSGIS, _params: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColorPointerListIBM = unsafe extern "system" fn(
    _size: GLint,
    _type: SecondaryColorPointerTypeIBM,
    _stride: GLint,
    _pointer: *mut *const std::os::raw::c_void,
    _ptrstride: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib2s = unsafe extern "system" fn(_index: GLuint, _x: GLshort, _y: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glGetCoverageModulationTableNV =
    unsafe extern "system" fn(_bufSize: GLsizei, _v: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI3uiv = unsafe extern "system" fn(_index: GLuint, _v: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib4f =
    unsafe extern "system" fn(_index: GLuint, _x: GLfloat, _y: GLfloat, _z: GLfloat, _w: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glScalexOES = unsafe extern "system" fn(_x: GLfixed, _y: GLfixed, _z: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glPushDebugGroup = unsafe extern "system" fn(
    _source: DebugSource,
    _id: GLuint,
    _length: GLsizei,
    _message: *const GLchar,
);
#[allow(non_camel_case_types)]
pub type PFN_glMakeImageHandleResidentARB =
    unsafe extern "system" fn(_handle: GLuint64, _access: GLenum);
#[allow(non_camel_case_types)]
pub type PFN_glProgramEnvParameter4dvARB =
    unsafe extern "system" fn(_target: ProgramTarget, _index: GLuint, _params: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glStencilOpValueAMD =
    unsafe extern "system" fn(_face: StencilFaceDirection, _value: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glBlendEquationSeparateiOES = unsafe extern "system" fn(
    _buf: GLuint,
    _modeRGB: BlendEquationModeEXT,
    _modeAlpha: BlendEquationModeEXT,
);
#[allow(non_camel_case_types)]
pub type PFN_glCopyMultiTexSubImage3DEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _zoffset: GLint,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
    _height: GLsizei,
);
#[allow(non_camel_case_types)]
pub type PFN_glMapVertexAttrib2dAPPLE = unsafe extern "system" fn(
    _index: GLuint,
    _size: GLuint,
    _u1: GLdouble,
    _u2: GLdouble,
    _ustride: GLint,
    _uorder: GLint,
    _v1: GLdouble,
    _v2: GLdouble,
    _vstride: GLint,
    _vorder: GLint,
    _points: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI4ui =
    unsafe extern "system" fn(_index: GLuint, _x: GLuint, _y: GLuint, _z: GLuint, _w: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream1sATI = unsafe extern "system" fn(_stream: VertexStreamATI, _x: GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glConvolutionParameterfvEXT = unsafe extern "system" fn(
    _target: ConvolutionTargetEXT,
    _pname: ConvolutionParameterEXT,
    _params: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glDrawTexiOES =
    unsafe extern "system" fn(_x: GLint, _y: GLint, _z: GLint, _width: GLint, _height: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGenerateMipmap = unsafe extern "system" fn(_target: TextureTarget);
#[allow(non_camel_case_types)]
pub type PFN_glGetMapControlPointsNV = unsafe extern "system" fn(
    _target: EvalTargetNV,
    _index: GLuint,
    _type: MapTypeNV,
    _ustride: GLsizei,
    _vstride: GLsizei,
    _packed: GLboolean,
    _points: *mut std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glMapBufferOES = unsafe extern "system" fn(
    _target: BufferTargetARB,
    _access: BufferAccessARB,
) -> *mut std::os::raw::c_void;
#[allow(non_camel_case_types)]
pub type PFN_glGetTextureParameterfv =
    unsafe extern "system" fn(_texture: GLuint, _pname: GetTextureParameter, _params: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glPathParameterfvNV =
    unsafe extern "system" fn(_path: GLuint, _pname: PathParameter, _value: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glColorFragmentOp3ATI = unsafe extern "system" fn(
    _op: FragmentOpATI,
    _dst: GLuint,
    _dstMask: GLuint,
    _dstMod: GLuint,
    _arg1: GLuint,
    _arg1Rep: GLuint,
    _arg1Mod: GLuint,
    _arg2: GLuint,
    _arg2Rep: GLuint,
    _arg2Mod: GLuint,
    _arg3: GLuint,
    _arg3Rep: GLuint,
    _arg3Mod: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform4ui64vARB = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glColorPointerListIBM = unsafe extern "system" fn(
    _size: GLint,
    _type: ColorPointerType,
    _stride: GLint,
    _pointer: *mut *const std::os::raw::c_void,
    _ptrstride: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glClientAttribDefaultEXT = unsafe extern "system" fn(_mask: ClientAttribMask);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferTextureLayerEXT = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _attachment: FramebufferAttachment,
    _texture: GLuint,
    _level: GLint,
    _layer: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glReadInstrumentsSGIX = unsafe extern "system" fn(_marker: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glBufferSubDataARB = unsafe extern "system" fn(
    _target: BufferTargetARB,
    _offset: GLintptrARB,
    _size: GLsizeiptrARB,
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glScissorIndexedv = unsafe extern "system" fn(_index: GLuint, _v: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glGetProgramNamedParameterdvNV = unsafe extern "system" fn(
    _id: GLuint,
    _len: GLsizei,
    _name: *const GLubyte,
    _params: *mut GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glNormalStream3dvATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _coords: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix4x2dvEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetMapdv =
    unsafe extern "system" fn(_target: MapTarget, _query: GetMapQuery, _v: *mut GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glQueryCounterEXT =
    unsafe extern "system" fn(_id: GLuint, _target: QueryCounterTarget);
#[allow(non_camel_case_types)]
pub type PFN_glBeginQuery = unsafe extern "system" fn(_target: QueryTarget, _id: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetPathMetricRangeNV = unsafe extern "system" fn(
    _metricQueryMask: PathMetricMask,
    _firstPathName: GLuint,
    _numPaths: GLsizei,
    _stride: GLsizei,
    _metrics: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColor3fv = unsafe extern "system" fn(_v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix4x3fv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos4f =
    unsafe extern "system" fn(_x: GLfloat, _y: GLfloat, _z: GLfloat, _w: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord4sv = unsafe extern "system" fn(_v: *const GLshort);
#[allow(non_camel_case_types)]
pub type PFN_glTextureParameterIivEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _pname: TextureParameterName,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexStream2dvATI =
    unsafe extern "system" fn(_stream: VertexStreamATI, _coords: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glTextureImage1DEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _level: GLint,
    _internalformat: GLint,
    _width: GLsizei,
    _border: GLint,
    _format: PixelFormat,
    _type: PixelType,
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glColor4us =
    unsafe extern "system" fn(_red: GLushort, _green: GLushort, _blue: GLushort, _alpha: GLushort);
#[allow(non_camel_case_types)]
pub type PFN_glVertexBindingDivisor =
    unsafe extern "system" fn(_bindingindex: GLuint, _divisor: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glEnableVertexArrayAttrib = unsafe extern "system" fn(_vaobj: GLuint, _index: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glCopyPathNV = unsafe extern "system" fn(_resultPath: GLuint, _srcPath: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glTexSubImage2DEXT = unsafe extern "system" fn(
    _target: TextureTarget,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _format: PixelFormat,
    _type: PixelType,
    _pixels: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glPatchParameteriEXT =
    unsafe extern "system" fn(_pname: PatchParameterName, _value: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glViewportIndexedfv = unsafe extern "system" fn(_index: GLuint, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos3i = unsafe extern "system" fn(_x: GLint, _y: GLint, _z: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glEnableVertexAttribArray = unsafe extern "system" fn(_index: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glNamedBufferStorageMemEXT = unsafe extern "system" fn(
    _buffer: GLuint,
    _size: GLsizeiptr,
    _memory: GLuint,
    _offset: GLuint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramParameter4dvNV =
    unsafe extern "system" fn(_target: VertexAttribEnumNV, _index: GLuint, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glFlushMappedBufferRangeAPPLE =
    unsafe extern "system" fn(_target: BufferTargetARB, _offset: GLintptr, _size: GLsizeiptr);
#[allow(non_camel_case_types)]
pub type PFN_glBlendColorxOES =
    unsafe extern "system" fn(_red: GLfixed, _green: GLfixed, _blue: GLfixed, _alpha: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glCheckNamedFramebufferStatusEXT =
    unsafe extern "system" fn(_framebuffer: GLuint, _target: FramebufferTarget) -> GLenum;
#[allow(non_camel_case_types)]
pub type PFN_glFragmentLightivSGIX = unsafe extern "system" fn(
    _light: FragmentLightNameSGIX,
    _pname: FragmentLightParameterSGIX,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetClipPlanefOES =
    unsafe extern "system" fn(_plane: ClipPlaneName, _equation: *mut GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform1d =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _v0: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform2iEXT =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _v0: GLint, _v1: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform3i = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _v0: GLint,
    _v1: GLint,
    _v2: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glGetIntegerui64vNV =
    unsafe extern "system" fn(_value: GLenum, _result: *mut GLuint64EXT);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform1ui =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _v0: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform3iEXT = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _v0: GLint,
    _v1: GLint,
    _v2: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformHandleui64ARB =
    unsafe extern "system" fn(_program: GLuint, _location: GLint, _value: GLuint64);
#[allow(non_camel_case_types)]
pub type PFN_glGetnPixelMapusvARB =
    unsafe extern "system" fn(_map: PixelMap, _bufSize: GLsizei, _values: *mut GLushort);
#[allow(non_camel_case_types)]
pub type PFN_glReplacementCodeuiVertex3fSUN =
    unsafe extern "system" fn(_rc: GLuint, _x: GLfloat, _y: GLfloat, _z: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glSpriteParameterivSGIX =
    unsafe extern "system" fn(_pname: SpriteParameterNameSGIX, _params: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glVDPAURegisterVideoSurfaceNV = unsafe extern "system" fn(
    _vdpSurface: *const std::os::raw::c_void,
    _target: GLenum,
    _numTextureNames: GLsizei,
    _textureNames: *const GLuint,
) -> GLvdpauSurfaceNV;
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib2d =
    unsafe extern "system" fn(_index: GLuint, _x: GLdouble, _y: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glFlushPixelDataRangeNV = unsafe extern "system" fn(_target: PixelDataRangeTargetNV);
#[allow(non_camel_case_types)]
pub type PFN_glBindBuffersBase = unsafe extern "system" fn(
    _target: BufferTargetARB,
    _first: GLuint,
    _count: GLsizei,
    _buffers: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glEvalMapsNV = unsafe extern "system" fn(_target: EvalTargetNV, _mode: EvalMapsModeNV);
#[allow(non_camel_case_types)]
pub type PFN_glInsertComponentEXT =
    unsafe extern "system" fn(_res: GLuint, _src: GLuint, _num: GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glResetHistogramEXT = unsafe extern "system" fn(_target: HistogramTargetEXT);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttribI4iEXT =
    unsafe extern "system" fn(_index: GLuint, _x: GLint, _y: GLint, _z: GLint, _w: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glPopDebugGroupKHR = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glGetTextureSamplerHandleARB =
    unsafe extern "system" fn(_texture: GLuint, _sampler: GLuint) -> GLuint64;
#[allow(non_camel_case_types)]
pub type PFN_glPopClientAttrib = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub type PFN_glFragmentLightModelfvSGIX =
    unsafe extern "system" fn(_pname: FragmentLightModelParameterSGIX, _params: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glReplacementCodeuiColor4ubVertex3fSUN = unsafe extern "system" fn(
    _rc: GLuint,
    _r: GLubyte,
    _g: GLubyte,
    _b: GLubyte,
    _a: GLubyte,
    _x: GLfloat,
    _y: GLfloat,
    _z: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glUniform3i =
    unsafe extern "system" fn(_location: GLint, _v0: GLint, _v1: GLint, _v2: GLint);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteRenderbuffers =
    unsafe extern "system" fn(_n: GLsizei, _renderbuffers: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glEndTilingQCOM = unsafe extern "system" fn(_preserveMask: BufferBitQCOM);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoordPointerListIBM = unsafe extern "system" fn(
    _size: GLint,
    _type: TexCoordPointerType,
    _stride: GLint,
    _pointer: *mut *const std::os::raw::c_void,
    _ptrstride: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glRasterPos2fv = unsafe extern "system" fn(_v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glUniform4fvARB =
    unsafe extern "system" fn(_location: GLint, _count: GLsizei, _value: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord3hvNV = unsafe extern "system" fn(_v: *const GLhalfNV);
#[allow(non_camel_case_types)]
pub type PFN_glCopyMultiTexImage1DEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _level: GLint,
    _internalformat: InternalFormat,
    _x: GLint,
    _y: GLint,
    _width: GLsizei,
    _border: GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glCreateSyncFromCLeventARB = unsafe extern "system" fn(
    _context: *mut CLContext,
    _event: *mut CLContext,
    _flags: GLbitfield,
) -> GLsync;
#[allow(non_camel_case_types)]
pub type PFN_glGetHandleARB = unsafe extern "system" fn(_pname: GLenum) -> GLhandleARB;
#[allow(non_camel_case_types)]
pub type PFN_glGetSamplerParameterIuivEXT =
    unsafe extern "system" fn(_sampler: GLuint, _pname: SamplerParameterI, _params: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glRectf =
    unsafe extern "system" fn(_x1: GLfloat, _y1: GLfloat, _x2: GLfloat, _y2: GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform3i64vARB = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLint64,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform2uiv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniformMatrix4x3dv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _transpose: GLboolean,
    _value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub type PFN_glStringMarkerGREMEDY =
    unsafe extern "system" fn(_len: GLsizei, _string: *const std::os::raw::c_void);
#[allow(non_camel_case_types)]
pub type PFN_glFragmentLightModelivSGIX =
    unsafe extern "system" fn(_pname: FragmentLightModelParameterSGIX, _params: *const GLint);
#[allow(non_camel_case_types)]
pub type PFN_glDepthRangeArrayv =
    unsafe extern "system" fn(_first: GLuint, _count: GLsizei, _v: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glGetTextureLevelParameterivEXT = unsafe extern "system" fn(
    _texture: GLuint,
    _target: TextureTarget,
    _level: GLint,
    _pname: GetTextureParameter,
    _params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertex3bOES = unsafe extern "system" fn(_x: GLbyte, _y: GLbyte, _z: GLbyte);
#[allow(non_camel_case_types)]
pub type PFN_glFogCoorddv = unsafe extern "system" fn(_coord: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glTransformPathNV = unsafe extern "system" fn(
    _resultPath: GLuint,
    _srcPath: GLuint,
    _transformType: PathTransformType,
    _transformValues: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glPointParameterxvOES =
    unsafe extern "system" fn(_pname: PointParameterNameARB, _params: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexCoordPointerEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _size: GLint,
    _type: TexCoordPointerType,
    _stride: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glGenQueries = unsafe extern "system" fn(_n: GLsizei, _ids: *mut GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glGetMapxvOES =
    unsafe extern "system" fn(_target: MapTarget, _query: GetMapQuery, _v: *mut GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glVariantusvEXT = unsafe extern "system" fn(_id: GLuint, _addr: *const GLushort);
#[allow(non_camel_case_types)]
pub type PFN_glMatrixLoaddEXT = unsafe extern "system" fn(_mode: MatrixMode, _m: *const GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glVertexArrayVertexAttribFormatEXT = unsafe extern "system" fn(
    _vaobj: GLuint,
    _attribindex: GLuint,
    _size: GLint,
    _type: VertexAttribType,
    _normalized: GLboolean,
    _relativeoffset: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glCompressedTexSubImage3D = unsafe extern "system" fn(
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
    _data: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glWindowPos3dARB = unsafe extern "system" fn(_x: GLdouble, _y: GLdouble, _z: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glColor3xOES =
    unsafe extern "system" fn(_red: GLfixed, _green: GLfixed, _blue: GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glDepthRangeIndexeddNV =
    unsafe extern "system" fn(_index: GLuint, _n: GLdouble, _f: GLdouble);
#[allow(non_camel_case_types)]
pub type PFN_glGetGraphicsResetStatusEXT = unsafe extern "system" fn() -> GLenum;
#[allow(non_camel_case_types)]
pub type PFN_glSecondaryColorPointer = unsafe extern "system" fn(
    _size: GLint,
    _type: ColorPointerType,
    _stride: GLsizei,
    _pointer: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glIndexxvOES = unsafe extern "system" fn(_component: *const GLfixed);
#[allow(non_camel_case_types)]
pub type PFN_glDrawCommandsAddressNV = unsafe extern "system" fn(
    _primitiveMode: GLenum,
    _indirects: *const GLuint64,
    _sizes: *const GLsizei,
    _count: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glTexCoord2fVertex3fvSUN =
    unsafe extern "system" fn(_tc: *const GLfloat, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glCompressedMultiTexImage1DEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _level: GLint,
    _internalformat: InternalFormat,
    _width: GLsizei,
    _border: GLint,
    _imageSize: GLsizei,
    _bits: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glColor4ubVertex2fSUN = unsafe extern "system" fn(
    _r: GLubyte,
    _g: GLubyte,
    _b: GLubyte,
    _a: GLubyte,
    _x: GLfloat,
    _y: GLfloat,
);
#[allow(non_camel_case_types)]
pub type PFN_glMultiTexParameterIivEXT = unsafe extern "system" fn(
    _texunit: TextureUnit,
    _target: TextureTarget,
    _pname: TextureParameterName,
    _params: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glCallLists = unsafe extern "system" fn(
    _n: GLsizei,
    _type: ListNameType,
    _lists: *const std::os::raw::c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_glNormalPointervINTEL =
    unsafe extern "system" fn(_type: NormalPointerType, _pointer: *mut *const std::os::raw::c_void);
#[allow(non_camel_case_types)]
pub type PFN_glUniform2i64ARB =
    unsafe extern "system" fn(_location: GLint, _x: GLint64, _y: GLint64);
#[allow(non_camel_case_types)]
pub type PFN_glDeleteSamplers =
    unsafe extern "system" fn(_count: GLsizei, _samplers: *const GLuint);
#[allow(non_camel_case_types)]
pub type PFN_glProgramUniform3iv = unsafe extern "system" fn(
    _program: GLuint,
    _location: GLint,
    _count: GLsizei,
    _value: *const GLint,
);
#[allow(non_camel_case_types)]
pub type PFN_glFramebufferRenderbuffer = unsafe extern "system" fn(
    _target: FramebufferTarget,
    _attachment: FramebufferAttachment,
    _renderbuffertarget: RenderbufferTarget,
    _renderbuffer: GLuint,
);
#[allow(non_camel_case_types)]
pub type PFN_glVertexAttrib1fvNV = unsafe extern "system" fn(_index: GLuint, _v: *const GLfloat);
#[allow(non_camel_case_types)]
pub type PFN_glGetString = unsafe extern "system" fn(_name: StringName) -> *const GLubyte;
