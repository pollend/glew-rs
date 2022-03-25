use crate::gl::bitflags::*;
use crate::gl::context::GLContext;
use crate::gl::enums::*;
use crate::types::*;

impl GLContext {
    pub unsafe fn glGetSynciv(
        &self,
        _sync: GLsync,
        _pname: SyncParameterName,
        _count: GLsizei,
        _length: *mut GLsizei,
        _values: *mut GLint,
    ) {
        self.entry_gl32
            .glGetSynciv(_sync, _pname, _count, _length, _values);
    }
    pub unsafe fn glDeleteSync(&self, _sync: GLsync) {
        self.entry_gl32.glDeleteSync(_sync);
    }
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
        self.entry_gl32.glDrawRangeElementsBaseVertex(
            _mode,
            _start,
            _end,
            _count,
            _type,
            _indices,
            _basevertex,
        );
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
        self.entry_gl32.glDrawElementsInstancedBaseVertex(
            _mode,
            _count,
            _type,
            _indices,
            _instancecount,
            _basevertex,
        );
    }
    pub unsafe fn glFenceSync(
        &self,
        _condition: SyncCondition,
        _flags: SyncBehaviorFlags,
    ) -> GLsync {
        self.entry_gl32.glFenceSync(_condition, _flags)
    }
    pub unsafe fn glGetInteger64v(&self, _pname: GetPName, _data: *mut GLint64) {
        self.entry_gl32.glGetInteger64v(_pname, _data);
    }
    pub unsafe fn glWaitSync(&self, _sync: GLsync, _flags: SyncBehaviorFlags, _timeout: GLuint64) {
        self.entry_gl32.glWaitSync(_sync, _flags, _timeout);
    }
    pub unsafe fn glGetInteger64i_v(&self, _target: GetPName, _index: GLuint, _data: *mut GLint64) {
        self.entry_gl32.glGetInteger64i_v(_target, _index, _data);
    }
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
        self.entry_gl32.glTexImage3DMultisample(
            _target,
            _samples,
            _internalformat,
            _width,
            _height,
            _depth,
            _fixedsamplelocations,
        );
    }
    pub unsafe fn glSampleMaski(&self, _maskNumber: GLuint, _mask: GLbitfield) {
        self.entry_gl32.glSampleMaski(_maskNumber, _mask);
    }
    pub unsafe fn glProvokingVertex(&self, _mode: VertexProvokingMode) {
        self.entry_gl32.glProvokingVertex(_mode);
    }
    pub unsafe fn glDrawElementsBaseVertex(
        &self,
        _mode: PrimitiveType,
        _count: GLsizei,
        _type: DrawElementsType,
        _indices: *const std::os::raw::c_void,
        _basevertex: GLint,
    ) {
        self.entry_gl32
            .glDrawElementsBaseVertex(_mode, _count, _type, _indices, _basevertex);
    }
    pub unsafe fn glIsSync(&self, _sync: GLsync) -> GLboolean {
        self.entry_gl32.glIsSync(_sync)
    }
    pub unsafe fn glGetBufferParameteri64v(
        &self,
        _target: BufferTargetARB,
        _pname: BufferPNameARB,
        _params: *mut GLint64,
    ) {
        self.entry_gl32
            .glGetBufferParameteri64v(_target, _pname, _params);
    }
    pub unsafe fn glTexImage2DMultisample(
        &self,
        _target: TextureTarget,
        _samples: GLsizei,
        _internalformat: InternalFormat,
        _width: GLsizei,
        _height: GLsizei,
        _fixedsamplelocations: GLboolean,
    ) {
        self.entry_gl32.glTexImage2DMultisample(
            _target,
            _samples,
            _internalformat,
            _width,
            _height,
            _fixedsamplelocations,
        );
    }
    pub unsafe fn glGetMultisamplefv(
        &self,
        _pname: GetMultisamplePNameNV,
        _index: GLuint,
        _val: *mut GLfloat,
    ) {
        self.entry_gl32.glGetMultisamplefv(_pname, _index, _val);
    }
    pub unsafe fn glMultiDrawElementsBaseVertex(
        &self,
        _mode: PrimitiveType,
        _count: *const GLsizei,
        _type: DrawElementsType,
        _indices: *const *const std::os::raw::c_void,
        _drawcount: GLsizei,
        _basevertex: *const GLint,
    ) {
        self.entry_gl32.glMultiDrawElementsBaseVertex(
            _mode,
            _count,
            _type,
            _indices,
            _drawcount,
            _basevertex,
        );
    }
    pub unsafe fn glClientWaitSync(
        &self,
        _sync: GLsync,
        _flags: SyncObjectMask,
        _timeout: GLuint64,
    ) -> GLenum {
        self.entry_gl32.glClientWaitSync(_sync, _flags, _timeout)
    }
    pub unsafe fn glFramebufferTexture(
        &self,
        _target: FramebufferTarget,
        _attachment: FramebufferAttachment,
        _texture: GLuint,
        _level: GLint,
    ) {
        self.entry_gl32
            .glFramebufferTexture(_target, _attachment, _texture, _level);
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
        self.entry_gl12
            .glDrawRangeElements(_mode, _start, _end, _count, _type, _indices);
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
        self.entry_gl12.glTexSubImage3D(
            _target, _level, _xoffset, _yoffset, _zoffset, _width, _height, _depth, _format, _type,
            _pixels,
        );
    }
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
        self.entry_gl12.glCopyTexSubImage3D(
            _target, _level, _xoffset, _yoffset, _zoffset, _x, _y, _width, _height,
        );
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
        self.entry_gl12.glTexImage3D(
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
        );
    }
    pub unsafe fn glGetAttachedShaders(
        &self,
        _program: GLuint,
        _maxCount: GLsizei,
        _count: *mut GLsizei,
        _shaders: *mut GLuint,
    ) {
        self.entry_gl20
            .glGetAttachedShaders(_program, _maxCount, _count, _shaders);
    }
    pub unsafe fn glAttachShader(&self, _program: GLuint, _shader: GLuint) {
        self.entry_gl20.glAttachShader(_program, _shader);
    }
    pub unsafe fn glUniform1iv(&self, _location: GLint, _count: GLsizei, _value: *const GLint) {
        self.entry_gl20.glUniform1iv(_location, _count, _value);
    }
    pub unsafe fn glUniform2iv(&self, _location: GLint, _count: GLsizei, _value: *const GLint) {
        self.entry_gl20.glUniform2iv(_location, _count, _value);
    }
    pub unsafe fn glCreateProgram(&self) -> GLuint {
        self.entry_gl20.glCreateProgram()
    }
    pub unsafe fn glGetUniformLocation(&self, _program: GLuint, _name: *const GLchar) -> GLint {
        self.entry_gl20.glGetUniformLocation(_program, _name)
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
        self.entry_gl20
            .glGetActiveUniform(_program, _index, _bufSize, _length, _size, _type, _name);
    }
    pub unsafe fn glVertexAttrib1fv(&self, _index: GLuint, _v: *const GLfloat) {
        self.entry_gl20.glVertexAttrib1fv(_index, _v);
    }
    pub unsafe fn glVertexAttrib2d(&self, _index: GLuint, _x: GLdouble, _y: GLdouble) {
        self.entry_gl20.glVertexAttrib2d(_index, _x, _y);
    }
    pub unsafe fn glUniform3fv(&self, _location: GLint, _count: GLsizei, _value: *const GLfloat) {
        self.entry_gl20.glUniform3fv(_location, _count, _value);
    }
    pub unsafe fn glVertexAttrib4Nub(
        &self,
        _index: GLuint,
        _x: GLubyte,
        _y: GLubyte,
        _z: GLubyte,
        _w: GLubyte,
    ) {
        self.entry_gl20.glVertexAttrib4Nub(_index, _x, _y, _z, _w);
    }
    pub unsafe fn glDetachShader(&self, _program: GLuint, _shader: GLuint) {
        self.entry_gl20.glDetachShader(_program, _shader);
    }
    pub unsafe fn glVertexAttrib4iv(&self, _index: GLuint, _v: *const GLint) {
        self.entry_gl20.glVertexAttrib4iv(_index, _v);
    }
    pub unsafe fn glVertexAttrib4s(
        &self,
        _index: GLuint,
        _x: GLshort,
        _y: GLshort,
        _z: GLshort,
        _w: GLshort,
    ) {
        self.entry_gl20.glVertexAttrib4s(_index, _x, _y, _z, _w);
    }
    pub unsafe fn glUniform4i(
        &self,
        _location: GLint,
        _v0: GLint,
        _v1: GLint,
        _v2: GLint,
        _v3: GLint,
    ) {
        self.entry_gl20.glUniform4i(_location, _v0, _v1, _v2, _v3);
    }
    pub unsafe fn glVertexAttrib4sv(&self, _index: GLuint, _v: *const GLshort) {
        self.entry_gl20.glVertexAttrib4sv(_index, _v);
    }
    pub unsafe fn glVertexAttrib4uiv(&self, _index: GLuint, _v: *const GLuint) {
        self.entry_gl20.glVertexAttrib4uiv(_index, _v);
    }
    pub unsafe fn glVertexAttrib1sv(&self, _index: GLuint, _v: *const GLshort) {
        self.entry_gl20.glVertexAttrib1sv(_index, _v);
    }
    pub unsafe fn glVertexAttrib2f(&self, _index: GLuint, _x: GLfloat, _y: GLfloat) {
        self.entry_gl20.glVertexAttrib2f(_index, _x, _y);
    }
    pub unsafe fn glVertexAttrib4Nsv(&self, _index: GLuint, _v: *const GLshort) {
        self.entry_gl20.glVertexAttrib4Nsv(_index, _v);
    }
    pub unsafe fn glEnableVertexAttribArray(&self, _index: GLuint) {
        self.entry_gl20.glEnableVertexAttribArray(_index);
    }
    pub unsafe fn glUniform4iv(&self, _location: GLint, _count: GLsizei, _value: *const GLint) {
        self.entry_gl20.glUniform4iv(_location, _count, _value);
    }
    pub unsafe fn glVertexAttrib2s(&self, _index: GLuint, _x: GLshort, _y: GLshort) {
        self.entry_gl20.glVertexAttrib2s(_index, _x, _y);
    }
    pub unsafe fn glGetShaderiv(
        &self,
        _shader: GLuint,
        _pname: ShaderParameterName,
        _params: *mut GLint,
    ) {
        self.entry_gl20.glGetShaderiv(_shader, _pname, _params);
    }
    pub unsafe fn glGetProgramInfoLog(
        &self,
        _program: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _infoLog: *mut GLchar,
    ) {
        self.entry_gl20
            .glGetProgramInfoLog(_program, _bufSize, _length, _infoLog);
    }
    pub unsafe fn glVertexAttrib4Nbv(&self, _index: GLuint, _v: *const GLbyte) {
        self.entry_gl20.glVertexAttrib4Nbv(_index, _v);
    }
    pub unsafe fn glVertexAttrib4Nusv(&self, _index: GLuint, _v: *const GLushort) {
        self.entry_gl20.glVertexAttrib4Nusv(_index, _v);
    }
    pub unsafe fn glVertexAttrib4bv(&self, _index: GLuint, _v: *const GLbyte) {
        self.entry_gl20.glVertexAttrib4bv(_index, _v);
    }
    pub unsafe fn glGetShaderInfoLog(
        &self,
        _shader: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _infoLog: *mut GLchar,
    ) {
        self.entry_gl20
            .glGetShaderInfoLog(_shader, _bufSize, _length, _infoLog);
    }
    pub unsafe fn glDisableVertexAttribArray(&self, _index: GLuint) {
        self.entry_gl20.glDisableVertexAttribArray(_index);
    }
    pub unsafe fn glUseProgram(&self, _program: GLuint) {
        self.entry_gl20.glUseProgram(_program);
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
        self.entry_gl20
            .glGetActiveAttrib(_program, _index, _bufSize, _length, _size, _type, _name);
    }
    pub unsafe fn glVertexAttrib3fv(&self, _index: GLuint, _v: *const GLfloat) {
        self.entry_gl20.glVertexAttrib3fv(_index, _v);
    }
    pub unsafe fn glVertexAttrib4usv(&self, _index: GLuint, _v: *const GLushort) {
        self.entry_gl20.glVertexAttrib4usv(_index, _v);
    }
    pub unsafe fn glVertexAttrib2dv(&self, _index: GLuint, _v: *const GLdouble) {
        self.entry_gl20.glVertexAttrib2dv(_index, _v);
    }
    pub unsafe fn glVertexAttribPointer(
        &self,
        _index: GLuint,
        _size: GLint,
        _type: VertexAttribPointerType,
        _normalized: GLboolean,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        self.entry_gl20
            .glVertexAttribPointer(_index, _size, _type, _normalized, _stride, _pointer);
    }
    pub unsafe fn glShaderSource(
        &self,
        _shader: GLuint,
        _count: GLsizei,
        _string: *const *const GLchar,
        _length: *const GLint,
    ) {
        self.entry_gl20
            .glShaderSource(_shader, _count, _string, _length);
    }
    pub unsafe fn glGetVertexAttribfv(
        &self,
        _index: GLuint,
        _pname: VertexAttribPropertyARB,
        _params: *mut GLfloat,
    ) {
        self.entry_gl20.glGetVertexAttribfv(_index, _pname, _params);
    }
    pub unsafe fn glUniform3iv(&self, _location: GLint, _count: GLsizei, _value: *const GLint) {
        self.entry_gl20.glUniform3iv(_location, _count, _value);
    }
    pub unsafe fn glVertexAttrib3dv(&self, _index: GLuint, _v: *const GLdouble) {
        self.entry_gl20.glVertexAttrib3dv(_index, _v);
    }
    pub unsafe fn glVertexAttrib4ubv(&self, _index: GLuint, _v: *const GLubyte) {
        self.entry_gl20.glVertexAttrib4ubv(_index, _v);
    }
    pub unsafe fn glUniform1fv(&self, _location: GLint, _count: GLsizei, _value: *const GLfloat) {
        self.entry_gl20.glUniform1fv(_location, _count, _value);
    }
    pub unsafe fn glValidateProgram(&self, _program: GLuint) {
        self.entry_gl20.glValidateProgram(_program);
    }
    pub unsafe fn glVertexAttrib4Nuiv(&self, _index: GLuint, _v: *const GLuint) {
        self.entry_gl20.glVertexAttrib4Nuiv(_index, _v);
    }
    pub unsafe fn glVertexAttrib4f(
        &self,
        _index: GLuint,
        _x: GLfloat,
        _y: GLfloat,
        _z: GLfloat,
        _w: GLfloat,
    ) {
        self.entry_gl20.glVertexAttrib4f(_index, _x, _y, _z, _w);
    }
    pub unsafe fn glUniform3f(&self, _location: GLint, _v0: GLfloat, _v1: GLfloat, _v2: GLfloat) {
        self.entry_gl20.glUniform3f(_location, _v0, _v1, _v2);
    }
    pub unsafe fn glUniform2i(&self, _location: GLint, _v0: GLint, _v1: GLint) {
        self.entry_gl20.glUniform2i(_location, _v0, _v1);
    }
    pub unsafe fn glUniformMatrix3fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        self.entry_gl20
            .glUniformMatrix3fv(_location, _count, _transpose, _value);
    }
    pub unsafe fn glDrawBuffers(&self, _n: GLsizei, _bufs: *const DrawBufferMode) {
        self.entry_gl20.glDrawBuffers(_n, _bufs);
    }
    pub unsafe fn glUniform3i(&self, _location: GLint, _v0: GLint, _v1: GLint, _v2: GLint) {
        self.entry_gl20.glUniform3i(_location, _v0, _v1, _v2);
    }
    pub unsafe fn glUniform1f(&self, _location: GLint, _v0: GLfloat) {
        self.entry_gl20.glUniform1f(_location, _v0);
    }
    pub unsafe fn glVertexAttrib2fv(&self, _index: GLuint, _v: *const GLfloat) {
        self.entry_gl20.glVertexAttrib2fv(_index, _v);
    }
    pub unsafe fn glVertexAttrib3s(&self, _index: GLuint, _x: GLshort, _y: GLshort, _z: GLshort) {
        self.entry_gl20.glVertexAttrib3s(_index, _x, _y, _z);
    }
    pub unsafe fn glGetShaderSource(
        &self,
        _shader: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _source: *mut GLchar,
    ) {
        self.entry_gl20
            .glGetShaderSource(_shader, _bufSize, _length, _source);
    }
    pub unsafe fn glStencilMaskSeparate(&self, _face: StencilFaceDirection, _mask: GLuint) {
        self.entry_gl20.glStencilMaskSeparate(_face, _mask);
    }
    pub unsafe fn glUniformMatrix2fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        self.entry_gl20
            .glUniformMatrix2fv(_location, _count, _transpose, _value);
    }
    pub unsafe fn glVertexAttrib1s(&self, _index: GLuint, _x: GLshort) {
        self.entry_gl20.glVertexAttrib1s(_index, _x);
    }
    pub unsafe fn glGetVertexAttribiv(
        &self,
        _index: GLuint,
        _pname: VertexAttribPropertyARB,
        _params: *mut GLint,
    ) {
        self.entry_gl20.glGetVertexAttribiv(_index, _pname, _params);
    }
    pub unsafe fn glVertexAttrib3sv(&self, _index: GLuint, _v: *const GLshort) {
        self.entry_gl20.glVertexAttrib3sv(_index, _v);
    }
    pub unsafe fn glGetUniformiv(&self, _program: GLuint, _location: GLint, _params: *mut GLint) {
        self.entry_gl20.glGetUniformiv(_program, _location, _params);
    }
    pub unsafe fn glIsShader(&self, _shader: GLuint) -> GLboolean {
        self.entry_gl20.glIsShader(_shader)
    }
    pub unsafe fn glVertexAttrib3f(&self, _index: GLuint, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        self.entry_gl20.glVertexAttrib3f(_index, _x, _y, _z);
    }
    pub unsafe fn glDeleteProgram(&self, _program: GLuint) {
        self.entry_gl20.glDeleteProgram(_program);
    }
    pub unsafe fn glVertexAttrib4Nubv(&self, _index: GLuint, _v: *const GLubyte) {
        self.entry_gl20.glVertexAttrib4Nubv(_index, _v);
    }
    pub unsafe fn glBlendEquationSeparate(
        &self,
        _modeRGB: BlendEquationModeEXT,
        _modeAlpha: BlendEquationModeEXT,
    ) {
        self.entry_gl20
            .glBlendEquationSeparate(_modeRGB, _modeAlpha);
    }
    pub unsafe fn glStencilOpSeparate(
        &self,
        _face: StencilFaceDirection,
        _sfail: StencilOp,
        _dpfail: StencilOp,
        _dppass: StencilOp,
    ) {
        self.entry_gl20
            .glStencilOpSeparate(_face, _sfail, _dpfail, _dppass);
    }
    pub unsafe fn glUniform2fv(&self, _location: GLint, _count: GLsizei, _value: *const GLfloat) {
        self.entry_gl20.glUniform2fv(_location, _count, _value);
    }
    pub unsafe fn glGetVertexAttribdv(
        &self,
        _index: GLuint,
        _pname: VertexAttribPropertyARB,
        _params: *mut GLdouble,
    ) {
        self.entry_gl20.glGetVertexAttribdv(_index, _pname, _params);
    }
    pub unsafe fn glIsProgram(&self, _program: GLuint) -> GLboolean {
        self.entry_gl20.glIsProgram(_program)
    }
    pub unsafe fn glGetAttribLocation(&self, _program: GLuint, _name: *const GLchar) -> GLint {
        self.entry_gl20.glGetAttribLocation(_program, _name)
    }
    pub unsafe fn glUniformMatrix4fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        self.entry_gl20
            .glUniformMatrix4fv(_location, _count, _transpose, _value);
    }
    pub unsafe fn glBindAttribLocation(
        &self,
        _program: GLuint,
        _index: GLuint,
        _name: *const GLchar,
    ) {
        self.entry_gl20
            .glBindAttribLocation(_program, _index, _name);
    }
    pub unsafe fn glStencilFuncSeparate(
        &self,
        _face: StencilFaceDirection,
        _func: StencilFunction,
        _ref: GLint,
        _mask: GLuint,
    ) {
        self.entry_gl20
            .glStencilFuncSeparate(_face, _func, _ref, _mask);
    }
    pub unsafe fn glVertexAttrib1dv(&self, _index: GLuint, _v: *const GLdouble) {
        self.entry_gl20.glVertexAttrib1dv(_index, _v);
    }
    pub unsafe fn glGetUniformfv(&self, _program: GLuint, _location: GLint, _params: *mut GLfloat) {
        self.entry_gl20.glGetUniformfv(_program, _location, _params);
    }
    pub unsafe fn glVertexAttrib4Niv(&self, _index: GLuint, _v: *const GLint) {
        self.entry_gl20.glVertexAttrib4Niv(_index, _v);
    }
    pub unsafe fn glVertexAttrib4d(
        &self,
        _index: GLuint,
        _x: GLdouble,
        _y: GLdouble,
        _z: GLdouble,
        _w: GLdouble,
    ) {
        self.entry_gl20.glVertexAttrib4d(_index, _x, _y, _z, _w);
    }
    pub unsafe fn glVertexAttrib2sv(&self, _index: GLuint, _v: *const GLshort) {
        self.entry_gl20.glVertexAttrib2sv(_index, _v);
    }
    pub unsafe fn glLinkProgram(&self, _program: GLuint) {
        self.entry_gl20.glLinkProgram(_program);
    }
    pub unsafe fn glUniform1i(&self, _location: GLint, _v0: GLint) {
        self.entry_gl20.glUniform1i(_location, _v0);
    }
    pub unsafe fn glVertexAttrib4dv(&self, _index: GLuint, _v: *const GLdouble) {
        self.entry_gl20.glVertexAttrib4dv(_index, _v);
    }
    pub unsafe fn glCompileShader(&self, _shader: GLuint) {
        self.entry_gl20.glCompileShader(_shader);
    }
    pub unsafe fn glUniform2f(&self, _location: GLint, _v0: GLfloat, _v1: GLfloat) {
        self.entry_gl20.glUniform2f(_location, _v0, _v1);
    }
    pub unsafe fn glVertexAttrib3d(
        &self,
        _index: GLuint,
        _x: GLdouble,
        _y: GLdouble,
        _z: GLdouble,
    ) {
        self.entry_gl20.glVertexAttrib3d(_index, _x, _y, _z);
    }
    pub unsafe fn glVertexAttrib4fv(&self, _index: GLuint, _v: *const GLfloat) {
        self.entry_gl20.glVertexAttrib4fv(_index, _v);
    }
    pub unsafe fn glGetProgramiv(
        &self,
        _program: GLuint,
        _pname: ProgramPropertyARB,
        _params: *mut GLint,
    ) {
        self.entry_gl20.glGetProgramiv(_program, _pname, _params);
    }
    pub unsafe fn glUniform4f(
        &self,
        _location: GLint,
        _v0: GLfloat,
        _v1: GLfloat,
        _v2: GLfloat,
        _v3: GLfloat,
    ) {
        self.entry_gl20.glUniform4f(_location, _v0, _v1, _v2, _v3);
    }
    pub unsafe fn glUniform4fv(&self, _location: GLint, _count: GLsizei, _value: *const GLfloat) {
        self.entry_gl20.glUniform4fv(_location, _count, _value);
    }
    pub unsafe fn glVertexAttrib1d(&self, _index: GLuint, _x: GLdouble) {
        self.entry_gl20.glVertexAttrib1d(_index, _x);
    }
    pub unsafe fn glVertexAttrib1f(&self, _index: GLuint, _x: GLfloat) {
        self.entry_gl20.glVertexAttrib1f(_index, _x);
    }
    pub unsafe fn glDeleteShader(&self, _shader: GLuint) {
        self.entry_gl20.glDeleteShader(_shader);
    }
    pub unsafe fn glGetVertexAttribPointerv(
        &self,
        _index: GLuint,
        _pname: VertexAttribPointerPropertyARB,
        _pointer: *mut *mut std::os::raw::c_void,
    ) {
        self.entry_gl20
            .glGetVertexAttribPointerv(_index, _pname, _pointer);
    }
    pub unsafe fn glCreateShader(&self, _type: ShaderType) -> GLuint {
        self.entry_gl20.glCreateShader(_type)
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
        self.entry_gl45.glCompressedTextureSubImage2D(
            _texture, _level, _xoffset, _yoffset, _width, _height, _format, _imageSize, _data,
        );
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
        self.entry_gl45.glVertexArrayAttribFormat(
            _vaobj,
            _attribindex,
            _size,
            _type,
            _normalized,
            _relativeoffset,
        );
    }
    pub unsafe fn glGetVertexArrayIndexed64iv(
        &self,
        _vaobj: GLuint,
        _index: GLuint,
        _pname: VertexArrayPName,
        _param: *mut GLint64,
    ) {
        self.entry_gl45
            .glGetVertexArrayIndexed64iv(_vaobj, _index, _pname, _param);
    }
    pub unsafe fn glTextureParameteri(
        &self,
        _texture: GLuint,
        _pname: TextureParameterName,
        _param: GLint,
    ) {
        self.entry_gl45
            .glTextureParameteri(_texture, _pname, _param);
    }
    pub unsafe fn glGetQueryBufferObjectiv(
        &self,
        _id: GLuint,
        _buffer: GLuint,
        _pname: QueryObjectParameterName,
        _offset: GLintptr,
    ) {
        self.entry_gl45
            .glGetQueryBufferObjectiv(_id, _buffer, _pname, _offset);
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
        self.entry_gl45.glGetnSeparableFilter(
            _target,
            _format,
            _type,
            _rowBufSize,
            _row,
            _columnBufSize,
            _column,
            _span,
        );
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
        self.entry_gl45.glCopyTextureSubImage3D(
            _texture, _level, _xoffset, _yoffset, _zoffset, _x, _y, _width, _height,
        );
    }
    pub unsafe fn glNamedBufferSubData(
        &self,
        _buffer: GLuint,
        _offset: GLintptr,
        _size: GLsizeiptr,
        _data: *const std::os::raw::c_void,
    ) {
        self.entry_gl45
            .glNamedBufferSubData(_buffer, _offset, _size, _data);
    }
    pub unsafe fn glGetnPixelMapuiv(
        &self,
        _map: PixelMap,
        _bufSize: GLsizei,
        _values: *mut GLuint,
    ) {
        self.entry_gl45.glGetnPixelMapuiv(_map, _bufSize, _values);
    }
    pub unsafe fn glGetNamedRenderbufferParameteriv(
        &self,
        _renderbuffer: GLuint,
        _pname: RenderbufferParameterName,
        _params: *mut GLint,
    ) {
        self.entry_gl45
            .glGetNamedRenderbufferParameteriv(_renderbuffer, _pname, _params);
    }
    pub unsafe fn glCreateFramebuffers(&self, _n: GLsizei, _framebuffers: *mut GLuint) {
        self.entry_gl45.glCreateFramebuffers(_n, _framebuffers);
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
        self.entry_gl45.glCompressedTextureSubImage1D(
            _texture, _level, _xoffset, _width, _format, _imageSize, _data,
        );
    }
    pub unsafe fn glNamedFramebufferRenderbuffer(
        &self,
        _framebuffer: GLuint,
        _attachment: FramebufferAttachment,
        _renderbuffertarget: RenderbufferTarget,
        _renderbuffer: GLuint,
    ) {
        self.entry_gl45.glNamedFramebufferRenderbuffer(
            _framebuffer,
            _attachment,
            _renderbuffertarget,
            _renderbuffer,
        );
    }
    pub unsafe fn glGetnUniformdv(
        &self,
        _program: GLuint,
        _location: GLint,
        _bufSize: GLsizei,
        _params: *mut GLdouble,
    ) {
        self.entry_gl45
            .glGetnUniformdv(_program, _location, _bufSize, _params);
    }
    pub unsafe fn glGetTransformFeedbackiv(
        &self,
        _xfb: GLuint,
        _pname: TransformFeedbackPName,
        _param: *mut GLint,
    ) {
        self.entry_gl45
            .glGetTransformFeedbackiv(_xfb, _pname, _param);
    }
    pub unsafe fn glGetTransformFeedbacki64_v(
        &self,
        _xfb: GLuint,
        _pname: TransformFeedbackPName,
        _index: GLuint,
        _param: *mut GLint64,
    ) {
        self.entry_gl45
            .glGetTransformFeedbacki64_v(_xfb, _pname, _index, _param);
    }
    pub unsafe fn glInvalidateNamedFramebufferData(
        &self,
        _framebuffer: GLuint,
        _numAttachments: GLsizei,
        _attachments: *const FramebufferAttachment,
    ) {
        self.entry_gl45.glInvalidateNamedFramebufferData(
            _framebuffer,
            _numAttachments,
            _attachments,
        );
    }
    pub unsafe fn glNamedFramebufferTextureLayer(
        &self,
        _framebuffer: GLuint,
        _attachment: FramebufferAttachment,
        _texture: GLuint,
        _level: GLint,
        _layer: GLint,
    ) {
        self.entry_gl45.glNamedFramebufferTextureLayer(
            _framebuffer,
            _attachment,
            _texture,
            _level,
            _layer,
        );
    }
    pub unsafe fn glTextureStorage1D(
        &self,
        _texture: GLuint,
        _levels: GLsizei,
        _internalformat: SizedInternalFormat,
        _width: GLsizei,
    ) {
        self.entry_gl45
            .glTextureStorage1D(_texture, _levels, _internalformat, _width);
    }
    pub unsafe fn glTextureBufferRange(
        &self,
        _texture: GLuint,
        _internalformat: SizedInternalFormat,
        _buffer: GLuint,
        _offset: GLintptr,
        _size: GLsizeiptr,
    ) {
        self.entry_gl45
            .glTextureBufferRange(_texture, _internalformat, _buffer, _offset, _size);
    }
    pub unsafe fn glGetnUniformiv(
        &self,
        _program: GLuint,
        _location: GLint,
        _bufSize: GLsizei,
        _params: *mut GLint,
    ) {
        self.entry_gl45
            .glGetnUniformiv(_program, _location, _bufSize, _params);
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
        self.entry_gl45.glCopyTextureSubImage2D(
            _texture, _level, _xoffset, _yoffset, _x, _y, _width, _height,
        );
    }
    pub unsafe fn glGetVertexArrayIndexediv(
        &self,
        _vaobj: GLuint,
        _index: GLuint,
        _pname: VertexArrayPName,
        _param: *mut GLint,
    ) {
        self.entry_gl45
            .glGetVertexArrayIndexediv(_vaobj, _index, _pname, _param);
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
        self.entry_gl45.glGetCompressedTextureSubImage(
            _texture, _level, _xoffset, _yoffset, _zoffset, _width, _height, _depth, _bufSize,
            _pixels,
        );
    }
    pub unsafe fn glNamedFramebufferDrawBuffer(&self, _framebuffer: GLuint, _buf: ColorBuffer) {
        self.entry_gl45
            .glNamedFramebufferDrawBuffer(_framebuffer, _buf);
    }
    pub unsafe fn glVertexArrayAttribBinding(
        &self,
        _vaobj: GLuint,
        _attribindex: GLuint,
        _bindingindex: GLuint,
    ) {
        self.entry_gl45
            .glVertexArrayAttribBinding(_vaobj, _attribindex, _bindingindex);
    }
    pub unsafe fn glGetQueryBufferObjecti64v(
        &self,
        _id: GLuint,
        _buffer: GLuint,
        _pname: QueryObjectParameterName,
        _offset: GLintptr,
    ) {
        self.entry_gl45
            .glGetQueryBufferObjecti64v(_id, _buffer, _pname, _offset);
    }
    pub unsafe fn glClearNamedFramebufferfv(
        &self,
        _framebuffer: GLuint,
        _buffer: Buffer,
        _drawbuffer: GLint,
        _value: *const GLfloat,
    ) {
        self.entry_gl45
            .glClearNamedFramebufferfv(_framebuffer, _buffer, _drawbuffer, _value);
    }
    pub unsafe fn glClearNamedFramebufferiv(
        &self,
        _framebuffer: GLuint,
        _buffer: Buffer,
        _drawbuffer: GLint,
        _value: *const GLint,
    ) {
        self.entry_gl45
            .glClearNamedFramebufferiv(_framebuffer, _buffer, _drawbuffer, _value);
    }
    pub unsafe fn glGetnPolygonStipple(&self, _bufSize: GLsizei, _pattern: *mut GLubyte) {
        self.entry_gl45.glGetnPolygonStipple(_bufSize, _pattern);
    }
    pub unsafe fn glCreateQueries(&self, _target: QueryTarget, _n: GLsizei, _ids: *mut GLuint) {
        self.entry_gl45.glCreateQueries(_target, _n, _ids);
    }
    pub unsafe fn glGetnMapiv(
        &self,
        _target: MapTarget,
        _query: MapQuery,
        _bufSize: GLsizei,
        _v: *mut GLint,
    ) {
        self.entry_gl45.glGetnMapiv(_target, _query, _bufSize, _v);
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
        self.entry_gl45
            .glReadnPixels(_x, _y, _width, _height, _format, _type, _bufSize, _data);
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
        self.entry_gl45.glTextureSubImage2D(
            _texture, _level, _xoffset, _yoffset, _width, _height, _format, _type, _pixels,
        );
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
        self.entry_gl45.glTextureStorage3DMultisample(
            _texture,
            _samples,
            _internalformat,
            _width,
            _height,
            _depth,
            _fixedsamplelocations,
        );
    }
    pub unsafe fn glNamedRenderbufferStorageMultisample(
        &self,
        _renderbuffer: GLuint,
        _samples: GLsizei,
        _internalformat: InternalFormat,
        _width: GLsizei,
        _height: GLsizei,
    ) {
        self.entry_gl45.glNamedRenderbufferStorageMultisample(
            _renderbuffer,
            _samples,
            _internalformat,
            _width,
            _height,
        );
    }
    pub unsafe fn glCreateTextures(
        &self,
        _target: TextureTarget,
        _n: GLsizei,
        _textures: *mut GLuint,
    ) {
        self.entry_gl45.glCreateTextures(_target, _n, _textures);
    }
    pub unsafe fn glCreateVertexArrays(&self, _n: GLsizei, _arrays: *mut GLuint) {
        self.entry_gl45.glCreateVertexArrays(_n, _arrays);
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
        self.entry_gl45
            .glGetTextureImage(_texture, _level, _format, _type, _bufSize, _pixels);
    }
    pub unsafe fn glNamedFramebufferParameteri(
        &self,
        _framebuffer: GLuint,
        _pname: FramebufferParameterName,
        _param: GLint,
    ) {
        self.entry_gl45
            .glNamedFramebufferParameteri(_framebuffer, _pname, _param);
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
        self.entry_gl45.glBlitNamedFramebuffer(
            _readFramebuffer,
            _drawFramebuffer,
            _srcX0,
            _srcY0,
            _srcX1,
            _srcY1,
            _dstX0,
            _dstY0,
            _dstX1,
            _dstY1,
            _mask,
            _filter,
        );
    }
    pub unsafe fn glNamedRenderbufferStorage(
        &self,
        _renderbuffer: GLuint,
        _internalformat: InternalFormat,
        _width: GLsizei,
        _height: GLsizei,
    ) {
        self.entry_gl45
            .glNamedRenderbufferStorage(_renderbuffer, _internalformat, _width, _height);
    }
    pub unsafe fn glGetTextureLevelParameteriv(
        &self,
        _texture: GLuint,
        _level: GLint,
        _pname: GetTextureParameter,
        _params: *mut GLint,
    ) {
        self.entry_gl45
            .glGetTextureLevelParameteriv(_texture, _level, _pname, _params);
    }
    pub unsafe fn glVertexArrayVertexBuffer(
        &self,
        _vaobj: GLuint,
        _bindingindex: GLuint,
        _buffer: GLuint,
        _offset: GLintptr,
        _stride: GLsizei,
    ) {
        self.entry_gl45
            .glVertexArrayVertexBuffer(_vaobj, _bindingindex, _buffer, _offset, _stride);
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
        self.entry_gl45.glGetTextureSubImage(
            _texture, _level, _xoffset, _yoffset, _zoffset, _width, _height, _depth, _format,
            _type, _bufSize, _pixels,
        );
    }
    pub unsafe fn glGetNamedBufferSubData(
        &self,
        _buffer: GLuint,
        _offset: GLintptr,
        _size: GLsizeiptr,
        _data: *mut std::os::raw::c_void,
    ) {
        self.entry_gl45
            .glGetNamedBufferSubData(_buffer, _offset, _size, _data);
    }
    pub unsafe fn glGetnMapdv(
        &self,
        _target: MapTarget,
        _query: MapQuery,
        _bufSize: GLsizei,
        _v: *mut GLdouble,
    ) {
        self.entry_gl45.glGetnMapdv(_target, _query, _bufSize, _v);
    }
    pub unsafe fn glCreateSamplers(&self, _n: GLsizei, _samplers: *mut GLuint) {
        self.entry_gl45.glCreateSamplers(_n, _samplers);
    }
    pub unsafe fn glCreateTransformFeedbacks(&self, _n: GLsizei, _ids: *mut GLuint) {
        self.entry_gl45.glCreateTransformFeedbacks(_n, _ids);
    }
    pub unsafe fn glMemoryBarrierByRegion(&self, _barriers: MemoryBarrierMask) {
        self.entry_gl45.glMemoryBarrierByRegion(_barriers);
    }
    pub unsafe fn glGetTransformFeedbacki_v(
        &self,
        _xfb: GLuint,
        _pname: TransformFeedbackPName,
        _index: GLuint,
        _param: *mut GLint,
    ) {
        self.entry_gl45
            .glGetTransformFeedbacki_v(_xfb, _pname, _index, _param);
    }
    pub unsafe fn glGetGraphicsResetStatus(&self) -> GLenum {
        self.entry_gl45.glGetGraphicsResetStatus()
    }
    pub unsafe fn glGetnUniformfv(
        &self,
        _program: GLuint,
        _location: GLint,
        _bufSize: GLsizei,
        _params: *mut GLfloat,
    ) {
        self.entry_gl45
            .glGetnUniformfv(_program, _location, _bufSize, _params);
    }
    pub unsafe fn glGetQueryBufferObjectuiv(
        &self,
        _id: GLuint,
        _buffer: GLuint,
        _pname: QueryObjectParameterName,
        _offset: GLintptr,
    ) {
        self.entry_gl45
            .glGetQueryBufferObjectuiv(_id, _buffer, _pname, _offset);
    }
    pub unsafe fn glGetCompressedTextureImage(
        &self,
        _texture: GLuint,
        _level: GLint,
        _bufSize: GLsizei,
        _pixels: *mut std::os::raw::c_void,
    ) {
        self.entry_gl45
            .glGetCompressedTextureImage(_texture, _level, _bufSize, _pixels);
    }
    pub unsafe fn glMapNamedBufferRange(
        &self,
        _buffer: GLuint,
        _offset: GLintptr,
        _length: GLsizeiptr,
        _access: MapBufferAccessMask,
    ) -> *mut std::os::raw::c_void {
        self.entry_gl45
            .glMapNamedBufferRange(_buffer, _offset, _length, _access)
    }
    pub unsafe fn glNamedFramebufferDrawBuffers(
        &self,
        _framebuffer: GLuint,
        _n: GLsizei,
        _bufs: *const ColorBuffer,
    ) {
        self.entry_gl45
            .glNamedFramebufferDrawBuffers(_framebuffer, _n, _bufs);
    }
    pub unsafe fn glVertexArrayElementBuffer(&self, _vaobj: GLuint, _buffer: GLuint) {
        self.entry_gl45.glVertexArrayElementBuffer(_vaobj, _buffer);
    }
    pub unsafe fn glTextureBuffer(
        &self,
        _texture: GLuint,
        _internalformat: SizedInternalFormat,
        _buffer: GLuint,
    ) {
        self.entry_gl45
            .glTextureBuffer(_texture, _internalformat, _buffer);
    }
    pub unsafe fn glNamedBufferStorage(
        &self,
        _buffer: GLuint,
        _size: GLsizeiptr,
        _data: *const std::os::raw::c_void,
        _flags: BufferStorageMask,
    ) {
        self.entry_gl45
            .glNamedBufferStorage(_buffer, _size, _data, _flags);
    }
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
        self.entry_gl45.glInvalidateNamedFramebufferSubData(
            _framebuffer,
            _numAttachments,
            _attachments,
            _x,
            _y,
            _width,
            _height,
        );
    }
    pub unsafe fn glGetNamedBufferParameteri64v(
        &self,
        _buffer: GLuint,
        _pname: BufferPNameARB,
        _params: *mut GLint64,
    ) {
        self.entry_gl45
            .glGetNamedBufferParameteri64v(_buffer, _pname, _params);
    }
    pub unsafe fn glCopyNamedBufferSubData(
        &self,
        _readBuffer: GLuint,
        _writeBuffer: GLuint,
        _readOffset: GLintptr,
        _writeOffset: GLintptr,
        _size: GLsizeiptr,
    ) {
        self.entry_gl45.glCopyNamedBufferSubData(
            _readBuffer,
            _writeBuffer,
            _readOffset,
            _writeOffset,
            _size,
        );
    }
    pub unsafe fn glClipControl(&self, _origin: ClipControlOrigin, _depth: ClipControlDepth) {
        self.entry_gl45.glClipControl(_origin, _depth);
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
        self.entry_gl45.glCompressedTextureSubImage3D(
            _texture, _level, _xoffset, _yoffset, _zoffset, _width, _height, _depth, _format,
            _imageSize, _data,
        );
    }
    pub unsafe fn glGetNamedFramebufferAttachmentParameteriv(
        &self,
        _framebuffer: GLuint,
        _attachment: FramebufferAttachment,
        _pname: FramebufferAttachmentParameterName,
        _params: *mut GLint,
    ) {
        self.entry_gl45.glGetNamedFramebufferAttachmentParameteriv(
            _framebuffer,
            _attachment,
            _pname,
            _params,
        );
    }
    pub unsafe fn glGetQueryBufferObjectui64v(
        &self,
        _id: GLuint,
        _buffer: GLuint,
        _pname: QueryObjectParameterName,
        _offset: GLintptr,
    ) {
        self.entry_gl45
            .glGetQueryBufferObjectui64v(_id, _buffer, _pname, _offset);
    }
    pub unsafe fn glNamedFramebufferReadBuffer(&self, _framebuffer: GLuint, _src: ColorBuffer) {
        self.entry_gl45
            .glNamedFramebufferReadBuffer(_framebuffer, _src);
    }
    pub unsafe fn glClearNamedFramebufferuiv(
        &self,
        _framebuffer: GLuint,
        _buffer: Buffer,
        _drawbuffer: GLint,
        _value: *const GLuint,
    ) {
        self.entry_gl45
            .glClearNamedFramebufferuiv(_framebuffer, _buffer, _drawbuffer, _value);
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
        self.entry_gl45.glTextureStorage3D(
            _texture,
            _levels,
            _internalformat,
            _width,
            _height,
            _depth,
        );
    }
    pub unsafe fn glCreateProgramPipelines(&self, _n: GLsizei, _pipelines: *mut GLuint) {
        self.entry_gl45.glCreateProgramPipelines(_n, _pipelines);
    }
    pub unsafe fn glGetnPixelMapusv(
        &self,
        _map: PixelMap,
        _bufSize: GLsizei,
        _values: *mut GLushort,
    ) {
        self.entry_gl45.glGetnPixelMapusv(_map, _bufSize, _values);
    }
    pub unsafe fn glTransformFeedbackBufferRange(
        &self,
        _xfb: GLuint,
        _index: GLuint,
        _buffer: GLuint,
        _offset: GLintptr,
        _size: GLsizeiptr,
    ) {
        self.entry_gl45
            .glTransformFeedbackBufferRange(_xfb, _index, _buffer, _offset, _size);
    }
    pub unsafe fn glGetNamedBufferParameteriv(
        &self,
        _buffer: GLuint,
        _pname: BufferPNameARB,
        _params: *mut GLint,
    ) {
        self.entry_gl45
            .glGetNamedBufferParameteriv(_buffer, _pname, _params);
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
        self.entry_gl45
            .glGetnMinmax(_target, _reset, _format, _type, _bufSize, _values);
    }
    pub unsafe fn glClearNamedBufferData(
        &self,
        _buffer: GLuint,
        _internalformat: SizedInternalFormat,
        _format: PixelFormat,
        _type: PixelType,
        _data: *const std::os::raw::c_void,
    ) {
        self.entry_gl45
            .glClearNamedBufferData(_buffer, _internalformat, _format, _type, _data);
    }
    pub unsafe fn glGetVertexArrayiv(
        &self,
        _vaobj: GLuint,
        _pname: VertexArrayPName,
        _param: *mut GLint,
    ) {
        self.entry_gl45.glGetVertexArrayiv(_vaobj, _pname, _param);
    }
    pub unsafe fn glGetnUniformuiv(
        &self,
        _program: GLuint,
        _location: GLint,
        _bufSize: GLsizei,
        _params: *mut GLuint,
    ) {
        self.entry_gl45
            .glGetnUniformuiv(_program, _location, _bufSize, _params);
    }
    pub unsafe fn glGetnCompressedTexImage(
        &self,
        _target: TextureTarget,
        _lod: GLint,
        _bufSize: GLsizei,
        _pixels: *mut std::os::raw::c_void,
    ) {
        self.entry_gl45
            .glGetnCompressedTexImage(_target, _lod, _bufSize, _pixels);
    }
    pub unsafe fn glClearNamedFramebufferfi(
        &self,
        _framebuffer: GLuint,
        _buffer: Buffer,
        _drawbuffer: GLint,
        _depth: GLfloat,
        _stencil: GLint,
    ) {
        self.entry_gl45.glClearNamedFramebufferfi(
            _framebuffer,
            _buffer,
            _drawbuffer,
            _depth,
            _stencil,
        );
    }
    pub unsafe fn glDisableVertexArrayAttrib(&self, _vaobj: GLuint, _index: GLuint) {
        self.entry_gl45.glDisableVertexArrayAttrib(_vaobj, _index);
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
        self.entry_gl45
            .glVertexArrayVertexBuffers(_vaobj, _first, _count, _buffers, _offsets, _strides);
    }
    pub unsafe fn glUnmapNamedBuffer(&self, _buffer: GLuint) -> GLboolean {
        self.entry_gl45.glUnmapNamedBuffer(_buffer)
    }
    pub unsafe fn glGetnTexImage(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _format: PixelFormat,
        _type: PixelType,
        _bufSize: GLsizei,
        _pixels: *mut std::os::raw::c_void,
    ) {
        self.entry_gl45
            .glGetnTexImage(_target, _level, _format, _type, _bufSize, _pixels);
    }
    pub unsafe fn glMapNamedBuffer(
        &self,
        _buffer: GLuint,
        _access: BufferAccessARB,
    ) -> *mut std::os::raw::c_void {
        self.entry_gl45.glMapNamedBuffer(_buffer, _access)
    }
    pub unsafe fn glTextureParameterf(
        &self,
        _texture: GLuint,
        _pname: TextureParameterName,
        _param: GLfloat,
    ) {
        self.entry_gl45
            .glTextureParameterf(_texture, _pname, _param);
    }
    pub unsafe fn glVertexArrayAttribIFormat(
        &self,
        _vaobj: GLuint,
        _attribindex: GLuint,
        _size: GLint,
        _type: VertexAttribIType,
        _relativeoffset: GLuint,
    ) {
        self.entry_gl45.glVertexArrayAttribIFormat(
            _vaobj,
            _attribindex,
            _size,
            _type,
            _relativeoffset,
        );
    }
    pub unsafe fn glCreateBuffers(&self, _n: GLsizei, _buffers: *mut GLuint) {
        self.entry_gl45.glCreateBuffers(_n, _buffers);
    }
    pub unsafe fn glTextureStorage2DMultisample(
        &self,
        _texture: GLuint,
        _samples: GLsizei,
        _internalformat: SizedInternalFormat,
        _width: GLsizei,
        _height: GLsizei,
        _fixedsamplelocations: GLboolean,
    ) {
        self.entry_gl45.glTextureStorage2DMultisample(
            _texture,
            _samples,
            _internalformat,
            _width,
            _height,
            _fixedsamplelocations,
        );
    }
    pub unsafe fn glGetTextureParameterIiv(
        &self,
        _texture: GLuint,
        _pname: GetTextureParameter,
        _params: *mut GLint,
    ) {
        self.entry_gl45
            .glGetTextureParameterIiv(_texture, _pname, _params);
    }
    pub unsafe fn glVertexArrayBindingDivisor(
        &self,
        _vaobj: GLuint,
        _bindingindex: GLuint,
        _divisor: GLuint,
    ) {
        self.entry_gl45
            .glVertexArrayBindingDivisor(_vaobj, _bindingindex, _divisor);
    }
    pub unsafe fn glGetTextureLevelParameterfv(
        &self,
        _texture: GLuint,
        _level: GLint,
        _pname: GetTextureParameter,
        _params: *mut GLfloat,
    ) {
        self.entry_gl45
            .glGetTextureLevelParameterfv(_texture, _level, _pname, _params);
    }
    pub unsafe fn glGetnPixelMapfv(
        &self,
        _map: PixelMap,
        _bufSize: GLsizei,
        _values: *mut GLfloat,
    ) {
        self.entry_gl45.glGetnPixelMapfv(_map, _bufSize, _values);
    }
    pub unsafe fn glGenerateTextureMipmap(&self, _texture: GLuint) {
        self.entry_gl45.glGenerateTextureMipmap(_texture);
    }
    pub unsafe fn glTransformFeedbackBufferBase(
        &self,
        _xfb: GLuint,
        _index: GLuint,
        _buffer: GLuint,
    ) {
        self.entry_gl45
            .glTransformFeedbackBufferBase(_xfb, _index, _buffer);
    }
    pub unsafe fn glCreateRenderbuffers(&self, _n: GLsizei, _renderbuffers: *mut GLuint) {
        self.entry_gl45.glCreateRenderbuffers(_n, _renderbuffers);
    }
    pub unsafe fn glEnableVertexArrayAttrib(&self, _vaobj: GLuint, _index: GLuint) {
        self.entry_gl45.glEnableVertexArrayAttrib(_vaobj, _index);
    }
    pub unsafe fn glGetNamedBufferPointerv(
        &self,
        _buffer: GLuint,
        _pname: BufferPointerNameARB,
        _params: *mut *mut std::os::raw::c_void,
    ) {
        self.entry_gl45
            .glGetNamedBufferPointerv(_buffer, _pname, _params);
    }
    pub unsafe fn glTextureParameterIiv(
        &self,
        _texture: GLuint,
        _pname: TextureParameterName,
        _params: *const GLint,
    ) {
        self.entry_gl45
            .glTextureParameterIiv(_texture, _pname, _params);
    }
    pub unsafe fn glGetTextureParameterIuiv(
        &self,
        _texture: GLuint,
        _pname: GetTextureParameter,
        _params: *mut GLuint,
    ) {
        self.entry_gl45
            .glGetTextureParameterIuiv(_texture, _pname, _params);
    }
    pub unsafe fn glVertexArrayAttribLFormat(
        &self,
        _vaobj: GLuint,
        _attribindex: GLuint,
        _size: GLint,
        _type: VertexAttribLType,
        _relativeoffset: GLuint,
    ) {
        self.entry_gl45.glVertexArrayAttribLFormat(
            _vaobj,
            _attribindex,
            _size,
            _type,
            _relativeoffset,
        );
    }
    pub unsafe fn glGetnHistogram(
        &self,
        _target: HistogramTarget,
        _reset: GLboolean,
        _format: PixelFormat,
        _type: PixelType,
        _bufSize: GLsizei,
        _values: *mut std::os::raw::c_void,
    ) {
        self.entry_gl45
            .glGetnHistogram(_target, _reset, _format, _type, _bufSize, _values);
    }
    pub unsafe fn glTextureBarrier(&self) {
        self.entry_gl45.glTextureBarrier();
    }
    pub unsafe fn glCheckNamedFramebufferStatus(
        &self,
        _framebuffer: GLuint,
        _target: FramebufferTarget,
    ) -> GLenum {
        self.entry_gl45
            .glCheckNamedFramebufferStatus(_framebuffer, _target)
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
        self.entry_gl45.glTextureSubImage3D(
            _texture, _level, _xoffset, _yoffset, _zoffset, _width, _height, _depth, _format,
            _type, _pixels,
        );
    }
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
        self.entry_gl45.glClearNamedBufferSubData(
            _buffer,
            _internalformat,
            _offset,
            _size,
            _format,
            _type,
            _data,
        );
    }
    pub unsafe fn glFlushMappedNamedBufferRange(
        &self,
        _buffer: GLuint,
        _offset: GLintptr,
        _length: GLsizeiptr,
    ) {
        self.entry_gl45
            .glFlushMappedNamedBufferRange(_buffer, _offset, _length);
    }
    pub unsafe fn glTextureParameterIuiv(
        &self,
        _texture: GLuint,
        _pname: TextureParameterName,
        _params: *const GLuint,
    ) {
        self.entry_gl45
            .glTextureParameterIuiv(_texture, _pname, _params);
    }
    pub unsafe fn glTextureStorage2D(
        &self,
        _texture: GLuint,
        _levels: GLsizei,
        _internalformat: SizedInternalFormat,
        _width: GLsizei,
        _height: GLsizei,
    ) {
        self.entry_gl45
            .glTextureStorage2D(_texture, _levels, _internalformat, _width, _height);
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
        self.entry_gl45
            .glTextureSubImage1D(_texture, _level, _xoffset, _width, _format, _type, _pixels);
    }
    pub unsafe fn glCopyTextureSubImage1D(
        &self,
        _texture: GLuint,
        _level: GLint,
        _xoffset: GLint,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
    ) {
        self.entry_gl45
            .glCopyTextureSubImage1D(_texture, _level, _xoffset, _x, _y, _width);
    }
    pub unsafe fn glGetNamedFramebufferParameteriv(
        &self,
        _framebuffer: GLuint,
        _pname: GetFramebufferParameter,
        _param: *mut GLint,
    ) {
        self.entry_gl45
            .glGetNamedFramebufferParameteriv(_framebuffer, _pname, _param);
    }
    pub unsafe fn glGetTextureParameterfv(
        &self,
        _texture: GLuint,
        _pname: GetTextureParameter,
        _params: *mut GLfloat,
    ) {
        self.entry_gl45
            .glGetTextureParameterfv(_texture, _pname, _params);
    }
    pub unsafe fn glGetnConvolutionFilter(
        &self,
        _target: ConvolutionTarget,
        _format: PixelFormat,
        _type: PixelType,
        _bufSize: GLsizei,
        _image: *mut std::os::raw::c_void,
    ) {
        self.entry_gl45
            .glGetnConvolutionFilter(_target, _format, _type, _bufSize, _image);
    }
    pub unsafe fn glNamedFramebufferTexture(
        &self,
        _framebuffer: GLuint,
        _attachment: FramebufferAttachment,
        _texture: GLuint,
        _level: GLint,
    ) {
        self.entry_gl45
            .glNamedFramebufferTexture(_framebuffer, _attachment, _texture, _level);
    }
    pub unsafe fn glGetTextureParameteriv(
        &self,
        _texture: GLuint,
        _pname: GetTextureParameter,
        _params: *mut GLint,
    ) {
        self.entry_gl45
            .glGetTextureParameteriv(_texture, _pname, _params);
    }
    pub unsafe fn glNamedBufferData(
        &self,
        _buffer: GLuint,
        _size: GLsizeiptr,
        _data: *const std::os::raw::c_void,
        _usage: VertexBufferObjectUsage,
    ) {
        self.entry_gl45
            .glNamedBufferData(_buffer, _size, _data, _usage);
    }
    pub unsafe fn glTextureParameteriv(
        &self,
        _texture: GLuint,
        _pname: TextureParameterName,
        _param: *const GLint,
    ) {
        self.entry_gl45
            .glTextureParameteriv(_texture, _pname, _param);
    }
    pub unsafe fn glBindTextureUnit(&self, _unit: GLuint, _texture: GLuint) {
        self.entry_gl45.glBindTextureUnit(_unit, _texture);
    }
    pub unsafe fn glGetnMapfv(
        &self,
        _target: MapTarget,
        _query: MapQuery,
        _bufSize: GLsizei,
        _v: *mut GLfloat,
    ) {
        self.entry_gl45.glGetnMapfv(_target, _query, _bufSize, _v);
    }
    pub unsafe fn glGetnColorTable(
        &self,
        _target: ColorTableTarget,
        _format: PixelFormat,
        _type: PixelType,
        _bufSize: GLsizei,
        _table: *mut std::os::raw::c_void,
    ) {
        self.entry_gl45
            .glGetnColorTable(_target, _format, _type, _bufSize, _table);
    }
    pub unsafe fn glTextureParameterfv(
        &self,
        _texture: GLuint,
        _pname: TextureParameterName,
        _param: *const GLfloat,
    ) {
        self.entry_gl45
            .glTextureParameterfv(_texture, _pname, _param);
    }
    pub unsafe fn glVertexAttribI2ui(&self, _index: GLuint, _x: GLuint, _y: GLuint) {
        self.entry_gl30.glVertexAttribI2ui(_index, _x, _y);
    }
    pub unsafe fn glVertexAttribI4i(
        &self,
        _index: GLuint,
        _x: GLint,
        _y: GLint,
        _z: GLint,
        _w: GLint,
    ) {
        self.entry_gl30.glVertexAttribI4i(_index, _x, _y, _z, _w);
    }
    pub unsafe fn glGenRenderbuffers(&self, _n: GLsizei, _renderbuffers: *mut GLuint) {
        self.entry_gl30.glGenRenderbuffers(_n, _renderbuffers);
    }
    pub unsafe fn glRenderbufferStorage(
        &self,
        _target: RenderbufferTarget,
        _internalformat: InternalFormat,
        _width: GLsizei,
        _height: GLsizei,
    ) {
        self.entry_gl30
            .glRenderbufferStorage(_target, _internalformat, _width, _height);
    }
    pub unsafe fn glFramebufferTexture2D(
        &self,
        _target: FramebufferTarget,
        _attachment: FramebufferAttachment,
        _textarget: TextureTarget,
        _texture: GLuint,
        _level: GLint,
    ) {
        self.entry_gl30
            .glFramebufferTexture2D(_target, _attachment, _textarget, _texture, _level);
    }
    pub unsafe fn glVertexAttribI3iv(&self, _index: GLuint, _v: *const GLint) {
        self.entry_gl30.glVertexAttribI3iv(_index, _v);
    }
    pub unsafe fn glVertexAttribI4ubv(&self, _index: GLuint, _v: *const GLubyte) {
        self.entry_gl30.glVertexAttribI4ubv(_index, _v);
    }
    pub unsafe fn glVertexAttribI2i(&self, _index: GLuint, _x: GLint, _y: GLint) {
        self.entry_gl30.glVertexAttribI2i(_index, _x, _y);
    }
    pub unsafe fn glVertexAttribI4ui(
        &self,
        _index: GLuint,
        _x: GLuint,
        _y: GLuint,
        _z: GLuint,
        _w: GLuint,
    ) {
        self.entry_gl30.glVertexAttribI4ui(_index, _x, _y, _z, _w);
    }
    pub unsafe fn glBindFramebuffer(&self, _target: FramebufferTarget, _framebuffer: GLuint) {
        self.entry_gl30.glBindFramebuffer(_target, _framebuffer);
    }
    pub unsafe fn glBindVertexArray(&self, _array: GLuint) {
        self.entry_gl30.glBindVertexArray(_array);
    }
    pub unsafe fn glGetFragDataLocation(&self, _program: GLuint, _name: *const GLchar) -> GLint {
        self.entry_gl30.glGetFragDataLocation(_program, _name)
    }
    pub unsafe fn glRenderbufferStorageMultisample(
        &self,
        _target: RenderbufferTarget,
        _samples: GLsizei,
        _internalformat: InternalFormat,
        _width: GLsizei,
        _height: GLsizei,
    ) {
        self.entry_gl30.glRenderbufferStorageMultisample(
            _target,
            _samples,
            _internalformat,
            _width,
            _height,
        );
    }
    pub unsafe fn glEnablei(&self, _target: EnableCap, _index: GLuint) {
        self.entry_gl30.glEnablei(_target, _index);
    }
    pub unsafe fn glClearBufferiv(
        &self,
        _buffer: Buffer,
        _drawbuffer: GLint,
        _value: *const GLint,
    ) {
        self.entry_gl30
            .glClearBufferiv(_buffer, _drawbuffer, _value);
    }
    pub unsafe fn glUniform1uiv(&self, _location: GLint, _count: GLsizei, _value: *const GLuint) {
        self.entry_gl30.glUniform1uiv(_location, _count, _value);
    }
    pub unsafe fn glVertexAttribI3uiv(&self, _index: GLuint, _v: *const GLuint) {
        self.entry_gl30.glVertexAttribI3uiv(_index, _v);
    }
    pub unsafe fn glGetTexParameterIiv(
        &self,
        _target: TextureTarget,
        _pname: GetTextureParameter,
        _params: *mut GLint,
    ) {
        self.entry_gl30
            .glGetTexParameterIiv(_target, _pname, _params);
    }
    pub unsafe fn glVertexAttribI4iv(&self, _index: GLuint, _v: *const GLint) {
        self.entry_gl30.glVertexAttribI4iv(_index, _v);
    }
    pub unsafe fn glVertexAttribI3i(&self, _index: GLuint, _x: GLint, _y: GLint, _z: GLint) {
        self.entry_gl30.glVertexAttribI3i(_index, _x, _y, _z);
    }
    pub unsafe fn glVertexAttribI2uiv(&self, _index: GLuint, _v: *const GLuint) {
        self.entry_gl30.glVertexAttribI2uiv(_index, _v);
    }
    pub unsafe fn glVertexAttribI1uiv(&self, _index: GLuint, _v: *const GLuint) {
        self.entry_gl30.glVertexAttribI1uiv(_index, _v);
    }
    pub unsafe fn glGetFramebufferAttachmentParameteriv(
        &self,
        _target: FramebufferTarget,
        _attachment: FramebufferAttachment,
        _pname: FramebufferAttachmentParameterName,
        _params: *mut GLint,
    ) {
        self.entry_gl30.glGetFramebufferAttachmentParameteriv(
            _target,
            _attachment,
            _pname,
            _params,
        );
    }
    pub unsafe fn glBindFragDataLocation(
        &self,
        _program: GLuint,
        _color: GLuint,
        _name: *const GLchar,
    ) {
        self.entry_gl30
            .glBindFragDataLocation(_program, _color, _name);
    }
    pub unsafe fn glGetVertexAttribIiv(
        &self,
        _index: GLuint,
        _pname: VertexAttribEnum,
        _params: *mut GLint,
    ) {
        self.entry_gl30
            .glGetVertexAttribIiv(_index, _pname, _params);
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
        self.entry_gl30.glBlitFramebuffer(
            _srcX0, _srcY0, _srcX1, _srcY1, _dstX0, _dstY0, _dstX1, _dstY1, _mask, _filter,
        );
    }
    pub unsafe fn glFlushMappedBufferRange(
        &self,
        _target: BufferTargetARB,
        _offset: GLintptr,
        _length: GLsizeiptr,
    ) {
        self.entry_gl30
            .glFlushMappedBufferRange(_target, _offset, _length);
    }
    pub unsafe fn glUniform4uiv(&self, _location: GLint, _count: GLsizei, _value: *const GLuint) {
        self.entry_gl30.glUniform4uiv(_location, _count, _value);
    }
    pub unsafe fn glGetUniformuiv(&self, _program: GLuint, _location: GLint, _params: *mut GLuint) {
        self.entry_gl30
            .glGetUniformuiv(_program, _location, _params);
    }
    pub unsafe fn glUniform1ui(&self, _location: GLint, _v0: GLuint) {
        self.entry_gl30.glUniform1ui(_location, _v0);
    }
    pub unsafe fn glUniform4ui(
        &self,
        _location: GLint,
        _v0: GLuint,
        _v1: GLuint,
        _v2: GLuint,
        _v3: GLuint,
    ) {
        self.entry_gl30.glUniform4ui(_location, _v0, _v1, _v2, _v3);
    }
    pub unsafe fn glClearBufferfi(
        &self,
        _buffer: Buffer,
        _drawbuffer: GLint,
        _depth: GLfloat,
        _stencil: GLint,
    ) {
        self.entry_gl30
            .glClearBufferfi(_buffer, _drawbuffer, _depth, _stencil);
    }
    pub unsafe fn glVertexAttribI1ui(&self, _index: GLuint, _x: GLuint) {
        self.entry_gl30.glVertexAttribI1ui(_index, _x);
    }
    pub unsafe fn glDeleteRenderbuffers(&self, _n: GLsizei, _renderbuffers: *const GLuint) {
        self.entry_gl30.glDeleteRenderbuffers(_n, _renderbuffers);
    }
    pub unsafe fn glGetRenderbufferParameteriv(
        &self,
        _target: RenderbufferTarget,
        _pname: RenderbufferParameterName,
        _params: *mut GLint,
    ) {
        self.entry_gl30
            .glGetRenderbufferParameteriv(_target, _pname, _params);
    }
    pub unsafe fn glVertexAttribI1i(&self, _index: GLuint, _x: GLint) {
        self.entry_gl30.glVertexAttribI1i(_index, _x);
    }
    pub unsafe fn glGenFramebuffers(&self, _n: GLsizei, _framebuffers: *mut GLuint) {
        self.entry_gl30.glGenFramebuffers(_n, _framebuffers);
    }
    pub unsafe fn glFramebufferRenderbuffer(
        &self,
        _target: FramebufferTarget,
        _attachment: FramebufferAttachment,
        _renderbuffertarget: RenderbufferTarget,
        _renderbuffer: GLuint,
    ) {
        self.entry_gl30.glFramebufferRenderbuffer(
            _target,
            _attachment,
            _renderbuffertarget,
            _renderbuffer,
        );
    }
    pub unsafe fn glVertexAttribI4bv(&self, _index: GLuint, _v: *const GLbyte) {
        self.entry_gl30.glVertexAttribI4bv(_index, _v);
    }
    pub unsafe fn glMapBufferRange(
        &self,
        _target: BufferTargetARB,
        _offset: GLintptr,
        _length: GLsizeiptr,
        _access: MapBufferAccessMask,
    ) -> *mut std::os::raw::c_void {
        self.entry_gl30
            .glMapBufferRange(_target, _offset, _length, _access)
    }
    pub unsafe fn glGenVertexArrays(&self, _n: GLsizei, _arrays: *mut GLuint) {
        self.entry_gl30.glGenVertexArrays(_n, _arrays);
    }
    pub unsafe fn glColorMaski(
        &self,
        _index: GLuint,
        _r: GLboolean,
        _g: GLboolean,
        _b: GLboolean,
        _a: GLboolean,
    ) {
        self.entry_gl30.glColorMaski(_index, _r, _g, _b, _a);
    }
    pub unsafe fn glGetStringi(&self, _name: StringName, _index: GLuint) -> *const GLubyte {
        self.entry_gl30.glGetStringi(_name, _index)
    }
    pub unsafe fn glClearBufferfv(
        &self,
        _buffer: Buffer,
        _drawbuffer: GLint,
        _value: *const GLfloat,
    ) {
        self.entry_gl30
            .glClearBufferfv(_buffer, _drawbuffer, _value);
    }
    pub unsafe fn glVertexAttribI4uiv(&self, _index: GLuint, _v: *const GLuint) {
        self.entry_gl30.glVertexAttribI4uiv(_index, _v);
    }
    pub unsafe fn glEndTransformFeedback(&self) {
        self.entry_gl30.glEndTransformFeedback();
    }
    pub unsafe fn glDeleteVertexArrays(&self, _n: GLsizei, _arrays: *const GLuint) {
        self.entry_gl30.glDeleteVertexArrays(_n, _arrays);
    }
    pub unsafe fn glUniform2ui(&self, _location: GLint, _v0: GLuint, _v1: GLuint) {
        self.entry_gl30.glUniform2ui(_location, _v0, _v1);
    }
    pub unsafe fn glEndConditionalRender(&self) {
        self.entry_gl30.glEndConditionalRender();
    }
    pub unsafe fn glGetTexParameterIuiv(
        &self,
        _target: TextureTarget,
        _pname: GetTextureParameter,
        _params: *mut GLuint,
    ) {
        self.entry_gl30
            .glGetTexParameterIuiv(_target, _pname, _params);
    }
    pub unsafe fn glDisablei(&self, _target: EnableCap, _index: GLuint) {
        self.entry_gl30.glDisablei(_target, _index);
    }
    pub unsafe fn glBindBufferBase(
        &self,
        _target: BufferTargetARB,
        _index: GLuint,
        _buffer: GLuint,
    ) {
        self.entry_gl30.glBindBufferBase(_target, _index, _buffer);
    }
    pub unsafe fn glTexParameterIuiv(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _params: *const GLuint,
    ) {
        self.entry_gl30.glTexParameterIuiv(_target, _pname, _params);
    }
    pub unsafe fn glIsFramebuffer(&self, _framebuffer: GLuint) -> GLboolean {
        self.entry_gl30.glIsFramebuffer(_framebuffer)
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
        self.entry_gl30.glGetTransformFeedbackVarying(
            _program, _index, _bufSize, _length, _size, _type, _name,
        );
    }
    pub unsafe fn glTransformFeedbackVaryings(
        &self,
        _program: GLuint,
        _count: GLsizei,
        _varyings: *const *const GLchar,
        _bufferMode: TransformFeedbackBufferMode,
    ) {
        self.entry_gl30
            .glTransformFeedbackVaryings(_program, _count, _varyings, _bufferMode);
    }
    pub unsafe fn glUniform3ui(&self, _location: GLint, _v0: GLuint, _v1: GLuint, _v2: GLuint) {
        self.entry_gl30.glUniform3ui(_location, _v0, _v1, _v2);
    }
    pub unsafe fn glVertexAttribIPointer(
        &self,
        _index: GLuint,
        _size: GLint,
        _type: VertexAttribIType,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        self.entry_gl30
            .glVertexAttribIPointer(_index, _size, _type, _stride, _pointer);
    }
    pub unsafe fn glVertexAttribI1iv(&self, _index: GLuint, _v: *const GLint) {
        self.entry_gl30.glVertexAttribI1iv(_index, _v);
    }
    pub unsafe fn glGetBooleani_v(
        &self,
        _target: BufferTargetARB,
        _index: GLuint,
        _data: *mut GLboolean,
    ) {
        self.entry_gl30.glGetBooleani_v(_target, _index, _data);
    }
    pub unsafe fn glIsEnabledi(&self, _target: EnableCap, _index: GLuint) -> GLboolean {
        self.entry_gl30.glIsEnabledi(_target, _index)
    }

    pub unsafe fn glGenerateMipmap(&self, _target: TextureTarget) {
        self.entry_gl30.glGenerateMipmap(_target);
    }
    pub unsafe fn glFramebufferTextureLayer(
        &self,
        _target: FramebufferTarget,
        _attachment: FramebufferAttachment,
        _texture: GLuint,
        _level: GLint,
        _layer: GLint,
    ) {
        self.entry_gl30
            .glFramebufferTextureLayer(_target, _attachment, _texture, _level, _layer);
    }
    pub unsafe fn glClampColor(&self, _target: ClampColorTargetARB, _clamp: ClampColorModeARB) {
        self.entry_gl30.glClampColor(_target, _clamp);
    }
    pub unsafe fn glVertexAttribI4sv(&self, _index: GLuint, _v: *const GLshort) {
        self.entry_gl30.glVertexAttribI4sv(_index, _v);
    }
    pub unsafe fn glIsVertexArray(&self, _array: GLuint) -> GLboolean {
        self.entry_gl30.glIsVertexArray(_array)
    }
    pub unsafe fn glVertexAttribI2iv(&self, _index: GLuint, _v: *const GLint) {
        self.entry_gl30.glVertexAttribI2iv(_index, _v);
    }
    pub unsafe fn glVertexAttribI4usv(&self, _index: GLuint, _v: *const GLushort) {
        self.entry_gl30.glVertexAttribI4usv(_index, _v);
    }
    pub unsafe fn glBeginTransformFeedback(&self, _primitiveMode: PrimitiveType) {
        self.entry_gl30.glBeginTransformFeedback(_primitiveMode);
    }
    pub unsafe fn glFramebufferTexture3D(
        &self,
        _target: FramebufferTarget,
        _attachment: FramebufferAttachment,
        _textarget: TextureTarget,
        _texture: GLuint,
        _level: GLint,
        _zoffset: GLint,
    ) {
        self.entry_gl30.glFramebufferTexture3D(
            _target,
            _attachment,
            _textarget,
            _texture,
            _level,
            _zoffset,
        );
    }
    pub unsafe fn glFramebufferTexture1D(
        &self,
        _target: FramebufferTarget,
        _attachment: FramebufferAttachment,
        _textarget: TextureTarget,
        _texture: GLuint,
        _level: GLint,
    ) {
        self.entry_gl30
            .glFramebufferTexture1D(_target, _attachment, _textarget, _texture, _level);
    }
    pub unsafe fn glCheckFramebufferStatus(&self, _target: FramebufferTarget) -> GLenum {
        self.entry_gl30.glCheckFramebufferStatus(_target)
    }
    pub unsafe fn glIsRenderbuffer(&self, _renderbuffer: GLuint) -> GLboolean {
        self.entry_gl30.glIsRenderbuffer(_renderbuffer)
    }
    pub unsafe fn glBeginConditionalRender(&self, _id: GLuint, _mode: ConditionalRenderMode) {
        self.entry_gl30.glBeginConditionalRender(_id, _mode);
    }
    pub unsafe fn glDeleteFramebuffers(&self, _n: GLsizei, _framebuffers: *const GLuint) {
        self.entry_gl30.glDeleteFramebuffers(_n, _framebuffers);
    }
    pub unsafe fn glTexParameterIiv(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _params: *const GLint,
    ) {
        self.entry_gl30.glTexParameterIiv(_target, _pname, _params);
    }
    pub unsafe fn glBindRenderbuffer(&self, _target: RenderbufferTarget, _renderbuffer: GLuint) {
        self.entry_gl30.glBindRenderbuffer(_target, _renderbuffer);
    }
    pub unsafe fn glVertexAttribI3ui(&self, _index: GLuint, _x: GLuint, _y: GLuint, _z: GLuint) {
        self.entry_gl30.glVertexAttribI3ui(_index, _x, _y, _z);
    }
    pub unsafe fn glUniform3uiv(&self, _location: GLint, _count: GLsizei, _value: *const GLuint) {
        self.entry_gl30.glUniform3uiv(_location, _count, _value);
    }
    pub unsafe fn glClearBufferuiv(
        &self,
        _buffer: Buffer,
        _drawbuffer: GLint,
        _value: *const GLuint,
    ) {
        self.entry_gl30
            .glClearBufferuiv(_buffer, _drawbuffer, _value);
    }
    pub unsafe fn glUniform2uiv(&self, _location: GLint, _count: GLsizei, _value: *const GLuint) {
        self.entry_gl30.glUniform2uiv(_location, _count, _value);
    }
    pub unsafe fn glGetVertexAttribIuiv(
        &self,
        _index: GLuint,
        _pname: VertexAttribEnum,
        _params: *mut GLuint,
    ) {
        self.entry_gl30
            .glGetVertexAttribIuiv(_index, _pname, _params);
    }
    pub unsafe fn glProgramUniform1i(&self, _program: GLuint, _location: GLint, _v0: GLint) {
        self.entry_gl41.glProgramUniform1i(_program, _location, _v0);
    }
    pub unsafe fn glViewportIndexedf(
        &self,
        _index: GLuint,
        _x: GLfloat,
        _y: GLfloat,
        _w: GLfloat,
        _h: GLfloat,
    ) {
        self.entry_gl41.glViewportIndexedf(_index, _x, _y, _w, _h);
    }
    pub unsafe fn glProgramUniformMatrix2x3dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        self.entry_gl41
            .glProgramUniformMatrix2x3dv(_program, _location, _count, _transpose, _value);
    }
    pub unsafe fn glProgramUniform1uiv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLuint,
    ) {
        self.entry_gl41
            .glProgramUniform1uiv(_program, _location, _count, _value);
    }
    pub unsafe fn glVertexAttribL2dv(&self, _index: GLuint, _v: *const GLdouble) {
        self.entry_gl41.glVertexAttribL2dv(_index, _v);
    }
    pub unsafe fn glProgramUniform3d(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLdouble,
        _v1: GLdouble,
        _v2: GLdouble,
    ) {
        self.entry_gl41
            .glProgramUniform3d(_program, _location, _v0, _v1, _v2);
    }
    pub unsafe fn glVertexAttribL3dv(&self, _index: GLuint, _v: *const GLdouble) {
        self.entry_gl41.glVertexAttribL3dv(_index, _v);
    }
    pub unsafe fn glProgramUniform2iv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLint,
    ) {
        self.entry_gl41
            .glProgramUniform2iv(_program, _location, _count, _value);
    }
    pub unsafe fn glGetDoublei_v(&self, _target: GetPName, _index: GLuint, _data: *mut GLdouble) {
        self.entry_gl41.glGetDoublei_v(_target, _index, _data);
    }
    pub unsafe fn glClearDepthf(&self, _d: GLfloat) {
        self.entry_gl41.glClearDepthf(_d);
    }
    pub unsafe fn glProgramUniform2d(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLdouble,
        _v1: GLdouble,
    ) {
        self.entry_gl41
            .glProgramUniform2d(_program, _location, _v0, _v1);
    }
    pub unsafe fn glGetProgramPipelineInfoLog(
        &self,
        _pipeline: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _infoLog: *mut GLchar,
    ) {
        self.entry_gl41
            .glGetProgramPipelineInfoLog(_pipeline, _bufSize, _length, _infoLog);
    }
    pub unsafe fn glValidateProgramPipeline(&self, _pipeline: GLuint) {
        self.entry_gl41.glValidateProgramPipeline(_pipeline);
    }
    pub unsafe fn glProgramUniform3i(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLint,
        _v1: GLint,
        _v2: GLint,
    ) {
        self.entry_gl41
            .glProgramUniform3i(_program, _location, _v0, _v1, _v2);
    }
    pub unsafe fn glGetVertexAttribLdv(
        &self,
        _index: GLuint,
        _pname: VertexAttribEnum,
        _params: *mut GLdouble,
    ) {
        self.entry_gl41
            .glGetVertexAttribLdv(_index, _pname, _params);
    }
    pub unsafe fn glProgramUniform3fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLfloat,
    ) {
        self.entry_gl41
            .glProgramUniform3fv(_program, _location, _count, _value);
    }
    pub unsafe fn glProgramUniform2i(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLint,
        _v1: GLint,
    ) {
        self.entry_gl41
            .glProgramUniform2i(_program, _location, _v0, _v1);
    }
    pub unsafe fn glProgramUniform4f(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLfloat,
        _v1: GLfloat,
        _v2: GLfloat,
        _v3: GLfloat,
    ) {
        self.entry_gl41
            .glProgramUniform4f(_program, _location, _v0, _v1, _v2, _v3);
    }
    pub unsafe fn glVertexAttribL3d(
        &self,
        _index: GLuint,
        _x: GLdouble,
        _y: GLdouble,
        _z: GLdouble,
    ) {
        self.entry_gl41.glVertexAttribL3d(_index, _x, _y, _z);
    }
    pub unsafe fn glProgramUniform2dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLdouble,
    ) {
        self.entry_gl41
            .glProgramUniform2dv(_program, _location, _count, _value);
    }
    pub unsafe fn glProgramUniformMatrix4x2fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        self.entry_gl41
            .glProgramUniformMatrix4x2fv(_program, _location, _count, _transpose, _value);
    }
    pub unsafe fn glBindProgramPipeline(&self, _pipeline: GLuint) {
        self.entry_gl41.glBindProgramPipeline(_pipeline);
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
        self.entry_gl41
            .glProgramUniform4d(_program, _location, _v0, _v1, _v2, _v3);
    }
    pub unsafe fn glProgramUniformMatrix3x4dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        self.entry_gl41
            .glProgramUniformMatrix3x4dv(_program, _location, _count, _transpose, _value);
    }
    pub unsafe fn glProgramUniformMatrix4dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        self.entry_gl41
            .glProgramUniformMatrix4dv(_program, _location, _count, _transpose, _value);
    }
    pub unsafe fn glVertexAttribL1dv(&self, _index: GLuint, _v: *const GLdouble) {
        self.entry_gl41.glVertexAttribL1dv(_index, _v);
    }
    pub unsafe fn glReleaseShaderCompiler(&self) {
        self.entry_gl41.glReleaseShaderCompiler();
    }
    pub unsafe fn glProgramUniform3dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLdouble,
    ) {
        self.entry_gl41
            .glProgramUniform3dv(_program, _location, _count, _value);
    }
    pub unsafe fn glVertexAttribL4dv(&self, _index: GLuint, _v: *const GLdouble) {
        self.entry_gl41.glVertexAttribL4dv(_index, _v);
    }
    pub unsafe fn glProgramUniform3ui(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLuint,
        _v1: GLuint,
        _v2: GLuint,
    ) {
        self.entry_gl41
            .glProgramUniform3ui(_program, _location, _v0, _v1, _v2);
    }
    pub unsafe fn glGetProgramBinary(
        &self,
        _program: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _binaryFormat: *mut GLenum,
        _binary: *mut std::os::raw::c_void,
    ) {
        self.entry_gl41
            .glGetProgramBinary(_program, _bufSize, _length, _binaryFormat, _binary);
    }
    pub unsafe fn glProgramUniform2ui(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLuint,
        _v1: GLuint,
    ) {
        self.entry_gl41
            .glProgramUniform2ui(_program, _location, _v0, _v1);
    }
    pub unsafe fn glProgramUniform2uiv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLuint,
    ) {
        self.entry_gl41
            .glProgramUniform2uiv(_program, _location, _count, _value);
    }
    pub unsafe fn glProgramUniformMatrix2x4fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        self.entry_gl41
            .glProgramUniformMatrix2x4fv(_program, _location, _count, _transpose, _value);
    }
    pub unsafe fn glScissorIndexedv(&self, _index: GLuint, _v: *const GLint) {
        self.entry_gl41.glScissorIndexedv(_index, _v);
    }
    pub unsafe fn glProgramUniform3iv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLint,
    ) {
        self.entry_gl41
            .glProgramUniform3iv(_program, _location, _count, _value);
    }
    pub unsafe fn glVertexAttribL1d(&self, _index: GLuint, _x: GLdouble) {
        self.entry_gl41.glVertexAttribL1d(_index, _x);
    }
    pub unsafe fn glVertexAttribL2d(&self, _index: GLuint, _x: GLdouble, _y: GLdouble) {
        self.entry_gl41.glVertexAttribL2d(_index, _x, _y);
    }
    pub unsafe fn glVertexAttribL4d(
        &self,
        _index: GLuint,
        _x: GLdouble,
        _y: GLdouble,
        _z: GLdouble,
        _w: GLdouble,
    ) {
        self.entry_gl41.glVertexAttribL4d(_index, _x, _y, _z, _w);
    }
    pub unsafe fn glDeleteProgramPipelines(&self, _n: GLsizei, _pipelines: *const GLuint) {
        self.entry_gl41.glDeleteProgramPipelines(_n, _pipelines);
    }
    pub unsafe fn glVertexAttribLPointer(
        &self,
        _index: GLuint,
        _size: GLint,
        _type: VertexAttribLType,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        self.entry_gl41
            .glVertexAttribLPointer(_index, _size, _type, _stride, _pointer);
    }
    pub unsafe fn glProgramUniformMatrix3x2dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        self.entry_gl41
            .glProgramUniformMatrix3x2dv(_program, _location, _count, _transpose, _value);
    }
    pub unsafe fn glProgramUniformMatrix3fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        self.entry_gl41
            .glProgramUniformMatrix3fv(_program, _location, _count, _transpose, _value);
    }
    pub unsafe fn glProgramUniform3f(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLfloat,
        _v1: GLfloat,
        _v2: GLfloat,
    ) {
        self.entry_gl41
            .glProgramUniform3f(_program, _location, _v0, _v1, _v2);
    }
    pub unsafe fn glProgramUniform3uiv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLuint,
    ) {
        self.entry_gl41
            .glProgramUniform3uiv(_program, _location, _count, _value);
    }
    pub unsafe fn glProgramUniform1fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLfloat,
    ) {
        self.entry_gl41
            .glProgramUniform1fv(_program, _location, _count, _value);
    }
    pub unsafe fn glScissorIndexed(
        &self,
        _index: GLuint,
        _left: GLint,
        _bottom: GLint,
        _width: GLsizei,
        _height: GLsizei,
    ) {
        self.entry_gl41
            .glScissorIndexed(_index, _left, _bottom, _width, _height);
    }
    pub unsafe fn glViewportArrayv(&self, _first: GLuint, _count: GLsizei, _v: *const GLfloat) {
        self.entry_gl41.glViewportArrayv(_first, _count, _v);
    }
    pub unsafe fn glDepthRangeArrayv(&self, _first: GLuint, _count: GLsizei, _v: *const GLdouble) {
        self.entry_gl41.glDepthRangeArrayv(_first, _count, _v);
    }
    pub unsafe fn glGetProgramPipelineiv(
        &self,
        _pipeline: GLuint,
        _pname: PipelineParameterName,
        _params: *mut GLint,
    ) {
        self.entry_gl41
            .glGetProgramPipelineiv(_pipeline, _pname, _params);
    }
    pub unsafe fn glProgramUniformMatrix2dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        self.entry_gl41
            .glProgramUniformMatrix2dv(_program, _location, _count, _transpose, _value);
    }
    pub unsafe fn glProgramParameteri(
        &self,
        _program: GLuint,
        _pname: ProgramParameterPName,
        _value: GLint,
    ) {
        self.entry_gl41
            .glProgramParameteri(_program, _pname, _value);
    }
    pub unsafe fn glProgramUniform4fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLfloat,
    ) {
        self.entry_gl41
            .glProgramUniform4fv(_program, _location, _count, _value);
    }
    pub unsafe fn glProgramUniform1ui(&self, _program: GLuint, _location: GLint, _v0: GLuint) {
        self.entry_gl41
            .glProgramUniform1ui(_program, _location, _v0);
    }
    pub unsafe fn glProgramUniformMatrix3dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        self.entry_gl41
            .glProgramUniformMatrix3dv(_program, _location, _count, _transpose, _value);
    }
    pub unsafe fn glProgramUniformMatrix3x4fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        self.entry_gl41
            .glProgramUniformMatrix3x4fv(_program, _location, _count, _transpose, _value);
    }
    pub unsafe fn glProgramUniformMatrix4x3fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        self.entry_gl41
            .glProgramUniformMatrix4x3fv(_program, _location, _count, _transpose, _value);
    }
    pub unsafe fn glDepthRangef(&self, _n: GLfloat, _f: GLfloat) {
        self.entry_gl41.glDepthRangef(_n, _f);
    }
    pub unsafe fn glProgramUniform4i(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLint,
        _v1: GLint,
        _v2: GLint,
        _v3: GLint,
    ) {
        self.entry_gl41
            .glProgramUniform4i(_program, _location, _v0, _v1, _v2, _v3);
    }
    pub unsafe fn glGetShaderPrecisionFormat(
        &self,
        _shadertype: ShaderType,
        _precisiontype: PrecisionType,
        _range: *mut GLint,
        _precision: *mut GLint,
    ) {
        self.entry_gl41
            .glGetShaderPrecisionFormat(_shadertype, _precisiontype, _range, _precision);
    }
    pub unsafe fn glScissorArrayv(&self, _first: GLuint, _count: GLsizei, _v: *const GLint) {
        self.entry_gl41.glScissorArrayv(_first, _count, _v);
    }
    pub unsafe fn glProgramUniformMatrix4fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        self.entry_gl41
            .glProgramUniformMatrix4fv(_program, _location, _count, _transpose, _value);
    }
    pub unsafe fn glProgramUniform4uiv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLuint,
    ) {
        self.entry_gl41
            .glProgramUniform4uiv(_program, _location, _count, _value);
    }
    pub unsafe fn glGenProgramPipelines(&self, _n: GLsizei, _pipelines: *mut GLuint) {
        self.entry_gl41.glGenProgramPipelines(_n, _pipelines);
    }
    pub unsafe fn glProgramUniform1iv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLint,
    ) {
        self.entry_gl41
            .glProgramUniform1iv(_program, _location, _count, _value);
    }
    pub unsafe fn glProgramUniform4dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLdouble,
    ) {
        self.entry_gl41
            .glProgramUniform4dv(_program, _location, _count, _value);
    }
    pub unsafe fn glViewportIndexedfv(&self, _index: GLuint, _v: *const GLfloat) {
        self.entry_gl41.glViewportIndexedfv(_index, _v);
    }
    pub unsafe fn glProgramUniformMatrix3x2fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        self.entry_gl41
            .glProgramUniformMatrix3x2fv(_program, _location, _count, _transpose, _value);
    }
    pub unsafe fn glShaderBinary(
        &self,
        _count: GLsizei,
        _shaders: *const GLuint,
        _binaryFormat: ShaderBinaryFormat,
        _binary: *const std::os::raw::c_void,
        _length: GLsizei,
    ) {
        self.entry_gl41
            .glShaderBinary(_count, _shaders, _binaryFormat, _binary, _length);
    }
    pub unsafe fn glGetFloati_v(&self, _target: GetPName, _index: GLuint, _data: *mut GLfloat) {
        self.entry_gl41.glGetFloati_v(_target, _index, _data);
    }
    pub unsafe fn glIsProgramPipeline(&self, _pipeline: GLuint) -> GLboolean {
        self.entry_gl41.glIsProgramPipeline(_pipeline)
    }
    pub unsafe fn glProgramUniform1dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLdouble,
    ) {
        self.entry_gl41
            .glProgramUniform1dv(_program, _location, _count, _value);
    }
    pub unsafe fn glCreateShaderProgramv(
        &self,
        _type: ShaderType,
        _count: GLsizei,
        _strings: *const *const GLchar,
    ) -> GLuint {
        self.entry_gl41
            .glCreateShaderProgramv(_type, _count, _strings)
    }
    pub unsafe fn glProgramUniform1f(&self, _program: GLuint, _location: GLint, _v0: GLfloat) {
        self.entry_gl41.glProgramUniform1f(_program, _location, _v0);
    }
    pub unsafe fn glProgramUniformMatrix4x3dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        self.entry_gl41
            .glProgramUniformMatrix4x3dv(_program, _location, _count, _transpose, _value);
    }
    pub unsafe fn glUseProgramStages(
        &self,
        _pipeline: GLuint,
        _stages: UseProgramStageMask,
        _program: GLuint,
    ) {
        self.entry_gl41
            .glUseProgramStages(_pipeline, _stages, _program);
    }
    pub unsafe fn glProgramUniform2f(
        &self,
        _program: GLuint,
        _location: GLint,
        _v0: GLfloat,
        _v1: GLfloat,
    ) {
        self.entry_gl41
            .glProgramUniform2f(_program, _location, _v0, _v1);
    }
    pub unsafe fn glProgramBinary(
        &self,
        _program: GLuint,
        _binaryFormat: GLenum,
        _binary: *const std::os::raw::c_void,
        _length: GLsizei,
    ) {
        self.entry_gl41
            .glProgramBinary(_program, _binaryFormat, _binary, _length);
    }
    pub unsafe fn glProgramUniformMatrix2fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        self.entry_gl41
            .glProgramUniformMatrix2fv(_program, _location, _count, _transpose, _value);
    }
    pub unsafe fn glProgramUniformMatrix2x3fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        self.entry_gl41
            .glProgramUniformMatrix2x3fv(_program, _location, _count, _transpose, _value);
    }
    pub unsafe fn glProgramUniformMatrix2x4dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        self.entry_gl41
            .glProgramUniformMatrix2x4dv(_program, _location, _count, _transpose, _value);
    }
    pub unsafe fn glProgramUniformMatrix4x2dv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        self.entry_gl41
            .glProgramUniformMatrix4x2dv(_program, _location, _count, _transpose, _value);
    }
    pub unsafe fn glDepthRangeIndexed(&self, _index: GLuint, _n: GLdouble, _f: GLdouble) {
        self.entry_gl41.glDepthRangeIndexed(_index, _n, _f);
    }
    pub unsafe fn glProgramUniform1d(&self, _program: GLuint, _location: GLint, _v0: GLdouble) {
        self.entry_gl41.glProgramUniform1d(_program, _location, _v0);
    }
    pub unsafe fn glActiveShaderProgram(&self, _pipeline: GLuint, _program: GLuint) {
        self.entry_gl41.glActiveShaderProgram(_pipeline, _program);
    }
    pub unsafe fn glProgramUniform2fv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLfloat,
    ) {
        self.entry_gl41
            .glProgramUniform2fv(_program, _location, _count, _value);
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
        self.entry_gl41
            .glProgramUniform4ui(_program, _location, _v0, _v1, _v2, _v3);
    }
    pub unsafe fn glProgramUniform4iv(
        &self,
        _program: GLuint,
        _location: GLint,
        _count: GLsizei,
        _value: *const GLint,
    ) {
        self.entry_gl41
            .glProgramUniform4iv(_program, _location, _count, _value);
    }
    pub unsafe fn glClearColor(
        &self,
        _red: GLfloat,
        _green: GLfloat,
        _blue: GLfloat,
        _alpha: GLfloat,
    ) {
        self.entry_gl10.glClearColor(_red, _green, _blue, _alpha);
    }
    pub unsafe fn glVertex4i(&self, _x: GLint, _y: GLint, _z: GLint, _w: GLint) {
        self.entry_gl10.glVertex4i(_x, _y, _z, _w);
    }
    pub unsafe fn glDrawBuffer(&self, _buf: DrawBufferMode) {
        self.entry_gl10.glDrawBuffer(_buf);
    }
    pub unsafe fn glTexCoord3fv(&self, _v: *const GLfloat) {
        self.entry_gl10.glTexCoord3fv(_v);
    }
    pub unsafe fn glEvalPoint1(&self, _i: GLint) {
        self.entry_gl10.glEvalPoint1(_i);
    }
    pub unsafe fn glGetLightfv(
        &self,
        _light: LightName,
        _pname: LightParameter,
        _params: *mut GLfloat,
    ) {
        self.entry_gl10.glGetLightfv(_light, _pname, _params);
    }
    pub unsafe fn glColor3fv(&self, _v: *const GLfloat) {
        self.entry_gl10.glColor3fv(_v);
    }
    pub unsafe fn glIndexi(&self, _c: GLint) {
        self.entry_gl10.glIndexi(_c);
    }
    pub unsafe fn glRasterPos2iv(&self, _v: *const GLint) {
        self.entry_gl10.glRasterPos2iv(_v);
    }
    pub unsafe fn glRasterPos4s(&self, _x: GLshort, _y: GLshort, _z: GLshort, _w: GLshort) {
        self.entry_gl10.glRasterPos4s(_x, _y, _z, _w);
    }
    pub unsafe fn glVertex3f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        self.entry_gl10.glVertex3f(_x, _y, _z);
    }
    pub unsafe fn glSelectBuffer(&self, _size: GLsizei, _buffer: *mut GLuint) {
        self.entry_gl10.glSelectBuffer(_size, _buffer);
    }
    pub unsafe fn glColor3ui(&self, _red: GLuint, _green: GLuint, _blue: GLuint) {
        self.entry_gl10.glColor3ui(_red, _green, _blue);
    }
    pub unsafe fn glMultMatrixd(&self, _m: *const GLdouble) {
        self.entry_gl10.glMultMatrixd(_m);
    }
    pub unsafe fn glEvalCoord1d(&self, _u: GLdouble) {
        self.entry_gl10.glEvalCoord1d(_u);
    }
    pub unsafe fn glFogiv(&self, _pname: FogParameter, _params: *const GLint) {
        self.entry_gl10.glFogiv(_pname, _params);
    }
    pub unsafe fn glColorMaterial(&self, _face: MaterialFace, _mode: ColorMaterialParameter) {
        self.entry_gl10.glColorMaterial(_face, _mode);
    }
    pub unsafe fn glLoadMatrixd(&self, _m: *const GLdouble) {
        self.entry_gl10.glLoadMatrixd(_m);
    }
    pub unsafe fn glIndexd(&self, _c: GLdouble) {
        self.entry_gl10.glIndexd(_c);
    }
    pub unsafe fn glRasterPos2i(&self, _x: GLint, _y: GLint) {
        self.entry_gl10.glRasterPos2i(_x, _y);
    }
    pub unsafe fn glTexCoord3f(&self, _s: GLfloat, _t: GLfloat, _r: GLfloat) {
        self.entry_gl10.glTexCoord3f(_s, _t, _r);
    }
    pub unsafe fn glTexCoord3iv(&self, _v: *const GLint) {
        self.entry_gl10.glTexCoord3iv(_v);
    }
    pub unsafe fn glTexGend(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _param: GLdouble,
    ) {
        self.entry_gl10.glTexGend(_coord, _pname, _param);
    }
    pub unsafe fn glIndexiv(&self, _c: *const GLint) {
        self.entry_gl10.glIndexiv(_c);
    }
    pub unsafe fn glGetPolygonStipple(&self, _mask: *mut GLubyte) {
        self.entry_gl10.glGetPolygonStipple(_mask);
    }
    pub unsafe fn glColor4us(
        &self,
        _red: GLushort,
        _green: GLushort,
        _blue: GLushort,
        _alpha: GLushort,
    ) {
        self.entry_gl10.glColor4us(_red, _green, _blue, _alpha);
    }
    pub unsafe fn glVertex4s(&self, _x: GLshort, _y: GLshort, _z: GLshort, _w: GLshort) {
        self.entry_gl10.glVertex4s(_x, _y, _z, _w);
    }
    pub unsafe fn glPushName(&self, _name: GLuint) {
        self.entry_gl10.glPushName(_name);
    }
    pub unsafe fn glColor3ub(&self, _red: GLubyte, _green: GLubyte, _blue: GLubyte) {
        self.entry_gl10.glColor3ub(_red, _green, _blue);
    }
    pub unsafe fn glColor4ui(&self, _red: GLuint, _green: GLuint, _blue: GLuint, _alpha: GLuint) {
        self.entry_gl10.glColor4ui(_red, _green, _blue, _alpha);
    }
    pub unsafe fn glRectsv(&self, _v1: *const GLshort, _v2: *const GLshort) {
        self.entry_gl10.glRectsv(_v1, _v2);
    }
    pub unsafe fn glRasterPos4i(&self, _x: GLint, _y: GLint, _z: GLint, _w: GLint) {
        self.entry_gl10.glRasterPos4i(_x, _y, _z, _w);
    }
    pub unsafe fn glVertex3s(&self, _x: GLshort, _y: GLshort, _z: GLshort) {
        self.entry_gl10.glVertex3s(_x, _y, _z);
    }
    pub unsafe fn glPixelZoom(&self, _xfactor: GLfloat, _yfactor: GLfloat) {
        self.entry_gl10.glPixelZoom(_xfactor, _yfactor);
    }
    pub unsafe fn glFrustum(
        &self,
        _left: GLdouble,
        _right: GLdouble,
        _bottom: GLdouble,
        _top: GLdouble,
        _zNear: GLdouble,
        _zFar: GLdouble,
    ) {
        self.entry_gl10
            .glFrustum(_left, _right, _bottom, _top, _zNear, _zFar);
    }
    pub unsafe fn glColor3uiv(&self, _v: *const GLuint) {
        self.entry_gl10.glColor3uiv(_v);
    }
    pub unsafe fn glEvalCoord2fv(&self, _u: *const GLfloat) {
        self.entry_gl10.glEvalCoord2fv(_u);
    }
    pub unsafe fn glCopyPixels(
        &self,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _type: PixelCopyType,
    ) {
        self.entry_gl10.glCopyPixels(_x, _y, _width, _height, _type);
    }
    pub unsafe fn glMapGrid1d(&self, _un: GLint, _u1: GLdouble, _u2: GLdouble) {
        self.entry_gl10.glMapGrid1d(_un, _u1, _u2);
    }
    pub unsafe fn glStencilMask(&self, _mask: GLuint) {
        self.entry_gl10.glStencilMask(_mask);
    }
    pub unsafe fn glGetTexLevelParameteriv(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _pname: GetTextureParameter,
        _params: *mut GLint,
    ) {
        self.entry_gl10
            .glGetTexLevelParameteriv(_target, _level, _pname, _params);
    }
    pub unsafe fn glRectdv(&self, _v1: *const GLdouble, _v2: *const GLdouble) {
        self.entry_gl10.glRectdv(_v1, _v2);
    }
    pub unsafe fn glMap1f(
        &self,
        _target: MapTarget,
        _u1: GLfloat,
        _u2: GLfloat,
        _stride: GLint,
        _order: GLint,
        _points: *const GLfloat,
    ) {
        self.entry_gl10
            .glMap1f(_target, _u1, _u2, _stride, _order, _points);
    }
    pub unsafe fn glTexCoord3dv(&self, _v: *const GLdouble) {
        self.entry_gl10.glTexCoord3dv(_v);
    }
    pub unsafe fn glColor4iv(&self, _v: *const GLint) {
        self.entry_gl10.glColor4iv(_v);
    }
    pub unsafe fn glRects(&self, _x1: GLshort, _y1: GLshort, _x2: GLshort, _y2: GLshort) {
        self.entry_gl10.glRects(_x1, _y1, _x2, _y2);
    }
    pub unsafe fn glPopName(&self) {
        self.entry_gl10.glPopName();
    }
    pub unsafe fn glTexCoord4s(&self, _s: GLshort, _t: GLshort, _r: GLshort, _q: GLshort) {
        self.entry_gl10.glTexCoord4s(_s, _t, _r, _q);
    }
    pub unsafe fn glMap2f(
        &self,
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
    ) {
        self.entry_gl10.glMap2f(
            _target, _u1, _u2, _ustride, _uorder, _v1, _v2, _vstride, _vorder, _points,
        );
    }
    pub unsafe fn glPixelStoref(&self, _pname: PixelStoreParameter, _param: GLfloat) {
        self.entry_gl10.glPixelStoref(_pname, _param);
    }
    pub unsafe fn glGetLightiv(
        &self,
        _light: LightName,
        _pname: LightParameter,
        _params: *mut GLint,
    ) {
        self.entry_gl10.glGetLightiv(_light, _pname, _params);
    }
    pub unsafe fn glBlendFunc(&self, _sfactor: BlendingFactor, _dfactor: BlendingFactor) {
        self.entry_gl10.glBlendFunc(_sfactor, _dfactor);
    }
    pub unsafe fn glRasterPos2dv(&self, _v: *const GLdouble) {
        self.entry_gl10.glRasterPos2dv(_v);
    }
    pub unsafe fn glTexEnvf(
        &self,
        _target: TextureEnvTarget,
        _pname: TextureEnvParameter,
        _param: GLfloat,
    ) {
        self.entry_gl10.glTexEnvf(_target, _pname, _param);
    }
    pub unsafe fn glGetTexLevelParameterfv(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _pname: GetTextureParameter,
        _params: *mut GLfloat,
    ) {
        self.entry_gl10
            .glGetTexLevelParameterfv(_target, _level, _pname, _params);
    }
    pub unsafe fn glPushMatrix(&self) {
        self.entry_gl10.glPushMatrix();
    }
    pub unsafe fn glGenLists(&self, _range: GLsizei) -> GLuint {
        self.entry_gl10.glGenLists(_range)
    }
    pub unsafe fn glColor4f(
        &self,
        _red: GLfloat,
        _green: GLfloat,
        _blue: GLfloat,
        _alpha: GLfloat,
    ) {
        self.entry_gl10.glColor4f(_red, _green, _blue, _alpha);
    }
    pub unsafe fn glOrtho(
        &self,
        _left: GLdouble,
        _right: GLdouble,
        _bottom: GLdouble,
        _top: GLdouble,
        _zNear: GLdouble,
        _zFar: GLdouble,
    ) {
        self.entry_gl10
            .glOrtho(_left, _right, _bottom, _top, _zNear, _zFar);
    }
    pub unsafe fn glLightModelf(&self, _pname: LightModelParameter, _param: GLfloat) {
        self.entry_gl10.glLightModelf(_pname, _param);
    }
    pub unsafe fn glPolygonMode(&self, _face: MaterialFace, _mode: PolygonMode) {
        self.entry_gl10.glPolygonMode(_face, _mode);
    }
    pub unsafe fn glPopAttrib(&self) {
        self.entry_gl10.glPopAttrib();
    }
    pub unsafe fn glColor3b(&self, _red: GLbyte, _green: GLbyte, _blue: GLbyte) {
        self.entry_gl10.glColor3b(_red, _green, _blue);
    }
    pub unsafe fn glFogf(&self, _pname: FogParameter, _param: GLfloat) {
        self.entry_gl10.glFogf(_pname, _param);
    }
    pub unsafe fn glBegin(&self, _mode: PrimitiveType) {
        self.entry_gl10.glBegin(_mode);
    }
    pub unsafe fn glPolygonStipple(&self, _mask: *const GLubyte) {
        self.entry_gl10.glPolygonStipple(_mask);
    }
    pub unsafe fn glGetTexGenfv(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _params: *mut GLfloat,
    ) {
        self.entry_gl10.glGetTexGenfv(_coord, _pname, _params);
    }
    pub unsafe fn glTranslatef(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        self.entry_gl10.glTranslatef(_x, _y, _z);
    }
    pub unsafe fn glMaterialiv(
        &self,
        _face: MaterialFace,
        _pname: MaterialParameter,
        _params: *const GLint,
    ) {
        self.entry_gl10.glMaterialiv(_face, _pname, _params);
    }
    pub unsafe fn glPixelTransferf(&self, _pname: PixelTransferParameter, _param: GLfloat) {
        self.entry_gl10.glPixelTransferf(_pname, _param);
    }
    pub unsafe fn glGetTexParameterfv(
        &self,
        _target: TextureTarget,
        _pname: GetTextureParameter,
        _params: *mut GLfloat,
    ) {
        self.entry_gl10
            .glGetTexParameterfv(_target, _pname, _params);
    }
    pub unsafe fn glLightModelfv(&self, _pname: LightModelParameter, _params: *const GLfloat) {
        self.entry_gl10.glLightModelfv(_pname, _params);
    }
    pub unsafe fn glGetTexGendv(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _params: *mut GLdouble,
    ) {
        self.entry_gl10.glGetTexGendv(_coord, _pname, _params);
    }
    pub unsafe fn glGetMapdv(&self, _target: MapTarget, _query: GetMapQuery, _v: *mut GLdouble) {
        self.entry_gl10.glGetMapdv(_target, _query, _v);
    }
    pub unsafe fn glTexEnvi(
        &self,
        _target: TextureEnvTarget,
        _pname: TextureEnvParameter,
        _param: GLint,
    ) {
        self.entry_gl10.glTexEnvi(_target, _pname, _param);
    }
    pub unsafe fn glRasterPos4dv(&self, _v: *const GLdouble) {
        self.entry_gl10.glRasterPos4dv(_v);
    }
    pub unsafe fn glRectiv(&self, _v1: *const GLint, _v2: *const GLint) {
        self.entry_gl10.glRectiv(_v1, _v2);
    }
    pub unsafe fn glFrontFace(&self, _mode: FrontFaceDirection) {
        self.entry_gl10.glFrontFace(_mode);
    }
    pub unsafe fn glTexEnvfv(
        &self,
        _target: TextureEnvTarget,
        _pname: TextureEnvParameter,
        _params: *const GLfloat,
    ) {
        self.entry_gl10.glTexEnvfv(_target, _pname, _params);
    }
    pub unsafe fn glTexParameteriv(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _params: *const GLint,
    ) {
        self.entry_gl10.glTexParameteriv(_target, _pname, _params);
    }
    pub unsafe fn glColor3f(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat) {
        self.entry_gl10.glColor3f(_red, _green, _blue);
    }
    pub unsafe fn glVertex2f(&self, _x: GLfloat, _y: GLfloat) {
        self.entry_gl10.glVertex2f(_x, _y);
    }
    pub unsafe fn glLineStipple(&self, _factor: GLint, _pattern: GLushort) {
        self.entry_gl10.glLineStipple(_factor, _pattern);
    }
    pub unsafe fn glReadBuffer(&self, _src: ReadBufferMode) {
        self.entry_gl10.glReadBuffer(_src);
    }
    pub unsafe fn glDepthMask(&self, _flag: GLboolean) {
        self.entry_gl10.glDepthMask(_flag);
    }
    pub unsafe fn glColorMask(
        &self,
        _red: GLboolean,
        _green: GLboolean,
        _blue: GLboolean,
        _alpha: GLboolean,
    ) {
        self.entry_gl10.glColorMask(_red, _green, _blue, _alpha);
    }
    pub unsafe fn glIndexs(&self, _c: GLshort) {
        self.entry_gl10.glIndexs(_c);
    }
    pub unsafe fn glPixelMapuiv(&self, _map: PixelMap, _mapsize: GLsizei, _values: *const GLuint) {
        self.entry_gl10.glPixelMapuiv(_map, _mapsize, _values);
    }
    pub unsafe fn glRotated(&self, _angle: GLdouble, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        self.entry_gl10.glRotated(_angle, _x, _y, _z);
    }
    pub unsafe fn glGetPixelMapfv(&self, _map: PixelMap, _values: *mut GLfloat) {
        self.entry_gl10.glGetPixelMapfv(_map, _values);
    }
    pub unsafe fn glRasterPos3s(&self, _x: GLshort, _y: GLshort, _z: GLshort) {
        self.entry_gl10.glRasterPos3s(_x, _y, _z);
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
        self.entry_gl10
            .glReadPixels(_x, _y, _width, _height, _format, _type, _pixels);
    }
    pub unsafe fn glNormal3bv(&self, _v: *const GLbyte) {
        self.entry_gl10.glNormal3bv(_v);
    }
    pub unsafe fn glClearDepth(&self, _depth: GLdouble) {
        self.entry_gl10.glClearDepth(_depth);
    }
    pub unsafe fn glTexGeniv(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _params: *const GLint,
    ) {
        self.entry_gl10.glTexGeniv(_coord, _pname, _params);
    }
    pub unsafe fn glColor4uiv(&self, _v: *const GLuint) {
        self.entry_gl10.glColor4uiv(_v);
    }
    pub unsafe fn glLoadName(&self, _name: GLuint) {
        self.entry_gl10.glLoadName(_name);
    }
    pub unsafe fn glRasterPos2f(&self, _x: GLfloat, _y: GLfloat) {
        self.entry_gl10.glRasterPos2f(_x, _y);
    }
    pub unsafe fn glVertex2sv(&self, _v: *const GLshort) {
        self.entry_gl10.glVertex2sv(_v);
    }
    pub unsafe fn glEdgeFlag(&self, _flag: GLboolean) {
        self.entry_gl10.glEdgeFlag(_flag);
    }
    pub unsafe fn glTexCoord3s(&self, _s: GLshort, _t: GLshort, _r: GLshort) {
        self.entry_gl10.glTexCoord3s(_s, _t, _r);
    }
    pub unsafe fn glVertex3sv(&self, _v: *const GLshort) {
        self.entry_gl10.glVertex3sv(_v);
    }
    pub unsafe fn glClearAccum(
        &self,
        _red: GLfloat,
        _green: GLfloat,
        _blue: GLfloat,
        _alpha: GLfloat,
    ) {
        self.entry_gl10.glClearAccum(_red, _green, _blue, _alpha);
    }
    pub unsafe fn glIsList(&self, _list: GLuint) -> GLboolean {
        self.entry_gl10.glIsList(_list)
    }
    pub unsafe fn glRasterPos2d(&self, _x: GLdouble, _y: GLdouble) {
        self.entry_gl10.glRasterPos2d(_x, _y);
    }
    pub unsafe fn glTexCoord3d(&self, _s: GLdouble, _t: GLdouble, _r: GLdouble) {
        self.entry_gl10.glTexCoord3d(_s, _t, _r);
    }
    pub unsafe fn glIndexf(&self, _c: GLfloat) {
        self.entry_gl10.glIndexf(_c);
    }
    pub unsafe fn glStencilOp(&self, _fail: StencilOp, _zfail: StencilOp, _zpass: StencilOp) {
        self.entry_gl10.glStencilOp(_fail, _zfail, _zpass);
    }
    pub unsafe fn glVertex3d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        self.entry_gl10.glVertex3d(_x, _y, _z);
    }
    pub unsafe fn glInitNames(&self) {
        self.entry_gl10.glInitNames();
    }
    pub unsafe fn glDisable(&self, _cap: EnableCap) {
        self.entry_gl10.glDisable(_cap);
    }
    pub unsafe fn glColor3dv(&self, _v: *const GLdouble) {
        self.entry_gl10.glColor3dv(_v);
    }
    pub unsafe fn glLineWidth(&self, _width: GLfloat) {
        self.entry_gl10.glLineWidth(_width);
    }
    pub unsafe fn glEvalCoord1dv(&self, _u: *const GLdouble) {
        self.entry_gl10.glEvalCoord1dv(_u);
    }
    pub unsafe fn glPointSize(&self, _size: GLfloat) {
        self.entry_gl10.glPointSize(_size);
    }
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
        self.entry_gl10.glTexImage1D(
            _target,
            _level,
            _internalformat,
            _width,
            _border,
            _format,
            _type,
            _pixels,
        );
    }
    pub unsafe fn glNormal3f(&self, _nx: GLfloat, _ny: GLfloat, _nz: GLfloat) {
        self.entry_gl10.glNormal3f(_nx, _ny, _nz);
    }
    pub unsafe fn glColor4usv(&self, _v: *const GLushort) {
        self.entry_gl10.glColor4usv(_v);
    }
    pub unsafe fn glTexCoord2f(&self, _s: GLfloat, _t: GLfloat) {
        self.entry_gl10.glTexCoord2f(_s, _t);
    }
    pub unsafe fn glRasterPos4d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble, _w: GLdouble) {
        self.entry_gl10.glRasterPos4d(_x, _y, _z, _w);
    }
    pub unsafe fn glFogfv(&self, _pname: FogParameter, _params: *const GLfloat) {
        self.entry_gl10.glFogfv(_pname, _params);
    }
    pub unsafe fn glRenderMode(&self, _mode: RenderingMode) -> GLint {
        self.entry_gl10.glRenderMode(_mode)
    }
    pub unsafe fn glNormal3s(&self, _nx: GLshort, _ny: GLshort, _nz: GLshort) {
        self.entry_gl10.glNormal3s(_nx, _ny, _nz);
    }
    pub unsafe fn glNormal3i(&self, _nx: GLint, _ny: GLint, _nz: GLint) {
        self.entry_gl10.glNormal3i(_nx, _ny, _nz);
    }
    pub unsafe fn glVertex4f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat, _w: GLfloat) {
        self.entry_gl10.glVertex4f(_x, _y, _z, _w);
    }
    pub unsafe fn glLightModeliv(&self, _pname: LightModelParameter, _params: *const GLint) {
        self.entry_gl10.glLightModeliv(_pname, _params);
    }
    pub unsafe fn glGetError(&self) -> GLenum {
        self.entry_gl10.glGetError()
    }
    pub unsafe fn glTexCoord1sv(&self, _v: *const GLshort) {
        self.entry_gl10.glTexCoord1sv(_v);
    }
    pub unsafe fn glTexCoord3sv(&self, _v: *const GLshort) {
        self.entry_gl10.glTexCoord3sv(_v);
    }
    pub unsafe fn glColor3us(&self, _red: GLushort, _green: GLushort, _blue: GLushort) {
        self.entry_gl10.glColor3us(_red, _green, _blue);
    }
    pub unsafe fn glTexCoord1dv(&self, _v: *const GLdouble) {
        self.entry_gl10.glTexCoord1dv(_v);
    }
    pub unsafe fn glRectd(&self, _x1: GLdouble, _y1: GLdouble, _x2: GLdouble, _y2: GLdouble) {
        self.entry_gl10.glRectd(_x1, _y1, _x2, _y2);
    }
    pub unsafe fn glLightiv(
        &self,
        _light: LightName,
        _pname: LightParameter,
        _params: *const GLint,
    ) {
        self.entry_gl10.glLightiv(_light, _pname, _params);
    }
    pub unsafe fn glRasterPos2fv(&self, _v: *const GLfloat) {
        self.entry_gl10.glRasterPos2fv(_v);
    }
    pub unsafe fn glTexCoord1s(&self, _s: GLshort) {
        self.entry_gl10.glTexCoord1s(_s);
    }
    pub unsafe fn glNormal3d(&self, _nx: GLdouble, _ny: GLdouble, _nz: GLdouble) {
        self.entry_gl10.glNormal3d(_nx, _ny, _nz);
    }
    pub unsafe fn glPixelStorei(&self, _pname: PixelStoreParameter, _param: GLint) {
        self.entry_gl10.glPixelStorei(_pname, _param);
    }
    pub unsafe fn glDepthRange(&self, _n: GLdouble, _f: GLdouble) {
        self.entry_gl10.glDepthRange(_n, _f);
    }
    pub unsafe fn glEvalCoord2d(&self, _u: GLdouble, _v: GLdouble) {
        self.entry_gl10.glEvalCoord2d(_u, _v);
    }
    pub unsafe fn glGetPixelMapuiv(&self, _map: PixelMap, _values: *mut GLuint) {
        self.entry_gl10.glGetPixelMapuiv(_map, _values);
    }
    pub unsafe fn glVertex2iv(&self, _v: *const GLint) {
        self.entry_gl10.glVertex2iv(_v);
    }
    pub unsafe fn glVertex4sv(&self, _v: *const GLshort) {
        self.entry_gl10.glVertex4sv(_v);
    }
    pub unsafe fn glPixelMapusv(
        &self,
        _map: PixelMap,
        _mapsize: GLsizei,
        _values: *const GLushort,
    ) {
        self.entry_gl10.glPixelMapusv(_map, _mapsize, _values);
    }
    pub unsafe fn glPopMatrix(&self) {
        self.entry_gl10.glPopMatrix();
    }
    pub unsafe fn glDrawPixels(
        &self,
        _width: GLsizei,
        _height: GLsizei,
        _format: PixelFormat,
        _type: PixelType,
        _pixels: *const std::os::raw::c_void,
    ) {
        self.entry_gl10
            .glDrawPixels(_width, _height, _format, _type, _pixels);
    }
    pub unsafe fn glGetBooleanv(&self, _pname: GetPName, _data: *mut GLboolean) {
        self.entry_gl10.glGetBooleanv(_pname, _data);
    }
    pub unsafe fn glCallLists(
        &self,
        _n: GLsizei,
        _type: ListNameType,
        _lists: *const std::os::raw::c_void,
    ) {
        self.entry_gl10.glCallLists(_n, _type, _lists);
    }
    pub unsafe fn glVertex2dv(&self, _v: *const GLdouble) {
        self.entry_gl10.glVertex2dv(_v);
    }
    pub unsafe fn glLightModeli(&self, _pname: LightModelParameter, _param: GLint) {
        self.entry_gl10.glLightModeli(_pname, _param);
    }
    pub unsafe fn glTexEnviv(
        &self,
        _target: TextureEnvTarget,
        _pname: TextureEnvParameter,
        _params: *const GLint,
    ) {
        self.entry_gl10.glTexEnviv(_target, _pname, _params);
    }
    pub unsafe fn glVertex2s(&self, _x: GLshort, _y: GLshort) {
        self.entry_gl10.glVertex2s(_x, _y);
    }
    pub unsafe fn glClipPlane(&self, _plane: ClipPlaneName, _equation: *const GLdouble) {
        self.entry_gl10.glClipPlane(_plane, _equation);
    }
    pub unsafe fn glVertex2fv(&self, _v: *const GLfloat) {
        self.entry_gl10.glVertex2fv(_v);
    }
    pub unsafe fn glGetString(&self, _name: StringName) -> *const GLubyte {
        self.entry_gl10.glGetString(_name)
    }
    pub unsafe fn glTexGenfv(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _params: *const GLfloat,
    ) {
        self.entry_gl10.glTexGenfv(_coord, _pname, _params);
    }
    pub unsafe fn glEvalMesh1(&self, _mode: MeshMode1, _i1: GLint, _i2: GLint) {
        self.entry_gl10.glEvalMesh1(_mode, _i1, _i2);
    }
    pub unsafe fn glEvalMesh2(
        &self,
        _mode: MeshMode2,
        _i1: GLint,
        _i2: GLint,
        _j1: GLint,
        _j2: GLint,
    ) {
        self.entry_gl10.glEvalMesh2(_mode, _i1, _i2, _j1, _j2);
    }
    pub unsafe fn glViewport(&self, _x: GLint, _y: GLint, _width: GLsizei, _height: GLsizei) {
        self.entry_gl10.glViewport(_x, _y, _width, _height);
    }
    pub unsafe fn glVertex4iv(&self, _v: *const GLint) {
        self.entry_gl10.glVertex4iv(_v);
    }
    pub unsafe fn glGetMaterialfv(
        &self,
        _face: MaterialFace,
        _pname: MaterialParameter,
        _params: *mut GLfloat,
    ) {
        self.entry_gl10.glGetMaterialfv(_face, _pname, _params);
    }
    pub unsafe fn glDepthFunc(&self, _func: DepthFunction) {
        self.entry_gl10.glDepthFunc(_func);
    }
    pub unsafe fn glMapGrid2f(
        &self,
        _un: GLint,
        _u1: GLfloat,
        _u2: GLfloat,
        _vn: GLint,
        _v1: GLfloat,
        _v2: GLfloat,
    ) {
        self.entry_gl10.glMapGrid2f(_un, _u1, _u2, _vn, _v1, _v2);
    }
    pub unsafe fn glPixelTransferi(&self, _pname: PixelTransferParameter, _param: GLint) {
        self.entry_gl10.glPixelTransferi(_pname, _param);
    }
    pub unsafe fn glColor3sv(&self, _v: *const GLshort) {
        self.entry_gl10.glColor3sv(_v);
    }
    pub unsafe fn glVertex2i(&self, _x: GLint, _y: GLint) {
        self.entry_gl10.glVertex2i(_x, _y);
    }
    pub unsafe fn glRasterPos4iv(&self, _v: *const GLint) {
        self.entry_gl10.glRasterPos4iv(_v);
    }
    pub unsafe fn glGetTexEnvfv(
        &self,
        _target: TextureEnvTarget,
        _pname: TextureEnvParameter,
        _params: *mut GLfloat,
    ) {
        self.entry_gl10.glGetTexEnvfv(_target, _pname, _params);
    }
    pub unsafe fn glColor4sv(&self, _v: *const GLshort) {
        self.entry_gl10.glColor4sv(_v);
    }
    pub unsafe fn glClear(&self, _mask: ClearBufferMask) {
        self.entry_gl10.glClear(_mask);
    }
    pub unsafe fn glColor4ubv(&self, _v: *const GLubyte) {
        self.entry_gl10.glColor4ubv(_v);
    }
    pub unsafe fn glNormal3b(&self, _nx: GLbyte, _ny: GLbyte, _nz: GLbyte) {
        self.entry_gl10.glNormal3b(_nx, _ny, _nz);
    }
    pub unsafe fn glPixelMapfv(&self, _map: PixelMap, _mapsize: GLsizei, _values: *const GLfloat) {
        self.entry_gl10.glPixelMapfv(_map, _mapsize, _values);
    }
    pub unsafe fn glEndList(&self) {
        self.entry_gl10.glEndList();
    }
    pub unsafe fn glColor4fv(&self, _v: *const GLfloat) {
        self.entry_gl10.glColor4fv(_v);
    }
    pub unsafe fn glGetClipPlane(&self, _plane: ClipPlaneName, _equation: *mut GLdouble) {
        self.entry_gl10.glGetClipPlane(_plane, _equation);
    }
    pub unsafe fn glColor3s(&self, _red: GLshort, _green: GLshort, _blue: GLshort) {
        self.entry_gl10.glColor3s(_red, _green, _blue);
    }
    pub unsafe fn glTexParameterfv(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _params: *const GLfloat,
    ) {
        self.entry_gl10.glTexParameterfv(_target, _pname, _params);
    }
    pub unsafe fn glScissor(&self, _x: GLint, _y: GLint, _width: GLsizei, _height: GLsizei) {
        self.entry_gl10.glScissor(_x, _y, _width, _height);
    }
    pub unsafe fn glColor3bv(&self, _v: *const GLbyte) {
        self.entry_gl10.glColor3bv(_v);
    }
    pub unsafe fn glTexCoord2d(&self, _s: GLdouble, _t: GLdouble) {
        self.entry_gl10.glTexCoord2d(_s, _t);
    }
    pub unsafe fn glNormal3sv(&self, _v: *const GLshort) {
        self.entry_gl10.glNormal3sv(_v);
    }
    pub unsafe fn glGetMapiv(&self, _target: MapTarget, _query: GetMapQuery, _v: *mut GLint) {
        self.entry_gl10.glGetMapiv(_target, _query, _v);
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
        self.entry_gl10.glTexImage2D(
            _target,
            _level,
            _internalformat,
            _width,
            _height,
            _border,
            _format,
            _type,
            _pixels,
        );
    }
    pub unsafe fn glClearStencil(&self, _s: GLint) {
        self.entry_gl10.glClearStencil(_s);
    }
    pub unsafe fn glTexCoord4f(&self, _s: GLfloat, _t: GLfloat, _r: GLfloat, _q: GLfloat) {
        self.entry_gl10.glTexCoord4f(_s, _t, _r, _q);
    }
    pub unsafe fn glVertex3fv(&self, _v: *const GLfloat) {
        self.entry_gl10.glVertex3fv(_v);
    }
    pub unsafe fn glEnable(&self, _cap: EnableCap) {
        self.entry_gl10.glEnable(_cap);
    }
    pub unsafe fn glCallList(&self, _list: GLuint) {
        self.entry_gl10.glCallList(_list);
    }
    pub unsafe fn glTexCoord2fv(&self, _v: *const GLfloat) {
        self.entry_gl10.glTexCoord2fv(_v);
    }
    pub unsafe fn glEvalPoint2(&self, _i: GLint, _j: GLint) {
        self.entry_gl10.glEvalPoint2(_i, _j);
    }
    pub unsafe fn glRasterPos3f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        self.entry_gl10.glRasterPos3f(_x, _y, _z);
    }
    pub unsafe fn glTexCoord1i(&self, _s: GLint) {
        self.entry_gl10.glTexCoord1i(_s);
    }
    pub unsafe fn glLighti(&self, _light: LightName, _pname: LightParameter, _param: GLint) {
        self.entry_gl10.glLighti(_light, _pname, _param);
    }
    pub unsafe fn glColor4dv(&self, _v: *const GLdouble) {
        self.entry_gl10.glColor4dv(_v);
    }
    pub unsafe fn glNormal3dv(&self, _v: *const GLdouble) {
        self.entry_gl10.glNormal3dv(_v);
    }
    pub unsafe fn glVertex3iv(&self, _v: *const GLint) {
        self.entry_gl10.glVertex3iv(_v);
    }
    pub unsafe fn glPassThrough(&self, _token: GLfloat) {
        self.entry_gl10.glPassThrough(_token);
    }
    pub unsafe fn glTexCoord2sv(&self, _v: *const GLshort) {
        self.entry_gl10.glTexCoord2sv(_v);
    }
    pub unsafe fn glGetTexGeniv(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _params: *mut GLint,
    ) {
        self.entry_gl10.glGetTexGeniv(_coord, _pname, _params);
    }
    pub unsafe fn glColor4bv(&self, _v: *const GLbyte) {
        self.entry_gl10.glColor4bv(_v);
    }
    pub unsafe fn glRasterPos4fv(&self, _v: *const GLfloat) {
        self.entry_gl10.glRasterPos4fv(_v);
    }
    pub unsafe fn glColor3usv(&self, _v: *const GLushort) {
        self.entry_gl10.glColor3usv(_v);
    }
    pub unsafe fn glIndexfv(&self, _c: *const GLfloat) {
        self.entry_gl10.glIndexfv(_c);
    }
    pub unsafe fn glIndexsv(&self, _c: *const GLshort) {
        self.entry_gl10.glIndexsv(_c);
    }
    pub unsafe fn glIndexMask(&self, _mask: GLuint) {
        self.entry_gl10.glIndexMask(_mask);
    }
    pub unsafe fn glRasterPos2s(&self, _x: GLshort, _y: GLshort) {
        self.entry_gl10.glRasterPos2s(_x, _y);
    }
    pub unsafe fn glVertex4dv(&self, _v: *const GLdouble) {
        self.entry_gl10.glVertex4dv(_v);
    }
    pub unsafe fn glFinish(&self) {
        self.entry_gl10.glFinish();
    }
    pub unsafe fn glTexGeni(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _param: GLint,
    ) {
        self.entry_gl10.glTexGeni(_coord, _pname, _param);
    }
    pub unsafe fn glColor3d(&self, _red: GLdouble, _green: GLdouble, _blue: GLdouble) {
        self.entry_gl10.glColor3d(_red, _green, _blue);
    }
    pub unsafe fn glTexCoord1fv(&self, _v: *const GLfloat) {
        self.entry_gl10.glTexCoord1fv(_v);
    }
    pub unsafe fn glTexCoord4fv(&self, _v: *const GLfloat) {
        self.entry_gl10.glTexCoord4fv(_v);
    }
    pub unsafe fn glTexParameteri(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _param: GLint,
    ) {
        self.entry_gl10.glTexParameteri(_target, _pname, _param);
    }
    pub unsafe fn glGetIntegerv(&self, _pname: GetPName, _data: *mut GLint) {
        self.entry_gl10.glGetIntegerv(_pname, _data);
    }
    pub unsafe fn glLogicOp(&self, _opcode: LogicOp) {
        self.entry_gl10.glLogicOp(_opcode);
    }
    pub unsafe fn glScalef(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        self.entry_gl10.glScalef(_x, _y, _z);
    }
    pub unsafe fn glTexCoord4dv(&self, _v: *const GLdouble) {
        self.entry_gl10.glTexCoord4dv(_v);
    }
    pub unsafe fn glStencilFunc(&self, _func: StencilFunction, _ref: GLint, _mask: GLuint) {
        self.entry_gl10.glStencilFunc(_func, _ref, _mask);
    }
    pub unsafe fn glRasterPos3fv(&self, _v: *const GLfloat) {
        self.entry_gl10.glRasterPos3fv(_v);
    }
    pub unsafe fn glTexCoord1d(&self, _s: GLdouble) {
        self.entry_gl10.glTexCoord1d(_s);
    }
    pub unsafe fn glEvalCoord1fv(&self, _u: *const GLfloat) {
        self.entry_gl10.glEvalCoord1fv(_u);
    }
    pub unsafe fn glRecti(&self, _x1: GLint, _y1: GLint, _x2: GLint, _y2: GLint) {
        self.entry_gl10.glRecti(_x1, _y1, _x2, _y2);
    }
    pub unsafe fn glEvalCoord1f(&self, _u: GLfloat) {
        self.entry_gl10.glEvalCoord1f(_u);
    }
    pub unsafe fn glColor4d(
        &self,
        _red: GLdouble,
        _green: GLdouble,
        _blue: GLdouble,
        _alpha: GLdouble,
    ) {
        self.entry_gl10.glColor4d(_red, _green, _blue, _alpha);
    }
    pub unsafe fn glVertex4fv(&self, _v: *const GLfloat) {
        self.entry_gl10.glVertex4fv(_v);
    }
    pub unsafe fn glEvalCoord2dv(&self, _u: *const GLdouble) {
        self.entry_gl10.glEvalCoord2dv(_u);
    }
    pub unsafe fn glTexParameterf(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _param: GLfloat,
    ) {
        self.entry_gl10.glTexParameterf(_target, _pname, _param);
    }
    pub unsafe fn glCullFace(&self, _mode: CullFaceMode) {
        self.entry_gl10.glCullFace(_mode);
    }
    pub unsafe fn glGetTexParameteriv(
        &self,
        _target: TextureTarget,
        _pname: GetTextureParameter,
        _params: *mut GLint,
    ) {
        self.entry_gl10
            .glGetTexParameteriv(_target, _pname, _params);
    }
    pub unsafe fn glTexCoord3i(&self, _s: GLint, _t: GLint, _r: GLint) {
        self.entry_gl10.glTexCoord3i(_s, _t, _r);
    }
    pub unsafe fn glRasterPos3d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        self.entry_gl10.glRasterPos3d(_x, _y, _z);
    }
    pub unsafe fn glGetPixelMapusv(&self, _map: PixelMap, _values: *mut GLushort) {
        self.entry_gl10.glGetPixelMapusv(_map, _values);
    }
    pub unsafe fn glMatrixMode(&self, _mode: MatrixMode) {
        self.entry_gl10.glMatrixMode(_mode);
    }
    pub unsafe fn glRasterPos4f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat, _w: GLfloat) {
        self.entry_gl10.glRasterPos4f(_x, _y, _z, _w);
    }
    pub unsafe fn glTexCoord1iv(&self, _v: *const GLint) {
        self.entry_gl10.glTexCoord1iv(_v);
    }
    pub unsafe fn glLoadMatrixf(&self, _m: *const GLfloat) {
        self.entry_gl10.glLoadMatrixf(_m);
    }
    pub unsafe fn glClearIndex(&self, _c: GLfloat) {
        self.entry_gl10.glClearIndex(_c);
    }
    pub unsafe fn glMapGrid1f(&self, _un: GLint, _u1: GLfloat, _u2: GLfloat) {
        self.entry_gl10.glMapGrid1f(_un, _u1, _u2);
    }
    pub unsafe fn glLoadIdentity(&self) {
        self.entry_gl10.glLoadIdentity();
    }
    pub unsafe fn glMultMatrixf(&self, _m: *const GLfloat) {
        self.entry_gl10.glMultMatrixf(_m);
    }
    pub unsafe fn glEnd(&self) {
        self.entry_gl10.glEnd();
    }
    pub unsafe fn glColor4ub(
        &self,
        _red: GLubyte,
        _green: GLubyte,
        _blue: GLubyte,
        _alpha: GLubyte,
    ) {
        self.entry_gl10.glColor4ub(_red, _green, _blue, _alpha);
    }
    pub unsafe fn glIndexdv(&self, _c: *const GLdouble) {
        self.entry_gl10.glIndexdv(_c);
    }
    pub unsafe fn glMateriali(
        &self,
        _face: MaterialFace,
        _pname: MaterialParameter,
        _param: GLint,
    ) {
        self.entry_gl10.glMateriali(_face, _pname, _param);
    }
    pub unsafe fn glColor4b(&self, _red: GLbyte, _green: GLbyte, _blue: GLbyte, _alpha: GLbyte) {
        self.entry_gl10.glColor4b(_red, _green, _blue, _alpha);
    }
    pub unsafe fn glGetMaterialiv(
        &self,
        _face: MaterialFace,
        _pname: MaterialParameter,
        _params: *mut GLint,
    ) {
        self.entry_gl10.glGetMaterialiv(_face, _pname, _params);
    }
    pub unsafe fn glTexCoord2iv(&self, _v: *const GLint) {
        self.entry_gl10.glTexCoord2iv(_v);
    }
    pub unsafe fn glMap1d(
        &self,
        _target: MapTarget,
        _u1: GLdouble,
        _u2: GLdouble,
        _stride: GLint,
        _order: GLint,
        _points: *const GLdouble,
    ) {
        self.entry_gl10
            .glMap1d(_target, _u1, _u2, _stride, _order, _points);
    }
    pub unsafe fn glGetFloatv(&self, _pname: GetPName, _data: *mut GLfloat) {
        self.entry_gl10.glGetFloatv(_pname, _data);
    }
    pub unsafe fn glGetTexImage(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _format: PixelFormat,
        _type: PixelType,
        _pixels: *mut std::os::raw::c_void,
    ) {
        self.entry_gl10
            .glGetTexImage(_target, _level, _format, _type, _pixels);
    }
    pub unsafe fn glColor3iv(&self, _v: *const GLint) {
        self.entry_gl10.glColor3iv(_v);
    }
    pub unsafe fn glColor3ubv(&self, _v: *const GLubyte) {
        self.entry_gl10.glColor3ubv(_v);
    }
    pub unsafe fn glPushAttrib(&self, _mask: AttribMask) {
        self.entry_gl10.glPushAttrib(_mask);
    }
    pub unsafe fn glRectfv(&self, _v1: *const GLfloat, _v2: *const GLfloat) {
        self.entry_gl10.glRectfv(_v1, _v2);
    }
    pub unsafe fn glGetMapfv(&self, _target: MapTarget, _query: GetMapQuery, _v: *mut GLfloat) {
        self.entry_gl10.glGetMapfv(_target, _query, _v);
    }
    pub unsafe fn glShadeModel(&self, _mode: ShadingModel) {
        self.entry_gl10.glShadeModel(_mode);
    }
    pub unsafe fn glIsEnabled(&self, _cap: EnableCap) -> GLboolean {
        self.entry_gl10.glIsEnabled(_cap)
    }
    pub unsafe fn glColor3i(&self, _red: GLint, _green: GLint, _blue: GLint) {
        self.entry_gl10.glColor3i(_red, _green, _blue);
    }
    pub unsafe fn glTexCoord2i(&self, _s: GLint, _t: GLint) {
        self.entry_gl10.glTexCoord2i(_s, _t);
    }
    pub unsafe fn glEvalCoord2f(&self, _u: GLfloat, _v: GLfloat) {
        self.entry_gl10.glEvalCoord2f(_u, _v);
    }
    pub unsafe fn glRotatef(&self, _angle: GLfloat, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        self.entry_gl10.glRotatef(_angle, _x, _y, _z);
    }
    pub unsafe fn glTranslated(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        self.entry_gl10.glTranslated(_x, _y, _z);
    }
    pub unsafe fn glNormal3fv(&self, _v: *const GLfloat) {
        self.entry_gl10.glNormal3fv(_v);
    }
    pub unsafe fn glNewList(&self, _list: GLuint, _mode: ListMode) {
        self.entry_gl10.glNewList(_list, _mode);
    }
    pub unsafe fn glFlush(&self) {
        self.entry_gl10.glFlush();
    }
    pub unsafe fn glDeleteLists(&self, _list: GLuint, _range: GLsizei) {
        self.entry_gl10.glDeleteLists(_list, _range);
    }
    pub unsafe fn glRasterPos3iv(&self, _v: *const GLint) {
        self.entry_gl10.glRasterPos3iv(_v);
    }
    pub unsafe fn glEdgeFlagv(&self, _flag: *const GLboolean) {
        self.entry_gl10.glEdgeFlagv(_flag);
    }
    pub unsafe fn glRectf(&self, _x1: GLfloat, _y1: GLfloat, _x2: GLfloat, _y2: GLfloat) {
        self.entry_gl10.glRectf(_x1, _y1, _x2, _y2);
    }
    pub unsafe fn glTexCoord4i(&self, _s: GLint, _t: GLint, _r: GLint, _q: GLint) {
        self.entry_gl10.glTexCoord4i(_s, _t, _r, _q);
    }
    pub unsafe fn glTexCoord4iv(&self, _v: *const GLint) {
        self.entry_gl10.glTexCoord4iv(_v);
    }
    pub unsafe fn glColor4s(
        &self,
        _red: GLshort,
        _green: GLshort,
        _blue: GLshort,
        _alpha: GLshort,
    ) {
        self.entry_gl10.glColor4s(_red, _green, _blue, _alpha);
    }
    pub unsafe fn glTexCoord2dv(&self, _v: *const GLdouble) {
        self.entry_gl10.glTexCoord2dv(_v);
    }
    pub unsafe fn glMap2d(
        &self,
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
    ) {
        self.entry_gl10.glMap2d(
            _target, _u1, _u2, _ustride, _uorder, _v1, _v2, _vstride, _vorder, _points,
        );
    }
    pub unsafe fn glGetTexEnviv(
        &self,
        _target: TextureEnvTarget,
        _pname: TextureEnvParameter,
        _params: *mut GLint,
    ) {
        self.entry_gl10.glGetTexEnviv(_target, _pname, _params);
    }
    pub unsafe fn glRasterPos3sv(&self, _v: *const GLshort) {
        self.entry_gl10.glRasterPos3sv(_v);
    }
    pub unsafe fn glLightf(&self, _light: LightName, _pname: LightParameter, _param: GLfloat) {
        self.entry_gl10.glLightf(_light, _pname, _param);
    }
    pub unsafe fn glColor4i(&self, _red: GLint, _green: GLint, _blue: GLint, _alpha: GLint) {
        self.entry_gl10.glColor4i(_red, _green, _blue, _alpha);
    }
    pub unsafe fn glHint(&self, _target: HintTarget, _mode: HintMode) {
        self.entry_gl10.glHint(_target, _mode);
    }
    pub unsafe fn glFeedbackBuffer(
        &self,
        _size: GLsizei,
        _type: FeedbackType,
        _buffer: *mut GLfloat,
    ) {
        self.entry_gl10.glFeedbackBuffer(_size, _type, _buffer);
    }
    pub unsafe fn glTexGendv(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _params: *const GLdouble,
    ) {
        self.entry_gl10.glTexGendv(_coord, _pname, _params);
    }
    pub unsafe fn glVertex4d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble, _w: GLdouble) {
        self.entry_gl10.glVertex4d(_x, _y, _z, _w);
    }
    pub unsafe fn glVertex3i(&self, _x: GLint, _y: GLint, _z: GLint) {
        self.entry_gl10.glVertex3i(_x, _y, _z);
    }
    pub unsafe fn glLightfv(
        &self,
        _light: LightName,
        _pname: LightParameter,
        _params: *const GLfloat,
    ) {
        self.entry_gl10.glLightfv(_light, _pname, _params);
    }
    pub unsafe fn glMaterialfv(
        &self,
        _face: MaterialFace,
        _pname: MaterialParameter,
        _params: *const GLfloat,
    ) {
        self.entry_gl10.glMaterialfv(_face, _pname, _params);
    }
    pub unsafe fn glAccum(&self, _op: AccumOp, _value: GLfloat) {
        self.entry_gl10.glAccum(_op, _value);
    }
    pub unsafe fn glMapGrid2d(
        &self,
        _un: GLint,
        _u1: GLdouble,
        _u2: GLdouble,
        _vn: GLint,
        _v1: GLdouble,
        _v2: GLdouble,
    ) {
        self.entry_gl10.glMapGrid2d(_un, _u1, _u2, _vn, _v1, _v2);
    }
    pub unsafe fn glTexGenf(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _param: GLfloat,
    ) {
        self.entry_gl10.glTexGenf(_coord, _pname, _param);
    }
    pub unsafe fn glTexCoord4d(&self, _s: GLdouble, _t: GLdouble, _r: GLdouble, _q: GLdouble) {
        self.entry_gl10.glTexCoord4d(_s, _t, _r, _q);
    }
    pub unsafe fn glGetDoublev(&self, _pname: GetPName, _data: *mut GLdouble) {
        self.entry_gl10.glGetDoublev(_pname, _data);
    }
    pub unsafe fn glBitmap(
        &self,
        _width: GLsizei,
        _height: GLsizei,
        _xorig: GLfloat,
        _yorig: GLfloat,
        _xmove: GLfloat,
        _ymove: GLfloat,
        _bitmap: *const GLubyte,
    ) {
        self.entry_gl10
            .glBitmap(_width, _height, _xorig, _yorig, _xmove, _ymove, _bitmap);
    }
    pub unsafe fn glTexCoord4sv(&self, _v: *const GLshort) {
        self.entry_gl10.glTexCoord4sv(_v);
    }
    pub unsafe fn glNormal3iv(&self, _v: *const GLint) {
        self.entry_gl10.glNormal3iv(_v);
    }
    pub unsafe fn glRasterPos3i(&self, _x: GLint, _y: GLint, _z: GLint) {
        self.entry_gl10.glRasterPos3i(_x, _y, _z);
    }
    pub unsafe fn glVertex2d(&self, _x: GLdouble, _y: GLdouble) {
        self.entry_gl10.glVertex2d(_x, _y);
    }
    pub unsafe fn glScaled(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        self.entry_gl10.glScaled(_x, _y, _z);
    }
    pub unsafe fn glRasterPos2sv(&self, _v: *const GLshort) {
        self.entry_gl10.glRasterPos2sv(_v);
    }
    pub unsafe fn glFogi(&self, _pname: FogParameter, _param: GLint) {
        self.entry_gl10.glFogi(_pname, _param);
    }
    pub unsafe fn glRasterPos3dv(&self, _v: *const GLdouble) {
        self.entry_gl10.glRasterPos3dv(_v);
    }
    pub unsafe fn glListBase(&self, _base: GLuint) {
        self.entry_gl10.glListBase(_base);
    }
    pub unsafe fn glAlphaFunc(&self, _func: AlphaFunction, _ref: GLfloat) {
        self.entry_gl10.glAlphaFunc(_func, _ref);
    }
    pub unsafe fn glVertex3dv(&self, _v: *const GLdouble) {
        self.entry_gl10.glVertex3dv(_v);
    }
    pub unsafe fn glRasterPos4sv(&self, _v: *const GLshort) {
        self.entry_gl10.glRasterPos4sv(_v);
    }
    pub unsafe fn glTexCoord1f(&self, _s: GLfloat) {
        self.entry_gl10.glTexCoord1f(_s);
    }
    pub unsafe fn glMaterialf(
        &self,
        _face: MaterialFace,
        _pname: MaterialParameter,
        _param: GLfloat,
    ) {
        self.entry_gl10.glMaterialf(_face, _pname, _param);
    }
    pub unsafe fn glTexCoord2s(&self, _s: GLshort, _t: GLshort) {
        self.entry_gl10.glTexCoord2s(_s, _t);
    }
    pub unsafe fn glNormalPointer(
        &self,
        _type: NormalPointerType,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        self.entry_gl11.glNormalPointer(_type, _stride, _pointer);
    }
    pub unsafe fn glIndexubv(&self, _c: *const GLubyte) {
        self.entry_gl11.glIndexubv(_c);
    }
    pub unsafe fn glVertexPointer(
        &self,
        _size: GLint,
        _type: VertexPointerType,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        self.entry_gl11
            .glVertexPointer(_size, _type, _stride, _pointer);
    }
    pub unsafe fn glBindTexture(&self, _target: TextureTarget, _texture: GLuint) {
        self.entry_gl11.glBindTexture(_target, _texture);
    }
    pub unsafe fn glDrawElements(
        &self,
        _mode: PrimitiveType,
        _count: GLsizei,
        _type: DrawElementsType,
        _indices: *const std::os::raw::c_void,
    ) {
        self.entry_gl11
            .glDrawElements(_mode, _count, _type, _indices);
    }
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
        self.entry_gl11.glCopyTexImage2D(
            _target,
            _level,
            _internalformat,
            _x,
            _y,
            _width,
            _height,
            _border,
        );
    }
    pub unsafe fn glDrawArrays(&self, _mode: PrimitiveType, _first: GLint, _count: GLsizei) {
        self.entry_gl11.glDrawArrays(_mode, _first, _count);
    }
    pub unsafe fn glDisableClientState(&self, _array: EnableCap) {
        self.entry_gl11.glDisableClientState(_array);
    }
    pub unsafe fn glIsTexture(&self, _texture: GLuint) -> GLboolean {
        self.entry_gl11.glIsTexture(_texture)
    }
    pub unsafe fn glIndexPointer(
        &self,
        _type: IndexPointerType,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        self.entry_gl11.glIndexPointer(_type, _stride, _pointer);
    }
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
        self.entry_gl11
            .glCopyTexImage1D(_target, _level, _internalformat, _x, _y, _width, _border);
    }
    pub unsafe fn glPrioritizeTextures(
        &self,
        _n: GLsizei,
        _textures: *const GLuint,
        _priorities: *const GLfloat,
    ) {
        self.entry_gl11
            .glPrioritizeTextures(_n, _textures, _priorities);
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
        self.entry_gl11
            .glCopyTexSubImage1D(_target, _level, _xoffset, _x, _y, _width);
    }
    pub unsafe fn glPushClientAttrib(&self, _mask: ClientAttribMask) {
        self.entry_gl11.glPushClientAttrib(_mask);
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
        self.entry_gl11.glTexSubImage2D(
            _target, _level, _xoffset, _yoffset, _width, _height, _format, _type, _pixels,
        );
    }
    pub unsafe fn glDeleteTextures(&self, _n: GLsizei, _textures: *const GLuint) {
        self.entry_gl11.glDeleteTextures(_n, _textures);
    }
    pub unsafe fn glTexCoordPointer(
        &self,
        _size: GLint,
        _type: TexCoordPointerType,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        self.entry_gl11
            .glTexCoordPointer(_size, _type, _stride, _pointer);
    }
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
        self.entry_gl11
            .glCopyTexSubImage2D(_target, _level, _xoffset, _yoffset, _x, _y, _width, _height);
    }
    pub unsafe fn glGetPointerv(
        &self,
        _pname: GetPointervPName,
        _params: *mut *mut std::os::raw::c_void,
    ) {
        self.entry_gl11.glGetPointerv(_pname, _params);
    }
    pub unsafe fn glPopClientAttrib(&self) {
        self.entry_gl11.glPopClientAttrib();
    }
    pub unsafe fn glArrayElement(&self, _i: GLint) {
        self.entry_gl11.glArrayElement(_i);
    }
    pub unsafe fn glEdgeFlagPointer(
        &self,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        self.entry_gl11.glEdgeFlagPointer(_stride, _pointer);
    }
    pub unsafe fn glEnableClientState(&self, _array: EnableCap) {
        self.entry_gl11.glEnableClientState(_array);
    }
    pub unsafe fn glIndexub(&self, _c: GLubyte) {
        self.entry_gl11.glIndexub(_c);
    }
    pub unsafe fn glColorPointer(
        &self,
        _size: GLint,
        _type: ColorPointerType,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        self.entry_gl11
            .glColorPointer(_size, _type, _stride, _pointer);
    }
    pub unsafe fn glInterleavedArrays(
        &self,
        _format: InterleavedArrayFormat,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        self.entry_gl11
            .glInterleavedArrays(_format, _stride, _pointer);
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
        self.entry_gl11
            .glTexSubImage1D(_target, _level, _xoffset, _width, _format, _type, _pixels);
    }
    pub unsafe fn glGenTextures(&self, _n: GLsizei, _textures: *mut GLuint) {
        self.entry_gl11.glGenTextures(_n, _textures);
    }
    pub unsafe fn glAreTexturesResident(
        &self,
        _n: GLsizei,
        _textures: *const GLuint,
        _residences: *mut GLboolean,
    ) -> GLboolean {
        self.entry_gl11
            .glAreTexturesResident(_n, _textures, _residences)
    }
    pub unsafe fn glPolygonOffset(&self, _factor: GLfloat, _units: GLfloat) {
        self.entry_gl11.glPolygonOffset(_factor, _units);
    }
    pub unsafe fn glDrawTransformFeedbackInstanced(
        &self,
        _mode: PrimitiveType,
        _id: GLuint,
        _instancecount: GLsizei,
    ) {
        self.entry_gl42
            .glDrawTransformFeedbackInstanced(_mode, _id, _instancecount);
    }
    pub unsafe fn glDrawTransformFeedbackStreamInstanced(
        &self,
        _mode: PrimitiveType,
        _id: GLuint,
        _stream: GLuint,
        _instancecount: GLsizei,
    ) {
        self.entry_gl42
            .glDrawTransformFeedbackStreamInstanced(_mode, _id, _stream, _instancecount);
    }
    pub unsafe fn glMemoryBarrier(&self, _barriers: MemoryBarrierMask) {
        self.entry_gl42.glMemoryBarrier(_barriers);
    }
    pub unsafe fn glGetInternalformativ(
        &self,
        _target: TextureTarget,
        _internalformat: InternalFormat,
        _pname: InternalFormatPName,
        _count: GLsizei,
        _params: *mut GLint,
    ) {
        self.entry_gl42
            .glGetInternalformativ(_target, _internalformat, _pname, _count, _params);
    }
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
        self.entry_gl42
            .glBindImageTexture(_unit, _texture, _level, _layered, _layer, _access, _format);
    }
    pub unsafe fn glDrawArraysInstancedBaseInstance(
        &self,
        _mode: PrimitiveType,
        _first: GLint,
        _count: GLsizei,
        _instancecount: GLsizei,
        _baseinstance: GLuint,
    ) {
        self.entry_gl42.glDrawArraysInstancedBaseInstance(
            _mode,
            _first,
            _count,
            _instancecount,
            _baseinstance,
        );
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
        self.entry_gl42
            .glDrawElementsInstancedBaseVertexBaseInstance(
                _mode,
                _count,
                _type,
                _indices,
                _instancecount,
                _basevertex,
                _baseinstance,
            );
    }
    pub unsafe fn glGetActiveAtomicCounterBufferiv(
        &self,
        _program: GLuint,
        _bufferIndex: GLuint,
        _pname: AtomicCounterBufferPName,
        _params: *mut GLint,
    ) {
        self.entry_gl42
            .glGetActiveAtomicCounterBufferiv(_program, _bufferIndex, _pname, _params);
    }
    pub unsafe fn glDrawElementsInstancedBaseInstance(
        &self,
        _mode: PrimitiveType,
        _count: GLsizei,
        _type: PrimitiveType,
        _indices: *const std::os::raw::c_void,
        _instancecount: GLsizei,
        _baseinstance: GLuint,
    ) {
        self.entry_gl42.glDrawElementsInstancedBaseInstance(
            _mode,
            _count,
            _type,
            _indices,
            _instancecount,
            _baseinstance,
        );
    }
    pub unsafe fn glTexStorage1D(
        &self,
        _target: TextureTarget,
        _levels: GLsizei,
        _internalformat: SizedInternalFormat,
        _width: GLsizei,
    ) {
        self.entry_gl42
            .glTexStorage1D(_target, _levels, _internalformat, _width);
    }
    pub unsafe fn glTexStorage2D(
        &self,
        _target: TextureTarget,
        _levels: GLsizei,
        _internalformat: SizedInternalFormat,
        _width: GLsizei,
        _height: GLsizei,
    ) {
        self.entry_gl42
            .glTexStorage2D(_target, _levels, _internalformat, _width, _height);
    }
    pub unsafe fn glTexStorage3D(
        &self,
        _target: TextureTarget,
        _levels: GLsizei,
        _internalformat: SizedInternalFormat,
        _width: GLsizei,
        _height: GLsizei,
        _depth: GLsizei,
    ) {
        self.entry_gl42
            .glTexStorage3D(_target, _levels, _internalformat, _width, _height, _depth);
    }
    pub unsafe fn glBindTextures(&self, _first: GLuint, _count: GLsizei, _textures: *const GLuint) {
        self.entry_gl44.glBindTextures(_first, _count, _textures);
    }
    pub unsafe fn glClearTexImage(
        &self,
        _texture: GLuint,
        _level: GLint,
        _format: PixelFormat,
        _type: PixelType,
        _data: *const std::os::raw::c_void,
    ) {
        self.entry_gl44
            .glClearTexImage(_texture, _level, _format, _type, _data);
    }
    pub unsafe fn glBindImageTextures(
        &self,
        _first: GLuint,
        _count: GLsizei,
        _textures: *const GLuint,
    ) {
        self.entry_gl44
            .glBindImageTextures(_first, _count, _textures);
    }
    pub unsafe fn glBindBuffersBase(
        &self,
        _target: BufferTargetARB,
        _first: GLuint,
        _count: GLsizei,
        _buffers: *const GLuint,
    ) {
        self.entry_gl44
            .glBindBuffersBase(_target, _first, _count, _buffers);
    }
    pub unsafe fn glBindVertexBuffers(
        &self,
        _first: GLuint,
        _count: GLsizei,
        _buffers: *const GLuint,
        _offsets: *const GLintptr,
        _strides: *const GLsizei,
    ) {
        self.entry_gl44
            .glBindVertexBuffers(_first, _count, _buffers, _offsets, _strides);
    }
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
        self.entry_gl44.glClearTexSubImage(
            _texture, _level, _xoffset, _yoffset, _zoffset, _width, _height, _depth, _format,
            _type, _data,
        );
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
        self.entry_gl44
            .glBindBuffersRange(_target, _first, _count, _buffers, _offsets, _sizes);
    }
    pub unsafe fn glBufferStorage(
        &self,
        _target: BufferStorageTarget,
        _size: GLsizeiptr,
        _data: *const std::os::raw::c_void,
        _flags: BufferStorageMask,
    ) {
        self.entry_gl44
            .glBufferStorage(_target, _size, _data, _flags);
    }
    pub unsafe fn glBindSamplers(&self, _first: GLuint, _count: GLsizei, _samplers: *const GLuint) {
        self.entry_gl44.glBindSamplers(_first, _count, _samplers);
    }
    pub unsafe fn glUnmapBuffer(&self, _target: BufferTargetARB) -> GLboolean {
        self.entry_gl15.glUnmapBuffer(_target)
    }
    pub unsafe fn glBeginQuery(&self, _target: QueryTarget, _id: GLuint) {
        self.entry_gl15.glBeginQuery(_target, _id);
    }
    pub unsafe fn glIsQuery(&self, _id: GLuint) -> GLboolean {
        self.entry_gl15.glIsQuery(_id)
    }
    pub unsafe fn glEndQuery(&self, _target: QueryTarget) {
        self.entry_gl15.glEndQuery(_target);
    }
    pub unsafe fn glGetQueryiv(
        &self,
        _target: QueryTarget,
        _pname: QueryParameterName,
        _params: *mut GLint,
    ) {
        self.entry_gl15.glGetQueryiv(_target, _pname, _params);
    }
    pub unsafe fn glGetBufferPointerv(
        &self,
        _target: BufferTargetARB,
        _pname: BufferPointerNameARB,
        _params: *mut *mut std::os::raw::c_void,
    ) {
        self.entry_gl15
            .glGetBufferPointerv(_target, _pname, _params);
    }
    pub unsafe fn glDeleteQueries(&self, _n: GLsizei, _ids: *const GLuint) {
        self.entry_gl15.glDeleteQueries(_n, _ids);
    }
    pub unsafe fn glDeleteBuffers(&self, _n: GLsizei, _buffers: *const GLuint) {
        self.entry_gl15.glDeleteBuffers(_n, _buffers);
    }
    pub unsafe fn glGenBuffers(&self, _n: GLsizei, _buffers: *mut GLuint) {
        self.entry_gl15.glGenBuffers(_n, _buffers);
    }
    pub unsafe fn glBufferData(
        &self,
        _target: BufferTargetARB,
        _size: GLsizeiptr,
        _data: *const std::os::raw::c_void,
        _usage: BufferUsageARB,
    ) {
        self.entry_gl15.glBufferData(_target, _size, _data, _usage);
    }
    pub unsafe fn glGetBufferSubData(
        &self,
        _target: BufferTargetARB,
        _offset: GLintptr,
        _size: GLsizeiptr,
        _data: *mut std::os::raw::c_void,
    ) {
        self.entry_gl15
            .glGetBufferSubData(_target, _offset, _size, _data);
    }
    pub unsafe fn glGetBufferParameteriv(
        &self,
        _target: BufferTargetARB,
        _pname: BufferPNameARB,
        _params: *mut GLint,
    ) {
        self.entry_gl15
            .glGetBufferParameteriv(_target, _pname, _params);
    }
    pub unsafe fn glGenQueries(&self, _n: GLsizei, _ids: *mut GLuint) {
        self.entry_gl15.glGenQueries(_n, _ids);
    }
    pub unsafe fn glGetQueryObjectiv(
        &self,
        _id: GLuint,
        _pname: QueryObjectParameterName,
        _params: *mut GLint,
    ) {
        self.entry_gl15.glGetQueryObjectiv(_id, _pname, _params);
    }
    pub unsafe fn glGetQueryObjectuiv(
        &self,
        _id: GLuint,
        _pname: QueryObjectParameterName,
        _params: *mut GLuint,
    ) {
        self.entry_gl15.glGetQueryObjectuiv(_id, _pname, _params);
    }
    pub unsafe fn glBindBuffer(&self, _target: BufferTargetARB, _buffer: GLuint) {
        self.entry_gl15.glBindBuffer(_target, _buffer);
    }
    pub unsafe fn glBufferSubData(
        &self,
        _target: BufferTargetARB,
        _offset: GLintptr,
        _size: GLsizeiptr,
        _data: *const std::os::raw::c_void,
    ) {
        self.entry_gl15
            .glBufferSubData(_target, _offset, _size, _data);
    }
    pub unsafe fn glMapBuffer(
        &self,
        _target: BufferTargetARB,
        _access: BufferAccessARB,
    ) -> *mut std::os::raw::c_void {
        self.entry_gl15.glMapBuffer(_target, _access)
    }
    pub unsafe fn glIsBuffer(&self, _buffer: GLuint) -> GLboolean {
        self.entry_gl15.glIsBuffer(_buffer)
    }
    pub unsafe fn glInvalidateBufferSubData(
        &self,
        _buffer: GLuint,
        _offset: GLintptr,
        _length: GLsizeiptr,
    ) {
        self.entry_gl43
            .glInvalidateBufferSubData(_buffer, _offset, _length);
    }
    pub unsafe fn glVertexAttribBinding(&self, _attribindex: GLuint, _bindingindex: GLuint) {
        self.entry_gl43
            .glVertexAttribBinding(_attribindex, _bindingindex);
    }
    pub unsafe fn glVertexAttribLFormat(
        &self,
        _attribindex: GLuint,
        _size: GLint,
        _type: VertexAttribLType,
        _relativeoffset: GLuint,
    ) {
        self.entry_gl43
            .glVertexAttribLFormat(_attribindex, _size, _type, _relativeoffset);
    }
    pub unsafe fn glInvalidateBufferData(&self, _buffer: GLuint) {
        self.entry_gl43.glInvalidateBufferData(_buffer);
    }
    pub unsafe fn glObjectLabel(
        &self,
        _identifier: ObjectIdentifier,
        _name: GLuint,
        _length: GLsizei,
        _label: *const GLchar,
    ) {
        self.entry_gl43
            .glObjectLabel(_identifier, _name, _length, _label);
    }
    pub unsafe fn glTexBufferRange(
        &self,
        _target: TextureTarget,
        _internalformat: SizedInternalFormat,
        _buffer: GLuint,
        _offset: GLintptr,
        _size: GLsizeiptr,
    ) {
        self.entry_gl43
            .glTexBufferRange(_target, _internalformat, _buffer, _offset, _size);
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
        self.entry_gl43.glGetProgramResourceName(
            _program,
            _programInterface,
            _index,
            _bufSize,
            _length,
            _name,
        );
    }
    pub unsafe fn glGetProgramResourceIndex(
        &self,
        _program: GLuint,
        _programInterface: ProgramInterface,
        _name: *const GLchar,
    ) -> GLuint {
        self.entry_gl43
            .glGetProgramResourceIndex(_program, _programInterface, _name)
    }
    pub unsafe fn glGetProgramResourceLocationIndex(
        &self,
        _program: GLuint,
        _programInterface: ProgramInterface,
        _name: *const GLchar,
    ) -> GLint {
        self.entry_gl43
            .glGetProgramResourceLocationIndex(_program, _programInterface, _name)
    }
    pub unsafe fn glShaderStorageBlockBinding(
        &self,
        _program: GLuint,
        _storageBlockIndex: GLuint,
        _storageBlockBinding: GLuint,
    ) {
        self.entry_gl43.glShaderStorageBlockBinding(
            _program,
            _storageBlockIndex,
            _storageBlockBinding,
        );
    }
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
    ) -> GLuint {
        self.entry_gl43.glGetDebugMessageLog(
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
    pub unsafe fn glGetInternalformati64v(
        &self,
        _target: TextureTarget,
        _internalformat: InternalFormat,
        _pname: InternalFormatPName,
        _count: GLsizei,
        _params: *mut GLint64,
    ) {
        self.entry_gl43
            .glGetInternalformati64v(_target, _internalformat, _pname, _count, _params);
    }
    pub unsafe fn glInvalidateFramebuffer(
        &self,
        _target: FramebufferTarget,
        _numAttachments: GLsizei,
        _attachments: *const InvalidateFramebufferAttachment,
    ) {
        self.entry_gl43
            .glInvalidateFramebuffer(_target, _numAttachments, _attachments);
    }
    pub unsafe fn glPushDebugGroup(
        &self,
        _source: DebugSource,
        _id: GLuint,
        _length: GLsizei,
        _message: *const GLchar,
    ) {
        self.entry_gl43
            .glPushDebugGroup(_source, _id, _length, _message);
    }
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
        self.entry_gl43.glInvalidateSubFramebuffer(
            _target,
            _numAttachments,
            _attachments,
            _x,
            _y,
            _width,
            _height,
        );
    }
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
        self.entry_gl43.glInvalidateTexSubImage(
            _texture, _level, _xoffset, _yoffset, _zoffset, _width, _height, _depth,
        );
    }
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
        self.entry_gl43.glClearBufferSubData(
            _target,
            _internalformat,
            _offset,
            _size,
            _format,
            _type,
            _data,
        );
    }
    pub unsafe fn glDispatchComputeIndirect(&self, _indirect: GLintptr) {
        self.entry_gl43.glDispatchComputeIndirect(_indirect);
    }
    pub unsafe fn glGetFramebufferParameteriv(
        &self,
        _target: FramebufferTarget,
        _pname: FramebufferAttachmentParameterName,
        _params: *mut GLint,
    ) {
        self.entry_gl43
            .glGetFramebufferParameteriv(_target, _pname, _params);
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
        self.entry_gl43.glGetProgramResourceiv(
            _program,
            _programInterface,
            _index,
            _propCount,
            _props,
            _count,
            _length,
            _params,
        );
    }
    pub unsafe fn glVertexAttribIFormat(
        &self,
        _attribindex: GLuint,
        _size: GLint,
        _type: VertexAttribIType,
        _relativeoffset: GLuint,
    ) {
        self.entry_gl43
            .glVertexAttribIFormat(_attribindex, _size, _type, _relativeoffset);
    }
    pub unsafe fn glMultiDrawArraysIndirect(
        &self,
        _mode: PrimitiveType,
        _indirect: *const std::os::raw::c_void,
        _drawcount: GLsizei,
        _stride: GLsizei,
    ) {
        self.entry_gl43
            .glMultiDrawArraysIndirect(_mode, _indirect, _drawcount, _stride);
    }
    pub unsafe fn glGetProgramResourceLocation(
        &self,
        _program: GLuint,
        _programInterface: ProgramInterface,
        _name: *const GLchar,
    ) -> GLint {
        self.entry_gl43
            .glGetProgramResourceLocation(_program, _programInterface, _name)
    }
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
        self.entry_gl43.glTexStorage3DMultisample(
            _target,
            _samples,
            _internalformat,
            _width,
            _height,
            _depth,
            _fixedsamplelocations,
        );
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
        self.entry_gl43.glTextureView(
            _texture,
            _target,
            _origtexture,
            _internalformat,
            _minlevel,
            _numlevels,
            _minlayer,
            _numlayers,
        );
    }
    pub unsafe fn glDispatchCompute(
        &self,
        _num_groups_x: GLuint,
        _num_groups_y: GLuint,
        _num_groups_z: GLuint,
    ) {
        self.entry_gl43
            .glDispatchCompute(_num_groups_x, _num_groups_y, _num_groups_z);
    }
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
        self.entry_gl43.glCopyImageSubData(
            _srcName, _srcTarget, _srcLevel, _srcX, _srcY, _srcZ, _dstName, _dstTarget, _dstLevel,
            _dstX, _dstY, _dstZ, _srcWidth, _srcHeight, _srcDepth,
        );
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
        self.entry_gl43.glTexStorage2DMultisample(
            _target,
            _samples,
            _internalformat,
            _width,
            _height,
            _fixedsamplelocations,
        );
    }
    pub unsafe fn glVertexAttribFormat(
        &self,
        _attribindex: GLuint,
        _size: GLint,
        _type: VertexAttribType,
        _normalized: GLboolean,
        _relativeoffset: GLuint,
    ) {
        self.entry_gl43.glVertexAttribFormat(
            _attribindex,
            _size,
            _type,
            _normalized,
            _relativeoffset,
        );
    }
    pub unsafe fn glBindVertexBuffer(
        &self,
        _bindingindex: GLuint,
        _buffer: GLuint,
        _offset: GLintptr,
        _stride: GLsizei,
    ) {
        self.entry_gl43
            .glBindVertexBuffer(_bindingindex, _buffer, _offset, _stride);
    }
    pub unsafe fn glDebugMessageCallback(
        &self,
        _callback: GLDEBUGPROC,
        _userParam: *const std::os::raw::c_void,
    ) {
        self.entry_gl43
            .glDebugMessageCallback(_callback, _userParam);
    }
    pub unsafe fn glGetObjectLabel(
        &self,
        _identifier: ObjectIdentifier,
        _name: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _label: *mut GLchar,
    ) {
        self.entry_gl43
            .glGetObjectLabel(_identifier, _name, _bufSize, _length, _label);
    }
    pub unsafe fn glObjectPtrLabel(
        &self,
        _ptr: *const std::os::raw::c_void,
        _length: GLsizei,
        _label: *const GLchar,
    ) {
        self.entry_gl43.glObjectPtrLabel(_ptr, _length, _label);
    }
    pub unsafe fn glClearBufferData(
        &self,
        _target: BufferStorageTarget,
        _internalformat: SizedInternalFormat,
        _format: PixelFormat,
        _type: PixelType,
        _data: *const std::os::raw::c_void,
    ) {
        self.entry_gl43
            .glClearBufferData(_target, _internalformat, _format, _type, _data);
    }
    pub unsafe fn glVertexBindingDivisor(&self, _bindingindex: GLuint, _divisor: GLuint) {
        self.entry_gl43
            .glVertexBindingDivisor(_bindingindex, _divisor);
    }
    pub unsafe fn glPopDebugGroup(&self) {
        self.entry_gl43.glPopDebugGroup();
    }
    pub unsafe fn glInvalidateTexImage(&self, _texture: GLuint, _level: GLint) {
        self.entry_gl43.glInvalidateTexImage(_texture, _level);
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
        self.entry_gl43
            .glDebugMessageControl(_source, _type, _severity, _count, _ids, _enabled);
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
        self.entry_gl43
            .glDebugMessageInsert(_source, _type, _id, _severity, _length, _buf);
    }
    pub unsafe fn glGetObjectPtrLabel(
        &self,
        _ptr: *const std::os::raw::c_void,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _label: *mut GLchar,
    ) {
        self.entry_gl43
            .glGetObjectPtrLabel(_ptr, _bufSize, _length, _label);
    }
    pub unsafe fn glMultiDrawElementsIndirect(
        &self,
        _mode: PrimitiveType,
        _type: DrawElementsType,
        _indirect: *const std::os::raw::c_void,
        _drawcount: GLsizei,
        _stride: GLsizei,
    ) {
        self.entry_gl43
            .glMultiDrawElementsIndirect(_mode, _type, _indirect, _drawcount, _stride);
    }
    pub unsafe fn glGetProgramInterfaceiv(
        &self,
        _program: GLuint,
        _programInterface: ProgramInterface,
        _pname: ProgramInterfacePName,
        _params: *mut GLint,
    ) {
        self.entry_gl43
            .glGetProgramInterfaceiv(_program, _programInterface, _pname, _params);
    }
    pub unsafe fn glFramebufferParameteri(
        &self,
        _target: FramebufferTarget,
        _pname: FramebufferParameterName,
        _param: GLint,
    ) {
        self.entry_gl43
            .glFramebufferParameteri(_target, _pname, _param);
    }

    pub unsafe fn glSecondaryColor3uiv(&self, _v: *const GLuint) {
        self.entry_gl14.glSecondaryColor3uiv(_v);
    }
    pub unsafe fn glSecondaryColor3bv(&self, _v: *const GLbyte) {
        self.entry_gl14.glSecondaryColor3bv(_v);
    }
    pub unsafe fn glWindowPos2sv(&self, _v: *const GLshort) {
        self.entry_gl14.glWindowPos2sv(_v);
    }
    pub unsafe fn glWindowPos3iv(&self, _v: *const GLint) {
        self.entry_gl14.glWindowPos3iv(_v);
    }
    pub unsafe fn glWindowPos3d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        self.entry_gl14.glWindowPos3d(_x, _y, _z);
    }
    pub unsafe fn glWindowPos3f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        self.entry_gl14.glWindowPos3f(_x, _y, _z);
    }
    pub unsafe fn glFogCoordfv(&self, _coord: *const GLfloat) {
        self.entry_gl14.glFogCoordfv(_coord);
    }
    pub unsafe fn glWindowPos3s(&self, _x: GLshort, _y: GLshort, _z: GLshort) {
        self.entry_gl14.glWindowPos3s(_x, _y, _z);
    }
    pub unsafe fn glWindowPos3sv(&self, _v: *const GLshort) {
        self.entry_gl14.glWindowPos3sv(_v);
    }
    pub unsafe fn glBlendColor(
        &self,
        _red: GLfloat,
        _green: GLfloat,
        _blue: GLfloat,
        _alpha: GLfloat,
    ) {
        self.entry_gl14.glBlendColor(_red, _green, _blue, _alpha);
    }
    pub unsafe fn glSecondaryColor3s(&self, _red: GLshort, _green: GLshort, _blue: GLshort) {
        self.entry_gl14.glSecondaryColor3s(_red, _green, _blue);
    }
    pub unsafe fn glBlendFuncSeparate(
        &self,
        _sfactorRGB: BlendingFactor,
        _dfactorRGB: BlendingFactor,
        _sfactorAlpha: BlendingFactor,
        _dfactorAlpha: BlendingFactor,
    ) {
        self.entry_gl14
            .glBlendFuncSeparate(_sfactorRGB, _dfactorRGB, _sfactorAlpha, _dfactorAlpha);
    }
    pub unsafe fn glFogCoordPointer(
        &self,
        _type: FogPointerTypeEXT,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        self.entry_gl14.glFogCoordPointer(_type, _stride, _pointer);
    }
    pub unsafe fn glSecondaryColor3sv(&self, _v: *const GLshort) {
        self.entry_gl14.glSecondaryColor3sv(_v);
    }
    pub unsafe fn glWindowPos2iv(&self, _v: *const GLint) {
        self.entry_gl14.glWindowPos2iv(_v);
    }
    pub unsafe fn glSecondaryColor3dv(&self, _v: *const GLdouble) {
        self.entry_gl14.glSecondaryColor3dv(_v);
    }
    pub unsafe fn glWindowPos3i(&self, _x: GLint, _y: GLint, _z: GLint) {
        self.entry_gl14.glWindowPos3i(_x, _y, _z);
    }
    pub unsafe fn glBlendEquation(&self, _mode: BlendEquationModeEXT) {
        self.entry_gl14.glBlendEquation(_mode);
    }
    pub unsafe fn glSecondaryColor3b(&self, _red: GLbyte, _green: GLbyte, _blue: GLbyte) {
        self.entry_gl14.glSecondaryColor3b(_red, _green, _blue);
    }
    pub unsafe fn glSecondaryColor3i(&self, _red: GLint, _green: GLint, _blue: GLint) {
        self.entry_gl14.glSecondaryColor3i(_red, _green, _blue);
    }
    pub unsafe fn glPointParameterfv(
        &self,
        _pname: PointParameterNameARB,
        _params: *const GLfloat,
    ) {
        self.entry_gl14.glPointParameterfv(_pname, _params);
    }
    pub unsafe fn glWindowPos2fv(&self, _v: *const GLfloat) {
        self.entry_gl14.glWindowPos2fv(_v);
    }
    pub unsafe fn glFogCoorddv(&self, _coord: *const GLdouble) {
        self.entry_gl14.glFogCoorddv(_coord);
    }
    pub unsafe fn glSecondaryColorPointer(
        &self,
        _size: GLint,
        _type: ColorPointerType,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        self.entry_gl14
            .glSecondaryColorPointer(_size, _type, _stride, _pointer);
    }
    pub unsafe fn glFogCoordd(&self, _coord: GLdouble) {
        self.entry_gl14.glFogCoordd(_coord);
    }
    pub unsafe fn glSecondaryColor3d(&self, _red: GLdouble, _green: GLdouble, _blue: GLdouble) {
        self.entry_gl14.glSecondaryColor3d(_red, _green, _blue);
    }
    pub unsafe fn glWindowPos2d(&self, _x: GLdouble, _y: GLdouble) {
        self.entry_gl14.glWindowPos2d(_x, _y);
    }
    pub unsafe fn glWindowPos2s(&self, _x: GLshort, _y: GLshort) {
        self.entry_gl14.glWindowPos2s(_x, _y);
    }
    pub unsafe fn glSecondaryColor3usv(&self, _v: *const GLushort) {
        self.entry_gl14.glSecondaryColor3usv(_v);
    }
    pub unsafe fn glSecondaryColor3ub(&self, _red: GLubyte, _green: GLubyte, _blue: GLubyte) {
        self.entry_gl14.glSecondaryColor3ub(_red, _green, _blue);
    }
    pub unsafe fn glMultiDrawArrays(
        &self,
        _mode: PrimitiveType,
        _first: *const GLint,
        _count: *const GLsizei,
        _drawcount: GLsizei,
    ) {
        self.entry_gl14
            .glMultiDrawArrays(_mode, _first, _count, _drawcount);
    }
    pub unsafe fn glSecondaryColor3fv(&self, _v: *const GLfloat) {
        self.entry_gl14.glSecondaryColor3fv(_v);
    }
    pub unsafe fn glPointParameterf(&self, _pname: PointParameterNameARB, _param: GLfloat) {
        self.entry_gl14.glPointParameterf(_pname, _param);
    }
    pub unsafe fn glPointParameteri(&self, _pname: PointParameterNameARB, _param: GLint) {
        self.entry_gl14.glPointParameteri(_pname, _param);
    }
    pub unsafe fn glPointParameteriv(&self, _pname: PointParameterNameARB, _params: *const GLint) {
        self.entry_gl14.glPointParameteriv(_pname, _params);
    }
    pub unsafe fn glSecondaryColor3ui(&self, _red: GLuint, _green: GLuint, _blue: GLuint) {
        self.entry_gl14.glSecondaryColor3ui(_red, _green, _blue);
    }
    pub unsafe fn glWindowPos2dv(&self, _v: *const GLdouble) {
        self.entry_gl14.glWindowPos2dv(_v);
    }
    pub unsafe fn glSecondaryColor3f(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat) {
        self.entry_gl14.glSecondaryColor3f(_red, _green, _blue);
    }
    pub unsafe fn glWindowPos2f(&self, _x: GLfloat, _y: GLfloat) {
        self.entry_gl14.glWindowPos2f(_x, _y);
    }
    pub unsafe fn glMultiDrawElements(
        &self,
        _mode: PrimitiveType,
        _count: *const GLsizei,
        _type: DrawElementsType,
        _indices: *const *const std::os::raw::c_void,
        _drawcount: GLsizei,
    ) {
        self.entry_gl14
            .glMultiDrawElements(_mode, _count, _type, _indices, _drawcount);
    }
    pub unsafe fn glSecondaryColor3us(&self, _red: GLushort, _green: GLushort, _blue: GLushort) {
        self.entry_gl14.glSecondaryColor3us(_red, _green, _blue);
    }
    pub unsafe fn glSecondaryColor3iv(&self, _v: *const GLint) {
        self.entry_gl14.glSecondaryColor3iv(_v);
    }
    pub unsafe fn glWindowPos2i(&self, _x: GLint, _y: GLint) {
        self.entry_gl14.glWindowPos2i(_x, _y);
    }
    pub unsafe fn glWindowPos3dv(&self, _v: *const GLdouble) {
        self.entry_gl14.glWindowPos3dv(_v);
    }
    pub unsafe fn glWindowPos3fv(&self, _v: *const GLfloat) {
        self.entry_gl14.glWindowPos3fv(_v);
    }
    pub unsafe fn glFogCoordf(&self, _coord: GLfloat) {
        self.entry_gl14.glFogCoordf(_coord);
    }
    pub unsafe fn glSecondaryColor3ubv(&self, _v: *const GLubyte) {
        self.entry_gl14.glSecondaryColor3ubv(_v);
    }
    pub unsafe fn glLoadTransposeMatrixf(&self, _m: *const GLfloat) {
        self.entry_gl13.glLoadTransposeMatrixf(_m);
    }
    pub unsafe fn glMultiTexCoord4f(
        &self,
        _target: TextureUnit,
        _s: GLfloat,
        _t: GLfloat,
        _r: GLfloat,
        _q: GLfloat,
    ) {
        self.entry_gl13.glMultiTexCoord4f(_target, _s, _t, _r, _q);
    }
    pub unsafe fn glMultiTexCoord3s(
        &self,
        _target: TextureUnit,
        _s: GLshort,
        _t: GLshort,
        _r: GLshort,
    ) {
        self.entry_gl13.glMultiTexCoord3s(_target, _s, _t, _r);
    }
    pub unsafe fn glMultiTexCoord2f(&self, _target: TextureUnit, _s: GLfloat, _t: GLfloat) {
        self.entry_gl13.glMultiTexCoord2f(_target, _s, _t);
    }
    pub unsafe fn glMultiTexCoord2s(&self, _target: TextureUnit, _s: GLshort, _t: GLshort) {
        self.entry_gl13.glMultiTexCoord2s(_target, _s, _t);
    }
    pub unsafe fn glMultiTexCoord4d(
        &self,
        _target: TextureUnit,
        _s: GLdouble,
        _t: GLdouble,
        _r: GLdouble,
        _q: GLdouble,
    ) {
        self.entry_gl13.glMultiTexCoord4d(_target, _s, _t, _r, _q);
    }
    pub unsafe fn glMultiTexCoord3iv(&self, _target: TextureUnit, _v: *const GLint) {
        self.entry_gl13.glMultiTexCoord3iv(_target, _v);
    }
    pub unsafe fn glMultiTexCoord4i(
        &self,
        _target: TextureUnit,
        _s: GLint,
        _t: GLint,
        _r: GLint,
        _q: GLint,
    ) {
        self.entry_gl13.glMultiTexCoord4i(_target, _s, _t, _r, _q);
    }
    pub unsafe fn glActiveTexture(&self, _texture: TextureUnit) {
        self.entry_gl13.glActiveTexture(_texture);
    }
    pub unsafe fn glMultiTexCoord3f(
        &self,
        _target: TextureUnit,
        _s: GLfloat,
        _t: GLfloat,
        _r: GLfloat,
    ) {
        self.entry_gl13.glMultiTexCoord3f(_target, _s, _t, _r);
    }
    pub unsafe fn glMultiTexCoord4iv(&self, _target: TextureUnit, _v: *const GLint) {
        self.entry_gl13.glMultiTexCoord4iv(_target, _v);
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
        self.entry_gl13.glCompressedTexImage1D(
            _target,
            _level,
            _internalformat,
            _width,
            _border,
            _imageSize,
            _data,
        );
    }
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
        self.entry_gl13.glCompressedTexImage2D(
            _target,
            _level,
            _internalformat,
            _width,
            _height,
            _border,
            _imageSize,
            _data,
        );
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
        self.entry_gl13.glCompressedTexSubImage3D(
            _target, _level, _xoffset, _yoffset, _zoffset, _width, _height, _depth, _format,
            _imageSize, _data,
        );
    }
    pub unsafe fn glSampleCoverage(&self, _value: GLfloat, _invert: GLboolean) {
        self.entry_gl13.glSampleCoverage(_value, _invert);
    }
    pub unsafe fn glMultiTexCoord3d(
        &self,
        _target: TextureUnit,
        _s: GLdouble,
        _t: GLdouble,
        _r: GLdouble,
    ) {
        self.entry_gl13.glMultiTexCoord3d(_target, _s, _t, _r);
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
        self.entry_gl13.glCompressedTexImage3D(
            _target,
            _level,
            _internalformat,
            _width,
            _height,
            _depth,
            _border,
            _imageSize,
            _data,
        );
    }
    pub unsafe fn glMultiTexCoord2d(&self, _target: TextureUnit, _s: GLdouble, _t: GLdouble) {
        self.entry_gl13.glMultiTexCoord2d(_target, _s, _t);
    }
    pub unsafe fn glMultiTexCoord2i(&self, _target: TextureUnit, _s: GLint, _t: GLint) {
        self.entry_gl13.glMultiTexCoord2i(_target, _s, _t);
    }
    pub unsafe fn glMultTransposeMatrixd(&self, _m: *const GLdouble) {
        self.entry_gl13.glMultTransposeMatrixd(_m);
    }
    pub unsafe fn glMultiTexCoord2iv(&self, _target: TextureUnit, _v: *const GLint) {
        self.entry_gl13.glMultiTexCoord2iv(_target, _v);
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
        self.entry_gl13.glCompressedTexSubImage2D(
            _target, _level, _xoffset, _yoffset, _width, _height, _format, _imageSize, _data,
        );
    }
    pub unsafe fn glMultiTexCoord1sv(&self, _target: TextureUnit, _v: *const GLshort) {
        self.entry_gl13.glMultiTexCoord1sv(_target, _v);
    }
    pub unsafe fn glMultiTexCoord1f(&self, _target: TextureUnit, _s: GLfloat) {
        self.entry_gl13.glMultiTexCoord1f(_target, _s);
    }
    pub unsafe fn glGetCompressedTexImage(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _img: *mut std::os::raw::c_void,
    ) {
        self.entry_gl13
            .glGetCompressedTexImage(_target, _level, _img);
    }
    pub unsafe fn glMultiTexCoord2fv(&self, _target: TextureUnit, _v: *const GLfloat) {
        self.entry_gl13.glMultiTexCoord2fv(_target, _v);
    }
    pub unsafe fn glMultiTexCoord3sv(&self, _target: TextureUnit, _v: *const GLshort) {
        self.entry_gl13.glMultiTexCoord3sv(_target, _v);
    }
    pub unsafe fn glMultiTexCoord3dv(&self, _target: TextureUnit, _v: *const GLdouble) {
        self.entry_gl13.glMultiTexCoord3dv(_target, _v);
    }
    pub unsafe fn glMultiTexCoord3i(&self, _target: TextureUnit, _s: GLint, _t: GLint, _r: GLint) {
        self.entry_gl13.glMultiTexCoord3i(_target, _s, _t, _r);
    }
    pub unsafe fn glMultiTexCoord4sv(&self, _target: TextureUnit, _v: *const GLshort) {
        self.entry_gl13.glMultiTexCoord4sv(_target, _v);
    }
    pub unsafe fn glMultiTexCoord4dv(&self, _target: TextureUnit, _v: *const GLdouble) {
        self.entry_gl13.glMultiTexCoord4dv(_target, _v);
    }
    pub unsafe fn glClientActiveTexture(&self, _texture: TextureUnit) {
        self.entry_gl13.glClientActiveTexture(_texture);
    }
    pub unsafe fn glMultiTexCoord4fv(&self, _target: TextureUnit, _v: *const GLfloat) {
        self.entry_gl13.glMultiTexCoord4fv(_target, _v);
    }
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
        self.entry_gl13.glCompressedTexSubImage1D(
            _target, _level, _xoffset, _width, _format, _imageSize, _data,
        );
    }
    pub unsafe fn glMultiTexCoord3fv(&self, _target: TextureUnit, _v: *const GLfloat) {
        self.entry_gl13.glMultiTexCoord3fv(_target, _v);
    }
    pub unsafe fn glMultiTexCoord1fv(&self, _target: TextureUnit, _v: *const GLfloat) {
        self.entry_gl13.glMultiTexCoord1fv(_target, _v);
    }
    pub unsafe fn glMultiTexCoord1s(&self, _target: TextureUnit, _s: GLshort) {
        self.entry_gl13.glMultiTexCoord1s(_target, _s);
    }
    pub unsafe fn glLoadTransposeMatrixd(&self, _m: *const GLdouble) {
        self.entry_gl13.glLoadTransposeMatrixd(_m);
    }
    pub unsafe fn glMultiTexCoord1d(&self, _target: TextureUnit, _s: GLdouble) {
        self.entry_gl13.glMultiTexCoord1d(_target, _s);
    }
    pub unsafe fn glMultiTexCoord2sv(&self, _target: TextureUnit, _v: *const GLshort) {
        self.entry_gl13.glMultiTexCoord2sv(_target, _v);
    }
    pub unsafe fn glMultiTexCoord1dv(&self, _target: TextureUnit, _v: *const GLdouble) {
        self.entry_gl13.glMultiTexCoord1dv(_target, _v);
    }
    pub unsafe fn glMultiTexCoord1i(&self, _target: TextureUnit, _s: GLint) {
        self.entry_gl13.glMultiTexCoord1i(_target, _s);
    }
    pub unsafe fn glMultiTexCoord2dv(&self, _target: TextureUnit, _v: *const GLdouble) {
        self.entry_gl13.glMultiTexCoord2dv(_target, _v);
    }
    pub unsafe fn glMultiTexCoord4s(
        &self,
        _target: TextureUnit,
        _s: GLshort,
        _t: GLshort,
        _r: GLshort,
        _q: GLshort,
    ) {
        self.entry_gl13.glMultiTexCoord4s(_target, _s, _t, _r, _q);
    }
    pub unsafe fn glMultTransposeMatrixf(&self, _m: *const GLfloat) {
        self.entry_gl13.glMultTransposeMatrixf(_m);
    }
    pub unsafe fn glMultiTexCoord1iv(&self, _target: TextureUnit, _v: *const GLint) {
        self.entry_gl13.glMultiTexCoord1iv(_target, _v);
    }

    pub unsafe fn glUniformMatrix2x3dv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        self.entry_gl40
            .glUniformMatrix2x3dv(_location, _count, _transpose, _value);
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
        self.entry_gl40.glGetActiveSubroutineName(
            _program,
            _shadertype,
            _index,
            _bufSize,
            _length,
            _name,
        );
    }
    pub unsafe fn glGenTransformFeedbacks(&self, _n: GLsizei, _ids: *mut GLuint) {
        self.entry_gl40.glGenTransformFeedbacks(_n, _ids);
    }
    pub unsafe fn glUniform1dv(&self, _location: GLint, _count: GLsizei, _value: *const GLdouble) {
        self.entry_gl40.glUniform1dv(_location, _count, _value);
    }
    pub unsafe fn glBlendFunci(&self, _buf: GLuint, _src: BlendingFactor, _dst: BlendingFactor) {
        self.entry_gl40.glBlendFunci(_buf, _src, _dst);
    }
    pub unsafe fn glUniform2d(&self, _location: GLint, _x: GLdouble, _y: GLdouble) {
        self.entry_gl40.glUniform2d(_location, _x, _y);
    }
    pub unsafe fn glUniformMatrix2dv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        self.entry_gl40
            .glUniformMatrix2dv(_location, _count, _transpose, _value);
    }
    pub unsafe fn glGetActiveSubroutineUniformiv(
        &self,
        _program: GLuint,
        _shadertype: ShaderType,
        _index: GLuint,
        _pname: SubroutineParameterName,
        _values: *mut GLint,
    ) {
        self.entry_gl40.glGetActiveSubroutineUniformiv(
            _program,
            _shadertype,
            _index,
            _pname,
            _values,
        );
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
        self.entry_gl40.glGetActiveSubroutineUniformName(
            _program,
            _shadertype,
            _index,
            _bufSize,
            _length,
            _name,
        );
    }
    pub unsafe fn glBlendEquationSeparatei(
        &self,
        _buf: GLuint,
        _modeRGB: BlendEquationModeEXT,
        _modeAlpha: BlendEquationModeEXT,
    ) {
        self.entry_gl40
            .glBlendEquationSeparatei(_buf, _modeRGB, _modeAlpha);
    }
    pub unsafe fn glDrawArraysIndirect(
        &self,
        _mode: PrimitiveType,
        _indirect: *const std::os::raw::c_void,
    ) {
        self.entry_gl40.glDrawArraysIndirect(_mode, _indirect);
    }
    pub unsafe fn glDeleteTransformFeedbacks(&self, _n: GLsizei, _ids: *const GLuint) {
        self.entry_gl40.glDeleteTransformFeedbacks(_n, _ids);
    }
    pub unsafe fn glPatchParameteri(&self, _pname: PatchParameterName, _value: GLint) {
        self.entry_gl40.glPatchParameteri(_pname, _value);
    }
    pub unsafe fn glUniform1d(&self, _location: GLint, _x: GLdouble) {
        self.entry_gl40.glUniform1d(_location, _x);
    }
    pub unsafe fn glDrawTransformFeedback(&self, _mode: PrimitiveType, _id: GLuint) {
        self.entry_gl40.glDrawTransformFeedback(_mode, _id);
    }
    pub unsafe fn glBeginQueryIndexed(&self, _target: QueryTarget, _index: GLuint, _id: GLuint) {
        self.entry_gl40.glBeginQueryIndexed(_target, _index, _id);
    }
    pub unsafe fn glGetUniformSubroutineuiv(
        &self,
        _shadertype: ShaderType,
        _location: GLint,
        _params: *mut GLuint,
    ) {
        self.entry_gl40
            .glGetUniformSubroutineuiv(_shadertype, _location, _params);
    }
    pub unsafe fn glGetQueryIndexediv(
        &self,
        _target: QueryTarget,
        _index: GLuint,
        _pname: QueryParameterName,
        _params: *mut GLint,
    ) {
        self.entry_gl40
            .glGetQueryIndexediv(_target, _index, _pname, _params);
    }
    pub unsafe fn glBlendEquationi(&self, _buf: GLuint, _mode: BlendEquationModeEXT) {
        self.entry_gl40.glBlendEquationi(_buf, _mode);
    }
    pub unsafe fn glResumeTransformFeedback(&self) {
        self.entry_gl40.glResumeTransformFeedback();
    }
    pub unsafe fn glGetProgramStageiv(
        &self,
        _program: GLuint,
        _shadertype: ShaderType,
        _pname: ProgramStagePName,
        _values: *mut GLint,
    ) {
        self.entry_gl40
            .glGetProgramStageiv(_program, _shadertype, _pname, _values);
    }
    pub unsafe fn glUniform2dv(&self, _location: GLint, _count: GLsizei, _value: *const GLdouble) {
        self.entry_gl40.glUniform2dv(_location, _count, _value);
    }
    pub unsafe fn glUniformMatrix4x3dv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        self.entry_gl40
            .glUniformMatrix4x3dv(_location, _count, _transpose, _value);
    }
    pub unsafe fn glUniformSubroutinesuiv(
        &self,
        _shadertype: ShaderType,
        _count: GLsizei,
        _indices: *const GLuint,
    ) {
        self.entry_gl40
            .glUniformSubroutinesuiv(_shadertype, _count, _indices);
    }
    pub unsafe fn glDrawTransformFeedbackStream(
        &self,
        _mode: PrimitiveType,
        _id: GLuint,
        _stream: GLuint,
    ) {
        self.entry_gl40
            .glDrawTransformFeedbackStream(_mode, _id, _stream);
    }
    pub unsafe fn glDrawElementsIndirect(
        &self,
        _mode: PrimitiveType,
        _type: DrawElementsType,
        _indirect: *const std::os::raw::c_void,
    ) {
        self.entry_gl40
            .glDrawElementsIndirect(_mode, _type, _indirect);
    }
    pub unsafe fn glUniformMatrix3dv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        self.entry_gl40
            .glUniformMatrix3dv(_location, _count, _transpose, _value);
    }
    pub unsafe fn glUniform4d(
        &self,
        _location: GLint,
        _x: GLdouble,
        _y: GLdouble,
        _z: GLdouble,
        _w: GLdouble,
    ) {
        self.entry_gl40.glUniform4d(_location, _x, _y, _z, _w);
    }
    pub unsafe fn glGetUniformdv(
        &self,
        _program: GLuint,
        _location: GLint,
        _params: *mut GLdouble,
    ) {
        self.entry_gl40.glGetUniformdv(_program, _location, _params);
    }
    pub unsafe fn glBindTransformFeedback(
        &self,
        _target: BindTransformFeedbackTarget,
        _id: GLuint,
    ) {
        self.entry_gl40.glBindTransformFeedback(_target, _id);
    }
    pub unsafe fn glUniform3dv(&self, _location: GLint, _count: GLsizei, _value: *const GLdouble) {
        self.entry_gl40.glUniform3dv(_location, _count, _value);
    }
    pub unsafe fn glUniformMatrix3x4dv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        self.entry_gl40
            .glUniformMatrix3x4dv(_location, _count, _transpose, _value);
    }
    pub unsafe fn glMinSampleShading(&self, _value: GLfloat) {
        self.entry_gl40.glMinSampleShading(_value);
    }
    pub unsafe fn glUniformMatrix4x2dv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        self.entry_gl40
            .glUniformMatrix4x2dv(_location, _count, _transpose, _value);
    }
    pub unsafe fn glIsTransformFeedback(&self, _id: GLuint) -> GLboolean {
        self.entry_gl40.glIsTransformFeedback(_id)
    }
    pub unsafe fn glPatchParameterfv(&self, _pname: PatchParameterName, _values: *const GLfloat) {
        self.entry_gl40.glPatchParameterfv(_pname, _values);
    }
    pub unsafe fn glEndQueryIndexed(&self, _target: QueryTarget, _index: GLuint) {
        self.entry_gl40.glEndQueryIndexed(_target, _index);
    }
    pub unsafe fn glUniformMatrix2x4dv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        self.entry_gl40
            .glUniformMatrix2x4dv(_location, _count, _transpose, _value);
    }
    pub unsafe fn glGetSubroutineIndex(
        &self,
        _program: GLuint,
        _shadertype: ShaderType,
        _name: *const GLchar,
    ) -> GLuint {
        self.entry_gl40
            .glGetSubroutineIndex(_program, _shadertype, _name)
    }
    pub unsafe fn glUniform3d(&self, _location: GLint, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        self.entry_gl40.glUniform3d(_location, _x, _y, _z);
    }
    pub unsafe fn glUniformMatrix4dv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        self.entry_gl40
            .glUniformMatrix4dv(_location, _count, _transpose, _value);
    }
    pub unsafe fn glBlendFuncSeparatei(
        &self,
        _buf: GLuint,
        _srcRGB: BlendingFactor,
        _dstRGB: BlendingFactor,
        _srcAlpha: BlendingFactor,
        _dstAlpha: BlendingFactor,
    ) {
        self.entry_gl40
            .glBlendFuncSeparatei(_buf, _srcRGB, _dstRGB, _srcAlpha, _dstAlpha);
    }
    pub unsafe fn glUniformMatrix3x2dv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLdouble,
    ) {
        self.entry_gl40
            .glUniformMatrix3x2dv(_location, _count, _transpose, _value);
    }
    pub unsafe fn glGetSubroutineUniformLocation(
        &self,
        _program: GLuint,
        _shadertype: ShaderType,
        _name: *const GLchar,
    ) -> GLint {
        self.entry_gl40
            .glGetSubroutineUniformLocation(_program, _shadertype, _name)
    }
    pub unsafe fn glPauseTransformFeedback(&self) {
        self.entry_gl40.glPauseTransformFeedback();
    }
    pub unsafe fn glUniform4dv(&self, _location: GLint, _count: GLsizei, _value: *const GLdouble) {
        self.entry_gl40.glUniform4dv(_location, _count, _value);
    }
    pub unsafe fn glUniformMatrix2x4fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        self.entry_gl21
            .glUniformMatrix2x4fv(_location, _count, _transpose, _value);
    }
    pub unsafe fn glUniformMatrix4x2fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        self.entry_gl21
            .glUniformMatrix4x2fv(_location, _count, _transpose, _value);
    }
    pub unsafe fn glUniformMatrix2x3fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        self.entry_gl21
            .glUniformMatrix2x3fv(_location, _count, _transpose, _value);
    }
    pub unsafe fn glUniformMatrix3x2fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        self.entry_gl21
            .glUniformMatrix3x2fv(_location, _count, _transpose, _value);
    }
    pub unsafe fn glUniformMatrix3x4fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        self.entry_gl21
            .glUniformMatrix3x4fv(_location, _count, _transpose, _value);
    }
    pub unsafe fn glUniformMatrix4x3fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        self.entry_gl21
            .glUniformMatrix4x3fv(_location, _count, _transpose, _value);
    }

    pub unsafe fn glSecondaryColorP3ui(&self, _type: ColorPointerType, _color: GLuint) {
        self.entry_gl33.glSecondaryColorP3ui(_type, _color);
    }
    pub unsafe fn glMultiTexCoordP3uiv(
        &self,
        _texture: TextureUnit,
        _type: TexCoordPointerType,
        _coords: *const GLuint,
    ) {
        self.entry_gl33
            .glMultiTexCoordP3uiv(_texture, _type, _coords);
    }
    pub unsafe fn glSamplerParameterfv(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterF,
        _param: *const GLfloat,
    ) {
        self.entry_gl33
            .glSamplerParameterfv(_sampler, _pname, _param);
    }
    pub unsafe fn glColorP4uiv(&self, _type: ColorPointerType, _color: *const GLuint) {
        self.entry_gl33.glColorP4uiv(_type, _color);
    }
    pub unsafe fn glVertexAttribP3ui(
        &self,
        _index: GLuint,
        _type: VertexAttribPointerType,
        _normalized: GLboolean,
        _value: GLuint,
    ) {
        self.entry_gl33
            .glVertexAttribP3ui(_index, _type, _normalized, _value);
    }
    pub unsafe fn glTexCoordP1ui(&self, _type: TexCoordPointerType, _coords: GLuint) {
        self.entry_gl33.glTexCoordP1ui(_type, _coords);
    }
    pub unsafe fn glVertexP2uiv(&self, _type: VertexPointerType, _value: *const GLuint) {
        self.entry_gl33.glVertexP2uiv(_type, _value);
    }
    pub unsafe fn glVertexP4uiv(&self, _type: VertexPointerType, _value: *const GLuint) {
        self.entry_gl33.glVertexP4uiv(_type, _value);
    }
    pub unsafe fn glVertexP3ui(&self, _type: VertexPointerType, _value: GLuint) {
        self.entry_gl33.glVertexP3ui(_type, _value);
    }
    pub unsafe fn glVertexAttribP1uiv(
        &self,
        _index: GLuint,
        _type: VertexAttribPointerType,
        _normalized: GLboolean,
        _value: *const GLuint,
    ) {
        self.entry_gl33
            .glVertexAttribP1uiv(_index, _type, _normalized, _value);
    }
    pub unsafe fn glVertexP2ui(&self, _type: VertexPointerType, _value: GLuint) {
        self.entry_gl33.glVertexP2ui(_type, _value);
    }
    pub unsafe fn glVertexAttribP4uiv(
        &self,
        _index: GLuint,
        _type: VertexAttribPointerType,
        _normalized: GLboolean,
        _value: *const GLuint,
    ) {
        self.entry_gl33
            .glVertexAttribP4uiv(_index, _type, _normalized, _value);
    }
    pub unsafe fn glSamplerParameterIiv(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterI,
        _param: *const GLint,
    ) {
        self.entry_gl33
            .glSamplerParameterIiv(_sampler, _pname, _param);
    }
    pub unsafe fn glVertexAttribP2uiv(
        &self,
        _index: GLuint,
        _type: VertexAttribPointerType,
        _normalized: GLboolean,
        _value: *const GLuint,
    ) {
        self.entry_gl33
            .glVertexAttribP2uiv(_index, _type, _normalized, _value);
    }
    pub unsafe fn glVertexP4ui(&self, _type: VertexPointerType, _value: GLuint) {
        self.entry_gl33.glVertexP4ui(_type, _value);
    }
    pub unsafe fn glTexCoordP2ui(&self, _type: TexCoordPointerType, _coords: GLuint) {
        self.entry_gl33.glTexCoordP2ui(_type, _coords);
    }
    pub unsafe fn glIsSampler(&self, _sampler: GLuint) -> GLboolean {
        self.entry_gl33.glIsSampler(_sampler)
    }
    pub unsafe fn glTexCoordP3ui(&self, _type: TexCoordPointerType, _coords: GLuint) {
        self.entry_gl33.glTexCoordP3ui(_type, _coords);
    }
    pub unsafe fn glMultiTexCoordP4uiv(
        &self,
        _texture: TextureUnit,
        _type: TexCoordPointerType,
        _coords: *const GLuint,
    ) {
        self.entry_gl33
            .glMultiTexCoordP4uiv(_texture, _type, _coords);
    }
    pub unsafe fn glSamplerParameteri(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterI,
        _param: GLint,
    ) {
        self.entry_gl33
            .glSamplerParameteri(_sampler, _pname, _param);
    }
    pub unsafe fn glSamplerParameteriv(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterI,
        _param: *const GLint,
    ) {
        self.entry_gl33
            .glSamplerParameteriv(_sampler, _pname, _param);
    }
    pub unsafe fn glVertexAttribP1ui(
        &self,
        _index: GLuint,
        _type: VertexAttribPointerType,
        _normalized: GLboolean,
        _value: GLuint,
    ) {
        self.entry_gl33
            .glVertexAttribP1ui(_index, _type, _normalized, _value);
    }
    pub unsafe fn glMultiTexCoordP4ui(
        &self,
        _texture: TextureUnit,
        _type: TexCoordPointerType,
        _coords: GLuint,
    ) {
        self.entry_gl33
            .glMultiTexCoordP4ui(_texture, _type, _coords);
    }
    pub unsafe fn glGetSamplerParameterIiv(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterI,
        _params: *mut GLint,
    ) {
        self.entry_gl33
            .glGetSamplerParameterIiv(_sampler, _pname, _params);
    }
    pub unsafe fn glVertexP3uiv(&self, _type: VertexPointerType, _value: *const GLuint) {
        self.entry_gl33.glVertexP3uiv(_type, _value);
    }
    pub unsafe fn glGetFragDataIndex(&self, _program: GLuint, _name: *const GLchar) -> GLint {
        self.entry_gl33.glGetFragDataIndex(_program, _name)
    }
    pub unsafe fn glMultiTexCoordP1uiv(
        &self,
        _texture: TextureUnit,
        _type: TexCoordPointerType,
        _coords: *const GLuint,
    ) {
        self.entry_gl33
            .glMultiTexCoordP1uiv(_texture, _type, _coords);
    }
    pub unsafe fn glVertexAttribP4ui(
        &self,
        _index: GLuint,
        _type: VertexAttribPointerType,
        _normalized: GLboolean,
        _value: GLuint,
    ) {
        self.entry_gl33
            .glVertexAttribP4ui(_index, _type, _normalized, _value);
    }
    pub unsafe fn glTexCoordP4ui(&self, _type: TexCoordPointerType, _coords: GLuint) {
        self.entry_gl33.glTexCoordP4ui(_type, _coords);
    }
    pub unsafe fn glVertexAttribP2ui(
        &self,
        _index: GLuint,
        _type: VertexAttribPointerType,
        _normalized: GLboolean,
        _value: GLuint,
    ) {
        self.entry_gl33
            .glVertexAttribP2ui(_index, _type, _normalized, _value);
    }
    pub unsafe fn glBindFragDataLocationIndexed(
        &self,
        _program: GLuint,
        _colorNumber: GLuint,
        _index: GLuint,
        _name: *const GLchar,
    ) {
        self.entry_gl33
            .glBindFragDataLocationIndexed(_program, _colorNumber, _index, _name);
    }
    pub unsafe fn glTexCoordP3uiv(&self, _type: TexCoordPointerType, _coords: *const GLuint) {
        self.entry_gl33.glTexCoordP3uiv(_type, _coords);
    }
    pub unsafe fn glNormalP3uiv(&self, _type: NormalPointerType, _coords: *const GLuint) {
        self.entry_gl33.glNormalP3uiv(_type, _coords);
    }
    pub unsafe fn glVertexAttribDivisor(&self, _index: GLuint, _divisor: GLuint) {
        self.entry_gl33.glVertexAttribDivisor(_index, _divisor);
    }
    pub unsafe fn glTexCoordP1uiv(&self, _type: TexCoordPointerType, _coords: *const GLuint) {
        self.entry_gl33.glTexCoordP1uiv(_type, _coords);
    }
    pub unsafe fn glColorP3ui(&self, _type: ColorPointerType, _color: GLuint) {
        self.entry_gl33.glColorP3ui(_type, _color);
    }
    pub unsafe fn glSecondaryColorP3uiv(&self, _type: ColorPointerType, _color: *const GLuint) {
        self.entry_gl33.glSecondaryColorP3uiv(_type, _color);
    }
    pub unsafe fn glSamplerParameterIuiv(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterI,
        _param: *const GLuint,
    ) {
        self.entry_gl33
            .glSamplerParameterIuiv(_sampler, _pname, _param);
    }
    pub unsafe fn glGenSamplers(&self, _count: GLsizei, _samplers: *mut GLuint) {
        self.entry_gl33.glGenSamplers(_count, _samplers);
    }
    pub unsafe fn glSamplerParameterf(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterF,
        _param: GLfloat,
    ) {
        self.entry_gl33
            .glSamplerParameterf(_sampler, _pname, _param);
    }
    pub unsafe fn glColorP3uiv(&self, _type: ColorPointerType, _color: *const GLuint) {
        self.entry_gl33.glColorP3uiv(_type, _color);
    }
    pub unsafe fn glGetQueryObjecti64v(
        &self,
        _id: GLuint,
        _pname: QueryObjectParameterName,
        _params: *mut GLint64,
    ) {
        self.entry_gl33.glGetQueryObjecti64v(_id, _pname, _params);
    }
    pub unsafe fn glMultiTexCoordP2uiv(
        &self,
        _texture: TextureUnit,
        _type: TexCoordPointerType,
        _coords: *const GLuint,
    ) {
        self.entry_gl33
            .glMultiTexCoordP2uiv(_texture, _type, _coords);
    }
    pub unsafe fn glGetSamplerParameterIuiv(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterI,
        _params: *mut GLuint,
    ) {
        self.entry_gl33
            .glGetSamplerParameterIuiv(_sampler, _pname, _params);
    }
    pub unsafe fn glMultiTexCoordP2ui(
        &self,
        _texture: TextureUnit,
        _type: TexCoordPointerType,
        _coords: GLuint,
    ) {
        self.entry_gl33
            .glMultiTexCoordP2ui(_texture, _type, _coords);
    }
    pub unsafe fn glNormalP3ui(&self, _type: NormalPointerType, _coords: GLuint) {
        self.entry_gl33.glNormalP3ui(_type, _coords);
    }
    pub unsafe fn glQueryCounter(&self, _id: GLuint, _target: QueryCounterTarget) {
        self.entry_gl33.glQueryCounter(_id, _target);
    }
    pub unsafe fn glGetQueryObjectui64v(
        &self,
        _id: GLuint,
        _pname: QueryObjectParameterName,
        _params: *mut GLuint64,
    ) {
        self.entry_gl33.glGetQueryObjectui64v(_id, _pname, _params);
    }
    pub unsafe fn glMultiTexCoordP3ui(
        &self,
        _texture: TextureUnit,
        _type: TexCoordPointerType,
        _coords: GLuint,
    ) {
        self.entry_gl33
            .glMultiTexCoordP3ui(_texture, _type, _coords);
    }
    pub unsafe fn glTexCoordP4uiv(&self, _type: TexCoordPointerType, _coords: *const GLuint) {
        self.entry_gl33.glTexCoordP4uiv(_type, _coords);
    }
    pub unsafe fn glBindSampler(&self, _unit: GLuint, _sampler: GLuint) {
        self.entry_gl33.glBindSampler(_unit, _sampler);
    }
    pub unsafe fn glDeleteSamplers(&self, _count: GLsizei, _samplers: *const GLuint) {
        self.entry_gl33.glDeleteSamplers(_count, _samplers);
    }
    pub unsafe fn glGetSamplerParameterfv(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterF,
        _params: *mut GLfloat,
    ) {
        self.entry_gl33
            .glGetSamplerParameterfv(_sampler, _pname, _params);
    }
    pub unsafe fn glColorP4ui(&self, _type: ColorPointerType, _color: GLuint) {
        self.entry_gl33.glColorP4ui(_type, _color);
    }
    pub unsafe fn glVertexAttribP3uiv(
        &self,
        _index: GLuint,
        _type: VertexAttribPointerType,
        _normalized: GLboolean,
        _value: *const GLuint,
    ) {
        self.entry_gl33
            .glVertexAttribP3uiv(_index, _type, _normalized, _value);
    }
    pub unsafe fn glTexCoordP2uiv(&self, _type: TexCoordPointerType, _coords: *const GLuint) {
        self.entry_gl33.glTexCoordP2uiv(_type, _coords);
    }
    pub unsafe fn glMultiTexCoordP1ui(
        &self,
        _texture: TextureUnit,
        _type: TexCoordPointerType,
        _coords: GLuint,
    ) {
        self.entry_gl33
            .glMultiTexCoordP1ui(_texture, _type, _coords);
    }
    pub unsafe fn glGetSamplerParameteriv(
        &self,
        _sampler: GLuint,
        _pname: SamplerParameterI,
        _params: *mut GLint,
    ) {
        self.entry_gl33
            .glGetSamplerParameteriv(_sampler, _pname, _params);
    }

    pub unsafe fn glGetActiveUniformName(
        &self,
        _program: GLuint,
        _uniformIndex: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _uniformName: *mut GLchar,
    ) {
        self.entry_gl31.glGetActiveUniformName(
            _program,
            _uniformIndex,
            _bufSize,
            _length,
            _uniformName,
        );
    }
    pub unsafe fn glCopyBufferSubData(
        &self,
        _readTarget: CopyBufferSubDataTarget,
        _writeTarget: CopyBufferSubDataTarget,
        _readOffset: GLintptr,
        _writeOffset: GLintptr,
        _size: GLsizeiptr,
    ) {
        self.entry_gl31.glCopyBufferSubData(
            _readTarget,
            _writeTarget,
            _readOffset,
            _writeOffset,
            _size,
        );
    }
    pub unsafe fn glBindBufferBase(
        &self,
        _target: BufferTargetARB,
        _index: GLuint,
        _buffer: GLuint,
    ) {
        self.entry_gl31.glBindBufferBase(_target, _index, _buffer);
    }
    pub unsafe fn glGetActiveUniformBlockiv(
        &self,
        _program: GLuint,
        _uniformBlockIndex: GLuint,
        _pname: UniformBlockPName,
        _params: *mut GLint,
    ) {
        self.entry_gl31
            .glGetActiveUniformBlockiv(_program, _uniformBlockIndex, _pname, _params);
    }
    pub unsafe fn glBindBufferRange(
        &self,
        _target: BufferTargetARB,
        _index: GLuint,
        _buffer: GLuint,
        _offset: GLintptr,
        _size: GLsizeiptr,
    ) {
        self.entry_gl31
            .glBindBufferRange(_target, _index, _buffer, _offset, _size);
    }
    pub unsafe fn glPrimitiveRestartIndex(&self, _index: GLuint) {
        self.entry_gl31.glPrimitiveRestartIndex(_index);
    }
    pub unsafe fn glDrawArraysInstanced(
        &self,
        _mode: PrimitiveType,
        _first: GLint,
        _count: GLsizei,
        _instancecount: GLsizei,
    ) {
        self.entry_gl31
            .glDrawArraysInstanced(_mode, _first, _count, _instancecount);
    }
    pub unsafe fn glDrawElementsInstanced(
        &self,
        _mode: PrimitiveType,
        _count: GLsizei,
        _type: DrawElementsType,
        _indices: *const std::os::raw::c_void,
        _instancecount: GLsizei,
    ) {
        self.entry_gl31
            .glDrawElementsInstanced(_mode, _count, _type, _indices, _instancecount);
    }
    pub unsafe fn glGetUniformIndices(
        &self,
        _program: GLuint,
        _uniformCount: GLsizei,
        _uniformNames: *const *const GLchar,
        _uniformIndices: *mut GLuint,
    ) {
        self.entry_gl31.glGetUniformIndices(
            _program,
            _uniformCount,
            _uniformNames,
            _uniformIndices,
        );
    }
    pub unsafe fn glGetUniformBlockIndex(
        &self,
        _program: GLuint,
        _uniformBlockName: *const GLchar,
    ) -> GLuint {
        self.entry_gl31
            .glGetUniformBlockIndex(_program, _uniformBlockName)
    }
    pub unsafe fn glUniformBlockBinding(
        &self,
        _program: GLuint,
        _uniformBlockIndex: GLuint,
        _uniformBlockBinding: GLuint,
    ) {
        self.entry_gl31
            .glUniformBlockBinding(_program, _uniformBlockIndex, _uniformBlockBinding);
    }
    pub unsafe fn glGetActiveUniformsiv(
        &self,
        _program: GLuint,
        _uniformCount: GLsizei,
        _uniformIndices: *const GLuint,
        _pname: UniformPName,
        _params: *mut GLint,
    ) {
        self.entry_gl31.glGetActiveUniformsiv(
            _program,
            _uniformCount,
            _uniformIndices,
            _pname,
            _params,
        );
    }
    pub unsafe fn glTexBuffer(
        &self,
        _target: TextureTarget,
        _internalformat: SizedInternalFormat,
        _buffer: GLuint,
    ) {
        self.entry_gl31
            .glTexBuffer(_target, _internalformat, _buffer);
    }
    pub unsafe fn glGetActiveUniformBlockName(
        &self,
        _program: GLuint,
        _uniformBlockIndex: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _uniformBlockName: *mut GLchar,
    ) {
        self.entry_gl31.glGetActiveUniformBlockName(
            _program,
            _uniformBlockIndex,
            _bufSize,
            _length,
            _uniformBlockName,
        );
    }
    pub unsafe fn glGetIntegeri_v(&self, _target: GetPName, _index: GLuint, _data: *mut GLint) {
        self.entry_gl31.glGetIntegeri_v(_target, _index, _data);
    }
}
