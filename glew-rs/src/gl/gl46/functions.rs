use crate::gl;
use crate::types::*;
use gl::bitflags::*;
use gl::enums::*;
use std::ffi::c_void;
use std::fmt;
impl GL46 {
    pub unsafe fn glUniform4f(
        &self,
        _location: GLint,
        _v0: GLfloat,
        _v1: GLfloat,
        _v2: GLfloat,
        _v3: GLfloat,
    ) {
    }
    pub unsafe fn glUniform1i(&self, _location: GLint, _v0: GLint) {}
    pub unsafe fn glVertexAttrib2s(&self, _index: GLuint, _x: GLshort, _y: GLshort) {}
    pub unsafe fn glGetBufferParameteri64v(
        &self,
        _target: BufferTargetARB,
        _pname: BufferPNameARB,
        _params: *mut GLint64,
    ) {
    }
    pub unsafe fn glTexStorage1D(
        &self,
        _target: TextureTarget,
        _levels: GLsizei,
        _internalformat: SizedInternalFormat,
        _width: GLsizei,
    ) {
    }
    pub unsafe fn glDrawTransformFeedbackInstanced(
        &self,
        _mode: PrimitiveType,
        _id: GLuint,
        _instancecount: GLsizei,
    ) {
    }
    pub unsafe fn glTextureStorage2D(
        &self,
        _texture: GLuint,
        _levels: GLsizei,
        _internalformat: SizedInternalFormat,
        _width: GLsizei,
        _height: GLsizei,
    ) {
    }
    pub unsafe fn glUniform2i(&self, _location: GLint, _v0: GLint, _v1: GLint) {}
    pub unsafe fn glTexImage3DMultisample(
        &self,
        _target: TextureTarget,
        _samples: GLsizei,
        _internalformat: InternalFormat,
        _width: GLsizei,
        _height: GLsizei,
        _depth: GLsizei,
        _fixedsamplelocations: GLboolean,
    ) {
    }
    pub unsafe fn glSamplerParameterIiv(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterI,
        _param: *const GLint,
    ) {
    }
    pub unsafe fn glProgramUniform3f(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLfloat,
        _v1: GLfloat,
        _v2: GLfloat,
    ) {
    }
    pub unsafe fn glClampColor(&self, _target: ClampColorTargetARB, _clamp: ClampColorModeARB) {}
    pub unsafe fn glTexCoordP4ui(&self, _type: TexCoordPointerType, _coords: GLuint) {}
    pub unsafe fn glFramebufferTexture2D(
        &self,
        _target: FramebufferTarget,
        _attachment: FramebufferAttachment,
        _textarget: TextureTarget,
        _texture: GLuint,
        _level: GLint,
    ) {
    }
    pub unsafe fn glInvalidateBufferData(&self, _buffer: GLuint) {}
    pub unsafe fn glMultiTexCoordP3uiv(
        &self,
        _texture: TextureUnit,
        _type: TexCoordPointerType,
        _coords: *const GLuint,
    ) {
    }
    pub unsafe fn glGetVertexAttribdv(
        &self,
        _index: GLuint,
        _pname: VertexAttribPropertyARB,
        _params: *mut GLdouble,
    ) {
    }
    pub unsafe fn glCreateFramebuffers(&self, _n: GLsizei, _framebuffers: *mut GLuint) {}
    pub unsafe fn glGetVertexArrayIndexediv(
        &self,
        _vaobj: GLuint,
        _index: GLuint,
        _pname: VertexArrayPName,
        _param: *mut GLint,
    ) {
    }
    pub unsafe fn glBlendEquation(&self, _mode: BlendEquationModeEXT) {}
    pub unsafe fn glClearBufferSubData(
        &self,
        _target: BufferTargetARB,
        _internalformat: SizedInternalFormat,
        _offset: GLintptr,
        _size: GLsizeiptr,
        _format: PixelFormat,
        _type: PixelType,
        _data: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glGetnPixelMapusv(
        &self,
        _map: PixelMap,
        _bufSize: GLsizei,
        _values: *mut GLushort,
    ) {
    }
    pub unsafe fn glVertexAttrib2fv(&self, _index: GLuint, _v: *const GLfloat) {}
    pub unsafe fn glCopyTexImage2D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _internalformat: InternalFormat,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _border: GLint,
    ) {
    }
    pub unsafe fn glVertexAttrib4s(
        &self,
        _index: GLuint,
        _x: GLshort,
        _y: GLshort,
        _z: GLshort,
        _w: GLshort,
    ) {
    }
    pub unsafe fn glStencilOpSeparate(
        &self,
        _face: StencilFaceDirection,
        _sfail: StencilOp,
        _dpfail: StencilOp,
        _dppass: StencilOp,
    ) {
    }
    pub unsafe fn glColorP4ui(&self, _type: ColorPointerType, _color: GLuint) {}
    pub unsafe fn glGenTransformFeedbacks(&self, _n: GLsizei, _ids: *mut GLuint) {}
    pub unsafe fn glGetQueryObjectui64v(
        &self,
        _id: GLuint,
        _pname: QueryObjectParameterName,
        _params: *mut GLuint64,
    ) {
    }
    pub unsafe fn glVertexAttrib4Nub(
        &self,
        _index: GLuint,
        _x: GLubyte,
        _y: GLubyte,
        _z: GLubyte,
        _w: GLubyte,
    ) {
    }
    pub unsafe fn glSamplerParameteriv(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterI,
        _param: *const GLint,
    ) {
    }
    pub unsafe fn glTexCoordP1ui(&self, _type: TexCoordPointerType, _coords: GLuint) {}
    pub unsafe fn glCompressedTexSubImage1D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _xoffset: GLint,
        _width: GLsizei,
        _format: InternalFormat,
        _imageSize: GLsizei,
        _data: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glBindAttribLocation(
        &self,
        _program: GLuint,
        _index: GLuint,
        _name: *const GLchar,
    ) {
    }
    pub unsafe fn glEnableVertexArrayAttrib(&self, _vaobj: GLuint, _index: GLuint) {}
    pub unsafe fn glTexSubImage1D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _xoffset: GLint,
        _width: GLsizei,
        _format: PixelFormat,
        _type: PixelType,
        _pixels: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glUniform3dv(&self, _location: GLint, _count: GLsizei, _value: *const GLdouble) {}
    pub unsafe fn glFramebufferTextureLayer(
        &self,
        _target: FramebufferTarget,
        _attachment: FramebufferAttachment,
        _texture: GLuint,
        _level: GLint,
        _layer: GLint,
    ) {
    }
    pub unsafe fn glVertexAttribL2d(&self, _index: GLuint, _x: GLdouble, _y: GLdouble) {}
    pub unsafe fn glGetProgramPipelineInfoLog(
        &self,
        _pipeline: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _infoLog: *mut GLchar,
    ) {
    }
    pub unsafe fn glVertexAttribP3uiv(
        &self,
        _index: GLuint,
        _type: VertexAttribPointerType,
        _normalized: GLboolean,
        _value: *const GLuint,
    ) {
    }
    pub unsafe fn glTexSubImage3D(
        &self,
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
    ) {
    }
    pub unsafe fn glCompressedTexImage3D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _internalformat: InternalFormat,
        _width: GLsizei,
        _height: GLsizei,
        _depth: GLsizei,
        _border: GLint,
        _imageSize: GLsizei,
        _data: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glVertexAttribI3uiv(&self, _index: GLuint, _v: *const GLuint) {}
    pub unsafe fn glGetTexParameterIuiv(
        &self,
        _target: TextureTarget,
        _pname: GetTextureParameter,
        _params: *mut GLuint,
    ) {
    }
    pub unsafe fn glVertexP4ui(&self, _type: VertexPointerType, _value: GLuint) {}
    pub unsafe fn glVertexAttrib4Nbv(&self, _index: GLuint, _v: *const GLbyte) {}
    pub unsafe fn glProgramUniformMatrix2dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
    }
    pub unsafe fn glBindSamplers(&self, _first: GLuint, _count: GLsizei, _samplers: *const GLuint) {
    }
    pub unsafe fn glValidateProgram(&self, _program: GLuint) {}
    pub unsafe fn glNamedRenderbufferStorage(
        &self,
        _renderbuffer: GLuint,
        _internalformat: InternalFormat,
        _width: GLsizei,
        _height: GLsizei,
    ) {
    }
    pub unsafe fn glTextureParameterfv(
        &self,
        _texture: GLuint,
        _pname: TextureParameterName,
        _param: *const GLfloat,
    ) {
    }
    pub unsafe fn glUniform1iv(&self, _location: GLint, _count: GLsizei, _value: *const GLint) {}
    pub unsafe fn glCreateShaderProgramv(
        &self,
        _type: ShaderType,
        _count: GLsizei,
        _strings: *const *const GLchar,
    ) {
    }
    pub unsafe fn glUniformMatrix2fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
    }
    pub unsafe fn glTextureParameteri(
        &self,
        _texture: GLuint,
        _pname: TextureParameterName,
        _param: GLint,
    ) {
    }
    pub unsafe fn glCheckNamedFramebufferStatus(
        &self,
        _framebuffer: GLuint,
        _target: FramebufferTarget,
    ) {
    }
    pub unsafe fn glCompressedTexSubImage2D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _xoffset: GLint,
        _yoffset: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _format: InternalFormat,
        _imageSize: GLsizei,
        _data: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glMultiTexCoordP4uiv(
        &self,
        _texture: TextureUnit,
        _type: TexCoordPointerType,
        _coords: *const GLuint,
    ) {
    }
    pub unsafe fn glGetShaderInfoLog(
        &self,
        _shader: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _infoLog: *mut GLchar,
    ) {
    }
    pub unsafe fn glProgramUniform1uiv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLuint,
    ) {
    }
    pub unsafe fn glGetProgramResourceiv(
        &self,
        _program: GLuint,
        _programInterface: ProgramInterface,
        _index: GLuint,
        _propCount: GLsizei,
        _props: *const ProgramResourceProperty,
        _count: GLsizei,
        _length: *mut GLsizei,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glDrawArrays(&self, _mode: PrimitiveType, _first: GLint, _count: GLsizei) {}
    pub unsafe fn glVertexAttribL4d(
        &self,
        _index: GLuint,
        _x: GLdouble,
        _y: GLdouble,
        _z: GLdouble,
        _w: GLdouble,
    ) {
    }
    pub unsafe fn glCompressedTextureSubImage1D(
        &self,
        _texture: GLuint,
        _level: GLint,
        _xoffset: GLint,
        _width: GLsizei,
        _format: InternalFormat,
        _imageSize: GLsizei,
        _data: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glUniformMatrix3x4dv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
    }
    pub unsafe fn glVertexArrayVertexBuffer(
        &self,
        _vaobj: GLuint,
        _bindingindex: GLuint,
        _buffer: GLuint,
        _offset: GLintptr,
        _stride: GLsizei,
    ) {
    }
    pub unsafe fn glVertexArrayBindingDivisor(
        &self,
        _vaobj: GLuint,
        _bindingindex: GLuint,
        _divisor: GLuint,
    ) {
    }
    pub unsafe fn glVertexAttrib1fv(&self, _index: GLuint, _v: *const GLfloat) {}
    pub unsafe fn glVertexAttrib3s(&self, _index: GLuint, _x: GLshort, _y: GLshort, _z: GLshort) {}
    pub unsafe fn glPointSize(&self, _size: GLfloat) {}
    pub unsafe fn glVertexAttribP2ui(
        &self,
        _index: GLuint,
        _type: VertexAttribPointerType,
        _normalized: GLboolean,
        _value: GLuint,
    ) {
    }
    pub unsafe fn glGetActiveSubroutineName(
        &self,
        _program: GLuint,
        _shadertype: ShaderType,
        _index: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _name: *mut GLchar,
    ) {
    }
    pub unsafe fn glMemoryBarrier(&self, _barriers: MemoryBarrierMask) {}
    pub unsafe fn glPointParameterfv(
        &self,
        _pname: PointParameterNameARB,
        _params: *const GLfloat,
    ) {
    }
    pub unsafe fn glGetBufferSubData(
        &self,
        _target: BufferTargetARB,
        _offset: GLintptr,
        _size: GLsizeiptr,
        _data: *mut std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glDetachShader(&self, _program: GLuint, _shader: GLuint) {}
    pub unsafe fn glDrawTransformFeedback(&self, _mode: PrimitiveType, _id: GLuint) {}
    pub unsafe fn glShaderStorageBlockBinding(
        &self,
        _program: GLuint,
        _storageBlockIndex: GLuint,
        _storageBlockBinding: GLuint,
    ) {
    }
    pub unsafe fn glGetRenderbufferParameteriv(
        &self,
        _target: RenderbufferTarget,
        _pname: RenderbufferParameterName,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glDeleteProgram(&self, _program: GLuint) {}
    pub unsafe fn glProvokingVertex(&self, _mode: VertexProvokingMode) {}
    pub unsafe fn glUniform1d(&self, _location: GLint, _x: GLdouble) {}
    pub unsafe fn glGetSynciv(
        &self,
        _sync: GLsync,
        _pname: SyncParameterName,
        _count: GLsizei,
        _length: *mut GLsizei,
        _values: *mut GLint,
    ) {
    }
    pub unsafe fn glClearNamedBufferData(
        &self,
        _buffer: GLuint,
        _internalformat: SizedInternalFormat,
        _format: PixelFormat,
        _type: PixelType,
        _data: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glGetInteger64i_v(&self, _target: GetPName, _index: GLuint, _data: *mut GLint64) {
    }
    pub unsafe fn glVertexAttrib4fv(&self, _index: GLuint, _v: *const GLfloat) {}
    pub unsafe fn glMultiTexCoordP3ui(
        &self,
        _texture: TextureUnit,
        _type: TexCoordPointerType,
        _coords: GLuint,
    ) {
    }
    pub unsafe fn glVertexArrayAttribFormat(
        &self,
        _vaobj: GLuint,
        _attribindex: GLuint,
        _size: GLint,
        _type: VertexAttribType,
        _normalized: GLboolean,
        _relativeoffset: GLuint,
    ) {
    }
    pub unsafe fn glUniformMatrix2x4fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
    }
    pub unsafe fn glPolygonOffsetClamp(&self, _factor: GLfloat, _units: GLfloat, _clamp: GLfloat) {}
    pub unsafe fn glGetnHistogram(
        &self,
        _target: HistogramTarget,
        _reset: GLboolean,
        _format: PixelFormat,
        _type: PixelType,
        _bufSize: GLsizei,
        _values: *mut std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glBlendFunci(&self, _buf: GLuint, _src: BlendingFactor, _dst: BlendingFactor) {}
    pub unsafe fn glDrawElementsInstancedBaseInstance(
        &self,
        _mode: PrimitiveType,
        _count: GLsizei,
        _type: PrimitiveType,
        _indices: *const std::os::raw::c_void,
        _instancecount: GLsizei,
        _baseinstance: GLuint,
    ) {
    }
    pub unsafe fn glGetVertexAttribIuiv(
        &self,
        _index: GLuint,
        _pname: VertexAttribEnum,
        _params: *mut GLuint,
    ) {
    }
    pub unsafe fn glVertexAttribI2i(&self, _index: GLuint, _x: GLint, _y: GLint) {}
    pub unsafe fn glIsRenderbuffer(&self, _renderbuffer: GLuint) {}
    pub unsafe fn glPrimitiveRestartIndex(&self, _index: GLuint) {}
    pub unsafe fn glGetProgramResourceIndex(
        &self,
        _program: GLuint,
        _programInterface: ProgramInterface,
        _name: *const GLchar,
    ) {
    }
    pub unsafe fn glTransformFeedbackBufferBase(
        &self,
        _xfb: GLuint,
        _index: GLuint,
        _buffer: GLuint,
    ) {
    }
    pub unsafe fn glMultiDrawArraysIndirectCount(
        &self,
        _mode: PrimitiveType,
        _indirect: *const std::os::raw::c_void,
        _drawcount: GLintptr,
        _maxdrawcount: GLsizei,
        _stride: GLsizei,
    ) {
    }
    pub unsafe fn glGetActiveUniformBlockName(
        &self,
        _program: GLuint,
        _uniformBlockIndex: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _uniformBlockName: *mut GLchar,
    ) {
    }
    pub unsafe fn glEndQueryIndexed(&self, _target: QueryTarget, _index: GLuint) {}
    pub unsafe fn glGetNamedFramebufferParameteriv(
        &self,
        _framebuffer: GLuint,
        _pname: GetFramebufferParameter,
        _param: *mut GLint,
    ) {
    }
    pub unsafe fn glVertexAttribI1uiv(&self, _index: GLuint, _v: *const GLuint) {}
    pub unsafe fn glUniform1dv(&self, _location: GLint, _count: GLsizei, _value: *const GLdouble) {}
    pub unsafe fn glDeleteProgramPipelines(&self, _n: GLsizei, _pipelines: *const GLuint) {}
    pub unsafe fn glMapBuffer(&self, _target: BufferTargetARB, _access: BufferAccessARB) {}
    pub unsafe fn glCopyTextureSubImage1D(
        &self,
        _texture: GLuint,
        _level: GLint,
        _xoffset: GLint,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
    ) {
    }
    pub unsafe fn glUniformMatrix3dv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
    }
    pub unsafe fn glProgramUniform4ui(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLuint,
        _v1: GLuint,
        _v2: GLuint,
        _v3: GLuint,
    ) {
    }
    pub unsafe fn glVertexAttrib1s(&self, _index: GLuint, _x: GLshort) {}
    pub unsafe fn glCreateTextures(
        &self,
        _target: TextureTarget,
        _n: GLsizei,
        _textures: *mut GLuint,
    ) {
    }
    pub unsafe fn glTextureParameteriv(
        &self,
        _texture: GLuint,
        _pname: TextureParameterName,
        _param: *const GLint,
    ) {
    }
    pub unsafe fn glTextureBuffer(
        &self,
        _texture: GLuint,
        _internalformat: SizedInternalFormat,
        _buffer: GLuint,
    ) {
    }
    pub unsafe fn glGetVertexArrayiv(
        &self,
        _vaobj: GLuint,
        _pname: VertexArrayPName,
        _param: *mut GLint,
    ) {
    }
    pub unsafe fn glUniform2dv(&self, _location: GLint, _count: GLsizei, _value: *const GLdouble) {}
    pub unsafe fn glProgramParameteri(
        &self,
        _program: GLuint,
        _pname: ProgramParameterPName,
        _value: GLint,
    ) {
    }
    pub unsafe fn glColorP3uiv(&self, _type: ColorPointerType, _color: *const GLuint) {}
    pub unsafe fn glGetTextureParameterIuiv(
        &self,
        _texture: GLuint,
        _pname: GetTextureParameter,
        _params: *mut GLuint,
    ) {
    }
    pub unsafe fn glVertexAttrib4Nsv(&self, _index: GLuint, _v: *const GLshort) {}
    pub unsafe fn glInvalidateFramebuffer(
        &self,
        _target: FramebufferTarget,
        _numAttachments: GLsizei,
        _attachments: *const InvalidateFramebufferAttachment,
    ) {
    }
    pub unsafe fn glSamplerParameterIuiv(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterI,
        _param: *const GLuint,
    ) {
    }
    pub unsafe fn glGetNamedFramebufferAttachmentParameteriv(
        &self,
        _framebuffer: GLuint,
        _attachment: FramebufferAttachment,
        _pname: FramebufferAttachmentParameterName,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glSampleMaski(&self, _maskNumber: GLuint, _mask: GLbitfield) {}
    pub unsafe fn glBindTransformFeedback(
        &self,
        _target: BindTransformFeedbackTarget,
        _id: GLuint,
    ) {
    }
    pub unsafe fn glBlendFuncSeparate(
        &self,
        _sfactorRGB: BlendingFactor,
        _dfactorRGB: BlendingFactor,
        _sfactorAlpha: BlendingFactor,
        _dfactorAlpha: BlendingFactor,
    ) {
    }
    pub unsafe fn glUniformMatrix4x3dv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
    }
    pub unsafe fn glLinkProgram(&self, _program: GLuint) {}
    pub unsafe fn glDrawRangeElementsBaseVertex(
        &self,
        _mode: PrimitiveType,
        _start: GLuint,
        _end: GLuint,
        _count: GLsizei,
        _type: DrawElementsType,
        _indices: *const std::os::raw::c_void,
        _basevertex: GLint,
    ) {
    }
    pub unsafe fn glGetTextureParameterIiv(
        &self,
        _texture: GLuint,
        _pname: GetTextureParameter,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glUniformBlockBinding(
        &self,
        _program: GLuint,
        _uniformBlockIndex: GLuint,
        _uniformBlockBinding: GLuint,
    ) {
    }
    pub unsafe fn glGetTexLevelParameterfv(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _pname: GetTextureParameter,
        _params: *mut GLfloat,
    ) {
    }
    pub unsafe fn glDepthRangef(&self, _n: GLfloat, _f: GLfloat) {}
    pub unsafe fn glCreateQueries(&self, _target: QueryTarget, _n: GLsizei, _ids: *mut GLuint) {}
    pub unsafe fn glGetSamplerParameteriv(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterI,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glTextureParameterIuiv(
        &self,
        _texture: GLuint,
        _pname: TextureParameterName,
        _params: *const GLuint,
    ) {
    }
    pub unsafe fn glVertexAttrib2f(&self, _index: GLuint, _x: GLfloat, _y: GLfloat) {}
    pub unsafe fn glStencilMaskSeparate(&self, _face: StencilFaceDirection, _mask: GLuint) {}
    pub unsafe fn glInvalidateTexSubImage(
        &self,
        _texture: GLuint,
        _level: GLint,
        _xoffset: GLint,
        _yoffset: GLint,
        _zoffset: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _depth: GLsizei,
    ) {
    }
    pub unsafe fn glGetUniformiv(&self, _program: GLuint, _location: GLint, _params: *mut GLint) {}
    pub unsafe fn glGenBuffers(&self, _n: GLsizei, _buffers: *mut GLuint) {}
    pub unsafe fn glBindBufferRange(
        &self,
        _target: BufferTargetARB,
        _index: GLuint,
        _buffer: GLuint,
        _offset: GLintptr,
        _size: GLsizeiptr,
    ) {
    }
    pub unsafe fn glUniform2uiv(&self, _location: GLint, _count: GLsizei, _value: *const GLuint) {}
    pub unsafe fn glVertexAttribI3iv(&self, _index: GLuint, _v: *const GLint) {}
    pub unsafe fn glGetSubroutineUniformLocation(
        &self,
        _program: GLuint,
        _shadertype: ShaderType,
        _name: *const GLchar,
    ) {
    }
    pub unsafe fn glRenderbufferStorage(
        &self,
        _target: RenderbufferTarget,
        _internalformat: InternalFormat,
        _width: GLsizei,
        _height: GLsizei,
    ) {
    }
    pub unsafe fn glDrawTransformFeedbackStream(
        &self,
        _mode: PrimitiveType,
        _id: GLuint,
        _stream: GLuint,
    ) {
    }
    pub unsafe fn glBindProgramPipeline(&self, _pipeline: GLuint) {}
    pub unsafe fn glTexParameterIiv(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _params: *const GLint,
    ) {
    }
    pub unsafe fn glPatchParameteri(&self, _pname: PatchParameterName, _value: GLint) {}
    pub unsafe fn glScissorArrayv(&self, _first: GLuint, _count: GLsizei, _v: *const GLint) {}
    pub unsafe fn glTexStorage3D(
        &self,
        _target: TextureTarget,
        _levels: GLsizei,
        _internalformat: SizedInternalFormat,
        _width: GLsizei,
        _height: GLsizei,
        _depth: GLsizei,
    ) {
    }
    pub unsafe fn glGetFramebufferParameteriv(
        &self,
        _target: FramebufferTarget,
        _pname: FramebufferAttachmentParameterName,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glMultiDrawArraysIndirect(
        &self,
        _mode: PrimitiveType,
        _indirect: *const std::os::raw::c_void,
        _drawcount: GLsizei,
        _stride: GLsizei,
    ) {
    }
    pub unsafe fn glGetObjectPtrLabel(
        &self,
        _ptr: *const std::os::raw::c_void,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _label: *mut GLchar,
    ) {
    }
    pub unsafe fn glGetVertexArrayIndexed64iv(
        &self,
        _vaobj: GLuint,
        _index: GLuint,
        _pname: VertexArrayPName,
        _param: *mut GLint64,
    ) {
    }
    pub unsafe fn glGetTextureSubImage(
        &self,
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
    ) {
    }
    pub unsafe fn glProgramUniformMatrix2fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
    }
    pub unsafe fn glBlendColor(
        &self,
        _red: GLfloat,
        _green: GLfloat,
        _blue: GLfloat,
        _alpha: GLfloat,
    ) {
    }
    pub unsafe fn glVertexAttribP4uiv(
        &self,
        _index: GLuint,
        _type: VertexAttribPointerType,
        _normalized: GLboolean,
        _value: *const GLuint,
    ) {
    }
    pub unsafe fn glGetBufferPointerv(
        &self,
        _target: BufferTargetARB,
        _pname: BufferPointerNameARB,
        _params: *mut *mut std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glTexCoordP3ui(&self, _type: TexCoordPointerType, _coords: GLuint) {}
    pub unsafe fn glVertexAttribI4bv(&self, _index: GLuint, _v: *const GLbyte) {}
    pub unsafe fn glUniform3fv(&self, _location: GLint, _count: GLsizei, _value: *const GLfloat) {}
    pub unsafe fn glMultiTexCoordP1ui(
        &self,
        _texture: TextureUnit,
        _type: TexCoordPointerType,
        _coords: GLuint,
    ) {
    }
    pub unsafe fn glMultiTexCoordP2uiv(
        &self,
        _texture: TextureUnit,
        _type: TexCoordPointerType,
        _coords: *const GLuint,
    ) {
    }
    pub unsafe fn glGetObjectLabel(
        &self,
        _identifier: ObjectIdentifier,
        _name: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _label: *mut GLchar,
    ) {
    }
    pub unsafe fn glTexCoordP3uiv(&self, _type: TexCoordPointerType, _coords: *const GLuint) {}
    pub unsafe fn glUniformMatrix2x3dv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
    }
    pub unsafe fn glTexStorage2DMultisample(
        &self,
        _target: TextureTarget,
        _samples: GLsizei,
        _internalformat: SizedInternalFormat,
        _width: GLsizei,
        _height: GLsizei,
        _fixedsamplelocations: GLboolean,
    ) {
    }
    pub unsafe fn glNamedBufferSubData(
        &self,
        _buffer: GLuint,
        _offset: GLintptr,
        _size: GLsizeiptr,
        _data: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glUniform2ui(&self, _location: GLint, _v0: GLuint, _v1: GLuint) {}
    pub unsafe fn glSampleCoverage(&self, _value: GLfloat, _invert: GLboolean) {}
    pub unsafe fn glGetInternalformativ(
        &self,
        _target: TextureTarget,
        _internalformat: InternalFormat,
        _pname: InternalFormatPName,
        _count: GLsizei,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glGetCompressedTextureSubImage(
        &self,
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
    ) {
    }
    pub unsafe fn glProgramUniform2i(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLint,
        _v1: GLint,
    ) {
    }
    pub unsafe fn glUniformMatrix3x2fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
    }
    pub unsafe fn glMultiTexCoordP4ui(
        &self,
        _texture: TextureUnit,
        _type: TexCoordPointerType,
        _coords: GLuint,
    ) {
    }
    pub unsafe fn glBindBufferBase(
        &self,
        _target: BufferTargetARB,
        _index: GLuint,
        _buffer: GLuint,
    ) {
    }
    pub unsafe fn glTextureView(
        &self,
        _texture: GLuint,
        _target: TextureTarget,
        _origtexture: GLuint,
        _internalformat: SizedInternalFormat,
        _minlevel: GLuint,
        _numlevels: GLuint,
        _minlayer: GLuint,
        _numlayers: GLuint,
    ) {
    }
    pub unsafe fn glDebugMessageControl(
        &self,
        _source: DebugSource,
        _type: DebugType,
        _severity: DebugSeverity,
        _count: GLsizei,
        _ids: *const GLuint,
        _enabled: GLboolean,
    ) {
    }
    pub unsafe fn glGetnPolygonStipple(&self, _bufSize: GLsizei, _pattern: *mut GLubyte) {}
    pub unsafe fn glColorMask(
        &self,
        _red: GLboolean,
        _green: GLboolean,
        _blue: GLboolean,
        _alpha: GLboolean,
    ) {
    }
    pub unsafe fn glGetQueryObjectuiv(
        &self,
        _id: GLuint,
        _pname: QueryObjectParameterName,
        _params: *mut GLuint,
    ) {
    }
    pub unsafe fn glGetActiveSubroutineUniformName(
        &self,
        _program: GLuint,
        _shadertype: ShaderType,
        _index: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _name: *mut GLchar,
    ) {
    }
    pub unsafe fn glGenerateTextureMipmap(&self, _texture: GLuint) {}
    pub unsafe fn glGetSubroutineIndex(
        &self,
        _program: GLuint,
        _shadertype: ShaderType,
        _name: *const GLchar,
    ) {
    }
    pub unsafe fn glUniform1f(&self, _location: GLint, _v0: GLfloat) {}
    pub unsafe fn glGetTexParameterfv(
        &self,
        _target: TextureTarget,
        _pname: GetTextureParameter,
        _params: *mut GLfloat,
    ) {
    }
    pub unsafe fn glGetnMapdv(
        &self,
        _target: MapTarget,
        _query: MapQuery,
        _bufSize: GLsizei,
        _v: *mut GLdouble,
    ) {
    }
    pub unsafe fn glStencilFuncSeparate(
        &self,
        _face: StencilFaceDirection,
        _func: StencilFunction,
        _ref: GLint,
        _mask: GLuint,
    ) {
    }
    pub unsafe fn glPixelStorei(&self, _pname: PixelStoreParameter, _param: GLint) {}
    pub unsafe fn glUniform4i(
        &self,
        _location: GLint,
        _v0: GLint,
        _v1: GLint,
        _v2: GLint,
        _v3: GLint,
    ) {
    }
    pub unsafe fn glCreateSamplers(&self, _n: GLsizei, _samplers: *mut GLuint) {}
    pub unsafe fn glGetnUniformdv(
        &self,
        _program: GLuint,
        _location: GLint,
        _bufSize: GLsizei,
        _params: *mut GLdouble,
    ) {
    }
    pub unsafe fn glGetVertexAttribfv(
        &self,
        _index: GLuint,
        _pname: VertexAttribPropertyARB,
        _params: *mut GLfloat,
    ) {
    }
    pub unsafe fn glEndConditionalRender(&self) {}
    pub unsafe fn glMultiDrawElements(
        &self,
        _mode: PrimitiveType,
        _count: *const GLsizei,
        _type: DrawElementsType,
        _indices: *const *const std::os::raw::c_void,
        _drawcount: GLsizei,
    ) {
    }
    pub unsafe fn glTexStorage2D(
        &self,
        _target: TextureTarget,
        _levels: GLsizei,
        _internalformat: SizedInternalFormat,
        _width: GLsizei,
        _height: GLsizei,
    ) {
    }
    pub unsafe fn glTransformFeedbackBufferRange(
        &self,
        _xfb: GLuint,
        _index: GLuint,
        _buffer: GLuint,
        _offset: GLintptr,
        _size: GLsizeiptr,
    ) {
    }
    pub unsafe fn glProgramUniform2uiv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLuint,
    ) {
    }
    pub unsafe fn glGetIntegerv(&self, _pname: GetPName, _data: *mut GLint) {}
    pub unsafe fn glVertexAttrib4sv(&self, _index: GLuint, _v: *const GLshort) {}
    pub unsafe fn glGetSamplerParameterIuiv(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterI,
        _params: *mut GLuint,
    ) {
    }
    pub unsafe fn glProgramUniformMatrix3x2dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
    }
    pub unsafe fn glTextureSubImage2D(
        &self,
        _texture: GLuint,
        _level: GLint,
        _xoffset: GLint,
        _yoffset: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _format: PixelFormat,
        _type: PixelType,
        _pixels: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glCopyTextureSubImage3D(
        &self,
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
    }
    pub unsafe fn glMapNamedBuffer(&self, _buffer: GLuint, _access: BufferAccessARB) {}
    pub unsafe fn glVertexAttribL3dv(&self, _index: GLuint, _v: *const GLdouble) {}
    pub unsafe fn glVertexAttribI4iv(&self, _index: GLuint, _v: *const GLint) {}
    pub unsafe fn glDrawArraysInstanced(
        &self,
        _mode: PrimitiveType,
        _first: GLint,
        _count: GLsizei,
        _instancecount: GLsizei,
    ) {
    }
    pub unsafe fn glVertexAttribL3d(
        &self,
        _index: GLuint,
        _x: GLdouble,
        _y: GLdouble,
        _z: GLdouble,
    ) {
    }
    pub unsafe fn glUnmapNamedBuffer(&self, _buffer: GLuint) {}
    pub unsafe fn glGetQueryBufferObjectiv(
        &self,
        _id: GLuint,
        _buffer: GLuint,
        _pname: QueryObjectParameterName,
        _offset: GLintptr,
    ) {
    }
    pub unsafe fn glIsVertexArray(&self, _array: GLuint) {}
    pub unsafe fn glBindBuffer(&self, _target: BufferTargetARB, _buffer: GLuint) {}
    pub unsafe fn glUniform3uiv(&self, _location: GLint, _count: GLsizei, _value: *const GLuint) {}
    pub unsafe fn glDrawElementsInstanced(
        &self,
        _mode: PrimitiveType,
        _count: GLsizei,
        _type: DrawElementsType,
        _indices: *const std::os::raw::c_void,
        _instancecount: GLsizei,
    ) {
    }
    pub unsafe fn glIsSync(&self, _sync: GLsync) {}
    pub unsafe fn glMinSampleShading(&self, _value: GLfloat) {}
    pub unsafe fn glResumeTransformFeedback(&self) {}
    pub unsafe fn glIsProgramPipeline(&self, _pipeline: GLuint) {}
    pub unsafe fn glProgramUniform4uiv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLuint,
    ) {
    }
    pub unsafe fn glGetBufferParameteriv(
        &self,
        _target: BufferTargetARB,
        _pname: BufferPNameARB,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glFinish(&self) {}
    pub unsafe fn glClearNamedBufferSubData(
        &self,
        _buffer: GLuint,
        _internalformat: SizedInternalFormat,
        _offset: GLintptr,
        _size: GLsizeiptr,
        _format: PixelFormat,
        _type: PixelType,
        _data: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glVertexAttribP4ui(
        &self,
        _index: GLuint,
        _type: VertexAttribPointerType,
        _normalized: GLboolean,
        _value: GLuint,
    ) {
    }
    pub unsafe fn glInvalidateBufferSubData(
        &self,
        _buffer: GLuint,
        _offset: GLintptr,
        _length: GLsizeiptr,
    ) {
    }
    pub unsafe fn glVertexAttribL1dv(&self, _index: GLuint, _v: *const GLdouble) {}
    pub unsafe fn glUniform1fv(&self, _location: GLint, _count: GLsizei, _value: *const GLfloat) {}
    pub unsafe fn glUniform4ui(
        &self,
        _location: GLint,
        _v0: GLuint,
        _v1: GLuint,
        _v2: GLuint,
        _v3: GLuint,
    ) {
    }
    pub unsafe fn glTextureBufferRange(
        &self,
        _texture: GLuint,
        _internalformat: SizedInternalFormat,
        _buffer: GLuint,
        _offset: GLintptr,
        _size: GLsizeiptr,
    ) {
    }
    pub unsafe fn glGetProgramiv(
        &self,
        _program: GLuint,
        _pname: ProgramPropertyARB,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glVertexAttribI4uiv(&self, _index: GLuint, _v: *const GLuint) {}
    pub unsafe fn glCreateTransformFeedbacks(&self, _n: GLsizei, _ids: *mut GLuint) {}
    pub unsafe fn glBeginTransformFeedback(&self, _primitiveMode: PrimitiveType) {}
    pub unsafe fn glUseProgramStages(
        &self,
        _pipeline: GLuint,
        _stages: UseProgramStageMask,
        _program: GLuint,
    ) {
    }
    pub unsafe fn glPushDebugGroup(
        &self,
        _source: DebugSource,
        _id: GLuint,
        _length: GLsizei,
        _message: *const GLchar,
    ) {
    }
    pub unsafe fn glGetnUniformfv(
        &self,
        _program: GLuint,
        _location: GLint,
        _bufSize: GLsizei,
        _params: *mut GLfloat,
    ) {
    }
    pub unsafe fn glUniformMatrix4x3fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
    }
    pub unsafe fn glBindTextures(&self, _first: GLuint, _count: GLsizei, _textures: *const GLuint) {
    }
    pub unsafe fn glGetError(&self) {}
    pub unsafe fn glVertexAttrib4d(
        &self,
        _index: GLuint,
        _x: GLdouble,
        _y: GLdouble,
        _z: GLdouble,
        _w: GLdouble,
    ) {
    }
    pub unsafe fn glUniform4iv(&self, _location: GLint, _count: GLsizei, _value: *const GLint) {}
    pub unsafe fn glGetIntegeri_v(&self, _target: GetPName, _index: GLuint, _data: *mut GLint) {}
    pub unsafe fn glClearBufferfv(
        &self,
        _buffer: Buffer,
        _drawbuffer: GLint,
        _value: *const GLfloat,
    ) {
    }
    pub unsafe fn glSamplerParameterf(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterF,
        _param: GLfloat,
    ) {
    }
    pub unsafe fn glVertexAttribI2iv(&self, _index: GLuint, _v: *const GLint) {}
    pub unsafe fn glGetShaderPrecisionFormat(
        &self,
        _shadertype: ShaderType,
        _precisiontype: PrecisionType,
        _range: *mut GLint,
        _precision: *mut GLint,
    ) {
    }
    pub unsafe fn glUseProgram(&self, _program: GLuint) {}
    pub unsafe fn glProgramBinary(
        &self,
        _program: GLuint,
        _binaryFormat: GLenum,
        _binary: *const std::os::raw::c_void,
        _length: GLsizei,
    ) {
    }
    pub unsafe fn glCompressedTexImage1D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _internalformat: InternalFormat,
        _width: GLsizei,
        _border: GLint,
        _imageSize: GLsizei,
        _data: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glUniform3i(&self, _location: GLint, _v0: GLint, _v1: GLint, _v2: GLint) {}
    pub unsafe fn glProgramUniform3i(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLint,
        _v1: GLint,
        _v2: GLint,
    ) {
    }
    pub unsafe fn glProgramUniformMatrix4x3dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
    }
    pub unsafe fn glProgramUniform4dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLdouble,
    ) {
    }
    pub unsafe fn glVertexP3ui(&self, _type: VertexPointerType, _value: GLuint) {}
    pub unsafe fn glUnmapBuffer(&self, _target: BufferTargetARB) {}
    pub unsafe fn glGetFloati_v(&self, _target: GetPName, _index: GLuint, _data: *mut GLfloat) {}
    pub unsafe fn glVertexAttribIFormat(
        &self,
        _attribindex: GLuint,
        _size: GLint,
        _type: VertexAttribIType,
        _relativeoffset: GLuint,
    ) {
    }
    pub unsafe fn glDepthMask(&self, _flag: GLboolean) {}
    pub unsafe fn glGetProgramInfoLog(
        &self,
        _program: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _infoLog: *mut GLchar,
    ) {
    }
    pub unsafe fn glReadPixels(
        &self,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _format: PixelFormat,
        _type: PixelType,
        _pixels: *mut std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glBindBuffersRange(
        &self,
        _target: BufferTargetARB,
        _first: GLuint,
        _count: GLsizei,
        _buffers: *const GLuint,
        _offsets: *const GLintptr,
        _sizes: *const GLsizeiptr,
    ) {
    }
    pub unsafe fn glTexCoordP4uiv(&self, _type: TexCoordPointerType, _coords: *const GLuint) {}
    pub unsafe fn glCreateVertexArrays(&self, _n: GLsizei, _arrays: *mut GLuint) {}
    pub unsafe fn glProgramUniformMatrix3x2fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
    }
    pub unsafe fn glTextureBarrier(&self) {}
    pub unsafe fn glCopyImageSubData(
        &self,
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
    }
    pub unsafe fn glNamedFramebufferTextureLayer(
        &self,
        _framebuffer: GLuint,
        _attachment: FramebufferAttachment,
        _texture: GLuint,
        _level: GLint,
        _layer: GLint,
    ) {
    }
    pub unsafe fn glCopyTexSubImage1D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _xoffset: GLint,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
    ) {
    }
    pub unsafe fn glBufferData(
        &self,
        _target: BufferTargetARB,
        _size: GLsizeiptr,
        _data: *const std::os::raw::c_void,
        _usage: BufferUsageARB,
    ) {
    }
    pub unsafe fn glPauseTransformFeedback(&self) {}
    pub unsafe fn glCompileShader(&self, _shader: GLuint) {}
    pub unsafe fn glInvalidateNamedFramebufferData(
        &self,
        _framebuffer: GLuint,
        _numAttachments: GLsizei,
        _attachments: *const FramebufferAttachment,
    ) {
    }
    pub unsafe fn glTexBufferRange(
        &self,
        _target: TextureTarget,
        _internalformat: SizedInternalFormat,
        _buffer: GLuint,
        _offset: GLintptr,
        _size: GLsizeiptr,
    ) {
    }
    pub unsafe fn glVertexArrayAttribIFormat(
        &self,
        _vaobj: GLuint,
        _attribindex: GLuint,
        _size: GLint,
        _type: VertexAttribIType,
        _relativeoffset: GLuint,
    ) {
    }
    pub unsafe fn glGetString(&self, _name: StringName) {}
    pub unsafe fn glInvalidateNamedFramebufferSubData(
        &self,
        _framebuffer: GLuint,
        _numAttachments: GLsizei,
        _attachments: *const FramebufferAttachment,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
        _height: GLsizei,
    ) {
    }
    pub unsafe fn glGetnMapfv(
        &self,
        _target: MapTarget,
        _query: MapQuery,
        _bufSize: GLsizei,
        _v: *mut GLfloat,
    ) {
    }
    pub unsafe fn glDrawArraysInstancedBaseInstance(
        &self,
        _mode: PrimitiveType,
        _first: GLint,
        _count: GLsizei,
        _instancecount: GLsizei,
        _baseinstance: GLuint,
    ) {
    }
    pub unsafe fn glVertexAttribLFormat(
        &self,
        _attribindex: GLuint,
        _size: GLint,
        _type: VertexAttribLType,
        _relativeoffset: GLuint,
    ) {
    }
    pub unsafe fn glProgramUniform2f(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLfloat,
        _v1: GLfloat,
    ) {
    }
    pub unsafe fn glCompressedTexSubImage3D(
        &self,
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
    ) {
    }
    pub unsafe fn glVertexAttrib3dv(&self, _index: GLuint, _v: *const GLdouble) {}
    pub unsafe fn glProgramUniform4i(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLint,
        _v1: GLint,
        _v2: GLint,
        _v3: GLint,
    ) {
    }
    pub unsafe fn glColorP3ui(&self, _type: ColorPointerType, _color: GLuint) {}
    pub unsafe fn glClientWaitSync(
        &self,
        _sync: GLsync,
        _flags: SyncObjectMask,
        _timeout: GLuint64,
    ) {
    }
    pub unsafe fn glMultiDrawArrays(
        &self,
        _mode: PrimitiveType,
        _first: *const GLint,
        _count: *const GLsizei,
        _drawcount: GLsizei,
    ) {
    }
    pub unsafe fn glGetFragDataLocation(&self, _program: GLuint, _name: *const GLchar) {}
    pub unsafe fn glFlushMappedNamedBufferRange(
        &self,
        _buffer: GLuint,
        _offset: GLintptr,
        _length: GLsizeiptr,
    ) {
    }
    pub unsafe fn glBlendEquationSeparatei(
        &self,
        _buf: GLuint,
        _modeRGB: BlendEquationModeEXT,
        _modeAlpha: BlendEquationModeEXT,
    ) {
    }
    pub unsafe fn glVertexAttrib4iv(&self, _index: GLuint, _v: *const GLint) {}
    pub unsafe fn glTexParameterIuiv(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _params: *const GLuint,
    ) {
    }
    pub unsafe fn glPopDebugGroup(&self) {}
    pub unsafe fn glColorMaski(
        &self,
        _index: GLuint,
        _r: GLboolean,
        _g: GLboolean,
        _b: GLboolean,
        _a: GLboolean,
    ) {
    }
    pub unsafe fn glFenceSync(&self, _condition: SyncCondition, _flags: SyncBehaviorFlags) {}
    pub unsafe fn glBeginConditionalRender(&self, _id: GLuint, _mode: ConditionalRenderMode) {}
    pub unsafe fn glIsEnabled(&self, _cap: EnableCap) {}
    pub unsafe fn glProgramUniform3dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLdouble,
    ) {
    }
    pub unsafe fn glClearNamedFramebufferuiv(
        &self,
        _framebuffer: GLuint,
        _buffer: Buffer,
        _drawbuffer: GLint,
        _value: *const GLuint,
    ) {
    }
    pub unsafe fn glGetQueryObjecti64v(
        &self,
        _id: GLuint,
        _pname: QueryObjectParameterName,
        _params: *mut GLint64,
    ) {
    }
    pub unsafe fn glClearStencil(&self, _s: GLint) {}
    pub unsafe fn glUniformSubroutinesuiv(
        &self,
        _shadertype: ShaderType,
        _count: GLsizei,
        _indices: *const GLuint,
    ) {
    }
    pub unsafe fn glCopyTextureSubImage2D(
        &self,
        _texture: GLuint,
        _level: GLint,
        _xoffset: GLint,
        _yoffset: GLint,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
        _height: GLsizei,
    ) {
    }
    pub unsafe fn glGetProgramBinary(
        &self,
        _program: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _binaryFormat: *mut GLenum,
        _binary: *mut std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glProgramUniformMatrix4fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
    }
    pub unsafe fn glTexImage2D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _internalformat: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _border: GLint,
        _format: PixelFormat,
        _type: PixelType,
        _pixels: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glGetUniformfv(&self, _program: GLuint, _location: GLint, _params: *mut GLfloat) {
    }
    pub unsafe fn glGetUniformLocation(&self, _program: GLuint, _name: *const GLchar) {}
    pub unsafe fn glIsShader(&self, _shader: GLuint) {}
    pub unsafe fn glVertexAttrib4usv(&self, _index: GLuint, _v: *const GLushort) {}
    pub unsafe fn glUniform3f(&self, _location: GLint, _v0: GLfloat, _v1: GLfloat, _v2: GLfloat) {}
    pub unsafe fn glTexCoordP2ui(&self, _type: TexCoordPointerType, _coords: GLuint) {}
    pub unsafe fn glGetActiveSubroutineUniformiv(
        &self,
        _program: GLuint,
        _shadertype: ShaderType,
        _index: GLuint,
        _pname: SubroutineParameterName,
        _values: *mut GLint,
    ) {
    }
    pub unsafe fn glClearNamedFramebufferfv(
        &self,
        _framebuffer: GLuint,
        _buffer: Buffer,
        _drawbuffer: GLint,
        _value: *const GLfloat,
    ) {
    }
    pub unsafe fn glIsQuery(&self, _id: GLuint) {}
    pub unsafe fn glBindImageTexture(
        &self,
        _unit: GLuint,
        _texture: GLuint,
        _level: GLint,
        _layered: GLboolean,
        _layer: GLint,
        _access: BufferAccessARB,
        _format: InternalFormat,
    ) {
    }
    pub unsafe fn glVertexAttrib4f(
        &self,
        _index: GLuint,
        _x: GLfloat,
        _y: GLfloat,
        _z: GLfloat,
        _w: GLfloat,
    ) {
    }
    pub unsafe fn glTexCoordP1uiv(&self, _type: TexCoordPointerType, _coords: *const GLuint) {}
    pub unsafe fn glMultiTexCoordP1uiv(
        &self,
        _texture: TextureUnit,
        _type: TexCoordPointerType,
        _coords: *const GLuint,
    ) {
    }
    pub unsafe fn glDepthFunc(&self, _func: DepthFunction) {}
    pub unsafe fn glClearDepthf(&self, _d: GLfloat) {}
    pub unsafe fn glNamedBufferData(
        &self,
        _buffer: GLuint,
        _size: GLsizeiptr,
        _data: *const std::os::raw::c_void,
        _usage: VertexBufferObjectUsage,
    ) {
    }
    pub unsafe fn glNamedFramebufferReadBuffer(&self, _framebuffer: GLuint, _src: ColorBuffer) {}
    pub unsafe fn glVertexBindingDivisor(&self, _bindingindex: GLuint, _divisor: GLuint) {}
    pub unsafe fn glBufferSubData(
        &self,
        _target: BufferTargetARB,
        _offset: GLintptr,
        _size: GLsizeiptr,
        _data: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glUniformMatrix2x3fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
    }
    pub unsafe fn glDeleteRenderbuffers(&self, _n: GLsizei, _renderbuffers: *const GLuint) {}
    pub unsafe fn glViewportIndexedfv(&self, _index: GLuint, _v: *const GLfloat) {}
    pub unsafe fn glUniform3ui(&self, _location: GLint, _v0: GLuint, _v1: GLuint, _v2: GLuint) {}
    pub unsafe fn glVertexAttrib4dv(&self, _index: GLuint, _v: *const GLdouble) {}
    pub unsafe fn glVertexAttrib3sv(&self, _index: GLuint, _v: *const GLshort) {}
    pub unsafe fn glVertexArrayAttribBinding(
        &self,
        _vaobj: GLuint,
        _attribindex: GLuint,
        _bindingindex: GLuint,
    ) {
    }
    pub unsafe fn glGetGraphicsResetStatus(&self) {}
    pub unsafe fn glUniformMatrix4x2fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
    }
    pub unsafe fn glGetVertexAttribLdv(
        &self,
        _index: GLuint,
        _pname: VertexAttribEnum,
        _params: *mut GLdouble,
    ) {
    }
    pub unsafe fn glIsFramebuffer(&self, _framebuffer: GLuint) {}
    pub unsafe fn glVertexAttrib1sv(&self, _index: GLuint, _v: *const GLshort) {}
    pub unsafe fn glNamedBufferStorage(
        &self,
        _buffer: GLuint,
        _size: GLsizeiptr,
        _data: *const std::os::raw::c_void,
        _flags: BufferStorageMask,
    ) {
    }
    pub unsafe fn glGetBooleani_v(
        &self,
        _target: BufferTargetARB,
        _index: GLuint,
        _data: *mut GLboolean,
    ) {
    }
    pub unsafe fn glGetnPixelMapuiv(
        &self,
        _map: PixelMap,
        _bufSize: GLsizei,
        _values: *mut GLuint,
    ) {
    }
    pub unsafe fn glUniform4dv(&self, _location: GLint, _count: GLsizei, _value: *const GLdouble) {}
    pub unsafe fn glDeleteSync(&self, _sync: GLsync) {}
    pub unsafe fn glUniform2fv(&self, _location: GLint, _count: GLsizei, _value: *const GLfloat) {}
    pub unsafe fn glProgramUniform3ui(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLuint,
        _v1: GLuint,
        _v2: GLuint,
    ) {
    }
    pub unsafe fn glVertexAttribP1ui(
        &self,
        _index: GLuint,
        _type: VertexAttribPointerType,
        _normalized: GLboolean,
        _value: GLuint,
    ) {
    }
    pub unsafe fn glScissorIndexedv(&self, _index: GLuint, _v: *const GLint) {}
    pub unsafe fn glPointParameteriv(&self, _pname: PointParameterNameARB, _params: *const GLint) {}
    pub unsafe fn glFramebufferTexture3D(
        &self,
        _target: FramebufferTarget,
        _attachment: FramebufferAttachment,
        _textarget: TextureTarget,
        _texture: GLuint,
        _level: GLint,
        _zoffset: GLint,
    ) {
    }
    pub unsafe fn glProgramUniform4d(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLdouble,
        _v1: GLdouble,
        _v2: GLdouble,
        _v3: GLdouble,
    ) {
    }
    pub unsafe fn glUniform2iv(&self, _location: GLint, _count: GLsizei, _value: *const GLint) {}
    pub unsafe fn glGenRenderbuffers(&self, _n: GLsizei, _renderbuffers: *mut GLuint) {}
    pub unsafe fn glVertexAttrib2dv(&self, _index: GLuint, _v: *const GLdouble) {}
    pub unsafe fn glProgramUniformMatrix2x3dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
    }
    pub unsafe fn glFlush(&self) {}
    pub unsafe fn glUniformMatrix4x2dv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
    }
    pub unsafe fn glGetTransformFeedbackVarying(
        &self,
        _program: GLuint,
        _index: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _size: *mut GLsizei,
        _type: *mut AttributeType,
        _name: *mut GLchar,
    ) {
    }
    pub unsafe fn glUniform4d(
        &self,
        _location: GLint,
        _x: GLdouble,
        _y: GLdouble,
        _z: GLdouble,
        _w: GLdouble,
    ) {
    }
    pub unsafe fn glClearNamedFramebufferfi(
        &self,
        _framebuffer: GLuint,
        _buffer: Buffer,
        _drawbuffer: GLint,
        _depth: GLfloat,
        _stencil: GLint,
    ) {
    }
    pub unsafe fn glDisableVertexAttribArray(&self, _index: GLuint) {}
    pub unsafe fn glGetActiveUniformsiv(
        &self,
        _program: GLuint,
        _uniformCount: GLsizei,
        _uniformIndices: *const GLuint,
        _pname: UniformPName,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glTexCoordP2uiv(&self, _type: TexCoordPointerType, _coords: *const GLuint) {}
    pub unsafe fn glVertexAttribL4dv(&self, _index: GLuint, _v: *const GLdouble) {}
    pub unsafe fn glGetNamedBufferParameteriv(
        &self,
        _buffer: GLuint,
        _pname: BufferPNameARB,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glVertexAttrib1f(&self, _index: GLuint, _x: GLfloat) {}
    pub unsafe fn glHint(&self, _target: HintTarget, _mode: HintMode) {}
    pub unsafe fn glVertexAttribP3ui(
        &self,
        _index: GLuint,
        _type: VertexAttribPointerType,
        _normalized: GLboolean,
        _value: GLuint,
    ) {
    }
    pub unsafe fn glCullFace(&self, _mode: CullFaceMode) {}
    pub unsafe fn glGetCompressedTexImage(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _img: *mut std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glStencilMask(&self, _mask: GLuint) {}
    pub unsafe fn glUniform3d(&self, _location: GLint, _x: GLdouble, _y: GLdouble, _z: GLdouble) {}
    pub unsafe fn glValidateProgramPipeline(&self, _pipeline: GLuint) {}
    pub unsafe fn glVertexAttrib4Niv(&self, _index: GLuint, _v: *const GLint) {}
    pub unsafe fn glGetShaderiv(
        &self,
        _shader: GLuint,
        _pname: ShaderParameterName,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glAttachShader(&self, _program: GLuint, _shader: GLuint) {}
    pub unsafe fn glShaderBinary(
        &self,
        _count: GLsizei,
        _shaders: *const GLuint,
        _binaryFormat: ShaderBinaryFormat,
        _binary: *const std::os::raw::c_void,
        _length: GLsizei,
    ) {
    }
    pub unsafe fn glBindTexture(&self, _target: TextureTarget, _texture: GLuint) {}
    pub unsafe fn glProgramUniformMatrix3fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
    }
    pub unsafe fn glGetTransformFeedbacki64_v(
        &self,
        _xfb: GLuint,
        _pname: TransformFeedbackPName,
        _index: GLuint,
        _param: *mut GLint64,
    ) {
    }
    pub unsafe fn glDrawElementsIndirect(
        &self,
        _mode: PrimitiveType,
        _type: DrawElementsType,
        _indirect: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glProgramUniform3iv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLint,
    ) {
    }
    pub unsafe fn glCopyNamedBufferSubData(
        &self,
        _readBuffer: GLuint,
        _writeBuffer: GLuint,
        _readOffset: GLintptr,
        _writeOffset: GLintptr,
        _size: GLsizeiptr,
    ) {
    }
    pub unsafe fn glDeleteQueries(&self, _n: GLsizei, _ids: *const GLuint) {}
    pub unsafe fn glVertexAttribLPointer(
        &self,
        _index: GLuint,
        _size: GLint,
        _type: VertexAttribLType,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glDeleteBuffers(&self, _n: GLsizei, _buffers: *const GLuint) {}
    pub unsafe fn glVertexAttribI1ui(&self, _index: GLuint, _x: GLuint) {}
    pub unsafe fn glCopyTexImage1D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _internalformat: InternalFormat,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
        _border: GLint,
    ) {
    }
    pub unsafe fn glClear(&self, _mask: ClearBufferMask) {}
    pub unsafe fn glGetUniformIndices(
        &self,
        _program: GLuint,
        _uniformCount: GLsizei,
        _uniformNames: *const *const GLchar,
        _uniformIndices: *mut GLuint,
    ) {
    }
    pub unsafe fn glDrawElementsInstancedBaseVertex(
        &self,
        _mode: PrimitiveType,
        _count: GLsizei,
        _type: DrawElementsType,
        _indices: *const std::os::raw::c_void,
        _instancecount: GLsizei,
        _basevertex: GLint,
    ) {
    }
    pub unsafe fn glSamplerParameteri(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterI,
        _param: GLint,
    ) {
    }
    pub unsafe fn glBlendFuncSeparatei(
        &self,
        _buf: GLuint,
        _srcRGB: BlendingFactor,
        _dstRGB: BlendingFactor,
        _srcAlpha: BlendingFactor,
        _dstAlpha: BlendingFactor,
    ) {
    }
    pub unsafe fn glProgramUniformMatrix2x4dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
    }
    pub unsafe fn glCompressedTextureSubImage3D(
        &self,
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
    ) {
    }
    pub unsafe fn glGetTextureParameteriv(
        &self,
        _texture: GLuint,
        _pname: GetTextureParameter,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glGetnConvolutionFilter(
        &self,
        _target: ConvolutionTarget,
        _format: PixelFormat,
        _type: PixelType,
        _bufSize: GLsizei,
        _image: *mut std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glStencilFunc(&self, _func: StencilFunction, _ref: GLint, _mask: GLuint) {}
    pub unsafe fn glSamplerParameterfv(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterF,
        _param: *const GLfloat,
    ) {
    }
    pub unsafe fn glVertexAttribDivisor(&self, _index: GLuint, _divisor: GLuint) {}
    pub unsafe fn glProgramUniform3d(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLdouble,
        _v1: GLdouble,
        _v2: GLdouble,
    ) {
    }
    pub unsafe fn glDebugMessageInsert(
        &self,
        _source: DebugSource,
        _type: DebugType,
        _id: GLuint,
        _severity: DebugSeverity,
        _length: GLsizei,
        _buf: *const GLchar,
    ) {
    }
    pub unsafe fn glIsEnabledi(&self, _target: EnableCap, _index: GLuint) {}
    pub unsafe fn glVertexAttrib4bv(&self, _index: GLuint, _v: *const GLbyte) {}
    pub unsafe fn glMultiDrawElementsBaseVertex(
        &self,
        _mode: PrimitiveType,
        _count: *const GLsizei,
        _type: DrawElementsType,
        _indices: *const *const std::os::raw::c_void,
        _drawcount: GLsizei,
        _basevertex: *const GLint,
    ) {
    }
    pub unsafe fn glBufferStorage(
        &self,
        _target: BufferStorageTarget,
        _size: GLsizeiptr,
        _data: *const std::os::raw::c_void,
        _flags: BufferStorageMask,
    ) {
    }
    pub unsafe fn glDepthRangeArrayv(&self, _first: GLuint, _count: GLsizei, _v: *const GLdouble) {}
    pub unsafe fn glEnable(&self, _cap: EnableCap) {}
    pub unsafe fn glEnableVertexAttribArray(&self, _index: GLuint) {}
    pub unsafe fn glUniformMatrix4fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
    }
    pub unsafe fn glDrawBuffers(&self, _n: GLsizei, _bufs: *const DrawBufferMode) {}
    pub unsafe fn glClearTexSubImage(
        &self,
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
    ) {
    }
    pub unsafe fn glMultiDrawElementsIndirectCount(
        &self,
        _mode: PrimitiveType,
        _type: DrawElementsType,
        _indirect: *const std::os::raw::c_void,
        _drawcount: GLintptr,
        _maxdrawcount: GLsizei,
        _stride: GLsizei,
    ) {
    }
    pub unsafe fn glPointParameteri(&self, _pname: PointParameterNameARB, _param: GLint) {}
    pub unsafe fn glScissor(&self, _x: GLint, _y: GLint, _width: GLsizei, _height: GLsizei) {}
    pub unsafe fn glBlendFunc(&self, _sfactor: BlendingFactor, _dfactor: BlendingFactor) {}
    pub unsafe fn glViewport(&self, _x: GLint, _y: GLint, _width: GLsizei, _height: GLsizei) {}
    pub unsafe fn glBlendEquationSeparate(
        &self,
        _modeRGB: BlendEquationModeEXT,
        _modeAlpha: BlendEquationModeEXT,
    ) {
    }
    pub unsafe fn glUniformMatrix3fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
    }
    pub unsafe fn glVertexAttribIPointer(
        &self,
        _index: GLuint,
        _size: GLint,
        _type: VertexAttribIType,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glProgramUniformMatrix2x3fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
    }
    pub unsafe fn glBindFragDataLocation(
        &self,
        _program: GLuint,
        _color: GLuint,
        _name: *const GLchar,
    ) {
    }
    pub unsafe fn glVertexAttribI4i(
        &self,
        _index: GLuint,
        _x: GLint,
        _y: GLint,
        _z: GLint,
        _w: GLint,
    ) {
    }
    pub unsafe fn glGetProgramPipelineiv(
        &self,
        _pipeline: GLuint,
        _pname: PipelineParameterName,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glTexImage3D(
        &self,
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
    ) {
    }
    pub unsafe fn glGetQueryiv(
        &self,
        _target: QueryTarget,
        _pname: QueryParameterName,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glVertexAttribBinding(&self, _attribindex: GLuint, _bindingindex: GLuint) {}
    pub unsafe fn glGetnPixelMapfv(
        &self,
        _map: PixelMap,
        _bufSize: GLsizei,
        _values: *mut GLfloat,
    ) {
    }
    pub unsafe fn glTextureStorage3DMultisample(
        &self,
        _texture: GLuint,
        _samples: GLsizei,
        _internalformat: SizedInternalFormat,
        _width: GLsizei,
        _height: GLsizei,
        _depth: GLsizei,
        _fixedsamplelocations: GLboolean,
    ) {
    }
    pub unsafe fn glVertexAttribI3i(&self, _index: GLuint, _x: GLint, _y: GLint, _z: GLint) {}
    pub unsafe fn glGetInteger64v(&self, _pname: GetPName, _data: *mut GLint64) {}
    pub unsafe fn glGetQueryBufferObjectui64v(
        &self,
        _id: GLuint,
        _buffer: GLuint,
        _pname: QueryObjectParameterName,
        _offset: GLintptr,
    ) {
    }
    pub unsafe fn glObjectLabel(
        &self,
        _identifier: ObjectIdentifier,
        _name: GLuint,
        _length: GLsizei,
        _label: *const GLchar,
    ) {
    }
    pub unsafe fn glGetTextureImage(
        &self,
        _texture: GLuint,
        _level: GLint,
        _format: PixelFormat,
        _type: PixelType,
        _bufSize: GLsizei,
        _pixels: *mut std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glBindVertexBuffer(
        &self,
        _bindingindex: GLuint,
        _buffer: GLuint,
        _offset: GLintptr,
        _stride: GLsizei,
    ) {
    }
    pub unsafe fn glGetnUniformiv(
        &self,
        _program: GLuint,
        _location: GLint,
        _bufSize: GLsizei,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glFramebufferParameteri(
        &self,
        _target: FramebufferTarget,
        _pname: FramebufferParameterName,
        _param: GLint,
    ) {
    }
    pub unsafe fn glDeleteTextures(&self, _n: GLsizei, _textures: *const GLuint) {}
    pub unsafe fn glActiveShaderProgram(&self, _pipeline: GLuint, _program: GLuint) {}
    pub unsafe fn glVertexAttribI1i(&self, _index: GLuint, _x: GLint) {}
    pub unsafe fn glTexParameteri(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _param: GLint,
    ) {
    }
    pub unsafe fn glVertexAttrib1d(&self, _index: GLuint, _x: GLdouble) {}
    pub unsafe fn glVertexAttrib4uiv(&self, _index: GLuint, _v: *const GLuint) {}
    pub unsafe fn glClearBufferiv(
        &self,
        _buffer: Buffer,
        _drawbuffer: GLint,
        _value: *const GLint,
    ) {
    }
    pub unsafe fn glVertexP2uiv(&self, _type: VertexPointerType, _value: *const GLuint) {}
    pub unsafe fn glProgramUniform4fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLfloat,
    ) {
    }
    pub unsafe fn glMemoryBarrierByRegion(&self, _barriers: MemoryBarrierMask) {}
    pub unsafe fn glUniform4uiv(&self, _location: GLint, _count: GLsizei, _value: *const GLuint) {}
    pub unsafe fn glGetnCompressedTexImage(
        &self,
        _target: TextureTarget,
        _lod: GLint,
        _bufSize: GLsizei,
        _pixels: *mut std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glTexParameteriv(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _params: *const GLint,
    ) {
    }
    pub unsafe fn glBindRenderbuffer(&self, _target: RenderbufferTarget, _renderbuffer: GLuint) {}
    pub unsafe fn glDrawBuffer(&self, _buf: DrawBufferMode) {}
    pub unsafe fn glGetActiveAtomicCounterBufferiv(
        &self,
        _program: GLuint,
        _bufferIndex: GLuint,
        _pname: AtomicCounterBufferPName,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glCompressedTextureSubImage2D(
        &self,
        _texture: GLuint,
        _level: GLint,
        _xoffset: GLint,
        _yoffset: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _format: InternalFormat,
        _imageSize: GLsizei,
        _data: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glPolygonMode(&self, _face: MaterialFace, _mode: PolygonMode) {}
    pub unsafe fn glGetStringi(&self, _name: StringName, _index: GLuint) {}
    pub unsafe fn glVertexAttribI4sv(&self, _index: GLuint, _v: *const GLshort) {}
    pub unsafe fn glGenTextures(&self, _n: GLsizei, _textures: *mut GLuint) {}
    pub unsafe fn glBindFramebuffer(&self, _target: FramebufferTarget, _framebuffer: GLuint) {}
    pub unsafe fn glFramebufferTexture(
        &self,
        _target: FramebufferTarget,
        _attachment: FramebufferAttachment,
        _texture: GLuint,
        _level: GLint,
    ) {
    }
    pub unsafe fn glDeleteFramebuffers(&self, _n: GLsizei, _framebuffers: *const GLuint) {}
    pub unsafe fn glGetUniformdv(
        &self,
        _program: GLuint,
        _location: GLint,
        _params: *mut GLdouble,
    ) {
    }
    pub unsafe fn glViewportIndexedf(
        &self,
        _index: GLuint,
        _x: GLfloat,
        _y: GLfloat,
        _w: GLfloat,
        _h: GLfloat,
    ) {
    }
    pub unsafe fn glMultiDrawElementsIndirect(
        &self,
        _mode: PrimitiveType,
        _type: DrawElementsType,
        _indirect: *const std::os::raw::c_void,
        _drawcount: GLsizei,
        _stride: GLsizei,
    ) {
    }
    pub unsafe fn glBindVertexBuffers(
        &self,
        _first: GLuint,
        _count: GLsizei,
        _buffers: *const GLuint,
        _offsets: *const GLintptr,
        _strides: *const GLsizei,
    ) {
    }
    pub unsafe fn glUniform4fv(&self, _location: GLint, _count: GLsizei, _value: *const GLfloat) {}
    pub unsafe fn glGenQueries(&self, _n: GLsizei, _ids: *mut GLuint) {}
    pub unsafe fn glVertexAttrib1dv(&self, _index: GLuint, _v: *const GLdouble) {}
    pub unsafe fn glStencilOp(&self, _fail: StencilOp, _zfail: StencilOp, _zpass: StencilOp) {}
    pub unsafe fn glEnablei(&self, _target: EnableCap, _index: GLuint) {}
    pub unsafe fn glCreateRenderbuffers(&self, _n: GLsizei, _renderbuffers: *mut GLuint) {}
    pub unsafe fn glGetnUniformuiv(
        &self,
        _program: GLuint,
        _location: GLint,
        _bufSize: GLsizei,
        _params: *mut GLuint,
    ) {
    }
    pub unsafe fn glGetProgramResourceLocation(
        &self,
        _program: GLuint,
        _programInterface: ProgramInterface,
        _name: *const GLchar,
    ) {
    }
    pub unsafe fn glBindVertexArray(&self, _array: GLuint) {}
    pub unsafe fn glProgramUniform2fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLfloat,
    ) {
    }
    pub unsafe fn glGenFramebuffers(&self, _n: GLsizei, _framebuffers: *mut GLuint) {}
    pub unsafe fn glGetNamedRenderbufferParameteriv(
        &self,
        _renderbuffer: GLuint,
        _pname: RenderbufferParameterName,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glDispatchComputeIndirect(&self, _indirect: GLintptr) {}
    pub unsafe fn glGetBooleanv(&self, _pname: GetPName, _data: *mut GLboolean) {}
    pub unsafe fn glProgramUniform3uiv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLuint,
    ) {
    }
    pub unsafe fn glCreateProgramPipelines(&self, _n: GLsizei, _pipelines: *mut GLuint) {}
    pub unsafe fn glGetTexImage(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _format: PixelFormat,
        _type: PixelType,
        _pixels: *mut std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glUniformMatrix4dv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
    }
    pub unsafe fn glPixelStoref(&self, _pname: PixelStoreParameter, _param: GLfloat) {}
    pub unsafe fn glGetnColorTable(
        &self,
        _target: ColorTableTarget,
        _format: PixelFormat,
        _type: PixelType,
        _bufSize: GLsizei,
        _table: *mut std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glGetVertexAttribiv(
        &self,
        _index: GLuint,
        _pname: VertexAttribPropertyARB,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glGetTextureLevelParameteriv(
        &self,
        _texture: GLuint,
        _level: GLint,
        _pname: GetTextureParameter,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glVertexAttribI4usv(&self, _index: GLuint, _v: *const GLushort) {}
    pub unsafe fn glProgramUniformMatrix2x4fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
    }
    pub unsafe fn glFramebufferRenderbuffer(
        &self,
        _target: FramebufferTarget,
        _attachment: FramebufferAttachment,
        _renderbuffertarget: RenderbufferTarget,
        _renderbuffer: GLuint,
    ) {
    }
    pub unsafe fn glGetFragDataIndex(&self, _program: GLuint, _name: *const GLchar) {}
    pub unsafe fn glReleaseShaderCompiler(&self) {}
    pub unsafe fn glVertexAttrib4Nuiv(&self, _index: GLuint, _v: *const GLuint) {}
    pub unsafe fn glNormalP3uiv(&self, _type: NormalPointerType, _coords: *const GLuint) {}
    pub unsafe fn glDepthRangeIndexed(&self, _index: GLuint, _n: GLdouble, _f: GLdouble) {}
    pub unsafe fn glTransformFeedbackVaryings(
        &self,
        _program: GLuint,
        _count: GLsizei,
        _varyings: *const *const GLchar,
        _bufferMode: TransformFeedbackBufferMode,
    ) {
    }
    pub unsafe fn glBindImageTextures(
        &self,
        _first: GLuint,
        _count: GLsizei,
        _textures: *const GLuint,
    ) {
    }
    pub unsafe fn glClearBufferfi(
        &self,
        _buffer: Buffer,
        _drawbuffer: GLint,
        _depth: GLfloat,
        _stencil: GLint,
    ) {
    }
    pub unsafe fn glWaitSync(&self, _sync: GLsync, _flags: SyncBehaviorFlags, _timeout: GLuint64) {}
    pub unsafe fn glLogicOp(&self, _opcode: LogicOp) {}
    pub unsafe fn glTexImage2DMultisample(
        &self,
        _target: TextureTarget,
        _samples: GLsizei,
        _internalformat: InternalFormat,
        _width: GLsizei,
        _height: GLsizei,
        _fixedsamplelocations: GLboolean,
    ) {
    }
    pub unsafe fn glMapNamedBufferRange(
        &self,
        _buffer: GLuint,
        _offset: GLintptr,
        _length: GLsizeiptr,
        _access: MapBufferAccessMask,
    ) {
    }
    pub unsafe fn glUniform1uiv(&self, _location: GLint, _count: GLsizei, _value: *const GLuint) {}
    pub unsafe fn glInvalidateSubFramebuffer(
        &self,
        _target: FramebufferTarget,
        _numAttachments: GLsizei,
        _attachments: *const InvalidateFramebufferAttachment,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
        _height: GLsizei,
    ) {
    }
    pub unsafe fn glCreateBuffers(&self, _n: GLsizei, _buffers: *mut GLuint) {}
    pub unsafe fn glGetQueryBufferObjecti64v(
        &self,
        _id: GLuint,
        _buffer: GLuint,
        _pname: QueryObjectParameterName,
        _offset: GLintptr,
    ) {
    }
    pub unsafe fn glGetUniformuiv(&self, _program: GLuint, _location: GLint, _params: *mut GLuint) {
    }
    pub unsafe fn glClearBufferuiv(
        &self,
        _buffer: Buffer,
        _drawbuffer: GLint,
        _value: *const GLuint,
    ) {
    }
    pub unsafe fn glVertexAttribP1uiv(
        &self,
        _index: GLuint,
        _type: VertexAttribPointerType,
        _normalized: GLboolean,
        _value: *const GLuint,
    ) {
    }
    pub unsafe fn glGetTransformFeedbacki_v(
        &self,
        _xfb: GLuint,
        _pname: TransformFeedbackPName,
        _index: GLuint,
        _param: *mut GLint,
    ) {
    }
    pub unsafe fn glVertexArrayAttribLFormat(
        &self,
        _vaobj: GLuint,
        _attribindex: GLuint,
        _size: GLint,
        _type: VertexAttribLType,
        _relativeoffset: GLuint,
    ) {
    }
    pub unsafe fn glVertexAttribL2dv(&self, _index: GLuint, _v: *const GLdouble) {}
    pub unsafe fn glVertexAttribI4ui(
        &self,
        _index: GLuint,
        _x: GLuint,
        _y: GLuint,
        _z: GLuint,
        _w: GLuint,
    ) {
    }
    pub unsafe fn glGetFloatv(&self, _pname: GetPName, _data: *mut GLfloat) {}
    pub unsafe fn glDeleteVertexArrays(&self, _n: GLsizei, _arrays: *const GLuint) {}
    pub unsafe fn glDrawArraysIndirect(
        &self,
        _mode: PrimitiveType,
        _indirect: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glGetDoublei_v(&self, _target: GetPName, _index: GLuint, _data: *mut GLdouble) {}
    pub unsafe fn glDisable(&self, _cap: EnableCap) {}
    pub unsafe fn glProgramUniform1dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLdouble,
    ) {
    }
    pub unsafe fn glTextureStorage3D(
        &self,
        _texture: GLuint,
        _levels: GLsizei,
        _internalformat: SizedInternalFormat,
        _width: GLsizei,
        _height: GLsizei,
        _depth: GLsizei,
    ) {
    }
    pub unsafe fn glTexParameterfv(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _params: *const GLfloat,
    ) {
    }
    pub unsafe fn glGetQueryBufferObjectuiv(
        &self,
        _id: GLuint,
        _buffer: GLuint,
        _pname: QueryObjectParameterName,
        _offset: GLintptr,
    ) {
    }
    pub unsafe fn glIsSampler(&self, _sampler: GLuint) {}
    pub unsafe fn glQueryCounter(&self, _id: GLuint, _target: QueryCounterTarget) {}
    pub unsafe fn glVertexAttrib4Nubv(&self, _index: GLuint, _v: *const GLubyte) {}
    pub unsafe fn glBlendEquationi(&self, _buf: GLuint, _mode: BlendEquationModeEXT) {}
    pub unsafe fn glScissorIndexed(
        &self,
        _index: GLuint,
        _left: GLint,
        _bottom: GLint,
        _width: GLsizei,
        _height: GLsizei,
    ) {
    }
    pub unsafe fn glTextureParameterIiv(
        &self,
        _texture: GLuint,
        _pname: TextureParameterName,
        _params: *const GLint,
    ) {
    }
    pub unsafe fn glProgramUniform2d(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLdouble,
        _v1: GLdouble,
    ) {
    }
    pub unsafe fn glGetActiveUniform(
        &self,
        _program: GLuint,
        _index: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _size: *mut GLint,
        _type: *mut UniformType,
        _name: *mut GLchar,
    ) {
    }
    pub unsafe fn glShaderSource(
        &self,
        _shader: GLuint,
        _count: GLsizei,
        _string: *const *const GLchar,
        _length: *const GLint,
    ) {
    }
    pub unsafe fn glProgramUniform1iv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLint,
    ) {
    }
    pub unsafe fn glGetTextureLevelParameterfv(
        &self,
        _texture: GLuint,
        _level: GLint,
        _pname: GetTextureParameter,
        _params: *mut GLfloat,
    ) {
    }
    pub unsafe fn glUniform2f(&self, _location: GLint, _v0: GLfloat, _v1: GLfloat) {}
    pub unsafe fn glVertexAttribI4ubv(&self, _index: GLuint, _v: *const GLubyte) {}
    pub unsafe fn glVertexP3uiv(&self, _type: VertexPointerType, _value: *const GLuint) {}
    pub unsafe fn glGetQueryObjectiv(
        &self,
        _id: GLuint,
        _pname: QueryObjectParameterName,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glUniform2d(&self, _location: GLint, _x: GLdouble, _y: GLdouble) {}
    pub unsafe fn glNamedFramebufferTexture(
        &self,
        _framebuffer: GLuint,
        _attachment: FramebufferAttachment,
        _texture: GLuint,
        _level: GLint,
    ) {
    }
    pub unsafe fn glGetProgramInterfaceiv(
        &self,
        _program: GLuint,
        _programInterface: ProgramInterface,
        _pname: ProgramInterfacePName,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glTexParameterf(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _param: GLfloat,
    ) {
    }
    pub unsafe fn glGetFramebufferAttachmentParameteriv(
        &self,
        _target: FramebufferTarget,
        _attachment: FramebufferAttachment,
        _pname: FramebufferAttachmentParameterName,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glBindSampler(&self, _unit: GLuint, _sampler: GLuint) {}
    pub unsafe fn glDispatchCompute(
        &self,
        _num_groups_x: GLuint,
        _num_groups_y: GLuint,
        _num_groups_z: GLuint,
    ) {
    }
    pub unsafe fn glGetPointerv(
        &self,
        _pname: GetPointervPName,
        _params: *mut *mut std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glTextureParameterf(
        &self,
        _texture: GLuint,
        _pname: TextureParameterName,
        _param: GLfloat,
    ) {
    }
    pub unsafe fn glDrawRangeElements(
        &self,
        _mode: PrimitiveType,
        _start: GLuint,
        _end: GLuint,
        _count: GLsizei,
        _type: DrawElementsType,
        _indices: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glFramebufferTexture1D(
        &self,
        _target: FramebufferTarget,
        _attachment: FramebufferAttachment,
        _textarget: TextureTarget,
        _texture: GLuint,
        _level: GLint,
    ) {
    }
    pub unsafe fn glPointParameterf(&self, _pname: PointParameterNameARB, _param: GLfloat) {}
    pub unsafe fn glVertexAttrib3fv(&self, _index: GLuint, _v: *const GLfloat) {}
    pub unsafe fn glActiveTexture(&self, _texture: TextureUnit) {}
    pub unsafe fn glProgramUniform1i(&self, _program: GLuint, _location: GLint, _v0: GLint) {}
    pub unsafe fn glClearColor(
        &self,
        _red: GLfloat,
        _green: GLfloat,
        _blue: GLfloat,
        _alpha: GLfloat,
    ) {
    }
    pub unsafe fn glUniform3iv(&self, _location: GLint, _count: GLsizei, _value: *const GLint) {}
    pub unsafe fn glProgramUniformMatrix3x4fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
    }
    pub unsafe fn glDisablei(&self, _target: EnableCap, _index: GLuint) {}
    pub unsafe fn glUniform1ui(&self, _location: GLint, _v0: GLuint) {}
    pub unsafe fn glCompressedTexImage2D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _internalformat: InternalFormat,
        _width: GLsizei,
        _height: GLsizei,
        _border: GLint,
        _imageSize: GLsizei,
        _data: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glBindTextureUnit(&self, _unit: GLuint, _texture: GLuint) {}
    pub unsafe fn glGetTexLevelParameteriv(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _pname: GetTextureParameter,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glProgramUniform1f(&self, _program: GLuint, _location: GLint, _v0: GLfloat) {}
    pub unsafe fn glGetnTexImage(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _format: PixelFormat,
        _type: PixelType,
        _bufSize: GLsizei,
        _pixels: *mut std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glBeginQueryIndexed(&self, _target: QueryTarget, _index: GLuint, _id: GLuint) {}
    pub unsafe fn glDepthRange(&self, _n: GLdouble, _f: GLdouble) {}
    pub unsafe fn glVertexAttribPointer(
        &self,
        _index: GLuint,
        _size: GLint,
        _type: VertexAttribPointerType,
        _normalized: GLboolean,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glGetTexParameterIiv(
        &self,
        _target: TextureTarget,
        _pname: GetTextureParameter,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glMapBufferRange(
        &self,
        _target: BufferTargetARB,
        _offset: GLintptr,
        _length: GLsizeiptr,
        _access: MapBufferAccessMask,
    ) {
    }
    pub unsafe fn glGenVertexArrays(&self, _n: GLsizei, _arrays: *mut GLuint) {}
    pub unsafe fn glIsTransformFeedback(&self, _id: GLuint) {}
    pub unsafe fn glGetDebugMessageLog(
        &self,
        _count: GLuint,
        _bufSize: GLsizei,
        _sources: *mut DebugSource,
        _types: *mut DebugType,
        _ids: *mut GLuint,
        _severities: *mut DebugSeverity,
        _lengths: *mut GLsizei,
        _messageLog: *mut GLchar,
    ) {
    }
    pub unsafe fn glProgramUniform2iv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLint,
    ) {
    }
    pub unsafe fn glGetTransformFeedbackiv(
        &self,
        _xfb: GLuint,
        _pname: TransformFeedbackPName,
        _param: *mut GLint,
    ) {
    }
    pub unsafe fn glProgramUniform2ui(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLuint,
        _v1: GLuint,
    ) {
    }
    pub unsafe fn glIsTexture(&self, _texture: GLuint) {}
    pub unsafe fn glClearTexImage(
        &self,
        _texture: GLuint,
        _level: GLint,
        _format: PixelFormat,
        _type: PixelType,
        _data: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glGetUniformSubroutineuiv(
        &self,
        _shadertype: ShaderType,
        _location: GLint,
        _params: *mut GLuint,
    ) {
    }
    pub unsafe fn glProgramUniform1ui(&self, _program: GLuint, _location: GLint, _v0: GLuint) {}
    pub unsafe fn glProgramUniformMatrix4dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
    }
    pub unsafe fn glProgramUniformMatrix4x2fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
    }
    pub unsafe fn glGetnMapiv(
        &self,
        _target: MapTarget,
        _query: MapQuery,
        _bufSize: GLsizei,
        _v: *mut GLint,
    ) {
    }
    pub unsafe fn glGetNamedBufferParameteri64v(
        &self,
        _buffer: GLuint,
        _pname: BufferPNameARB,
        _params: *mut GLint64,
    ) {
    }
    pub unsafe fn glBlitNamedFramebuffer(
        &self,
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
    }
    pub unsafe fn glTextureSubImage3D(
        &self,
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
    ) {
    }
    pub unsafe fn glNamedRenderbufferStorageMultisample(
        &self,
        _renderbuffer: GLuint,
        _samples: GLsizei,
        _internalformat: InternalFormat,
        _width: GLsizei,
        _height: GLsizei,
    ) {
    }
    pub unsafe fn glGetTexParameteriv(
        &self,
        _target: TextureTarget,
        _pname: GetTextureParameter,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glGetQueryIndexediv(
        &self,
        _target: QueryTarget,
        _index: GLuint,
        _pname: QueryParameterName,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glCreateProgram(&self) {}
    pub unsafe fn glClipControl(&self, _origin: ClipControlOrigin, _depth: ClipControlDepth) {}
    pub unsafe fn glPatchParameterfv(&self, _pname: PatchParameterName, _values: *const GLfloat) {}
    pub unsafe fn glSpecializeShader(
        &self,
        _shader: GLuint,
        _pEntryPoint: *const GLchar,
        _numSpecializationConstants: GLuint,
        _pConstantIndex: *const GLuint,
        _pConstantValue: *const GLuint,
    ) {
    }
    pub unsafe fn glDeleteSamplers(&self, _count: GLsizei, _samplers: *const GLuint) {}
    pub unsafe fn glNormalP3ui(&self, _type: NormalPointerType, _coords: GLuint) {}
    pub unsafe fn glBeginQuery(&self, _target: QueryTarget, _id: GLuint) {}
    pub unsafe fn glVertexAttribI3ui(&self, _index: GLuint, _x: GLuint, _y: GLuint, _z: GLuint) {}
    pub unsafe fn glGetProgramStageiv(
        &self,
        _program: GLuint,
        _shadertype: ShaderType,
        _pname: ProgramStagePName,
        _values: *mut GLint,
    ) {
    }
    pub unsafe fn glLineWidth(&self, _width: GLfloat) {}
    pub unsafe fn glVertexArrayElementBuffer(&self, _vaobj: GLuint, _buffer: GLuint) {}
    pub unsafe fn glProgramUniform2dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLdouble,
    ) {
    }
    pub unsafe fn glFrontFace(&self, _mode: FrontFaceDirection) {}
    pub unsafe fn glNamedFramebufferDrawBuffer(&self, _framebuffer: GLuint, _buf: ColorBuffer) {}
    pub unsafe fn glVertexAttrib3f(&self, _index: GLuint, _x: GLfloat, _y: GLfloat, _z: GLfloat) {}
    pub unsafe fn glTextureStorage2DMultisample(
        &self,
        _texture: GLuint,
        _samples: GLsizei,
        _internalformat: SizedInternalFormat,
        _width: GLsizei,
        _height: GLsizei,
        _fixedsamplelocations: GLboolean,
    ) {
    }
    pub unsafe fn glGetActiveUniformName(
        &self,
        _program: GLuint,
        _uniformIndex: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _uniformName: *mut GLchar,
    ) {
    }
    pub unsafe fn glGetMultisamplefv(
        &self,
        _pname: GetMultisamplePNameNV,
        _index: GLuint,
        _val: *mut GLfloat,
    ) {
    }
    pub unsafe fn glProgramUniformMatrix3x4dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
    }
    pub unsafe fn glGetProgramResourceName(
        &self,
        _program: GLuint,
        _programInterface: ProgramInterface,
        _index: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _name: *mut GLchar,
    ) {
    }
    pub unsafe fn glNamedFramebufferParameteri(
        &self,
        _framebuffer: GLuint,
        _pname: FramebufferParameterName,
        _param: GLint,
    ) {
    }
    pub unsafe fn glVertexAttribI1iv(&self, _index: GLuint, _v: *const GLint) {}
    pub unsafe fn glGetCompressedTextureImage(
        &self,
        _texture: GLuint,
        _level: GLint,
        _bufSize: GLsizei,
        _pixels: *mut std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glDrawElementsBaseVertex(
        &self,
        _mode: PrimitiveType,
        _count: GLsizei,
        _type: DrawElementsType,
        _indices: *const std::os::raw::c_void,
        _basevertex: GLint,
    ) {
    }
    pub unsafe fn glUniformMatrix2dv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
    }
    pub unsafe fn glGetAttachedShaders(
        &self,
        _program: GLuint,
        _maxCount: GLsizei,
        _count: *mut GLsizei,
        _shaders: *mut GLuint,
    ) {
    }
    pub unsafe fn glTexSubImage2D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _xoffset: GLint,
        _yoffset: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _format: PixelFormat,
        _type: PixelType,
        _pixels: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glGetShaderSource(
        &self,
        _shader: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _source: *mut GLchar,
    ) {
    }
    pub unsafe fn glInvalidateTexImage(&self, _texture: GLuint, _level: GLint) {}
    pub unsafe fn glClearNamedFramebufferiv(
        &self,
        _framebuffer: GLuint,
        _buffer: Buffer,
        _drawbuffer: GLint,
        _value: *const GLint,
    ) {
    }
    pub unsafe fn glTextureSubImage1D(
        &self,
        _texture: GLuint,
        _level: GLint,
        _xoffset: GLint,
        _width: GLsizei,
        _format: PixelFormat,
        _type: PixelType,
        _pixels: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glRenderbufferStorageMultisample(
        &self,
        _target: RenderbufferTarget,
        _samples: GLsizei,
        _internalformat: InternalFormat,
        _width: GLsizei,
        _height: GLsizei,
    ) {
    }
    pub unsafe fn glVertexP4uiv(&self, _type: VertexPointerType, _value: *const GLuint) {}
    pub unsafe fn glProgramUniform4iv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLint,
    ) {
    }
    pub unsafe fn glProgramUniform1d(&self, _program: GLuint, _location: GLint, _v0: GLdouble) {}
    pub unsafe fn glProgramUniform4f(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLfloat,
        _v1: GLfloat,
        _v2: GLfloat,
        _v3: GLfloat,
    ) {
    }
    pub unsafe fn glNamedFramebufferRenderbuffer(
        &self,
        _framebuffer: GLuint,
        _attachment: FramebufferAttachment,
        _renderbuffertarget: RenderbufferTarget,
        _renderbuffer: GLuint,
    ) {
    }
    pub unsafe fn glProgramUniformMatrix4x2dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
    }
    pub unsafe fn glFlushMappedBufferRange(
        &self,
        _target: BufferTargetARB,
        _offset: GLintptr,
        _length: GLsizeiptr,
    ) {
    }
    pub unsafe fn glVertexAttribP2uiv(
        &self,
        _index: GLuint,
        _type: VertexAttribPointerType,
        _normalized: GLboolean,
        _value: *const GLuint,
    ) {
    }
    pub unsafe fn glTexBuffer(
        &self,
        _target: TextureTarget,
        _internalformat: SizedInternalFormat,
        _buffer: GLuint,
    ) {
    }
    pub unsafe fn glIsBuffer(&self, _buffer: GLuint) {}
    pub unsafe fn glCopyBufferSubData(
        &self,
        _readTarget: CopyBufferSubDataTarget,
        _writeTarget: CopyBufferSubDataTarget,
        _readOffset: GLintptr,
        _writeOffset: GLintptr,
        _size: GLsizeiptr,
    ) {
    }
    pub unsafe fn glEndTransformFeedback(&self) {}
    pub unsafe fn glVertexP2ui(&self, _type: VertexPointerType, _value: GLuint) {}
    pub unsafe fn glNamedFramebufferDrawBuffers(
        &self,
        _framebuffer: GLuint,
        _n: GLsizei,
        _bufs: *const ColorBuffer,
    ) {
    }
    pub unsafe fn glUniformMatrix3x4fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
    }
    pub unsafe fn glColorP4uiv(&self, _type: ColorPointerType, _color: *const GLuint) {}
    pub unsafe fn glVertexAttribL1d(&self, _index: GLuint, _x: GLdouble) {}
    pub unsafe fn glSecondaryColorP3ui(&self, _type: ColorPointerType, _color: GLuint) {}
    pub unsafe fn glCopyTexSubImage2D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _xoffset: GLint,
        _yoffset: GLint,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
        _height: GLsizei,
    ) {
    }
    pub unsafe fn glVertexAttrib2sv(&self, _index: GLuint, _v: *const GLshort) {}
    pub unsafe fn glMultiTexCoordP2ui(
        &self,
        _texture: TextureUnit,
        _type: TexCoordPointerType,
        _coords: GLuint,
    ) {
    }
    pub unsafe fn glGetNamedBufferPointerv(
        &self,
        _buffer: GLuint,
        _pname: BufferPointerNameARB,
        _params: *mut *mut std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glReadBuffer(&self, _src: ReadBufferMode) {}
    pub unsafe fn glProgramUniform1fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLfloat,
    ) {
    }
    pub unsafe fn glDrawTransformFeedbackStreamInstanced(
        &self,
        _mode: PrimitiveType,
        _id: GLuint,
        _stream: GLuint,
        _instancecount: GLsizei,
    ) {
    }
    pub unsafe fn glClearBufferData(
        &self,
        _target: BufferStorageTarget,
        _internalformat: SizedInternalFormat,
        _format: PixelFormat,
        _type: PixelType,
        _data: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glBlitFramebuffer(
        &self,
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
    }
    pub unsafe fn glTextureStorage1D(
        &self,
        _texture: GLuint,
        _levels: GLsizei,
        _internalformat: SizedInternalFormat,
        _width: GLsizei,
    ) {
    }
    pub unsafe fn glGetVertexAttribPointerv(
        &self,
        _index: GLuint,
        _pname: VertexAttribPointerPropertyARB,
        _pointer: *mut *mut std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glVertexAttribI2ui(&self, _index: GLuint, _x: GLuint, _y: GLuint) {}
    pub unsafe fn glSecondaryColorP3uiv(&self, _type: ColorPointerType, _color: *const GLuint) {}
    pub unsafe fn glVertexAttrib3d(
        &self,
        _index: GLuint,
        _x: GLdouble,
        _y: GLdouble,
        _z: GLdouble,
    ) {
    }
    pub unsafe fn glGetActiveUniformBlockiv(
        &self,
        _program: GLuint,
        _uniformBlockIndex: GLuint,
        _pname: UniformBlockPName,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glGetInternalformati64v(
        &self,
        _target: TextureTarget,
        _internalformat: InternalFormat,
        _pname: InternalFormatPName,
        _count: GLsizei,
        _params: *mut GLint64,
    ) {
    }
    pub unsafe fn glIsProgram(&self, _program: GLuint) {}
    pub unsafe fn glVertexAttrib2d(&self, _index: GLuint, _x: GLdouble, _y: GLdouble) {}
    pub unsafe fn glProgramUniformMatrix3dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
    }
    pub unsafe fn glVertexArrayVertexBuffers(
        &self,
        _vaobj: GLuint,
        _first: GLuint,
        _count: GLsizei,
        _buffers: *const GLuint,
        _offsets: *const GLintptr,
        _strides: *const GLsizei,
    ) {
    }
    pub unsafe fn glGetSamplerParameterIiv(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterI,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glUniformMatrix3x2dv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
    }
    pub unsafe fn glGetnSeparableFilter(
        &self,
        _target: SeparableTarget,
        _format: PixelFormat,
        _type: PixelType,
        _rowBufSize: GLsizei,
        _row: *mut std::os::raw::c_void,
        _columnBufSize: GLsizei,
        _column: *mut std::os::raw::c_void,
        _span: *mut std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glGetnMinmax(
        &self,
        _target: MinmaxTarget,
        _reset: GLboolean,
        _format: PixelFormat,
        _type: PixelType,
        _bufSize: GLsizei,
        _values: *mut std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glDebugMessageCallback(
        &self,
        _callback: GLDEBUGPROC,
        _userParam: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glGetActiveAttrib(
        &self,
        _program: GLuint,
        _index: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _size: *mut GLint,
        _type: *mut AttributeType,
        _name: *mut GLchar,
    ) {
    }
    pub unsafe fn glDeleteTransformFeedbacks(&self, _n: GLsizei, _ids: *const GLuint) {}
    pub unsafe fn glGenSamplers(&self, _count: GLsizei, _samplers: *mut GLuint) {}
    pub unsafe fn glTexStorage3DMultisample(
        &self,
        _target: TextureTarget,
        _samples: GLsizei,
        _internalformat: SizedInternalFormat,
        _width: GLsizei,
        _height: GLsizei,
        _depth: GLsizei,
        _fixedsamplelocations: GLboolean,
    ) {
    }
    pub unsafe fn glGetTextureParameterfv(
        &self,
        _texture: GLuint,
        _pname: GetTextureParameter,
        _params: *mut GLfloat,
    ) {
    }
    pub unsafe fn glDisableVertexArrayAttrib(&self, _vaobj: GLuint, _index: GLuint) {}
    pub unsafe fn glVertexAttrib4Nusv(&self, _index: GLuint, _v: *const GLushort) {}
    pub unsafe fn glBindFragDataLocationIndexed(
        &self,
        _program: GLuint,
        _colorNumber: GLuint,
        _index: GLuint,
        _name: *const GLchar,
    ) {
    }
    pub unsafe fn glVertexAttribFormat(
        &self,
        _attribindex: GLuint,
        _size: GLint,
        _type: VertexAttribType,
        _normalized: GLboolean,
        _relativeoffset: GLuint,
    ) {
    }
    pub unsafe fn glProgramUniform3fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLfloat,
    ) {
    }
    pub unsafe fn glObjectPtrLabel(
        &self,
        _ptr: *const std::os::raw::c_void,
        _length: GLsizei,
        _label: *const GLchar,
    ) {
    }
    pub unsafe fn glPolygonOffset(&self, _factor: GLfloat, _units: GLfloat) {}
    pub unsafe fn glGenerateMipmap(&self, _target: TextureTarget) {}
    pub unsafe fn glGetVertexAttribIiv(
        &self,
        _index: GLuint,
        _pname: VertexAttribEnum,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glViewportArrayv(&self, _first: GLuint, _count: GLsizei, _v: *const GLfloat) {}
    pub unsafe fn glVertexAttrib4ubv(&self, _index: GLuint, _v: *const GLubyte) {}
    pub unsafe fn glCopyTexSubImage3D(
        &self,
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
    }
    pub unsafe fn glGetAttribLocation(&self, _program: GLuint, _name: *const GLchar) {}
    pub unsafe fn glTexImage1D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _internalformat: GLint,
        _width: GLsizei,
        _border: GLint,
        _format: PixelFormat,
        _type: PixelType,
        _pixels: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glDrawElementsInstancedBaseVertexBaseInstance(
        &self,
        _mode: PrimitiveType,
        _count: GLsizei,
        _type: DrawElementsType,
        _indices: *const std::os::raw::c_void,
        _instancecount: GLsizei,
        _basevertex: GLint,
        _baseinstance: GLuint,
    ) {
    }
    pub unsafe fn glDrawElements(
        &self,
        _mode: PrimitiveType,
        _count: GLsizei,
        _type: DrawElementsType,
        _indices: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glVertexAttribI2uiv(&self, _index: GLuint, _v: *const GLuint) {}
    pub unsafe fn glGetNamedBufferSubData(
        &self,
        _buffer: GLuint,
        _offset: GLintptr,
        _size: GLsizeiptr,
        _data: *mut std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glGetUniformBlockIndex(
        &self,
        _program: GLuint,
        _uniformBlockName: *const GLchar,
    ) {
    }
    pub unsafe fn glCheckFramebufferStatus(&self, _target: FramebufferTarget) {}
    pub unsafe fn glProgramUniformMatrix4x3fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
    }
    pub unsafe fn glGetDoublev(&self, _pname: GetPName, _data: *mut GLdouble) {}
    pub unsafe fn glGenProgramPipelines(&self, _n: GLsizei, _pipelines: *mut GLuint) {}
    pub unsafe fn glUniformMatrix2x4dv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
    }
    pub unsafe fn glGetSamplerParameterfv(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterF,
        _params: *mut GLfloat,
    ) {
    }
    pub unsafe fn glCreateShader(&self, _type: ShaderType) {}
    pub unsafe fn glDeleteShader(&self, _shader: GLuint) {}
    pub unsafe fn glClearDepth(&self, _depth: GLdouble) {}
    pub unsafe fn glEndQuery(&self, _target: QueryTarget) {}
    pub unsafe fn glGetProgramResourceLocationIndex(
        &self,
        _program: GLuint,
        _programInterface: ProgramInterface,
        _name: *const GLchar,
    ) {
    }
    pub unsafe fn glBindBuffersBase(
        &self,
        _target: BufferTargetARB,
        _first: GLuint,
        _count: GLsizei,
        _buffers: *const GLuint,
    ) {
    }
    pub unsafe fn glReadnPixels(
        &self,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _format: PixelFormat,
        _type: PixelType,
        _bufSize: GLsizei,
        _data: *mut std::os::raw::c_void,
    ) {
    }
}
