use crate::gl;
use crate::gl::feature::EntryGLFn;
use crate::types::*;
use gl::bitflags::*;
use gl::enums::*;
use std::ffi::c_void;
use std::fmt;
pub trait GL44 {
    unsafe fn entry(&self) -> &EntryGLFn;
    unsafe fn glDepthMask(&self, _flag: GLboolean) {
        (self.entry().glDepthMask)(_flag)
    }
    unsafe fn glDeleteShader(&self, _shader: GLuint) {
        (self.entry().glDeleteShader)(_shader)
    }
    unsafe fn glBindFragDataLocation(
        &self,
        _program: GLuint,
        _color: GLuint,
        _name: *const GLchar,
    ) {
        (self.entry().glBindFragDataLocation)(_program, _color, _name)
    }
    unsafe fn glVertexAttrib4bv(&self, _index: GLuint, _v: *const GLbyte) {
        (self.entry().glVertexAttrib4bv)(_index, _v)
    }
    unsafe fn glBlendEquationSeparate(
        &self,
        _modeRGB: BlendEquationModeEXT,
        _modeAlpha: BlendEquationModeEXT,
    ) {
        (self.entry().glBlendEquationSeparate)(_modeRGB, _modeAlpha)
    }
    unsafe fn glGetActiveSubroutineName(
        &self,
        _program: GLuint,
        _shadertype: ShaderType,
        _index: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _name: *mut GLchar,
    ) {
        (self.entry().glGetActiveSubroutineName)(
            _program,
            _shadertype,
            _index,
            _bufSize,
            _length,
            _name,
        )
    }
    unsafe fn glVertexAttribI4uiv(&self, _index: GLuint, _v: *const GLuint) {
        (self.entry().glVertexAttribI4uiv)(_index, _v)
    }
    unsafe fn glUniform1ui(&self, _location: GLint, _v0: GLuint) {
        (self.entry().glUniform1ui)(_location, _v0)
    }
    unsafe fn glNormalP3ui(&self, _type: NormalPointerType, _coords: GLuint) {
        (self.entry().glNormalP3ui)(_type, _coords)
    }
    unsafe fn glVertexAttribPointer(
        &self,
        _index: GLuint,
        _size: GLint,
        _type: VertexAttribPointerType,
        _normalized: GLboolean,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glVertexAttribPointer)(_index, _size, _type, _normalized, _stride, _pointer)
    }
    unsafe fn glVertexAttribLPointer(
        &self,
        _index: GLuint,
        _size: GLint,
        _type: VertexAttribLType,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glVertexAttribLPointer)(_index, _size, _type, _stride, _pointer)
    }
    unsafe fn glVertexAttrib1f(&self, _index: GLuint, _x: GLfloat) {
        (self.entry().glVertexAttrib1f)(_index, _x)
    }
    unsafe fn glVertexAttribI3iv(&self, _index: GLuint, _v: *const GLint) {
        (self.entry().glVertexAttribI3iv)(_index, _v)
    }
    unsafe fn glGetSamplerParameterIiv(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterI,
        _params: *mut GLint,
    ) {
        (self.entry().glGetSamplerParameterIiv)(_sampler, _pname, _params)
    }
    unsafe fn glSamplerParameterIiv(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterI,
        _param: *const GLint,
    ) {
        (self.entry().glSamplerParameterIiv)(_sampler, _pname, _param)
    }
    unsafe fn glUniform1i(&self, _location: GLint, _v0: GLint) {
        (self.entry().glUniform1i)(_location, _v0)
    }
    unsafe fn glBlendEquationSeparatei(
        &self,
        _buf: GLuint,
        _modeRGB: BlendEquationModeEXT,
        _modeAlpha: BlendEquationModeEXT,
    ) {
        (self.entry().glBlendEquationSeparatei)(_buf, _modeRGB, _modeAlpha)
    }
    unsafe fn glUniform4d(
        &self,
        _location: GLint,
        _x: GLdouble,
        _y: GLdouble,
        _z: GLdouble,
        _w: GLdouble,
    ) {
        (self.entry().glUniform4d)(_location, _x, _y, _z, _w)
    }
    unsafe fn glUniform1d(&self, _location: GLint, _x: GLdouble) {
        (self.entry().glUniform1d)(_location, _x)
    }
    unsafe fn glShaderBinary(
        &self,
        _count: GLsizei,
        _shaders: *const GLuint,
        _binaryFormat: ShaderBinaryFormat,
        _binary: *const std::os::raw::c_void,
        _length: GLsizei,
    ) {
        (self.entry().glShaderBinary)(_count, _shaders, _binaryFormat, _binary, _length)
    }
    unsafe fn glGetVertexAttribLdv(
        &self,
        _index: GLuint,
        _pname: VertexAttribEnum,
        _params: *mut GLdouble,
    ) {
        (self.entry().glGetVertexAttribLdv)(_index, _pname, _params)
    }
    unsafe fn glTexSubImage2D(
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
        (self.entry().glTexSubImage2D)(
            _target, _level, _xoffset, _yoffset, _width, _height, _format, _type, _pixels,
        )
    }
    unsafe fn glUniformSubroutinesuiv(
        &self,
        _shadertype: ShaderType,
        _count: GLsizei,
        _indices: *const GLuint,
    ) {
        (self.entry().glUniformSubroutinesuiv)(_shadertype, _count, _indices)
    }
    unsafe fn glIsEnabledi(&self, _target: EnableCap, _index: GLuint) -> GLboolean {
        (self.entry().glIsEnabledi)(_target, _index)
    }
    unsafe fn glDrawArraysInstanced(
        &self,
        _mode: PrimitiveType,
        _first: GLint,
        _count: GLsizei,
        _instancecount: GLsizei,
    ) {
        (self.entry().glDrawArraysInstanced)(_mode, _first, _count, _instancecount)
    }
    unsafe fn glMultiTexCoordP4ui(
        &self,
        _texture: TextureUnit,
        _type: TexCoordPointerType,
        _coords: GLuint,
    ) {
        (self.entry().glMultiTexCoordP4ui)(_texture, _type, _coords)
    }
    unsafe fn glGetObjectPtrLabel(
        &self,
        _ptr: *const std::os::raw::c_void,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _label: *mut GLchar,
    ) {
        (self.entry().glGetObjectPtrLabel)(_ptr, _bufSize, _length, _label)
    }
    unsafe fn glBlitFramebuffer(
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
        (self.entry().glBlitFramebuffer)(
            _srcX0, _srcY0, _srcX1, _srcY1, _dstX0, _dstY0, _dstX1, _dstY1, _mask, _filter,
        )
    }
    unsafe fn glGetQueryiv(
        &self,
        _target: QueryTarget,
        _pname: QueryParameterName,
        _params: *mut GLint,
    ) {
        (self.entry().glGetQueryiv)(_target, _pname, _params)
    }
    unsafe fn glGetSynciv(
        &self,
        _sync: GLsync,
        _pname: SyncParameterName,
        _count: GLsizei,
        _length: *mut GLsizei,
        _values: *mut GLint,
    ) {
        (self.entry().glGetSynciv)(_sync, _pname, _count, _length, _values)
    }
    unsafe fn glViewportIndexedf(
        &self,
        _index: GLuint,
        _x: GLfloat,
        _y: GLfloat,
        _w: GLfloat,
        _h: GLfloat,
    ) {
        (self.entry().glViewportIndexedf)(_index, _x, _y, _w, _h)
    }
    unsafe fn glVertexAttrib4Niv(&self, _index: GLuint, _v: *const GLint) {
        (self.entry().glVertexAttrib4Niv)(_index, _v)
    }
    unsafe fn glGetFloatv(&self, _pname: GetPName, _data: *mut GLfloat) {
        (self.entry().glGetFloatv)(_pname, _data)
    }
    unsafe fn glProgramUniformMatrix3dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        (self.entry().glProgramUniformMatrix3dv)(_program, _location, _count, _transpose, _value)
    }
    unsafe fn glGetBooleanv(&self, _pname: GetPName, _data: *mut GLboolean) {
        (self.entry().glGetBooleanv)(_pname, _data)
    }
    unsafe fn glGetInternalformati64v(
        &self,
        _target: TextureTarget,
        _internalformat: InternalFormat,
        _pname: InternalFormatPName,
        _count: GLsizei,
        _params: *mut GLint64,
    ) {
        (self.entry().glGetInternalformati64v)(_target, _internalformat, _pname, _count, _params)
    }
    unsafe fn glVertexAttribI4ubv(&self, _index: GLuint, _v: *const GLubyte) {
        (self.entry().glVertexAttribI4ubv)(_index, _v)
    }
    unsafe fn glGetAttribLocation(&self, _program: GLuint, _name: *const GLchar) -> GLint {
        (self.entry().glGetAttribLocation)(_program, _name)
    }
    unsafe fn glVertexAttrib4Nsv(&self, _index: GLuint, _v: *const GLshort) {
        (self.entry().glVertexAttrib4Nsv)(_index, _v)
    }
    unsafe fn glVertexAttribI4sv(&self, _index: GLuint, _v: *const GLshort) {
        (self.entry().glVertexAttribI4sv)(_index, _v)
    }
    unsafe fn glUniform1f(&self, _location: GLint, _v0: GLfloat) {
        (self.entry().glUniform1f)(_location, _v0)
    }
    unsafe fn glPushDebugGroup(
        &self,
        _source: DebugSource,
        _id: GLuint,
        _length: GLsizei,
        _message: *const GLchar,
    ) {
        (self.entry().glPushDebugGroup)(_source, _id, _length, _message)
    }
    unsafe fn glDrawArrays(&self, _mode: PrimitiveType, _first: GLint, _count: GLsizei) {
        (self.entry().glDrawArrays)(_mode, _first, _count)
    }
    unsafe fn glFramebufferRenderbuffer(
        &self,
        _target: FramebufferTarget,
        _attachment: FramebufferAttachment,
        _renderbuffertarget: RenderbufferTarget,
        _renderbuffer: GLuint,
    ) {
        (self.entry().glFramebufferRenderbuffer)(
            _target,
            _attachment,
            _renderbuffertarget,
            _renderbuffer,
        )
    }
    unsafe fn glMapBufferRange(
        &self,
        _target: BufferTargetARB,
        _offset: GLintptr,
        _length: GLsizeiptr,
        _access: MapBufferAccessMask,
    ) -> *mut std::os::raw::c_void {
        (self.entry().glMapBufferRange)(_target, _offset, _length, _access)
    }
    unsafe fn glUseProgram(&self, _program: GLuint) {
        (self.entry().glUseProgram)(_program)
    }
    unsafe fn glGetUniformBlockIndex(
        &self,
        _program: GLuint,
        _uniformBlockName: *const GLchar,
    ) -> GLuint {
        (self.entry().glGetUniformBlockIndex)(_program, _uniformBlockName)
    }
    unsafe fn glProvokingVertex(&self, _mode: VertexProvokingMode) {
        (self.entry().glProvokingVertex)(_mode)
    }
    unsafe fn glBlendFuncSeparate(
        &self,
        _sfactorRGB: BlendingFactor,
        _dfactorRGB: BlendingFactor,
        _sfactorAlpha: BlendingFactor,
        _dfactorAlpha: BlendingFactor,
    ) {
        (self.entry().glBlendFuncSeparate)(_sfactorRGB, _dfactorRGB, _sfactorAlpha, _dfactorAlpha)
    }
    unsafe fn glGetBooleani_v(
        &self,
        _target: BufferTargetARB,
        _index: GLuint,
        _data: *mut GLboolean,
    ) {
        (self.entry().glGetBooleani_v)(_target, _index, _data)
    }
    unsafe fn glGetQueryObjectui64v(
        &self,
        _id: GLuint,
        _pname: QueryObjectParameterName,
        _params: *mut GLuint64,
    ) {
        (self.entry().glGetQueryObjectui64v)(_id, _pname, _params)
    }
    unsafe fn glMultiTexCoordP1uiv(
        &self,
        _texture: TextureUnit,
        _type: TexCoordPointerType,
        _coords: *const GLuint,
    ) {
        (self.entry().glMultiTexCoordP1uiv)(_texture, _type, _coords)
    }
    unsafe fn glRenderbufferStorage(
        &self,
        _target: RenderbufferTarget,
        _internalformat: InternalFormat,
        _width: GLsizei,
        _height: GLsizei,
    ) {
        (self.entry().glRenderbufferStorage)(_target, _internalformat, _width, _height)
    }
    unsafe fn glVertexP3ui(&self, _type: VertexPointerType, _value: GLuint) {
        (self.entry().glVertexP3ui)(_type, _value)
    }
    unsafe fn glVertexAttrib4iv(&self, _index: GLuint, _v: *const GLint) {
        (self.entry().glVertexAttrib4iv)(_index, _v)
    }
    unsafe fn glProgramUniform2f(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLfloat,
        _v1: GLfloat,
    ) {
        (self.entry().glProgramUniform2f)(_program, _location, _v0, _v1)
    }
    unsafe fn glDeleteProgramPipelines(&self, _n: GLsizei, _pipelines: *const GLuint) {
        (self.entry().glDeleteProgramPipelines)(_n, _pipelines)
    }
    unsafe fn glProgramBinary(
        &self,
        _program: GLuint,
        _binaryFormat: GLenum,
        _binary: *const std::os::raw::c_void,
        _length: GLsizei,
    ) {
        (self.entry().glProgramBinary)(_program, _binaryFormat, _binary, _length)
    }
    unsafe fn glProgramUniformMatrix2x4dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        (self.entry().glProgramUniformMatrix2x4dv)(_program, _location, _count, _transpose, _value)
    }
    unsafe fn glInvalidateSubFramebuffer(
        &self,
        _target: FramebufferTarget,
        _numAttachments: GLsizei,
        _attachments: *const InvalidateFramebufferAttachment,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
        _height: GLsizei,
    ) {
        (self.entry().glInvalidateSubFramebuffer)(
            _target,
            _numAttachments,
            _attachments,
            _x,
            _y,
            _width,
            _height,
        )
    }
    unsafe fn glMultiDrawElements(
        &self,
        _mode: PrimitiveType,
        _count: *const GLsizei,
        _type: DrawElementsType,
        _indices: *const *const std::os::raw::c_void,
        _drawcount: GLsizei,
    ) {
        (self.entry().glMultiDrawElements)(_mode, _count, _type, _indices, _drawcount)
    }
    unsafe fn glUniformMatrix3fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        (self.entry().glUniformMatrix3fv)(_location, _count, _transpose, _value)
    }
    unsafe fn glGetActiveUniformBlockName(
        &self,
        _program: GLuint,
        _uniformBlockIndex: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _uniformBlockName: *mut GLchar,
    ) {
        (self.entry().glGetActiveUniformBlockName)(
            _program,
            _uniformBlockIndex,
            _bufSize,
            _length,
            _uniformBlockName,
        )
    }
    unsafe fn glMultiTexCoordP3uiv(
        &self,
        _texture: TextureUnit,
        _type: TexCoordPointerType,
        _coords: *const GLuint,
    ) {
        (self.entry().glMultiTexCoordP3uiv)(_texture, _type, _coords)
    }
    unsafe fn glPointParameteri(&self, _pname: PointParameterNameARB, _param: GLint) {
        (self.entry().glPointParameteri)(_pname, _param)
    }
    unsafe fn glVertexAttribI3ui(&self, _index: GLuint, _x: GLuint, _y: GLuint, _z: GLuint) {
        (self.entry().glVertexAttribI3ui)(_index, _x, _y, _z)
    }
    unsafe fn glIsRenderbuffer(&self, _renderbuffer: GLuint) -> GLboolean {
        (self.entry().glIsRenderbuffer)(_renderbuffer)
    }
    unsafe fn glColorP4ui(&self, _type: ColorPointerType, _color: GLuint) {
        (self.entry().glColorP4ui)(_type, _color)
    }
    unsafe fn glProgramUniformMatrix2x4fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        (self.entry().glProgramUniformMatrix2x4fv)(_program, _location, _count, _transpose, _value)
    }
    unsafe fn glVertexAttrib3sv(&self, _index: GLuint, _v: *const GLshort) {
        (self.entry().glVertexAttrib3sv)(_index, _v)
    }
    unsafe fn glProgramUniform4i(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLint,
        _v1: GLint,
        _v2: GLint,
        _v3: GLint,
    ) {
        (self.entry().glProgramUniform4i)(_program, _location, _v0, _v1, _v2, _v3)
    }
    unsafe fn glDrawTransformFeedbackInstanced(
        &self,
        _mode: PrimitiveType,
        _id: GLuint,
        _instancecount: GLsizei,
    ) {
        (self.entry().glDrawTransformFeedbackInstanced)(_mode, _id, _instancecount)
    }
    unsafe fn glVertexAttrib4fv(&self, _index: GLuint, _v: *const GLfloat) {
        (self.entry().glVertexAttrib4fv)(_index, _v)
    }
    unsafe fn glDepthRange(&self, _n: GLdouble, _f: GLdouble) {
        (self.entry().glDepthRange)(_n, _f)
    }
    unsafe fn glBlendEquationi(&self, _buf: GLuint, _mode: BlendEquationModeEXT) {
        (self.entry().glBlendEquationi)(_buf, _mode)
    }
    unsafe fn glTexImage3DMultisample(
        &self,
        _target: TextureTarget,
        _samples: GLsizei,
        _internalformat: InternalFormat,
        _width: GLsizei,
        _height: GLsizei,
        _depth: GLsizei,
        _fixedsamplelocations: GLboolean,
    ) {
        (self.entry().glTexImage3DMultisample)(
            _target,
            _samples,
            _internalformat,
            _width,
            _height,
            _depth,
            _fixedsamplelocations,
        )
    }
    unsafe fn glProgramUniform2ui(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLuint,
        _v1: GLuint,
    ) {
        (self.entry().glProgramUniform2ui)(_program, _location, _v0, _v1)
    }
    unsafe fn glLineWidth(&self, _width: GLfloat) {
        (self.entry().glLineWidth)(_width)
    }
    unsafe fn glVertexAttribL4dv(&self, _index: GLuint, _v: *const GLdouble) {
        (self.entry().glVertexAttribL4dv)(_index, _v)
    }
    unsafe fn glVertexAttrib3dv(&self, _index: GLuint, _v: *const GLdouble) {
        (self.entry().glVertexAttrib3dv)(_index, _v)
    }
    unsafe fn glGetShaderiv(
        &self,
        _shader: GLuint,
        _pname: ShaderParameterName,
        _params: *mut GLint,
    ) {
        (self.entry().glGetShaderiv)(_shader, _pname, _params)
    }
    unsafe fn glGetInteger64v(&self, _pname: GetPName, _data: *mut GLint64) {
        (self.entry().glGetInteger64v)(_pname, _data)
    }
    unsafe fn glTexSubImage1D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _xoffset: GLint,
        _width: GLsizei,
        _format: PixelFormat,
        _type: PixelType,
        _pixels: *const std::os::raw::c_void,
    ) {
        (self.entry().glTexSubImage1D)(_target, _level, _xoffset, _width, _format, _type, _pixels)
    }
    unsafe fn glVertexAttrib3d(&self, _index: GLuint, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glVertexAttrib3d)(_index, _x, _y, _z)
    }
    unsafe fn glDrawElementsInstancedBaseVertex(
        &self,
        _mode: PrimitiveType,
        _count: GLsizei,
        _type: DrawElementsType,
        _indices: *const std::os::raw::c_void,
        _instancecount: GLsizei,
        _basevertex: GLint,
    ) {
        (self.entry().glDrawElementsInstancedBaseVertex)(
            _mode,
            _count,
            _type,
            _indices,
            _instancecount,
            _basevertex,
        )
    }
    unsafe fn glUniformMatrix3x2dv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        (self.entry().glUniformMatrix3x2dv)(_location, _count, _transpose, _value)
    }
    unsafe fn glViewport(&self, _x: GLint, _y: GLint, _width: GLsizei, _height: GLsizei) {
        (self.entry().glViewport)(_x, _y, _width, _height)
    }
    unsafe fn glProgramUniform1d(&self, _program: GLuint, _location: GLint, _v0: GLdouble) {
        (self.entry().glProgramUniform1d)(_program, _location, _v0)
    }
    unsafe fn glProgramUniform4fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLfloat,
    ) {
        (self.entry().glProgramUniform4fv)(_program, _location, _count, _value)
    }
    unsafe fn glProgramUniform4uiv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLuint,
    ) {
        (self.entry().glProgramUniform4uiv)(_program, _location, _count, _value)
    }
    unsafe fn glBindVertexBuffer(
        &self,
        _bindingindex: GLuint,
        _buffer: GLuint,
        _offset: GLintptr,
        _stride: GLsizei,
    ) {
        (self.entry().glBindVertexBuffer)(_bindingindex, _buffer, _offset, _stride)
    }
    unsafe fn glVertexAttrib1sv(&self, _index: GLuint, _v: *const GLshort) {
        (self.entry().glVertexAttrib1sv)(_index, _v)
    }
    unsafe fn glGetVertexAttribdv(
        &self,
        _index: GLuint,
        _pname: VertexAttribPropertyARB,
        _params: *mut GLdouble,
    ) {
        (self.entry().glGetVertexAttribdv)(_index, _pname, _params)
    }
    unsafe fn glCreateShaderProgramv(
        &self,
        _type: ShaderType,
        _count: GLsizei,
        _strings: *const *const GLchar,
    ) -> GLuint {
        (self.entry().glCreateShaderProgramv)(_type, _count, _strings)
    }
    unsafe fn glProgramUniform3f(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLfloat,
        _v1: GLfloat,
        _v2: GLfloat,
    ) {
        (self.entry().glProgramUniform3f)(_program, _location, _v0, _v1, _v2)
    }
    unsafe fn glUniform2f(&self, _location: GLint, _v0: GLfloat, _v1: GLfloat) {
        (self.entry().glUniform2f)(_location, _v0, _v1)
    }
    unsafe fn glMultiTexCoordP4uiv(
        &self,
        _texture: TextureUnit,
        _type: TexCoordPointerType,
        _coords: *const GLuint,
    ) {
        (self.entry().glMultiTexCoordP4uiv)(_texture, _type, _coords)
    }
    unsafe fn glUniformMatrix2x4fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        (self.entry().glUniformMatrix2x4fv)(_location, _count, _transpose, _value)
    }
    unsafe fn glDetachShader(&self, _program: GLuint, _shader: GLuint) {
        (self.entry().glDetachShader)(_program, _shader)
    }
    unsafe fn glTexParameterIiv(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _params: *const GLint,
    ) {
        (self.entry().glTexParameterIiv)(_target, _pname, _params)
    }
    unsafe fn glFramebufferTexture2D(
        &self,
        _target: FramebufferTarget,
        _attachment: FramebufferAttachment,
        _textarget: TextureTarget,
        _texture: GLuint,
        _level: GLint,
    ) {
        (self.entry().glFramebufferTexture2D)(_target, _attachment, _textarget, _texture, _level)
    }
    unsafe fn glColorP3ui(&self, _type: ColorPointerType, _color: GLuint) {
        (self.entry().glColorP3ui)(_type, _color)
    }
    unsafe fn glIsQuery(&self, _id: GLuint) -> GLboolean {
        (self.entry().glIsQuery)(_id)
    }
    unsafe fn glMultiTexCoordP3ui(
        &self,
        _texture: TextureUnit,
        _type: TexCoordPointerType,
        _coords: GLuint,
    ) {
        (self.entry().glMultiTexCoordP3ui)(_texture, _type, _coords)
    }
    unsafe fn glTexCoordP1ui(&self, _type: TexCoordPointerType, _coords: GLuint) {
        (self.entry().glTexCoordP1ui)(_type, _coords)
    }
    unsafe fn glPixelStoref(&self, _pname: PixelStoreParameter, _param: GLfloat) {
        (self.entry().glPixelStoref)(_pname, _param)
    }
    unsafe fn glCopyTexImage1D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _internalformat: InternalFormat,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
        _border: GLint,
    ) {
        (self.entry().glCopyTexImage1D)(_target, _level, _internalformat, _x, _y, _width, _border)
    }
    unsafe fn glProgramParameteri(
        &self,
        _program: GLuint,
        _pname: ProgramParameterPName,
        _value: GLint,
    ) {
        (self.entry().glProgramParameteri)(_program, _pname, _value)
    }
    unsafe fn glDisableVertexAttribArray(&self, _index: GLuint) {
        (self.entry().glDisableVertexAttribArray)(_index)
    }
    unsafe fn glGetActiveAttrib(
        &self,
        _program: GLuint,
        _index: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _size: *mut GLint,
        _type: *mut AttributeType,
        _name: *mut GLchar,
    ) {
        (self.entry().glGetActiveAttrib)(_program, _index, _bufSize, _length, _size, _type, _name)
    }
    unsafe fn glBindBufferBase(&self, _target: BufferTargetARB, _index: GLuint, _buffer: GLuint) {
        (self.entry().glBindBufferBase)(_target, _index, _buffer)
    }
    unsafe fn glDeleteVertexArrays(&self, _n: GLsizei, _arrays: *const GLuint) {
        (self.entry().glDeleteVertexArrays)(_n, _arrays)
    }
    unsafe fn glGetInteger64i_v(&self, _target: GetPName, _index: GLuint, _data: *mut GLint64) {
        (self.entry().glGetInteger64i_v)(_target, _index, _data)
    }
    unsafe fn glUniformMatrix4fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        (self.entry().glUniformMatrix4fv)(_location, _count, _transpose, _value)
    }
    unsafe fn glVertexAttribL3d(&self, _index: GLuint, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glVertexAttribL3d)(_index, _x, _y, _z)
    }
    unsafe fn glDepthFunc(&self, _func: DepthFunction) {
        (self.entry().glDepthFunc)(_func)
    }
    unsafe fn glProgramUniform2uiv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLuint,
    ) {
        (self.entry().glProgramUniform2uiv)(_program, _location, _count, _value)
    }
    unsafe fn glVertexBindingDivisor(&self, _bindingindex: GLuint, _divisor: GLuint) {
        (self.entry().glVertexBindingDivisor)(_bindingindex, _divisor)
    }
    unsafe fn glClearStencil(&self, _s: GLint) {
        (self.entry().glClearStencil)(_s)
    }
    unsafe fn glIsFramebuffer(&self, _framebuffer: GLuint) -> GLboolean {
        (self.entry().glIsFramebuffer)(_framebuffer)
    }
    unsafe fn glGetQueryObjectiv(
        &self,
        _id: GLuint,
        _pname: QueryObjectParameterName,
        _params: *mut GLint,
    ) {
        (self.entry().glGetQueryObjectiv)(_id, _pname, _params)
    }
    unsafe fn glVertexAttribI1i(&self, _index: GLuint, _x: GLint) {
        (self.entry().glVertexAttribI1i)(_index, _x)
    }
    unsafe fn glDeleteTransformFeedbacks(&self, _n: GLsizei, _ids: *const GLuint) {
        (self.entry().glDeleteTransformFeedbacks)(_n, _ids)
    }
    unsafe fn glBindImageTextures(
        &self,
        _first: GLuint,
        _count: GLsizei,
        _textures: *const GLuint,
    ) {
        (self.entry().glBindImageTextures)(_first, _count, _textures)
    }
    unsafe fn glBindTransformFeedback(&self, _target: BindTransformFeedbackTarget, _id: GLuint) {
        (self.entry().glBindTransformFeedback)(_target, _id)
    }
    unsafe fn glCreateShader(&self, _type: ShaderType) -> GLuint {
        (self.entry().glCreateShader)(_type)
    }
    unsafe fn glGetBufferParameteri64v(
        &self,
        _target: BufferTargetARB,
        _pname: BufferPNameARB,
        _params: *mut GLint64,
    ) {
        (self.entry().glGetBufferParameteri64v)(_target, _pname, _params)
    }
    unsafe fn glMinSampleShading(&self, _value: GLfloat) {
        (self.entry().glMinSampleShading)(_value)
    }
    unsafe fn glUniform3dv(&self, _location: GLint, _count: GLsizei, _value: *const GLdouble) {
        (self.entry().glUniform3dv)(_location, _count, _value)
    }
    unsafe fn glTexParameterf(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _param: GLfloat,
    ) {
        (self.entry().glTexParameterf)(_target, _pname, _param)
    }
    unsafe fn glUniformMatrix4dv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        (self.entry().glUniformMatrix4dv)(_location, _count, _transpose, _value)
    }
    unsafe fn glProgramUniform1fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLfloat,
    ) {
        (self.entry().glProgramUniform1fv)(_program, _location, _count, _value)
    }
    unsafe fn glTexStorage2DMultisample(
        &self,
        _target: TextureTarget,
        _samples: GLsizei,
        _internalformat: SizedInternalFormat,
        _width: GLsizei,
        _height: GLsizei,
        _fixedsamplelocations: GLboolean,
    ) {
        (self.entry().glTexStorage2DMultisample)(
            _target,
            _samples,
            _internalformat,
            _width,
            _height,
            _fixedsamplelocations,
        )
    }
    unsafe fn glVertexAttribLFormat(
        &self,
        _attribindex: GLuint,
        _size: GLint,
        _type: VertexAttribLType,
        _relativeoffset: GLuint,
    ) {
        (self.entry().glVertexAttribLFormat)(_attribindex, _size, _type, _relativeoffset)
    }
    unsafe fn glClearTexSubImage(
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
        (self.entry().glClearTexSubImage)(
            _texture, _level, _xoffset, _yoffset, _zoffset, _width, _height, _depth, _format,
            _type, _data,
        )
    }
    unsafe fn glProgramUniform2i(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLint,
        _v1: GLint,
    ) {
        (self.entry().glProgramUniform2i)(_program, _location, _v0, _v1)
    }
    unsafe fn glGetUniformuiv(&self, _program: GLuint, _location: GLint, _params: *mut GLuint) {
        (self.entry().glGetUniformuiv)(_program, _location, _params)
    }
    unsafe fn glStencilMaskSeparate(&self, _face: StencilFaceDirection, _mask: GLuint) {
        (self.entry().glStencilMaskSeparate)(_face, _mask)
    }
    unsafe fn glUniformMatrix2x3dv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        (self.entry().glUniformMatrix2x3dv)(_location, _count, _transpose, _value)
    }
    unsafe fn glTexStorage1D(
        &self,
        _target: TextureTarget,
        _levels: GLsizei,
        _internalformat: SizedInternalFormat,
        _width: GLsizei,
    ) {
        (self.entry().glTexStorage1D)(_target, _levels, _internalformat, _width)
    }
    unsafe fn glTexCoordP4ui(&self, _type: TexCoordPointerType, _coords: GLuint) {
        (self.entry().glTexCoordP4ui)(_type, _coords)
    }
    unsafe fn glIsVertexArray(&self, _array: GLuint) -> GLboolean {
        (self.entry().glIsVertexArray)(_array)
    }
    unsafe fn glClearBufferData(
        &self,
        _target: BufferStorageTarget,
        _internalformat: SizedInternalFormat,
        _format: PixelFormat,
        _type: PixelType,
        _data: *const std::os::raw::c_void,
    ) {
        (self.entry().glClearBufferData)(_target, _internalformat, _format, _type, _data)
    }
    unsafe fn glProgramUniformMatrix3x2dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        (self.entry().glProgramUniformMatrix3x2dv)(_program, _location, _count, _transpose, _value)
    }
    unsafe fn glIsSync(&self, _sync: GLsync) -> GLboolean {
        (self.entry().glIsSync)(_sync)
    }
    unsafe fn glVertexAttrib4f(
        &self,
        _index: GLuint,
        _x: GLfloat,
        _y: GLfloat,
        _z: GLfloat,
        _w: GLfloat,
    ) {
        (self.entry().glVertexAttrib4f)(_index, _x, _y, _z, _w)
    }
    unsafe fn glDrawElementsInstancedBaseVertexBaseInstance(
        &self,
        _mode: PrimitiveType,
        _count: GLsizei,
        _type: DrawElementsType,
        _indices: *const std::os::raw::c_void,
        _instancecount: GLsizei,
        _basevertex: GLint,
        _baseinstance: GLuint,
    ) {
        (self.entry().glDrawElementsInstancedBaseVertexBaseInstance)(
            _mode,
            _count,
            _type,
            _indices,
            _instancecount,
            _basevertex,
            _baseinstance,
        )
    }
    unsafe fn glVertexAttrib2sv(&self, _index: GLuint, _v: *const GLshort) {
        (self.entry().glVertexAttrib2sv)(_index, _v)
    }
    unsafe fn glUniform3f(&self, _location: GLint, _v0: GLfloat, _v1: GLfloat, _v2: GLfloat) {
        (self.entry().glUniform3f)(_location, _v0, _v1, _v2)
    }
    unsafe fn glGetTexParameteriv(
        &self,
        _target: TextureTarget,
        _pname: GetTextureParameter,
        _params: *mut GLint,
    ) {
        (self.entry().glGetTexParameteriv)(_target, _pname, _params)
    }
    unsafe fn glGetUniformIndices(
        &self,
        _program: GLuint,
        _uniformCount: GLsizei,
        _uniformNames: *const *const GLchar,
        _uniformIndices: *mut GLuint,
    ) {
        (self.entry().glGetUniformIndices)(_program, _uniformCount, _uniformNames, _uniformIndices)
    }
    unsafe fn glVertexAttribI1ui(&self, _index: GLuint, _x: GLuint) {
        (self.entry().glVertexAttribI1ui)(_index, _x)
    }
    unsafe fn glVertexAttribP2ui(
        &self,
        _index: GLuint,
        _type: VertexAttribPointerType,
        _normalized: GLboolean,
        _value: GLuint,
    ) {
        (self.entry().glVertexAttribP2ui)(_index, _type, _normalized, _value)
    }
    unsafe fn glGetShaderInfoLog(
        &self,
        _shader: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _infoLog: *mut GLchar,
    ) {
        (self.entry().glGetShaderInfoLog)(_shader, _bufSize, _length, _infoLog)
    }
    unsafe fn glUniformMatrix3dv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        (self.entry().glUniformMatrix3dv)(_location, _count, _transpose, _value)
    }
    unsafe fn glGetTexImage(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _format: PixelFormat,
        _type: PixelType,
        _pixels: *mut std::os::raw::c_void,
    ) {
        (self.entry().glGetTexImage)(_target, _level, _format, _type, _pixels)
    }
    unsafe fn glPatchParameteri(&self, _pname: PatchParameterName, _value: GLint) {
        (self.entry().glPatchParameteri)(_pname, _value)
    }
    unsafe fn glProgramUniform1dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLdouble,
    ) {
        (self.entry().glProgramUniform1dv)(_program, _location, _count, _value)
    }
    unsafe fn glProgramUniformMatrix2x3fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        (self.entry().glProgramUniformMatrix2x3fv)(_program, _location, _count, _transpose, _value)
    }
    unsafe fn glBlendColor(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat, _alpha: GLfloat) {
        (self.entry().glBlendColor)(_red, _green, _blue, _alpha)
    }
    unsafe fn glProgramUniformMatrix4x3dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        (self.entry().glProgramUniformMatrix4x3dv)(_program, _location, _count, _transpose, _value)
    }
    unsafe fn glStencilOp(&self, _fail: StencilOp, _zfail: StencilOp, _zpass: StencilOp) {
        (self.entry().glStencilOp)(_fail, _zfail, _zpass)
    }
    unsafe fn glClampColor(&self, _target: ClampColorTargetARB, _clamp: ClampColorModeARB) {
        (self.entry().glClampColor)(_target, _clamp)
    }
    unsafe fn glTexImage2DMultisample(
        &self,
        _target: TextureTarget,
        _samples: GLsizei,
        _internalformat: InternalFormat,
        _width: GLsizei,
        _height: GLsizei,
        _fixedsamplelocations: GLboolean,
    ) {
        (self.entry().glTexImage2DMultisample)(
            _target,
            _samples,
            _internalformat,
            _width,
            _height,
            _fixedsamplelocations,
        )
    }
    unsafe fn glVertexP4uiv(&self, _type: VertexPointerType, _value: *const GLuint) {
        (self.entry().glVertexP4uiv)(_type, _value)
    }
    unsafe fn glDrawElements(
        &self,
        _mode: PrimitiveType,
        _count: GLsizei,
        _type: DrawElementsType,
        _indices: *const std::os::raw::c_void,
    ) {
        (self.entry().glDrawElements)(_mode, _count, _type, _indices)
    }
    unsafe fn glFramebufferTexture1D(
        &self,
        _target: FramebufferTarget,
        _attachment: FramebufferAttachment,
        _textarget: TextureTarget,
        _texture: GLuint,
        _level: GLint,
    ) {
        (self.entry().glFramebufferTexture1D)(_target, _attachment, _textarget, _texture, _level)
    }
    unsafe fn glProgramUniformMatrix2x3dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        (self.entry().glProgramUniformMatrix2x3dv)(_program, _location, _count, _transpose, _value)
    }
    unsafe fn glProgramUniform3d(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLdouble,
        _v1: GLdouble,
        _v2: GLdouble,
    ) {
        (self.entry().glProgramUniform3d)(_program, _location, _v0, _v1, _v2)
    }
    unsafe fn glBeginTransformFeedback(&self, _primitiveMode: PrimitiveType) {
        (self.entry().glBeginTransformFeedback)(_primitiveMode)
    }
    unsafe fn glUniformMatrix4x3dv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        (self.entry().glUniformMatrix4x3dv)(_location, _count, _transpose, _value)
    }
    unsafe fn glUniform3ui(&self, _location: GLint, _v0: GLuint, _v1: GLuint, _v2: GLuint) {
        (self.entry().glUniform3ui)(_location, _v0, _v1, _v2)
    }
    unsafe fn glTexImage3D(
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
        (self.entry().glTexImage3D)(
            _target,
            _level,
            _internalformat,
            _width,
            _height,
            _depth,
            _border,
            _format,
            _type,
            _pixels,
        )
    }
    unsafe fn glBindImageTexture(
        &self,
        _unit: GLuint,
        _texture: GLuint,
        _level: GLint,
        _layered: GLboolean,
        _layer: GLint,
        _access: BufferAccessARB,
        _format: InternalFormat,
    ) {
        (self.entry().glBindImageTexture)(
            _unit, _texture, _level, _layered, _layer, _access, _format,
        )
    }
    unsafe fn glDispatchCompute(
        &self,
        _num_groups_x: GLuint,
        _num_groups_y: GLuint,
        _num_groups_z: GLuint,
    ) {
        (self.entry().glDispatchCompute)(_num_groups_x, _num_groups_y, _num_groups_z)
    }
    unsafe fn glGetVertexAttribIiv(
        &self,
        _index: GLuint,
        _pname: VertexAttribEnum,
        _params: *mut GLint,
    ) {
        (self.entry().glGetVertexAttribIiv)(_index, _pname, _params)
    }
    unsafe fn glDisable(&self, _cap: EnableCap) {
        (self.entry().glDisable)(_cap)
    }
    unsafe fn glInvalidateTexImage(&self, _texture: GLuint, _level: GLint) {
        (self.entry().glInvalidateTexImage)(_texture, _level)
    }
    unsafe fn glUnmapBuffer(&self, _target: BufferTargetARB) -> GLboolean {
        (self.entry().glUnmapBuffer)(_target)
    }
    unsafe fn glUniform3fv(&self, _location: GLint, _count: GLsizei, _value: *const GLfloat) {
        (self.entry().glUniform3fv)(_location, _count, _value)
    }
    unsafe fn glWaitSync(&self, _sync: GLsync, _flags: SyncBehaviorFlags, _timeout: GLuint64) {
        (self.entry().glWaitSync)(_sync, _flags, _timeout)
    }
    unsafe fn glProgramUniform1i(&self, _program: GLuint, _location: GLint, _v0: GLint) {
        (self.entry().glProgramUniform1i)(_program, _location, _v0)
    }
    unsafe fn glClientWaitSync(
        &self,
        _sync: GLsync,
        _flags: SyncObjectMask,
        _timeout: GLuint64,
    ) -> GLenum {
        (self.entry().glClientWaitSync)(_sync, _flags, _timeout)
    }
    unsafe fn glBlendFunci(&self, _buf: GLuint, _src: BlendingFactor, _dst: BlendingFactor) {
        (self.entry().glBlendFunci)(_buf, _src, _dst)
    }
    unsafe fn glTexParameteriv(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _params: *const GLint,
    ) {
        (self.entry().glTexParameteriv)(_target, _pname, _params)
    }
    unsafe fn glVertexP2uiv(&self, _type: VertexPointerType, _value: *const GLuint) {
        (self.entry().glVertexP2uiv)(_type, _value)
    }
    unsafe fn glDrawElementsIndirect(
        &self,
        _mode: PrimitiveType,
        _type: DrawElementsType,
        _indirect: *const std::os::raw::c_void,
    ) {
        (self.entry().glDrawElementsIndirect)(_mode, _type, _indirect)
    }
    unsafe fn glGetProgramResourceName(
        &self,
        _program: GLuint,
        _programInterface: ProgramInterface,
        _index: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _name: *mut GLchar,
    ) {
        (self.entry().glGetProgramResourceName)(
            _program,
            _programInterface,
            _index,
            _bufSize,
            _length,
            _name,
        )
    }
    unsafe fn glVertexAttribI2iv(&self, _index: GLuint, _v: *const GLint) {
        (self.entry().glVertexAttribI2iv)(_index, _v)
    }
    unsafe fn glEndConditionalRender(&self) {
        (self.entry().glEndConditionalRender)()
    }
    unsafe fn glTexBuffer(
        &self,
        _target: TextureTarget,
        _internalformat: SizedInternalFormat,
        _buffer: GLuint,
    ) {
        (self.entry().glTexBuffer)(_target, _internalformat, _buffer)
    }
    unsafe fn glStencilMask(&self, _mask: GLuint) {
        (self.entry().glStencilMask)(_mask)
    }
    unsafe fn glUniform4dv(&self, _location: GLint, _count: GLsizei, _value: *const GLdouble) {
        (self.entry().glUniform4dv)(_location, _count, _value)
    }
    unsafe fn glEndQuery(&self, _target: QueryTarget) {
        (self.entry().glEndQuery)(_target)
    }
    unsafe fn glBlendFunc(&self, _sfactor: BlendingFactor, _dfactor: BlendingFactor) {
        (self.entry().glBlendFunc)(_sfactor, _dfactor)
    }
    unsafe fn glBindTexture(&self, _target: TextureTarget, _texture: GLuint) {
        (self.entry().glBindTexture)(_target, _texture)
    }
    unsafe fn glUniform1uiv(&self, _location: GLint, _count: GLsizei, _value: *const GLuint) {
        (self.entry().glUniform1uiv)(_location, _count, _value)
    }
    unsafe fn glDrawElementsInstanced(
        &self,
        _mode: PrimitiveType,
        _count: GLsizei,
        _type: DrawElementsType,
        _indices: *const std::os::raw::c_void,
        _instancecount: GLsizei,
    ) {
        (self.entry().glDrawElementsInstanced)(_mode, _count, _type, _indices, _instancecount)
    }
    unsafe fn glSamplerParameterIuiv(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterI,
        _param: *const GLuint,
    ) {
        (self.entry().glSamplerParameterIuiv)(_sampler, _pname, _param)
    }
    unsafe fn glGetTexLevelParameteriv(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _pname: GetTextureParameter,
        _params: *mut GLint,
    ) {
        (self.entry().glGetTexLevelParameteriv)(_target, _level, _pname, _params)
    }
    unsafe fn glSecondaryColorP3uiv(&self, _type: ColorPointerType, _color: *const GLuint) {
        (self.entry().glSecondaryColorP3uiv)(_type, _color)
    }
    unsafe fn glIsProgramPipeline(&self, _pipeline: GLuint) -> GLboolean {
        (self.entry().glIsProgramPipeline)(_pipeline)
    }
    unsafe fn glProgramUniform4iv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLint,
    ) {
        (self.entry().glProgramUniform4iv)(_program, _location, _count, _value)
    }
    unsafe fn glGetBufferSubData(
        &self,
        _target: BufferTargetARB,
        _offset: GLintptr,
        _size: GLsizeiptr,
        _data: *mut std::os::raw::c_void,
    ) {
        (self.entry().glGetBufferSubData)(_target, _offset, _size, _data)
    }
    unsafe fn glValidateProgram(&self, _program: GLuint) {
        (self.entry().glValidateProgram)(_program)
    }
    unsafe fn glProgramUniformMatrix2fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        (self.entry().glProgramUniformMatrix2fv)(_program, _location, _count, _transpose, _value)
    }
    unsafe fn glGetProgramPipelineiv(
        &self,
        _pipeline: GLuint,
        _pname: PipelineParameterName,
        _params: *mut GLint,
    ) {
        (self.entry().glGetProgramPipelineiv)(_pipeline, _pname, _params)
    }
    unsafe fn glGenFramebuffers(&self, _n: GLsizei, _framebuffers: *mut GLuint) {
        (self.entry().glGenFramebuffers)(_n, _framebuffers)
    }
    unsafe fn glEnablei(&self, _target: EnableCap, _index: GLuint) {
        (self.entry().glEnablei)(_target, _index)
    }
    unsafe fn glGenProgramPipelines(&self, _n: GLsizei, _pipelines: *mut GLuint) {
        (self.entry().glGenProgramPipelines)(_n, _pipelines)
    }
    unsafe fn glBindBuffer(&self, _target: BufferTargetARB, _buffer: GLuint) {
        (self.entry().glBindBuffer)(_target, _buffer)
    }
    unsafe fn glTexBufferRange(
        &self,
        _target: TextureTarget,
        _internalformat: SizedInternalFormat,
        _buffer: GLuint,
        _offset: GLintptr,
        _size: GLsizeiptr,
    ) {
        (self.entry().glTexBufferRange)(_target, _internalformat, _buffer, _offset, _size)
    }
    unsafe fn glTexImage1D(
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
        (self.entry().glTexImage1D)(
            _target,
            _level,
            _internalformat,
            _width,
            _border,
            _format,
            _type,
            _pixels,
        )
    }
    unsafe fn glBeginQuery(&self, _target: QueryTarget, _id: GLuint) {
        (self.entry().glBeginQuery)(_target, _id)
    }
    unsafe fn glBufferSubData(
        &self,
        _target: BufferTargetARB,
        _offset: GLintptr,
        _size: GLsizeiptr,
        _data: *const std::os::raw::c_void,
    ) {
        (self.entry().glBufferSubData)(_target, _offset, _size, _data)
    }
    unsafe fn glGetUniformiv(&self, _program: GLuint, _location: GLint, _params: *mut GLint) {
        (self.entry().glGetUniformiv)(_program, _location, _params)
    }
    unsafe fn glUniform2d(&self, _location: GLint, _x: GLdouble, _y: GLdouble) {
        (self.entry().glUniform2d)(_location, _x, _y)
    }
    unsafe fn glGetProgramInfoLog(
        &self,
        _program: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _infoLog: *mut GLchar,
    ) {
        (self.entry().glGetProgramInfoLog)(_program, _bufSize, _length, _infoLog)
    }
    unsafe fn glVertexAttrib4usv(&self, _index: GLuint, _v: *const GLushort) {
        (self.entry().glVertexAttrib4usv)(_index, _v)
    }
    unsafe fn glTexImage2D(
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
        (self.entry().glTexImage2D)(
            _target,
            _level,
            _internalformat,
            _width,
            _height,
            _border,
            _format,
            _type,
            _pixels,
        )
    }
    unsafe fn glGetProgramBinary(
        &self,
        _program: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _binaryFormat: *mut GLenum,
        _binary: *mut std::os::raw::c_void,
    ) {
        (self.entry().glGetProgramBinary)(_program, _bufSize, _length, _binaryFormat, _binary)
    }
    unsafe fn glTexParameterfv(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _params: *const GLfloat,
    ) {
        (self.entry().glTexParameterfv)(_target, _pname, _params)
    }
    unsafe fn glPixelStorei(&self, _pname: PixelStoreParameter, _param: GLint) {
        (self.entry().glPixelStorei)(_pname, _param)
    }
    unsafe fn glGetTexParameterfv(
        &self,
        _target: TextureTarget,
        _pname: GetTextureParameter,
        _params: *mut GLfloat,
    ) {
        (self.entry().glGetTexParameterfv)(_target, _pname, _params)
    }
    unsafe fn glDrawBuffer(&self, _buf: DrawBufferMode) {
        (self.entry().glDrawBuffer)(_buf)
    }
    unsafe fn glUniform4iv(&self, _location: GLint, _count: GLsizei, _value: *const GLint) {
        (self.entry().glUniform4iv)(_location, _count, _value)
    }
    unsafe fn glBeginQueryIndexed(&self, _target: QueryTarget, _index: GLuint, _id: GLuint) {
        (self.entry().glBeginQueryIndexed)(_target, _index, _id)
    }
    unsafe fn glTexCoordP2ui(&self, _type: TexCoordPointerType, _coords: GLuint) {
        (self.entry().glTexCoordP2ui)(_type, _coords)
    }
    unsafe fn glUniform1dv(&self, _location: GLint, _count: GLsizei, _value: *const GLdouble) {
        (self.entry().glUniform1dv)(_location, _count, _value)
    }
    unsafe fn glGetDoublev(&self, _pname: GetPName, _data: *mut GLdouble) {
        (self.entry().glGetDoublev)(_pname, _data)
    }
    unsafe fn glSamplerParameteriv(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterI,
        _param: *const GLint,
    ) {
        (self.entry().glSamplerParameteriv)(_sampler, _pname, _param)
    }
    unsafe fn glGenVertexArrays(&self, _n: GLsizei, _arrays: *mut GLuint) {
        (self.entry().glGenVertexArrays)(_n, _arrays)
    }
    unsafe fn glStencilFunc(&self, _func: StencilFunction, _ref: GLint, _mask: GLuint) {
        (self.entry().glStencilFunc)(_func, _ref, _mask)
    }
    unsafe fn glUniformMatrix2x4dv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        (self.entry().glUniformMatrix2x4dv)(_location, _count, _transpose, _value)
    }
    unsafe fn glProgramUniform1f(&self, _program: GLuint, _location: GLint, _v0: GLfloat) {
        (self.entry().glProgramUniform1f)(_program, _location, _v0)
    }
    unsafe fn glBindBuffersRange(
        &self,
        _target: BufferTargetARB,
        _first: GLuint,
        _count: GLsizei,
        _buffers: *const GLuint,
        _offsets: *const GLintptr,
        _sizes: *const GLsizeiptr,
    ) {
        (self.entry().glBindBuffersRange)(_target, _first, _count, _buffers, _offsets, _sizes)
    }
    unsafe fn glUniform4i(&self, _location: GLint, _v0: GLint, _v1: GLint, _v2: GLint, _v3: GLint) {
        (self.entry().glUniform4i)(_location, _v0, _v1, _v2, _v3)
    }
    unsafe fn glScissorIndexedv(&self, _index: GLuint, _v: *const GLint) {
        (self.entry().glScissorIndexedv)(_index, _v)
    }
    unsafe fn glSamplerParameterf(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterF,
        _param: GLfloat,
    ) {
        (self.entry().glSamplerParameterf)(_sampler, _pname, _param)
    }
    unsafe fn glClearBufferiv(&self, _buffer: Buffer, _drawbuffer: GLint, _value: *const GLint) {
        (self.entry().glClearBufferiv)(_buffer, _drawbuffer, _value)
    }
    unsafe fn glInvalidateTexSubImage(
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
        (self.entry().glInvalidateTexSubImage)(
            _texture, _level, _xoffset, _yoffset, _zoffset, _width, _height, _depth,
        )
    }
    unsafe fn glProgramUniform1ui(&self, _program: GLuint, _location: GLint, _v0: GLuint) {
        (self.entry().glProgramUniform1ui)(_program, _location, _v0)
    }
    unsafe fn glUniform2iv(&self, _location: GLint, _count: GLsizei, _value: *const GLint) {
        (self.entry().glUniform2iv)(_location, _count, _value)
    }
    unsafe fn glVertexAttrib4dv(&self, _index: GLuint, _v: *const GLdouble) {
        (self.entry().glVertexAttrib4dv)(_index, _v)
    }
    unsafe fn glFramebufferTexture3D(
        &self,
        _target: FramebufferTarget,
        _attachment: FramebufferAttachment,
        _textarget: TextureTarget,
        _texture: GLuint,
        _level: GLint,
        _zoffset: GLint,
    ) {
        (self.entry().glFramebufferTexture3D)(
            _target,
            _attachment,
            _textarget,
            _texture,
            _level,
            _zoffset,
        )
    }
    unsafe fn glGetActiveSubroutineUniformiv(
        &self,
        _program: GLuint,
        _shadertype: ShaderType,
        _index: GLuint,
        _pname: SubroutineParameterName,
        _values: *mut GLint,
    ) {
        (self.entry().glGetActiveSubroutineUniformiv)(
            _program,
            _shadertype,
            _index,
            _pname,
            _values,
        )
    }
    unsafe fn glMultiDrawArraysIndirect(
        &self,
        _mode: PrimitiveType,
        _indirect: *const std::os::raw::c_void,
        _drawcount: GLsizei,
        _stride: GLsizei,
    ) {
        (self.entry().glMultiDrawArraysIndirect)(_mode, _indirect, _drawcount, _stride)
    }
    unsafe fn glBindSampler(&self, _unit: GLuint, _sampler: GLuint) {
        (self.entry().glBindSampler)(_unit, _sampler)
    }
    unsafe fn glStencilFuncSeparate(
        &self,
        _face: StencilFaceDirection,
        _func: StencilFunction,
        _ref: GLint,
        _mask: GLuint,
    ) {
        (self.entry().glStencilFuncSeparate)(_face, _func, _ref, _mask)
    }
    unsafe fn glUniform3i(&self, _location: GLint, _v0: GLint, _v1: GLint, _v2: GLint) {
        (self.entry().glUniform3i)(_location, _v0, _v1, _v2)
    }
    unsafe fn glProgramUniformMatrix3fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        (self.entry().glProgramUniformMatrix3fv)(_program, _location, _count, _transpose, _value)
    }
    unsafe fn glProgramUniform4dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLdouble,
    ) {
        (self.entry().glProgramUniform4dv)(_program, _location, _count, _value)
    }
    unsafe fn glGenBuffers(&self, _n: GLsizei, _buffers: *mut GLuint) {
        (self.entry().glGenBuffers)(_n, _buffers)
    }
    unsafe fn glGetUniformdv(&self, _program: GLuint, _location: GLint, _params: *mut GLdouble) {
        (self.entry().glGetUniformdv)(_program, _location, _params)
    }
    unsafe fn glVertexAttribI1iv(&self, _index: GLuint, _v: *const GLint) {
        (self.entry().glVertexAttribI1iv)(_index, _v)
    }
    unsafe fn glGetProgramStageiv(
        &self,
        _program: GLuint,
        _shadertype: ShaderType,
        _pname: ProgramStagePName,
        _values: *mut GLint,
    ) {
        (self.entry().glGetProgramStageiv)(_program, _shadertype, _pname, _values)
    }
    unsafe fn glUniform2dv(&self, _location: GLint, _count: GLsizei, _value: *const GLdouble) {
        (self.entry().glUniform2dv)(_location, _count, _value)
    }
    unsafe fn glUniform2fv(&self, _location: GLint, _count: GLsizei, _value: *const GLfloat) {
        (self.entry().glUniform2fv)(_location, _count, _value)
    }
    unsafe fn glPointParameteriv(&self, _pname: PointParameterNameARB, _params: *const GLint) {
        (self.entry().glPointParameteriv)(_pname, _params)
    }
    unsafe fn glDeleteSamplers(&self, _count: GLsizei, _samplers: *const GLuint) {
        (self.entry().glDeleteSamplers)(_count, _samplers)
    }
    unsafe fn glBeginConditionalRender(&self, _id: GLuint, _mode: ConditionalRenderMode) {
        (self.entry().glBeginConditionalRender)(_id, _mode)
    }
    unsafe fn glVertexAttribL2dv(&self, _index: GLuint, _v: *const GLdouble) {
        (self.entry().glVertexAttribL2dv)(_index, _v)
    }
    unsafe fn glMultiTexCoordP1ui(
        &self,
        _texture: TextureUnit,
        _type: TexCoordPointerType,
        _coords: GLuint,
    ) {
        (self.entry().glMultiTexCoordP1ui)(_texture, _type, _coords)
    }
    unsafe fn glGetVertexAttribiv(
        &self,
        _index: GLuint,
        _pname: VertexAttribPropertyARB,
        _params: *mut GLint,
    ) {
        (self.entry().glGetVertexAttribiv)(_index, _pname, _params)
    }
    unsafe fn glGetVertexAttribfv(
        &self,
        _index: GLuint,
        _pname: VertexAttribPropertyARB,
        _params: *mut GLfloat,
    ) {
        (self.entry().glGetVertexAttribfv)(_index, _pname, _params)
    }
    unsafe fn glVertexAttribI4usv(&self, _index: GLuint, _v: *const GLushort) {
        (self.entry().glVertexAttribI4usv)(_index, _v)
    }
    unsafe fn glVertexAttribIFormat(
        &self,
        _attribindex: GLuint,
        _size: GLint,
        _type: VertexAttribIType,
        _relativeoffset: GLuint,
    ) {
        (self.entry().glVertexAttribIFormat)(_attribindex, _size, _type, _relativeoffset)
    }
    unsafe fn glFramebufferTexture(
        &self,
        _target: FramebufferTarget,
        _attachment: FramebufferAttachment,
        _texture: GLuint,
        _level: GLint,
    ) {
        (self.entry().glFramebufferTexture)(_target, _attachment, _texture, _level)
    }
    unsafe fn glProgramUniform2d(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLdouble,
        _v1: GLdouble,
    ) {
        (self.entry().glProgramUniform2d)(_program, _location, _v0, _v1)
    }
    unsafe fn glLogicOp(&self, _opcode: LogicOp) {
        (self.entry().glLogicOp)(_opcode)
    }
    unsafe fn glGetProgramResourceiv(
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
        (self.entry().glGetProgramResourceiv)(
            _program,
            _programInterface,
            _index,
            _propCount,
            _props,
            _count,
            _length,
            _params,
        )
    }
    unsafe fn glVertexAttrib4Nubv(&self, _index: GLuint, _v: *const GLubyte) {
        (self.entry().glVertexAttrib4Nubv)(_index, _v)
    }
    unsafe fn glVertexAttrib1fv(&self, _index: GLuint, _v: *const GLfloat) {
        (self.entry().glVertexAttrib1fv)(_index, _v)
    }
    unsafe fn glUniform3d(&self, _location: GLint, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glUniform3d)(_location, _x, _y, _z)
    }
    unsafe fn glDisablei(&self, _target: EnableCap, _index: GLuint) {
        (self.entry().glDisablei)(_target, _index)
    }
    unsafe fn glProgramUniform3fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLfloat,
    ) {
        (self.entry().glProgramUniform3fv)(_program, _location, _count, _value)
    }
    unsafe fn glObjectPtrLabel(
        &self,
        _ptr: *const std::os::raw::c_void,
        _length: GLsizei,
        _label: *const GLchar,
    ) {
        (self.entry().glObjectPtrLabel)(_ptr, _length, _label)
    }
    unsafe fn glTexCoordP3uiv(&self, _type: TexCoordPointerType, _coords: *const GLuint) {
        (self.entry().glTexCoordP3uiv)(_type, _coords)
    }
    unsafe fn glGetSamplerParameterIuiv(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterI,
        _params: *mut GLuint,
    ) {
        (self.entry().glGetSamplerParameterIuiv)(_sampler, _pname, _params)
    }
    unsafe fn glTransformFeedbackVaryings(
        &self,
        _program: GLuint,
        _count: GLsizei,
        _varyings: *const *const GLchar,
        _bufferMode: TransformFeedbackBufferMode,
    ) {
        (self.entry().glTransformFeedbackVaryings)(_program, _count, _varyings, _bufferMode)
    }
    unsafe fn glDepthRangef(&self, _n: GLfloat, _f: GLfloat) {
        (self.entry().glDepthRangef)(_n, _f)
    }
    unsafe fn glMultiTexCoordP2uiv(
        &self,
        _texture: TextureUnit,
        _type: TexCoordPointerType,
        _coords: *const GLuint,
    ) {
        (self.entry().glMultiTexCoordP2uiv)(_texture, _type, _coords)
    }
    unsafe fn glGetFramebufferParameteriv(
        &self,
        _target: FramebufferTarget,
        _pname: FramebufferAttachmentParameterName,
        _params: *mut GLint,
    ) {
        (self.entry().glGetFramebufferParameteriv)(_target, _pname, _params)
    }
    unsafe fn glGenTextures(&self, _n: GLsizei, _textures: *mut GLuint) {
        (self.entry().glGenTextures)(_n, _textures)
    }
    unsafe fn glIsEnabled(&self, _cap: EnableCap) -> GLboolean {
        (self.entry().glIsEnabled)(_cap)
    }
    unsafe fn glIsTexture(&self, _texture: GLuint) -> GLboolean {
        (self.entry().glIsTexture)(_texture)
    }
    unsafe fn glGetProgramiv(
        &self,
        _program: GLuint,
        _pname: ProgramPropertyARB,
        _params: *mut GLint,
    ) {
        (self.entry().glGetProgramiv)(_program, _pname, _params)
    }
    unsafe fn glTexCoordP3ui(&self, _type: TexCoordPointerType, _coords: GLuint) {
        (self.entry().glTexCoordP3ui)(_type, _coords)
    }
    unsafe fn glUniformMatrix4x2fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        (self.entry().glUniformMatrix4x2fv)(_location, _count, _transpose, _value)
    }
    unsafe fn glGetFloati_v(&self, _target: GetPName, _index: GLuint, _data: *mut GLfloat) {
        (self.entry().glGetFloati_v)(_target, _index, _data)
    }
    unsafe fn glMultiDrawArrays(
        &self,
        _mode: PrimitiveType,
        _first: *const GLint,
        _count: *const GLsizei,
        _drawcount: GLsizei,
    ) {
        (self.entry().glMultiDrawArrays)(_mode, _first, _count, _drawcount)
    }
    unsafe fn glFrontFace(&self, _mode: FrontFaceDirection) {
        (self.entry().glFrontFace)(_mode)
    }
    unsafe fn glVertexAttribP4ui(
        &self,
        _index: GLuint,
        _type: VertexAttribPointerType,
        _normalized: GLboolean,
        _value: GLuint,
    ) {
        (self.entry().glVertexAttribP4ui)(_index, _type, _normalized, _value)
    }
    unsafe fn glUseProgramStages(
        &self,
        _pipeline: GLuint,
        _stages: UseProgramStageMask,
        _program: GLuint,
    ) {
        (self.entry().glUseProgramStages)(_pipeline, _stages, _program)
    }
    unsafe fn glUniform1fv(&self, _location: GLint, _count: GLsizei, _value: *const GLfloat) {
        (self.entry().glUniform1fv)(_location, _count, _value)
    }
    unsafe fn glVertexAttribL1d(&self, _index: GLuint, _x: GLdouble) {
        (self.entry().glVertexAttribL1d)(_index, _x)
    }
    unsafe fn glReadBuffer(&self, _src: ReadBufferMode) {
        (self.entry().glReadBuffer)(_src)
    }
    unsafe fn glActiveShaderProgram(&self, _pipeline: GLuint, _program: GLuint) {
        (self.entry().glActiveShaderProgram)(_pipeline, _program)
    }
    unsafe fn glMapBuffer(
        &self,
        _target: BufferTargetARB,
        _access: BufferAccessARB,
    ) -> *mut std::os::raw::c_void {
        (self.entry().glMapBuffer)(_target, _access)
    }
    unsafe fn glCompressedTexSubImage3D(
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
        (self.entry().glCompressedTexSubImage3D)(
            _target, _level, _xoffset, _yoffset, _zoffset, _width, _height, _depth, _format,
            _imageSize, _data,
        )
    }
    unsafe fn glGetUniformLocation(&self, _program: GLuint, _name: *const GLchar) -> GLint {
        (self.entry().glGetUniformLocation)(_program, _name)
    }
    unsafe fn glBlendEquation(&self, _mode: BlendEquationModeEXT) {
        (self.entry().glBlendEquation)(_mode)
    }
    unsafe fn glClearColor(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat, _alpha: GLfloat) {
        (self.entry().glClearColor)(_red, _green, _blue, _alpha)
    }
    unsafe fn glPointParameterf(&self, _pname: PointParameterNameARB, _param: GLfloat) {
        (self.entry().glPointParameterf)(_pname, _param)
    }
    unsafe fn glProgramUniform1uiv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLuint,
    ) {
        (self.entry().glProgramUniform1uiv)(_program, _location, _count, _value)
    }
    unsafe fn glInvalidateFramebuffer(
        &self,
        _target: FramebufferTarget,
        _numAttachments: GLsizei,
        _attachments: *const InvalidateFramebufferAttachment,
    ) {
        (self.entry().glInvalidateFramebuffer)(_target, _numAttachments, _attachments)
    }
    unsafe fn glGetCompressedTexImage(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _img: *mut std::os::raw::c_void,
    ) {
        (self.entry().glGetCompressedTexImage)(_target, _level, _img)
    }
    unsafe fn glFramebufferParameteri(
        &self,
        _target: FramebufferTarget,
        _pname: FramebufferParameterName,
        _param: GLint,
    ) {
        (self.entry().glFramebufferParameteri)(_target, _pname, _param)
    }
    unsafe fn glVertexP2ui(&self, _type: VertexPointerType, _value: GLuint) {
        (self.entry().glVertexP2ui)(_type, _value)
    }
    unsafe fn glBindTextures(&self, _first: GLuint, _count: GLsizei, _textures: *const GLuint) {
        (self.entry().glBindTextures)(_first, _count, _textures)
    }
    unsafe fn glDeleteTextures(&self, _n: GLsizei, _textures: *const GLuint) {
        (self.entry().glDeleteTextures)(_n, _textures)
    }
    unsafe fn glProgramUniformMatrix4dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        (self.entry().glProgramUniformMatrix4dv)(_program, _location, _count, _transpose, _value)
    }
    unsafe fn glPatchParameterfv(&self, _pname: PatchParameterName, _values: *const GLfloat) {
        (self.entry().glPatchParameterfv)(_pname, _values)
    }
    unsafe fn glDrawElementsInstancedBaseInstance(
        &self,
        _mode: PrimitiveType,
        _count: GLsizei,
        _type: PrimitiveType,
        _indices: *const std::os::raw::c_void,
        _instancecount: GLsizei,
        _baseinstance: GLuint,
    ) {
        (self.entry().glDrawElementsInstancedBaseInstance)(
            _mode,
            _count,
            _type,
            _indices,
            _instancecount,
            _baseinstance,
        )
    }
    unsafe fn glTextureView(
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
        (self.entry().glTextureView)(
            _texture,
            _target,
            _origtexture,
            _internalformat,
            _minlevel,
            _numlevels,
            _minlayer,
            _numlayers,
        )
    }
    unsafe fn glProgramUniform3iv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLint,
    ) {
        (self.entry().glProgramUniform3iv)(_program, _location, _count, _value)
    }
    unsafe fn glVertexAttrib4Nub(
        &self,
        _index: GLuint,
        _x: GLubyte,
        _y: GLubyte,
        _z: GLubyte,
        _w: GLubyte,
    ) {
        (self.entry().glVertexAttrib4Nub)(_index, _x, _y, _z, _w)
    }
    unsafe fn glGetObjectLabel(
        &self,
        _identifier: ObjectIdentifier,
        _name: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _label: *mut GLchar,
    ) {
        (self.entry().glGetObjectLabel)(_identifier, _name, _bufSize, _length, _label)
    }
    unsafe fn glLinkProgram(&self, _program: GLuint) {
        (self.entry().glLinkProgram)(_program)
    }
    unsafe fn glProgramUniform3dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLdouble,
    ) {
        (self.entry().glProgramUniform3dv)(_program, _location, _count, _value)
    }
    unsafe fn glTexSubImage3D(
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
        (self.entry().glTexSubImage3D)(
            _target, _level, _xoffset, _yoffset, _zoffset, _width, _height, _depth, _format, _type,
            _pixels,
        )
    }
    unsafe fn glGetInternalformativ(
        &self,
        _target: TextureTarget,
        _internalformat: InternalFormat,
        _pname: InternalFormatPName,
        _count: GLsizei,
        _params: *mut GLint,
    ) {
        (self.entry().glGetInternalformativ)(_target, _internalformat, _pname, _count, _params)
    }
    unsafe fn glGetAttachedShaders(
        &self,
        _program: GLuint,
        _maxCount: GLsizei,
        _count: *mut GLsizei,
        _shaders: *mut GLuint,
    ) {
        (self.entry().glGetAttachedShaders)(_program, _maxCount, _count, _shaders)
    }
    unsafe fn glGetFragDataIndex(&self, _program: GLuint, _name: *const GLchar) -> GLint {
        (self.entry().glGetFragDataIndex)(_program, _name)
    }
    unsafe fn glDebugMessageCallback(
        &self,
        _callback: GLDEBUGPROC,
        _userParam: *const std::os::raw::c_void,
    ) {
        (self.entry().glDebugMessageCallback)(_callback, _userParam)
    }
    unsafe fn glCompressedTexImage1D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _internalformat: InternalFormat,
        _width: GLsizei,
        _border: GLint,
        _imageSize: GLsizei,
        _data: *const std::os::raw::c_void,
    ) {
        (self.entry().glCompressedTexImage1D)(
            _target,
            _level,
            _internalformat,
            _width,
            _border,
            _imageSize,
            _data,
        )
    }
    unsafe fn glGetActiveUniform(
        &self,
        _program: GLuint,
        _index: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _size: *mut GLint,
        _type: *mut UniformType,
        _name: *mut GLchar,
    ) {
        (self.entry().glGetActiveUniform)(_program, _index, _bufSize, _length, _size, _type, _name)
    }
    unsafe fn glUniformMatrix2fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        (self.entry().glUniformMatrix2fv)(_location, _count, _transpose, _value)
    }
    unsafe fn glVertexAttrib4s(
        &self,
        _index: GLuint,
        _x: GLshort,
        _y: GLshort,
        _z: GLshort,
        _w: GLshort,
    ) {
        (self.entry().glVertexAttrib4s)(_index, _x, _y, _z, _w)
    }
    unsafe fn glGetActiveUniformBlockiv(
        &self,
        _program: GLuint,
        _uniformBlockIndex: GLuint,
        _pname: UniformBlockPName,
        _params: *mut GLint,
    ) {
        (self.entry().glGetActiveUniformBlockiv)(_program, _uniformBlockIndex, _pname, _params)
    }
    unsafe fn glGetSamplerParameterfv(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterF,
        _params: *mut GLfloat,
    ) {
        (self.entry().glGetSamplerParameterfv)(_sampler, _pname, _params)
    }
    unsafe fn glIsTransformFeedback(&self, _id: GLuint) -> GLboolean {
        (self.entry().glIsTransformFeedback)(_id)
    }
    unsafe fn glBindBuffersBase(
        &self,
        _target: BufferTargetARB,
        _first: GLuint,
        _count: GLsizei,
        _buffers: *const GLuint,
    ) {
        (self.entry().glBindBuffersBase)(_target, _first, _count, _buffers)
    }
    unsafe fn glViewportArrayv(&self, _first: GLuint, _count: GLsizei, _v: *const GLfloat) {
        (self.entry().glViewportArrayv)(_first, _count, _v)
    }
    unsafe fn glCompressedTexImage3D(
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
        (self.entry().glCompressedTexImage3D)(
            _target,
            _level,
            _internalformat,
            _width,
            _height,
            _depth,
            _border,
            _imageSize,
            _data,
        )
    }
    unsafe fn glVertexAttrib3f(&self, _index: GLuint, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glVertexAttrib3f)(_index, _x, _y, _z)
    }
    unsafe fn glUniform2i(&self, _location: GLint, _v0: GLint, _v1: GLint) {
        (self.entry().glUniform2i)(_location, _v0, _v1)
    }
    unsafe fn glGetIntegerv(&self, _pname: GetPName, _data: *mut GLint) {
        (self.entry().glGetIntegerv)(_pname, _data)
    }
    unsafe fn glMultiDrawElementsIndirect(
        &self,
        _mode: PrimitiveType,
        _type: DrawElementsType,
        _indirect: *const std::os::raw::c_void,
        _drawcount: GLsizei,
        _stride: GLsizei,
    ) {
        (self.entry().glMultiDrawElementsIndirect)(_mode, _type, _indirect, _drawcount, _stride)
    }
    unsafe fn glCopyBufferSubData(
        &self,
        _readTarget: CopyBufferSubDataTarget,
        _writeTarget: CopyBufferSubDataTarget,
        _readOffset: GLintptr,
        _writeOffset: GLintptr,
        _size: GLsizeiptr,
    ) {
        (self.entry().glCopyBufferSubData)(
            _readTarget,
            _writeTarget,
            _readOffset,
            _writeOffset,
            _size,
        )
    }
    unsafe fn glMemoryBarrier(&self, _barriers: MemoryBarrierMask) {
        (self.entry().glMemoryBarrier)(_barriers)
    }
    unsafe fn glProgramUniformMatrix3x2fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        (self.entry().glProgramUniformMatrix3x2fv)(_program, _location, _count, _transpose, _value)
    }
    unsafe fn glDrawRangeElements(
        &self,
        _mode: PrimitiveType,
        _start: GLuint,
        _end: GLuint,
        _count: GLsizei,
        _type: DrawElementsType,
        _indices: *const std::os::raw::c_void,
    ) {
        (self.entry().glDrawRangeElements)(_mode, _start, _end, _count, _type, _indices)
    }
    unsafe fn glGenRenderbuffers(&self, _n: GLsizei, _renderbuffers: *mut GLuint) {
        (self.entry().glGenRenderbuffers)(_n, _renderbuffers)
    }
    unsafe fn glProgramUniform4f(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLfloat,
        _v1: GLfloat,
        _v2: GLfloat,
        _v3: GLfloat,
    ) {
        (self.entry().glProgramUniform4f)(_program, _location, _v0, _v1, _v2, _v3)
    }
    unsafe fn glVertexAttribFormat(
        &self,
        _attribindex: GLuint,
        _size: GLint,
        _type: VertexAttribType,
        _normalized: GLboolean,
        _relativeoffset: GLuint,
    ) {
        (self.entry().glVertexAttribFormat)(
            _attribindex,
            _size,
            _type,
            _normalized,
            _relativeoffset,
        )
    }
    unsafe fn glScissorIndexed(
        &self,
        _index: GLuint,
        _left: GLint,
        _bottom: GLint,
        _width: GLsizei,
        _height: GLsizei,
    ) {
        (self.entry().glScissorIndexed)(_index, _left, _bottom, _width, _height)
    }
    unsafe fn glScissorArrayv(&self, _first: GLuint, _count: GLsizei, _v: *const GLint) {
        (self.entry().glScissorArrayv)(_first, _count, _v)
    }
    unsafe fn glGetProgramResourceIndex(
        &self,
        _program: GLuint,
        _programInterface: ProgramInterface,
        _name: *const GLchar,
    ) -> GLuint {
        (self.entry().glGetProgramResourceIndex)(_program, _programInterface, _name)
    }
    unsafe fn glFlush(&self) {
        (self.entry().glFlush)()
    }
    unsafe fn glEndQueryIndexed(&self, _target: QueryTarget, _index: GLuint) {
        (self.entry().glEndQueryIndexed)(_target, _index)
    }
    unsafe fn glGenQueries(&self, _n: GLsizei, _ids: *mut GLuint) {
        (self.entry().glGenQueries)(_n, _ids)
    }
    unsafe fn glUniform2ui(&self, _location: GLint, _v0: GLuint, _v1: GLuint) {
        (self.entry().glUniform2ui)(_location, _v0, _v1)
    }
    unsafe fn glGetTexParameterIuiv(
        &self,
        _target: TextureTarget,
        _pname: GetTextureParameter,
        _params: *mut GLuint,
    ) {
        (self.entry().glGetTexParameterIuiv)(_target, _pname, _params)
    }
    unsafe fn glMultiTexCoordP2ui(
        &self,
        _texture: TextureUnit,
        _type: TexCoordPointerType,
        _coords: GLuint,
    ) {
        (self.entry().glMultiTexCoordP2ui)(_texture, _type, _coords)
    }
    unsafe fn glVertexAttribI3i(&self, _index: GLuint, _x: GLint, _y: GLint, _z: GLint) {
        (self.entry().glVertexAttribI3i)(_index, _x, _y, _z)
    }
    unsafe fn glGetString(&self, _name: StringName) -> *const GLubyte {
        (self.entry().glGetString)(_name)
    }
    unsafe fn glVertexAttribL4d(
        &self,
        _index: GLuint,
        _x: GLdouble,
        _y: GLdouble,
        _z: GLdouble,
        _w: GLdouble,
    ) {
        (self.entry().glVertexAttribL4d)(_index, _x, _y, _z, _w)
    }
    unsafe fn glBufferData(
        &self,
        _target: BufferTargetARB,
        _size: GLsizeiptr,
        _data: *const std::os::raw::c_void,
        _usage: BufferUsageARB,
    ) {
        (self.entry().glBufferData)(_target, _size, _data, _usage)
    }
    unsafe fn glIsShader(&self, _shader: GLuint) -> GLboolean {
        (self.entry().glIsShader)(_shader)
    }
    unsafe fn glVertexAttrib1s(&self, _index: GLuint, _x: GLshort) {
        (self.entry().glVertexAttrib1s)(_index, _x)
    }
    unsafe fn glVertexAttrib2f(&self, _index: GLuint, _x: GLfloat, _y: GLfloat) {
        (self.entry().glVertexAttrib2f)(_index, _x, _y)
    }
    unsafe fn glVertexAttrib3fv(&self, _index: GLuint, _v: *const GLfloat) {
        (self.entry().glVertexAttrib3fv)(_index, _v)
    }
    unsafe fn glClearBufferuiv(&self, _buffer: Buffer, _drawbuffer: GLint, _value: *const GLuint) {
        (self.entry().glClearBufferuiv)(_buffer, _drawbuffer, _value)
    }
    unsafe fn glGetStringi(&self, _name: StringName, _index: GLuint) -> *const GLubyte {
        (self.entry().glGetStringi)(_name, _index)
    }
    unsafe fn glGetFramebufferAttachmentParameteriv(
        &self,
        _target: FramebufferTarget,
        _attachment: FramebufferAttachment,
        _pname: FramebufferAttachmentParameterName,
        _params: *mut GLint,
    ) {
        (self.entry().glGetFramebufferAttachmentParameteriv)(_target, _attachment, _pname, _params)
    }
    unsafe fn glActiveTexture(&self, _texture: TextureUnit) {
        (self.entry().glActiveTexture)(_texture)
    }
    unsafe fn glTexParameterIuiv(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _params: *const GLuint,
    ) {
        (self.entry().glTexParameterIuiv)(_target, _pname, _params)
    }
    unsafe fn glObjectLabel(
        &self,
        _identifier: ObjectIdentifier,
        _name: GLuint,
        _length: GLsizei,
        _label: *const GLchar,
    ) {
        (self.entry().glObjectLabel)(_identifier, _name, _length, _label)
    }
    unsafe fn glProgramUniformMatrix4fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        (self.entry().glProgramUniformMatrix4fv)(_program, _location, _count, _transpose, _value)
    }
    unsafe fn glUniform1iv(&self, _location: GLint, _count: GLsizei, _value: *const GLint) {
        (self.entry().glUniform1iv)(_location, _count, _value)
    }
    unsafe fn glGetMultisamplefv(
        &self,
        _pname: GetMultisamplePNameNV,
        _index: GLuint,
        _val: *mut GLfloat,
    ) {
        (self.entry().glGetMultisamplefv)(_pname, _index, _val)
    }
    unsafe fn glCopyTexSubImage3D(
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
        (self.entry().glCopyTexSubImage3D)(
            _target, _level, _xoffset, _yoffset, _zoffset, _x, _y, _width, _height,
        )
    }
    unsafe fn glGetQueryIndexediv(
        &self,
        _target: QueryTarget,
        _index: GLuint,
        _pname: QueryParameterName,
        _params: *mut GLint,
    ) {
        (self.entry().glGetQueryIndexediv)(_target, _index, _pname, _params)
    }
    unsafe fn glGetSubroutineUniformLocation(
        &self,
        _program: GLuint,
        _shadertype: ShaderType,
        _name: *const GLchar,
    ) -> GLint {
        (self.entry().glGetSubroutineUniformLocation)(_program, _shadertype, _name)
    }
    unsafe fn glShaderStorageBlockBinding(
        &self,
        _program: GLuint,
        _storageBlockIndex: GLuint,
        _storageBlockBinding: GLuint,
    ) {
        (self.entry().glShaderStorageBlockBinding)(
            _program,
            _storageBlockIndex,
            _storageBlockBinding,
        )
    }
    unsafe fn glGetVertexAttribIuiv(
        &self,
        _index: GLuint,
        _pname: VertexAttribEnum,
        _params: *mut GLuint,
    ) {
        (self.entry().glGetVertexAttribIuiv)(_index, _pname, _params)
    }
    unsafe fn glCullFace(&self, _mode: CullFaceMode) {
        (self.entry().glCullFace)(_mode)
    }
    unsafe fn glAttachShader(&self, _program: GLuint, _shader: GLuint) {
        (self.entry().glAttachShader)(_program, _shader)
    }
    unsafe fn glIsBuffer(&self, _buffer: GLuint) -> GLboolean {
        (self.entry().glIsBuffer)(_buffer)
    }
    unsafe fn glGetBufferParameteriv(
        &self,
        _target: BufferTargetARB,
        _pname: BufferPNameARB,
        _params: *mut GLint,
    ) {
        (self.entry().glGetBufferParameteriv)(_target, _pname, _params)
    }
    unsafe fn glSampleMaski(&self, _maskNumber: GLuint, _mask: GLbitfield) {
        (self.entry().glSampleMaski)(_maskNumber, _mask)
    }
    unsafe fn glDeleteProgram(&self, _program: GLuint) {
        (self.entry().glDeleteProgram)(_program)
    }
    unsafe fn glDispatchComputeIndirect(&self, _indirect: GLintptr) {
        (self.entry().glDispatchComputeIndirect)(_indirect)
    }
    unsafe fn glUniform4uiv(&self, _location: GLint, _count: GLsizei, _value: *const GLuint) {
        (self.entry().glUniform4uiv)(_location, _count, _value)
    }
    unsafe fn glIsSampler(&self, _sampler: GLuint) -> GLboolean {
        (self.entry().glIsSampler)(_sampler)
    }
    unsafe fn glGetDoublei_v(&self, _target: GetPName, _index: GLuint, _data: *mut GLdouble) {
        (self.entry().glGetDoublei_v)(_target, _index, _data)
    }
    unsafe fn glDebugMessageControl(
        &self,
        _source: DebugSource,
        _type: DebugType,
        _severity: DebugSeverity,
        _count: GLsizei,
        _ids: *const GLuint,
        _enabled: GLboolean,
    ) {
        (self.entry().glDebugMessageControl)(_source, _type, _severity, _count, _ids, _enabled)
    }
    unsafe fn glTexParameteri(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _param: GLint,
    ) {
        (self.entry().glTexParameteri)(_target, _pname, _param)
    }
    unsafe fn glGenSamplers(&self, _count: GLsizei, _samplers: *mut GLuint) {
        (self.entry().glGenSamplers)(_count, _samplers)
    }
    unsafe fn glVertexAttribL2d(&self, _index: GLuint, _x: GLdouble, _y: GLdouble) {
        (self.entry().glVertexAttribL2d)(_index, _x, _y)
    }
    unsafe fn glVertexAttribL1dv(&self, _index: GLuint, _v: *const GLdouble) {
        (self.entry().glVertexAttribL1dv)(_index, _v)
    }
    unsafe fn glBindVertexArray(&self, _array: GLuint) {
        (self.entry().glBindVertexArray)(_array)
    }
    unsafe fn glTexStorage3D(
        &self,
        _target: TextureTarget,
        _levels: GLsizei,
        _internalformat: SizedInternalFormat,
        _width: GLsizei,
        _height: GLsizei,
        _depth: GLsizei,
    ) {
        (self.entry().glTexStorage3D)(_target, _levels, _internalformat, _width, _height, _depth)
    }
    unsafe fn glGenTransformFeedbacks(&self, _n: GLsizei, _ids: *mut GLuint) {
        (self.entry().glGenTransformFeedbacks)(_n, _ids)
    }
    unsafe fn glBindBufferRange(
        &self,
        _target: BufferTargetARB,
        _index: GLuint,
        _buffer: GLuint,
        _offset: GLintptr,
        _size: GLsizeiptr,
    ) {
        (self.entry().glBindBufferRange)(_target, _index, _buffer, _offset, _size)
    }
    unsafe fn glCopyTexImage2D(
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
        (self.entry().glCopyTexImage2D)(
            _target,
            _level,
            _internalformat,
            _x,
            _y,
            _width,
            _height,
            _border,
        )
    }
    unsafe fn glUniform3uiv(&self, _location: GLint, _count: GLsizei, _value: *const GLuint) {
        (self.entry().glUniform3uiv)(_location, _count, _value)
    }
    unsafe fn glBindRenderbuffer(&self, _target: RenderbufferTarget, _renderbuffer: GLuint) {
        (self.entry().glBindRenderbuffer)(_target, _renderbuffer)
    }
    unsafe fn glVertexAttrib3s(&self, _index: GLuint, _x: GLshort, _y: GLshort, _z: GLshort) {
        (self.entry().glVertexAttrib3s)(_index, _x, _y, _z)
    }
    unsafe fn glGetQueryObjecti64v(
        &self,
        _id: GLuint,
        _pname: QueryObjectParameterName,
        _params: *mut GLint64,
    ) {
        (self.entry().glGetQueryObjecti64v)(_id, _pname, _params)
    }
    unsafe fn glUniformMatrix3x4fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        (self.entry().glUniformMatrix3x4fv)(_location, _count, _transpose, _value)
    }
    unsafe fn glStencilOpSeparate(
        &self,
        _face: StencilFaceDirection,
        _sfail: StencilOp,
        _dpfail: StencilOp,
        _dppass: StencilOp,
    ) {
        (self.entry().glStencilOpSeparate)(_face, _sfail, _dpfail, _dppass)
    }
    unsafe fn glIsProgram(&self, _program: GLuint) -> GLboolean {
        (self.entry().glIsProgram)(_program)
    }
    unsafe fn glColorMask(
        &self,
        _red: GLboolean,
        _green: GLboolean,
        _blue: GLboolean,
        _alpha: GLboolean,
    ) {
        (self.entry().glColorMask)(_red, _green, _blue, _alpha)
    }
    unsafe fn glVertexP3uiv(&self, _type: VertexPointerType, _value: *const GLuint) {
        (self.entry().glVertexP3uiv)(_type, _value)
    }
    unsafe fn glBlendFuncSeparatei(
        &self,
        _buf: GLuint,
        _srcRGB: BlendingFactor,
        _dstRGB: BlendingFactor,
        _srcAlpha: BlendingFactor,
        _dstAlpha: BlendingFactor,
    ) {
        (self.entry().glBlendFuncSeparatei)(_buf, _srcRGB, _dstRGB, _srcAlpha, _dstAlpha)
    }
    unsafe fn glTexStorage3DMultisample(
        &self,
        _target: TextureTarget,
        _samples: GLsizei,
        _internalformat: SizedInternalFormat,
        _width: GLsizei,
        _height: GLsizei,
        _depth: GLsizei,
        _fixedsamplelocations: GLboolean,
    ) {
        (self.entry().glTexStorage3DMultisample)(
            _target,
            _samples,
            _internalformat,
            _width,
            _height,
            _depth,
            _fixedsamplelocations,
        )
    }
    unsafe fn glVertexAttribP4uiv(
        &self,
        _index: GLuint,
        _type: VertexAttribPointerType,
        _normalized: GLboolean,
        _value: *const GLuint,
    ) {
        (self.entry().glVertexAttribP4uiv)(_index, _type, _normalized, _value)
    }
    unsafe fn glClear(&self, _mask: ClearBufferMask) {
        (self.entry().glClear)(_mask)
    }
    unsafe fn glVertexAttrib4Nbv(&self, _index: GLuint, _v: *const GLbyte) {
        (self.entry().glVertexAttrib4Nbv)(_index, _v)
    }
    unsafe fn glVertexAttrib4Nuiv(&self, _index: GLuint, _v: *const GLuint) {
        (self.entry().glVertexAttrib4Nuiv)(_index, _v)
    }
    unsafe fn glVertexAttribI4iv(&self, _index: GLuint, _v: *const GLint) {
        (self.entry().glVertexAttribI4iv)(_index, _v)
    }
    unsafe fn glDrawRangeElementsBaseVertex(
        &self,
        _mode: PrimitiveType,
        _start: GLuint,
        _end: GLuint,
        _count: GLsizei,
        _type: DrawElementsType,
        _indices: *const std::os::raw::c_void,
        _basevertex: GLint,
    ) {
        (self.entry().glDrawRangeElementsBaseVertex)(
            _mode,
            _start,
            _end,
            _count,
            _type,
            _indices,
            _basevertex,
        )
    }
    unsafe fn glGetSamplerParameteriv(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterI,
        _params: *mut GLint,
    ) {
        (self.entry().glGetSamplerParameteriv)(_sampler, _pname, _params)
    }
    unsafe fn glVertexAttrib1d(&self, _index: GLuint, _x: GLdouble) {
        (self.entry().glVertexAttrib1d)(_index, _x)
    }
    unsafe fn glUniform2uiv(&self, _location: GLint, _count: GLsizei, _value: *const GLuint) {
        (self.entry().glUniform2uiv)(_location, _count, _value)
    }
    unsafe fn glQueryCounter(&self, _id: GLuint, _target: QueryCounterTarget) {
        (self.entry().glQueryCounter)(_id, _target)
    }
    unsafe fn glVertexAttrib2fv(&self, _index: GLuint, _v: *const GLfloat) {
        (self.entry().glVertexAttrib2fv)(_index, _v)
    }
    unsafe fn glGetUniformfv(&self, _program: GLuint, _location: GLint, _params: *mut GLfloat) {
        (self.entry().glGetUniformfv)(_program, _location, _params)
    }
    unsafe fn glUniformMatrix4x2dv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        (self.entry().glUniformMatrix4x2dv)(_location, _count, _transpose, _value)
    }
    unsafe fn glClearBufferfi(
        &self,
        _buffer: Buffer,
        _drawbuffer: GLint,
        _depth: GLfloat,
        _stencil: GLint,
    ) {
        (self.entry().glClearBufferfi)(_buffer, _drawbuffer, _depth, _stencil)
    }
    unsafe fn glProgramUniform1iv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLint,
    ) {
        (self.entry().glProgramUniform1iv)(_program, _location, _count, _value)
    }
    unsafe fn glInvalidateBufferSubData(
        &self,
        _buffer: GLuint,
        _offset: GLintptr,
        _length: GLsizeiptr,
    ) {
        (self.entry().glInvalidateBufferSubData)(_buffer, _offset, _length)
    }
    unsafe fn glCopyTexSubImage1D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _xoffset: GLint,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
    ) {
        (self.entry().glCopyTexSubImage1D)(_target, _level, _xoffset, _x, _y, _width)
    }
    unsafe fn glDeleteSync(&self, _sync: GLsync) {
        (self.entry().glDeleteSync)(_sync)
    }
    unsafe fn glGetProgramPipelineInfoLog(
        &self,
        _pipeline: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _infoLog: *mut GLchar,
    ) {
        (self.entry().glGetProgramPipelineInfoLog)(_pipeline, _bufSize, _length, _infoLog)
    }
    unsafe fn glDepthRangeIndexed(&self, _index: GLuint, _n: GLdouble, _f: GLdouble) {
        (self.entry().glDepthRangeIndexed)(_index, _n, _f)
    }
    unsafe fn glDeleteRenderbuffers(&self, _n: GLsizei, _renderbuffers: *const GLuint) {
        (self.entry().glDeleteRenderbuffers)(_n, _renderbuffers)
    }
    unsafe fn glCheckFramebufferStatus(&self, _target: FramebufferTarget) -> GLenum {
        (self.entry().glCheckFramebufferStatus)(_target)
    }
    unsafe fn glNormalP3uiv(&self, _type: NormalPointerType, _coords: *const GLuint) {
        (self.entry().glNormalP3uiv)(_type, _coords)
    }
    unsafe fn glShaderSource(
        &self,
        _shader: GLuint,
        _count: GLsizei,
        _string: *const *const GLchar,
        _length: *const GLint,
    ) {
        (self.entry().glShaderSource)(_shader, _count, _string, _length)
    }
    unsafe fn glGetBufferPointerv(
        &self,
        _target: BufferTargetARB,
        _pname: BufferPointerNameARB,
        _params: *mut *mut std::os::raw::c_void,
    ) {
        (self.entry().glGetBufferPointerv)(_target, _pname, _params)
    }
    unsafe fn glCopyImageSubData(
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
        (self.entry().glCopyImageSubData)(
            _srcName, _srcTarget, _srcLevel, _srcX, _srcY, _srcZ, _dstName, _dstTarget, _dstLevel,
            _dstX, _dstY, _dstZ, _srcWidth, _srcHeight, _srcDepth,
        )
    }
    unsafe fn glGetError(&self) -> GLenum {
        (self.entry().glGetError)()
    }
    unsafe fn glCopyTexSubImage2D(
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
        (self.entry().glCopyTexSubImage2D)(
            _target, _level, _xoffset, _yoffset, _x, _y, _width, _height,
        )
    }
    unsafe fn glPointParameterfv(&self, _pname: PointParameterNameARB, _params: *const GLfloat) {
        (self.entry().glPointParameterfv)(_pname, _params)
    }
    unsafe fn glVertexAttribDivisor(&self, _index: GLuint, _divisor: GLuint) {
        (self.entry().glVertexAttribDivisor)(_index, _divisor)
    }
    unsafe fn glSecondaryColorP3ui(&self, _type: ColorPointerType, _color: GLuint) {
        (self.entry().glSecondaryColorP3ui)(_type, _color)
    }
    unsafe fn glDrawArraysIndirect(
        &self,
        _mode: PrimitiveType,
        _indirect: *const std::os::raw::c_void,
    ) {
        (self.entry().glDrawArraysIndirect)(_mode, _indirect)
    }
    unsafe fn glEnableVertexAttribArray(&self, _index: GLuint) {
        (self.entry().glEnableVertexAttribArray)(_index)
    }
    unsafe fn glUniform3iv(&self, _location: GLint, _count: GLsizei, _value: *const GLint) {
        (self.entry().glUniform3iv)(_location, _count, _value)
    }
    unsafe fn glVertexAttribP3ui(
        &self,
        _index: GLuint,
        _type: VertexAttribPointerType,
        _normalized: GLboolean,
        _value: GLuint,
    ) {
        (self.entry().glVertexAttribP3ui)(_index, _type, _normalized, _value)
    }
    unsafe fn glVertexAttrib2s(&self, _index: GLuint, _x: GLshort, _y: GLshort) {
        (self.entry().glVertexAttrib2s)(_index, _x, _y)
    }
    unsafe fn glVertexAttrib4sv(&self, _index: GLuint, _v: *const GLshort) {
        (self.entry().glVertexAttrib4sv)(_index, _v)
    }
    unsafe fn glBindProgramPipeline(&self, _pipeline: GLuint) {
        (self.entry().glBindProgramPipeline)(_pipeline)
    }
    unsafe fn glProgramUniform3i(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLint,
        _v1: GLint,
        _v2: GLint,
    ) {
        (self.entry().glProgramUniform3i)(_program, _location, _v0, _v1, _v2)
    }
    unsafe fn glUniformMatrix3x2fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        (self.entry().glUniformMatrix3x2fv)(_location, _count, _transpose, _value)
    }
    unsafe fn glGetActiveUniformName(
        &self,
        _program: GLuint,
        _uniformIndex: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _uniformName: *mut GLchar,
    ) {
        (self.entry().glGetActiveUniformName)(
            _program,
            _uniformIndex,
            _bufSize,
            _length,
            _uniformName,
        )
    }
    unsafe fn glDeleteQueries(&self, _n: GLsizei, _ids: *const GLuint) {
        (self.entry().glDeleteQueries)(_n, _ids)
    }
    unsafe fn glFenceSync(&self, _condition: SyncCondition, _flags: SyncBehaviorFlags) -> GLsync {
        (self.entry().glFenceSync)(_condition, _flags)
    }
    unsafe fn glSamplerParameteri(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterI,
        _param: GLint,
    ) {
        (self.entry().glSamplerParameteri)(_sampler, _pname, _param)
    }
    unsafe fn glGetActiveSubroutineUniformName(
        &self,
        _program: GLuint,
        _shadertype: ShaderType,
        _index: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _name: *mut GLchar,
    ) {
        (self.entry().glGetActiveSubroutineUniformName)(
            _program,
            _shadertype,
            _index,
            _bufSize,
            _length,
            _name,
        )
    }
    unsafe fn glVertexAttribI4bv(&self, _index: GLuint, _v: *const GLbyte) {
        (self.entry().glVertexAttribI4bv)(_index, _v)
    }
    unsafe fn glDepthRangeArrayv(&self, _first: GLuint, _count: GLsizei, _v: *const GLdouble) {
        (self.entry().glDepthRangeArrayv)(_first, _count, _v)
    }
    unsafe fn glGetShaderPrecisionFormat(
        &self,
        _shadertype: ShaderType,
        _precisiontype: PrecisionType,
        _range: *mut GLint,
        _precision: *mut GLint,
    ) {
        (self.entry().glGetShaderPrecisionFormat)(_shadertype, _precisiontype, _range, _precision)
    }
    unsafe fn glGenerateMipmap(&self, _target: TextureTarget) {
        (self.entry().glGenerateMipmap)(_target)
    }
    unsafe fn glPrimitiveRestartIndex(&self, _index: GLuint) {
        (self.entry().glPrimitiveRestartIndex)(_index)
    }
    unsafe fn glDrawTransformFeedback(&self, _mode: PrimitiveType, _id: GLuint) {
        (self.entry().glDrawTransformFeedback)(_mode, _id)
    }
    unsafe fn glGetRenderbufferParameteriv(
        &self,
        _target: RenderbufferTarget,
        _pname: RenderbufferParameterName,
        _params: *mut GLint,
    ) {
        (self.entry().glGetRenderbufferParameteriv)(_target, _pname, _params)
    }
    unsafe fn glGetSubroutineIndex(
        &self,
        _program: GLuint,
        _shadertype: ShaderType,
        _name: *const GLchar,
    ) -> GLuint {
        (self.entry().glGetSubroutineIndex)(_program, _shadertype, _name)
    }
    unsafe fn glBindVertexBuffers(
        &self,
        _first: GLuint,
        _count: GLsizei,
        _buffers: *const GLuint,
        _offsets: *const GLintptr,
        _strides: *const GLsizei,
    ) {
        (self.entry().glBindVertexBuffers)(_first, _count, _buffers, _offsets, _strides)
    }
    unsafe fn glVertexAttribI3uiv(&self, _index: GLuint, _v: *const GLuint) {
        (self.entry().glVertexAttribI3uiv)(_index, _v)
    }
    unsafe fn glResumeTransformFeedback(&self) {
        (self.entry().glResumeTransformFeedback)()
    }
    unsafe fn glFlushMappedBufferRange(
        &self,
        _target: BufferTargetARB,
        _offset: GLintptr,
        _length: GLsizeiptr,
    ) {
        (self.entry().glFlushMappedBufferRange)(_target, _offset, _length)
    }
    unsafe fn glProgramUniformMatrix3x4fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        (self.entry().glProgramUniformMatrix3x4fv)(_program, _location, _count, _transpose, _value)
    }
    unsafe fn glVertexAttribI2ui(&self, _index: GLuint, _x: GLuint, _y: GLuint) {
        (self.entry().glVertexAttribI2ui)(_index, _x, _y)
    }
    unsafe fn glScissor(&self, _x: GLint, _y: GLint, _width: GLsizei, _height: GLsizei) {
        (self.entry().glScissor)(_x, _y, _width, _height)
    }
    unsafe fn glPolygonMode(&self, _face: MaterialFace, _mode: PolygonMode) {
        (self.entry().glPolygonMode)(_face, _mode)
    }
    unsafe fn glGetShaderSource(
        &self,
        _shader: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _source: *mut GLchar,
    ) {
        (self.entry().glGetShaderSource)(_shader, _bufSize, _length, _source)
    }
    unsafe fn glVertexAttribI2i(&self, _index: GLuint, _x: GLint, _y: GLint) {
        (self.entry().glVertexAttribI2i)(_index, _x, _y)
    }
    unsafe fn glGetTexParameterIiv(
        &self,
        _target: TextureTarget,
        _pname: GetTextureParameter,
        _params: *mut GLint,
    ) {
        (self.entry().glGetTexParameterIiv)(_target, _pname, _params)
    }
    unsafe fn glClearBufferfv(&self, _buffer: Buffer, _drawbuffer: GLint, _value: *const GLfloat) {
        (self.entry().glClearBufferfv)(_buffer, _drawbuffer, _value)
    }
    unsafe fn glVertexAttribP1ui(
        &self,
        _index: GLuint,
        _type: VertexAttribPointerType,
        _normalized: GLboolean,
        _value: GLuint,
    ) {
        (self.entry().glVertexAttribP1ui)(_index, _type, _normalized, _value)
    }
    unsafe fn glVertexAttribP1uiv(
        &self,
        _index: GLuint,
        _type: VertexAttribPointerType,
        _normalized: GLboolean,
        _value: *const GLuint,
    ) {
        (self.entry().glVertexAttribP1uiv)(_index, _type, _normalized, _value)
    }
    unsafe fn glVertexAttribI4i(&self, _index: GLuint, _x: GLint, _y: GLint, _z: GLint, _w: GLint) {
        (self.entry().glVertexAttribI4i)(_index, _x, _y, _z, _w)
    }
    unsafe fn glEndTransformFeedback(&self) {
        (self.entry().glEndTransformFeedback)()
    }
    unsafe fn glClearTexImage(
        &self,
        _texture: GLuint,
        _level: GLint,
        _format: PixelFormat,
        _type: PixelType,
        _data: *const std::os::raw::c_void,
    ) {
        (self.entry().glClearTexImage)(_texture, _level, _format, _type, _data)
    }
    unsafe fn glEnable(&self, _cap: EnableCap) {
        (self.entry().glEnable)(_cap)
    }
    unsafe fn glVertexAttrib4uiv(&self, _index: GLuint, _v: *const GLuint) {
        (self.entry().glVertexAttrib4uiv)(_index, _v)
    }
    unsafe fn glVertexAttribBinding(&self, _attribindex: GLuint, _bindingindex: GLuint) {
        (self.entry().glVertexAttribBinding)(_attribindex, _bindingindex)
    }
    unsafe fn glVertexAttribL3dv(&self, _index: GLuint, _v: *const GLdouble) {
        (self.entry().glVertexAttribL3dv)(_index, _v)
    }
    unsafe fn glCompressedTexImage2D(
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
        (self.entry().glCompressedTexImage2D)(
            _target,
            _level,
            _internalformat,
            _width,
            _height,
            _border,
            _imageSize,
            _data,
        )
    }
    unsafe fn glVertexP4ui(&self, _type: VertexPointerType, _value: GLuint) {
        (self.entry().glVertexP4ui)(_type, _value)
    }
    unsafe fn glInvalidateBufferData(&self, _buffer: GLuint) {
        (self.entry().glInvalidateBufferData)(_buffer)
    }
    unsafe fn glGetProgramResourceLocation(
        &self,
        _program: GLuint,
        _programInterface: ProgramInterface,
        _name: *const GLchar,
    ) -> GLint {
        (self.entry().glGetProgramResourceLocation)(_program, _programInterface, _name)
    }
    unsafe fn glCompressedTexSubImage1D(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _xoffset: GLint,
        _width: GLsizei,
        _format: InternalFormat,
        _imageSize: GLsizei,
        _data: *const std::os::raw::c_void,
    ) {
        (self.entry().glCompressedTexSubImage1D)(
            _target, _level, _xoffset, _width, _format, _imageSize, _data,
        )
    }
    unsafe fn glVertexAttribP2uiv(
        &self,
        _index: GLuint,
        _type: VertexAttribPointerType,
        _normalized: GLboolean,
        _value: *const GLuint,
    ) {
        (self.entry().glVertexAttribP2uiv)(_index, _type, _normalized, _value)
    }
    unsafe fn glProgramUniformMatrix2dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        (self.entry().glProgramUniformMatrix2dv)(_program, _location, _count, _transpose, _value)
    }
    unsafe fn glGetProgramResourceLocationIndex(
        &self,
        _program: GLuint,
        _programInterface: ProgramInterface,
        _name: *const GLchar,
    ) -> GLint {
        (self.entry().glGetProgramResourceLocationIndex)(_program, _programInterface, _name)
    }
    unsafe fn glProgramUniform3ui(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLuint,
        _v1: GLuint,
        _v2: GLuint,
    ) {
        (self.entry().glProgramUniform3ui)(_program, _location, _v0, _v1, _v2)
    }
    unsafe fn glSamplerParameterfv(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterF,
        _param: *const GLfloat,
    ) {
        (self.entry().glSamplerParameterfv)(_sampler, _pname, _param)
    }
    unsafe fn glProgramUniformMatrix3x4dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        (self.entry().glProgramUniformMatrix3x4dv)(_program, _location, _count, _transpose, _value)
    }
    unsafe fn glCreateProgram(&self) -> GLuint {
        (self.entry().glCreateProgram)()
    }
    unsafe fn glTexCoordP4uiv(&self, _type: TexCoordPointerType, _coords: *const GLuint) {
        (self.entry().glTexCoordP4uiv)(_type, _coords)
    }
    unsafe fn glVertexAttrib1dv(&self, _index: GLuint, _v: *const GLdouble) {
        (self.entry().glVertexAttrib1dv)(_index, _v)
    }
    unsafe fn glClearDepth(&self, _depth: GLdouble) {
        (self.entry().glClearDepth)(_depth)
    }
    unsafe fn glUniformMatrix2dv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        (self.entry().glUniformMatrix2dv)(_location, _count, _transpose, _value)
    }
    unsafe fn glDeleteFramebuffers(&self, _n: GLsizei, _framebuffers: *const GLuint) {
        (self.entry().glDeleteFramebuffers)(_n, _framebuffers)
    }
    unsafe fn glPolygonOffset(&self, _factor: GLfloat, _units: GLfloat) {
        (self.entry().glPolygonOffset)(_factor, _units)
    }
    unsafe fn glValidateProgramPipeline(&self, _pipeline: GLuint) {
        (self.entry().glValidateProgramPipeline)(_pipeline)
    }
    unsafe fn glUniform4f(
        &self,
        _location: GLint,
        _v0: GLfloat,
        _v1: GLfloat,
        _v2: GLfloat,
        _v3: GLfloat,
    ) {
        (self.entry().glUniform4f)(_location, _v0, _v1, _v2, _v3)
    }
    unsafe fn glFramebufferTextureLayer(
        &self,
        _target: FramebufferTarget,
        _attachment: FramebufferAttachment,
        _texture: GLuint,
        _level: GLint,
        _layer: GLint,
    ) {
        (self.entry().glFramebufferTextureLayer)(_target, _attachment, _texture, _level, _layer)
    }
    unsafe fn glCompileShader(&self, _shader: GLuint) {
        (self.entry().glCompileShader)(_shader)
    }
    unsafe fn glPopDebugGroup(&self) {
        (self.entry().glPopDebugGroup)()
    }
    unsafe fn glGetActiveUniformsiv(
        &self,
        _program: GLuint,
        _uniformCount: GLsizei,
        _uniformIndices: *const GLuint,
        _pname: UniformPName,
        _params: *mut GLint,
    ) {
        (self.entry().glGetActiveUniformsiv)(
            _program,
            _uniformCount,
            _uniformIndices,
            _pname,
            _params,
        )
    }
    unsafe fn glDrawArraysInstancedBaseInstance(
        &self,
        _mode: PrimitiveType,
        _first: GLint,
        _count: GLsizei,
        _instancecount: GLsizei,
        _baseinstance: GLuint,
    ) {
        (self.entry().glDrawArraysInstancedBaseInstance)(
            _mode,
            _first,
            _count,
            _instancecount,
            _baseinstance,
        )
    }
    unsafe fn glSampleCoverage(&self, _value: GLfloat, _invert: GLboolean) {
        (self.entry().glSampleCoverage)(_value, _invert)
    }
    unsafe fn glBufferStorage(
        &self,
        _target: BufferStorageTarget,
        _size: GLsizeiptr,
        _data: *const std::os::raw::c_void,
        _flags: BufferStorageMask,
    ) {
        (self.entry().glBufferStorage)(_target, _size, _data, _flags)
    }
    unsafe fn glDebugMessageInsert(
        &self,
        _source: DebugSource,
        _type: DebugType,
        _id: GLuint,
        _severity: DebugSeverity,
        _length: GLsizei,
        _buf: *const GLchar,
    ) {
        (self.entry().glDebugMessageInsert)(_source, _type, _id, _severity, _length, _buf)
    }
    unsafe fn glTexStorage2D(
        &self,
        _target: TextureTarget,
        _levels: GLsizei,
        _internalformat: SizedInternalFormat,
        _width: GLsizei,
        _height: GLsizei,
    ) {
        (self.entry().glTexStorage2D)(_target, _levels, _internalformat, _width, _height)
    }
    unsafe fn glTexCoordP1uiv(&self, _type: TexCoordPointerType, _coords: *const GLuint) {
        (self.entry().glTexCoordP1uiv)(_type, _coords)
    }
    unsafe fn glProgramUniform3uiv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLuint,
    ) {
        (self.entry().glProgramUniform3uiv)(_program, _location, _count, _value)
    }
    unsafe fn glUniformMatrix2x3fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        (self.entry().glUniformMatrix2x3fv)(_location, _count, _transpose, _value)
    }
    unsafe fn glTexCoordP2uiv(&self, _type: TexCoordPointerType, _coords: *const GLuint) {
        (self.entry().glTexCoordP2uiv)(_type, _coords)
    }
    unsafe fn glUniformMatrix3x4dv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        (self.entry().glUniformMatrix3x4dv)(_location, _count, _transpose, _value)
    }
    unsafe fn glBindFragDataLocationIndexed(
        &self,
        _program: GLuint,
        _colorNumber: GLuint,
        _index: GLuint,
        _name: *const GLchar,
    ) {
        (self.entry().glBindFragDataLocationIndexed)(_program, _colorNumber, _index, _name)
    }
    unsafe fn glClearDepthf(&self, _d: GLfloat) {
        (self.entry().glClearDepthf)(_d)
    }
    unsafe fn glGetDebugMessageLog(
        &self,
        _count: GLuint,
        _bufSize: GLsizei,
        _sources: *mut DebugSource,
        _types: *mut DebugType,
        _ids: *mut GLuint,
        _severities: *mut DebugSeverity,
        _lengths: *mut GLsizei,
        _messageLog: *mut GLchar,
    ) -> GLuint {
        (self.entry().glGetDebugMessageLog)(
            _count,
            _bufSize,
            _sources,
            _types,
            _ids,
            _severities,
            _lengths,
            _messageLog,
        )
    }
    unsafe fn glProgramUniformMatrix4x2dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        (self.entry().glProgramUniformMatrix4x2dv)(_program, _location, _count, _transpose, _value)
    }
    unsafe fn glGetFragDataLocation(&self, _program: GLuint, _name: *const GLchar) -> GLint {
        (self.entry().glGetFragDataLocation)(_program, _name)
    }
    unsafe fn glUniformBlockBinding(
        &self,
        _program: GLuint,
        _uniformBlockIndex: GLuint,
        _uniformBlockBinding: GLuint,
    ) {
        (self.entry().glUniformBlockBinding)(_program, _uniformBlockIndex, _uniformBlockBinding)
    }
    unsafe fn glUniformMatrix4x3fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        (self.entry().glUniformMatrix4x3fv)(_location, _count, _transpose, _value)
    }
    unsafe fn glVertexAttrib4ubv(&self, _index: GLuint, _v: *const GLubyte) {
        (self.entry().glVertexAttrib4ubv)(_index, _v)
    }
    unsafe fn glColorMaski(
        &self,
        _index: GLuint,
        _r: GLboolean,
        _g: GLboolean,
        _b: GLboolean,
        _a: GLboolean,
    ) {
        (self.entry().glColorMaski)(_index, _r, _g, _b, _a)
    }
    unsafe fn glVertexAttribI4ui(
        &self,
        _index: GLuint,
        _x: GLuint,
        _y: GLuint,
        _z: GLuint,
        _w: GLuint,
    ) {
        (self.entry().glVertexAttribI4ui)(_index, _x, _y, _z, _w)
    }
    unsafe fn glRenderbufferStorageMultisample(
        &self,
        _target: RenderbufferTarget,
        _samples: GLsizei,
        _internalformat: InternalFormat,
        _width: GLsizei,
        _height: GLsizei,
    ) {
        (self.entry().glRenderbufferStorageMultisample)(
            _target,
            _samples,
            _internalformat,
            _width,
            _height,
        )
    }
    unsafe fn glHint(&self, _target: HintTarget, _mode: HintMode) {
        (self.entry().glHint)(_target, _mode)
    }
    unsafe fn glVertexAttrib2dv(&self, _index: GLuint, _v: *const GLdouble) {
        (self.entry().glVertexAttrib2dv)(_index, _v)
    }
    unsafe fn glVertexAttribIPointer(
        &self,
        _index: GLuint,
        _size: GLint,
        _type: VertexAttribIType,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glVertexAttribIPointer)(_index, _size, _type, _stride, _pointer)
    }
    unsafe fn glGetTransformFeedbackVarying(
        &self,
        _program: GLuint,
        _index: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _size: *mut GLsizei,
        _type: *mut AttributeType,
        _name: *mut GLchar,
    ) {
        (self.entry().glGetTransformFeedbackVarying)(
            _program, _index, _bufSize, _length, _size, _type, _name,
        )
    }
    unsafe fn glProgramUniformMatrix4x3fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        (self.entry().glProgramUniformMatrix4x3fv)(_program, _location, _count, _transpose, _value)
    }
    unsafe fn glCompressedTexSubImage2D(
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
        (self.entry().glCompressedTexSubImage2D)(
            _target, _level, _xoffset, _yoffset, _width, _height, _format, _imageSize, _data,
        )
    }
    unsafe fn glGetIntegeri_v(&self, _target: GetPName, _index: GLuint, _data: *mut GLint) {
        (self.entry().glGetIntegeri_v)(_target, _index, _data)
    }
    unsafe fn glViewportIndexedfv(&self, _index: GLuint, _v: *const GLfloat) {
        (self.entry().glViewportIndexedfv)(_index, _v)
    }
    unsafe fn glColorP3uiv(&self, _type: ColorPointerType, _color: *const GLuint) {
        (self.entry().glColorP3uiv)(_type, _color)
    }
    unsafe fn glGetPointerv(
        &self,
        _pname: GetPointervPName,
        _params: *mut *mut std::os::raw::c_void,
    ) {
        (self.entry().glGetPointerv)(_pname, _params)
    }
    unsafe fn glGetProgramInterfaceiv(
        &self,
        _program: GLuint,
        _programInterface: ProgramInterface,
        _pname: ProgramInterfacePName,
        _params: *mut GLint,
    ) {
        (self.entry().glGetProgramInterfaceiv)(_program, _programInterface, _pname, _params)
    }
    unsafe fn glDrawTransformFeedbackStreamInstanced(
        &self,
        _mode: PrimitiveType,
        _id: GLuint,
        _stream: GLuint,
        _instancecount: GLsizei,
    ) {
        (self.entry().glDrawTransformFeedbackStreamInstanced)(_mode, _id, _stream, _instancecount)
    }
    unsafe fn glDrawElementsBaseVertex(
        &self,
        _mode: PrimitiveType,
        _count: GLsizei,
        _type: DrawElementsType,
        _indices: *const std::os::raw::c_void,
        _basevertex: GLint,
    ) {
        (self.entry().glDrawElementsBaseVertex)(_mode, _count, _type, _indices, _basevertex)
    }
    unsafe fn glPauseTransformFeedback(&self) {
        (self.entry().glPauseTransformFeedback)()
    }
    unsafe fn glBindFramebuffer(&self, _target: FramebufferTarget, _framebuffer: GLuint) {
        (self.entry().glBindFramebuffer)(_target, _framebuffer)
    }
    unsafe fn glReleaseShaderCompiler(&self) {
        (self.entry().glReleaseShaderCompiler)()
    }
    unsafe fn glVertexAttrib4Nusv(&self, _index: GLuint, _v: *const GLushort) {
        (self.entry().glVertexAttrib4Nusv)(_index, _v)
    }
    unsafe fn glVertexAttribI2uiv(&self, _index: GLuint, _v: *const GLuint) {
        (self.entry().glVertexAttribI2uiv)(_index, _v)
    }
    unsafe fn glProgramUniform4d(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLdouble,
        _v1: GLdouble,
        _v2: GLdouble,
        _v3: GLdouble,
    ) {
        (self.entry().glProgramUniform4d)(_program, _location, _v0, _v1, _v2, _v3)
    }
    unsafe fn glProgramUniform4ui(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLuint,
        _v1: GLuint,
        _v2: GLuint,
        _v3: GLuint,
    ) {
        (self.entry().glProgramUniform4ui)(_program, _location, _v0, _v1, _v2, _v3)
    }
    unsafe fn glDrawBuffers(&self, _n: GLsizei, _bufs: *const DrawBufferMode) {
        (self.entry().glDrawBuffers)(_n, _bufs)
    }
    unsafe fn glGetActiveAtomicCounterBufferiv(
        &self,
        _program: GLuint,
        _bufferIndex: GLuint,
        _pname: AtomicCounterBufferPName,
        _params: *mut GLint,
    ) {
        (self.entry().glGetActiveAtomicCounterBufferiv)(_program, _bufferIndex, _pname, _params)
    }
    unsafe fn glMultiDrawElementsBaseVertex(
        &self,
        _mode: PrimitiveType,
        _count: *const GLsizei,
        _type: DrawElementsType,
        _indices: *const *const std::os::raw::c_void,
        _drawcount: GLsizei,
        _basevertex: *const GLint,
    ) {
        (self.entry().glMultiDrawElementsBaseVertex)(
            _mode,
            _count,
            _type,
            _indices,
            _drawcount,
            _basevertex,
        )
    }
    unsafe fn glColorP4uiv(&self, _type: ColorPointerType, _color: *const GLuint) {
        (self.entry().glColorP4uiv)(_type, _color)
    }
    unsafe fn glPointSize(&self, _size: GLfloat) {
        (self.entry().glPointSize)(_size)
    }
    unsafe fn glUniform4fv(&self, _location: GLint, _count: GLsizei, _value: *const GLfloat) {
        (self.entry().glUniform4fv)(_location, _count, _value)
    }
    unsafe fn glVertexAttrib4d(
        &self,
        _index: GLuint,
        _x: GLdouble,
        _y: GLdouble,
        _z: GLdouble,
        _w: GLdouble,
    ) {
        (self.entry().glVertexAttrib4d)(_index, _x, _y, _z, _w)
    }
    unsafe fn glProgramUniform2iv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLint,
    ) {
        (self.entry().glProgramUniform2iv)(_program, _location, _count, _value)
    }
    unsafe fn glVertexAttribP3uiv(
        &self,
        _index: GLuint,
        _type: VertexAttribPointerType,
        _normalized: GLboolean,
        _value: *const GLuint,
    ) {
        (self.entry().glVertexAttribP3uiv)(_index, _type, _normalized, _value)
    }
    unsafe fn glGetVertexAttribPointerv(
        &self,
        _index: GLuint,
        _pname: VertexAttribPointerPropertyARB,
        _pointer: *mut *mut std::os::raw::c_void,
    ) {
        (self.entry().glGetVertexAttribPointerv)(_index, _pname, _pointer)
    }
    unsafe fn glProgramUniform2dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLdouble,
    ) {
        (self.entry().glProgramUniform2dv)(_program, _location, _count, _value)
    }
    unsafe fn glDeleteBuffers(&self, _n: GLsizei, _buffers: *const GLuint) {
        (self.entry().glDeleteBuffers)(_n, _buffers)
    }
    unsafe fn glProgramUniformMatrix4x2fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        (self.entry().glProgramUniformMatrix4x2fv)(_program, _location, _count, _transpose, _value)
    }
    unsafe fn glBindSamplers(&self, _first: GLuint, _count: GLsizei, _samplers: *const GLuint) {
        (self.entry().glBindSamplers)(_first, _count, _samplers)
    }
    unsafe fn glGetUniformSubroutineuiv(
        &self,
        _shadertype: ShaderType,
        _location: GLint,
        _params: *mut GLuint,
    ) {
        (self.entry().glGetUniformSubroutineuiv)(_shadertype, _location, _params)
    }
    unsafe fn glVertexAttrib2d(&self, _index: GLuint, _x: GLdouble, _y: GLdouble) {
        (self.entry().glVertexAttrib2d)(_index, _x, _y)
    }
    unsafe fn glGetQueryObjectuiv(
        &self,
        _id: GLuint,
        _pname: QueryObjectParameterName,
        _params: *mut GLuint,
    ) {
        (self.entry().glGetQueryObjectuiv)(_id, _pname, _params)
    }
    unsafe fn glUniform4ui(
        &self,
        _location: GLint,
        _v0: GLuint,
        _v1: GLuint,
        _v2: GLuint,
        _v3: GLuint,
    ) {
        (self.entry().glUniform4ui)(_location, _v0, _v1, _v2, _v3)
    }
    unsafe fn glBindAttribLocation(&self, _program: GLuint, _index: GLuint, _name: *const GLchar) {
        (self.entry().glBindAttribLocation)(_program, _index, _name)
    }
    unsafe fn glProgramUniform2fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLfloat,
    ) {
        (self.entry().glProgramUniform2fv)(_program, _location, _count, _value)
    }
    unsafe fn glVertexAttribI1uiv(&self, _index: GLuint, _v: *const GLuint) {
        (self.entry().glVertexAttribI1uiv)(_index, _v)
    }
    unsafe fn glGetTexLevelParameterfv(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _pname: GetTextureParameter,
        _params: *mut GLfloat,
    ) {
        (self.entry().glGetTexLevelParameterfv)(_target, _level, _pname, _params)
    }
    unsafe fn glClearBufferSubData(
        &self,
        _target: BufferTargetARB,
        _internalformat: SizedInternalFormat,
        _offset: GLintptr,
        _size: GLsizeiptr,
        _format: PixelFormat,
        _type: PixelType,
        _data: *const std::os::raw::c_void,
    ) {
        (self.entry().glClearBufferSubData)(
            _target,
            _internalformat,
            _offset,
            _size,
            _format,
            _type,
            _data,
        )
    }
    unsafe fn glDrawTransformFeedbackStream(
        &self,
        _mode: PrimitiveType,
        _id: GLuint,
        _stream: GLuint,
    ) {
        (self.entry().glDrawTransformFeedbackStream)(_mode, _id, _stream)
    }
    unsafe fn glReadPixels(
        &self,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _format: PixelFormat,
        _type: PixelType,
        _pixels: *mut std::os::raw::c_void,
    ) {
        (self.entry().glReadPixels)(_x, _y, _width, _height, _format, _type, _pixels)
    }
    unsafe fn glFinish(&self) {
        (self.entry().glFinish)()
    }
}
