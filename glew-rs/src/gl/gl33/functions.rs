use crate::gl;
use crate::types::*;
use gl::bitflags::*;
use gl::enums::*;
use std::ffi::c_void;
use std::fmt;
impl GL33 {
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
    pub unsafe fn glBlendEquation(&self, _mode: BlendEquationModeEXT) {}
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
    pub unsafe fn glFramebufferTextureLayer(
        &self,
        _target: FramebufferTarget,
        _attachment: FramebufferAttachment,
        _texture: GLuint,
        _level: GLint,
        _layer: GLint,
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
    pub unsafe fn glValidateProgram(&self, _program: GLuint) {}
    pub unsafe fn glUniform1iv(&self, _location: GLint, _count: GLsizei, _value: *const GLint) {}
    pub unsafe fn glUniformMatrix2fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
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
    pub unsafe fn glDrawArrays(&self, _mode: PrimitiveType, _first: GLint, _count: GLsizei) {}
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
    pub unsafe fn glGetRenderbufferParameteriv(
        &self,
        _target: RenderbufferTarget,
        _pname: RenderbufferParameterName,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glDeleteProgram(&self, _program: GLuint) {}
    pub unsafe fn glProvokingVertex(&self, _mode: VertexProvokingMode) {}
    pub unsafe fn glGetSynciv(
        &self,
        _sync: GLsync,
        _pname: SyncParameterName,
        _count: GLsizei,
        _length: *mut GLsizei,
        _values: *mut GLint,
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
    pub unsafe fn glUniformMatrix2x4fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
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
    pub unsafe fn glGetActiveUniformBlockName(
        &self,
        _program: GLuint,
        _uniformBlockIndex: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _uniformBlockName: *mut GLchar,
    ) {
    }
    pub unsafe fn glVertexAttribI1uiv(&self, _index: GLuint, _v: *const GLuint) {}
    pub unsafe fn glMapBuffer(&self, _target: BufferTargetARB, _access: BufferAccessARB) {}
    pub unsafe fn glVertexAttrib1s(&self, _index: GLuint, _x: GLshort) {}
    pub unsafe fn glColorP3uiv(&self, _type: ColorPointerType, _color: *const GLuint) {}
    pub unsafe fn glVertexAttrib4Nsv(&self, _index: GLuint, _v: *const GLshort) {}
    pub unsafe fn glSamplerParameterIuiv(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterI,
        _param: *const GLuint,
    ) {
    }
    pub unsafe fn glSampleMaski(&self, _maskNumber: GLuint, _mask: GLbitfield) {}
    pub unsafe fn glBlendFuncSeparate(
        &self,
        _sfactorRGB: BlendingFactor,
        _dfactorRGB: BlendingFactor,
        _sfactorAlpha: BlendingFactor,
        _dfactorAlpha: BlendingFactor,
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
    pub unsafe fn glGetSamplerParameteriv(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterI,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glVertexAttrib2f(&self, _index: GLuint, _x: GLfloat, _y: GLfloat) {}
    pub unsafe fn glStencilMaskSeparate(&self, _face: StencilFaceDirection, _mask: GLuint) {}
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
    pub unsafe fn glRenderbufferStorage(
        &self,
        _target: RenderbufferTarget,
        _internalformat: InternalFormat,
        _width: GLsizei,
        _height: GLsizei,
    ) {
    }
    pub unsafe fn glTexParameterIiv(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _params: *const GLint,
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
    pub unsafe fn glTexCoordP3uiv(&self, _type: TexCoordPointerType, _coords: *const GLuint) {}
    pub unsafe fn glUniform2ui(&self, _location: GLint, _v0: GLuint, _v1: GLuint) {}
    pub unsafe fn glSampleCoverage(&self, _value: GLfloat, _invert: GLboolean) {}
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
    pub unsafe fn glUniform1f(&self, _location: GLint, _v0: GLfloat) {}
    pub unsafe fn glGetTexParameterfv(
        &self,
        _target: TextureTarget,
        _pname: GetTextureParameter,
        _params: *mut GLfloat,
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
    pub unsafe fn glGetIntegerv(&self, _pname: GetPName, _data: *mut GLint) {}
    pub unsafe fn glVertexAttrib4sv(&self, _index: GLuint, _v: *const GLshort) {}
    pub unsafe fn glGetSamplerParameterIuiv(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterI,
        _params: *mut GLuint,
    ) {
    }
    pub unsafe fn glVertexAttribI4iv(&self, _index: GLuint, _v: *const GLint) {}
    pub unsafe fn glDrawArraysInstanced(
        &self,
        _mode: PrimitiveType,
        _first: GLint,
        _count: GLsizei,
        _instancecount: GLsizei,
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
    pub unsafe fn glGetBufferParameteriv(
        &self,
        _target: BufferTargetARB,
        _pname: BufferPNameARB,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glFinish(&self) {}
    pub unsafe fn glVertexAttribP4ui(
        &self,
        _index: GLuint,
        _type: VertexAttribPointerType,
        _normalized: GLboolean,
        _value: GLuint,
    ) {
    }
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
    pub unsafe fn glGetProgramiv(
        &self,
        _program: GLuint,
        _pname: ProgramPropertyARB,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glVertexAttribI4uiv(&self, _index: GLuint, _v: *const GLuint) {}
    pub unsafe fn glBeginTransformFeedback(&self, _primitiveMode: PrimitiveType) {}
    pub unsafe fn glUniformMatrix4x3fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
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
    pub unsafe fn glUseProgram(&self, _program: GLuint) {}
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
    pub unsafe fn glVertexP3ui(&self, _type: VertexPointerType, _value: GLuint) {}
    pub unsafe fn glUnmapBuffer(&self, _target: BufferTargetARB) {}
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
    pub unsafe fn glTexCoordP4uiv(&self, _type: TexCoordPointerType, _coords: *const GLuint) {}
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
    pub unsafe fn glCompileShader(&self, _shader: GLuint) {}
    pub unsafe fn glGetString(&self, _name: StringName) {}
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
    pub unsafe fn glVertexAttrib4iv(&self, _index: GLuint, _v: *const GLint) {}
    pub unsafe fn glTexParameterIuiv(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _params: *const GLuint,
    ) {
    }
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
    pub unsafe fn glGetQueryObjecti64v(
        &self,
        _id: GLuint,
        _pname: QueryObjectParameterName,
        _params: *mut GLint64,
    ) {
    }
    pub unsafe fn glClearStencil(&self, _s: GLint) {}
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
    pub unsafe fn glIsQuery(&self, _id: GLuint) {}
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
    pub unsafe fn glUniform3ui(&self, _location: GLint, _v0: GLuint, _v1: GLuint, _v2: GLuint) {}
    pub unsafe fn glVertexAttrib4dv(&self, _index: GLuint, _v: *const GLdouble) {}
    pub unsafe fn glVertexAttrib3sv(&self, _index: GLuint, _v: *const GLshort) {}
    pub unsafe fn glUniformMatrix4x2fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
    }
    pub unsafe fn glIsFramebuffer(&self, _framebuffer: GLuint) {}
    pub unsafe fn glVertexAttrib1sv(&self, _index: GLuint, _v: *const GLshort) {}
    pub unsafe fn glGetBooleani_v(
        &self,
        _target: BufferTargetARB,
        _index: GLuint,
        _data: *mut GLboolean,
    ) {
    }
    pub unsafe fn glDeleteSync(&self, _sync: GLsync) {}
    pub unsafe fn glUniform2fv(&self, _location: GLint, _count: GLsizei, _value: *const GLfloat) {}
    pub unsafe fn glVertexAttribP1ui(
        &self,
        _index: GLuint,
        _type: VertexAttribPointerType,
        _normalized: GLboolean,
        _value: GLuint,
    ) {
    }
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
    pub unsafe fn glUniform2iv(&self, _location: GLint, _count: GLsizei, _value: *const GLint) {}
    pub unsafe fn glGenRenderbuffers(&self, _n: GLsizei, _renderbuffers: *mut GLuint) {}
    pub unsafe fn glVertexAttrib2dv(&self, _index: GLuint, _v: *const GLdouble) {}
    pub unsafe fn glFlush(&self) {}
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
    pub unsafe fn glVertexAttrib4Niv(&self, _index: GLuint, _v: *const GLint) {}
    pub unsafe fn glGetShaderiv(
        &self,
        _shader: GLuint,
        _pname: ShaderParameterName,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glAttachShader(&self, _program: GLuint, _shader: GLuint) {}
    pub unsafe fn glBindTexture(&self, _target: TextureTarget, _texture: GLuint) {}
    pub unsafe fn glDeleteQueries(&self, _n: GLsizei, _ids: *const GLuint) {}
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
    pub unsafe fn glStencilFunc(&self, _func: StencilFunction, _ref: GLint, _mask: GLuint) {}
    pub unsafe fn glSamplerParameterfv(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterF,
        _param: *const GLfloat,
    ) {
    }
    pub unsafe fn glVertexAttribDivisor(&self, _index: GLuint, _divisor: GLuint) {}
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
    pub unsafe fn glVertexAttribI3i(&self, _index: GLuint, _x: GLint, _y: GLint, _z: GLint) {}
    pub unsafe fn glGetInteger64v(&self, _pname: GetPName, _data: *mut GLint64) {}
    pub unsafe fn glDeleteTextures(&self, _n: GLsizei, _textures: *const GLuint) {}
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
    pub unsafe fn glUniform4uiv(&self, _location: GLint, _count: GLsizei, _value: *const GLuint) {}
    pub unsafe fn glTexParameteriv(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _params: *const GLint,
    ) {
    }
    pub unsafe fn glBindRenderbuffer(&self, _target: RenderbufferTarget, _renderbuffer: GLuint) {}
    pub unsafe fn glDrawBuffer(&self, _buf: DrawBufferMode) {}
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
    pub unsafe fn glUniform4fv(&self, _location: GLint, _count: GLsizei, _value: *const GLfloat) {}
    pub unsafe fn glGenQueries(&self, _n: GLsizei, _ids: *mut GLuint) {}
    pub unsafe fn glVertexAttrib1dv(&self, _index: GLuint, _v: *const GLdouble) {}
    pub unsafe fn glStencilOp(&self, _fail: StencilOp, _zfail: StencilOp, _zpass: StencilOp) {}
    pub unsafe fn glEnablei(&self, _target: EnableCap, _index: GLuint) {}
    pub unsafe fn glBindVertexArray(&self, _array: GLuint) {}
    pub unsafe fn glGenFramebuffers(&self, _n: GLsizei, _framebuffers: *mut GLuint) {}
    pub unsafe fn glGetBooleanv(&self, _pname: GetPName, _data: *mut GLboolean) {}
    pub unsafe fn glGetTexImage(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _format: PixelFormat,
        _type: PixelType,
        _pixels: *mut std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glPixelStoref(&self, _pname: PixelStoreParameter, _param: GLfloat) {}
    pub unsafe fn glGetVertexAttribiv(
        &self,
        _index: GLuint,
        _pname: VertexAttribPropertyARB,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glVertexAttribI4usv(&self, _index: GLuint, _v: *const GLushort) {}
    pub unsafe fn glFramebufferRenderbuffer(
        &self,
        _target: FramebufferTarget,
        _attachment: FramebufferAttachment,
        _renderbuffertarget: RenderbufferTarget,
        _renderbuffer: GLuint,
    ) {
    }
    pub unsafe fn glGetFragDataIndex(&self, _program: GLuint, _name: *const GLchar) {}
    pub unsafe fn glVertexAttrib4Nuiv(&self, _index: GLuint, _v: *const GLuint) {}
    pub unsafe fn glNormalP3uiv(&self, _type: NormalPointerType, _coords: *const GLuint) {}
    pub unsafe fn glTransformFeedbackVaryings(
        &self,
        _program: GLuint,
        _count: GLsizei,
        _varyings: *const *const GLchar,
        _bufferMode: TransformFeedbackBufferMode,
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
    pub unsafe fn glUniform1uiv(&self, _location: GLint, _count: GLsizei, _value: *const GLuint) {}
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
    pub unsafe fn glDisable(&self, _cap: EnableCap) {}
    pub unsafe fn glTexParameterfv(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _params: *const GLfloat,
    ) {
    }
    pub unsafe fn glIsSampler(&self, _sampler: GLuint) {}
    pub unsafe fn glQueryCounter(&self, _id: GLuint, _target: QueryCounterTarget) {}
    pub unsafe fn glVertexAttrib4Nubv(&self, _index: GLuint, _v: *const GLubyte) {}
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
    pub unsafe fn glClearColor(
        &self,
        _red: GLfloat,
        _green: GLfloat,
        _blue: GLfloat,
        _alpha: GLfloat,
    ) {
    }
    pub unsafe fn glUniform3iv(&self, _location: GLint, _count: GLsizei, _value: *const GLint) {}
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
    pub unsafe fn glGetTexLevelParameteriv(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _pname: GetTextureParameter,
        _params: *mut GLint,
    ) {
    }
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
    pub unsafe fn glIsTexture(&self, _texture: GLuint) {}
    pub unsafe fn glGetTexParameteriv(
        &self,
        _target: TextureTarget,
        _pname: GetTextureParameter,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glCreateProgram(&self) {}
    pub unsafe fn glDeleteSamplers(&self, _count: GLsizei, _samplers: *const GLuint) {}
    pub unsafe fn glNormalP3ui(&self, _type: NormalPointerType, _coords: GLuint) {}
    pub unsafe fn glBeginQuery(&self, _target: QueryTarget, _id: GLuint) {}
    pub unsafe fn glVertexAttribI3ui(&self, _index: GLuint, _x: GLuint, _y: GLuint, _z: GLuint) {}
    pub unsafe fn glLineWidth(&self, _width: GLfloat) {}
    pub unsafe fn glFrontFace(&self, _mode: FrontFaceDirection) {}
    pub unsafe fn glVertexAttrib3f(&self, _index: GLuint, _x: GLfloat, _y: GLfloat, _z: GLfloat) {}
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
    pub unsafe fn glVertexAttribI1iv(&self, _index: GLuint, _v: *const GLint) {}
    pub unsafe fn glDrawElementsBaseVertex(
        &self,
        _mode: PrimitiveType,
        _count: GLsizei,
        _type: DrawElementsType,
        _indices: *const std::os::raw::c_void,
        _basevertex: GLint,
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
    pub unsafe fn glUniformMatrix3x4fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
    }
    pub unsafe fn glColorP4uiv(&self, _type: ColorPointerType, _color: *const GLuint) {}
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
    pub unsafe fn glReadBuffer(&self, _src: ReadBufferMode) {}
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
    pub unsafe fn glIsProgram(&self, _program: GLuint) {}
    pub unsafe fn glVertexAttrib2d(&self, _index: GLuint, _x: GLdouble, _y: GLdouble) {}
    pub unsafe fn glGetSamplerParameterIiv(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterI,
        _params: *mut GLint,
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
    pub unsafe fn glGenSamplers(&self, _count: GLsizei, _samplers: *mut GLuint) {}
    pub unsafe fn glVertexAttrib4Nusv(&self, _index: GLuint, _v: *const GLushort) {}
    pub unsafe fn glBindFragDataLocationIndexed(
        &self,
        _program: GLuint,
        _colorNumber: GLuint,
        _index: GLuint,
        _name: *const GLchar,
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
    pub unsafe fn glDrawElements(
        &self,
        _mode: PrimitiveType,
        _count: GLsizei,
        _type: DrawElementsType,
        _indices: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glVertexAttribI2uiv(&self, _index: GLuint, _v: *const GLuint) {}
    pub unsafe fn glGetUniformBlockIndex(
        &self,
        _program: GLuint,
        _uniformBlockName: *const GLchar,
    ) {
    }
    pub unsafe fn glCheckFramebufferStatus(&self, _target: FramebufferTarget) {}
    pub unsafe fn glGetDoublev(&self, _pname: GetPName, _data: *mut GLdouble) {}
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
}
