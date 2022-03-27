use crate::gl;
use crate::gl::feature::EntryGLFn;
use crate::types::*;
use gl::bitflags::*;
use gl::enums::*;
use std::ffi::c_void;
use std::fmt;
pub trait GL31 {
    unsafe fn entry(&self) -> &EntryGLFn;
    unsafe fn glEnd(&self) {
        (self.entry().glEnd)()
    }
    unsafe fn glGetMapdv(&self, _target: MapTarget, _query: GetMapQuery, _v: *mut GLdouble) {
        (self.entry().glGetMapdv)(_target, _query, _v)
    }
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
    unsafe fn glColor3uiv(&self, _v: *const GLuint) {
        (self.entry().glColor3uiv)(_v)
    }
    unsafe fn glVertexAttribI4uiv(&self, _index: GLuint, _v: *const GLuint) {
        (self.entry().glVertexAttribI4uiv)(_index, _v)
    }
    unsafe fn glUniform1ui(&self, _location: GLint, _v0: GLuint) {
        (self.entry().glUniform1ui)(_location, _v0)
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
    unsafe fn glVertexAttrib1f(&self, _index: GLuint, _x: GLfloat) {
        (self.entry().glVertexAttrib1f)(_index, _x)
    }
    unsafe fn glVertexAttribI3iv(&self, _index: GLuint, _v: *const GLint) {
        (self.entry().glVertexAttribI3iv)(_index, _v)
    }
    unsafe fn glColor3iv(&self, _v: *const GLint) {
        (self.entry().glColor3iv)(_v)
    }
    unsafe fn glRasterPos3i(&self, _x: GLint, _y: GLint, _z: GLint) {
        (self.entry().glRasterPos3i)(_x, _y, _z)
    }
    unsafe fn glUniform1i(&self, _location: GLint, _v0: GLint) {
        (self.entry().glUniform1i)(_location, _v0)
    }
    unsafe fn glEvalCoord1f(&self, _u: GLfloat) {
        (self.entry().glEvalCoord1f)(_u)
    }
    unsafe fn glVertex2s(&self, _x: GLshort, _y: GLshort) {
        (self.entry().glVertex2s)(_x, _y)
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
    unsafe fn glBegin(&self, _mode: PrimitiveType) {
        (self.entry().glBegin)(_mode)
    }
    unsafe fn glGetClipPlane(&self, _plane: ClipPlaneName, _equation: *mut GLdouble) {
        (self.entry().glGetClipPlane)(_plane, _equation)
    }
    unsafe fn glWindowPos3fv(&self, _v: *const GLfloat) {
        (self.entry().glWindowPos3fv)(_v)
    }
    unsafe fn glInterleavedArrays(
        &self,
        _format: InterleavedArrayFormat,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glInterleavedArrays)(_format, _stride, _pointer)
    }
    unsafe fn glMaterialfv(
        &self,
        _face: MaterialFace,
        _pname: MaterialParameter,
        _params: *const GLfloat,
    ) {
        (self.entry().glMaterialfv)(_face, _pname, _params)
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
    unsafe fn glIndexiv(&self, _c: *const GLint) {
        (self.entry().glIndexiv)(_c)
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
    unsafe fn glSecondaryColor3i(&self, _red: GLint, _green: GLint, _blue: GLint) {
        (self.entry().glSecondaryColor3i)(_red, _green, _blue)
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
    unsafe fn glColor4f(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat, _alpha: GLfloat) {
        (self.entry().glColor4f)(_red, _green, _blue, _alpha)
    }
    unsafe fn glLightfv(&self, _light: LightName, _pname: LightParameter, _params: *const GLfloat) {
        (self.entry().glLightfv)(_light, _pname, _params)
    }
    unsafe fn glWindowPos2f(&self, _x: GLfloat, _y: GLfloat) {
        (self.entry().glWindowPos2f)(_x, _y)
    }
    unsafe fn glGetMaterialfv(
        &self,
        _face: MaterialFace,
        _pname: MaterialParameter,
        _params: *mut GLfloat,
    ) {
        (self.entry().glGetMaterialfv)(_face, _pname, _params)
    }
    unsafe fn glVertex3f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glVertex3f)(_x, _y, _z)
    }
    unsafe fn glVertexAttrib4Niv(&self, _index: GLuint, _v: *const GLint) {
        (self.entry().glVertexAttrib4Niv)(_index, _v)
    }
    unsafe fn glGetFloatv(&self, _pname: GetPName, _data: *mut GLfloat) {
        (self.entry().glGetFloatv)(_pname, _data)
    }
    unsafe fn glVertex3sv(&self, _v: *const GLshort) {
        (self.entry().glVertex3sv)(_v)
    }
    unsafe fn glGetLightfv(
        &self,
        _light: LightName,
        _pname: LightParameter,
        _params: *mut GLfloat,
    ) {
        (self.entry().glGetLightfv)(_light, _pname, _params)
    }
    unsafe fn glGetBooleanv(&self, _pname: GetPName, _data: *mut GLboolean) {
        (self.entry().glGetBooleanv)(_pname, _data)
    }
    unsafe fn glVertexAttribI4ubv(&self, _index: GLuint, _v: *const GLubyte) {
        (self.entry().glVertexAttribI4ubv)(_index, _v)
    }
    unsafe fn glColor3s(&self, _red: GLshort, _green: GLshort, _blue: GLshort) {
        (self.entry().glColor3s)(_red, _green, _blue)
    }
    unsafe fn glGetAttribLocation(&self, _program: GLuint, _name: *const GLchar) -> GLint {
        (self.entry().glGetAttribLocation)(_program, _name)
    }
    unsafe fn glMultiTexCoord4sv(&self, _target: TextureUnit, _v: *const GLshort) {
        (self.entry().glMultiTexCoord4sv)(_target, _v)
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
    unsafe fn glGetPixelMapusv(&self, _map: PixelMap, _values: *mut GLushort) {
        (self.entry().glGetPixelMapusv)(_map, _values)
    }
    unsafe fn glPopClientAttrib(&self) {
        (self.entry().glPopClientAttrib)()
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
    unsafe fn glRasterPos3sv(&self, _v: *const GLshort) {
        (self.entry().glRasterPos3sv)(_v)
    }
    unsafe fn glRectsv(&self, _v1: *const GLshort, _v2: *const GLshort) {
        (self.entry().glRectsv)(_v1, _v2)
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
    unsafe fn glRectd(&self, _x1: GLdouble, _y1: GLdouble, _x2: GLdouble, _y2: GLdouble) {
        (self.entry().glRectd)(_x1, _y1, _x2, _y2)
    }
    unsafe fn glTexCoord4s(&self, _s: GLshort, _t: GLshort, _r: GLshort, _q: GLshort) {
        (self.entry().glTexCoord4s)(_s, _t, _r, _q)
    }
    unsafe fn glRasterPos2iv(&self, _v: *const GLint) {
        (self.entry().glRasterPos2iv)(_v)
    }
    unsafe fn glColor3fv(&self, _v: *const GLfloat) {
        (self.entry().glColor3fv)(_v)
    }
    unsafe fn glIndexi(&self, _c: GLint) {
        (self.entry().glIndexi)(_c)
    }
    unsafe fn glVertexAttrib4iv(&self, _index: GLuint, _v: *const GLint) {
        (self.entry().glVertexAttrib4iv)(_index, _v)
    }
    unsafe fn glSecondaryColor3ub(&self, _red: GLubyte, _green: GLubyte, _blue: GLubyte) {
        (self.entry().glSecondaryColor3ub)(_red, _green, _blue)
    }
    unsafe fn glPolygonStipple(&self, _mask: *const GLubyte) {
        (self.entry().glPolygonStipple)(_mask)
    }
    unsafe fn glPushClientAttrib(&self, _mask: ClientAttribMask) {
        (self.entry().glPushClientAttrib)(_mask)
    }
    unsafe fn glNormal3i(&self, _nx: GLint, _ny: GLint, _nz: GLint) {
        (self.entry().glNormal3i)(_nx, _ny, _nz)
    }
    unsafe fn glTexGenf(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _param: GLfloat,
    ) {
        (self.entry().glTexGenf)(_coord, _pname, _param)
    }
    unsafe fn glTexGenfv(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _params: *const GLfloat,
    ) {
        (self.entry().glTexGenfv)(_coord, _pname, _params)
    }
    unsafe fn glLightModelf(&self, _pname: LightModelParameter, _param: GLfloat) {
        (self.entry().glLightModelf)(_pname, _param)
    }
    unsafe fn glTranslatef(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glTranslatef)(_x, _y, _z)
    }
    unsafe fn glColorPointer(
        &self,
        _size: GLint,
        _type: ColorPointerType,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glColorPointer)(_size, _type, _stride, _pointer)
    }
    unsafe fn glMultiTexCoord2d(&self, _target: TextureUnit, _s: GLdouble, _t: GLdouble) {
        (self.entry().glMultiTexCoord2d)(_target, _s, _t)
    }
    unsafe fn glEvalCoord2d(&self, _u: GLdouble, _v: GLdouble) {
        (self.entry().glEvalCoord2d)(_u, _v)
    }
    unsafe fn glWindowPos3iv(&self, _v: *const GLint) {
        (self.entry().glWindowPos3iv)(_v)
    }
    unsafe fn glRasterPos4i(&self, _x: GLint, _y: GLint, _z: GLint, _w: GLint) {
        (self.entry().glRasterPos4i)(_x, _y, _z, _w)
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
    unsafe fn glColor4ubv(&self, _v: *const GLubyte) {
        (self.entry().glColor4ubv)(_v)
    }
    unsafe fn glEnableClientState(&self, _array: EnableCap) {
        (self.entry().glEnableClientState)(_array)
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
    unsafe fn glMultiTexCoord3sv(&self, _target: TextureUnit, _v: *const GLshort) {
        (self.entry().glMultiTexCoord3sv)(_target, _v)
    }
    unsafe fn glMaterialf(&self, _face: MaterialFace, _pname: MaterialParameter, _param: GLfloat) {
        (self.entry().glMaterialf)(_face, _pname, _param)
    }
    unsafe fn glMultiTexCoord2iv(&self, _target: TextureUnit, _v: *const GLint) {
        (self.entry().glMultiTexCoord2iv)(_target, _v)
    }
    unsafe fn glMultiTexCoord1f(&self, _target: TextureUnit, _s: GLfloat) {
        (self.entry().glMultiTexCoord1f)(_target, _s)
    }
    unsafe fn glMultiTexCoord4iv(&self, _target: TextureUnit, _v: *const GLint) {
        (self.entry().glMultiTexCoord4iv)(_target, _v)
    }
    unsafe fn glPointParameteri(&self, _pname: PointParameterNameARB, _param: GLint) {
        (self.entry().glPointParameteri)(_pname, _param)
    }
    unsafe fn glSecondaryColor3fv(&self, _v: *const GLfloat) {
        (self.entry().glSecondaryColor3fv)(_v)
    }
    unsafe fn glVertexAttribI3ui(&self, _index: GLuint, _x: GLuint, _y: GLuint, _z: GLuint) {
        (self.entry().glVertexAttribI3ui)(_index, _x, _y, _z)
    }
    unsafe fn glIsRenderbuffer(&self, _renderbuffer: GLuint) -> GLboolean {
        (self.entry().glIsRenderbuffer)(_renderbuffer)
    }
    unsafe fn glVertex3s(&self, _x: GLshort, _y: GLshort, _z: GLshort) {
        (self.entry().glVertex3s)(_x, _y, _z)
    }
    unsafe fn glPixelMapusv(&self, _map: PixelMap, _mapsize: GLsizei, _values: *const GLushort) {
        (self.entry().glPixelMapusv)(_map, _mapsize, _values)
    }
    unsafe fn glVertexAttrib3sv(&self, _index: GLuint, _v: *const GLshort) {
        (self.entry().glVertexAttrib3sv)(_index, _v)
    }
    unsafe fn glTexCoord2f(&self, _s: GLfloat, _t: GLfloat) {
        (self.entry().glTexCoord2f)(_s, _t)
    }
    unsafe fn glVertexAttrib4fv(&self, _index: GLuint, _v: *const GLfloat) {
        (self.entry().glVertexAttrib4fv)(_index, _v)
    }
    unsafe fn glTexCoord2dv(&self, _v: *const GLdouble) {
        (self.entry().glTexCoord2dv)(_v)
    }
    unsafe fn glDepthRange(&self, _n: GLdouble, _f: GLdouble) {
        (self.entry().glDepthRange)(_n, _f)
    }
    unsafe fn glRasterPos2dv(&self, _v: *const GLdouble) {
        (self.entry().glRasterPos2dv)(_v)
    }
    unsafe fn glWindowPos3f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glWindowPos3f)(_x, _y, _z)
    }
    unsafe fn glLineWidth(&self, _width: GLfloat) {
        (self.entry().glLineWidth)(_width)
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
    unsafe fn glMultiTexCoord1i(&self, _target: TextureUnit, _s: GLint) {
        (self.entry().glMultiTexCoord1i)(_target, _s)
    }
    unsafe fn glViewport(&self, _x: GLint, _y: GLint, _width: GLsizei, _height: GLsizei) {
        (self.entry().glViewport)(_x, _y, _width, _height)
    }
    unsafe fn glMaterialiv(
        &self,
        _face: MaterialFace,
        _pname: MaterialParameter,
        _params: *const GLint,
    ) {
        (self.entry().glMaterialiv)(_face, _pname, _params)
    }
    unsafe fn glGetLightiv(&self, _light: LightName, _pname: LightParameter, _params: *mut GLint) {
        (self.entry().glGetLightiv)(_light, _pname, _params)
    }
    unsafe fn glPushAttrib(&self, _mask: AttribMask) {
        (self.entry().glPushAttrib)(_mask)
    }
    unsafe fn glVertexAttrib1sv(&self, _index: GLuint, _v: *const GLshort) {
        (self.entry().glVertexAttrib1sv)(_index, _v)
    }
    unsafe fn glVertex3iv(&self, _v: *const GLint) {
        (self.entry().glVertex3iv)(_v)
    }
    unsafe fn glGetVertexAttribdv(
        &self,
        _index: GLuint,
        _pname: VertexAttribPropertyARB,
        _params: *mut GLdouble,
    ) {
        (self.entry().glGetVertexAttribdv)(_index, _pname, _params)
    }
    unsafe fn glTexCoord3d(&self, _s: GLdouble, _t: GLdouble, _r: GLdouble) {
        (self.entry().glTexCoord3d)(_s, _t, _r)
    }
    unsafe fn glMultiTexCoord3s(
        &self,
        _target: TextureUnit,
        _s: GLshort,
        _t: GLshort,
        _r: GLshort,
    ) {
        (self.entry().glMultiTexCoord3s)(_target, _s, _t, _r)
    }
    unsafe fn glUniform2f(&self, _location: GLint, _v0: GLfloat, _v1: GLfloat) {
        (self.entry().glUniform2f)(_location, _v0, _v1)
    }
    unsafe fn glRasterPos3s(&self, _x: GLshort, _y: GLshort, _z: GLshort) {
        (self.entry().glRasterPos3s)(_x, _y, _z)
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
    unsafe fn glColor4d(
        &self,
        _red: GLdouble,
        _green: GLdouble,
        _blue: GLdouble,
        _alpha: GLdouble,
    ) {
        (self.entry().glColor4d)(_red, _green, _blue, _alpha)
    }
    unsafe fn glTexParameterIiv(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _params: *const GLint,
    ) {
        (self.entry().glTexParameterIiv)(_target, _pname, _params)
    }
    unsafe fn glClientActiveTexture(&self, _texture: TextureUnit) {
        (self.entry().glClientActiveTexture)(_texture)
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
    unsafe fn glMap2f(
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
        (self.entry().glMap2f)(
            _target, _u1, _u2, _ustride, _uorder, _v1, _v2, _vstride, _vorder, _points,
        )
    }
    unsafe fn glIsQuery(&self, _id: GLuint) -> GLboolean {
        (self.entry().glIsQuery)(_id)
    }
    unsafe fn glVertex3dv(&self, _v: *const GLdouble) {
        (self.entry().glVertex3dv)(_v)
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
    unsafe fn glMultiTexCoord2fv(&self, _target: TextureUnit, _v: *const GLfloat) {
        (self.entry().glMultiTexCoord2fv)(_target, _v)
    }
    unsafe fn glLightModelfv(&self, _pname: LightModelParameter, _params: *const GLfloat) {
        (self.entry().glLightModelfv)(_pname, _params)
    }
    unsafe fn glSecondaryColor3s(&self, _red: GLshort, _green: GLshort, _blue: GLshort) {
        (self.entry().glSecondaryColor3s)(_red, _green, _blue)
    }
    unsafe fn glTexCoord2iv(&self, _v: *const GLint) {
        (self.entry().glTexCoord2iv)(_v)
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
    unsafe fn glUniformMatrix4fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        (self.entry().glUniformMatrix4fv)(_location, _count, _transpose, _value)
    }
    unsafe fn glDepthFunc(&self, _func: DepthFunction) {
        (self.entry().glDepthFunc)(_func)
    }
    unsafe fn glMultiTexCoord3f(
        &self,
        _target: TextureUnit,
        _s: GLfloat,
        _t: GLfloat,
        _r: GLfloat,
    ) {
        (self.entry().glMultiTexCoord3f)(_target, _s, _t, _r)
    }
    unsafe fn glPixelMapuiv(&self, _map: PixelMap, _mapsize: GLsizei, _values: *const GLuint) {
        (self.entry().glPixelMapuiv)(_map, _mapsize, _values)
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
    unsafe fn glTexGeni(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _param: GLint,
    ) {
        (self.entry().glTexGeni)(_coord, _pname, _param)
    }
    unsafe fn glVertex2i(&self, _x: GLint, _y: GLint) {
        (self.entry().glVertex2i)(_x, _y)
    }
    unsafe fn glRasterPos2fv(&self, _v: *const GLfloat) {
        (self.entry().glRasterPos2fv)(_v)
    }
    unsafe fn glSecondaryColor3iv(&self, _v: *const GLint) {
        (self.entry().glSecondaryColor3iv)(_v)
    }
    unsafe fn glCreateShader(&self, _type: ShaderType) -> GLuint {
        (self.entry().glCreateShader)(_type)
    }
    unsafe fn glTexCoord1i(&self, _s: GLint) {
        (self.entry().glTexCoord1i)(_s)
    }
    unsafe fn glRasterPos3iv(&self, _v: *const GLint) {
        (self.entry().glRasterPos3iv)(_v)
    }
    unsafe fn glMultiTexCoord3d(
        &self,
        _target: TextureUnit,
        _s: GLdouble,
        _t: GLdouble,
        _r: GLdouble,
    ) {
        (self.entry().glMultiTexCoord3d)(_target, _s, _t, _r)
    }
    unsafe fn glWindowPos2iv(&self, _v: *const GLint) {
        (self.entry().glWindowPos2iv)(_v)
    }
    unsafe fn glTexParameterf(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _param: GLfloat,
    ) {
        (self.entry().glTexParameterf)(_target, _pname, _param)
    }
    unsafe fn glFogiv(&self, _pname: FogParameter, _params: *const GLint) {
        (self.entry().glFogiv)(_pname, _params)
    }
    unsafe fn glIndexub(&self, _c: GLubyte) {
        (self.entry().glIndexub)(_c)
    }
    unsafe fn glGetUniformuiv(&self, _program: GLuint, _location: GLint, _params: *mut GLuint) {
        (self.entry().glGetUniformuiv)(_program, _location, _params)
    }
    unsafe fn glStencilMaskSeparate(&self, _face: StencilFaceDirection, _mask: GLuint) {
        (self.entry().glStencilMaskSeparate)(_face, _mask)
    }
    unsafe fn glIsVertexArray(&self, _array: GLuint) -> GLboolean {
        (self.entry().glIsVertexArray)(_array)
    }
    unsafe fn glSecondaryColor3dv(&self, _v: *const GLdouble) {
        (self.entry().glSecondaryColor3dv)(_v)
    }
    unsafe fn glRasterPos4fv(&self, _v: *const GLfloat) {
        (self.entry().glRasterPos4fv)(_v)
    }
    unsafe fn glRasterPos4d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble, _w: GLdouble) {
        (self.entry().glRasterPos4d)(_x, _y, _z, _w)
    }
    unsafe fn glIndexPointer(
        &self,
        _type: IndexPointerType,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glIndexPointer)(_type, _stride, _pointer)
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
    unsafe fn glRectfv(&self, _v1: *const GLfloat, _v2: *const GLfloat) {
        (self.entry().glRectfv)(_v1, _v2)
    }
    unsafe fn glVertexAttrib2sv(&self, _index: GLuint, _v: *const GLshort) {
        (self.entry().glVertexAttrib2sv)(_index, _v)
    }
    unsafe fn glTexCoord2s(&self, _s: GLshort, _t: GLshort) {
        (self.entry().glTexCoord2s)(_s, _t)
    }
    unsafe fn glEvalPoint2(&self, _i: GLint, _j: GLint) {
        (self.entry().glEvalPoint2)(_i, _j)
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
    unsafe fn glGetShaderInfoLog(
        &self,
        _shader: GLuint,
        _bufSize: GLsizei,
        _length: *mut GLsizei,
        _infoLog: *mut GLchar,
    ) {
        (self.entry().glGetShaderInfoLog)(_shader, _bufSize, _length, _infoLog)
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
    unsafe fn glGetMaterialiv(
        &self,
        _face: MaterialFace,
        _pname: MaterialParameter,
        _params: *mut GLint,
    ) {
        (self.entry().glGetMaterialiv)(_face, _pname, _params)
    }
    unsafe fn glNormal3s(&self, _nx: GLshort, _ny: GLshort, _nz: GLshort) {
        (self.entry().glNormal3s)(_nx, _ny, _nz)
    }
    unsafe fn glColor3ui(&self, _red: GLuint, _green: GLuint, _blue: GLuint) {
        (self.entry().glColor3ui)(_red, _green, _blue)
    }
    unsafe fn glBlendColor(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat, _alpha: GLfloat) {
        (self.entry().glBlendColor)(_red, _green, _blue, _alpha)
    }
    unsafe fn glPixelZoom(&self, _xfactor: GLfloat, _yfactor: GLfloat) {
        (self.entry().glPixelZoom)(_xfactor, _yfactor)
    }
    unsafe fn glStencilOp(&self, _fail: StencilOp, _zfail: StencilOp, _zpass: StencilOp) {
        (self.entry().glStencilOp)(_fail, _zfail, _zpass)
    }
    unsafe fn glMultiTexCoord2dv(&self, _target: TextureUnit, _v: *const GLdouble) {
        (self.entry().glMultiTexCoord2dv)(_target, _v)
    }
    unsafe fn glWindowPos2i(&self, _x: GLint, _y: GLint) {
        (self.entry().glWindowPos2i)(_x, _y)
    }
    unsafe fn glClampColor(&self, _target: ClampColorTargetARB, _clamp: ClampColorModeARB) {
        (self.entry().glClampColor)(_target, _clamp)
    }
    unsafe fn glGetPolygonStipple(&self, _mask: *mut GLubyte) {
        (self.entry().glGetPolygonStipple)(_mask)
    }
    unsafe fn glSecondaryColor3f(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat) {
        (self.entry().glSecondaryColor3f)(_red, _green, _blue)
    }
    unsafe fn glTexEnvi(
        &self,
        _target: TextureEnvTarget,
        _pname: TextureEnvParameter,
        _param: GLint,
    ) {
        (self.entry().glTexEnvi)(_target, _pname, _param)
    }
    unsafe fn glPushName(&self, _name: GLuint) {
        (self.entry().glPushName)(_name)
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
    unsafe fn glDeleteLists(&self, _list: GLuint, _range: GLsizei) {
        (self.entry().glDeleteLists)(_list, _range)
    }
    unsafe fn glBeginTransformFeedback(&self, _primitiveMode: PrimitiveType) {
        (self.entry().glBeginTransformFeedback)(_primitiveMode)
    }
    unsafe fn glRasterPos4s(&self, _x: GLshort, _y: GLshort, _z: GLshort, _w: GLshort) {
        (self.entry().glRasterPos4s)(_x, _y, _z, _w)
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
    unsafe fn glGetVertexAttribIiv(
        &self,
        _index: GLuint,
        _pname: VertexAttribEnum,
        _params: *mut GLint,
    ) {
        (self.entry().glGetVertexAttribIiv)(_index, _pname, _params)
    }
    unsafe fn glIndexs(&self, _c: GLshort) {
        (self.entry().glIndexs)(_c)
    }
    unsafe fn glDisable(&self, _cap: EnableCap) {
        (self.entry().glDisable)(_cap)
    }
    unsafe fn glTexCoord3i(&self, _s: GLint, _t: GLint, _r: GLint) {
        (self.entry().glTexCoord3i)(_s, _t, _r)
    }
    unsafe fn glUnmapBuffer(&self, _target: BufferTargetARB) -> GLboolean {
        (self.entry().glUnmapBuffer)(_target)
    }
    unsafe fn glFogCoordf(&self, _coord: GLfloat) {
        (self.entry().glFogCoordf)(_coord)
    }
    unsafe fn glUniform3fv(&self, _location: GLint, _count: GLsizei, _value: *const GLfloat) {
        (self.entry().glUniform3fv)(_location, _count, _value)
    }
    unsafe fn glTexEnviv(
        &self,
        _target: TextureEnvTarget,
        _pname: TextureEnvParameter,
        _params: *const GLint,
    ) {
        (self.entry().glTexEnviv)(_target, _pname, _params)
    }
    unsafe fn glPassThrough(&self, _token: GLfloat) {
        (self.entry().glPassThrough)(_token)
    }
    unsafe fn glEvalCoord1fv(&self, _u: *const GLfloat) {
        (self.entry().glEvalCoord1fv)(_u)
    }
    unsafe fn glColor4usv(&self, _v: *const GLushort) {
        (self.entry().glColor4usv)(_v)
    }
    unsafe fn glClearIndex(&self, _c: GLfloat) {
        (self.entry().glClearIndex)(_c)
    }
    unsafe fn glEvalPoint1(&self, _i: GLint) {
        (self.entry().glEvalPoint1)(_i)
    }
    unsafe fn glGetPixelMapfv(&self, _map: PixelMap, _values: *mut GLfloat) {
        (self.entry().glGetPixelMapfv)(_map, _values)
    }
    unsafe fn glRasterPos3f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glRasterPos3f)(_x, _y, _z)
    }
    unsafe fn glIndexdv(&self, _c: *const GLdouble) {
        (self.entry().glIndexdv)(_c)
    }
    unsafe fn glTexParameteriv(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _params: *const GLint,
    ) {
        (self.entry().glTexParameteriv)(_target, _pname, _params)
    }
    unsafe fn glClearAccum(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat, _alpha: GLfloat) {
        (self.entry().glClearAccum)(_red, _green, _blue, _alpha)
    }
    unsafe fn glNormal3dv(&self, _v: *const GLdouble) {
        (self.entry().glNormal3dv)(_v)
    }
    unsafe fn glSecondaryColor3b(&self, _red: GLbyte, _green: GLbyte, _blue: GLbyte) {
        (self.entry().glSecondaryColor3b)(_red, _green, _blue)
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
    unsafe fn glEndQuery(&self, _target: QueryTarget) {
        (self.entry().glEndQuery)(_target)
    }
    unsafe fn glInitNames(&self) {
        (self.entry().glInitNames)()
    }
    unsafe fn glSecondaryColor3us(&self, _red: GLushort, _green: GLushort, _blue: GLushort) {
        (self.entry().glSecondaryColor3us)(_red, _green, _blue)
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
    unsafe fn glPopMatrix(&self) {
        (self.entry().glPopMatrix)()
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
    unsafe fn glTexCoord3s(&self, _s: GLshort, _t: GLshort, _r: GLshort) {
        (self.entry().glTexCoord3s)(_s, _t, _r)
    }
    unsafe fn glSecondaryColor3bv(&self, _v: *const GLbyte) {
        (self.entry().glSecondaryColor3bv)(_v)
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
    unsafe fn glPopAttrib(&self) {
        (self.entry().glPopAttrib)()
    }
    unsafe fn glSecondaryColor3d(&self, _red: GLdouble, _green: GLdouble, _blue: GLdouble) {
        (self.entry().glSecondaryColor3d)(_red, _green, _blue)
    }
    unsafe fn glGenFramebuffers(&self, _n: GLsizei, _framebuffers: *mut GLuint) {
        (self.entry().glGenFramebuffers)(_n, _framebuffers)
    }
    unsafe fn glEnablei(&self, _target: EnableCap, _index: GLuint) {
        (self.entry().glEnablei)(_target, _index)
    }
    unsafe fn glBindBuffer(&self, _target: BufferTargetARB, _buffer: GLuint) {
        (self.entry().glBindBuffer)(_target, _buffer)
    }
    unsafe fn glNormal3f(&self, _nx: GLfloat, _ny: GLfloat, _nz: GLfloat) {
        (self.entry().glNormal3f)(_nx, _ny, _nz)
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
    unsafe fn glScaled(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glScaled)(_x, _y, _z)
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
    unsafe fn glMultiTexCoord3dv(&self, _target: TextureUnit, _v: *const GLdouble) {
        (self.entry().glMultiTexCoord3dv)(_target, _v)
    }
    unsafe fn glMultTransposeMatrixd(&self, _m: *const GLdouble) {
        (self.entry().glMultTransposeMatrixd)(_m)
    }
    unsafe fn glGetUniformiv(&self, _program: GLuint, _location: GLint, _params: *mut GLint) {
        (self.entry().glGetUniformiv)(_program, _location, _params)
    }
    unsafe fn glVertex4iv(&self, _v: *const GLint) {
        (self.entry().glVertex4iv)(_v)
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
    unsafe fn glEndList(&self) {
        (self.entry().glEndList)()
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
    unsafe fn glAccum(&self, _op: AccumOp, _value: GLfloat) {
        (self.entry().glAccum)(_op, _value)
    }
    unsafe fn glPopName(&self) {
        (self.entry().glPopName)()
    }
    unsafe fn glGetDoublev(&self, _pname: GetPName, _data: *mut GLdouble) {
        (self.entry().glGetDoublev)(_pname, _data)
    }
    unsafe fn glNormal3iv(&self, _v: *const GLint) {
        (self.entry().glNormal3iv)(_v)
    }
    unsafe fn glVertex2d(&self, _x: GLdouble, _y: GLdouble) {
        (self.entry().glVertex2d)(_x, _y)
    }
    unsafe fn glGenVertexArrays(&self, _n: GLsizei, _arrays: *mut GLuint) {
        (self.entry().glGenVertexArrays)(_n, _arrays)
    }
    unsafe fn glStencilFunc(&self, _func: StencilFunction, _ref: GLint, _mask: GLuint) {
        (self.entry().glStencilFunc)(_func, _ref, _mask)
    }
    unsafe fn glRasterPos4dv(&self, _v: *const GLdouble) {
        (self.entry().glRasterPos4dv)(_v)
    }
    unsafe fn glVertex3fv(&self, _v: *const GLfloat) {
        (self.entry().glVertex3fv)(_v)
    }
    unsafe fn glMultiTexCoord1iv(&self, _target: TextureUnit, _v: *const GLint) {
        (self.entry().glMultiTexCoord1iv)(_target, _v)
    }
    unsafe fn glUniform4i(&self, _location: GLint, _v0: GLint, _v1: GLint, _v2: GLint, _v3: GLint) {
        (self.entry().glUniform4i)(_location, _v0, _v1, _v2, _v3)
    }
    unsafe fn glDrawPixels(
        &self,
        _width: GLsizei,
        _height: GLsizei,
        _format: PixelFormat,
        _type: PixelType,
        _pixels: *const std::os::raw::c_void,
    ) {
        (self.entry().glDrawPixels)(_width, _height, _format, _type, _pixels)
    }
    unsafe fn glVertex4d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble, _w: GLdouble) {
        (self.entry().glVertex4d)(_x, _y, _z, _w)
    }
    unsafe fn glClearBufferiv(&self, _buffer: Buffer, _drawbuffer: GLint, _value: *const GLint) {
        (self.entry().glClearBufferiv)(_buffer, _drawbuffer, _value)
    }
    unsafe fn glLighti(&self, _light: LightName, _pname: LightParameter, _param: GLint) {
        (self.entry().glLighti)(_light, _pname, _param)
    }
    unsafe fn glEvalCoord1d(&self, _u: GLdouble) {
        (self.entry().glEvalCoord1d)(_u)
    }
    unsafe fn glCopyPixels(
        &self,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _type: PixelCopyType,
    ) {
        (self.entry().glCopyPixels)(_x, _y, _width, _height, _type)
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
    unsafe fn glTexCoord3dv(&self, _v: *const GLdouble) {
        (self.entry().glTexCoord3dv)(_v)
    }
    unsafe fn glLineStipple(&self, _factor: GLint, _pattern: GLushort) {
        (self.entry().glLineStipple)(_factor, _pattern)
    }
    unsafe fn glLightModeli(&self, _pname: LightModelParameter, _param: GLint) {
        (self.entry().glLightModeli)(_pname, _param)
    }
    unsafe fn glGetMapiv(&self, _target: MapTarget, _query: GetMapQuery, _v: *mut GLint) {
        (self.entry().glGetMapiv)(_target, _query, _v)
    }
    unsafe fn glRecti(&self, _x1: GLint, _y1: GLint, _x2: GLint, _y2: GLint) {
        (self.entry().glRecti)(_x1, _y1, _x2, _y2)
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
    unsafe fn glTexCoord2fv(&self, _v: *const GLfloat) {
        (self.entry().glTexCoord2fv)(_v)
    }
    unsafe fn glArrayElement(&self, _i: GLint) {
        (self.entry().glArrayElement)(_i)
    }
    unsafe fn glRasterPos3fv(&self, _v: *const GLfloat) {
        (self.entry().glRasterPos3fv)(_v)
    }
    unsafe fn glMultiTexCoord3fv(&self, _target: TextureUnit, _v: *const GLfloat) {
        (self.entry().glMultiTexCoord3fv)(_target, _v)
    }
    unsafe fn glMultiTexCoord4f(
        &self,
        _target: TextureUnit,
        _s: GLfloat,
        _t: GLfloat,
        _r: GLfloat,
        _q: GLfloat,
    ) {
        (self.entry().glMultiTexCoord4f)(_target, _s, _t, _r, _q)
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
    unsafe fn glGenBuffers(&self, _n: GLsizei, _buffers: *mut GLuint) {
        (self.entry().glGenBuffers)(_n, _buffers)
    }
    unsafe fn glMultiTexCoord1s(&self, _target: TextureUnit, _s: GLshort) {
        (self.entry().glMultiTexCoord1s)(_target, _s)
    }
    unsafe fn glColor4dv(&self, _v: *const GLdouble) {
        (self.entry().glColor4dv)(_v)
    }
    unsafe fn glTexCoord1s(&self, _s: GLshort) {
        (self.entry().glTexCoord1s)(_s)
    }
    unsafe fn glVertexAttribI1iv(&self, _index: GLuint, _v: *const GLint) {
        (self.entry().glVertexAttribI1iv)(_index, _v)
    }
    unsafe fn glTexCoord4fv(&self, _v: *const GLfloat) {
        (self.entry().glTexCoord4fv)(_v)
    }
    unsafe fn glWindowPos2dv(&self, _v: *const GLdouble) {
        (self.entry().glWindowPos2dv)(_v)
    }
    unsafe fn glUniform2fv(&self, _location: GLint, _count: GLsizei, _value: *const GLfloat) {
        (self.entry().glUniform2fv)(_location, _count, _value)
    }
    unsafe fn glPointParameteriv(&self, _pname: PointParameterNameARB, _params: *const GLint) {
        (self.entry().glPointParameteriv)(_pname, _params)
    }
    unsafe fn glIndexf(&self, _c: GLfloat) {
        (self.entry().glIndexf)(_c)
    }
    unsafe fn glLightModeliv(&self, _pname: LightModelParameter, _params: *const GLint) {
        (self.entry().glLightModeliv)(_pname, _params)
    }
    unsafe fn glBeginConditionalRender(&self, _id: GLuint, _mode: ConditionalRenderMode) {
        (self.entry().glBeginConditionalRender)(_id, _mode)
    }
    unsafe fn glSecondaryColor3ubv(&self, _v: *const GLubyte) {
        (self.entry().glSecondaryColor3ubv)(_v)
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
    unsafe fn glNewList(&self, _list: GLuint, _mode: ListMode) {
        (self.entry().glNewList)(_list, _mode)
    }
    unsafe fn glLogicOp(&self, _opcode: LogicOp) {
        (self.entry().glLogicOp)(_opcode)
    }
    unsafe fn glVertexAttrib4Nubv(&self, _index: GLuint, _v: *const GLubyte) {
        (self.entry().glVertexAttrib4Nubv)(_index, _v)
    }
    unsafe fn glVertexAttrib1fv(&self, _index: GLuint, _v: *const GLfloat) {
        (self.entry().glVertexAttrib1fv)(_index, _v)
    }
    unsafe fn glDisablei(&self, _target: EnableCap, _index: GLuint) {
        (self.entry().glDisablei)(_target, _index)
    }
    unsafe fn glRects(&self, _x1: GLshort, _y1: GLshort, _x2: GLshort, _y2: GLshort) {
        (self.entry().glRects)(_x1, _y1, _x2, _y2)
    }
    unsafe fn glColor3ubv(&self, _v: *const GLubyte) {
        (self.entry().glColor3ubv)(_v)
    }
    unsafe fn glGetTexGeniv(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _params: *mut GLint,
    ) {
        (self.entry().glGetTexGeniv)(_coord, _pname, _params)
    }
    unsafe fn glEdgeFlagPointer(&self, _stride: GLsizei, _pointer: *const std::os::raw::c_void) {
        (self.entry().glEdgeFlagPointer)(_stride, _pointer)
    }
    unsafe fn glTexCoord4sv(&self, _v: *const GLshort) {
        (self.entry().glTexCoord4sv)(_v)
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
    unsafe fn glTexCoord3sv(&self, _v: *const GLshort) {
        (self.entry().glTexCoord3sv)(_v)
    }
    unsafe fn glTexCoord3fv(&self, _v: *const GLfloat) {
        (self.entry().glTexCoord3fv)(_v)
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
    unsafe fn glUniformMatrix4x2fv(
        &self,
        _location: GLint,
        _count: GLsizei,
        _transpose: GLboolean,
        _value: *const GLfloat,
    ) {
        (self.entry().glUniformMatrix4x2fv)(_location, _count, _transpose, _value)
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
    unsafe fn glTexGendv(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _params: *const GLdouble,
    ) {
        (self.entry().glTexGendv)(_coord, _pname, _params)
    }
    unsafe fn glFrontFace(&self, _mode: FrontFaceDirection) {
        (self.entry().glFrontFace)(_mode)
    }
    unsafe fn glNormal3fv(&self, _v: *const GLfloat) {
        (self.entry().glNormal3fv)(_v)
    }
    unsafe fn glTexGend(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _param: GLdouble,
    ) {
        (self.entry().glTexGend)(_coord, _pname, _param)
    }
    unsafe fn glUniform1fv(&self, _location: GLint, _count: GLsizei, _value: *const GLfloat) {
        (self.entry().glUniform1fv)(_location, _count, _value)
    }
    unsafe fn glReadBuffer(&self, _src: ReadBufferMode) {
        (self.entry().glReadBuffer)(_src)
    }
    unsafe fn glEvalCoord2f(&self, _u: GLfloat, _v: GLfloat) {
        (self.entry().glEvalCoord2f)(_u, _v)
    }
    unsafe fn glMultiTexCoord2i(&self, _target: TextureUnit, _s: GLint, _t: GLint) {
        (self.entry().glMultiTexCoord2i)(_target, _s, _t)
    }
    unsafe fn glWindowPos2sv(&self, _v: *const GLshort) {
        (self.entry().glWindowPos2sv)(_v)
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
    unsafe fn glVertex2dv(&self, _v: *const GLdouble) {
        (self.entry().glVertex2dv)(_v)
    }
    unsafe fn glVertex4fv(&self, _v: *const GLfloat) {
        (self.entry().glVertex4fv)(_v)
    }
    unsafe fn glVertex4s(&self, _x: GLshort, _y: GLshort, _z: GLshort, _w: GLshort) {
        (self.entry().glVertex4s)(_x, _y, _z, _w)
    }
    unsafe fn glLoadMatrixf(&self, _m: *const GLfloat) {
        (self.entry().glLoadMatrixf)(_m)
    }
    unsafe fn glGetCompressedTexImage(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _img: *mut std::os::raw::c_void,
    ) {
        (self.entry().glGetCompressedTexImage)(_target, _level, _img)
    }
    unsafe fn glColor4ui(&self, _red: GLuint, _green: GLuint, _blue: GLuint, _alpha: GLuint) {
        (self.entry().glColor4ui)(_red, _green, _blue, _alpha)
    }
    unsafe fn glWindowPos3d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glWindowPos3d)(_x, _y, _z)
    }
    unsafe fn glDeleteTextures(&self, _n: GLsizei, _textures: *const GLuint) {
        (self.entry().glDeleteTextures)(_n, _textures)
    }
    unsafe fn glSelectBuffer(&self, _size: GLsizei, _buffer: *mut GLuint) {
        (self.entry().glSelectBuffer)(_size, _buffer)
    }
    unsafe fn glLoadName(&self, _name: GLuint) {
        (self.entry().glLoadName)(_name)
    }
    unsafe fn glPixelTransferi(&self, _pname: PixelTransferParameter, _param: GLint) {
        (self.entry().glPixelTransferi)(_pname, _param)
    }
    unsafe fn glTexCoord1fv(&self, _v: *const GLfloat) {
        (self.entry().glTexCoord1fv)(_v)
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
    unsafe fn glColor3bv(&self, _v: *const GLbyte) {
        (self.entry().glColor3bv)(_v)
    }
    unsafe fn glLoadMatrixd(&self, _m: *const GLdouble) {
        (self.entry().glLoadMatrixd)(_m)
    }
    unsafe fn glPixelTransferf(&self, _pname: PixelTransferParameter, _param: GLfloat) {
        (self.entry().glPixelTransferf)(_pname, _param)
    }
    unsafe fn glEdgeFlagv(&self, _flag: *const GLboolean) {
        (self.entry().glEdgeFlagv)(_flag)
    }
    unsafe fn glNormal3bv(&self, _v: *const GLbyte) {
        (self.entry().glNormal3bv)(_v)
    }
    unsafe fn glTexEnvf(
        &self,
        _target: TextureEnvTarget,
        _pname: TextureEnvParameter,
        _param: GLfloat,
    ) {
        (self.entry().glTexEnvf)(_target, _pname, _param)
    }
    unsafe fn glEvalCoord2dv(&self, _u: *const GLdouble) {
        (self.entry().glEvalCoord2dv)(_u)
    }
    unsafe fn glLinkProgram(&self, _program: GLuint) {
        (self.entry().glLinkProgram)(_program)
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
    unsafe fn glGetAttachedShaders(
        &self,
        _program: GLuint,
        _maxCount: GLsizei,
        _count: *mut GLsizei,
        _shaders: *mut GLuint,
    ) {
        (self.entry().glGetAttachedShaders)(_program, _maxCount, _count, _shaders)
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
    unsafe fn glColor3ub(&self, _red: GLubyte, _green: GLubyte, _blue: GLubyte) {
        (self.entry().glColor3ub)(_red, _green, _blue)
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
    unsafe fn glWindowPos2s(&self, _x: GLshort, _y: GLshort) {
        (self.entry().glWindowPos2s)(_x, _y)
    }
    unsafe fn glTexEnvfv(
        &self,
        _target: TextureEnvTarget,
        _pname: TextureEnvParameter,
        _params: *const GLfloat,
    ) {
        (self.entry().glTexEnvfv)(_target, _pname, _params)
    }
    unsafe fn glLoadTransposeMatrixf(&self, _m: *const GLfloat) {
        (self.entry().glLoadTransposeMatrixf)(_m)
    }
    unsafe fn glColor3us(&self, _red: GLushort, _green: GLushort, _blue: GLushort) {
        (self.entry().glColor3us)(_red, _green, _blue)
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
    unsafe fn glDisableClientState(&self, _array: EnableCap) {
        (self.entry().glDisableClientState)(_array)
    }
    unsafe fn glVertexAttrib3f(&self, _index: GLuint, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glVertexAttrib3f)(_index, _x, _y, _z)
    }
    unsafe fn glAreTexturesResident(
        &self,
        _n: GLsizei,
        _textures: *const GLuint,
        _residences: *mut GLboolean,
    ) -> GLboolean {
        (self.entry().glAreTexturesResident)(_n, _textures, _residences)
    }
    unsafe fn glMap1d(
        &self,
        _target: MapTarget,
        _u1: GLdouble,
        _u2: GLdouble,
        _stride: GLint,
        _order: GLint,
        _points: *const GLdouble,
    ) {
        (self.entry().glMap1d)(_target, _u1, _u2, _stride, _order, _points)
    }
    unsafe fn glUniform2i(&self, _location: GLint, _v0: GLint, _v1: GLint) {
        (self.entry().glUniform2i)(_location, _v0, _v1)
    }
    unsafe fn glColor3sv(&self, _v: *const GLshort) {
        (self.entry().glColor3sv)(_v)
    }
    unsafe fn glGetIntegerv(&self, _pname: GetPName, _data: *mut GLint) {
        (self.entry().glGetIntegerv)(_pname, _data)
    }
    unsafe fn glColor4iv(&self, _v: *const GLint) {
        (self.entry().glColor4iv)(_v)
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
    unsafe fn glRotated(&self, _angle: GLdouble, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glRotated)(_angle, _x, _y, _z)
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
    unsafe fn glMap2d(
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
        (self.entry().glMap2d)(
            _target, _u1, _u2, _ustride, _uorder, _v1, _v2, _vstride, _vorder, _points,
        )
    }
    unsafe fn glSecondaryColor3uiv(&self, _v: *const GLuint) {
        (self.entry().glSecondaryColor3uiv)(_v)
    }
    unsafe fn glFlush(&self) {
        (self.entry().glFlush)()
    }
    unsafe fn glVertex4f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat, _w: GLfloat) {
        (self.entry().glVertex4f)(_x, _y, _z, _w)
    }
    unsafe fn glFogi(&self, _pname: FogParameter, _param: GLint) {
        (self.entry().glFogi)(_pname, _param)
    }
    unsafe fn glGenQueries(&self, _n: GLsizei, _ids: *mut GLuint) {
        (self.entry().glGenQueries)(_n, _ids)
    }
    unsafe fn glPixelMapfv(&self, _map: PixelMap, _mapsize: GLsizei, _values: *const GLfloat) {
        (self.entry().glPixelMapfv)(_map, _mapsize, _values)
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
    unsafe fn glMultMatrixf(&self, _m: *const GLfloat) {
        (self.entry().glMultMatrixf)(_m)
    }
    unsafe fn glVertexAttribI3i(&self, _index: GLuint, _x: GLint, _y: GLint, _z: GLint) {
        (self.entry().glVertexAttribI3i)(_index, _x, _y, _z)
    }
    unsafe fn glGetTexGendv(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _params: *mut GLdouble,
    ) {
        (self.entry().glGetTexGendv)(_coord, _pname, _params)
    }
    unsafe fn glRasterPos2f(&self, _x: GLfloat, _y: GLfloat) {
        (self.entry().glRasterPos2f)(_x, _y)
    }
    unsafe fn glMultiTexCoord3i(&self, _target: TextureUnit, _s: GLint, _t: GLint, _r: GLint) {
        (self.entry().glMultiTexCoord3i)(_target, _s, _t, _r)
    }
    unsafe fn glGetString(&self, _name: StringName) -> *const GLubyte {
        (self.entry().glGetString)(_name)
    }
    unsafe fn glRasterPos4f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat, _w: GLfloat) {
        (self.entry().glRasterPos4f)(_x, _y, _z, _w)
    }
    unsafe fn glTexCoord4d(&self, _s: GLdouble, _t: GLdouble, _r: GLdouble, _q: GLdouble) {
        (self.entry().glTexCoord4d)(_s, _t, _r, _q)
    }
    unsafe fn glVertex4sv(&self, _v: *const GLshort) {
        (self.entry().glVertex4sv)(_v)
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
    unsafe fn glTexCoord1dv(&self, _v: *const GLdouble) {
        (self.entry().glTexCoord1dv)(_v)
    }
    unsafe fn glGetMapfv(&self, _target: MapTarget, _query: GetMapQuery, _v: *mut GLfloat) {
        (self.entry().glGetMapfv)(_target, _query, _v)
    }
    unsafe fn glColor4b(&self, _red: GLbyte, _green: GLbyte, _blue: GLbyte, _alpha: GLbyte) {
        (self.entry().glColor4b)(_red, _green, _blue, _alpha)
    }
    unsafe fn glTexCoord1f(&self, _s: GLfloat) {
        (self.entry().glTexCoord1f)(_s)
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
    unsafe fn glMultiTexCoord1d(&self, _target: TextureUnit, _s: GLdouble) {
        (self.entry().glMultiTexCoord1d)(_target, _s)
    }
    unsafe fn glFogCoordPointer(
        &self,
        _type: FogPointerTypeEXT,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glFogCoordPointer)(_type, _stride, _pointer)
    }
    unsafe fn glUniform1iv(&self, _location: GLint, _count: GLsizei, _value: *const GLint) {
        (self.entry().glUniform1iv)(_location, _count, _value)
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
    unsafe fn glColor4sv(&self, _v: *const GLshort) {
        (self.entry().glColor4sv)(_v)
    }
    unsafe fn glIndexd(&self, _c: GLdouble) {
        (self.entry().glIndexd)(_c)
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
    unsafe fn glColorMaterial(&self, _face: MaterialFace, _mode: ColorMaterialParameter) {
        (self.entry().glColorMaterial)(_face, _mode)
    }
    unsafe fn glAttachShader(&self, _program: GLuint, _shader: GLuint) {
        (self.entry().glAttachShader)(_program, _shader)
    }
    unsafe fn glIsBuffer(&self, _buffer: GLuint) -> GLboolean {
        (self.entry().glIsBuffer)(_buffer)
    }
    unsafe fn glPrioritizeTextures(
        &self,
        _n: GLsizei,
        _textures: *const GLuint,
        _priorities: *const GLfloat,
    ) {
        (self.entry().glPrioritizeTextures)(_n, _textures, _priorities)
    }
    unsafe fn glGetBufferParameteriv(
        &self,
        _target: BufferTargetARB,
        _pname: BufferPNameARB,
        _params: *mut GLint,
    ) {
        (self.entry().glGetBufferParameteriv)(_target, _pname, _params)
    }
    unsafe fn glColor3usv(&self, _v: *const GLushort) {
        (self.entry().glColor3usv)(_v)
    }
    unsafe fn glMultTransposeMatrixf(&self, _m: *const GLfloat) {
        (self.entry().glMultTransposeMatrixf)(_m)
    }
    unsafe fn glDeleteProgram(&self, _program: GLuint) {
        (self.entry().glDeleteProgram)(_program)
    }
    unsafe fn glRasterPos3d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glRasterPos3d)(_x, _y, _z)
    }
    unsafe fn glListBase(&self, _base: GLuint) {
        (self.entry().glListBase)(_base)
    }
    unsafe fn glUniform4uiv(&self, _location: GLint, _count: GLsizei, _value: *const GLuint) {
        (self.entry().glUniform4uiv)(_location, _count, _value)
    }
    unsafe fn glEvalMesh1(&self, _mode: MeshMode1, _i1: GLint, _i2: GLint) {
        (self.entry().glEvalMesh1)(_mode, _i1, _i2)
    }
    unsafe fn glIndexMask(&self, _mask: GLuint) {
        (self.entry().glIndexMask)(_mask)
    }
    unsafe fn glIndexubv(&self, _c: *const GLubyte) {
        (self.entry().glIndexubv)(_c)
    }
    unsafe fn glMultiTexCoord2sv(&self, _target: TextureUnit, _v: *const GLshort) {
        (self.entry().glMultiTexCoord2sv)(_target, _v)
    }
    unsafe fn glSecondaryColor3sv(&self, _v: *const GLshort) {
        (self.entry().glSecondaryColor3sv)(_v)
    }
    unsafe fn glTexParameteri(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _param: GLint,
    ) {
        (self.entry().glTexParameteri)(_target, _pname, _param)
    }
    unsafe fn glColor3d(&self, _red: GLdouble, _green: GLdouble, _blue: GLdouble) {
        (self.entry().glColor3d)(_red, _green, _blue)
    }
    unsafe fn glRasterPos2s(&self, _x: GLshort, _y: GLshort) {
        (self.entry().glRasterPos2s)(_x, _y)
    }
    unsafe fn glMultMatrixd(&self, _m: *const GLdouble) {
        (self.entry().glMultMatrixd)(_m)
    }
    unsafe fn glMultiTexCoord4s(
        &self,
        _target: TextureUnit,
        _s: GLshort,
        _t: GLshort,
        _r: GLshort,
        _q: GLshort,
    ) {
        (self.entry().glMultiTexCoord4s)(_target, _s, _t, _r, _q)
    }
    unsafe fn glBindVertexArray(&self, _array: GLuint) {
        (self.entry().glBindVertexArray)(_array)
    }
    unsafe fn glWindowPos2d(&self, _x: GLdouble, _y: GLdouble) {
        (self.entry().glWindowPos2d)(_x, _y)
    }
    unsafe fn glRenderMode(&self, _mode: RenderingMode) -> GLint {
        (self.entry().glRenderMode)(_mode)
    }
    unsafe fn glColor4ub(&self, _red: GLubyte, _green: GLubyte, _blue: GLubyte, _alpha: GLubyte) {
        (self.entry().glColor4ub)(_red, _green, _blue, _alpha)
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
    unsafe fn glTexGeniv(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _params: *const GLint,
    ) {
        (self.entry().glTexGeniv)(_coord, _pname, _params)
    }
    unsafe fn glMultiTexCoord4i(
        &self,
        _target: TextureUnit,
        _s: GLint,
        _t: GLint,
        _r: GLint,
        _q: GLint,
    ) {
        (self.entry().glMultiTexCoord4i)(_target, _s, _t, _r, _q)
    }
    unsafe fn glVertexAttrib3s(&self, _index: GLuint, _x: GLshort, _y: GLshort, _z: GLshort) {
        (self.entry().glVertexAttrib3s)(_index, _x, _y, _z)
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
    unsafe fn glEdgeFlag(&self, _flag: GLboolean) {
        (self.entry().glEdgeFlag)(_flag)
    }
    unsafe fn glTranslated(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glTranslated)(_x, _y, _z)
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
    unsafe fn glVertex4i(&self, _x: GLint, _y: GLint, _z: GLint, _w: GLint) {
        (self.entry().glVertex4i)(_x, _y, _z, _w)
    }
    unsafe fn glTexCoord3iv(&self, _v: *const GLint) {
        (self.entry().glTexCoord3iv)(_v)
    }
    unsafe fn glNormal3d(&self, _nx: GLdouble, _ny: GLdouble, _nz: GLdouble) {
        (self.entry().glNormal3d)(_nx, _ny, _nz)
    }
    unsafe fn glColor3i(&self, _red: GLint, _green: GLint, _blue: GLint) {
        (self.entry().glColor3i)(_red, _green, _blue)
    }
    unsafe fn glClear(&self, _mask: ClearBufferMask) {
        (self.entry().glClear)(_mask)
    }
    unsafe fn glNormal3sv(&self, _v: *const GLshort) {
        (self.entry().glNormal3sv)(_v)
    }
    unsafe fn glTexCoord4i(&self, _s: GLint, _t: GLint, _r: GLint, _q: GLint) {
        (self.entry().glTexCoord4i)(_s, _t, _r, _q)
    }
    unsafe fn glGetPixelMapuiv(&self, _map: PixelMap, _values: *mut GLuint) {
        (self.entry().glGetPixelMapuiv)(_map, _values)
    }
    unsafe fn glVertexAttrib4Nbv(&self, _index: GLuint, _v: *const GLbyte) {
        (self.entry().glVertexAttrib4Nbv)(_index, _v)
    }
    unsafe fn glFogCoorddv(&self, _coord: *const GLdouble) {
        (self.entry().glFogCoorddv)(_coord)
    }
    unsafe fn glVertexAttrib4Nuiv(&self, _index: GLuint, _v: *const GLuint) {
        (self.entry().glVertexAttrib4Nuiv)(_index, _v)
    }
    unsafe fn glTexCoord4dv(&self, _v: *const GLdouble) {
        (self.entry().glTexCoord4dv)(_v)
    }
    unsafe fn glVertexAttribI4iv(&self, _index: GLuint, _v: *const GLint) {
        (self.entry().glVertexAttribI4iv)(_index, _v)
    }
    unsafe fn glVertexAttrib1d(&self, _index: GLuint, _x: GLdouble) {
        (self.entry().glVertexAttrib1d)(_index, _x)
    }
    unsafe fn glUniform2uiv(&self, _location: GLint, _count: GLsizei, _value: *const GLuint) {
        (self.entry().glUniform2uiv)(_location, _count, _value)
    }
    unsafe fn glWindowPos3dv(&self, _v: *const GLdouble) {
        (self.entry().glWindowPos3dv)(_v)
    }
    unsafe fn glLightf(&self, _light: LightName, _pname: LightParameter, _param: GLfloat) {
        (self.entry().glLightf)(_light, _pname, _param)
    }
    unsafe fn glGetTexEnvfv(
        &self,
        _target: TextureEnvTarget,
        _pname: TextureEnvParameter,
        _params: *mut GLfloat,
    ) {
        (self.entry().glGetTexEnvfv)(_target, _pname, _params)
    }
    unsafe fn glGetTexEnviv(
        &self,
        _target: TextureEnvTarget,
        _pname: TextureEnvParameter,
        _params: *mut GLint,
    ) {
        (self.entry().glGetTexEnviv)(_target, _pname, _params)
    }
    unsafe fn glVertexAttrib2fv(&self, _index: GLuint, _v: *const GLfloat) {
        (self.entry().glVertexAttrib2fv)(_index, _v)
    }
    unsafe fn glGetUniformfv(&self, _program: GLuint, _location: GLint, _params: *mut GLfloat) {
        (self.entry().glGetUniformfv)(_program, _location, _params)
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
    unsafe fn glMultiTexCoord4fv(&self, _target: TextureUnit, _v: *const GLfloat) {
        (self.entry().glMultiTexCoord4fv)(_target, _v)
    }
    unsafe fn glIndexfv(&self, _c: *const GLfloat) {
        (self.entry().glIndexfv)(_c)
    }
    unsafe fn glRasterPos4sv(&self, _v: *const GLshort) {
        (self.entry().glRasterPos4sv)(_v)
    }
    unsafe fn glDeleteRenderbuffers(&self, _n: GLsizei, _renderbuffers: *const GLuint) {
        (self.entry().glDeleteRenderbuffers)(_n, _renderbuffers)
    }
    unsafe fn glCheckFramebufferStatus(&self, _target: FramebufferTarget) -> GLenum {
        (self.entry().glCheckFramebufferStatus)(_target)
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
    unsafe fn glEvalCoord2fv(&self, _u: *const GLfloat) {
        (self.entry().glEvalCoord2fv)(_u)
    }
    unsafe fn glGetBufferPointerv(
        &self,
        _target: BufferTargetARB,
        _pname: BufferPointerNameARB,
        _params: *mut *mut std::os::raw::c_void,
    ) {
        (self.entry().glGetBufferPointerv)(_target, _pname, _params)
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
    unsafe fn glRasterPos3dv(&self, _v: *const GLdouble) {
        (self.entry().glRasterPos3dv)(_v)
    }
    unsafe fn glMultiTexCoord1dv(&self, _target: TextureUnit, _v: *const GLdouble) {
        (self.entry().glMultiTexCoord1dv)(_target, _v)
    }
    unsafe fn glMultiTexCoord2f(&self, _target: TextureUnit, _s: GLfloat, _t: GLfloat) {
        (self.entry().glMultiTexCoord2f)(_target, _s, _t)
    }
    unsafe fn glEnableVertexAttribArray(&self, _index: GLuint) {
        (self.entry().glEnableVertexAttribArray)(_index)
    }
    unsafe fn glUniform3iv(&self, _location: GLint, _count: GLsizei, _value: *const GLint) {
        (self.entry().glUniform3iv)(_location, _count, _value)
    }
    unsafe fn glWindowPos3s(&self, _x: GLshort, _y: GLshort, _z: GLshort) {
        (self.entry().glWindowPos3s)(_x, _y, _z)
    }
    unsafe fn glVertexAttrib2s(&self, _index: GLuint, _x: GLshort, _y: GLshort) {
        (self.entry().glVertexAttrib2s)(_index, _x, _y)
    }
    unsafe fn glVertexAttrib4sv(&self, _index: GLuint, _v: *const GLshort) {
        (self.entry().glVertexAttrib4sv)(_index, _v)
    }
    unsafe fn glTexCoord2sv(&self, _v: *const GLshort) {
        (self.entry().glTexCoord2sv)(_v)
    }
    unsafe fn glMultiTexCoord4d(
        &self,
        _target: TextureUnit,
        _s: GLdouble,
        _t: GLdouble,
        _r: GLdouble,
        _q: GLdouble,
    ) {
        (self.entry().glMultiTexCoord4d)(_target, _s, _t, _r, _q)
    }
    unsafe fn glSecondaryColor3usv(&self, _v: *const GLushort) {
        (self.entry().glSecondaryColor3usv)(_v)
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
    unsafe fn glLoadTransposeMatrixd(&self, _m: *const GLdouble) {
        (self.entry().glLoadTransposeMatrixd)(_m)
    }
    unsafe fn glTexCoord3f(&self, _s: GLfloat, _t: GLfloat, _r: GLfloat) {
        (self.entry().glTexCoord3f)(_s, _t, _r)
    }
    unsafe fn glWindowPos3i(&self, _x: GLint, _y: GLint, _z: GLint) {
        (self.entry().glWindowPos3i)(_x, _y, _z)
    }
    unsafe fn glDeleteQueries(&self, _n: GLsizei, _ids: *const GLuint) {
        (self.entry().glDeleteQueries)(_n, _ids)
    }
    unsafe fn glTexCoord1sv(&self, _v: *const GLshort) {
        (self.entry().glTexCoord1sv)(_v)
    }
    unsafe fn glWindowPos2fv(&self, _v: *const GLfloat) {
        (self.entry().glWindowPos2fv)(_v)
    }
    unsafe fn glShadeModel(&self, _mode: ShadingModel) {
        (self.entry().glShadeModel)(_mode)
    }
    unsafe fn glVertexAttribI4bv(&self, _index: GLuint, _v: *const GLbyte) {
        (self.entry().glVertexAttribI4bv)(_index, _v)
    }
    unsafe fn glMultiTexCoord2s(&self, _target: TextureUnit, _s: GLshort, _t: GLshort) {
        (self.entry().glMultiTexCoord2s)(_target, _s, _t)
    }
    unsafe fn glClipPlane(&self, _plane: ClipPlaneName, _equation: *const GLdouble) {
        (self.entry().glClipPlane)(_plane, _equation)
    }
    unsafe fn glGenerateMipmap(&self, _target: TextureTarget) {
        (self.entry().glGenerateMipmap)(_target)
    }
    unsafe fn glPrimitiveRestartIndex(&self, _index: GLuint) {
        (self.entry().glPrimitiveRestartIndex)(_index)
    }
    unsafe fn glRasterPos4iv(&self, _v: *const GLint) {
        (self.entry().glRasterPos4iv)(_v)
    }
    unsafe fn glFogCoordd(&self, _coord: GLdouble) {
        (self.entry().glFogCoordd)(_coord)
    }
    unsafe fn glGetRenderbufferParameteriv(
        &self,
        _target: RenderbufferTarget,
        _pname: RenderbufferParameterName,
        _params: *mut GLint,
    ) {
        (self.entry().glGetRenderbufferParameteriv)(_target, _pname, _params)
    }
    unsafe fn glRectdv(&self, _v1: *const GLdouble, _v2: *const GLdouble) {
        (self.entry().glRectdv)(_v1, _v2)
    }
    unsafe fn glVertexAttribI3uiv(&self, _index: GLuint, _v: *const GLuint) {
        (self.entry().glVertexAttribI3uiv)(_index, _v)
    }
    unsafe fn glFlushMappedBufferRange(
        &self,
        _target: BufferTargetARB,
        _offset: GLintptr,
        _length: GLsizeiptr,
    ) {
        (self.entry().glFlushMappedBufferRange)(_target, _offset, _length)
    }
    unsafe fn glVertexAttribI2ui(&self, _index: GLuint, _x: GLuint, _y: GLuint) {
        (self.entry().glVertexAttribI2ui)(_index, _x, _y)
    }
    unsafe fn glTexCoord1d(&self, _s: GLdouble) {
        (self.entry().glTexCoord1d)(_s)
    }
    unsafe fn glMap1f(
        &self,
        _target: MapTarget,
        _u1: GLfloat,
        _u2: GLfloat,
        _stride: GLint,
        _order: GLint,
        _points: *const GLfloat,
    ) {
        (self.entry().glMap1f)(_target, _u1, _u2, _stride, _order, _points)
    }
    unsafe fn glMatrixMode(&self, _mode: MatrixMode) {
        (self.entry().glMatrixMode)(_mode)
    }
    unsafe fn glScissor(&self, _x: GLint, _y: GLint, _width: GLsizei, _height: GLsizei) {
        (self.entry().glScissor)(_x, _y, _width, _height)
    }
    unsafe fn glPolygonMode(&self, _face: MaterialFace, _mode: PolygonMode) {
        (self.entry().glPolygonMode)(_face, _mode)
    }
    unsafe fn glCallLists(
        &self,
        _n: GLsizei,
        _type: ListNameType,
        _lists: *const std::os::raw::c_void,
    ) {
        (self.entry().glCallLists)(_n, _type, _lists)
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
    unsafe fn glTexCoord4iv(&self, _v: *const GLint) {
        (self.entry().glTexCoord4iv)(_v)
    }
    unsafe fn glVertexAttribI4i(&self, _index: GLuint, _x: GLint, _y: GLint, _z: GLint, _w: GLint) {
        (self.entry().glVertexAttribI4i)(_index, _x, _y, _z, _w)
    }
    unsafe fn glEndTransformFeedback(&self) {
        (self.entry().glEndTransformFeedback)()
    }
    unsafe fn glColor4bv(&self, _v: *const GLbyte) {
        (self.entry().glColor4bv)(_v)
    }
    unsafe fn glRectf(&self, _x1: GLfloat, _y1: GLfloat, _x2: GLfloat, _y2: GLfloat) {
        (self.entry().glRectf)(_x1, _y1, _x2, _y2)
    }
    unsafe fn glTexCoordPointer(
        &self,
        _size: GLint,
        _type: TexCoordPointerType,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glTexCoordPointer)(_size, _type, _stride, _pointer)
    }
    unsafe fn glEnable(&self, _cap: EnableCap) {
        (self.entry().glEnable)(_cap)
    }
    unsafe fn glGenLists(&self, _range: GLsizei) -> GLuint {
        (self.entry().glGenLists)(_range)
    }
    unsafe fn glVertexAttrib4uiv(&self, _index: GLuint, _v: *const GLuint) {
        (self.entry().glVertexAttrib4uiv)(_index, _v)
    }
    unsafe fn glGetTexGenfv(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _params: *mut GLfloat,
    ) {
        (self.entry().glGetTexGenfv)(_coord, _pname, _params)
    }
    unsafe fn glVertex3d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glVertex3d)(_x, _y, _z)
    }
    unsafe fn glPushMatrix(&self) {
        (self.entry().glPushMatrix)()
    }
    unsafe fn glEvalMesh2(&self, _mode: MeshMode2, _i1: GLint, _i2: GLint, _j1: GLint, _j2: GLint) {
        (self.entry().glEvalMesh2)(_mode, _i1, _i2, _j1, _j2)
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
    unsafe fn glVertex4dv(&self, _v: *const GLdouble) {
        (self.entry().glVertex4dv)(_v)
    }
    unsafe fn glColor4uiv(&self, _v: *const GLuint) {
        (self.entry().glColor4uiv)(_v)
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
    unsafe fn glRasterPos2d(&self, _x: GLdouble, _y: GLdouble) {
        (self.entry().glRasterPos2d)(_x, _y)
    }
    unsafe fn glCreateProgram(&self) -> GLuint {
        (self.entry().glCreateProgram)()
    }
    unsafe fn glFeedbackBuffer(&self, _size: GLsizei, _type: FeedbackType, _buffer: *mut GLfloat) {
        (self.entry().glFeedbackBuffer)(_size, _type, _buffer)
    }
    unsafe fn glIsList(&self, _list: GLuint) -> GLboolean {
        (self.entry().glIsList)(_list)
    }
    unsafe fn glVertexAttrib1dv(&self, _index: GLuint, _v: *const GLdouble) {
        (self.entry().glVertexAttrib1dv)(_index, _v)
    }
    unsafe fn glClearDepth(&self, _depth: GLdouble) {
        (self.entry().glClearDepth)(_depth)
    }
    unsafe fn glFogCoordfv(&self, _coord: *const GLfloat) {
        (self.entry().glFogCoordfv)(_coord)
    }
    unsafe fn glWindowPos3sv(&self, _v: *const GLshort) {
        (self.entry().glWindowPos3sv)(_v)
    }
    unsafe fn glDeleteFramebuffers(&self, _n: GLsizei, _framebuffers: *const GLuint) {
        (self.entry().glDeleteFramebuffers)(_n, _framebuffers)
    }
    unsafe fn glPolygonOffset(&self, _factor: GLfloat, _units: GLfloat) {
        (self.entry().glPolygonOffset)(_factor, _units)
    }
    unsafe fn glColor4fv(&self, _v: *const GLfloat) {
        (self.entry().glColor4fv)(_v)
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
    unsafe fn glColor3f(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat) {
        (self.entry().glColor3f)(_red, _green, _blue)
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
    unsafe fn glTexCoord2d(&self, _s: GLdouble, _t: GLdouble) {
        (self.entry().glTexCoord2d)(_s, _t)
    }
    unsafe fn glSampleCoverage(&self, _value: GLfloat, _invert: GLboolean) {
        (self.entry().glSampleCoverage)(_value, _invert)
    }
    unsafe fn glRotatef(&self, _angle: GLfloat, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glRotatef)(_angle, _x, _y, _z)
    }
    unsafe fn glVertex2iv(&self, _v: *const GLint) {
        (self.entry().glVertex2iv)(_v)
    }
    unsafe fn glSecondaryColorPointer(
        &self,
        _size: GLint,
        _type: ColorPointerType,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glSecondaryColorPointer)(_size, _type, _stride, _pointer)
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
    unsafe fn glColor4i(&self, _red: GLint, _green: GLint, _blue: GLint, _alpha: GLint) {
        (self.entry().glColor4i)(_red, _green, _blue, _alpha)
    }
    unsafe fn glVertex2f(&self, _x: GLfloat, _y: GLfloat) {
        (self.entry().glVertex2f)(_x, _y)
    }
    unsafe fn glMapGrid1d(&self, _un: GLint, _u1: GLdouble, _u2: GLdouble) {
        (self.entry().glMapGrid1d)(_un, _u1, _u2)
    }
    unsafe fn glVertex2sv(&self, _v: *const GLshort) {
        (self.entry().glVertex2sv)(_v)
    }
    unsafe fn glGetFragDataLocation(&self, _program: GLuint, _name: *const GLchar) -> GLint {
        (self.entry().glGetFragDataLocation)(_program, _name)
    }
    unsafe fn glMultiTexCoord1sv(&self, _target: TextureUnit, _v: *const GLshort) {
        (self.entry().glMultiTexCoord1sv)(_target, _v)
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
    unsafe fn glMultiTexCoord4dv(&self, _target: TextureUnit, _v: *const GLdouble) {
        (self.entry().glMultiTexCoord4dv)(_target, _v)
    }
    unsafe fn glColor4s(&self, _red: GLshort, _green: GLshort, _blue: GLshort, _alpha: GLshort) {
        (self.entry().glColor4s)(_red, _green, _blue, _alpha)
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
    unsafe fn glIndexsv(&self, _c: *const GLshort) {
        (self.entry().glIndexsv)(_c)
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
    unsafe fn glSecondaryColor3ui(&self, _red: GLuint, _green: GLuint, _blue: GLuint) {
        (self.entry().glSecondaryColor3ui)(_red, _green, _blue)
    }
    unsafe fn glAlphaFunc(&self, _func: AlphaFunction, _ref: GLfloat) {
        (self.entry().glAlphaFunc)(_func, _ref)
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
    unsafe fn glTexCoord1iv(&self, _v: *const GLint) {
        (self.entry().glTexCoord1iv)(_v)
    }
    unsafe fn glColor3dv(&self, _v: *const GLdouble) {
        (self.entry().glColor3dv)(_v)
    }
    unsafe fn glFogf(&self, _pname: FogParameter, _param: GLfloat) {
        (self.entry().glFogf)(_pname, _param)
    }
    unsafe fn glCallList(&self, _list: GLuint) {
        (self.entry().glCallList)(_list)
    }
    unsafe fn glMateriali(&self, _face: MaterialFace, _pname: MaterialParameter, _param: GLint) {
        (self.entry().glMateriali)(_face, _pname, _param)
    }
    unsafe fn glMapGrid1f(&self, _un: GLint, _u1: GLfloat, _u2: GLfloat) {
        (self.entry().glMapGrid1f)(_un, _u1, _u2)
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
    unsafe fn glNormalPointer(
        &self,
        _type: NormalPointerType,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glNormalPointer)(_type, _stride, _pointer)
    }
    unsafe fn glGetIntegeri_v(&self, _target: GetPName, _index: GLuint, _data: *mut GLint) {
        (self.entry().glGetIntegeri_v)(_target, _index, _data)
    }
    unsafe fn glColor3b(&self, _red: GLbyte, _green: GLbyte, _blue: GLbyte) {
        (self.entry().glColor3b)(_red, _green, _blue)
    }
    unsafe fn glLoadIdentity(&self) {
        (self.entry().glLoadIdentity)()
    }
    unsafe fn glGetPointerv(
        &self,
        _pname: GetPointervPName,
        _params: *mut *mut std::os::raw::c_void,
    ) {
        (self.entry().glGetPointerv)(_pname, _params)
    }
    unsafe fn glLightiv(&self, _light: LightName, _pname: LightParameter, _params: *const GLint) {
        (self.entry().glLightiv)(_light, _pname, _params)
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
    unsafe fn glRasterPos2sv(&self, _v: *const GLshort) {
        (self.entry().glRasterPos2sv)(_v)
    }
    unsafe fn glVertex3i(&self, _x: GLint, _y: GLint, _z: GLint) {
        (self.entry().glVertex3i)(_x, _y, _z)
    }
    unsafe fn glTexCoord4f(&self, _s: GLfloat, _t: GLfloat, _r: GLfloat, _q: GLfloat) {
        (self.entry().glTexCoord4f)(_s, _t, _r, _q)
    }
    unsafe fn glScalef(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glScalef)(_x, _y, _z)
    }
    unsafe fn glMultiTexCoord1fv(&self, _target: TextureUnit, _v: *const GLfloat) {
        (self.entry().glMultiTexCoord1fv)(_target, _v)
    }
    unsafe fn glNormal3b(&self, _nx: GLbyte, _ny: GLbyte, _nz: GLbyte) {
        (self.entry().glNormal3b)(_nx, _ny, _nz)
    }
    unsafe fn glVertexPointer(
        &self,
        _size: GLint,
        _type: VertexPointerType,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glVertexPointer)(_size, _type, _stride, _pointer)
    }
    unsafe fn glBindFramebuffer(&self, _target: FramebufferTarget, _framebuffer: GLuint) {
        (self.entry().glBindFramebuffer)(_target, _framebuffer)
    }
    unsafe fn glVertexAttrib4Nusv(&self, _index: GLuint, _v: *const GLushort) {
        (self.entry().glVertexAttrib4Nusv)(_index, _v)
    }
    unsafe fn glVertexAttribI2uiv(&self, _index: GLuint, _v: *const GLuint) {
        (self.entry().glVertexAttribI2uiv)(_index, _v)
    }
    unsafe fn glDrawBuffers(&self, _n: GLsizei, _bufs: *const DrawBufferMode) {
        (self.entry().glDrawBuffers)(_n, _bufs)
    }
    unsafe fn glVertex2fv(&self, _v: *const GLfloat) {
        (self.entry().glVertex2fv)(_v)
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
    unsafe fn glEvalCoord1dv(&self, _u: *const GLdouble) {
        (self.entry().glEvalCoord1dv)(_u)
    }
    unsafe fn glGetVertexAttribPointerv(
        &self,
        _index: GLuint,
        _pname: VertexAttribPointerPropertyARB,
        _pointer: *mut *mut std::os::raw::c_void,
    ) {
        (self.entry().glGetVertexAttribPointerv)(_index, _pname, _pointer)
    }
    unsafe fn glDeleteBuffers(&self, _n: GLsizei, _buffers: *const GLuint) {
        (self.entry().glDeleteBuffers)(_n, _buffers)
    }
    unsafe fn glRasterPos2i(&self, _x: GLint, _y: GLint) {
        (self.entry().glRasterPos2i)(_x, _y)
    }
    unsafe fn glFogfv(&self, _pname: FogParameter, _params: *const GLfloat) {
        (self.entry().glFogfv)(_pname, _params)
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
    unsafe fn glTexCoord2i(&self, _s: GLint, _t: GLint) {
        (self.entry().glTexCoord2i)(_s, _t)
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
    unsafe fn glMultiTexCoord3iv(&self, _target: TextureUnit, _v: *const GLint) {
        (self.entry().glMultiTexCoord3iv)(_target, _v)
    }
    unsafe fn glRectiv(&self, _v1: *const GLint, _v2: *const GLint) {
        (self.entry().glRectiv)(_v1, _v2)
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
