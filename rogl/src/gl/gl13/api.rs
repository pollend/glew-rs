use crate::gl::feature::EntryGLFn;
use crate::types::*;
pub trait GL13 {
    unsafe fn entry(&self) -> &EntryGLFn;
    unsafe fn glSelectBuffer(&self, _size: GLsizei, _buffer: *mut GLuint) {
        (self.entry().glSelectBuffer)(_size, _buffer)
    }
    unsafe fn glRectd(&self, _x1: GLdouble, _y1: GLdouble, _x2: GLdouble, _y2: GLdouble) {
        (self.entry().glRectd)(_x1, _y1, _x2, _y2)
    }
    unsafe fn glEvalCoord2fv(&self, _u: *const GLfloat) {
        (self.entry().glEvalCoord2fv)(_u)
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
    unsafe fn glPixelMapuiv(&self, _map: GLenum, _mapsize: GLsizei, _values: *const GLuint) {
        (self.entry().glPixelMapuiv)(_map, _mapsize, _values)
    }
    unsafe fn glIndexfv(&self, _c: *const GLfloat) {
        (self.entry().glIndexfv)(_c)
    }
    unsafe fn glGetPixelMapusv(&self, _map: GLenum, _values: *mut GLushort) {
        (self.entry().glGetPixelMapusv)(_map, _values)
    }
    unsafe fn glGetTexGeniv(&self, _coord: GLenum, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetTexGeniv)(_coord, _pname, _params)
    }
    unsafe fn glIndexsv(&self, _c: *const GLshort) {
        (self.entry().glIndexsv)(_c)
    }
    unsafe fn glTexCoord3d(&self, _s: GLdouble, _t: GLdouble, _r: GLdouble) {
        (self.entry().glTexCoord3d)(_s, _t, _r)
    }
    unsafe fn glClear(&self, _mask: GLbitfield) {
        (self.entry().glClear)(_mask)
    }
    unsafe fn glGetTexGendv(&self, _coord: GLenum, _pname: GLenum, _params: *mut GLdouble) {
        (self.entry().glGetTexGendv)(_coord, _pname, _params)
    }
    unsafe fn glLoadIdentity(&self) {
        (self.entry().glLoadIdentity)()
    }
    unsafe fn glTranslatef(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glTranslatef)(_x, _y, _z)
    }
    unsafe fn glEnableClientState(&self, _array: GLenum) {
        (self.entry().glEnableClientState)(_array)
    }
    unsafe fn glRasterPos2i(&self, _x: GLint, _y: GLint) {
        (self.entry().glRasterPos2i)(_x, _y)
    }
    unsafe fn glColor4b(&self, _red: GLbyte, _green: GLbyte, _blue: GLbyte, _alpha: GLbyte) {
        (self.entry().glColor4b)(_red, _green, _blue, _alpha)
    }
    unsafe fn glRasterPos3f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glRasterPos3f)(_x, _y, _z)
    }
    unsafe fn glRasterPos3sv(&self, _v: *const GLshort) {
        (self.entry().glRasterPos3sv)(_v)
    }
    unsafe fn glRecti(&self, _x1: GLint, _y1: GLint, _x2: GLint, _y2: GLint) {
        (self.entry().glRecti)(_x1, _y1, _x2, _y2)
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
    unsafe fn glMultiTexCoord2i(&self, _target: GLenum, _s: GLint, _t: GLint) {
        (self.entry().glMultiTexCoord2i)(_target, _s, _t)
    }
    unsafe fn glMultiTexCoord3iv(&self, _target: GLenum, _v: *const GLint) {
        (self.entry().glMultiTexCoord3iv)(_target, _v)
    }
    unsafe fn glMapGrid1f(&self, _un: GLint, _u1: GLfloat, _u2: GLfloat) {
        (self.entry().glMapGrid1f)(_un, _u1, _u2)
    }
    unsafe fn glIndexubv(&self, _c: *const GLubyte) {
        (self.entry().glIndexubv)(_c)
    }
    unsafe fn glNormal3b(&self, _nx: GLbyte, _ny: GLbyte, _nz: GLbyte) {
        (self.entry().glNormal3b)(_nx, _ny, _nz)
    }
    unsafe fn glColor4uiv(&self, _v: *const GLuint) {
        (self.entry().glColor4uiv)(_v)
    }
    unsafe fn glPopName(&self) {
        (self.entry().glPopName)()
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
    unsafe fn glTexCoord2fv(&self, _v: *const GLfloat) {
        (self.entry().glTexCoord2fv)(_v)
    }
    unsafe fn glLightiv(&self, _light: GLenum, _pname: GLenum, _params: *const GLint) {
        (self.entry().glLightiv)(_light, _pname, _params)
    }
    unsafe fn glFogi(&self, _pname: GLenum, _param: GLint) {
        (self.entry().glFogi)(_pname, _param)
    }
    unsafe fn glBlendFunc(&self, _sfactor: GLenum, _dfactor: GLenum) {
        (self.entry().glBlendFunc)(_sfactor, _dfactor)
    }
    unsafe fn glRasterPos2f(&self, _x: GLfloat, _y: GLfloat) {
        (self.entry().glRasterPos2f)(_x, _y)
    }
    unsafe fn glTexCoord2f(&self, _s: GLfloat, _t: GLfloat) {
        (self.entry().glTexCoord2f)(_s, _t)
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
    unsafe fn glGetBooleanv(&self, _pname: GLenum, _data: *mut GLboolean) {
        (self.entry().glGetBooleanv)(_pname, _data)
    }
    unsafe fn glColor3i(&self, _red: GLint, _green: GLint, _blue: GLint) {
        (self.entry().glColor3i)(_red, _green, _blue)
    }
    unsafe fn glLoadTransposeMatrixd(&self, _m: *const GLdouble) {
        (self.entry().glLoadTransposeMatrixd)(_m)
    }
    unsafe fn glIsList(&self, _list: GLuint) -> GLboolean {
        (self.entry().glIsList)(_list)
    }
    unsafe fn glTexEnvi(&self, _target: GLenum, _pname: GLenum, _param: GLint) {
        (self.entry().glTexEnvi)(_target, _pname, _param)
    }
    unsafe fn glRasterPos2sv(&self, _v: *const GLshort) {
        (self.entry().glRasterPos2sv)(_v)
    }
    unsafe fn glPushAttrib(&self, _mask: GLbitfield) {
        (self.entry().glPushAttrib)(_mask)
    }
    unsafe fn glPolygonMode(&self, _face: GLenum, _mode: GLenum) {
        (self.entry().glPolygonMode)(_face, _mode)
    }
    unsafe fn glColor3ubv(&self, _v: *const GLubyte) {
        (self.entry().glColor3ubv)(_v)
    }
    unsafe fn glEvalPoint1(&self, _i: GLint) {
        (self.entry().glEvalPoint1)(_i)
    }
    unsafe fn glColor4f(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat, _alpha: GLfloat) {
        (self.entry().glColor4f)(_red, _green, _blue, _alpha)
    }
    unsafe fn glEvalCoord2d(&self, _u: GLdouble, _v: GLdouble) {
        (self.entry().glEvalCoord2d)(_u, _v)
    }
    unsafe fn glVertex3f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glVertex3f)(_x, _y, _z)
    }
    unsafe fn glColor3bv(&self, _v: *const GLbyte) {
        (self.entry().glColor3bv)(_v)
    }
    unsafe fn glGetLightfv(&self, _light: GLenum, _pname: GLenum, _params: *mut GLfloat) {
        (self.entry().glGetLightfv)(_light, _pname, _params)
    }
    unsafe fn glRectsv(&self, _v1: *const GLshort, _v2: *const GLshort) {
        (self.entry().glRectsv)(_v1, _v2)
    }
    unsafe fn glNormal3bv(&self, _v: *const GLbyte) {
        (self.entry().glNormal3bv)(_v)
    }
    unsafe fn glTexCoord1iv(&self, _v: *const GLint) {
        (self.entry().glTexCoord1iv)(_v)
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
    unsafe fn glRasterPos4d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble, _w: GLdouble) {
        (self.entry().glRasterPos4d)(_x, _y, _z, _w)
    }
    unsafe fn glGetMapiv(&self, _target: GLenum, _query: GLenum, _v: *mut GLint) {
        (self.entry().glGetMapiv)(_target, _query, _v)
    }
    unsafe fn glVertex4f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat, _w: GLfloat) {
        (self.entry().glVertex4f)(_x, _y, _z, _w)
    }
    unsafe fn glRasterPos4sv(&self, _v: *const GLshort) {
        (self.entry().glRasterPos4sv)(_v)
    }
    unsafe fn glColor3usv(&self, _v: *const GLushort) {
        (self.entry().glColor3usv)(_v)
    }
    unsafe fn glGetPolygonStipple(&self, _mask: *mut GLubyte) {
        (self.entry().glGetPolygonStipple)(_mask)
    }
    unsafe fn glRotatef(&self, _angle: GLfloat, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glRotatef)(_angle, _x, _y, _z)
    }
    unsafe fn glGetTexParameteriv(&self, _target: GLenum, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetTexParameteriv)(_target, _pname, _params)
    }
    unsafe fn glColor4s(&self, _red: GLshort, _green: GLshort, _blue: GLshort, _alpha: GLshort) {
        (self.entry().glColor4s)(_red, _green, _blue, _alpha)
    }
    unsafe fn glDrawArrays(&self, _mode: GLenum, _first: GLint, _count: GLsizei) {
        (self.entry().glDrawArrays)(_mode, _first, _count)
    }
    unsafe fn glArrayElement(&self, _i: GLint) {
        (self.entry().glArrayElement)(_i)
    }
    unsafe fn glPixelStorei(&self, _pname: GLenum, _param: GLint) {
        (self.entry().glPixelStorei)(_pname, _param)
    }
    unsafe fn glPopClientAttrib(&self) {
        (self.entry().glPopClientAttrib)()
    }
    unsafe fn glMultiTexCoord1i(&self, _target: GLenum, _s: GLint) {
        (self.entry().glMultiTexCoord1i)(_target, _s)
    }
    unsafe fn glMultiTexCoord1iv(&self, _target: GLenum, _v: *const GLint) {
        (self.entry().glMultiTexCoord1iv)(_target, _v)
    }
    unsafe fn glMultiTexCoord2s(&self, _target: GLenum, _s: GLshort, _t: GLshort) {
        (self.entry().glMultiTexCoord2s)(_target, _s, _t)
    }
    unsafe fn glTexCoord2sv(&self, _v: *const GLshort) {
        (self.entry().glTexCoord2sv)(_v)
    }
    unsafe fn glIndexf(&self, _c: GLfloat) {
        (self.entry().glIndexf)(_c)
    }
    unsafe fn glRectiv(&self, _v1: *const GLint, _v2: *const GLint) {
        (self.entry().glRectiv)(_v1, _v2)
    }
    unsafe fn glRasterPos2dv(&self, _v: *const GLdouble) {
        (self.entry().glRasterPos2dv)(_v)
    }
    unsafe fn glVertex2s(&self, _x: GLshort, _y: GLshort) {
        (self.entry().glVertex2s)(_x, _y)
    }
    unsafe fn glVertex4iv(&self, _v: *const GLint) {
        (self.entry().glVertex4iv)(_v)
    }
    unsafe fn glColor3ub(&self, _red: GLubyte, _green: GLubyte, _blue: GLubyte) {
        (self.entry().glColor3ub)(_red, _green, _blue)
    }
    unsafe fn glStencilOp(&self, _fail: GLenum, _zfail: GLenum, _zpass: GLenum) {
        (self.entry().glStencilOp)(_fail, _zfail, _zpass)
    }
    unsafe fn glLightModelf(&self, _pname: GLenum, _param: GLfloat) {
        (self.entry().glLightModelf)(_pname, _param)
    }
    unsafe fn glTexEnvf(&self, _target: GLenum, _pname: GLenum, _param: GLfloat) {
        (self.entry().glTexEnvf)(_target, _pname, _param)
    }
    unsafe fn glVertex4fv(&self, _v: *const GLfloat) {
        (self.entry().glVertex4fv)(_v)
    }
    unsafe fn glBindTexture(&self, _target: GLenum, _texture: GLuint) {
        (self.entry().glBindTexture)(_target, _texture)
    }
    unsafe fn glAreTexturesResident(
        &self,
        _n: GLsizei,
        _textures: *const GLuint,
        _residences: *mut GLboolean,
    ) -> GLboolean {
        (self.entry().glAreTexturesResident)(_n, _textures, _residences)
    }
    unsafe fn glRasterPos2d(&self, _x: GLdouble, _y: GLdouble) {
        (self.entry().glRasterPos2d)(_x, _y)
    }
    unsafe fn glMultiTexCoord3d(&self, _target: GLenum, _s: GLdouble, _t: GLdouble, _r: GLdouble) {
        (self.entry().glMultiTexCoord3d)(_target, _s, _t, _r)
    }
    unsafe fn glTexParameteriv(&self, _target: GLenum, _pname: GLenum, _params: *const GLint) {
        (self.entry().glTexParameteriv)(_target, _pname, _params)
    }
    unsafe fn glPixelTransferf(&self, _pname: GLenum, _param: GLfloat) {
        (self.entry().glPixelTransferf)(_pname, _param)
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
    unsafe fn glTexCoord3dv(&self, _v: *const GLdouble) {
        (self.entry().glTexCoord3dv)(_v)
    }
    unsafe fn glRasterPos3iv(&self, _v: *const GLint) {
        (self.entry().glRasterPos3iv)(_v)
    }
    unsafe fn glLoadMatrixf(&self, _m: *const GLfloat) {
        (self.entry().glLoadMatrixf)(_m)
    }
    unsafe fn glColor4usv(&self, _v: *const GLushort) {
        (self.entry().glColor4usv)(_v)
    }
    unsafe fn glRasterPos3s(&self, _x: GLshort, _y: GLshort, _z: GLshort) {
        (self.entry().glRasterPos3s)(_x, _y, _z)
    }
    unsafe fn glRasterPos4dv(&self, _v: *const GLdouble) {
        (self.entry().glRasterPos4dv)(_v)
    }
    unsafe fn glVertex2i(&self, _x: GLint, _y: GLint) {
        (self.entry().glVertex2i)(_x, _y)
    }
    unsafe fn glColorMaterial(&self, _face: GLenum, _mode: GLenum) {
        (self.entry().glColorMaterial)(_face, _mode)
    }
    unsafe fn glDepthRange(&self, _n: GLdouble, _f: GLdouble) {
        (self.entry().glDepthRange)(_n, _f)
    }
    unsafe fn glMaterialiv(&self, _face: GLenum, _pname: GLenum, _params: *const GLint) {
        (self.entry().glMaterialiv)(_face, _pname, _params)
    }
    unsafe fn glGetClipPlane(&self, _plane: GLenum, _equation: *mut GLdouble) {
        (self.entry().glGetClipPlane)(_plane, _equation)
    }
    unsafe fn glNewList(&self, _list: GLuint, _mode: GLenum) {
        (self.entry().glNewList)(_list, _mode)
    }
    unsafe fn glIsEnabled(&self, _cap: GLenum) -> GLboolean {
        (self.entry().glIsEnabled)(_cap)
    }
    unsafe fn glTexCoord3f(&self, _s: GLfloat, _t: GLfloat, _r: GLfloat) {
        (self.entry().glTexCoord3f)(_s, _t, _r)
    }
    unsafe fn glGenLists(&self, _range: GLsizei) -> GLuint {
        (self.entry().glGenLists)(_range)
    }
    unsafe fn glVertex2dv(&self, _v: *const GLdouble) {
        (self.entry().glVertex2dv)(_v)
    }
    unsafe fn glMultiTexCoord3f(&self, _target: GLenum, _s: GLfloat, _t: GLfloat, _r: GLfloat) {
        (self.entry().glMultiTexCoord3f)(_target, _s, _t, _r)
    }
    unsafe fn glNormal3fv(&self, _v: *const GLfloat) {
        (self.entry().glNormal3fv)(_v)
    }
    unsafe fn glNormalPointer(
        &self,
        _type: GLenum,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glNormalPointer)(_type, _stride, _pointer)
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
    unsafe fn glTexCoord2d(&self, _s: GLdouble, _t: GLdouble) {
        (self.entry().glTexCoord2d)(_s, _t)
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
    unsafe fn glClientActiveTexture(&self, _texture: GLenum) {
        (self.entry().glClientActiveTexture)(_texture)
    }
    unsafe fn glTexCoord1d(&self, _s: GLdouble) {
        (self.entry().glTexCoord1d)(_s)
    }
    unsafe fn glRasterPos3i(&self, _x: GLint, _y: GLint, _z: GLint) {
        (self.entry().glRasterPos3i)(_x, _y, _z)
    }
    unsafe fn glAccum(&self, _op: GLenum, _value: GLfloat) {
        (self.entry().glAccum)(_op, _value)
    }
    unsafe fn glTexCoord2iv(&self, _v: *const GLint) {
        (self.entry().glTexCoord2iv)(_v)
    }
    unsafe fn glTexCoord4d(&self, _s: GLdouble, _t: GLdouble, _r: GLdouble, _q: GLdouble) {
        (self.entry().glTexCoord4d)(_s, _t, _r, _q)
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
    unsafe fn glClearDepth(&self, _depth: GLdouble) {
        (self.entry().glClearDepth)(_depth)
    }
    unsafe fn glFogfv(&self, _pname: GLenum, _params: *const GLfloat) {
        (self.entry().glFogfv)(_pname, _params)
    }
    unsafe fn glTexCoord1dv(&self, _v: *const GLdouble) {
        (self.entry().glTexCoord1dv)(_v)
    }
    unsafe fn glIndexi(&self, _c: GLint) {
        (self.entry().glIndexi)(_c)
    }
    unsafe fn glRasterPos3dv(&self, _v: *const GLdouble) {
        (self.entry().glRasterPos3dv)(_v)
    }
    unsafe fn glVertex3s(&self, _x: GLshort, _y: GLshort, _z: GLshort) {
        (self.entry().glVertex3s)(_x, _y, _z)
    }
    unsafe fn glVertex4i(&self, _x: GLint, _y: GLint, _z: GLint, _w: GLint) {
        (self.entry().glVertex4i)(_x, _y, _z, _w)
    }
    unsafe fn glTexCoord1sv(&self, _v: *const GLshort) {
        (self.entry().glTexCoord1sv)(_v)
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
    unsafe fn glMultiTexCoord1sv(&self, _target: GLenum, _v: *const GLshort) {
        (self.entry().glMultiTexCoord1sv)(_target, _v)
    }
    unsafe fn glColor3fv(&self, _v: *const GLfloat) {
        (self.entry().glColor3fv)(_v)
    }
    unsafe fn glTexGeniv(&self, _coord: GLenum, _pname: GLenum, _params: *const GLint) {
        (self.entry().glTexGeniv)(_coord, _pname, _params)
    }
    unsafe fn glNormal3d(&self, _nx: GLdouble, _ny: GLdouble, _nz: GLdouble) {
        (self.entry().glNormal3d)(_nx, _ny, _nz)
    }
    unsafe fn glIndexPointer(
        &self,
        _type: GLenum,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glIndexPointer)(_type, _stride, _pointer)
    }
    unsafe fn glRotated(&self, _angle: GLdouble, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glRotated)(_angle, _x, _y, _z)
    }
    unsafe fn glReadBuffer(&self, _src: GLenum) {
        (self.entry().glReadBuffer)(_src)
    }
    unsafe fn glColor4i(&self, _red: GLint, _green: GLint, _blue: GLint, _alpha: GLint) {
        (self.entry().glColor4i)(_red, _green, _blue, _alpha)
    }
    unsafe fn glVertex4dv(&self, _v: *const GLdouble) {
        (self.entry().glVertex4dv)(_v)
    }
    unsafe fn glIndexiv(&self, _c: *const GLint) {
        (self.entry().glIndexiv)(_c)
    }
    unsafe fn glDepthFunc(&self, _func: GLenum) {
        (self.entry().glDepthFunc)(_func)
    }
    unsafe fn glColor3uiv(&self, _v: *const GLuint) {
        (self.entry().glColor3uiv)(_v)
    }
    unsafe fn glCallLists(&self, _n: GLsizei, _type: GLenum, _lists: *const std::os::raw::c_void) {
        (self.entry().glCallLists)(_n, _type, _lists)
    }
    unsafe fn glTexCoord4f(&self, _s: GLfloat, _t: GLfloat, _r: GLfloat, _q: GLfloat) {
        (self.entry().glTexCoord4f)(_s, _t, _r, _q)
    }
    unsafe fn glPushName(&self, _name: GLuint) {
        (self.entry().glPushName)(_name)
    }
    unsafe fn glEvalCoord1d(&self, _u: GLdouble) {
        (self.entry().glEvalCoord1d)(_u)
    }
    unsafe fn glEdgeFlag(&self, _flag: GLboolean) {
        (self.entry().glEdgeFlag)(_flag)
    }
    unsafe fn glIndexd(&self, _c: GLdouble) {
        (self.entry().glIndexd)(_c)
    }
    unsafe fn glVertex4s(&self, _x: GLshort, _y: GLshort, _z: GLshort, _w: GLshort) {
        (self.entry().glVertex4s)(_x, _y, _z, _w)
    }
    unsafe fn glTexGenf(&self, _coord: GLenum, _pname: GLenum, _param: GLfloat) {
        (self.entry().glTexGenf)(_coord, _pname, _param)
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
    unsafe fn glPrioritizeTextures(
        &self,
        _n: GLsizei,
        _textures: *const GLuint,
        _priorities: *const GLfloat,
    ) {
        (self.entry().glPrioritizeTextures)(_n, _textures, _priorities)
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
    unsafe fn glMultiTexCoord2dv(&self, _target: GLenum, _v: *const GLdouble) {
        (self.entry().glMultiTexCoord2dv)(_target, _v)
    }
    unsafe fn glLineStipple(&self, _factor: GLint, _pattern: GLushort) {
        (self.entry().glLineStipple)(_factor, _pattern)
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
    unsafe fn glMultiTexCoord2f(&self, _target: GLenum, _s: GLfloat, _t: GLfloat) {
        (self.entry().glMultiTexCoord2f)(_target, _s, _t)
    }
    unsafe fn glLoadName(&self, _name: GLuint) {
        (self.entry().glLoadName)(_name)
    }
    unsafe fn glVertex3i(&self, _x: GLint, _y: GLint, _z: GLint) {
        (self.entry().glVertex3i)(_x, _y, _z)
    }
    unsafe fn glScaled(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glScaled)(_x, _y, _z)
    }
    unsafe fn glAlphaFunc(&self, _func: GLenum, _ref: GLfloat) {
        (self.entry().glAlphaFunc)(_func, _ref)
    }
    unsafe fn glCullFace(&self, _mode: GLenum) {
        (self.entry().glCullFace)(_mode)
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
    unsafe fn glMultiTexCoord1s(&self, _target: GLenum, _s: GLshort) {
        (self.entry().glMultiTexCoord1s)(_target, _s)
    }
    unsafe fn glMultiTexCoord3i(&self, _target: GLenum, _s: GLint, _t: GLint, _r: GLint) {
        (self.entry().glMultiTexCoord3i)(_target, _s, _t, _r)
    }
    unsafe fn glRectdv(&self, _v1: *const GLdouble, _v2: *const GLdouble) {
        (self.entry().glRectdv)(_v1, _v2)
    }
    unsafe fn glEnable(&self, _cap: GLenum) {
        (self.entry().glEnable)(_cap)
    }
    unsafe fn glTranslated(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glTranslated)(_x, _y, _z)
    }
    unsafe fn glTexCoord1f(&self, _s: GLfloat) {
        (self.entry().glTexCoord1f)(_s)
    }
    unsafe fn glIndexdv(&self, _c: *const GLdouble) {
        (self.entry().glIndexdv)(_c)
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
    unsafe fn glVertex2iv(&self, _v: *const GLint) {
        (self.entry().glVertex2iv)(_v)
    }
    unsafe fn glEvalPoint2(&self, _i: GLint, _j: GLint) {
        (self.entry().glEvalPoint2)(_i, _j)
    }
    unsafe fn glPixelTransferi(&self, _pname: GLenum, _param: GLint) {
        (self.entry().glPixelTransferi)(_pname, _param)
    }
    unsafe fn glDepthMask(&self, _flag: GLboolean) {
        (self.entry().glDepthMask)(_flag)
    }
    unsafe fn glColor4iv(&self, _v: *const GLint) {
        (self.entry().glColor4iv)(_v)
    }
    unsafe fn glPointSize(&self, _size: GLfloat) {
        (self.entry().glPointSize)(_size)
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
    unsafe fn glShadeModel(&self, _mode: GLenum) {
        (self.entry().glShadeModel)(_mode)
    }
    unsafe fn glIndexs(&self, _c: GLshort) {
        (self.entry().glIndexs)(_c)
    }
    unsafe fn glDeleteTextures(&self, _n: GLsizei, _textures: *const GLuint) {
        (self.entry().glDeleteTextures)(_n, _textures)
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
    unsafe fn glColor4ubv(&self, _v: *const GLubyte) {
        (self.entry().glColor4ubv)(_v)
    }
    unsafe fn glMaterialfv(&self, _face: GLenum, _pname: GLenum, _params: *const GLfloat) {
        (self.entry().glMaterialfv)(_face, _pname, _params)
    }
    unsafe fn glSampleCoverage(&self, _value: GLfloat, _invert: GLboolean) {
        (self.entry().glSampleCoverage)(_value, _invert)
    }
    unsafe fn glPixelMapfv(&self, _map: GLenum, _mapsize: GLsizei, _values: *const GLfloat) {
        (self.entry().glPixelMapfv)(_map, _mapsize, _values)
    }
    unsafe fn glGetTexEnviv(&self, _target: GLenum, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetTexEnviv)(_target, _pname, _params)
    }
    unsafe fn glActiveTexture(&self, _texture: GLenum) {
        (self.entry().glActiveTexture)(_texture)
    }
    unsafe fn glVertex2sv(&self, _v: *const GLshort) {
        (self.entry().glVertex2sv)(_v)
    }
    unsafe fn glColor3ui(&self, _red: GLuint, _green: GLuint, _blue: GLuint) {
        (self.entry().glColor3ui)(_red, _green, _blue)
    }
    unsafe fn glColor3dv(&self, _v: *const GLdouble) {
        (self.entry().glColor3dv)(_v)
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
    unsafe fn glMatrixMode(&self, _mode: GLenum) {
        (self.entry().glMatrixMode)(_mode)
    }
    unsafe fn glGetString(&self, _name: GLenum) -> *const GLubyte {
        (self.entry().glGetString)(_name)
    }
    unsafe fn glClearColor(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat, _alpha: GLfloat) {
        (self.entry().glClearColor)(_red, _green, _blue, _alpha)
    }
    unsafe fn glTexCoord1s(&self, _s: GLshort) {
        (self.entry().glTexCoord1s)(_s)
    }
    unsafe fn glClearAccum(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat, _alpha: GLfloat) {
        (self.entry().glClearAccum)(_red, _green, _blue, _alpha)
    }
    unsafe fn glClearIndex(&self, _c: GLfloat) {
        (self.entry().glClearIndex)(_c)
    }
    unsafe fn glEvalCoord1fv(&self, _u: *const GLfloat) {
        (self.entry().glEvalCoord1fv)(_u)
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
    unsafe fn glPopAttrib(&self) {
        (self.entry().glPopAttrib)()
    }
    unsafe fn glInterleavedArrays(
        &self,
        _format: GLenum,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glInterleavedArrays)(_format, _stride, _pointer)
    }
    unsafe fn glMultiTexCoord3fv(&self, _target: GLenum, _v: *const GLfloat) {
        (self.entry().glMultiTexCoord3fv)(_target, _v)
    }
    unsafe fn glGetPixelMapuiv(&self, _map: GLenum, _values: *mut GLuint) {
        (self.entry().glGetPixelMapuiv)(_map, _values)
    }
    unsafe fn glTexGend(&self, _coord: GLenum, _pname: GLenum, _param: GLdouble) {
        (self.entry().glTexGend)(_coord, _pname, _param)
    }
    unsafe fn glLoadMatrixd(&self, _m: *const GLdouble) {
        (self.entry().glLoadMatrixd)(_m)
    }
    unsafe fn glVertex3fv(&self, _v: *const GLfloat) {
        (self.entry().glVertex3fv)(_v)
    }
    unsafe fn glTexCoord3iv(&self, _v: *const GLint) {
        (self.entry().glTexCoord3iv)(_v)
    }
    unsafe fn glGetMapdv(&self, _target: GLenum, _query: GLenum, _v: *mut GLdouble) {
        (self.entry().glGetMapdv)(_target, _query, _v)
    }
    unsafe fn glFrontFace(&self, _mode: GLenum) {
        (self.entry().glFrontFace)(_mode)
    }
    unsafe fn glEndList(&self) {
        (self.entry().glEndList)()
    }
    unsafe fn glTexCoord1i(&self, _s: GLint) {
        (self.entry().glTexCoord1i)(_s)
    }
    unsafe fn glRasterPos4i(&self, _x: GLint, _y: GLint, _z: GLint, _w: GLint) {
        (self.entry().glRasterPos4i)(_x, _y, _z, _w)
    }
    unsafe fn glLightf(&self, _light: GLenum, _pname: GLenum, _param: GLfloat) {
        (self.entry().glLightf)(_light, _pname, _param)
    }
    unsafe fn glNormal3s(&self, _nx: GLshort, _ny: GLshort, _nz: GLshort) {
        (self.entry().glNormal3s)(_nx, _ny, _nz)
    }
    unsafe fn glPassThrough(&self, _token: GLfloat) {
        (self.entry().glPassThrough)(_token)
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
    unsafe fn glMultiTexCoord2sv(&self, _target: GLenum, _v: *const GLshort) {
        (self.entry().glMultiTexCoord2sv)(_target, _v)
    }
    unsafe fn glMultiTexCoord4iv(&self, _target: GLenum, _v: *const GLint) {
        (self.entry().glMultiTexCoord4iv)(_target, _v)
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
    unsafe fn glVertexPointer(
        &self,
        _size: GLint,
        _type: GLenum,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glVertexPointer)(_size, _type, _stride, _pointer)
    }
    unsafe fn glLineWidth(&self, _width: GLfloat) {
        (self.entry().glLineWidth)(_width)
    }
    unsafe fn glGetPixelMapfv(&self, _map: GLenum, _values: *mut GLfloat) {
        (self.entry().glGetPixelMapfv)(_map, _values)
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
    unsafe fn glMultiTexCoord2iv(&self, _target: GLenum, _v: *const GLint) {
        (self.entry().glMultiTexCoord2iv)(_target, _v)
    }
    unsafe fn glLightfv(&self, _light: GLenum, _pname: GLenum, _params: *const GLfloat) {
        (self.entry().glLightfv)(_light, _pname, _params)
    }
    unsafe fn glFeedbackBuffer(&self, _size: GLsizei, _type: GLenum, _buffer: *mut GLfloat) {
        (self.entry().glFeedbackBuffer)(_size, _type, _buffer)
    }
    unsafe fn glPixelMapusv(&self, _map: GLenum, _mapsize: GLsizei, _values: *const GLushort) {
        (self.entry().glPixelMapusv)(_map, _mapsize, _values)
    }
    unsafe fn glTexCoord4i(&self, _s: GLint, _t: GLint, _r: GLint, _q: GLint) {
        (self.entry().glTexCoord4i)(_s, _t, _r, _q)
    }
    unsafe fn glColor4sv(&self, _v: *const GLshort) {
        (self.entry().glColor4sv)(_v)
    }
    unsafe fn glTexParameterf(&self, _target: GLenum, _pname: GLenum, _param: GLfloat) {
        (self.entry().glTexParameterf)(_target, _pname, _param)
    }
    unsafe fn glTexParameteri(&self, _target: GLenum, _pname: GLenum, _param: GLint) {
        (self.entry().glTexParameteri)(_target, _pname, _param)
    }
    unsafe fn glMultiTexCoord1fv(&self, _target: GLenum, _v: *const GLfloat) {
        (self.entry().glMultiTexCoord1fv)(_target, _v)
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
    unsafe fn glMultiTexCoord2d(&self, _target: GLenum, _s: GLdouble, _t: GLdouble) {
        (self.entry().glMultiTexCoord2d)(_target, _s, _t)
    }
    unsafe fn glFogf(&self, _pname: GLenum, _param: GLfloat) {
        (self.entry().glFogf)(_pname, _param)
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
    unsafe fn glGetTexParameterfv(&self, _target: GLenum, _pname: GLenum, _params: *mut GLfloat) {
        (self.entry().glGetTexParameterfv)(_target, _pname, _params)
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
    unsafe fn glPushMatrix(&self) {
        (self.entry().glPushMatrix)()
    }
    unsafe fn glColor3b(&self, _red: GLbyte, _green: GLbyte, _blue: GLbyte) {
        (self.entry().glColor3b)(_red, _green, _blue)
    }
    unsafe fn glFogiv(&self, _pname: GLenum, _params: *const GLint) {
        (self.entry().glFogiv)(_pname, _params)
    }
    unsafe fn glEdgeFlagv(&self, _flag: *const GLboolean) {
        (self.entry().glEdgeFlagv)(_flag)
    }
    unsafe fn glGenTextures(&self, _n: GLsizei, _textures: *mut GLuint) {
        (self.entry().glGenTextures)(_n, _textures)
    }
    unsafe fn glTexCoord2i(&self, _s: GLint, _t: GLint) {
        (self.entry().glTexCoord2i)(_s, _t)
    }
    unsafe fn glPixelZoom(&self, _xfactor: GLfloat, _yfactor: GLfloat) {
        (self.entry().glPixelZoom)(_xfactor, _yfactor)
    }
    unsafe fn glRasterPos2s(&self, _x: GLshort, _y: GLshort) {
        (self.entry().glRasterPos2s)(_x, _y)
    }
    unsafe fn glGetMapfv(&self, _target: GLenum, _query: GLenum, _v: *mut GLfloat) {
        (self.entry().glGetMapfv)(_target, _query, _v)
    }
    unsafe fn glEdgeFlagPointer(&self, _stride: GLsizei, _pointer: *const std::os::raw::c_void) {
        (self.entry().glEdgeFlagPointer)(_stride, _pointer)
    }
    unsafe fn glPushClientAttrib(&self, _mask: GLbitfield) {
        (self.entry().glPushClientAttrib)(_mask)
    }
    unsafe fn glMultiTexCoord3s(&self, _target: GLenum, _s: GLshort, _t: GLshort, _r: GLshort) {
        (self.entry().glMultiTexCoord3s)(_target, _s, _t, _r)
    }
    unsafe fn glPopMatrix(&self) {
        (self.entry().glPopMatrix)()
    }
    unsafe fn glEvalCoord1f(&self, _u: GLfloat) {
        (self.entry().glEvalCoord1f)(_u)
    }
    unsafe fn glTexCoord4iv(&self, _v: *const GLint) {
        (self.entry().glTexCoord4iv)(_v)
    }
    unsafe fn glRasterPos4iv(&self, _v: *const GLint) {
        (self.entry().glRasterPos4iv)(_v)
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
    unsafe fn glGetFloatv(&self, _pname: GLenum, _data: *mut GLfloat) {
        (self.entry().glGetFloatv)(_pname, _data)
    }
    unsafe fn glGetMaterialfv(&self, _face: GLenum, _pname: GLenum, _params: *mut GLfloat) {
        (self.entry().glGetMaterialfv)(_face, _pname, _params)
    }
    unsafe fn glVertex3dv(&self, _v: *const GLdouble) {
        (self.entry().glVertex3dv)(_v)
    }
    unsafe fn glVertex4sv(&self, _v: *const GLshort) {
        (self.entry().glVertex4sv)(_v)
    }
    unsafe fn glNormal3sv(&self, _v: *const GLshort) {
        (self.entry().glNormal3sv)(_v)
    }
    unsafe fn glStencilFunc(&self, _func: GLenum, _ref: GLint, _mask: GLuint) {
        (self.entry().glStencilFunc)(_func, _ref, _mask)
    }
    unsafe fn glVertex2fv(&self, _v: *const GLfloat) {
        (self.entry().glVertex2fv)(_v)
    }
    unsafe fn glMultTransposeMatrixf(&self, _m: *const GLfloat) {
        (self.entry().glMultTransposeMatrixf)(_m)
    }
    unsafe fn glDeleteLists(&self, _list: GLuint, _range: GLsizei) {
        (self.entry().glDeleteLists)(_list, _range)
    }
    unsafe fn glListBase(&self, _base: GLuint) {
        (self.entry().glListBase)(_base)
    }
    unsafe fn glGetIntegerv(&self, _pname: GLenum, _data: *mut GLint) {
        (self.entry().glGetIntegerv)(_pname, _data)
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
    unsafe fn glGetMaterialiv(&self, _face: GLenum, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetMaterialiv)(_face, _pname, _params)
    }
    unsafe fn glTexGeni(&self, _coord: GLenum, _pname: GLenum, _param: GLint) {
        (self.entry().glTexGeni)(_coord, _pname, _param)
    }
    unsafe fn glMultMatrixf(&self, _m: *const GLfloat) {
        (self.entry().glMultMatrixf)(_m)
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
    unsafe fn glTexCoord1fv(&self, _v: *const GLfloat) {
        (self.entry().glTexCoord1fv)(_v)
    }
    unsafe fn glLoadTransposeMatrixf(&self, _m: *const GLfloat) {
        (self.entry().glLoadTransposeMatrixf)(_m)
    }
    unsafe fn glEvalMesh2(&self, _mode: GLenum, _i1: GLint, _i2: GLint, _j1: GLint, _j2: GLint) {
        (self.entry().glEvalMesh2)(_mode, _i1, _i2, _j1, _j2)
    }
    unsafe fn glRasterPos4s(&self, _x: GLshort, _y: GLshort, _z: GLshort, _w: GLshort) {
        (self.entry().glRasterPos4s)(_x, _y, _z, _w)
    }
    unsafe fn glTexCoord3i(&self, _s: GLint, _t: GLint, _r: GLint) {
        (self.entry().glTexCoord3i)(_s, _t, _r)
    }
    unsafe fn glMultiTexCoord4sv(&self, _target: GLenum, _v: *const GLshort) {
        (self.entry().glMultiTexCoord4sv)(_target, _v)
    }
    unsafe fn glColor3f(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat) {
        (self.entry().glColor3f)(_red, _green, _blue)
    }
    unsafe fn glTexGenfv(&self, _coord: GLenum, _pname: GLenum, _params: *const GLfloat) {
        (self.entry().glTexGenfv)(_coord, _pname, _params)
    }
    unsafe fn glNormal3iv(&self, _v: *const GLint) {
        (self.entry().glNormal3iv)(_v)
    }
    unsafe fn glRasterPos2iv(&self, _v: *const GLint) {
        (self.entry().glRasterPos2iv)(_v)
    }
    unsafe fn glLogicOp(&self, _opcode: GLenum) {
        (self.entry().glLogicOp)(_opcode)
    }
    unsafe fn glRasterPos3d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glRasterPos3d)(_x, _y, _z)
    }
    unsafe fn glNormal3f(&self, _nx: GLfloat, _ny: GLfloat, _nz: GLfloat) {
        (self.entry().glNormal3f)(_nx, _ny, _nz)
    }
    unsafe fn glRasterPos4fv(&self, _v: *const GLfloat) {
        (self.entry().glRasterPos4fv)(_v)
    }
    unsafe fn glVertex3sv(&self, _v: *const GLshort) {
        (self.entry().glVertex3sv)(_v)
    }
    unsafe fn glGetError(&self) -> GLenum {
        (self.entry().glGetError)()
    }
    unsafe fn glIndexMask(&self, _mask: GLuint) {
        (self.entry().glIndexMask)(_mask)
    }
    unsafe fn glClearStencil(&self, _s: GLint) {
        (self.entry().glClearStencil)(_s)
    }
    unsafe fn glGetTexEnvfv(&self, _target: GLenum, _pname: GLenum, _params: *mut GLfloat) {
        (self.entry().glGetTexEnvfv)(_target, _pname, _params)
    }
    unsafe fn glInitNames(&self) {
        (self.entry().glInitNames)()
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
    unsafe fn glMultTransposeMatrixd(&self, _m: *const GLdouble) {
        (self.entry().glMultTransposeMatrixd)(_m)
    }
    unsafe fn glStencilMask(&self, _mask: GLuint) {
        (self.entry().glStencilMask)(_mask)
    }
    unsafe fn glScalef(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glScalef)(_x, _y, _z)
    }
    unsafe fn glIsTexture(&self, _texture: GLuint) -> GLboolean {
        (self.entry().glIsTexture)(_texture)
    }
    unsafe fn glColor3sv(&self, _v: *const GLshort) {
        (self.entry().glColor3sv)(_v)
    }
    unsafe fn glCallList(&self, _list: GLuint) {
        (self.entry().glCallList)(_list)
    }
    unsafe fn glMultMatrixd(&self, _m: *const GLdouble) {
        (self.entry().glMultMatrixd)(_m)
    }
    unsafe fn glTexCoord2dv(&self, _v: *const GLdouble) {
        (self.entry().glTexCoord2dv)(_v)
    }
    unsafe fn glRasterPos3fv(&self, _v: *const GLfloat) {
        (self.entry().glRasterPos3fv)(_v)
    }
    unsafe fn glTexCoord4fv(&self, _v: *const GLfloat) {
        (self.entry().glTexCoord4fv)(_v)
    }
    unsafe fn glMateriali(&self, _face: GLenum, _pname: GLenum, _param: GLint) {
        (self.entry().glMateriali)(_face, _pname, _param)
    }
    unsafe fn glMapGrid1d(&self, _un: GLint, _u1: GLdouble, _u2: GLdouble) {
        (self.entry().glMapGrid1d)(_un, _u1, _u2)
    }
    unsafe fn glEvalCoord1dv(&self, _u: *const GLdouble) {
        (self.entry().glEvalCoord1dv)(_u)
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
    unsafe fn glRenderMode(&self, _mode: GLenum) -> GLint {
        (self.entry().glRenderMode)(_mode)
    }
    unsafe fn glHint(&self, _target: GLenum, _mode: GLenum) {
        (self.entry().glHint)(_target, _mode)
    }
    unsafe fn glRectfv(&self, _v1: *const GLfloat, _v2: *const GLfloat) {
        (self.entry().glRectfv)(_v1, _v2)
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
    unsafe fn glBegin(&self, _mode: GLenum) {
        (self.entry().glBegin)(_mode)
    }
    unsafe fn glEvalMesh1(&self, _mode: GLenum, _i1: GLint, _i2: GLint) {
        (self.entry().glEvalMesh1)(_mode, _i1, _i2)
    }
    unsafe fn glColor4ui(&self, _red: GLuint, _green: GLuint, _blue: GLuint, _alpha: GLuint) {
        (self.entry().glColor4ui)(_red, _green, _blue, _alpha)
    }
    unsafe fn glGetLightiv(&self, _light: GLenum, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetLightiv)(_light, _pname, _params)
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
    unsafe fn glTexGendv(&self, _coord: GLenum, _pname: GLenum, _params: *const GLdouble) {
        (self.entry().glTexGendv)(_coord, _pname, _params)
    }
    unsafe fn glMultiTexCoord4dv(&self, _target: GLenum, _v: *const GLdouble) {
        (self.entry().glMultiTexCoord4dv)(_target, _v)
    }
    unsafe fn glRasterPos2fv(&self, _v: *const GLfloat) {
        (self.entry().glRasterPos2fv)(_v)
    }
    unsafe fn glRectf(&self, _x1: GLfloat, _y1: GLfloat, _x2: GLfloat, _y2: GLfloat) {
        (self.entry().glRectf)(_x1, _y1, _x2, _y2)
    }
    unsafe fn glTexCoord2s(&self, _s: GLshort, _t: GLshort) {
        (self.entry().glTexCoord2s)(_s, _t)
    }
    unsafe fn glPixelStoref(&self, _pname: GLenum, _param: GLfloat) {
        (self.entry().glPixelStoref)(_pname, _param)
    }
    unsafe fn glScissor(&self, _x: GLint, _y: GLint, _width: GLsizei, _height: GLsizei) {
        (self.entry().glScissor)(_x, _y, _width, _height)
    }
    unsafe fn glRasterPos4f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat, _w: GLfloat) {
        (self.entry().glRasterPos4f)(_x, _y, _z, _w)
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
    unsafe fn glTexEnviv(&self, _target: GLenum, _pname: GLenum, _params: *const GLint) {
        (self.entry().glTexEnviv)(_target, _pname, _params)
    }
    unsafe fn glColor4bv(&self, _v: *const GLbyte) {
        (self.entry().glColor4bv)(_v)
    }
    unsafe fn glVertex4d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble, _w: GLdouble) {
        (self.entry().glVertex4d)(_x, _y, _z, _w)
    }
    unsafe fn glColor3s(&self, _red: GLshort, _green: GLshort, _blue: GLshort) {
        (self.entry().glColor3s)(_red, _green, _blue)
    }
    unsafe fn glMultiTexCoord1f(&self, _target: GLenum, _s: GLfloat) {
        (self.entry().glMultiTexCoord1f)(_target, _s)
    }
    unsafe fn glMultiTexCoord2fv(&self, _target: GLenum, _v: *const GLfloat) {
        (self.entry().glMultiTexCoord2fv)(_target, _v)
    }
    unsafe fn glEvalCoord2f(&self, _u: GLfloat, _v: GLfloat) {
        (self.entry().glEvalCoord2f)(_u, _v)
    }
    unsafe fn glMultiTexCoord1dv(&self, _target: GLenum, _v: *const GLdouble) {
        (self.entry().glMultiTexCoord1dv)(_target, _v)
    }
    unsafe fn glTexCoord3sv(&self, _v: *const GLshort) {
        (self.entry().glTexCoord3sv)(_v)
    }
    unsafe fn glMultiTexCoord1d(&self, _target: GLenum, _s: GLdouble) {
        (self.entry().glMultiTexCoord1d)(_target, _s)
    }
    unsafe fn glTexCoord4s(&self, _s: GLshort, _t: GLshort, _r: GLshort, _q: GLshort) {
        (self.entry().glTexCoord4s)(_s, _t, _r, _q)
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
    unsafe fn glGetPointerv(&self, _pname: GLenum, _params: *mut *mut std::os::raw::c_void) {
        (self.entry().glGetPointerv)(_pname, _params)
    }
    unsafe fn glNormal3dv(&self, _v: *const GLdouble) {
        (self.entry().glNormal3dv)(_v)
    }
    unsafe fn glVertex3d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glVertex3d)(_x, _y, _z)
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
    unsafe fn glColor3iv(&self, _v: *const GLint) {
        (self.entry().glColor3iv)(_v)
    }
    unsafe fn glVertex2f(&self, _x: GLfloat, _y: GLfloat) {
        (self.entry().glVertex2f)(_x, _y)
    }
    unsafe fn glEvalCoord2dv(&self, _u: *const GLdouble) {
        (self.entry().glEvalCoord2dv)(_u)
    }
    unsafe fn glGetTexGenfv(&self, _coord: GLenum, _pname: GLenum, _params: *mut GLfloat) {
        (self.entry().glGetTexGenfv)(_coord, _pname, _params)
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
    unsafe fn glDisable(&self, _cap: GLenum) {
        (self.entry().glDisable)(_cap)
    }
    unsafe fn glIndexub(&self, _c: GLubyte) {
        (self.entry().glIndexub)(_c)
    }
    unsafe fn glEnd(&self) {
        (self.entry().glEnd)()
    }
    unsafe fn glTexCoord3fv(&self, _v: *const GLfloat) {
        (self.entry().glTexCoord3fv)(_v)
    }
    unsafe fn glColor3d(&self, _red: GLdouble, _green: GLdouble, _blue: GLdouble) {
        (self.entry().glColor3d)(_red, _green, _blue)
    }
    unsafe fn glVertex2d(&self, _x: GLdouble, _y: GLdouble) {
        (self.entry().glVertex2d)(_x, _y)
    }
    unsafe fn glMaterialf(&self, _face: GLenum, _pname: GLenum, _param: GLfloat) {
        (self.entry().glMaterialf)(_face, _pname, _param)
    }
    unsafe fn glMultiTexCoord4fv(&self, _target: GLenum, _v: *const GLfloat) {
        (self.entry().glMultiTexCoord4fv)(_target, _v)
    }
    unsafe fn glTexCoord3s(&self, _s: GLshort, _t: GLshort, _r: GLshort) {
        (self.entry().glTexCoord3s)(_s, _t, _r)
    }
    unsafe fn glGetCompressedTexImage(
        &self,
        _target: GLenum,
        _level: GLint,
        _img: *mut std::os::raw::c_void,
    ) {
        (self.entry().glGetCompressedTexImage)(_target, _level, _img)
    }
    unsafe fn glTexParameterfv(&self, _target: GLenum, _pname: GLenum, _params: *const GLfloat) {
        (self.entry().glTexParameterfv)(_target, _pname, _params)
    }
    unsafe fn glLightModeli(&self, _pname: GLenum, _param: GLint) {
        (self.entry().glLightModeli)(_pname, _param)
    }
    unsafe fn glGetDoublev(&self, _pname: GLenum, _data: *mut GLdouble) {
        (self.entry().glGetDoublev)(_pname, _data)
    }
    unsafe fn glColor4ub(&self, _red: GLubyte, _green: GLubyte, _blue: GLubyte, _alpha: GLubyte) {
        (self.entry().glColor4ub)(_red, _green, _blue, _alpha)
    }
    unsafe fn glColor4dv(&self, _v: *const GLdouble) {
        (self.entry().glColor4dv)(_v)
    }
    unsafe fn glLighti(&self, _light: GLenum, _pname: GLenum, _param: GLint) {
        (self.entry().glLighti)(_light, _pname, _param)
    }
    unsafe fn glDisableClientState(&self, _array: GLenum) {
        (self.entry().glDisableClientState)(_array)
    }
    unsafe fn glMultiTexCoord3sv(&self, _target: GLenum, _v: *const GLshort) {
        (self.entry().glMultiTexCoord3sv)(_target, _v)
    }
    unsafe fn glFinish(&self) {
        (self.entry().glFinish)()
    }
    unsafe fn glFlush(&self) {
        (self.entry().glFlush)()
    }
    unsafe fn glTexCoord4sv(&self, _v: *const GLshort) {
        (self.entry().glTexCoord4sv)(_v)
    }
    unsafe fn glLightModeliv(&self, _pname: GLenum, _params: *const GLint) {
        (self.entry().glLightModeliv)(_pname, _params)
    }
    unsafe fn glMultiTexCoord3dv(&self, _target: GLenum, _v: *const GLdouble) {
        (self.entry().glMultiTexCoord3dv)(_target, _v)
    }
    unsafe fn glRects(&self, _x1: GLshort, _y1: GLshort, _x2: GLshort, _y2: GLshort) {
        (self.entry().glRects)(_x1, _y1, _x2, _y2)
    }
    unsafe fn glPolygonOffset(&self, _factor: GLfloat, _units: GLfloat) {
        (self.entry().glPolygonOffset)(_factor, _units)
    }
    unsafe fn glNormal3i(&self, _nx: GLint, _ny: GLint, _nz: GLint) {
        (self.entry().glNormal3i)(_nx, _ny, _nz)
    }
    unsafe fn glClipPlane(&self, _plane: GLenum, _equation: *const GLdouble) {
        (self.entry().glClipPlane)(_plane, _equation)
    }
    unsafe fn glDrawBuffer(&self, _buf: GLenum) {
        (self.entry().glDrawBuffer)(_buf)
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
    unsafe fn glPolygonStipple(&self, _mask: *const GLubyte) {
        (self.entry().glPolygonStipple)(_mask)
    }
    unsafe fn glTexEnvfv(&self, _target: GLenum, _pname: GLenum, _params: *const GLfloat) {
        (self.entry().glTexEnvfv)(_target, _pname, _params)
    }
    unsafe fn glTexCoord4dv(&self, _v: *const GLdouble) {
        (self.entry().glTexCoord4dv)(_v)
    }
    unsafe fn glColor3us(&self, _red: GLushort, _green: GLushort, _blue: GLushort) {
        (self.entry().glColor3us)(_red, _green, _blue)
    }
    unsafe fn glColor4fv(&self, _v: *const GLfloat) {
        (self.entry().glColor4fv)(_v)
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
    unsafe fn glLightModelfv(&self, _pname: GLenum, _params: *const GLfloat) {
        (self.entry().glLightModelfv)(_pname, _params)
    }
    unsafe fn glViewport(&self, _x: GLint, _y: GLint, _width: GLsizei, _height: GLsizei) {
        (self.entry().glViewport)(_x, _y, _width, _height)
    }
    unsafe fn glVertex3iv(&self, _v: *const GLint) {
        (self.entry().glVertex3iv)(_v)
    }
}
