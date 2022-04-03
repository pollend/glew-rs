use crate::gles::feature::EntryGLESFn;
use crate::types::*;
use std::mem;
pub trait GLES32 {
    unsafe fn entry(&self) -> &EntryGLESFn;
    unsafe fn glActiveShaderProgram(&self, _pipeline: GLuint, _program: GLuint) {
        (self.entry().glActiveShaderProgram)(_pipeline, _program)
    }
    unsafe fn glActiveTexture(&self, _texture: GLenum) {
        (self.entry().glActiveTexture)(_texture)
    }
    unsafe fn glAttachShader(&self, _program: GLuint, _shader: GLuint) {
        (self.entry().glAttachShader)(_program, _shader)
    }
    unsafe fn glBeginQuery(&self, _target: GLenum, _id: GLuint) {
        (self.entry().glBeginQuery)(_target, _id)
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
    unsafe fn glBlendBarrier(&self) {
        (self.entry().glBlendBarrier)()
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
    unsafe fn glClear(&self, _mask: GLbitfield) {
        (self.entry().glClear)(_mask)
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
    unsafe fn glCompileShader(&self, _shader: GLuint) {
        (self.entry().glCompileShader)(_shader)
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
    unsafe fn glEnable(&self, _cap: GLenum) {
        (self.entry().glEnable)(_cap)
    }
    unsafe fn glEnableVertexAttribArray(&self, _index: GLuint) {
        (self.entry().glEnableVertexAttribArray)(_index)
    }
    unsafe fn glEnablei(&self, _target: GLenum, _index: GLuint) {
        (self.entry().glEnablei)(_target, _index)
    }
    unsafe fn glEndQuery(&self, _target: GLenum) {
        (self.entry().glEndQuery)(_target)
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
    unsafe fn glGetError(&self) -> GLenum {
        (self.entry().glGetError)()
    }
    unsafe fn glGetFloatv(&self, _pname: GLenum, _data: *mut GLfloat) {
        (self.entry().glGetFloatv)(_pname, _data)
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
    unsafe fn glGetGraphicsResetStatus(&self) -> GLenum {
        (self.entry().glGetGraphicsResetStatus)()
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
    unsafe fn glGetProgramiv(&self, _program: GLuint, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetProgramiv)(_program, _pname, _params)
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
    unsafe fn glGetVertexAttribPointerv(
        &self,
        _index: GLuint,
        _pname: GLenum,
        _pointer: *mut *mut std::os::raw::c_void,
    ) {
        (self.entry().glGetVertexAttribPointerv)(_index, _pname, _pointer)
    }
    unsafe fn glGetVertexAttribfv(&self, _index: GLuint, _pname: GLenum, _params: *mut GLfloat) {
        (self.entry().glGetVertexAttribfv)(_index, _pname, _params)
    }
    unsafe fn glGetVertexAttribiv(&self, _index: GLuint, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetVertexAttribiv)(_index, _pname, _params)
    }
    unsafe fn glGetnUniformfv(&self, program: GLuint, location: GLint, params: &mut [GLfloat]) {
        (self.entry().glGetnUniformfv)(
            program,
            location,
            ((params.len() * mem::size_of::<GLfloat>()) as GLsizei),
            params.as_mut_ptr(),
        )
    }
    unsafe fn glGetnUniformiv(&self, program: GLuint, location: GLint, params: &mut [GLint]) {
        (self.entry().glGetnUniformiv)(
            program,
            location,
            ((params.len() * mem::size_of::<GLint>()) as GLsizei),
            params.as_mut_ptr(),
        )
    }
    unsafe fn glGetnUniformuiv(&self, program: GLuint, location: GLint, params: &mut [GLuint]) {
        (self.entry().glGetnUniformuiv)(
            program,
            location,
            ((params.len() * mem::size_of::<GLuint>()) as GLsizei),
            params.as_mut_ptr(),
        )
    }
    unsafe fn glHint(&self, _target: GLenum, _mode: GLenum) {
        (self.entry().glHint)(_target, _mode)
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
    unsafe fn glMemoryBarrierByRegion(&self, _barriers: GLbitfield) {
        (self.entry().glMemoryBarrierByRegion)(_barriers)
    }
    unsafe fn glMinSampleShading(&self, _value: GLfloat) {
        (self.entry().glMinSampleShading)(_value)
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
    unsafe fn glPatchParameteri(&self, _pname: GLenum, _value: GLint) {
        (self.entry().glPatchParameteri)(_pname, _value)
    }
    unsafe fn glPauseTransformFeedback(&self) {
        (self.entry().glPauseTransformFeedback)()
    }
    unsafe fn glPixelStorei(&self, _pname: GLenum, _param: GLint) {
        (self.entry().glPixelStorei)(_pname, _param)
    }
    unsafe fn glPolygonOffset(&self, _factor: GLfloat, _units: GLfloat) {
        (self.entry().glPolygonOffset)(_factor, _units)
    }
    unsafe fn glPopDebugGroup(&self) {
        (self.entry().glPopDebugGroup)()
    }
    unsafe fn glPrimitiveBoundingBox(
        &self,
        _minX: GLfloat,
        _minY: GLfloat,
        _minZ: GLfloat,
        _minW: GLfloat,
        _maxX: GLfloat,
        _maxY: GLfloat,
        _maxZ: GLfloat,
        _maxW: GLfloat,
    ) {
        (self.entry().glPrimitiveBoundingBox)(
            _minX, _minY, _minZ, _minW, _maxX, _maxY, _maxZ, _maxW,
        )
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
    unsafe fn glPushDebugGroup(
        &self,
        _source: GLenum,
        _id: GLuint,
        _length: GLsizei,
        _message: *const GLchar,
    ) {
        (self.entry().glPushDebugGroup)(_source, _id, _length, _message)
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
    unsafe fn glReadnPixels(
        &self,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _format: GLenum,
        _type: GLenum,
        _bufSize: GLsizei,
        _data: *mut std::os::raw::c_void,
    ) {
        (self.entry().glReadnPixels)(_x, _y, _width, _height, _format, _type, _bufSize, _data)
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
    unsafe fn glTransformFeedbackVaryings(
        &self,
        _program: GLuint,
        _count: GLsizei,
        _varyings: *const *const GLchar,
        _bufferMode: GLenum,
    ) {
        (self.entry().glTransformFeedbackVaryings)(_program, _count, _varyings, _bufferMode)
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
    unsafe fn glVertexAttrib1f(&self, _index: GLuint, _x: GLfloat) {
        (self.entry().glVertexAttrib1f)(_index, _x)
    }
    unsafe fn glVertexAttrib1fv(&self, _index: GLuint, _v: *const GLfloat) {
        (self.entry().glVertexAttrib1fv)(_index, _v)
    }
    unsafe fn glVertexAttrib2f(&self, _index: GLuint, _x: GLfloat, _y: GLfloat) {
        (self.entry().glVertexAttrib2f)(_index, _x, _y)
    }
    unsafe fn glVertexAttrib2fv(&self, _index: GLuint, _v: *const GLfloat) {
        (self.entry().glVertexAttrib2fv)(_index, _v)
    }
    unsafe fn glVertexAttrib3f(&self, _index: GLuint, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glVertexAttrib3f)(_index, _x, _y, _z)
    }
    unsafe fn glVertexAttrib3fv(&self, _index: GLuint, _v: *const GLfloat) {
        (self.entry().glVertexAttrib3fv)(_index, _v)
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
    unsafe fn glVertexAttribI4i(&self, _index: GLuint, _x: GLint, _y: GLint, _z: GLint, _w: GLint) {
        (self.entry().glVertexAttribI4i)(_index, _x, _y, _z, _w)
    }
    unsafe fn glVertexAttribI4iv(&self, _index: GLuint, _v: *const GLint) {
        (self.entry().glVertexAttribI4iv)(_index, _v)
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
    unsafe fn glViewport(&self, _x: GLint, _y: GLint, _width: GLsizei, _height: GLsizei) {
        (self.entry().glViewport)(_x, _y, _width, _height)
    }
    unsafe fn glWaitSync(&self, _sync: GLsync, _flags: GLbitfield, _timeout: GLuint64) {
        (self.entry().glWaitSync)(_sync, _flags, _timeout)
    }
}
