use crate::gl::feature::EntryGLFn;
use crate::types::*;
use std::mem;
pub trait GL21 {
    unsafe fn entry(&self) -> &EntryGLFn;
    unsafe fn glAccum(&self, _op: GLenum, _value: GLfloat) {
        (self.entry().glAccum)(_op, _value)
    }
    unsafe fn glActiveTexture(&self, _texture: GLenum) {
        (self.entry().glActiveTexture)(_texture)
    }
    unsafe fn glAlphaFunc(&self, _func: GLenum, _ref: GLfloat) {
        (self.entry().glAlphaFunc)(_func, _ref)
    }
    unsafe fn glAreTexturesResident(
        &self,
        _n: GLsizei,
        _textures: *const GLuint,
        _residences: *mut GLboolean,
    ) -> GLboolean {
        (self.entry().glAreTexturesResident)(_n, _textures, _residences)
    }
    unsafe fn glArrayElement(&self, _i: GLint) {
        (self.entry().glArrayElement)(_i)
    }
    unsafe fn glAttachShader(&self, _program: GLuint, _shader: GLuint) {
        (self.entry().glAttachShader)(_program, _shader)
    }
    unsafe fn glBegin(&self, _mode: GLenum) {
        (self.entry().glBegin)(_mode)
    }
    unsafe fn glBeginQuery(&self, _target: GLenum, _id: GLuint) {
        (self.entry().glBeginQuery)(_target, _id)
    }
    unsafe fn glBindAttribLocation(&self, _program: GLuint, _index: GLuint, _name: *const GLchar) {
        (self.entry().glBindAttribLocation)(_program, _index, _name)
    }
    unsafe fn glBindBuffer(&self, _target: GLenum, _buffer: GLuint) {
        (self.entry().glBindBuffer)(_target, _buffer)
    }
    unsafe fn glBindTexture(&self, _target: GLenum, _texture: GLuint) {
        (self.entry().glBindTexture)(_target, _texture)
    }
    unsafe fn glBitmap(
        &self,
        _width: GLsizei,
        _height: GLsizei,
        _xorig: GLfloat,
        _yorig: GLfloat,
        _xmove: GLfloat,
        _ymove: GLfloat,
        _bitmap: *const GLubyte,
    ) {
        (self.entry().glBitmap)(_width, _height, _xorig, _yorig, _xmove, _ymove, _bitmap)
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
    unsafe fn glCallList(&self, _list: GLuint) {
        (self.entry().glCallList)(_list)
    }
    unsafe fn glCallLists(&self, _n: GLsizei, _type: GLenum, _lists: *const std::os::raw::c_void) {
        (self.entry().glCallLists)(_n, _type, _lists)
    }
    unsafe fn glClear(&self, _mask: GLbitfield) {
        (self.entry().glClear)(_mask)
    }
    unsafe fn glClearAccum(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat, _alpha: GLfloat) {
        (self.entry().glClearAccum)(_red, _green, _blue, _alpha)
    }
    unsafe fn glClearColor(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat, _alpha: GLfloat) {
        (self.entry().glClearColor)(_red, _green, _blue, _alpha)
    }
    unsafe fn glClearDepth(&self, _depth: GLdouble) {
        (self.entry().glClearDepth)(_depth)
    }
    unsafe fn glClearIndex(&self, _c: GLfloat) {
        (self.entry().glClearIndex)(_c)
    }
    unsafe fn glClearStencil(&self, _s: GLint) {
        (self.entry().glClearStencil)(_s)
    }
    unsafe fn glClientActiveTexture(&self, _texture: GLenum) {
        (self.entry().glClientActiveTexture)(_texture)
    }
    unsafe fn glClipPlane(&self, _plane: GLenum, _equation: *const GLdouble) {
        (self.entry().glClipPlane)(_plane, _equation)
    }
    unsafe fn glColor3b(&self, _red: GLbyte, _green: GLbyte, _blue: GLbyte) {
        (self.entry().glColor3b)(_red, _green, _blue)
    }
    unsafe fn glColor3bv(&self, _v: *const GLbyte) {
        (self.entry().glColor3bv)(_v)
    }
    unsafe fn glColor3d(&self, _red: GLdouble, _green: GLdouble, _blue: GLdouble) {
        (self.entry().glColor3d)(_red, _green, _blue)
    }
    unsafe fn glColor3dv(&self, _v: *const GLdouble) {
        (self.entry().glColor3dv)(_v)
    }
    unsafe fn glColor3f(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat) {
        (self.entry().glColor3f)(_red, _green, _blue)
    }
    unsafe fn glColor3fv(&self, _v: *const GLfloat) {
        (self.entry().glColor3fv)(_v)
    }
    unsafe fn glColor3i(&self, _red: GLint, _green: GLint, _blue: GLint) {
        (self.entry().glColor3i)(_red, _green, _blue)
    }
    unsafe fn glColor3iv(&self, _v: *const GLint) {
        (self.entry().glColor3iv)(_v)
    }
    unsafe fn glColor3s(&self, _red: GLshort, _green: GLshort, _blue: GLshort) {
        (self.entry().glColor3s)(_red, _green, _blue)
    }
    unsafe fn glColor3sv(&self, _v: *const GLshort) {
        (self.entry().glColor3sv)(_v)
    }
    unsafe fn glColor3ub(&self, _red: GLubyte, _green: GLubyte, _blue: GLubyte) {
        (self.entry().glColor3ub)(_red, _green, _blue)
    }
    unsafe fn glColor3ubv(&self, _v: *const GLubyte) {
        (self.entry().glColor3ubv)(_v)
    }
    unsafe fn glColor3ui(&self, _red: GLuint, _green: GLuint, _blue: GLuint) {
        (self.entry().glColor3ui)(_red, _green, _blue)
    }
    unsafe fn glColor3uiv(&self, _v: *const GLuint) {
        (self.entry().glColor3uiv)(_v)
    }
    unsafe fn glColor3us(&self, _red: GLushort, _green: GLushort, _blue: GLushort) {
        (self.entry().glColor3us)(_red, _green, _blue)
    }
    unsafe fn glColor3usv(&self, _v: *const GLushort) {
        (self.entry().glColor3usv)(_v)
    }
    unsafe fn glColor4b(&self, _red: GLbyte, _green: GLbyte, _blue: GLbyte, _alpha: GLbyte) {
        (self.entry().glColor4b)(_red, _green, _blue, _alpha)
    }
    unsafe fn glColor4bv(&self, _v: *const GLbyte) {
        (self.entry().glColor4bv)(_v)
    }
    unsafe fn glColor4d(
        &self,
        _red: GLdouble,
        _green: GLdouble,
        _blue: GLdouble,
        _alpha: GLdouble,
    ) {
        (self.entry().glColor4d)(_red, _green, _blue, _alpha)
    }
    unsafe fn glColor4dv(&self, _v: *const GLdouble) {
        (self.entry().glColor4dv)(_v)
    }
    unsafe fn glColor4f(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat, _alpha: GLfloat) {
        (self.entry().glColor4f)(_red, _green, _blue, _alpha)
    }
    unsafe fn glColor4fv(&self, _v: *const GLfloat) {
        (self.entry().glColor4fv)(_v)
    }
    unsafe fn glColor4i(&self, _red: GLint, _green: GLint, _blue: GLint, _alpha: GLint) {
        (self.entry().glColor4i)(_red, _green, _blue, _alpha)
    }
    unsafe fn glColor4iv(&self, _v: *const GLint) {
        (self.entry().glColor4iv)(_v)
    }
    unsafe fn glColor4s(&self, _red: GLshort, _green: GLshort, _blue: GLshort, _alpha: GLshort) {
        (self.entry().glColor4s)(_red, _green, _blue, _alpha)
    }
    unsafe fn glColor4sv(&self, _v: *const GLshort) {
        (self.entry().glColor4sv)(_v)
    }
    unsafe fn glColor4ub(&self, _red: GLubyte, _green: GLubyte, _blue: GLubyte, _alpha: GLubyte) {
        (self.entry().glColor4ub)(_red, _green, _blue, _alpha)
    }
    unsafe fn glColor4ubv(&self, _v: *const GLubyte) {
        (self.entry().glColor4ubv)(_v)
    }
    unsafe fn glColor4ui(&self, _red: GLuint, _green: GLuint, _blue: GLuint, _alpha: GLuint) {
        (self.entry().glColor4ui)(_red, _green, _blue, _alpha)
    }
    unsafe fn glColor4uiv(&self, _v: *const GLuint) {
        (self.entry().glColor4uiv)(_v)
    }
    unsafe fn glColor4us(
        &self,
        _red: GLushort,
        _green: GLushort,
        _blue: GLushort,
        _alpha: GLushort,
    ) {
        (self.entry().glColor4us)(_red, _green, _blue, _alpha)
    }
    unsafe fn glColor4usv(&self, _v: *const GLushort) {
        (self.entry().glColor4usv)(_v)
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
    unsafe fn glColorMaterial(&self, _face: GLenum, _mode: GLenum) {
        (self.entry().glColorMaterial)(_face, _mode)
    }
    unsafe fn glColorPointer(
        &self,
        _size: GLint,
        _type: GLenum,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glColorPointer)(_size, _type, _stride, _pointer)
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
    unsafe fn glCopyPixels(
        &self,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _type: GLenum,
    ) {
        (self.entry().glCopyPixels)(_x, _y, _width, _height, _type)
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
    unsafe fn glCullFace(&self, _mode: GLenum) {
        (self.entry().glCullFace)(_mode)
    }
    unsafe fn glDeleteBuffers(&self, array: &[GLuint]) {
        (self.entry().glDeleteBuffers)(array.len() as GLsizei, array.as_ptr())
    }
    unsafe fn glDeleteLists(&self, _list: GLuint, _range: GLsizei) {
        (self.entry().glDeleteLists)(_list, _range)
    }
    unsafe fn glDeleteProgram(&self, _program: GLuint) {
        (self.entry().glDeleteProgram)(_program)
    }
    unsafe fn glDeleteQueries(&self, array: &[GLuint]) {
        (self.entry().glDeleteQueries)(array.len() as GLsizei, array.as_ptr())
    }
    unsafe fn glDeleteShader(&self, _shader: GLuint) {
        (self.entry().glDeleteShader)(_shader)
    }
    unsafe fn glDeleteTextures(&self, _n: GLsizei, _textures: *const GLuint) {
        (self.entry().glDeleteTextures)(_n, _textures)
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
    unsafe fn glDetachShader(&self, _program: GLuint, _shader: GLuint) {
        (self.entry().glDetachShader)(_program, _shader)
    }
    unsafe fn glDisable(&self, _cap: GLenum) {
        (self.entry().glDisable)(_cap)
    }
    unsafe fn glDisableClientState(&self, _array: GLenum) {
        (self.entry().glDisableClientState)(_array)
    }
    unsafe fn glDisableVertexAttribArray(&self, _index: GLuint) {
        (self.entry().glDisableVertexAttribArray)(_index)
    }
    unsafe fn glDrawArrays(&self, _mode: GLenum, _first: GLint, _count: GLsizei) {
        (self.entry().glDrawArrays)(_mode, _first, _count)
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
    unsafe fn glDrawPixels(
        &self,
        _width: GLsizei,
        _height: GLsizei,
        _format: GLenum,
        _type: GLenum,
        _pixels: *const std::os::raw::c_void,
    ) {
        (self.entry().glDrawPixels)(_width, _height, _format, _type, _pixels)
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
    unsafe fn glEdgeFlag(&self, _flag: GLboolean) {
        (self.entry().glEdgeFlag)(_flag)
    }
    unsafe fn glEdgeFlagPointer(&self, _stride: GLsizei, _pointer: *const std::os::raw::c_void) {
        (self.entry().glEdgeFlagPointer)(_stride, _pointer)
    }
    unsafe fn glEdgeFlagv(&self, _flag: *const GLboolean) {
        (self.entry().glEdgeFlagv)(_flag)
    }
    unsafe fn glEnable(&self, _cap: GLenum) {
        (self.entry().glEnable)(_cap)
    }
    unsafe fn glEnableClientState(&self, _array: GLenum) {
        (self.entry().glEnableClientState)(_array)
    }
    unsafe fn glEnableVertexAttribArray(&self, _index: GLuint) {
        (self.entry().glEnableVertexAttribArray)(_index)
    }
    unsafe fn glEnd(&self) {
        (self.entry().glEnd)()
    }
    unsafe fn glEndList(&self) {
        (self.entry().glEndList)()
    }
    unsafe fn glEndQuery(&self, _target: GLenum) {
        (self.entry().glEndQuery)(_target)
    }
    unsafe fn glEvalCoord1d(&self, _u: GLdouble) {
        (self.entry().glEvalCoord1d)(_u)
    }
    unsafe fn glEvalCoord1dv(&self, _u: *const GLdouble) {
        (self.entry().glEvalCoord1dv)(_u)
    }
    unsafe fn glEvalCoord1f(&self, _u: GLfloat) {
        (self.entry().glEvalCoord1f)(_u)
    }
    unsafe fn glEvalCoord1fv(&self, _u: *const GLfloat) {
        (self.entry().glEvalCoord1fv)(_u)
    }
    unsafe fn glEvalCoord2d(&self, _u: GLdouble, _v: GLdouble) {
        (self.entry().glEvalCoord2d)(_u, _v)
    }
    unsafe fn glEvalCoord2dv(&self, _u: *const GLdouble) {
        (self.entry().glEvalCoord2dv)(_u)
    }
    unsafe fn glEvalCoord2f(&self, _u: GLfloat, _v: GLfloat) {
        (self.entry().glEvalCoord2f)(_u, _v)
    }
    unsafe fn glEvalCoord2fv(&self, _u: *const GLfloat) {
        (self.entry().glEvalCoord2fv)(_u)
    }
    unsafe fn glEvalMesh1(&self, _mode: GLenum, _i1: GLint, _i2: GLint) {
        (self.entry().glEvalMesh1)(_mode, _i1, _i2)
    }
    unsafe fn glEvalMesh2(&self, _mode: GLenum, _i1: GLint, _i2: GLint, _j1: GLint, _j2: GLint) {
        (self.entry().glEvalMesh2)(_mode, _i1, _i2, _j1, _j2)
    }
    unsafe fn glEvalPoint1(&self, _i: GLint) {
        (self.entry().glEvalPoint1)(_i)
    }
    unsafe fn glEvalPoint2(&self, _i: GLint, _j: GLint) {
        (self.entry().glEvalPoint2)(_i, _j)
    }
    unsafe fn glFeedbackBuffer(&self, _size: GLsizei, _type: GLenum, _buffer: *mut GLfloat) {
        (self.entry().glFeedbackBuffer)(_size, _type, _buffer)
    }
    unsafe fn glFinish(&self) {
        (self.entry().glFinish)()
    }
    unsafe fn glFlush(&self) {
        (self.entry().glFlush)()
    }
    unsafe fn glFogCoordPointer(
        &self,
        _type: GLenum,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glFogCoordPointer)(_type, _stride, _pointer)
    }
    unsafe fn glFogCoordd(&self, _coord: GLdouble) {
        (self.entry().glFogCoordd)(_coord)
    }
    unsafe fn glFogCoorddv(&self, _coord: *const GLdouble) {
        (self.entry().glFogCoorddv)(_coord)
    }
    unsafe fn glFogCoordf(&self, _coord: GLfloat) {
        (self.entry().glFogCoordf)(_coord)
    }
    unsafe fn glFogCoordfv(&self, _coord: *const GLfloat) {
        (self.entry().glFogCoordfv)(_coord)
    }
    unsafe fn glFogf(&self, _pname: GLenum, _param: GLfloat) {
        (self.entry().glFogf)(_pname, _param)
    }
    unsafe fn glFogfv(&self, _pname: GLenum, _params: *const GLfloat) {
        (self.entry().glFogfv)(_pname, _params)
    }
    unsafe fn glFogi(&self, _pname: GLenum, _param: GLint) {
        (self.entry().glFogi)(_pname, _param)
    }
    unsafe fn glFogiv(&self, _pname: GLenum, _params: *const GLint) {
        (self.entry().glFogiv)(_pname, _params)
    }
    unsafe fn glFrontFace(&self, _mode: GLenum) {
        (self.entry().glFrontFace)(_mode)
    }
    unsafe fn glFrustum(
        &self,
        _left: GLdouble,
        _right: GLdouble,
        _bottom: GLdouble,
        _top: GLdouble,
        _zNear: GLdouble,
        _zFar: GLdouble,
    ) {
        (self.entry().glFrustum)(_left, _right, _bottom, _top, _zNear, _zFar)
    }
    unsafe fn glGenBuffers(&self, _n: GLsizei, _buffers: *mut GLuint) {
        (self.entry().glGenBuffers)(_n, _buffers)
    }
    unsafe fn glGenLists(&self, _range: GLsizei) -> GLuint {
        (self.entry().glGenLists)(_range)
    }
    unsafe fn glGenQueries(&self, array: &mut [GLuint]) {
        (self.entry().glGenQueries)(array.len() as GLsizei, array.as_mut_ptr())
    }
    unsafe fn glGenTextures(&self, array: &mut [GLuint]) {
        (self.entry().glGenTextures)(array.len() as GLsizei, array.as_mut_ptr())
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
    unsafe fn glGetBooleanv(&self, _pname: GLenum, _data: *mut GLboolean) {
        (self.entry().glGetBooleanv)(_pname, _data)
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
    unsafe fn glGetClipPlane(&self, _plane: GLenum, _equation: *mut GLdouble) {
        (self.entry().glGetClipPlane)(_plane, _equation)
    }
    unsafe fn glGetCompressedTexImage(
        &self,
        _target: GLenum,
        _level: GLint,
        _img: *mut std::os::raw::c_void,
    ) {
        (self.entry().glGetCompressedTexImage)(_target, _level, _img)
    }
    unsafe fn glGetDoublev(&self, _pname: GLenum, _data: *mut GLdouble) {
        (self.entry().glGetDoublev)(_pname, _data)
    }
    unsafe fn glGetError(&self) -> GLenum {
        (self.entry().glGetError)()
    }
    unsafe fn glGetFloatv(&self, _pname: GLenum, _data: *mut GLfloat) {
        (self.entry().glGetFloatv)(_pname, _data)
    }
    unsafe fn glGetIntegerv(&self, _pname: GLenum, _data: *mut GLint) {
        (self.entry().glGetIntegerv)(_pname, _data)
    }
    unsafe fn glGetLightfv(&self, _light: GLenum, _pname: GLenum, _params: *mut GLfloat) {
        (self.entry().glGetLightfv)(_light, _pname, _params)
    }
    unsafe fn glGetLightiv(&self, _light: GLenum, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetLightiv)(_light, _pname, _params)
    }
    unsafe fn glGetMapdv(&self, _target: GLenum, _query: GLenum, _v: *mut GLdouble) {
        (self.entry().glGetMapdv)(_target, _query, _v)
    }
    unsafe fn glGetMapfv(&self, _target: GLenum, _query: GLenum, _v: *mut GLfloat) {
        (self.entry().glGetMapfv)(_target, _query, _v)
    }
    unsafe fn glGetMapiv(&self, _target: GLenum, _query: GLenum, _v: *mut GLint) {
        (self.entry().glGetMapiv)(_target, _query, _v)
    }
    unsafe fn glGetMaterialfv(&self, _face: GLenum, _pname: GLenum, _params: *mut GLfloat) {
        (self.entry().glGetMaterialfv)(_face, _pname, _params)
    }
    unsafe fn glGetMaterialiv(&self, _face: GLenum, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetMaterialiv)(_face, _pname, _params)
    }
    unsafe fn glGetPixelMapfv(&self, _map: GLenum, _values: *mut GLfloat) {
        (self.entry().glGetPixelMapfv)(_map, _values)
    }
    unsafe fn glGetPixelMapuiv(&self, _map: GLenum, _values: *mut GLuint) {
        (self.entry().glGetPixelMapuiv)(_map, _values)
    }
    unsafe fn glGetPixelMapusv(&self, _map: GLenum, _values: *mut GLushort) {
        (self.entry().glGetPixelMapusv)(_map, _values)
    }
    unsafe fn glGetPointerv(&self, _pname: GLenum, _params: *mut *mut std::os::raw::c_void) {
        (self.entry().glGetPointerv)(_pname, _params)
    }
    unsafe fn glGetPolygonStipple(&self, _mask: *mut GLubyte) {
        (self.entry().glGetPolygonStipple)(_mask)
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
    unsafe fn glGetProgramiv(&self, _program: GLuint, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetProgramiv)(_program, _pname, _params)
    }
    unsafe fn glGetQueryObjectiv(&self, _id: GLuint, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetQueryObjectiv)(_id, _pname, _params)
    }
    unsafe fn glGetQueryObjectuiv(&self, _id: GLuint, _pname: GLenum, _params: *mut GLuint) {
        (self.entry().glGetQueryObjectuiv)(_id, _pname, _params)
    }
    unsafe fn glGetQueryiv(&self, _target: GLenum, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetQueryiv)(_target, _pname, _params)
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
    unsafe fn glGetTexEnvfv(&self, _target: GLenum, _pname: GLenum, _params: *mut GLfloat) {
        (self.entry().glGetTexEnvfv)(_target, _pname, _params)
    }
    unsafe fn glGetTexEnviv(&self, _target: GLenum, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetTexEnviv)(_target, _pname, _params)
    }
    unsafe fn glGetTexGendv(&self, _coord: GLenum, _pname: GLenum, _params: *mut GLdouble) {
        (self.entry().glGetTexGendv)(_coord, _pname, _params)
    }
    unsafe fn glGetTexGenfv(&self, _coord: GLenum, _pname: GLenum, _params: *mut GLfloat) {
        (self.entry().glGetTexGenfv)(_coord, _pname, _params)
    }
    unsafe fn glGetTexGeniv(&self, _coord: GLenum, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetTexGeniv)(_coord, _pname, _params)
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
    unsafe fn glGetTexParameterfv(&self, _target: GLenum, _pname: GLenum, _params: *mut GLfloat) {
        (self.entry().glGetTexParameterfv)(_target, _pname, _params)
    }
    unsafe fn glGetTexParameteriv(&self, _target: GLenum, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetTexParameteriv)(_target, _pname, _params)
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
    unsafe fn glIndexMask(&self, _mask: GLuint) {
        (self.entry().glIndexMask)(_mask)
    }
    unsafe fn glIndexPointer(
        &self,
        _type: GLenum,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glIndexPointer)(_type, _stride, _pointer)
    }
    unsafe fn glIndexd(&self, _c: GLdouble) {
        (self.entry().glIndexd)(_c)
    }
    unsafe fn glIndexdv(&self, _c: *const GLdouble) {
        (self.entry().glIndexdv)(_c)
    }
    unsafe fn glIndexf(&self, _c: GLfloat) {
        (self.entry().glIndexf)(_c)
    }
    unsafe fn glIndexfv(&self, _c: *const GLfloat) {
        (self.entry().glIndexfv)(_c)
    }
    unsafe fn glIndexi(&self, _c: GLint) {
        (self.entry().glIndexi)(_c)
    }
    unsafe fn glIndexiv(&self, _c: *const GLint) {
        (self.entry().glIndexiv)(_c)
    }
    unsafe fn glIndexs(&self, _c: GLshort) {
        (self.entry().glIndexs)(_c)
    }
    unsafe fn glIndexsv(&self, _c: *const GLshort) {
        (self.entry().glIndexsv)(_c)
    }
    unsafe fn glIndexub(&self, _c: GLubyte) {
        (self.entry().glIndexub)(_c)
    }
    unsafe fn glIndexubv(&self, _c: *const GLubyte) {
        (self.entry().glIndexubv)(_c)
    }
    unsafe fn glInitNames(&self) {
        (self.entry().glInitNames)()
    }
    unsafe fn glInterleavedArrays(
        &self,
        _format: GLenum,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glInterleavedArrays)(_format, _stride, _pointer)
    }
    unsafe fn glIsBuffer(&self, _buffer: GLuint) -> GLboolean {
        (self.entry().glIsBuffer)(_buffer)
    }
    unsafe fn glIsEnabled(&self, _cap: GLenum) -> GLboolean {
        (self.entry().glIsEnabled)(_cap)
    }
    unsafe fn glIsList(&self, _list: GLuint) -> GLboolean {
        (self.entry().glIsList)(_list)
    }
    unsafe fn glIsProgram(&self, _program: GLuint) -> GLboolean {
        (self.entry().glIsProgram)(_program)
    }
    unsafe fn glIsQuery(&self, _id: GLuint) -> GLboolean {
        (self.entry().glIsQuery)(_id)
    }
    unsafe fn glIsShader(&self, _shader: GLuint) -> GLboolean {
        (self.entry().glIsShader)(_shader)
    }
    unsafe fn glIsTexture(&self, _texture: GLuint) -> GLboolean {
        (self.entry().glIsTexture)(_texture)
    }
    unsafe fn glLightModelf(&self, _pname: GLenum, _param: GLfloat) {
        (self.entry().glLightModelf)(_pname, _param)
    }
    unsafe fn glLightModelfv(&self, _pname: GLenum, _params: *const GLfloat) {
        (self.entry().glLightModelfv)(_pname, _params)
    }
    unsafe fn glLightModeli(&self, _pname: GLenum, _param: GLint) {
        (self.entry().glLightModeli)(_pname, _param)
    }
    unsafe fn glLightModeliv(&self, _pname: GLenum, _params: *const GLint) {
        (self.entry().glLightModeliv)(_pname, _params)
    }
    unsafe fn glLightf(&self, _light: GLenum, _pname: GLenum, _param: GLfloat) {
        (self.entry().glLightf)(_light, _pname, _param)
    }
    unsafe fn glLightfv(&self, _light: GLenum, _pname: GLenum, _params: *const GLfloat) {
        (self.entry().glLightfv)(_light, _pname, _params)
    }
    unsafe fn glLighti(&self, _light: GLenum, _pname: GLenum, _param: GLint) {
        (self.entry().glLighti)(_light, _pname, _param)
    }
    unsafe fn glLightiv(&self, _light: GLenum, _pname: GLenum, _params: *const GLint) {
        (self.entry().glLightiv)(_light, _pname, _params)
    }
    unsafe fn glLineStipple(&self, _factor: GLint, _pattern: GLushort) {
        (self.entry().glLineStipple)(_factor, _pattern)
    }
    unsafe fn glLineWidth(&self, _width: GLfloat) {
        (self.entry().glLineWidth)(_width)
    }
    unsafe fn glLinkProgram(&self, _program: GLuint) {
        (self.entry().glLinkProgram)(_program)
    }
    unsafe fn glListBase(&self, _base: GLuint) {
        (self.entry().glListBase)(_base)
    }
    unsafe fn glLoadIdentity(&self) {
        (self.entry().glLoadIdentity)()
    }
    unsafe fn glLoadMatrixd(&self, _m: *const GLdouble) {
        (self.entry().glLoadMatrixd)(_m)
    }
    unsafe fn glLoadMatrixf(&self, _m: *const GLfloat) {
        (self.entry().glLoadMatrixf)(_m)
    }
    unsafe fn glLoadName(&self, _name: GLuint) {
        (self.entry().glLoadName)(_name)
    }
    unsafe fn glLoadTransposeMatrixd(&self, _m: *const GLdouble) {
        (self.entry().glLoadTransposeMatrixd)(_m)
    }
    unsafe fn glLoadTransposeMatrixf(&self, _m: *const GLfloat) {
        (self.entry().glLoadTransposeMatrixf)(_m)
    }
    unsafe fn glLogicOp(&self, _opcode: GLenum) {
        (self.entry().glLogicOp)(_opcode)
    }
    unsafe fn glMap1d(
        &self,
        _target: GLenum,
        _u1: GLdouble,
        _u2: GLdouble,
        _stride: GLint,
        _order: GLint,
        _points: *const GLdouble,
    ) {
        (self.entry().glMap1d)(_target, _u1, _u2, _stride, _order, _points)
    }
    unsafe fn glMap1f(
        &self,
        _target: GLenum,
        _u1: GLfloat,
        _u2: GLfloat,
        _stride: GLint,
        _order: GLint,
        _points: *const GLfloat,
    ) {
        (self.entry().glMap1f)(_target, _u1, _u2, _stride, _order, _points)
    }
    unsafe fn glMap2d(
        &self,
        _target: GLenum,
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
        (self.entry().glMap2d)(
            _target, _u1, _u2, _ustride, _uorder, _v1, _v2, _vstride, _vorder, _points,
        )
    }
    unsafe fn glMap2f(
        &self,
        _target: GLenum,
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
        (self.entry().glMap2f)(
            _target, _u1, _u2, _ustride, _uorder, _v1, _v2, _vstride, _vorder, _points,
        )
    }
    unsafe fn glMapBuffer(&self, _target: GLenum, _access: GLenum) -> *mut std::os::raw::c_void {
        (self.entry().glMapBuffer)(_target, _access)
    }
    unsafe fn glMapGrid1d(&self, _un: GLint, _u1: GLdouble, _u2: GLdouble) {
        (self.entry().glMapGrid1d)(_un, _u1, _u2)
    }
    unsafe fn glMapGrid1f(&self, _un: GLint, _u1: GLfloat, _u2: GLfloat) {
        (self.entry().glMapGrid1f)(_un, _u1, _u2)
    }
    unsafe fn glMapGrid2d(
        &self,
        _un: GLint,
        _u1: GLdouble,
        _u2: GLdouble,
        _vn: GLint,
        _v1: GLdouble,
        _v2: GLdouble,
    ) {
        (self.entry().glMapGrid2d)(_un, _u1, _u2, _vn, _v1, _v2)
    }
    unsafe fn glMapGrid2f(
        &self,
        _un: GLint,
        _u1: GLfloat,
        _u2: GLfloat,
        _vn: GLint,
        _v1: GLfloat,
        _v2: GLfloat,
    ) {
        (self.entry().glMapGrid2f)(_un, _u1, _u2, _vn, _v1, _v2)
    }
    unsafe fn glMaterialf(&self, _face: GLenum, _pname: GLenum, _param: GLfloat) {
        (self.entry().glMaterialf)(_face, _pname, _param)
    }
    unsafe fn glMaterialfv(&self, _face: GLenum, _pname: GLenum, _params: *const GLfloat) {
        (self.entry().glMaterialfv)(_face, _pname, _params)
    }
    unsafe fn glMateriali(&self, _face: GLenum, _pname: GLenum, _param: GLint) {
        (self.entry().glMateriali)(_face, _pname, _param)
    }
    unsafe fn glMaterialiv(&self, _face: GLenum, _pname: GLenum, _params: *const GLint) {
        (self.entry().glMaterialiv)(_face, _pname, _params)
    }
    unsafe fn glMatrixMode(&self, _mode: GLenum) {
        (self.entry().glMatrixMode)(_mode)
    }
    unsafe fn glMultMatrixd(&self, _m: *const GLdouble) {
        (self.entry().glMultMatrixd)(_m)
    }
    unsafe fn glMultMatrixf(&self, _m: *const GLfloat) {
        (self.entry().glMultMatrixf)(_m)
    }
    unsafe fn glMultTransposeMatrixd(&self, _m: *const GLdouble) {
        (self.entry().glMultTransposeMatrixd)(_m)
    }
    unsafe fn glMultTransposeMatrixf(&self, _m: *const GLfloat) {
        (self.entry().glMultTransposeMatrixf)(_m)
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
    unsafe fn glMultiTexCoord1d(&self, _target: GLenum, _s: GLdouble) {
        (self.entry().glMultiTexCoord1d)(_target, _s)
    }
    unsafe fn glMultiTexCoord1dv(&self, _target: GLenum, _v: *const GLdouble) {
        (self.entry().glMultiTexCoord1dv)(_target, _v)
    }
    unsafe fn glMultiTexCoord1f(&self, _target: GLenum, _s: GLfloat) {
        (self.entry().glMultiTexCoord1f)(_target, _s)
    }
    unsafe fn glMultiTexCoord1fv(&self, _target: GLenum, _v: *const GLfloat) {
        (self.entry().glMultiTexCoord1fv)(_target, _v)
    }
    unsafe fn glMultiTexCoord1i(&self, _target: GLenum, _s: GLint) {
        (self.entry().glMultiTexCoord1i)(_target, _s)
    }
    unsafe fn glMultiTexCoord1iv(&self, _target: GLenum, _v: *const GLint) {
        (self.entry().glMultiTexCoord1iv)(_target, _v)
    }
    unsafe fn glMultiTexCoord1s(&self, _target: GLenum, _s: GLshort) {
        (self.entry().glMultiTexCoord1s)(_target, _s)
    }
    unsafe fn glMultiTexCoord1sv(&self, _target: GLenum, _v: *const GLshort) {
        (self.entry().glMultiTexCoord1sv)(_target, _v)
    }
    unsafe fn glMultiTexCoord2d(&self, _target: GLenum, _s: GLdouble, _t: GLdouble) {
        (self.entry().glMultiTexCoord2d)(_target, _s, _t)
    }
    unsafe fn glMultiTexCoord2dv(&self, _target: GLenum, _v: *const GLdouble) {
        (self.entry().glMultiTexCoord2dv)(_target, _v)
    }
    unsafe fn glMultiTexCoord2f(&self, _target: GLenum, _s: GLfloat, _t: GLfloat) {
        (self.entry().glMultiTexCoord2f)(_target, _s, _t)
    }
    unsafe fn glMultiTexCoord2fv(&self, _target: GLenum, _v: *const GLfloat) {
        (self.entry().glMultiTexCoord2fv)(_target, _v)
    }
    unsafe fn glMultiTexCoord2i(&self, _target: GLenum, _s: GLint, _t: GLint) {
        (self.entry().glMultiTexCoord2i)(_target, _s, _t)
    }
    unsafe fn glMultiTexCoord2iv(&self, _target: GLenum, _v: *const GLint) {
        (self.entry().glMultiTexCoord2iv)(_target, _v)
    }
    unsafe fn glMultiTexCoord2s(&self, _target: GLenum, _s: GLshort, _t: GLshort) {
        (self.entry().glMultiTexCoord2s)(_target, _s, _t)
    }
    unsafe fn glMultiTexCoord2sv(&self, _target: GLenum, _v: *const GLshort) {
        (self.entry().glMultiTexCoord2sv)(_target, _v)
    }
    unsafe fn glMultiTexCoord3d(&self, _target: GLenum, _s: GLdouble, _t: GLdouble, _r: GLdouble) {
        (self.entry().glMultiTexCoord3d)(_target, _s, _t, _r)
    }
    unsafe fn glMultiTexCoord3dv(&self, _target: GLenum, _v: *const GLdouble) {
        (self.entry().glMultiTexCoord3dv)(_target, _v)
    }
    unsafe fn glMultiTexCoord3f(&self, _target: GLenum, _s: GLfloat, _t: GLfloat, _r: GLfloat) {
        (self.entry().glMultiTexCoord3f)(_target, _s, _t, _r)
    }
    unsafe fn glMultiTexCoord3fv(&self, _target: GLenum, _v: *const GLfloat) {
        (self.entry().glMultiTexCoord3fv)(_target, _v)
    }
    unsafe fn glMultiTexCoord3i(&self, _target: GLenum, _s: GLint, _t: GLint, _r: GLint) {
        (self.entry().glMultiTexCoord3i)(_target, _s, _t, _r)
    }
    unsafe fn glMultiTexCoord3iv(&self, _target: GLenum, _v: *const GLint) {
        (self.entry().glMultiTexCoord3iv)(_target, _v)
    }
    unsafe fn glMultiTexCoord3s(&self, _target: GLenum, _s: GLshort, _t: GLshort, _r: GLshort) {
        (self.entry().glMultiTexCoord3s)(_target, _s, _t, _r)
    }
    unsafe fn glMultiTexCoord3sv(&self, _target: GLenum, _v: *const GLshort) {
        (self.entry().glMultiTexCoord3sv)(_target, _v)
    }
    unsafe fn glMultiTexCoord4d(
        &self,
        _target: GLenum,
        _s: GLdouble,
        _t: GLdouble,
        _r: GLdouble,
        _q: GLdouble,
    ) {
        (self.entry().glMultiTexCoord4d)(_target, _s, _t, _r, _q)
    }
    unsafe fn glMultiTexCoord4dv(&self, _target: GLenum, _v: *const GLdouble) {
        (self.entry().glMultiTexCoord4dv)(_target, _v)
    }
    unsafe fn glMultiTexCoord4f(
        &self,
        _target: GLenum,
        _s: GLfloat,
        _t: GLfloat,
        _r: GLfloat,
        _q: GLfloat,
    ) {
        (self.entry().glMultiTexCoord4f)(_target, _s, _t, _r, _q)
    }
    unsafe fn glMultiTexCoord4fv(&self, _target: GLenum, _v: *const GLfloat) {
        (self.entry().glMultiTexCoord4fv)(_target, _v)
    }
    unsafe fn glMultiTexCoord4i(
        &self,
        _target: GLenum,
        _s: GLint,
        _t: GLint,
        _r: GLint,
        _q: GLint,
    ) {
        (self.entry().glMultiTexCoord4i)(_target, _s, _t, _r, _q)
    }
    unsafe fn glMultiTexCoord4iv(&self, _target: GLenum, _v: *const GLint) {
        (self.entry().glMultiTexCoord4iv)(_target, _v)
    }
    unsafe fn glMultiTexCoord4s(
        &self,
        _target: GLenum,
        _s: GLshort,
        _t: GLshort,
        _r: GLshort,
        _q: GLshort,
    ) {
        (self.entry().glMultiTexCoord4s)(_target, _s, _t, _r, _q)
    }
    unsafe fn glMultiTexCoord4sv(&self, _target: GLenum, _v: *const GLshort) {
        (self.entry().glMultiTexCoord4sv)(_target, _v)
    }
    unsafe fn glNewList(&self, _list: GLuint, _mode: GLenum) {
        (self.entry().glNewList)(_list, _mode)
    }
    unsafe fn glNormal3b(&self, _nx: GLbyte, _ny: GLbyte, _nz: GLbyte) {
        (self.entry().glNormal3b)(_nx, _ny, _nz)
    }
    unsafe fn glNormal3bv(&self, _v: *const GLbyte) {
        (self.entry().glNormal3bv)(_v)
    }
    unsafe fn glNormal3d(&self, _nx: GLdouble, _ny: GLdouble, _nz: GLdouble) {
        (self.entry().glNormal3d)(_nx, _ny, _nz)
    }
    unsafe fn glNormal3dv(&self, _v: *const GLdouble) {
        (self.entry().glNormal3dv)(_v)
    }
    unsafe fn glNormal3f(&self, _nx: GLfloat, _ny: GLfloat, _nz: GLfloat) {
        (self.entry().glNormal3f)(_nx, _ny, _nz)
    }
    unsafe fn glNormal3fv(&self, _v: *const GLfloat) {
        (self.entry().glNormal3fv)(_v)
    }
    unsafe fn glNormal3i(&self, _nx: GLint, _ny: GLint, _nz: GLint) {
        (self.entry().glNormal3i)(_nx, _ny, _nz)
    }
    unsafe fn glNormal3iv(&self, _v: *const GLint) {
        (self.entry().glNormal3iv)(_v)
    }
    unsafe fn glNormal3s(&self, _nx: GLshort, _ny: GLshort, _nz: GLshort) {
        (self.entry().glNormal3s)(_nx, _ny, _nz)
    }
    unsafe fn glNormal3sv(&self, _v: *const GLshort) {
        (self.entry().glNormal3sv)(_v)
    }
    unsafe fn glNormalPointer(
        &self,
        _type: GLenum,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glNormalPointer)(_type, _stride, _pointer)
    }
    unsafe fn glOrtho(
        &self,
        _left: GLdouble,
        _right: GLdouble,
        _bottom: GLdouble,
        _top: GLdouble,
        _zNear: GLdouble,
        _zFar: GLdouble,
    ) {
        (self.entry().glOrtho)(_left, _right, _bottom, _top, _zNear, _zFar)
    }
    unsafe fn glPassThrough(&self, _token: GLfloat) {
        (self.entry().glPassThrough)(_token)
    }
    unsafe fn glPixelMapfv(&self, _map: GLenum, _mapsize: GLsizei, _values: *const GLfloat) {
        (self.entry().glPixelMapfv)(_map, _mapsize, _values)
    }
    unsafe fn glPixelMapuiv(&self, _map: GLenum, _mapsize: GLsizei, _values: *const GLuint) {
        (self.entry().glPixelMapuiv)(_map, _mapsize, _values)
    }
    unsafe fn glPixelMapusv(&self, _map: GLenum, _mapsize: GLsizei, _values: *const GLushort) {
        (self.entry().glPixelMapusv)(_map, _mapsize, _values)
    }
    unsafe fn glPixelStoref(&self, _pname: GLenum, _param: GLfloat) {
        (self.entry().glPixelStoref)(_pname, _param)
    }
    unsafe fn glPixelStorei(&self, _pname: GLenum, _param: GLint) {
        (self.entry().glPixelStorei)(_pname, _param)
    }
    unsafe fn glPixelTransferf(&self, _pname: GLenum, _param: GLfloat) {
        (self.entry().glPixelTransferf)(_pname, _param)
    }
    unsafe fn glPixelTransferi(&self, _pname: GLenum, _param: GLint) {
        (self.entry().glPixelTransferi)(_pname, _param)
    }
    unsafe fn glPixelZoom(&self, _xfactor: GLfloat, _yfactor: GLfloat) {
        (self.entry().glPixelZoom)(_xfactor, _yfactor)
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
    unsafe fn glPolygonStipple(&self, _mask: *const GLubyte) {
        (self.entry().glPolygonStipple)(_mask)
    }
    unsafe fn glPopAttrib(&self) {
        (self.entry().glPopAttrib)()
    }
    unsafe fn glPopClientAttrib(&self) {
        (self.entry().glPopClientAttrib)()
    }
    unsafe fn glPopMatrix(&self) {
        (self.entry().glPopMatrix)()
    }
    unsafe fn glPopName(&self) {
        (self.entry().glPopName)()
    }
    unsafe fn glPrioritizeTextures(
        &self,
        _n: GLsizei,
        _textures: *const GLuint,
        _priorities: *const GLfloat,
    ) {
        (self.entry().glPrioritizeTextures)(_n, _textures, _priorities)
    }
    unsafe fn glPushAttrib(&self, _mask: GLbitfield) {
        (self.entry().glPushAttrib)(_mask)
    }
    unsafe fn glPushClientAttrib(&self, _mask: GLbitfield) {
        (self.entry().glPushClientAttrib)(_mask)
    }
    unsafe fn glPushMatrix(&self) {
        (self.entry().glPushMatrix)()
    }
    unsafe fn glPushName(&self, _name: GLuint) {
        (self.entry().glPushName)(_name)
    }
    unsafe fn glRasterPos2d(&self, _x: GLdouble, _y: GLdouble) {
        (self.entry().glRasterPos2d)(_x, _y)
    }
    unsafe fn glRasterPos2dv(&self, _v: *const GLdouble) {
        (self.entry().glRasterPos2dv)(_v)
    }
    unsafe fn glRasterPos2f(&self, _x: GLfloat, _y: GLfloat) {
        (self.entry().glRasterPos2f)(_x, _y)
    }
    unsafe fn glRasterPos2fv(&self, _v: *const GLfloat) {
        (self.entry().glRasterPos2fv)(_v)
    }
    unsafe fn glRasterPos2i(&self, _x: GLint, _y: GLint) {
        (self.entry().glRasterPos2i)(_x, _y)
    }
    unsafe fn glRasterPos2iv(&self, _v: *const GLint) {
        (self.entry().glRasterPos2iv)(_v)
    }
    unsafe fn glRasterPos2s(&self, _x: GLshort, _y: GLshort) {
        (self.entry().glRasterPos2s)(_x, _y)
    }
    unsafe fn glRasterPos2sv(&self, _v: *const GLshort) {
        (self.entry().glRasterPos2sv)(_v)
    }
    unsafe fn glRasterPos3d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glRasterPos3d)(_x, _y, _z)
    }
    unsafe fn glRasterPos3dv(&self, _v: *const GLdouble) {
        (self.entry().glRasterPos3dv)(_v)
    }
    unsafe fn glRasterPos3f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glRasterPos3f)(_x, _y, _z)
    }
    unsafe fn glRasterPos3fv(&self, _v: *const GLfloat) {
        (self.entry().glRasterPos3fv)(_v)
    }
    unsafe fn glRasterPos3i(&self, _x: GLint, _y: GLint, _z: GLint) {
        (self.entry().glRasterPos3i)(_x, _y, _z)
    }
    unsafe fn glRasterPos3iv(&self, _v: *const GLint) {
        (self.entry().glRasterPos3iv)(_v)
    }
    unsafe fn glRasterPos3s(&self, _x: GLshort, _y: GLshort, _z: GLshort) {
        (self.entry().glRasterPos3s)(_x, _y, _z)
    }
    unsafe fn glRasterPos3sv(&self, _v: *const GLshort) {
        (self.entry().glRasterPos3sv)(_v)
    }
    unsafe fn glRasterPos4d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble, _w: GLdouble) {
        (self.entry().glRasterPos4d)(_x, _y, _z, _w)
    }
    unsafe fn glRasterPos4dv(&self, _v: *const GLdouble) {
        (self.entry().glRasterPos4dv)(_v)
    }
    unsafe fn glRasterPos4f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat, _w: GLfloat) {
        (self.entry().glRasterPos4f)(_x, _y, _z, _w)
    }
    unsafe fn glRasterPos4fv(&self, _v: *const GLfloat) {
        (self.entry().glRasterPos4fv)(_v)
    }
    unsafe fn glRasterPos4i(&self, _x: GLint, _y: GLint, _z: GLint, _w: GLint) {
        (self.entry().glRasterPos4i)(_x, _y, _z, _w)
    }
    unsafe fn glRasterPos4iv(&self, _v: *const GLint) {
        (self.entry().glRasterPos4iv)(_v)
    }
    unsafe fn glRasterPos4s(&self, _x: GLshort, _y: GLshort, _z: GLshort, _w: GLshort) {
        (self.entry().glRasterPos4s)(_x, _y, _z, _w)
    }
    unsafe fn glRasterPos4sv(&self, _v: *const GLshort) {
        (self.entry().glRasterPos4sv)(_v)
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
    unsafe fn glRectd(&self, _x1: GLdouble, _y1: GLdouble, _x2: GLdouble, _y2: GLdouble) {
        (self.entry().glRectd)(_x1, _y1, _x2, _y2)
    }
    unsafe fn glRectdv(&self, _v1: *const GLdouble, _v2: *const GLdouble) {
        (self.entry().glRectdv)(_v1, _v2)
    }
    unsafe fn glRectf(&self, _x1: GLfloat, _y1: GLfloat, _x2: GLfloat, _y2: GLfloat) {
        (self.entry().glRectf)(_x1, _y1, _x2, _y2)
    }
    unsafe fn glRectfv(&self, _v1: *const GLfloat, _v2: *const GLfloat) {
        (self.entry().glRectfv)(_v1, _v2)
    }
    unsafe fn glRecti(&self, _x1: GLint, _y1: GLint, _x2: GLint, _y2: GLint) {
        (self.entry().glRecti)(_x1, _y1, _x2, _y2)
    }
    unsafe fn glRectiv(&self, _v1: *const GLint, _v2: *const GLint) {
        (self.entry().glRectiv)(_v1, _v2)
    }
    unsafe fn glRects(&self, _x1: GLshort, _y1: GLshort, _x2: GLshort, _y2: GLshort) {
        (self.entry().glRects)(_x1, _y1, _x2, _y2)
    }
    unsafe fn glRectsv(&self, _v1: *const GLshort, _v2: *const GLshort) {
        (self.entry().glRectsv)(_v1, _v2)
    }
    unsafe fn glRenderMode(&self, _mode: GLenum) -> GLint {
        (self.entry().glRenderMode)(_mode)
    }
    unsafe fn glRotated(&self, _angle: GLdouble, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glRotated)(_angle, _x, _y, _z)
    }
    unsafe fn glRotatef(&self, _angle: GLfloat, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glRotatef)(_angle, _x, _y, _z)
    }
    unsafe fn glSampleCoverage(&self, _value: GLfloat, _invert: GLboolean) {
        (self.entry().glSampleCoverage)(_value, _invert)
    }
    unsafe fn glScaled(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glScaled)(_x, _y, _z)
    }
    unsafe fn glScalef(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glScalef)(_x, _y, _z)
    }
    unsafe fn glScissor(&self, _x: GLint, _y: GLint, _width: GLsizei, _height: GLsizei) {
        (self.entry().glScissor)(_x, _y, _width, _height)
    }
    unsafe fn glSecondaryColor3b(&self, _red: GLbyte, _green: GLbyte, _blue: GLbyte) {
        (self.entry().glSecondaryColor3b)(_red, _green, _blue)
    }
    unsafe fn glSecondaryColor3bv(&self, _v: *const GLbyte) {
        (self.entry().glSecondaryColor3bv)(_v)
    }
    unsafe fn glSecondaryColor3d(&self, _red: GLdouble, _green: GLdouble, _blue: GLdouble) {
        (self.entry().glSecondaryColor3d)(_red, _green, _blue)
    }
    unsafe fn glSecondaryColor3dv(&self, _v: *const GLdouble) {
        (self.entry().glSecondaryColor3dv)(_v)
    }
    unsafe fn glSecondaryColor3f(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat) {
        (self.entry().glSecondaryColor3f)(_red, _green, _blue)
    }
    unsafe fn glSecondaryColor3fv(&self, _v: *const GLfloat) {
        (self.entry().glSecondaryColor3fv)(_v)
    }
    unsafe fn glSecondaryColor3i(&self, _red: GLint, _green: GLint, _blue: GLint) {
        (self.entry().glSecondaryColor3i)(_red, _green, _blue)
    }
    unsafe fn glSecondaryColor3iv(&self, _v: *const GLint) {
        (self.entry().glSecondaryColor3iv)(_v)
    }
    unsafe fn glSecondaryColor3s(&self, _red: GLshort, _green: GLshort, _blue: GLshort) {
        (self.entry().glSecondaryColor3s)(_red, _green, _blue)
    }
    unsafe fn glSecondaryColor3sv(&self, _v: *const GLshort) {
        (self.entry().glSecondaryColor3sv)(_v)
    }
    unsafe fn glSecondaryColor3ub(&self, _red: GLubyte, _green: GLubyte, _blue: GLubyte) {
        (self.entry().glSecondaryColor3ub)(_red, _green, _blue)
    }
    unsafe fn glSecondaryColor3ubv(&self, _v: *const GLubyte) {
        (self.entry().glSecondaryColor3ubv)(_v)
    }
    unsafe fn glSecondaryColor3ui(&self, _red: GLuint, _green: GLuint, _blue: GLuint) {
        (self.entry().glSecondaryColor3ui)(_red, _green, _blue)
    }
    unsafe fn glSecondaryColor3uiv(&self, _v: *const GLuint) {
        (self.entry().glSecondaryColor3uiv)(_v)
    }
    unsafe fn glSecondaryColor3us(&self, _red: GLushort, _green: GLushort, _blue: GLushort) {
        (self.entry().glSecondaryColor3us)(_red, _green, _blue)
    }
    unsafe fn glSecondaryColor3usv(&self, _v: *const GLushort) {
        (self.entry().glSecondaryColor3usv)(_v)
    }
    unsafe fn glSecondaryColorPointer(
        &self,
        _size: GLint,
        _type: GLenum,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glSecondaryColorPointer)(_size, _type, _stride, _pointer)
    }
    unsafe fn glSelectBuffer(&self, _size: GLsizei, _buffer: *mut GLuint) {
        (self.entry().glSelectBuffer)(_size, _buffer)
    }
    unsafe fn glShadeModel(&self, _mode: GLenum) {
        (self.entry().glShadeModel)(_mode)
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
    unsafe fn glTexCoord1d(&self, _s: GLdouble) {
        (self.entry().glTexCoord1d)(_s)
    }
    unsafe fn glTexCoord1dv(&self, _v: *const GLdouble) {
        (self.entry().glTexCoord1dv)(_v)
    }
    unsafe fn glTexCoord1f(&self, _s: GLfloat) {
        (self.entry().glTexCoord1f)(_s)
    }
    unsafe fn glTexCoord1fv(&self, _v: *const GLfloat) {
        (self.entry().glTexCoord1fv)(_v)
    }
    unsafe fn glTexCoord1i(&self, _s: GLint) {
        (self.entry().glTexCoord1i)(_s)
    }
    unsafe fn glTexCoord1iv(&self, _v: *const GLint) {
        (self.entry().glTexCoord1iv)(_v)
    }
    unsafe fn glTexCoord1s(&self, _s: GLshort) {
        (self.entry().glTexCoord1s)(_s)
    }
    unsafe fn glTexCoord1sv(&self, _v: *const GLshort) {
        (self.entry().glTexCoord1sv)(_v)
    }
    unsafe fn glTexCoord2d(&self, _s: GLdouble, _t: GLdouble) {
        (self.entry().glTexCoord2d)(_s, _t)
    }
    unsafe fn glTexCoord2dv(&self, _v: *const GLdouble) {
        (self.entry().glTexCoord2dv)(_v)
    }
    unsafe fn glTexCoord2f(&self, _s: GLfloat, _t: GLfloat) {
        (self.entry().glTexCoord2f)(_s, _t)
    }
    unsafe fn glTexCoord2fv(&self, _v: *const GLfloat) {
        (self.entry().glTexCoord2fv)(_v)
    }
    unsafe fn glTexCoord2i(&self, _s: GLint, _t: GLint) {
        (self.entry().glTexCoord2i)(_s, _t)
    }
    unsafe fn glTexCoord2iv(&self, _v: *const GLint) {
        (self.entry().glTexCoord2iv)(_v)
    }
    unsafe fn glTexCoord2s(&self, _s: GLshort, _t: GLshort) {
        (self.entry().glTexCoord2s)(_s, _t)
    }
    unsafe fn glTexCoord2sv(&self, _v: *const GLshort) {
        (self.entry().glTexCoord2sv)(_v)
    }
    unsafe fn glTexCoord3d(&self, _s: GLdouble, _t: GLdouble, _r: GLdouble) {
        (self.entry().glTexCoord3d)(_s, _t, _r)
    }
    unsafe fn glTexCoord3dv(&self, _v: *const GLdouble) {
        (self.entry().glTexCoord3dv)(_v)
    }
    unsafe fn glTexCoord3f(&self, _s: GLfloat, _t: GLfloat, _r: GLfloat) {
        (self.entry().glTexCoord3f)(_s, _t, _r)
    }
    unsafe fn glTexCoord3fv(&self, _v: *const GLfloat) {
        (self.entry().glTexCoord3fv)(_v)
    }
    unsafe fn glTexCoord3i(&self, _s: GLint, _t: GLint, _r: GLint) {
        (self.entry().glTexCoord3i)(_s, _t, _r)
    }
    unsafe fn glTexCoord3iv(&self, _v: *const GLint) {
        (self.entry().glTexCoord3iv)(_v)
    }
    unsafe fn glTexCoord3s(&self, _s: GLshort, _t: GLshort, _r: GLshort) {
        (self.entry().glTexCoord3s)(_s, _t, _r)
    }
    unsafe fn glTexCoord3sv(&self, _v: *const GLshort) {
        (self.entry().glTexCoord3sv)(_v)
    }
    unsafe fn glTexCoord4d(&self, _s: GLdouble, _t: GLdouble, _r: GLdouble, _q: GLdouble) {
        (self.entry().glTexCoord4d)(_s, _t, _r, _q)
    }
    unsafe fn glTexCoord4dv(&self, _v: *const GLdouble) {
        (self.entry().glTexCoord4dv)(_v)
    }
    unsafe fn glTexCoord4f(&self, _s: GLfloat, _t: GLfloat, _r: GLfloat, _q: GLfloat) {
        (self.entry().glTexCoord4f)(_s, _t, _r, _q)
    }
    unsafe fn glTexCoord4fv(&self, _v: *const GLfloat) {
        (self.entry().glTexCoord4fv)(_v)
    }
    unsafe fn glTexCoord4i(&self, _s: GLint, _t: GLint, _r: GLint, _q: GLint) {
        (self.entry().glTexCoord4i)(_s, _t, _r, _q)
    }
    unsafe fn glTexCoord4iv(&self, _v: *const GLint) {
        (self.entry().glTexCoord4iv)(_v)
    }
    unsafe fn glTexCoord4s(&self, _s: GLshort, _t: GLshort, _r: GLshort, _q: GLshort) {
        (self.entry().glTexCoord4s)(_s, _t, _r, _q)
    }
    unsafe fn glTexCoord4sv(&self, _v: *const GLshort) {
        (self.entry().glTexCoord4sv)(_v)
    }
    unsafe fn glTexCoordPointer(
        &self,
        _size: GLint,
        _type: GLenum,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glTexCoordPointer)(_size, _type, _stride, _pointer)
    }
    unsafe fn glTexEnvf(&self, _target: GLenum, _pname: GLenum, _param: GLfloat) {
        (self.entry().glTexEnvf)(_target, _pname, _param)
    }
    unsafe fn glTexEnvfv(&self, _target: GLenum, _pname: GLenum, _params: *const GLfloat) {
        (self.entry().glTexEnvfv)(_target, _pname, _params)
    }
    unsafe fn glTexEnvi(&self, _target: GLenum, _pname: GLenum, _param: GLint) {
        (self.entry().glTexEnvi)(_target, _pname, _param)
    }
    unsafe fn glTexEnviv(&self, _target: GLenum, _pname: GLenum, _params: *const GLint) {
        (self.entry().glTexEnviv)(_target, _pname, _params)
    }
    unsafe fn glTexGend(&self, _coord: GLenum, _pname: GLenum, _param: GLdouble) {
        (self.entry().glTexGend)(_coord, _pname, _param)
    }
    unsafe fn glTexGendv(&self, _coord: GLenum, _pname: GLenum, _params: *const GLdouble) {
        (self.entry().glTexGendv)(_coord, _pname, _params)
    }
    unsafe fn glTexGenf(&self, _coord: GLenum, _pname: GLenum, _param: GLfloat) {
        (self.entry().glTexGenf)(_coord, _pname, _param)
    }
    unsafe fn glTexGenfv(&self, _coord: GLenum, _pname: GLenum, _params: *const GLfloat) {
        (self.entry().glTexGenfv)(_coord, _pname, _params)
    }
    unsafe fn glTexGeni(&self, _coord: GLenum, _pname: GLenum, _param: GLint) {
        (self.entry().glTexGeni)(_coord, _pname, _param)
    }
    unsafe fn glTexGeniv(&self, _coord: GLenum, _pname: GLenum, _params: *const GLint) {
        (self.entry().glTexGeniv)(_coord, _pname, _params)
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
    unsafe fn glTranslated(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glTranslated)(_x, _y, _z)
    }
    unsafe fn glTranslatef(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glTranslatef)(_x, _y, _z)
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
    unsafe fn glValidateProgram(&self, _program: GLuint) {
        (self.entry().glValidateProgram)(_program)
    }
    unsafe fn glVertex2d(&self, _x: GLdouble, _y: GLdouble) {
        (self.entry().glVertex2d)(_x, _y)
    }
    unsafe fn glVertex2dv(&self, _v: *const GLdouble) {
        (self.entry().glVertex2dv)(_v)
    }
    unsafe fn glVertex2f(&self, _x: GLfloat, _y: GLfloat) {
        (self.entry().glVertex2f)(_x, _y)
    }
    unsafe fn glVertex2fv(&self, _v: *const GLfloat) {
        (self.entry().glVertex2fv)(_v)
    }
    unsafe fn glVertex2i(&self, _x: GLint, _y: GLint) {
        (self.entry().glVertex2i)(_x, _y)
    }
    unsafe fn glVertex2iv(&self, _v: *const GLint) {
        (self.entry().glVertex2iv)(_v)
    }
    unsafe fn glVertex2s(&self, _x: GLshort, _y: GLshort) {
        (self.entry().glVertex2s)(_x, _y)
    }
    unsafe fn glVertex2sv(&self, _v: *const GLshort) {
        (self.entry().glVertex2sv)(_v)
    }
    unsafe fn glVertex3d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glVertex3d)(_x, _y, _z)
    }
    unsafe fn glVertex3dv(&self, _v: *const GLdouble) {
        (self.entry().glVertex3dv)(_v)
    }
    unsafe fn glVertex3f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glVertex3f)(_x, _y, _z)
    }
    unsafe fn glVertex3fv(&self, _v: *const GLfloat) {
        (self.entry().glVertex3fv)(_v)
    }
    unsafe fn glVertex3i(&self, _x: GLint, _y: GLint, _z: GLint) {
        (self.entry().glVertex3i)(_x, _y, _z)
    }
    unsafe fn glVertex3iv(&self, _v: *const GLint) {
        (self.entry().glVertex3iv)(_v)
    }
    unsafe fn glVertex3s(&self, _x: GLshort, _y: GLshort, _z: GLshort) {
        (self.entry().glVertex3s)(_x, _y, _z)
    }
    unsafe fn glVertex3sv(&self, _v: *const GLshort) {
        (self.entry().glVertex3sv)(_v)
    }
    unsafe fn glVertex4d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble, _w: GLdouble) {
        (self.entry().glVertex4d)(_x, _y, _z, _w)
    }
    unsafe fn glVertex4dv(&self, _v: *const GLdouble) {
        (self.entry().glVertex4dv)(_v)
    }
    unsafe fn glVertex4f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat, _w: GLfloat) {
        (self.entry().glVertex4f)(_x, _y, _z, _w)
    }
    unsafe fn glVertex4fv(&self, _v: *const GLfloat) {
        (self.entry().glVertex4fv)(_v)
    }
    unsafe fn glVertex4i(&self, _x: GLint, _y: GLint, _z: GLint, _w: GLint) {
        (self.entry().glVertex4i)(_x, _y, _z, _w)
    }
    unsafe fn glVertex4iv(&self, _v: *const GLint) {
        (self.entry().glVertex4iv)(_v)
    }
    unsafe fn glVertex4s(&self, _x: GLshort, _y: GLshort, _z: GLshort, _w: GLshort) {
        (self.entry().glVertex4s)(_x, _y, _z, _w)
    }
    unsafe fn glVertex4sv(&self, _v: *const GLshort) {
        (self.entry().glVertex4sv)(_v)
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
    unsafe fn glVertexPointer(
        &self,
        _size: GLint,
        _type: GLenum,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glVertexPointer)(_size, _type, _stride, _pointer)
    }
    unsafe fn glViewport(&self, _x: GLint, _y: GLint, _width: GLsizei, _height: GLsizei) {
        (self.entry().glViewport)(_x, _y, _width, _height)
    }
    unsafe fn glWindowPos2d(&self, _x: GLdouble, _y: GLdouble) {
        (self.entry().glWindowPos2d)(_x, _y)
    }
    unsafe fn glWindowPos2dv(&self, _v: *const GLdouble) {
        (self.entry().glWindowPos2dv)(_v)
    }
    unsafe fn glWindowPos2f(&self, _x: GLfloat, _y: GLfloat) {
        (self.entry().glWindowPos2f)(_x, _y)
    }
    unsafe fn glWindowPos2fv(&self, _v: *const GLfloat) {
        (self.entry().glWindowPos2fv)(_v)
    }
    unsafe fn glWindowPos2i(&self, _x: GLint, _y: GLint) {
        (self.entry().glWindowPos2i)(_x, _y)
    }
    unsafe fn glWindowPos2iv(&self, _v: *const GLint) {
        (self.entry().glWindowPos2iv)(_v)
    }
    unsafe fn glWindowPos2s(&self, _x: GLshort, _y: GLshort) {
        (self.entry().glWindowPos2s)(_x, _y)
    }
    unsafe fn glWindowPos2sv(&self, _v: *const GLshort) {
        (self.entry().glWindowPos2sv)(_v)
    }
    unsafe fn glWindowPos3d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glWindowPos3d)(_x, _y, _z)
    }
    unsafe fn glWindowPos3dv(&self, _v: *const GLdouble) {
        (self.entry().glWindowPos3dv)(_v)
    }
    unsafe fn glWindowPos3f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glWindowPos3f)(_x, _y, _z)
    }
    unsafe fn glWindowPos3fv(&self, _v: *const GLfloat) {
        (self.entry().glWindowPos3fv)(_v)
    }
    unsafe fn glWindowPos3i(&self, _x: GLint, _y: GLint, _z: GLint) {
        (self.entry().glWindowPos3i)(_x, _y, _z)
    }
    unsafe fn glWindowPos3iv(&self, _v: *const GLint) {
        (self.entry().glWindowPos3iv)(_v)
    }
    unsafe fn glWindowPos3s(&self, _x: GLshort, _y: GLshort, _z: GLshort) {
        (self.entry().glWindowPos3s)(_x, _y, _z)
    }
    unsafe fn glWindowPos3sv(&self, _v: *const GLshort) {
        (self.entry().glWindowPos3sv)(_v)
    }
}
