use crate::gl::feature::EntryGLFn;
use crate::types::*;
use std::mem;
pub trait GL43 {
    unsafe fn entry(&self) -> &EntryGLFn;
    unsafe fn glActiveShaderProgram(&self, _pipeline: GLuint, _program: GLuint) {
        (self.entry().glActiveShaderProgram)(_pipeline, _program)
    }
    unsafe fn glActiveTexture(&self, _texture: GLenum) {
        (self.entry().glActiveTexture)(_texture)
    }
    unsafe fn glAttachShader(&self, _program: GLuint, _shader: GLuint) {
        (self.entry().glAttachShader)(_program, _shader)
    }
    unsafe fn glBeginConditionalRender(&self, _id: GLuint, _mode: GLenum) {
        (self.entry().glBeginConditionalRender)(_id, _mode)
    }
    unsafe fn glBeginQuery(&self, _target: GLenum, _id: GLuint) {
        (self.entry().glBeginQuery)(_target, _id)
    }
    unsafe fn glBeginQueryIndexed(&self, _target: GLenum, _index: GLuint, _id: GLuint) {
        (self.entry().glBeginQueryIndexed)(_target, _index, _id)
    }
    unsafe fn glBeginTransformFeedback(&self, _primitiveMode: GLenum) {
        (self.entry().glBeginTransformFeedback)(_primitiveMode)
    }
    unsafe fn glBindAttribLocation(&self, _program: GLuint, _index: GLuint, _name: *const GLchar) {
        (self.entry().glBindAttribLocation)(_program, _index, _name)
    }
    unsafe fn glBindBuffer(&self, _target: GLenum, _buffer: GLuint) {
        (self.entry().glBindBuffer)(_target, _buffer)
    }
    unsafe fn glBindBufferBase(&self, _target: GLenum, _index: GLuint, _buffer: GLuint) {
        (self.entry().glBindBufferBase)(_target, _index, _buffer)
    }
    unsafe fn glBindBufferRange(
        &self,
        _target: GLenum,
        _index: GLuint,
        _buffer: GLuint,
        _offset: GLintptr,
        _size: GLsizeiptr,
    ) {
        (self.entry().glBindBufferRange)(_target, _index, _buffer, _offset, _size)
    }
    unsafe fn glBindFragDataLocation(
        &self,
        _program: GLuint,
        _color: GLuint,
        _name: *const GLchar,
    ) {
        (self.entry().glBindFragDataLocation)(_program, _color, _name)
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
    unsafe fn glBindFramebuffer(&self, _target: GLenum, _framebuffer: GLuint) {
        (self.entry().glBindFramebuffer)(_target, _framebuffer)
    }
    unsafe fn glBindImageTexture(
        &self,
        _unit: GLuint,
        _texture: GLuint,
        _level: GLint,
        _layered: GLboolean,
        _layer: GLint,
        _access: GLenum,
        _format: GLenum,
    ) {
        (self.entry().glBindImageTexture)(
            _unit, _texture, _level, _layered, _layer, _access, _format,
        )
    }
    unsafe fn glBindProgramPipeline(&self, _pipeline: GLuint) {
        (self.entry().glBindProgramPipeline)(_pipeline)
    }
    unsafe fn glBindRenderbuffer(&self, _target: GLenum, _renderbuffer: GLuint) {
        (self.entry().glBindRenderbuffer)(_target, _renderbuffer)
    }
    unsafe fn glBindSampler(&self, _unit: GLuint, _sampler: GLuint) {
        (self.entry().glBindSampler)(_unit, _sampler)
    }
    unsafe fn glBindTexture(&self, _target: GLenum, _texture: GLuint) {
        (self.entry().glBindTexture)(_target, _texture)
    }
    unsafe fn glBindTransformFeedback(&self, _target: GLenum, _id: GLuint) {
        (self.entry().glBindTransformFeedback)(_target, _id)
    }
    unsafe fn glBindVertexArray(&self, _array: GLuint) {
        (self.entry().glBindVertexArray)(_array)
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
    unsafe fn glBlendColor(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat, _alpha: GLfloat) {
        (self.entry().glBlendColor)(_red, _green, _blue, _alpha)
    }
    unsafe fn glBlendEquation(&self, _mode: GLenum) {
        (self.entry().glBlendEquation)(_mode)
    }
    unsafe fn glBlendEquationSeparate(&self, _modeRGB: GLenum, _modeAlpha: GLenum) {
        (self.entry().glBlendEquationSeparate)(_modeRGB, _modeAlpha)
    }
    unsafe fn glBlendEquationSeparatei(&self, _buf: GLuint, _modeRGB: GLenum, _modeAlpha: GLenum) {
        (self.entry().glBlendEquationSeparatei)(_buf, _modeRGB, _modeAlpha)
    }
    unsafe fn glBlendEquationi(&self, _buf: GLuint, _mode: GLenum) {
        (self.entry().glBlendEquationi)(_buf, _mode)
    }
    unsafe fn glBlendFunc(&self, _sfactor: GLenum, _dfactor: GLenum) {
        (self.entry().glBlendFunc)(_sfactor, _dfactor)
    }
    unsafe fn glBlendFuncSeparate(
        &self,
        _sfactorRGB: GLenum,
        _dfactorRGB: GLenum,
        _sfactorAlpha: GLenum,
        _dfactorAlpha: GLenum,
    ) {
        (self.entry().glBlendFuncSeparate)(_sfactorRGB, _dfactorRGB, _sfactorAlpha, _dfactorAlpha)
    }
    unsafe fn glBlendFuncSeparatei(
        &self,
        _buf: GLuint,
        _srcRGB: GLenum,
        _dstRGB: GLenum,
        _srcAlpha: GLenum,
        _dstAlpha: GLenum,
    ) {
        (self.entry().glBlendFuncSeparatei)(_buf, _srcRGB, _dstRGB, _srcAlpha, _dstAlpha)
    }
    unsafe fn glBlendFunci(&self, _buf: GLuint, _src: GLenum, _dst: GLenum) {
        (self.entry().glBlendFunci)(_buf, _src, _dst)
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
        _mask: GLbitfield,
        _filter: GLenum,
    ) {
        (self.entry().glBlitFramebuffer)(
            _srcX0, _srcY0, _srcX1, _srcY1, _dstX0, _dstY0, _dstX1, _dstY1, _mask, _filter,
        )
    }
    unsafe fn glBufferData(
        &self,
        _target: GLenum,
        _size: GLsizeiptr,
        _data: *const std::os::raw::c_void,
        _usage: GLenum,
    ) {
        (self.entry().glBufferData)(_target, _size, _data, _usage)
    }
    unsafe fn glBufferSubData(
        &self,
        _target: GLenum,
        _offset: GLintptr,
        _size: GLsizeiptr,
        _data: *const std::os::raw::c_void,
    ) {
        (self.entry().glBufferSubData)(_target, _offset, _size, _data)
    }
    unsafe fn glCheckFramebufferStatus(&self, _target: GLenum) -> GLenum {
        (self.entry().glCheckFramebufferStatus)(_target)
    }
    unsafe fn glClampColor(&self, _target: GLenum, _clamp: GLenum) {
        (self.entry().glClampColor)(_target, _clamp)
    }
    unsafe fn glClear(&self, _mask: GLbitfield) {
        (self.entry().glClear)(_mask)
    }
    unsafe fn glClearBufferData(
        &self,
        _target: GLenum,
        _internalformat: GLenum,
        _format: GLenum,
        _type: GLenum,
        _data: *const std::os::raw::c_void,
    ) {
        (self.entry().glClearBufferData)(_target, _internalformat, _format, _type, _data)
    }
    unsafe fn glClearBufferSubData(
        &self,
        _target: GLenum,
        _internalformat: GLenum,
        _offset: GLintptr,
        _size: GLsizeiptr,
        _format: GLenum,
        _type: GLenum,
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
    unsafe fn glClearBufferfi(
        &self,
        _buffer: GLenum,
        _drawbuffer: GLint,
        _depth: GLfloat,
        _stencil: GLint,
    ) {
        (self.entry().glClearBufferfi)(_buffer, _drawbuffer, _depth, _stencil)
    }
    unsafe fn glClearBufferfv(&self, _buffer: GLenum, _drawbuffer: GLint, _value: *const GLfloat) {
        (self.entry().glClearBufferfv)(_buffer, _drawbuffer, _value)
    }
    unsafe fn glClearBufferiv(&self, _buffer: GLenum, _drawbuffer: GLint, _value: *const GLint) {
        (self.entry().glClearBufferiv)(_buffer, _drawbuffer, _value)
    }
    unsafe fn glClearBufferuiv(&self, _buffer: GLenum, _drawbuffer: GLint, _value: *const GLuint) {
        (self.entry().glClearBufferuiv)(_buffer, _drawbuffer, _value)
    }
    unsafe fn glClearColor(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat, _alpha: GLfloat) {
        (self.entry().glClearColor)(_red, _green, _blue, _alpha)
    }
    unsafe fn glClearDepth(&self, _depth: GLdouble) {
        (self.entry().glClearDepth)(_depth)
    }
    unsafe fn glClearDepthf(&self, _d: GLfloat) {
        (self.entry().glClearDepthf)(_d)
    }
    unsafe fn glClearStencil(&self, _s: GLint) {
        (self.entry().glClearStencil)(_s)
    }
    unsafe fn glClientWaitSync(
        &self,
        _sync: GLsync,
        _flags: GLbitfield,
        _timeout: GLuint64,
    ) -> GLenum {
        (self.entry().glClientWaitSync)(_sync, _flags, _timeout)
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
    unsafe fn glColorP3ui(&self, _type: GLenum, _color: GLuint) {
        (self.entry().glColorP3ui)(_type, _color)
    }
    unsafe fn glColorP3uiv(&self, _type: GLenum, _color: *const GLuint) {
        (self.entry().glColorP3uiv)(_type, _color)
    }
    unsafe fn glColorP4ui(&self, _type: GLenum, _color: GLuint) {
        (self.entry().glColorP4ui)(_type, _color)
    }
    unsafe fn glColorP4uiv(&self, _type: GLenum, _color: *const GLuint) {
        (self.entry().glColorP4uiv)(_type, _color)
    }
    unsafe fn glCompileShader(&self, _shader: GLuint) {
        (self.entry().glCompileShader)(_shader)
    }
    unsafe fn glCompressedTexImage1D(
        &self,
        _target: GLenum,
        _level: GLint,
        _internalformat: GLenum,
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
    unsafe fn glCompressedTexImage2D(
        &self,
        _target: GLenum,
        _level: GLint,
        _internalformat: GLenum,
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
    unsafe fn glCompressedTexImage3D(
        &self,
        _target: GLenum,
        _level: GLint,
        _internalformat: GLenum,
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
    unsafe fn glCompressedTexSubImage1D(
        &self,
        _target: GLenum,
        _level: GLint,
        _xoffset: GLint,
        _width: GLsizei,
        _format: GLenum,
        _imageSize: GLsizei,
        _data: *const std::os::raw::c_void,
    ) {
        (self.entry().glCompressedTexSubImage1D)(
            _target, _level, _xoffset, _width, _format, _imageSize, _data,
        )
    }
    unsafe fn glCompressedTexSubImage2D(
        &self,
        _target: GLenum,
        _level: GLint,
        _xoffset: GLint,
        _yoffset: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _format: GLenum,
        _imageSize: GLsizei,
        _data: *const std::os::raw::c_void,
    ) {
        (self.entry().glCompressedTexSubImage2D)(
            _target, _level, _xoffset, _yoffset, _width, _height, _format, _imageSize, _data,
        )
    }
    unsafe fn glCompressedTexSubImage3D(
        &self,
        _target: GLenum,
        _level: GLint,
        _xoffset: GLint,
        _yoffset: GLint,
        _zoffset: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _depth: GLsizei,
        _format: GLenum,
        _imageSize: GLsizei,
        _data: *const std::os::raw::c_void,
    ) {
        (self.entry().glCompressedTexSubImage3D)(
            _target, _level, _xoffset, _yoffset, _zoffset, _width, _height, _depth, _format,
            _imageSize, _data,
        )
    }
    unsafe fn glCopyBufferSubData(
        &self,
        _readTarget: GLenum,
        _writeTarget: GLenum,
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
    unsafe fn glCopyImageSubData(
        &self,
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
    ) {
        (self.entry().glCopyImageSubData)(
            _srcName, _srcTarget, _srcLevel, _srcX, _srcY, _srcZ, _dstName, _dstTarget, _dstLevel,
            _dstX, _dstY, _dstZ, _srcWidth, _srcHeight, _srcDepth,
        )
    }
    unsafe fn glCopyTexImage1D(
        &self,
        _target: GLenum,
        _level: GLint,
        _internalformat: GLenum,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
        _border: GLint,
    ) {
        (self.entry().glCopyTexImage1D)(_target, _level, _internalformat, _x, _y, _width, _border)
    }
    unsafe fn glCopyTexImage2D(
        &self,
        _target: GLenum,
        _level: GLint,
        _internalformat: GLenum,
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
    unsafe fn glCopyTexSubImage1D(
        &self,
        _target: GLenum,
        _level: GLint,
        _xoffset: GLint,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
    ) {
        (self.entry().glCopyTexSubImage1D)(_target, _level, _xoffset, _x, _y, _width)
    }
    unsafe fn glCopyTexSubImage2D(
        &self,
        _target: GLenum,
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
    unsafe fn glCopyTexSubImage3D(
        &self,
        _target: GLenum,
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
    unsafe fn glCreateProgram(&self) -> GLuint {
        (self.entry().glCreateProgram)()
    }
    unsafe fn glCreateShader(&self, _type: GLenum) -> GLuint {
        (self.entry().glCreateShader)(_type)
    }
    unsafe fn glCreateShaderProgramv(
        &self,
        _type: GLenum,
        _count: GLsizei,
        _strings: *const *const GLchar,
    ) -> GLuint {
        (self.entry().glCreateShaderProgramv)(_type, _count, _strings)
    }
    unsafe fn glCullFace(&self, _mode: GLenum) {
        (self.entry().glCullFace)(_mode)
    }
    unsafe fn glDebugMessageCallback(
        &self,
        _callback: GLDEBUGPROC,
        _userParam: *const std::os::raw::c_void,
    ) {
        (self.entry().glDebugMessageCallback)(_callback, _userParam)
    }
    unsafe fn glDebugMessageControl(
        &self,
        _source: GLenum,
        _type: GLenum,
        _severity: GLenum,
        _count: GLsizei,
        _ids: *const GLuint,
        _enabled: GLboolean,
    ) {
        (self.entry().glDebugMessageControl)(_source, _type, _severity, _count, _ids, _enabled)
    }
    unsafe fn glDebugMessageInsert(
        &self,
        _source: GLenum,
        _type: GLenum,
        _id: GLuint,
        _severity: GLenum,
        _length: GLsizei,
        _buf: *const GLchar,
    ) {
        (self.entry().glDebugMessageInsert)(_source, _type, _id, _severity, _length, _buf)
    }
    unsafe fn glDeleteBuffers(&self, array: &[GLuint]) {
        (self.entry().glDeleteBuffers)(array.len() as GLsizei, array.as_ptr())
    }
    unsafe fn glDeleteFramebuffers(&self, array: &[GLuint]) {
        (self.entry().glDeleteFramebuffers)(array.len() as GLsizei, array.as_ptr())
    }
    unsafe fn glDeleteProgram(&self, _program: GLuint) {
        (self.entry().glDeleteProgram)(_program)
    }
    unsafe fn glDeleteProgramPipelines(&self, _n: GLsizei, _pipelines: *const GLuint) {
        (self.entry().glDeleteProgramPipelines)(_n, _pipelines)
    }
    unsafe fn glDeleteQueries(&self, array: &[GLuint]) {
        (self.entry().glDeleteQueries)(array.len() as GLsizei, array.as_ptr())
    }
    unsafe fn glDeleteRenderbuffers(&self, array: &[GLuint]) {
        (self.entry().glDeleteRenderbuffers)(array.len() as GLsizei, array.as_ptr())
    }
    unsafe fn glDeleteSamplers(&self, _count: GLsizei, _samplers: *const GLuint) {
        (self.entry().glDeleteSamplers)(_count, _samplers)
    }
    unsafe fn glDeleteShader(&self, _shader: GLuint) {
        (self.entry().glDeleteShader)(_shader)
    }
    unsafe fn glDeleteSync(&self, _sync: GLsync) {
        (self.entry().glDeleteSync)(_sync)
    }
    unsafe fn glDeleteTextures(&self, _n: GLsizei, _textures: *const GLuint) {
        (self.entry().glDeleteTextures)(_n, _textures)
    }
    unsafe fn glDeleteTransformFeedbacks(&self, _n: GLsizei, _ids: *const GLuint) {
        (self.entry().glDeleteTransformFeedbacks)(_n, _ids)
    }
    unsafe fn glDeleteVertexArrays(&self, _n: GLsizei, _arrays: *const GLuint) {
        (self.entry().glDeleteVertexArrays)(_n, _arrays)
    }
    unsafe fn glDepthFunc(&self, _func: GLenum) {
        (self.entry().glDepthFunc)(_func)
    }
    unsafe fn glDepthMask(&self, _flag: GLboolean) {
        (self.entry().glDepthMask)(_flag)
    }
    unsafe fn glDepthRange(&self, _n: GLdouble, _f: GLdouble) {
        (self.entry().glDepthRange)(_n, _f)
    }
    unsafe fn glDepthRangeArrayv(&self, _first: GLuint, _count: GLsizei, _v: *const GLdouble) {
        (self.entry().glDepthRangeArrayv)(_first, _count, _v)
    }
    unsafe fn glDepthRangeIndexed(&self, _index: GLuint, _n: GLdouble, _f: GLdouble) {
        (self.entry().glDepthRangeIndexed)(_index, _n, _f)
    }
    unsafe fn glDepthRangef(&self, _n: GLfloat, _f: GLfloat) {
        (self.entry().glDepthRangef)(_n, _f)
    }
    unsafe fn glDetachShader(&self, _program: GLuint, _shader: GLuint) {
        (self.entry().glDetachShader)(_program, _shader)
    }
    unsafe fn glDisable(&self, _cap: GLenum) {
        (self.entry().glDisable)(_cap)
    }
    unsafe fn glDisableVertexAttribArray(&self, _index: GLuint) {
        (self.entry().glDisableVertexAttribArray)(_index)
    }
    unsafe fn glDisablei(&self, _target: GLenum, _index: GLuint) {
        (self.entry().glDisablei)(_target, _index)
    }
    unsafe fn glDispatchCompute(
        &self,
        _num_groups_x: GLuint,
        _num_groups_y: GLuint,
        _num_groups_z: GLuint,
    ) {
        (self.entry().glDispatchCompute)(_num_groups_x, _num_groups_y, _num_groups_z)
    }
    unsafe fn glDispatchComputeIndirect(&self, _indirect: GLintptr) {
        (self.entry().glDispatchComputeIndirect)(_indirect)
    }
    unsafe fn glDrawArrays(&self, _mode: GLenum, _first: GLint, _count: GLsizei) {
        (self.entry().glDrawArrays)(_mode, _first, _count)
    }
    unsafe fn glDrawArraysIndirect(&self, _mode: GLenum, _indirect: *const std::os::raw::c_void) {
        (self.entry().glDrawArraysIndirect)(_mode, _indirect)
    }
    unsafe fn glDrawArraysInstanced(
        &self,
        _mode: GLenum,
        _first: GLint,
        _count: GLsizei,
        _instancecount: GLsizei,
    ) {
        (self.entry().glDrawArraysInstanced)(_mode, _first, _count, _instancecount)
    }
    unsafe fn glDrawArraysInstancedBaseInstance(
        &self,
        _mode: GLenum,
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
    unsafe fn glDrawBuffer(&self, _buf: GLenum) {
        (self.entry().glDrawBuffer)(_buf)
    }
    unsafe fn glDrawBuffers(&self, _n: GLsizei, _bufs: *const GLenum) {
        (self.entry().glDrawBuffers)(_n, _bufs)
    }
    unsafe fn glDrawElements(
        &self,
        _mode: GLenum,
        _count: GLsizei,
        _type: GLenum,
        _indices: *const std::os::raw::c_void,
    ) {
        (self.entry().glDrawElements)(_mode, _count, _type, _indices)
    }
    unsafe fn glDrawElementsBaseVertex(
        &self,
        _mode: GLenum,
        _count: GLsizei,
        _type: GLenum,
        _indices: *const std::os::raw::c_void,
        _basevertex: GLint,
    ) {
        (self.entry().glDrawElementsBaseVertex)(_mode, _count, _type, _indices, _basevertex)
    }
    unsafe fn glDrawElementsIndirect(
        &self,
        _mode: GLenum,
        _type: GLenum,
        _indirect: *const std::os::raw::c_void,
    ) {
        (self.entry().glDrawElementsIndirect)(_mode, _type, _indirect)
    }
    unsafe fn glDrawElementsInstanced(
        &self,
        _mode: GLenum,
        _count: GLsizei,
        _type: GLenum,
        _indices: *const std::os::raw::c_void,
        _instancecount: GLsizei,
    ) {
        (self.entry().glDrawElementsInstanced)(_mode, _count, _type, _indices, _instancecount)
    }
    unsafe fn glDrawElementsInstancedBaseInstance(
        &self,
        _mode: GLenum,
        _count: GLsizei,
        _type: GLenum,
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
    unsafe fn glDrawElementsInstancedBaseVertex(
        &self,
        _mode: GLenum,
        _count: GLsizei,
        _type: GLenum,
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
    unsafe fn glDrawElementsInstancedBaseVertexBaseInstance(
        &self,
        _mode: GLenum,
        _count: GLsizei,
        _type: GLenum,
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
    unsafe fn glDrawRangeElements(
        &self,
        _mode: GLenum,
        _start: GLuint,
        _end: GLuint,
        _count: GLsizei,
        _type: GLenum,
        _indices: *const std::os::raw::c_void,
    ) {
        (self.entry().glDrawRangeElements)(_mode, _start, _end, _count, _type, _indices)
    }
    unsafe fn glDrawRangeElementsBaseVertex(
        &self,
        _mode: GLenum,
        _start: GLuint,
        _end: GLuint,
        _count: GLsizei,
        _type: GLenum,
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
    unsafe fn glDrawTransformFeedback(&self, _mode: GLenum, _id: GLuint) {
        (self.entry().glDrawTransformFeedback)(_mode, _id)
    }
    unsafe fn glDrawTransformFeedbackInstanced(
        &self,
        _mode: GLenum,
        _id: GLuint,
        _instancecount: GLsizei,
    ) {
        (self.entry().glDrawTransformFeedbackInstanced)(_mode, _id, _instancecount)
    }
    unsafe fn glDrawTransformFeedbackStream(&self, _mode: GLenum, _id: GLuint, _stream: GLuint) {
        (self.entry().glDrawTransformFeedbackStream)(_mode, _id, _stream)
    }
    unsafe fn glDrawTransformFeedbackStreamInstanced(
        &self,
        _mode: GLenum,
        _id: GLuint,
        _stream: GLuint,
        _instancecount: GLsizei,
    ) {
        (self.entry().glDrawTransformFeedbackStreamInstanced)(_mode, _id, _stream, _instancecount)
    }
    unsafe fn glEnable(&self, _cap: GLenum) {
        (self.entry().glEnable)(_cap)
    }
    unsafe fn glEnableVertexAttribArray(&self, _index: GLuint) {
        (self.entry().glEnableVertexAttribArray)(_index)
    }
    unsafe fn glEnablei(&self, _target: GLenum, _index: GLuint) {
        (self.entry().glEnablei)(_target, _index)
    }
    unsafe fn glEndConditionalRender(&self) {
        (self.entry().glEndConditionalRender)()
    }
    unsafe fn glEndQuery(&self, _target: GLenum) {
        (self.entry().glEndQuery)(_target)
    }
    unsafe fn glEndQueryIndexed(&self, _target: GLenum, _index: GLuint) {
        (self.entry().glEndQueryIndexed)(_target, _index)
    }
    unsafe fn glEndTransformFeedback(&self) {
        (self.entry().glEndTransformFeedback)()
    }
    unsafe fn glFenceSync(&self, _condition: GLenum, _flags: GLbitfield) -> GLsync {
        (self.entry().glFenceSync)(_condition, _flags)
    }
    unsafe fn glFinish(&self) {
        (self.entry().glFinish)()
    }
    unsafe fn glFlush(&self) {
        (self.entry().glFlush)()
    }
    unsafe fn glFlushMappedBufferRange(
        &self,
        _target: GLenum,
        _offset: GLintptr,
        _length: GLsizeiptr,
    ) {
        (self.entry().glFlushMappedBufferRange)(_target, _offset, _length)
    }
    unsafe fn glFramebufferParameteri(&self, _target: GLenum, _pname: GLenum, _param: GLint) {
        (self.entry().glFramebufferParameteri)(_target, _pname, _param)
    }
    unsafe fn glFramebufferRenderbuffer(
        &self,
        _target: GLenum,
        _attachment: GLenum,
        _renderbuffertarget: GLenum,
        _renderbuffer: GLuint,
    ) {
        (self.entry().glFramebufferRenderbuffer)(
            _target,
            _attachment,
            _renderbuffertarget,
            _renderbuffer,
        )
    }
    unsafe fn glFramebufferTexture(
        &self,
        _target: GLenum,
        _attachment: GLenum,
        _texture: GLuint,
        _level: GLint,
    ) {
        (self.entry().glFramebufferTexture)(_target, _attachment, _texture, _level)
    }
    unsafe fn glFramebufferTexture1D(
        &self,
        _target: GLenum,
        _attachment: GLenum,
        _textarget: GLenum,
        _texture: GLuint,
        _level: GLint,
    ) {
        (self.entry().glFramebufferTexture1D)(_target, _attachment, _textarget, _texture, _level)
    }
    unsafe fn glFramebufferTexture2D(
        &self,
        _target: GLenum,
        _attachment: GLenum,
        _textarget: GLenum,
        _texture: GLuint,
        _level: GLint,
    ) {
        (self.entry().glFramebufferTexture2D)(_target, _attachment, _textarget, _texture, _level)
    }
    unsafe fn glFramebufferTexture3D(
        &self,
        _target: GLenum,
        _attachment: GLenum,
        _textarget: GLenum,
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
    unsafe fn glFramebufferTextureLayer(
        &self,
        _target: GLenum,
        _attachment: GLenum,
        _texture: GLuint,
        _level: GLint,
        _layer: GLint,
    ) {
        (self.entry().glFramebufferTextureLayer)(_target, _attachment, _texture, _level, _layer)
    }
    unsafe fn glFrontFace(&self, _mode: GLenum) {
        (self.entry().glFrontFace)(_mode)
    }
    unsafe fn glGenBuffers(&self, _n: GLsizei, _buffers: *mut GLuint) {
        (self.entry().glGenBuffers)(_n, _buffers)
    }
    unsafe fn glGenFramebuffers(&self, array: &mut [GLuint]) {
        (self.entry().glGenFramebuffers)(array.len() as GLsizei, array.as_mut_ptr())
    }
    unsafe fn glGenProgramPipelines(&self, array: &mut [GLuint]) {
        (self.entry().glGenProgramPipelines)(array.len() as GLsizei, array.as_mut_ptr())
    }
    unsafe fn glGenQueries(&self, array: &mut [GLuint]) {
        (self.entry().glGenQueries)(array.len() as GLsizei, array.as_mut_ptr())
    }
    unsafe fn glGenRenderbuffers(&self, renderbuffers: &mut [GLuint]) {
        (self.entry().glGenRenderbuffers)(
            renderbuffers.len() as GLsizei,
            renderbuffers.as_mut_ptr(),
        )
    }
    unsafe fn glGenSamplers(&self, samplers: &mut [GLuint]) {
        (self.entry().glGenSamplers)(samplers.len() as GLsizei, samplers.as_mut_ptr())
    }
    unsafe fn glGenTextures(&self, array: &mut [GLuint]) {
        (self.entry().glGenTextures)(array.len() as GLsizei, array.as_mut_ptr())
    }
    unsafe fn glGenTransformFeedbacks(&self, array: &mut [GLuint]) {
        (self.entry().glGenTransformFeedbacks)(array.len() as GLsizei, array.as_mut_ptr())
    }
    unsafe fn glGenVertexArrays(&self, array: &mut [GLuint]) {
        (self.entry().glGenVertexArrays)(array.len() as GLsizei, array.as_mut_ptr())
    }
    unsafe fn glGenerateMipmap(&self, _target: GLenum) {
        (self.entry().glGenerateMipmap)(_target)
    }
    unsafe fn glGetActiveAtomicCounterBufferiv(
        &self,
        _program: GLuint,
        _bufferIndex: GLuint,
        _pname: GLenum,
        _params: *mut GLint,
    ) {
        (self.entry().glGetActiveAtomicCounterBufferiv)(_program, _bufferIndex, _pname, _params)
    }
    unsafe fn glGetActiveAttrib(
        &self,
        _program: GLuint,
        _index: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _size: *mut GLint,
        _type: *mut GLenum,
        _name: *mut GLchar,
    ) {
        (self.entry().glGetActiveAttrib)(_program, _index, _bufSize, _length, _size, _type, _name)
    }
    unsafe fn glGetActiveSubroutineName(
        &self,
        _program: GLuint,
        _shadertype: GLenum,
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
    unsafe fn glGetActiveSubroutineUniformName(
        &self,
        _program: GLuint,
        _shadertype: GLenum,
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
    unsafe fn glGetActiveSubroutineUniformiv(
        &self,
        _program: GLuint,
        _shadertype: GLenum,
        _index: GLuint,
        _pname: GLenum,
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
    unsafe fn glGetActiveUniform(
        &self,
        _program: GLuint,
        _index: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _size: *mut GLint,
        _type: *mut GLenum,
        _name: *mut GLchar,
    ) {
        (self.entry().glGetActiveUniform)(_program, _index, _bufSize, _length, _size, _type, _name)
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
    unsafe fn glGetActiveUniformBlockiv(
        &self,
        _program: GLuint,
        _uniformBlockIndex: GLuint,
        _pname: GLenum,
        _params: *mut GLint,
    ) {
        (self.entry().glGetActiveUniformBlockiv)(_program, _uniformBlockIndex, _pname, _params)
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
    unsafe fn glGetActiveUniformsiv(
        &self,
        _program: GLuint,
        _uniformCount: GLsizei,
        _uniformIndices: *const GLuint,
        _pname: GLenum,
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
    unsafe fn glGetAttachedShaders(
        &self,
        _program: GLuint,
        _maxCount: GLsizei,
        _count: *mut GLsizei,
        _shaders: *mut GLuint,
    ) {
        (self.entry().glGetAttachedShaders)(_program, _maxCount, _count, _shaders)
    }
    unsafe fn glGetAttribLocation(&self, _program: GLuint, _name: *const GLchar) -> GLint {
        (self.entry().glGetAttribLocation)(_program, _name)
    }
    unsafe fn glGetBooleani_v(&self, _target: GLenum, _index: GLuint, _data: *mut GLboolean) {
        (self.entry().glGetBooleani_v)(_target, _index, _data)
    }
    unsafe fn glGetBooleanv(&self, _pname: GLenum, _data: *mut GLboolean) {
        (self.entry().glGetBooleanv)(_pname, _data)
    }
    unsafe fn glGetBufferParameteri64v(
        &self,
        _target: GLenum,
        _pname: GLenum,
        _params: *mut GLint64,
    ) {
        (self.entry().glGetBufferParameteri64v)(_target, _pname, _params)
    }
    unsafe fn glGetBufferParameteriv(&self, _target: GLenum, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetBufferParameteriv)(_target, _pname, _params)
    }
    unsafe fn glGetBufferPointerv(
        &self,
        _target: GLenum,
        _pname: GLenum,
        _params: *mut *mut std::os::raw::c_void,
    ) {
        (self.entry().glGetBufferPointerv)(_target, _pname, _params)
    }
    unsafe fn glGetBufferSubData(
        &self,
        _target: GLenum,
        _offset: GLintptr,
        _size: GLsizeiptr,
        _data: *mut std::os::raw::c_void,
    ) {
        (self.entry().glGetBufferSubData)(_target, _offset, _size, _data)
    }
    unsafe fn glGetCompressedTexImage(
        &self,
        _target: GLenum,
        _level: GLint,
        _img: *mut std::os::raw::c_void,
    ) {
        (self.entry().glGetCompressedTexImage)(_target, _level, _img)
    }
    unsafe fn glGetDebugMessageLog(
        &self,
        _count: GLuint,
        _bufSize: GLsizei,
        _sources: *mut GLenum,
        _types: *mut GLenum,
        _ids: *mut GLuint,
        _severities: *mut GLenum,
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
    unsafe fn glGetDoublei_v(&self, _target: GLenum, _index: GLuint, _data: *mut GLdouble) {
        (self.entry().glGetDoublei_v)(_target, _index, _data)
    }
    unsafe fn glGetDoublev(&self, _pname: GLenum, _data: *mut GLdouble) {
        (self.entry().glGetDoublev)(_pname, _data)
    }
    unsafe fn glGetError(&self) -> GLenum {
        (self.entry().glGetError)()
    }
    unsafe fn glGetFloati_v(&self, _target: GLenum, _index: GLuint, _data: *mut GLfloat) {
        (self.entry().glGetFloati_v)(_target, _index, _data)
    }
    unsafe fn glGetFloatv(&self, _pname: GLenum, _data: *mut GLfloat) {
        (self.entry().glGetFloatv)(_pname, _data)
    }
    unsafe fn glGetFragDataIndex(&self, _program: GLuint, _name: *const GLchar) -> GLint {
        (self.entry().glGetFragDataIndex)(_program, _name)
    }
    unsafe fn glGetFragDataLocation(&self, _program: GLuint, _name: *const GLchar) -> GLint {
        (self.entry().glGetFragDataLocation)(_program, _name)
    }
    unsafe fn glGetFramebufferAttachmentParameteriv(
        &self,
        _target: GLenum,
        _attachment: GLenum,
        _pname: GLenum,
        _params: *mut GLint,
    ) {
        (self.entry().glGetFramebufferAttachmentParameteriv)(_target, _attachment, _pname, _params)
    }
    unsafe fn glGetFramebufferParameteriv(
        &self,
        _target: GLenum,
        _pname: GLenum,
        _params: *mut GLint,
    ) {
        (self.entry().glGetFramebufferParameteriv)(_target, _pname, _params)
    }
    unsafe fn glGetInteger64i_v(&self, _target: GLenum, _index: GLuint, _data: *mut GLint64) {
        (self.entry().glGetInteger64i_v)(_target, _index, _data)
    }
    unsafe fn glGetInteger64v(&self, _pname: GLenum, _data: *mut GLint64) {
        (self.entry().glGetInteger64v)(_pname, _data)
    }
    unsafe fn glGetIntegeri_v(&self, _target: GLenum, _index: GLuint, _data: *mut GLint) {
        (self.entry().glGetIntegeri_v)(_target, _index, _data)
    }
    unsafe fn glGetIntegerv(&self, _pname: GLenum, _data: *mut GLint) {
        (self.entry().glGetIntegerv)(_pname, _data)
    }
    unsafe fn glGetInternalformati64v(
        &self,
        _target: GLenum,
        _internalformat: GLenum,
        _pname: GLenum,
        _count: GLsizei,
        _params: *mut GLint64,
    ) {
        (self.entry().glGetInternalformati64v)(_target, _internalformat, _pname, _count, _params)
    }
    unsafe fn glGetInternalformativ(
        &self,
        _target: GLenum,
        _internalformat: GLenum,
        _pname: GLenum,
        _count: GLsizei,
        _params: *mut GLint,
    ) {
        (self.entry().glGetInternalformativ)(_target, _internalformat, _pname, _count, _params)
    }
    unsafe fn glGetMultisamplefv(&self, _pname: GLenum, _index: GLuint, _val: *mut GLfloat) {
        (self.entry().glGetMultisamplefv)(_pname, _index, _val)
    }
    unsafe fn glGetObjectLabel(
        &self,
        _identifier: GLenum,
        _name: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _label: *mut GLchar,
    ) {
        (self.entry().glGetObjectLabel)(_identifier, _name, _bufSize, _length, _label)
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
    unsafe fn glGetPointerv(&self, _pname: GLenum, _params: *mut *mut std::os::raw::c_void) {
        (self.entry().glGetPointerv)(_pname, _params)
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
    unsafe fn glGetProgramInfoLog(
        &self,
        _program: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _infoLog: *mut GLchar,
    ) {
        (self.entry().glGetProgramInfoLog)(_program, _bufSize, _length, _infoLog)
    }
    unsafe fn glGetProgramInterfaceiv(
        &self,
        _program: GLuint,
        _programInterface: GLenum,
        _pname: GLenum,
        _params: *mut GLint,
    ) {
        (self.entry().glGetProgramInterfaceiv)(_program, _programInterface, _pname, _params)
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
    unsafe fn glGetProgramPipelineiv(
        &self,
        _pipeline: GLuint,
        _pname: GLenum,
        _params: *mut GLint,
    ) {
        (self.entry().glGetProgramPipelineiv)(_pipeline, _pname, _params)
    }
    unsafe fn glGetProgramResourceIndex(
        &self,
        _program: GLuint,
        _programInterface: GLenum,
        _name: *const GLchar,
    ) -> GLuint {
        (self.entry().glGetProgramResourceIndex)(_program, _programInterface, _name)
    }
    unsafe fn glGetProgramResourceLocation(
        &self,
        _program: GLuint,
        _programInterface: GLenum,
        _name: *const GLchar,
    ) -> GLint {
        (self.entry().glGetProgramResourceLocation)(_program, _programInterface, _name)
    }
    unsafe fn glGetProgramResourceLocationIndex(
        &self,
        _program: GLuint,
        _programInterface: GLenum,
        _name: *const GLchar,
    ) -> GLint {
        (self.entry().glGetProgramResourceLocationIndex)(_program, _programInterface, _name)
    }
    unsafe fn glGetProgramResourceName(
        &self,
        _program: GLuint,
        _programInterface: GLenum,
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
    unsafe fn glGetProgramResourceiv(
        &self,
        _program: GLuint,
        _programInterface: GLenum,
        _index: GLuint,
        _propCount: GLsizei,
        _props: *const GLenum,
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
    unsafe fn glGetProgramStageiv(
        &self,
        _program: GLuint,
        _shadertype: GLenum,
        _pname: GLenum,
        _values: *mut GLint,
    ) {
        (self.entry().glGetProgramStageiv)(_program, _shadertype, _pname, _values)
    }
    unsafe fn glGetProgramiv(&self, _program: GLuint, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetProgramiv)(_program, _pname, _params)
    }
    unsafe fn glGetQueryIndexediv(
        &self,
        _target: GLenum,
        _index: GLuint,
        _pname: GLenum,
        _params: *mut GLint,
    ) {
        (self.entry().glGetQueryIndexediv)(_target, _index, _pname, _params)
    }
    unsafe fn glGetQueryObjecti64v(&self, _id: GLuint, _pname: GLenum, _params: *mut GLint64) {
        (self.entry().glGetQueryObjecti64v)(_id, _pname, _params)
    }
    unsafe fn glGetQueryObjectiv(&self, _id: GLuint, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetQueryObjectiv)(_id, _pname, _params)
    }
    unsafe fn glGetQueryObjectui64v(&self, _id: GLuint, _pname: GLenum, _params: *mut GLuint64) {
        (self.entry().glGetQueryObjectui64v)(_id, _pname, _params)
    }
    unsafe fn glGetQueryObjectuiv(&self, _id: GLuint, _pname: GLenum, _params: *mut GLuint) {
        (self.entry().glGetQueryObjectuiv)(_id, _pname, _params)
    }
    unsafe fn glGetQueryiv(&self, _target: GLenum, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetQueryiv)(_target, _pname, _params)
    }
    unsafe fn glGetRenderbufferParameteriv(
        &self,
        _target: GLenum,
        _pname: GLenum,
        _params: *mut GLint,
    ) {
        (self.entry().glGetRenderbufferParameteriv)(_target, _pname, _params)
    }
    unsafe fn glGetSamplerParameterIiv(
        &self,
        _sampler: GLuint,
        _pname: GLenum,
        _params: *mut GLint,
    ) {
        (self.entry().glGetSamplerParameterIiv)(_sampler, _pname, _params)
    }
    unsafe fn glGetSamplerParameterIuiv(
        &self,
        _sampler: GLuint,
        _pname: GLenum,
        _params: *mut GLuint,
    ) {
        (self.entry().glGetSamplerParameterIuiv)(_sampler, _pname, _params)
    }
    unsafe fn glGetSamplerParameterfv(
        &self,
        _sampler: GLuint,
        _pname: GLenum,
        _params: *mut GLfloat,
    ) {
        (self.entry().glGetSamplerParameterfv)(_sampler, _pname, _params)
    }
    unsafe fn glGetSamplerParameteriv(
        &self,
        _sampler: GLuint,
        _pname: GLenum,
        _params: *mut GLint,
    ) {
        (self.entry().glGetSamplerParameteriv)(_sampler, _pname, _params)
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
    unsafe fn glGetShaderPrecisionFormat(
        &self,
        _shadertype: GLenum,
        _precisiontype: GLenum,
        _range: *mut GLint,
        _precision: *mut GLint,
    ) {
        (self.entry().glGetShaderPrecisionFormat)(_shadertype, _precisiontype, _range, _precision)
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
    unsafe fn glGetShaderiv(&self, _shader: GLuint, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetShaderiv)(_shader, _pname, _params)
    }
    unsafe fn glGetString(&self, _name: GLenum) -> *const GLubyte {
        (self.entry().glGetString)(_name)
    }
    unsafe fn glGetStringi(&self, _name: GLenum, _index: GLuint) -> *const GLubyte {
        (self.entry().glGetStringi)(_name, _index)
    }
    unsafe fn glGetSubroutineIndex(
        &self,
        _program: GLuint,
        _shadertype: GLenum,
        _name: *const GLchar,
    ) -> GLuint {
        (self.entry().glGetSubroutineIndex)(_program, _shadertype, _name)
    }
    unsafe fn glGetSubroutineUniformLocation(
        &self,
        _program: GLuint,
        _shadertype: GLenum,
        _name: *const GLchar,
    ) -> GLint {
        (self.entry().glGetSubroutineUniformLocation)(_program, _shadertype, _name)
    }
    unsafe fn glGetSynciv(
        &self,
        _sync: GLsync,
        _pname: GLenum,
        _count: GLsizei,
        _length: *mut GLsizei,
        _values: *mut GLint,
    ) {
        (self.entry().glGetSynciv)(_sync, _pname, _count, _length, _values)
    }
    unsafe fn glGetTexImage(
        &self,
        _target: GLenum,
        _level: GLint,
        _format: GLenum,
        _type: GLenum,
        _pixels: *mut std::os::raw::c_void,
    ) {
        (self.entry().glGetTexImage)(_target, _level, _format, _type, _pixels)
    }
    unsafe fn glGetTexLevelParameterfv(
        &self,
        _target: GLenum,
        _level: GLint,
        _pname: GLenum,
        _params: *mut GLfloat,
    ) {
        (self.entry().glGetTexLevelParameterfv)(_target, _level, _pname, _params)
    }
    unsafe fn glGetTexLevelParameteriv(
        &self,
        _target: GLenum,
        _level: GLint,
        _pname: GLenum,
        _params: *mut GLint,
    ) {
        (self.entry().glGetTexLevelParameteriv)(_target, _level, _pname, _params)
    }
    unsafe fn glGetTexParameterIiv(&self, _target: GLenum, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetTexParameterIiv)(_target, _pname, _params)
    }
    unsafe fn glGetTexParameterIuiv(&self, _target: GLenum, _pname: GLenum, _params: *mut GLuint) {
        (self.entry().glGetTexParameterIuiv)(_target, _pname, _params)
    }
    unsafe fn glGetTexParameterfv(&self, _target: GLenum, _pname: GLenum, _params: *mut GLfloat) {
        (self.entry().glGetTexParameterfv)(_target, _pname, _params)
    }
    unsafe fn glGetTexParameteriv(&self, _target: GLenum, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetTexParameteriv)(_target, _pname, _params)
    }
    unsafe fn glGetTransformFeedbackVarying(
        &self,
        _program: GLuint,
        _index: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _size: *mut GLsizei,
        _type: *mut GLenum,
        _name: *mut GLchar,
    ) {
        (self.entry().glGetTransformFeedbackVarying)(
            _program, _index, _bufSize, _length, _size, _type, _name,
        )
    }
    unsafe fn glGetUniformBlockIndex(
        &self,
        _program: GLuint,
        _uniformBlockName: *const GLchar,
    ) -> GLuint {
        (self.entry().glGetUniformBlockIndex)(_program, _uniformBlockName)
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
    unsafe fn glGetUniformLocation(&self, _program: GLuint, _name: *const GLchar) -> GLint {
        (self.entry().glGetUniformLocation)(_program, _name)
    }
    unsafe fn glGetUniformSubroutineuiv(
        &self,
        _shadertype: GLenum,
        _location: GLint,
        _params: *mut GLuint,
    ) {
        (self.entry().glGetUniformSubroutineuiv)(_shadertype, _location, _params)
    }
    unsafe fn glGetUniformdv(&self, program: GLuint, location: GLint, params: &mut [GLdouble]) {
        (self.entry().glGetUniformdv)(program, location, params.as_mut_ptr())
    }
    unsafe fn glGetUniformfv(&self, program: GLuint, location: GLint, params: &mut [GLfloat]) {
        (self.entry().glGetUniformfv)(program, location, params.as_mut_ptr())
    }
    unsafe fn glGetUniformiv(&self, program: GLuint, location: GLint, params: &mut [GLint]) {
        (self.entry().glGetUniformiv)(program, location, params.as_mut_ptr())
    }
    unsafe fn glGetUniformuiv(&self, program: GLuint, location: GLint, params: &mut [GLuint]) {
        (self.entry().glGetUniformuiv)(program, location, params.as_mut_ptr())
    }
    unsafe fn glGetVertexAttribIiv(&self, _index: GLuint, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetVertexAttribIiv)(_index, _pname, _params)
    }
    unsafe fn glGetVertexAttribIuiv(&self, _index: GLuint, _pname: GLenum, _params: *mut GLuint) {
        (self.entry().glGetVertexAttribIuiv)(_index, _pname, _params)
    }
    unsafe fn glGetVertexAttribLdv(&self, _index: GLuint, _pname: GLenum, _params: *mut GLdouble) {
        (self.entry().glGetVertexAttribLdv)(_index, _pname, _params)
    }
    unsafe fn glGetVertexAttribPointerv(
        &self,
        _index: GLuint,
        _pname: GLenum,
        _pointer: *mut *mut std::os::raw::c_void,
    ) {
        (self.entry().glGetVertexAttribPointerv)(_index, _pname, _pointer)
    }
    unsafe fn glGetVertexAttribdv(&self, _index: GLuint, _pname: GLenum, _params: *mut GLdouble) {
        (self.entry().glGetVertexAttribdv)(_index, _pname, _params)
    }
    unsafe fn glGetVertexAttribfv(&self, _index: GLuint, _pname: GLenum, _params: *mut GLfloat) {
        (self.entry().glGetVertexAttribfv)(_index, _pname, _params)
    }
    unsafe fn glGetVertexAttribiv(&self, _index: GLuint, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetVertexAttribiv)(_index, _pname, _params)
    }
    unsafe fn glHint(&self, _target: GLenum, _mode: GLenum) {
        (self.entry().glHint)(_target, _mode)
    }
    unsafe fn glInvalidateBufferData(&self, _buffer: GLuint) {
        (self.entry().glInvalidateBufferData)(_buffer)
    }
    unsafe fn glInvalidateBufferSubData(
        &self,
        _buffer: GLuint,
        _offset: GLintptr,
        _length: GLsizeiptr,
    ) {
        (self.entry().glInvalidateBufferSubData)(_buffer, _offset, _length)
    }
    unsafe fn glInvalidateFramebuffer(
        &self,
        _target: GLenum,
        _numAttachments: GLsizei,
        _attachments: *const GLenum,
    ) {
        (self.entry().glInvalidateFramebuffer)(_target, _numAttachments, _attachments)
    }
    unsafe fn glInvalidateSubFramebuffer(
        &self,
        _target: GLenum,
        _numAttachments: GLsizei,
        _attachments: *const GLenum,
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
    unsafe fn glInvalidateTexImage(&self, _texture: GLuint, _level: GLint) {
        (self.entry().glInvalidateTexImage)(_texture, _level)
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
    unsafe fn glIsBuffer(&self, _buffer: GLuint) -> GLboolean {
        (self.entry().glIsBuffer)(_buffer)
    }
    unsafe fn glIsEnabled(&self, _cap: GLenum) -> GLboolean {
        (self.entry().glIsEnabled)(_cap)
    }
    unsafe fn glIsEnabledi(&self, _target: GLenum, _index: GLuint) -> GLboolean {
        (self.entry().glIsEnabledi)(_target, _index)
    }
    unsafe fn glIsFramebuffer(&self, _framebuffer: GLuint) -> GLboolean {
        (self.entry().glIsFramebuffer)(_framebuffer)
    }
    unsafe fn glIsProgram(&self, _program: GLuint) -> GLboolean {
        (self.entry().glIsProgram)(_program)
    }
    unsafe fn glIsProgramPipeline(&self, _pipeline: GLuint) -> GLboolean {
        (self.entry().glIsProgramPipeline)(_pipeline)
    }
    unsafe fn glIsQuery(&self, _id: GLuint) -> GLboolean {
        (self.entry().glIsQuery)(_id)
    }
    unsafe fn glIsRenderbuffer(&self, _renderbuffer: GLuint) -> GLboolean {
        (self.entry().glIsRenderbuffer)(_renderbuffer)
    }
    unsafe fn glIsSampler(&self, _sampler: GLuint) -> GLboolean {
        (self.entry().glIsSampler)(_sampler)
    }
    unsafe fn glIsShader(&self, _shader: GLuint) -> GLboolean {
        (self.entry().glIsShader)(_shader)
    }
    unsafe fn glIsSync(&self, _sync: GLsync) -> GLboolean {
        (self.entry().glIsSync)(_sync)
    }
    unsafe fn glIsTexture(&self, _texture: GLuint) -> GLboolean {
        (self.entry().glIsTexture)(_texture)
    }
    unsafe fn glIsTransformFeedback(&self, _id: GLuint) -> GLboolean {
        (self.entry().glIsTransformFeedback)(_id)
    }
    unsafe fn glIsVertexArray(&self, _array: GLuint) -> GLboolean {
        (self.entry().glIsVertexArray)(_array)
    }
    unsafe fn glLineWidth(&self, _width: GLfloat) {
        (self.entry().glLineWidth)(_width)
    }
    unsafe fn glLinkProgram(&self, _program: GLuint) {
        (self.entry().glLinkProgram)(_program)
    }
    unsafe fn glLogicOp(&self, _opcode: GLenum) {
        (self.entry().glLogicOp)(_opcode)
    }
    unsafe fn glMapBuffer(&self, _target: GLenum, _access: GLenum) -> *mut std::os::raw::c_void {
        (self.entry().glMapBuffer)(_target, _access)
    }
    unsafe fn glMapBufferRange(
        &self,
        _target: GLenum,
        _offset: GLintptr,
        _length: GLsizeiptr,
        _access: GLbitfield,
    ) -> *mut std::os::raw::c_void {
        (self.entry().glMapBufferRange)(_target, _offset, _length, _access)
    }
    unsafe fn glMemoryBarrier(&self, _barriers: GLbitfield) {
        (self.entry().glMemoryBarrier)(_barriers)
    }
    unsafe fn glMinSampleShading(&self, _value: GLfloat) {
        (self.entry().glMinSampleShading)(_value)
    }
    unsafe fn glMultiDrawArrays(
        &self,
        _mode: GLenum,
        _first: *const GLint,
        _count: *const GLsizei,
        _drawcount: GLsizei,
    ) {
        (self.entry().glMultiDrawArrays)(_mode, _first, _count, _drawcount)
    }
    unsafe fn glMultiDrawArraysIndirect(
        &self,
        _mode: GLenum,
        _indirect: *const std::os::raw::c_void,
        _drawcount: GLsizei,
        _stride: GLsizei,
    ) {
        (self.entry().glMultiDrawArraysIndirect)(_mode, _indirect, _drawcount, _stride)
    }
    unsafe fn glMultiDrawElements(
        &self,
        _mode: GLenum,
        _count: *const GLsizei,
        _type: GLenum,
        _indices: *const *const std::os::raw::c_void,
        _drawcount: GLsizei,
    ) {
        (self.entry().glMultiDrawElements)(_mode, _count, _type, _indices, _drawcount)
    }
    unsafe fn glMultiDrawElementsBaseVertex(
        &self,
        _mode: GLenum,
        _count: *const GLsizei,
        _type: GLenum,
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
    unsafe fn glMultiDrawElementsIndirect(
        &self,
        _mode: GLenum,
        _type: GLenum,
        _indirect: *const std::os::raw::c_void,
        _drawcount: GLsizei,
        _stride: GLsizei,
    ) {
        (self.entry().glMultiDrawElementsIndirect)(_mode, _type, _indirect, _drawcount, _stride)
    }
    unsafe fn glMultiTexCoordP1ui(&self, _texture: GLenum, _type: GLenum, _coords: GLuint) {
        (self.entry().glMultiTexCoordP1ui)(_texture, _type, _coords)
    }
    unsafe fn glMultiTexCoordP1uiv(&self, _texture: GLenum, _type: GLenum, _coords: *const GLuint) {
        (self.entry().glMultiTexCoordP1uiv)(_texture, _type, _coords)
    }
    unsafe fn glMultiTexCoordP2ui(&self, _texture: GLenum, _type: GLenum, _coords: GLuint) {
        (self.entry().glMultiTexCoordP2ui)(_texture, _type, _coords)
    }
    unsafe fn glMultiTexCoordP2uiv(&self, _texture: GLenum, _type: GLenum, _coords: *const GLuint) {
        (self.entry().glMultiTexCoordP2uiv)(_texture, _type, _coords)
    }
    unsafe fn glMultiTexCoordP3ui(&self, _texture: GLenum, _type: GLenum, _coords: GLuint) {
        (self.entry().glMultiTexCoordP3ui)(_texture, _type, _coords)
    }
    unsafe fn glMultiTexCoordP3uiv(&self, _texture: GLenum, _type: GLenum, _coords: *const GLuint) {
        (self.entry().glMultiTexCoordP3uiv)(_texture, _type, _coords)
    }
    unsafe fn glMultiTexCoordP4ui(&self, _texture: GLenum, _type: GLenum, _coords: GLuint) {
        (self.entry().glMultiTexCoordP4ui)(_texture, _type, _coords)
    }
    unsafe fn glMultiTexCoordP4uiv(&self, _texture: GLenum, _type: GLenum, _coords: *const GLuint) {
        (self.entry().glMultiTexCoordP4uiv)(_texture, _type, _coords)
    }
    unsafe fn glNormalP3ui(&self, _type: GLenum, _coords: GLuint) {
        (self.entry().glNormalP3ui)(_type, _coords)
    }
    unsafe fn glNormalP3uiv(&self, _type: GLenum, _coords: *const GLuint) {
        (self.entry().glNormalP3uiv)(_type, _coords)
    }
    unsafe fn glObjectLabel(
        &self,
        _identifier: GLenum,
        _name: GLuint,
        _length: GLsizei,
        _label: *const GLchar,
    ) {
        (self.entry().glObjectLabel)(_identifier, _name, _length, _label)
    }
    unsafe fn glObjectPtrLabel(
        &self,
        _ptr: *const std::os::raw::c_void,
        _length: GLsizei,
        _label: *const GLchar,
    ) {
        (self.entry().glObjectPtrLabel)(_ptr, _length, _label)
    }
    unsafe fn glPatchParameterfv(&self, _pname: GLenum, _values: *const GLfloat) {
        (self.entry().glPatchParameterfv)(_pname, _values)
    }
    unsafe fn glPatchParameteri(&self, _pname: GLenum, _value: GLint) {
        (self.entry().glPatchParameteri)(_pname, _value)
    }
    unsafe fn glPauseTransformFeedback(&self) {
        (self.entry().glPauseTransformFeedback)()
    }
    unsafe fn glPixelStoref(&self, _pname: GLenum, _param: GLfloat) {
        (self.entry().glPixelStoref)(_pname, _param)
    }
    unsafe fn glPixelStorei(&self, _pname: GLenum, _param: GLint) {
        (self.entry().glPixelStorei)(_pname, _param)
    }
    unsafe fn glPointParameterf(&self, _pname: GLenum, _param: GLfloat) {
        (self.entry().glPointParameterf)(_pname, _param)
    }
    unsafe fn glPointParameterfv(&self, _pname: GLenum, _params: *const GLfloat) {
        (self.entry().glPointParameterfv)(_pname, _params)
    }
    unsafe fn glPointParameteri(&self, _pname: GLenum, _param: GLint) {
        (self.entry().glPointParameteri)(_pname, _param)
    }
    unsafe fn glPointParameteriv(&self, _pname: GLenum, _params: *const GLint) {
        (self.entry().glPointParameteriv)(_pname, _params)
    }
    unsafe fn glPointSize(&self, _size: GLfloat) {
        (self.entry().glPointSize)(_size)
    }
    unsafe fn glPolygonMode(&self, _face: GLenum, _mode: GLenum) {
        (self.entry().glPolygonMode)(_face, _mode)
    }
    unsafe fn glPolygonOffset(&self, _factor: GLfloat, _units: GLfloat) {
        (self.entry().glPolygonOffset)(_factor, _units)
    }
    unsafe fn glPopDebugGroup(&self) {
        (self.entry().glPopDebugGroup)()
    }
    unsafe fn glPrimitiveRestartIndex(&self, _index: GLuint) {
        (self.entry().glPrimitiveRestartIndex)(_index)
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
    unsafe fn glProgramParameteri(&self, _program: GLuint, _pname: GLenum, _value: GLint) {
        (self.entry().glProgramParameteri)(_program, _pname, _value)
    }
    unsafe fn glProgramUniform1d(&self, _program: GLuint, _location: GLint, _v0: GLdouble) {
        (self.entry().glProgramUniform1d)(_program, _location, _v0)
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
    unsafe fn glProgramUniform1f(&self, _program: GLuint, _location: GLint, _v0: GLfloat) {
        (self.entry().glProgramUniform1f)(_program, _location, _v0)
    }
    unsafe fn glProgramUniform1fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: &[GLfloat],
    ) {
        assert!(count <= value.len() as GLsizei);
        (self.entry().glProgramUniform1fv)(program, location, count, value.as_ptr())
    }
    unsafe fn glProgramUniform1i(&self, _program: GLuint, _location: GLint, _v0: GLint) {
        (self.entry().glProgramUniform1i)(_program, _location, _v0)
    }
    unsafe fn glProgramUniform1iv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: &[GLint],
    ) {
        assert!(count <= value.len() as GLsizei);
        (self.entry().glProgramUniform1iv)(program, location, count, value.as_ptr())
    }
    unsafe fn glProgramUniform1ui(&self, _program: GLuint, _location: GLint, _v0: GLuint) {
        (self.entry().glProgramUniform1ui)(_program, _location, _v0)
    }
    unsafe fn glProgramUniform1uiv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: &[GLuint],
    ) {
        assert!(count <= value.len() as GLsizei);
        (self.entry().glProgramUniform1uiv)(program, location, count, value.as_ptr())
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
    unsafe fn glProgramUniform2dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLdouble,
    ) {
        (self.entry().glProgramUniform2dv)(_program, _location, _count, _value)
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
    unsafe fn glProgramUniform2fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: &[GLfloat],
    ) {
        assert!((count * 2) <= value.len() as GLsizei);
        (self.entry().glProgramUniform2fv)(program, location, count, value.as_ptr())
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
    unsafe fn glProgramUniform2iv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: &[GLint],
    ) {
        assert!((count * 2) <= value.len() as GLsizei);
        (self.entry().glProgramUniform2iv)(program, location, count, value.as_ptr())
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
    unsafe fn glProgramUniform2uiv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: &[GLuint],
    ) {
        assert!((count * 2) <= value.len() as GLsizei);
        (self.entry().glProgramUniform2uiv)(program, location, count, value.as_ptr())
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
    unsafe fn glProgramUniform3dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLdouble,
    ) {
        (self.entry().glProgramUniform3dv)(_program, _location, _count, _value)
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
    unsafe fn glProgramUniform3fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: &[GLfloat],
    ) {
        assert!((count * 3) <= value.len() as GLsizei);
        (self.entry().glProgramUniform3fv)(program, location, count, value.as_ptr())
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
    unsafe fn glProgramUniform3iv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: &[GLint],
    ) {
        assert!((count * 3) <= value.len() as GLsizei);
        (self.entry().glProgramUniform3iv)(program, location, count, value.as_ptr())
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
    unsafe fn glProgramUniform3uiv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: &[GLuint],
    ) {
        assert!((count * 3) <= value.len() as GLsizei);
        (self.entry().glProgramUniform3uiv)(program, location, count, value.as_ptr())
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
    unsafe fn glProgramUniform4dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLdouble,
    ) {
        (self.entry().glProgramUniform4dv)(_program, _location, _count, _value)
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
    unsafe fn glProgramUniform4fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: &[GLfloat],
    ) {
        assert!((count * 4) <= value.len() as GLsizei);
        (self.entry().glProgramUniform4fv)(program, location, count, value.as_ptr())
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
    unsafe fn glProgramUniform4iv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: &[GLint],
    ) {
        assert!((count * 4) <= value.len() as GLsizei);
        (self.entry().glProgramUniform4iv)(program, location, count, value.as_ptr())
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
    unsafe fn glProgramUniform4uiv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: &[GLuint],
    ) {
        assert!((count * 4) <= value.len() as GLsizei);
        (self.entry().glProgramUniform4uiv)(program, location, count, value.as_ptr())
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
    unsafe fn glProgramUniformMatrix2fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: &[GLfloat],
    ) {
        assert!((count * (2 * 2)) <= value.len() as GLsizei);
        (self.entry().glProgramUniformMatrix2fv)(
            program,
            location,
            count,
            transpose,
            value.as_ptr(),
        )
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
    unsafe fn glProgramUniformMatrix2x3fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: &[GLfloat],
    ) {
        assert!((count * (2 * 3)) <= value.len() as GLsizei);
        (self.entry().glProgramUniformMatrix2x3fv)(
            program,
            location,
            count,
            transpose,
            value.as_ptr(),
        )
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
    unsafe fn glProgramUniformMatrix2x4fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: &[GLfloat],
    ) {
        assert!((count * (2 * 4)) <= value.len() as GLsizei);
        (self.entry().glProgramUniformMatrix2x4fv)(
            program,
            location,
            count,
            transpose,
            value.as_ptr(),
        )
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
    unsafe fn glProgramUniformMatrix3fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: &[GLfloat],
    ) {
        assert!((count * (3 * 3)) <= value.len() as GLsizei);
        (self.entry().glProgramUniformMatrix3fv)(
            program,
            location,
            count,
            transpose,
            value.as_ptr(),
        )
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
    unsafe fn glProgramUniformMatrix3x2fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: &[GLfloat],
    ) {
        assert!((count * (3 * 2)) <= value.len() as GLsizei);
        (self.entry().glProgramUniformMatrix3x2fv)(
            program,
            location,
            count,
            transpose,
            value.as_ptr(),
        )
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
    unsafe fn glProgramUniformMatrix3x4fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: &[GLfloat],
    ) {
        assert!((count * (3 * 4)) <= value.len() as GLsizei);
        (self.entry().glProgramUniformMatrix3x4fv)(
            program,
            location,
            count,
            transpose,
            value.as_ptr(),
        )
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
    unsafe fn glProgramUniformMatrix4fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: &[GLfloat],
    ) {
        assert!((count * (4 * 4)) <= value.len() as GLsizei);
        (self.entry().glProgramUniformMatrix4fv)(
            program,
            location,
            count,
            transpose,
            value.as_ptr(),
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
    unsafe fn glProgramUniformMatrix4x2fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: &[GLfloat],
    ) {
        assert!((count * (4 * 2)) <= value.len() as GLsizei);
        (self.entry().glProgramUniformMatrix4x2fv)(
            program,
            location,
            count,
            transpose,
            value.as_ptr(),
        )
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
    unsafe fn glProgramUniformMatrix4x3fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: &[GLfloat],
    ) {
        assert!((count * (4 * 3)) <= value.len() as GLsizei);
        (self.entry().glProgramUniformMatrix4x3fv)(
            program,
            location,
            count,
            transpose,
            value.as_ptr(),
        )
    }
    unsafe fn glProvokingVertex(&self, _mode: GLenum) {
        (self.entry().glProvokingVertex)(_mode)
    }
    unsafe fn glPushDebugGroup(
        &self,
        _source: GLenum,
        _id: GLuint,
        _length: GLsizei,
        _message: *const GLchar,
    ) {
        (self.entry().glPushDebugGroup)(_source, _id, _length, _message)
    }
    unsafe fn glQueryCounter(&self, _id: GLuint, _target: GLenum) {
        (self.entry().glQueryCounter)(_id, _target)
    }
    unsafe fn glReadBuffer(&self, _src: GLenum) {
        (self.entry().glReadBuffer)(_src)
    }
    unsafe fn glReadPixels(
        &self,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _format: GLenum,
        _type: GLenum,
        _pixels: *mut std::os::raw::c_void,
    ) {
        (self.entry().glReadPixels)(_x, _y, _width, _height, _format, _type, _pixels)
    }
    unsafe fn glReleaseShaderCompiler(&self) {
        (self.entry().glReleaseShaderCompiler)()
    }
    unsafe fn glRenderbufferStorage(
        &self,
        _target: GLenum,
        _internalformat: GLenum,
        _width: GLsizei,
        _height: GLsizei,
    ) {
        (self.entry().glRenderbufferStorage)(_target, _internalformat, _width, _height)
    }
    unsafe fn glRenderbufferStorageMultisample(
        &self,
        _target: GLenum,
        _samples: GLsizei,
        _internalformat: GLenum,
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
    unsafe fn glResumeTransformFeedback(&self) {
        (self.entry().glResumeTransformFeedback)()
    }
    unsafe fn glSampleCoverage(&self, _value: GLfloat, _invert: GLboolean) {
        (self.entry().glSampleCoverage)(_value, _invert)
    }
    unsafe fn glSampleMaski(&self, _maskNumber: GLuint, _mask: GLbitfield) {
        (self.entry().glSampleMaski)(_maskNumber, _mask)
    }
    unsafe fn glSamplerParameterIiv(&self, _sampler: GLuint, _pname: GLenum, _param: *const GLint) {
        (self.entry().glSamplerParameterIiv)(_sampler, _pname, _param)
    }
    unsafe fn glSamplerParameterIuiv(
        &self,
        _sampler: GLuint,
        _pname: GLenum,
        _param: *const GLuint,
    ) {
        (self.entry().glSamplerParameterIuiv)(_sampler, _pname, _param)
    }
    unsafe fn glSamplerParameterf(&self, _sampler: GLuint, _pname: GLenum, _param: GLfloat) {
        (self.entry().glSamplerParameterf)(_sampler, _pname, _param)
    }
    unsafe fn glSamplerParameterfv(
        &self,
        _sampler: GLuint,
        _pname: GLenum,
        _param: *const GLfloat,
    ) {
        (self.entry().glSamplerParameterfv)(_sampler, _pname, _param)
    }
    unsafe fn glSamplerParameteri(&self, _sampler: GLuint, _pname: GLenum, _param: GLint) {
        (self.entry().glSamplerParameteri)(_sampler, _pname, _param)
    }
    unsafe fn glSamplerParameteriv(&self, _sampler: GLuint, _pname: GLenum, _param: *const GLint) {
        (self.entry().glSamplerParameteriv)(_sampler, _pname, _param)
    }
    unsafe fn glScissor(&self, _x: GLint, _y: GLint, _width: GLsizei, _height: GLsizei) {
        (self.entry().glScissor)(_x, _y, _width, _height)
    }
    unsafe fn glScissorArrayv(&self, _first: GLuint, _count: GLsizei, _v: *const GLint) {
        (self.entry().glScissorArrayv)(_first, _count, _v)
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
    unsafe fn glScissorIndexedv(&self, _index: GLuint, _v: *const GLint) {
        (self.entry().glScissorIndexedv)(_index, _v)
    }
    unsafe fn glSecondaryColorP3ui(&self, _type: GLenum, _color: GLuint) {
        (self.entry().glSecondaryColorP3ui)(_type, _color)
    }
    unsafe fn glSecondaryColorP3uiv(&self, _type: GLenum, _color: *const GLuint) {
        (self.entry().glSecondaryColorP3uiv)(_type, _color)
    }
    unsafe fn glShaderBinary(
        &self,
        _count: GLsizei,
        _shaders: *const GLuint,
        _binaryFormat: GLenum,
        _binary: *const std::os::raw::c_void,
        _length: GLsizei,
    ) {
        (self.entry().glShaderBinary)(_count, _shaders, _binaryFormat, _binary, _length)
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
    unsafe fn glStencilFunc(&self, _func: GLenum, _ref: GLint, _mask: GLuint) {
        (self.entry().glStencilFunc)(_func, _ref, _mask)
    }
    unsafe fn glStencilFuncSeparate(
        &self,
        _face: GLenum,
        _func: GLenum,
        _ref: GLint,
        _mask: GLuint,
    ) {
        (self.entry().glStencilFuncSeparate)(_face, _func, _ref, _mask)
    }
    unsafe fn glStencilMask(&self, _mask: GLuint) {
        (self.entry().glStencilMask)(_mask)
    }
    unsafe fn glStencilMaskSeparate(&self, _face: GLenum, _mask: GLuint) {
        (self.entry().glStencilMaskSeparate)(_face, _mask)
    }
    unsafe fn glStencilOp(&self, _fail: GLenum, _zfail: GLenum, _zpass: GLenum) {
        (self.entry().glStencilOp)(_fail, _zfail, _zpass)
    }
    unsafe fn glStencilOpSeparate(
        &self,
        _face: GLenum,
        _sfail: GLenum,
        _dpfail: GLenum,
        _dppass: GLenum,
    ) {
        (self.entry().glStencilOpSeparate)(_face, _sfail, _dpfail, _dppass)
    }
    unsafe fn glTexBuffer(&self, _target: GLenum, _internalformat: GLenum, _buffer: GLuint) {
        (self.entry().glTexBuffer)(_target, _internalformat, _buffer)
    }
    unsafe fn glTexBufferRange(
        &self,
        _target: GLenum,
        _internalformat: GLenum,
        _buffer: GLuint,
        _offset: GLintptr,
        _size: GLsizeiptr,
    ) {
        (self.entry().glTexBufferRange)(_target, _internalformat, _buffer, _offset, _size)
    }
    unsafe fn glTexCoordP1ui(&self, _type: GLenum, _coords: GLuint) {
        (self.entry().glTexCoordP1ui)(_type, _coords)
    }
    unsafe fn glTexCoordP1uiv(&self, _type: GLenum, _coords: *const GLuint) {
        (self.entry().glTexCoordP1uiv)(_type, _coords)
    }
    unsafe fn glTexCoordP2ui(&self, _type: GLenum, _coords: GLuint) {
        (self.entry().glTexCoordP2ui)(_type, _coords)
    }
    unsafe fn glTexCoordP2uiv(&self, _type: GLenum, _coords: *const GLuint) {
        (self.entry().glTexCoordP2uiv)(_type, _coords)
    }
    unsafe fn glTexCoordP3ui(&self, _type: GLenum, _coords: GLuint) {
        (self.entry().glTexCoordP3ui)(_type, _coords)
    }
    unsafe fn glTexCoordP3uiv(&self, _type: GLenum, _coords: *const GLuint) {
        (self.entry().glTexCoordP3uiv)(_type, _coords)
    }
    unsafe fn glTexCoordP4ui(&self, _type: GLenum, _coords: GLuint) {
        (self.entry().glTexCoordP4ui)(_type, _coords)
    }
    unsafe fn glTexCoordP4uiv(&self, _type: GLenum, _coords: *const GLuint) {
        (self.entry().glTexCoordP4uiv)(_type, _coords)
    }
    unsafe fn glTexImage1D(
        &self,
        _target: GLenum,
        _level: GLint,
        _internalformat: GLint,
        _width: GLsizei,
        _border: GLint,
        _format: GLenum,
        _type: GLenum,
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
    unsafe fn glTexImage2D(
        &self,
        _target: GLenum,
        _level: GLint,
        _internalformat: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _border: GLint,
        _format: GLenum,
        _type: GLenum,
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
    unsafe fn glTexImage2DMultisample(
        &self,
        _target: GLenum,
        _samples: GLsizei,
        _internalformat: GLenum,
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
    unsafe fn glTexImage3D(
        &self,
        _target: GLenum,
        _level: GLint,
        _internalformat: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _depth: GLsizei,
        _border: GLint,
        _format: GLenum,
        _type: GLenum,
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
    unsafe fn glTexImage3DMultisample(
        &self,
        _target: GLenum,
        _samples: GLsizei,
        _internalformat: GLenum,
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
    unsafe fn glTexParameterIiv(&self, _target: GLenum, _pname: GLenum, _params: *const GLint) {
        (self.entry().glTexParameterIiv)(_target, _pname, _params)
    }
    unsafe fn glTexParameterIuiv(&self, _target: GLenum, _pname: GLenum, _params: *const GLuint) {
        (self.entry().glTexParameterIuiv)(_target, _pname, _params)
    }
    unsafe fn glTexParameterf(&self, _target: GLenum, _pname: GLenum, _param: GLfloat) {
        (self.entry().glTexParameterf)(_target, _pname, _param)
    }
    unsafe fn glTexParameterfv(&self, _target: GLenum, _pname: GLenum, _params: *const GLfloat) {
        (self.entry().glTexParameterfv)(_target, _pname, _params)
    }
    unsafe fn glTexParameteri(&self, _target: GLenum, _pname: GLenum, _param: GLint) {
        (self.entry().glTexParameteri)(_target, _pname, _param)
    }
    unsafe fn glTexParameteriv(&self, _target: GLenum, _pname: GLenum, _params: *const GLint) {
        (self.entry().glTexParameteriv)(_target, _pname, _params)
    }
    unsafe fn glTexStorage1D(
        &self,
        _target: GLenum,
        _levels: GLsizei,
        _internalformat: GLenum,
        _width: GLsizei,
    ) {
        (self.entry().glTexStorage1D)(_target, _levels, _internalformat, _width)
    }
    unsafe fn glTexStorage2D(
        &self,
        _target: GLenum,
        _levels: GLsizei,
        _internalformat: GLenum,
        _width: GLsizei,
        _height: GLsizei,
    ) {
        (self.entry().glTexStorage2D)(_target, _levels, _internalformat, _width, _height)
    }
    unsafe fn glTexStorage2DMultisample(
        &self,
        _target: GLenum,
        _samples: GLsizei,
        _internalformat: GLenum,
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
    unsafe fn glTexStorage3D(
        &self,
        _target: GLenum,
        _levels: GLsizei,
        _internalformat: GLenum,
        _width: GLsizei,
        _height: GLsizei,
        _depth: GLsizei,
    ) {
        (self.entry().glTexStorage3D)(_target, _levels, _internalformat, _width, _height, _depth)
    }
    unsafe fn glTexStorage3DMultisample(
        &self,
        _target: GLenum,
        _samples: GLsizei,
        _internalformat: GLenum,
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
    unsafe fn glTexSubImage1D(
        &self,
        _target: GLenum,
        _level: GLint,
        _xoffset: GLint,
        _width: GLsizei,
        _format: GLenum,
        _type: GLenum,
        _pixels: *const std::os::raw::c_void,
    ) {
        (self.entry().glTexSubImage1D)(_target, _level, _xoffset, _width, _format, _type, _pixels)
    }
    unsafe fn glTexSubImage2D(
        &self,
        _target: GLenum,
        _level: GLint,
        _xoffset: GLint,
        _yoffset: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _format: GLenum,
        _type: GLenum,
        _pixels: *const std::os::raw::c_void,
    ) {
        (self.entry().glTexSubImage2D)(
            _target, _level, _xoffset, _yoffset, _width, _height, _format, _type, _pixels,
        )
    }
    unsafe fn glTexSubImage3D(
        &self,
        _target: GLenum,
        _level: GLint,
        _xoffset: GLint,
        _yoffset: GLint,
        _zoffset: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _depth: GLsizei,
        _format: GLenum,
        _type: GLenum,
        _pixels: *const std::os::raw::c_void,
    ) {
        (self.entry().glTexSubImage3D)(
            _target, _level, _xoffset, _yoffset, _zoffset, _width, _height, _depth, _format, _type,
            _pixels,
        )
    }
    unsafe fn glTextureView(
        &self,
        _texture: GLuint,
        _target: GLenum,
        _origtexture: GLuint,
        _internalformat: GLenum,
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
    unsafe fn glTransformFeedbackVaryings(
        &self,
        _program: GLuint,
        _count: GLsizei,
        _varyings: *const *const GLchar,
        _bufferMode: GLenum,
    ) {
        (self.entry().glTransformFeedbackVaryings)(_program, _count, _varyings, _bufferMode)
    }
    unsafe fn glUniform1d(&self, _location: GLint, _x: GLdouble) {
        (self.entry().glUniform1d)(_location, _x)
    }
    unsafe fn glUniform1dv(&self, _location: GLint, _count: GLsizei, _value: *const GLdouble) {
        (self.entry().glUniform1dv)(_location, _count, _value)
    }
    unsafe fn glUniform1f(&self, _location: GLint, _v0: GLfloat) {
        (self.entry().glUniform1f)(_location, _v0)
    }
    unsafe fn glUniform1fv(&self, location: GLint, count: GLsizei, value: &[GLfloat]) {
        assert!(count <= value.len() as GLsizei);
        (self.entry().glUniform1fv)(location, count, value.as_ptr())
    }
    unsafe fn glUniform1i(&self, _location: GLint, _v0: GLint) {
        (self.entry().glUniform1i)(_location, _v0)
    }
    unsafe fn glUniform1iv(&self, location: GLint, count: GLsizei, value: &[GLint]) {
        assert!(count <= value.len() as GLsizei);
        (self.entry().glUniform1iv)(location, count, value.as_ptr())
    }
    unsafe fn glUniform1ui(&self, _location: GLint, _v0: GLuint) {
        (self.entry().glUniform1ui)(_location, _v0)
    }
    unsafe fn glUniform1uiv(&self, location: GLint, count: GLsizei, value: &[GLuint]) {
        assert!(count <= value.len() as GLsizei);
        (self.entry().glUniform1uiv)(location, count, value.as_ptr())
    }
    unsafe fn glUniform2d(&self, _location: GLint, _x: GLdouble, _y: GLdouble) {
        (self.entry().glUniform2d)(_location, _x, _y)
    }
    unsafe fn glUniform2dv(&self, _location: GLint, _count: GLsizei, _value: *const GLdouble) {
        (self.entry().glUniform2dv)(_location, _count, _value)
    }
    unsafe fn glUniform2f(&self, _location: GLint, _v0: GLfloat, _v1: GLfloat) {
        (self.entry().glUniform2f)(_location, _v0, _v1)
    }
    unsafe fn glUniform2fv(&self, location: GLint, count: GLsizei, value: &[GLfloat]) {
        assert!((count * 2) <= value.len() as GLsizei);
        (self.entry().glUniform2fv)(location, count, value.as_ptr())
    }
    unsafe fn glUniform2i(&self, _location: GLint, _v0: GLint, _v1: GLint) {
        (self.entry().glUniform2i)(_location, _v0, _v1)
    }
    unsafe fn glUniform2iv(&self, location: GLint, count: GLsizei, value: &[GLint]) {
        assert!((count * 2) <= value.len() as GLsizei);
        (self.entry().glUniform2iv)(location, count, value.as_ptr())
    }
    unsafe fn glUniform2ui(&self, _location: GLint, _v0: GLuint, _v1: GLuint) {
        (self.entry().glUniform2ui)(_location, _v0, _v1)
    }
    unsafe fn glUniform2uiv(&self, location: GLint, count: GLsizei, value: &[GLuint]) {
        assert!((count * 2) <= value.len() as GLsizei);
        (self.entry().glUniform2uiv)(location, count, value.as_ptr())
    }
    unsafe fn glUniform3d(&self, _location: GLint, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glUniform3d)(_location, _x, _y, _z)
    }
    unsafe fn glUniform3dv(&self, _location: GLint, _count: GLsizei, _value: *const GLdouble) {
        (self.entry().glUniform3dv)(_location, _count, _value)
    }
    unsafe fn glUniform3f(&self, _location: GLint, _v0: GLfloat, _v1: GLfloat, _v2: GLfloat) {
        (self.entry().glUniform3f)(_location, _v0, _v1, _v2)
    }
    unsafe fn glUniform3fv(&self, location: GLint, count: GLsizei, value: &[GLfloat]) {
        assert!((count * 3) <= value.len() as GLsizei);
        (self.entry().glUniform3fv)(location, count, value.as_ptr())
    }
    unsafe fn glUniform3i(&self, _location: GLint, _v0: GLint, _v1: GLint, _v2: GLint) {
        (self.entry().glUniform3i)(_location, _v0, _v1, _v2)
    }
    unsafe fn glUniform3iv(&self, location: GLint, count: GLsizei, value: &[GLint]) {
        assert!((count * 3) <= value.len() as GLsizei);
        (self.entry().glUniform3iv)(location, count, value.as_ptr())
    }
    unsafe fn glUniform3ui(&self, _location: GLint, _v0: GLuint, _v1: GLuint, _v2: GLuint) {
        (self.entry().glUniform3ui)(_location, _v0, _v1, _v2)
    }
    unsafe fn glUniform3uiv(&self, location: GLint, count: GLsizei, value: &[GLuint]) {
        assert!((count * 3) <= value.len() as GLsizei);
        (self.entry().glUniform3uiv)(location, count, value.as_ptr())
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
    unsafe fn glUniform4dv(&self, _location: GLint, _count: GLsizei, _value: *const GLdouble) {
        (self.entry().glUniform4dv)(_location, _count, _value)
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
    unsafe fn glUniform4fv(&self, location: GLint, count: GLsizei, value: &[GLfloat]) {
        assert!((count * 4) <= value.len() as GLsizei);
        (self.entry().glUniform4fv)(location, count, value.as_ptr())
    }
    unsafe fn glUniform4i(&self, _location: GLint, _v0: GLint, _v1: GLint, _v2: GLint, _v3: GLint) {
        (self.entry().glUniform4i)(_location, _v0, _v1, _v2, _v3)
    }
    unsafe fn glUniform4iv(&self, location: GLint, count: GLsizei, value: &[GLint]) {
        assert!((count * 4) <= value.len() as GLsizei);
        (self.entry().glUniform4iv)(location, count, value.as_ptr())
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
    unsafe fn glUniform4uiv(&self, location: GLint, count: GLsizei, value: &[GLuint]) {
        assert!((count * 4) <= value.len() as GLsizei);
        (self.entry().glUniform4uiv)(location, count, value.as_ptr())
    }
    unsafe fn glUniformBlockBinding(
        &self,
        _program: GLuint,
        _uniformBlockIndex: GLuint,
        _uniformBlockBinding: GLuint,
    ) {
        (self.entry().glUniformBlockBinding)(_program, _uniformBlockIndex, _uniformBlockBinding)
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
    unsafe fn glUniformMatrix2fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: &[GLfloat],
    ) {
        assert!((count * (2 * 2)) <= value.len() as GLsizei);
        (self.entry().glUniformMatrix2fv)(location, count, transpose, value.as_ptr())
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
    unsafe fn glUniformMatrix2x3fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: &[GLfloat],
    ) {
        assert!((count * (2 * 3)) <= value.len() as GLsizei);
        (self.entry().glUniformMatrix2x3fv)(location, count, transpose, value.as_ptr())
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
    unsafe fn glUniformMatrix2x4fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: &[GLfloat],
    ) {
        assert!((count * (2 * 4)) <= value.len() as GLsizei);
        (self.entry().glUniformMatrix2x4fv)(location, count, transpose, value.as_ptr())
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
    unsafe fn glUniformMatrix3fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: &[GLfloat],
    ) {
        assert!((count * (3 * 3)) <= value.len() as GLsizei);
        (self.entry().glUniformMatrix3fv)(location, count, transpose, value.as_ptr())
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
    unsafe fn glUniformMatrix3x2fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: &[GLfloat],
    ) {
        assert!((count * (3 * 2)) <= value.len() as GLsizei);
        (self.entry().glUniformMatrix3x2fv)(location, count, transpose, value.as_ptr())
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
    unsafe fn glUniformMatrix3x4fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: &[GLfloat],
    ) {
        assert!((count * (3 * 4)) <= value.len() as GLsizei);
        (self.entry().glUniformMatrix3x4fv)(location, count, transpose, value.as_ptr())
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
    unsafe fn glUniformMatrix4fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: &[GLfloat],
    ) {
        assert!((count * (4 * 4)) <= value.len() as GLsizei);
        (self.entry().glUniformMatrix4fv)(location, count, transpose, value.as_ptr())
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
    unsafe fn glUniformMatrix4x2fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: &[GLfloat],
    ) {
        assert!((count * (2 * 4)) <= value.len() as GLsizei);
        (self.entry().glUniformMatrix4x2fv)(location, count, transpose, value.as_ptr())
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
    unsafe fn glUniformMatrix4x3fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: &[GLfloat],
    ) {
        assert!((count * (4 * 3)) <= value.len() as GLsizei);
        (self.entry().glUniformMatrix4x3fv)(location, count, transpose, value.as_ptr())
    }
    unsafe fn glUniformSubroutinesuiv(
        &self,
        _shadertype: GLenum,
        _count: GLsizei,
        _indices: *const GLuint,
    ) {
        (self.entry().glUniformSubroutinesuiv)(_shadertype, _count, _indices)
    }
    unsafe fn glUnmapBuffer(&self, _target: GLenum) -> GLboolean {
        (self.entry().glUnmapBuffer)(_target)
    }
    unsafe fn glUseProgram(&self, _program: GLuint) {
        (self.entry().glUseProgram)(_program)
    }
    unsafe fn glUseProgramStages(&self, _pipeline: GLuint, _stages: GLbitfield, _program: GLuint) {
        (self.entry().glUseProgramStages)(_pipeline, _stages, _program)
    }
    unsafe fn glValidateProgram(&self, _program: GLuint) {
        (self.entry().glValidateProgram)(_program)
    }
    unsafe fn glValidateProgramPipeline(&self, _pipeline: GLuint) {
        (self.entry().glValidateProgramPipeline)(_pipeline)
    }
    unsafe fn glVertexAttrib1d(&self, _index: GLuint, _x: GLdouble) {
        (self.entry().glVertexAttrib1d)(_index, _x)
    }
    unsafe fn glVertexAttrib1dv(&self, _index: GLuint, _v: *const GLdouble) {
        (self.entry().glVertexAttrib1dv)(_index, _v)
    }
    unsafe fn glVertexAttrib1f(&self, _index: GLuint, _x: GLfloat) {
        (self.entry().glVertexAttrib1f)(_index, _x)
    }
    unsafe fn glVertexAttrib1fv(&self, _index: GLuint, _v: *const GLfloat) {
        (self.entry().glVertexAttrib1fv)(_index, _v)
    }
    unsafe fn glVertexAttrib1s(&self, _index: GLuint, _x: GLshort) {
        (self.entry().glVertexAttrib1s)(_index, _x)
    }
    unsafe fn glVertexAttrib1sv(&self, _index: GLuint, _v: *const GLshort) {
        (self.entry().glVertexAttrib1sv)(_index, _v)
    }
    unsafe fn glVertexAttrib2d(&self, _index: GLuint, _x: GLdouble, _y: GLdouble) {
        (self.entry().glVertexAttrib2d)(_index, _x, _y)
    }
    unsafe fn glVertexAttrib2dv(&self, _index: GLuint, _v: *const GLdouble) {
        (self.entry().glVertexAttrib2dv)(_index, _v)
    }
    unsafe fn glVertexAttrib2f(&self, _index: GLuint, _x: GLfloat, _y: GLfloat) {
        (self.entry().glVertexAttrib2f)(_index, _x, _y)
    }
    unsafe fn glVertexAttrib2fv(&self, _index: GLuint, _v: *const GLfloat) {
        (self.entry().glVertexAttrib2fv)(_index, _v)
    }
    unsafe fn glVertexAttrib2s(&self, _index: GLuint, _x: GLshort, _y: GLshort) {
        (self.entry().glVertexAttrib2s)(_index, _x, _y)
    }
    unsafe fn glVertexAttrib2sv(&self, _index: GLuint, _v: *const GLshort) {
        (self.entry().glVertexAttrib2sv)(_index, _v)
    }
    unsafe fn glVertexAttrib3d(&self, _index: GLuint, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glVertexAttrib3d)(_index, _x, _y, _z)
    }
    unsafe fn glVertexAttrib3dv(&self, _index: GLuint, _v: *const GLdouble) {
        (self.entry().glVertexAttrib3dv)(_index, _v)
    }
    unsafe fn glVertexAttrib3f(&self, _index: GLuint, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glVertexAttrib3f)(_index, _x, _y, _z)
    }
    unsafe fn glVertexAttrib3fv(&self, _index: GLuint, _v: *const GLfloat) {
        (self.entry().glVertexAttrib3fv)(_index, _v)
    }
    unsafe fn glVertexAttrib3s(&self, _index: GLuint, _x: GLshort, _y: GLshort, _z: GLshort) {
        (self.entry().glVertexAttrib3s)(_index, _x, _y, _z)
    }
    unsafe fn glVertexAttrib3sv(&self, _index: GLuint, _v: *const GLshort) {
        (self.entry().glVertexAttrib3sv)(_index, _v)
    }
    unsafe fn glVertexAttrib4Nbv(&self, _index: GLuint, _v: *const GLbyte) {
        (self.entry().glVertexAttrib4Nbv)(_index, _v)
    }
    unsafe fn glVertexAttrib4Niv(&self, _index: GLuint, _v: *const GLint) {
        (self.entry().glVertexAttrib4Niv)(_index, _v)
    }
    unsafe fn glVertexAttrib4Nsv(&self, _index: GLuint, _v: *const GLshort) {
        (self.entry().glVertexAttrib4Nsv)(_index, _v)
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
    unsafe fn glVertexAttrib4Nubv(&self, _index: GLuint, _v: *const GLubyte) {
        (self.entry().glVertexAttrib4Nubv)(_index, _v)
    }
    unsafe fn glVertexAttrib4Nuiv(&self, _index: GLuint, _v: *const GLuint) {
        (self.entry().glVertexAttrib4Nuiv)(_index, _v)
    }
    unsafe fn glVertexAttrib4Nusv(&self, _index: GLuint, _v: *const GLushort) {
        (self.entry().glVertexAttrib4Nusv)(_index, _v)
    }
    unsafe fn glVertexAttrib4bv(&self, _index: GLuint, _v: *const GLbyte) {
        (self.entry().glVertexAttrib4bv)(_index, _v)
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
    unsafe fn glVertexAttrib4dv(&self, _index: GLuint, _v: *const GLdouble) {
        (self.entry().glVertexAttrib4dv)(_index, _v)
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
    unsafe fn glVertexAttrib4fv(&self, _index: GLuint, _v: *const GLfloat) {
        (self.entry().glVertexAttrib4fv)(_index, _v)
    }
    unsafe fn glVertexAttrib4iv(&self, _index: GLuint, _v: *const GLint) {
        (self.entry().glVertexAttrib4iv)(_index, _v)
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
    unsafe fn glVertexAttrib4sv(&self, _index: GLuint, _v: *const GLshort) {
        (self.entry().glVertexAttrib4sv)(_index, _v)
    }
    unsafe fn glVertexAttrib4ubv(&self, _index: GLuint, _v: *const GLubyte) {
        (self.entry().glVertexAttrib4ubv)(_index, _v)
    }
    unsafe fn glVertexAttrib4uiv(&self, _index: GLuint, _v: *const GLuint) {
        (self.entry().glVertexAttrib4uiv)(_index, _v)
    }
    unsafe fn glVertexAttrib4usv(&self, _index: GLuint, _v: *const GLushort) {
        (self.entry().glVertexAttrib4usv)(_index, _v)
    }
    unsafe fn glVertexAttribBinding(&self, _attribindex: GLuint, _bindingindex: GLuint) {
        (self.entry().glVertexAttribBinding)(_attribindex, _bindingindex)
    }
    unsafe fn glVertexAttribDivisor(&self, _index: GLuint, _divisor: GLuint) {
        (self.entry().glVertexAttribDivisor)(_index, _divisor)
    }
    unsafe fn glVertexAttribFormat(
        &self,
        _attribindex: GLuint,
        _size: GLint,
        _type: GLenum,
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
    unsafe fn glVertexAttribI1i(&self, _index: GLuint, _x: GLint) {
        (self.entry().glVertexAttribI1i)(_index, _x)
    }
    unsafe fn glVertexAttribI1iv(&self, _index: GLuint, _v: *const GLint) {
        (self.entry().glVertexAttribI1iv)(_index, _v)
    }
    unsafe fn glVertexAttribI1ui(&self, _index: GLuint, _x: GLuint) {
        (self.entry().glVertexAttribI1ui)(_index, _x)
    }
    unsafe fn glVertexAttribI1uiv(&self, _index: GLuint, _v: *const GLuint) {
        (self.entry().glVertexAttribI1uiv)(_index, _v)
    }
    unsafe fn glVertexAttribI2i(&self, _index: GLuint, _x: GLint, _y: GLint) {
        (self.entry().glVertexAttribI2i)(_index, _x, _y)
    }
    unsafe fn glVertexAttribI2iv(&self, _index: GLuint, _v: *const GLint) {
        (self.entry().glVertexAttribI2iv)(_index, _v)
    }
    unsafe fn glVertexAttribI2ui(&self, _index: GLuint, _x: GLuint, _y: GLuint) {
        (self.entry().glVertexAttribI2ui)(_index, _x, _y)
    }
    unsafe fn glVertexAttribI2uiv(&self, _index: GLuint, _v: *const GLuint) {
        (self.entry().glVertexAttribI2uiv)(_index, _v)
    }
    unsafe fn glVertexAttribI3i(&self, _index: GLuint, _x: GLint, _y: GLint, _z: GLint) {
        (self.entry().glVertexAttribI3i)(_index, _x, _y, _z)
    }
    unsafe fn glVertexAttribI3iv(&self, _index: GLuint, _v: *const GLint) {
        (self.entry().glVertexAttribI3iv)(_index, _v)
    }
    unsafe fn glVertexAttribI3ui(&self, _index: GLuint, _x: GLuint, _y: GLuint, _z: GLuint) {
        (self.entry().glVertexAttribI3ui)(_index, _x, _y, _z)
    }
    unsafe fn glVertexAttribI3uiv(&self, _index: GLuint, _v: *const GLuint) {
        (self.entry().glVertexAttribI3uiv)(_index, _v)
    }
    unsafe fn glVertexAttribI4bv(&self, _index: GLuint, _v: *const GLbyte) {
        (self.entry().glVertexAttribI4bv)(_index, _v)
    }
    unsafe fn glVertexAttribI4i(&self, _index: GLuint, _x: GLint, _y: GLint, _z: GLint, _w: GLint) {
        (self.entry().glVertexAttribI4i)(_index, _x, _y, _z, _w)
    }
    unsafe fn glVertexAttribI4iv(&self, _index: GLuint, _v: *const GLint) {
        (self.entry().glVertexAttribI4iv)(_index, _v)
    }
    unsafe fn glVertexAttribI4sv(&self, _index: GLuint, _v: *const GLshort) {
        (self.entry().glVertexAttribI4sv)(_index, _v)
    }
    unsafe fn glVertexAttribI4ubv(&self, _index: GLuint, _v: *const GLubyte) {
        (self.entry().glVertexAttribI4ubv)(_index, _v)
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
    unsafe fn glVertexAttribI4uiv(&self, _index: GLuint, _v: *const GLuint) {
        (self.entry().glVertexAttribI4uiv)(_index, _v)
    }
    unsafe fn glVertexAttribI4usv(&self, _index: GLuint, _v: *const GLushort) {
        (self.entry().glVertexAttribI4usv)(_index, _v)
    }
    unsafe fn glVertexAttribIFormat(
        &self,
        _attribindex: GLuint,
        _size: GLint,
        _type: GLenum,
        _relativeoffset: GLuint,
    ) {
        (self.entry().glVertexAttribIFormat)(_attribindex, _size, _type, _relativeoffset)
    }
    unsafe fn glVertexAttribIPointer(
        &self,
        _index: GLuint,
        _size: GLint,
        _type: GLenum,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glVertexAttribIPointer)(_index, _size, _type, _stride, _pointer)
    }
    unsafe fn glVertexAttribL1d(&self, _index: GLuint, _x: GLdouble) {
        (self.entry().glVertexAttribL1d)(_index, _x)
    }
    unsafe fn glVertexAttribL1dv(&self, _index: GLuint, _v: *const GLdouble) {
        (self.entry().glVertexAttribL1dv)(_index, _v)
    }
    unsafe fn glVertexAttribL2d(&self, _index: GLuint, _x: GLdouble, _y: GLdouble) {
        (self.entry().glVertexAttribL2d)(_index, _x, _y)
    }
    unsafe fn glVertexAttribL2dv(&self, _index: GLuint, _v: *const GLdouble) {
        (self.entry().glVertexAttribL2dv)(_index, _v)
    }
    unsafe fn glVertexAttribL3d(&self, _index: GLuint, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glVertexAttribL3d)(_index, _x, _y, _z)
    }
    unsafe fn glVertexAttribL3dv(&self, _index: GLuint, _v: *const GLdouble) {
        (self.entry().glVertexAttribL3dv)(_index, _v)
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
    unsafe fn glVertexAttribL4dv(&self, _index: GLuint, _v: *const GLdouble) {
        (self.entry().glVertexAttribL4dv)(_index, _v)
    }
    unsafe fn glVertexAttribLFormat(
        &self,
        _attribindex: GLuint,
        _size: GLint,
        _type: GLenum,
        _relativeoffset: GLuint,
    ) {
        (self.entry().glVertexAttribLFormat)(_attribindex, _size, _type, _relativeoffset)
    }
    unsafe fn glVertexAttribLPointer(
        &self,
        _index: GLuint,
        _size: GLint,
        _type: GLenum,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glVertexAttribLPointer)(_index, _size, _type, _stride, _pointer)
    }
    unsafe fn glVertexAttribP1ui(
        &self,
        _index: GLuint,
        _type: GLenum,
        _normalized: GLboolean,
        _value: GLuint,
    ) {
        (self.entry().glVertexAttribP1ui)(_index, _type, _normalized, _value)
    }
    unsafe fn glVertexAttribP1uiv(
        &self,
        _index: GLuint,
        _type: GLenum,
        _normalized: GLboolean,
        _value: *const GLuint,
    ) {
        (self.entry().glVertexAttribP1uiv)(_index, _type, _normalized, _value)
    }
    unsafe fn glVertexAttribP2ui(
        &self,
        _index: GLuint,
        _type: GLenum,
        _normalized: GLboolean,
        _value: GLuint,
    ) {
        (self.entry().glVertexAttribP2ui)(_index, _type, _normalized, _value)
    }
    unsafe fn glVertexAttribP2uiv(
        &self,
        _index: GLuint,
        _type: GLenum,
        _normalized: GLboolean,
        _value: *const GLuint,
    ) {
        (self.entry().glVertexAttribP2uiv)(_index, _type, _normalized, _value)
    }
    unsafe fn glVertexAttribP3ui(
        &self,
        _index: GLuint,
        _type: GLenum,
        _normalized: GLboolean,
        _value: GLuint,
    ) {
        (self.entry().glVertexAttribP3ui)(_index, _type, _normalized, _value)
    }
    unsafe fn glVertexAttribP3uiv(
        &self,
        _index: GLuint,
        _type: GLenum,
        _normalized: GLboolean,
        _value: *const GLuint,
    ) {
        (self.entry().glVertexAttribP3uiv)(_index, _type, _normalized, _value)
    }
    unsafe fn glVertexAttribP4ui(
        &self,
        _index: GLuint,
        _type: GLenum,
        _normalized: GLboolean,
        _value: GLuint,
    ) {
        (self.entry().glVertexAttribP4ui)(_index, _type, _normalized, _value)
    }
    unsafe fn glVertexAttribP4uiv(
        &self,
        _index: GLuint,
        _type: GLenum,
        _normalized: GLboolean,
        _value: *const GLuint,
    ) {
        (self.entry().glVertexAttribP4uiv)(_index, _type, _normalized, _value)
    }
    unsafe fn glVertexAttribPointer(
        &self,
        _index: GLuint,
        _size: GLint,
        _type: GLenum,
        _normalized: GLboolean,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glVertexAttribPointer)(_index, _size, _type, _normalized, _stride, _pointer)
    }
    unsafe fn glVertexBindingDivisor(&self, _bindingindex: GLuint, _divisor: GLuint) {
        (self.entry().glVertexBindingDivisor)(_bindingindex, _divisor)
    }
    unsafe fn glVertexP2ui(&self, _type: GLenum, _value: GLuint) {
        (self.entry().glVertexP2ui)(_type, _value)
    }
    unsafe fn glVertexP2uiv(&self, _type: GLenum, _value: *const GLuint) {
        (self.entry().glVertexP2uiv)(_type, _value)
    }
    unsafe fn glVertexP3ui(&self, _type: GLenum, _value: GLuint) {
        (self.entry().glVertexP3ui)(_type, _value)
    }
    unsafe fn glVertexP3uiv(&self, _type: GLenum, _value: *const GLuint) {
        (self.entry().glVertexP3uiv)(_type, _value)
    }
    unsafe fn glVertexP4ui(&self, _type: GLenum, _value: GLuint) {
        (self.entry().glVertexP4ui)(_type, _value)
    }
    unsafe fn glVertexP4uiv(&self, _type: GLenum, _value: *const GLuint) {
        (self.entry().glVertexP4uiv)(_type, _value)
    }
    unsafe fn glViewport(&self, _x: GLint, _y: GLint, _width: GLsizei, _height: GLsizei) {
        (self.entry().glViewport)(_x, _y, _width, _height)
    }
    unsafe fn glViewportArrayv(&self, _first: GLuint, _count: GLsizei, _v: *const GLfloat) {
        (self.entry().glViewportArrayv)(_first, _count, _v)
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
    unsafe fn glViewportIndexedfv(&self, _index: GLuint, _v: *const GLfloat) {
        (self.entry().glViewportIndexedfv)(_index, _v)
    }
    unsafe fn glWaitSync(&self, _sync: GLsync, _flags: GLbitfield, _timeout: GLuint64) {
        (self.entry().glWaitSync)(_sync, _flags, _timeout)
    }
}
