use crate::gl::feature::EntryGLFn;
use crate::types::*;
pub trait GL10 {
    unsafe fn entry(&self) -> &EntryGLFn;
    unsafe fn glRasterPos2sv(&self, _v: *const GLshort) {
        (self.entry().glRasterPos2sv)(_v)
    }
    unsafe fn glGetMapiv(&self, _target: GLenum, _query: GLenum, _v: *mut GLint) {
        (self.entry().glGetMapiv)(_target, _query, _v)
    }
    unsafe fn glRotated(&self, _angle: GLdouble, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glRotated)(_angle, _x, _y, _z)
    }
    unsafe fn glTexParameterf(&self, _target: GLenum, _pname: GLenum, _param: GLfloat) {
        (self.entry().glTexParameterf)(_target, _pname, _param)
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
    unsafe fn glRasterPos3f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glRasterPos3f)(_x, _y, _z)
    }
    unsafe fn glVertex4fv(&self, _v: *const GLfloat) {
        (self.entry().glVertex4fv)(_v)
    }
    unsafe fn glNormal3bv(&self, _v: *const GLbyte) {
        (self.entry().glNormal3bv)(_v)
    }
    unsafe fn glTexCoord4dv(&self, _v: *const GLdouble) {
        (self.entry().glTexCoord4dv)(_v)
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
    unsafe fn glTexCoord2i(&self, _s: GLint, _t: GLint) {
        (self.entry().glTexCoord2i)(_s, _t)
    }
    unsafe fn glRasterPos4d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble, _w: GLdouble) {
        (self.entry().glRasterPos4d)(_x, _y, _z, _w)
    }
    unsafe fn glColor3f(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat) {
        (self.entry().glColor3f)(_red, _green, _blue)
    }
    unsafe fn glNormal3d(&self, _nx: GLdouble, _ny: GLdouble, _nz: GLdouble) {
        (self.entry().glNormal3d)(_nx, _ny, _nz)
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
    unsafe fn glGetPixelMapusv(&self, _map: GLenum, _values: *mut GLushort) {
        (self.entry().glGetPixelMapusv)(_map, _values)
    }
    unsafe fn glTexCoord4i(&self, _s: GLint, _t: GLint, _r: GLint, _q: GLint) {
        (self.entry().glTexCoord4i)(_s, _t, _r, _q)
    }
    unsafe fn glRasterPos3d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glRasterPos3d)(_x, _y, _z)
    }
    unsafe fn glClearColor(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat, _alpha: GLfloat) {
        (self.entry().glClearColor)(_red, _green, _blue, _alpha)
    }
    unsafe fn glDepthFunc(&self, _func: GLenum) {
        (self.entry().glDepthFunc)(_func)
    }
    unsafe fn glViewport(&self, _x: GLint, _y: GLint, _width: GLsizei, _height: GLsizei) {
        (self.entry().glViewport)(_x, _y, _width, _height)
    }
    unsafe fn glTexCoord3f(&self, _s: GLfloat, _t: GLfloat, _r: GLfloat) {
        (self.entry().glTexCoord3f)(_s, _t, _r)
    }
    unsafe fn glTexGenf(&self, _coord: GLenum, _pname: GLenum, _param: GLfloat) {
        (self.entry().glTexGenf)(_coord, _pname, _param)
    }
    unsafe fn glIndexi(&self, _c: GLint) {
        (self.entry().glIndexi)(_c)
    }
    unsafe fn glTexCoord2d(&self, _s: GLdouble, _t: GLdouble) {
        (self.entry().glTexCoord2d)(_s, _t)
    }
    unsafe fn glGetLightfv(&self, _light: GLenum, _pname: GLenum, _params: *mut GLfloat) {
        (self.entry().glGetLightfv)(_light, _pname, _params)
    }
    unsafe fn glDisable(&self, _cap: GLenum) {
        (self.entry().glDisable)(_cap)
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
    unsafe fn glColor3bv(&self, _v: *const GLbyte) {
        (self.entry().glColor3bv)(_v)
    }
    unsafe fn glRasterPos4sv(&self, _v: *const GLshort) {
        (self.entry().glRasterPos4sv)(_v)
    }
    unsafe fn glVertex2s(&self, _x: GLshort, _y: GLshort) {
        (self.entry().glVertex2s)(_x, _y)
    }
    unsafe fn glVertex2iv(&self, _v: *const GLint) {
        (self.entry().glVertex2iv)(_v)
    }
    unsafe fn glColor3uiv(&self, _v: *const GLuint) {
        (self.entry().glColor3uiv)(_v)
    }
    unsafe fn glGetIntegerv(&self, _pname: GLenum, _data: *mut GLint) {
        (self.entry().glGetIntegerv)(_pname, _data)
    }
    unsafe fn glColor4s(&self, _red: GLshort, _green: GLshort, _blue: GLshort, _alpha: GLshort) {
        (self.entry().glColor4s)(_red, _green, _blue, _alpha)
    }
    unsafe fn glColor4bv(&self, _v: *const GLbyte) {
        (self.entry().glColor4bv)(_v)
    }
    unsafe fn glRasterPos3iv(&self, _v: *const GLint) {
        (self.entry().glRasterPos3iv)(_v)
    }
    unsafe fn glTexCoord1s(&self, _s: GLshort) {
        (self.entry().glTexCoord1s)(_s)
    }
    unsafe fn glScissor(&self, _x: GLint, _y: GLint, _width: GLsizei, _height: GLsizei) {
        (self.entry().glScissor)(_x, _y, _width, _height)
    }
    unsafe fn glTexCoord2iv(&self, _v: *const GLint) {
        (self.entry().glTexCoord2iv)(_v)
    }
    unsafe fn glTexCoord3s(&self, _s: GLshort, _t: GLshort, _r: GLshort) {
        (self.entry().glTexCoord3s)(_s, _t, _r)
    }
    unsafe fn glVertex3iv(&self, _v: *const GLint) {
        (self.entry().glVertex3iv)(_v)
    }
    unsafe fn glLightModelf(&self, _pname: GLenum, _param: GLfloat) {
        (self.entry().glLightModelf)(_pname, _param)
    }
    unsafe fn glGetLightiv(&self, _light: GLenum, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetLightiv)(_light, _pname, _params)
    }
    unsafe fn glMultMatrixf(&self, _m: *const GLfloat) {
        (self.entry().glMultMatrixf)(_m)
    }
    unsafe fn glPopMatrix(&self) {
        (self.entry().glPopMatrix)()
    }
    unsafe fn glClearAccum(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat, _alpha: GLfloat) {
        (self.entry().glClearAccum)(_red, _green, _blue, _alpha)
    }
    unsafe fn glPolygonMode(&self, _face: GLenum, _mode: GLenum) {
        (self.entry().glPolygonMode)(_face, _mode)
    }
    unsafe fn glVertex2dv(&self, _v: *const GLdouble) {
        (self.entry().glVertex2dv)(_v)
    }
    unsafe fn glMaterialf(&self, _face: GLenum, _pname: GLenum, _param: GLfloat) {
        (self.entry().glMaterialf)(_face, _pname, _param)
    }
    unsafe fn glGetClipPlane(&self, _plane: GLenum, _equation: *mut GLdouble) {
        (self.entry().glGetClipPlane)(_plane, _equation)
    }
    unsafe fn glTranslatef(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glTranslatef)(_x, _y, _z)
    }
    unsafe fn glRasterPos4dv(&self, _v: *const GLdouble) {
        (self.entry().glRasterPos4dv)(_v)
    }
    unsafe fn glGetPolygonStipple(&self, _mask: *mut GLubyte) {
        (self.entry().glGetPolygonStipple)(_mask)
    }
    unsafe fn glVertex3d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glVertex3d)(_x, _y, _z)
    }
    unsafe fn glBegin(&self, _mode: GLenum) {
        (self.entry().glBegin)(_mode)
    }
    unsafe fn glDeleteLists(&self, _list: GLuint, _range: GLsizei) {
        (self.entry().glDeleteLists)(_list, _range)
    }
    unsafe fn glGetError(&self) -> GLenum {
        (self.entry().glGetError)()
    }
    unsafe fn glColor3s(&self, _red: GLshort, _green: GLshort, _blue: GLshort) {
        (self.entry().glColor3s)(_red, _green, _blue)
    }
    unsafe fn glLogicOp(&self, _opcode: GLenum) {
        (self.entry().glLogicOp)(_opcode)
    }
    unsafe fn glIsEnabled(&self, _cap: GLenum) -> GLboolean {
        (self.entry().glIsEnabled)(_cap)
    }
    unsafe fn glTexCoord1sv(&self, _v: *const GLshort) {
        (self.entry().glTexCoord1sv)(_v)
    }
    unsafe fn glNormal3fv(&self, _v: *const GLfloat) {
        (self.entry().glNormal3fv)(_v)
    }
    unsafe fn glTexCoord3sv(&self, _v: *const GLshort) {
        (self.entry().glTexCoord3sv)(_v)
    }
    unsafe fn glGetTexEnviv(&self, _target: GLenum, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetTexEnviv)(_target, _pname, _params)
    }
    unsafe fn glGetFloatv(&self, _pname: GLenum, _data: *mut GLfloat) {
        (self.entry().glGetFloatv)(_pname, _data)
    }
    unsafe fn glIndexf(&self, _c: GLfloat) {
        (self.entry().glIndexf)(_c)
    }
    unsafe fn glRects(&self, _x1: GLshort, _y1: GLshort, _x2: GLshort, _y2: GLshort) {
        (self.entry().glRects)(_x1, _y1, _x2, _y2)
    }
    unsafe fn glVertex4f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat, _w: GLfloat) {
        (self.entry().glVertex4f)(_x, _y, _z, _w)
    }
    unsafe fn glPushMatrix(&self) {
        (self.entry().glPushMatrix)()
    }
    unsafe fn glRecti(&self, _x1: GLint, _y1: GLint, _x2: GLint, _y2: GLint) {
        (self.entry().glRecti)(_x1, _y1, _x2, _y2)
    }
    unsafe fn glShadeModel(&self, _mode: GLenum) {
        (self.entry().glShadeModel)(_mode)
    }
    unsafe fn glVertex3s(&self, _x: GLshort, _y: GLshort, _z: GLshort) {
        (self.entry().glVertex3s)(_x, _y, _z)
    }
    unsafe fn glVertex4s(&self, _x: GLshort, _y: GLshort, _z: GLshort, _w: GLshort) {
        (self.entry().glVertex4s)(_x, _y, _z, _w)
    }
    unsafe fn glLineWidth(&self, _width: GLfloat) {
        (self.entry().glLineWidth)(_width)
    }
    unsafe fn glGetPixelMapuiv(&self, _map: GLenum, _values: *mut GLuint) {
        (self.entry().glGetPixelMapuiv)(_map, _values)
    }
    unsafe fn glPassThrough(&self, _token: GLfloat) {
        (self.entry().glPassThrough)(_token)
    }
    unsafe fn glGetMaterialfv(&self, _face: GLenum, _pname: GLenum, _params: *mut GLfloat) {
        (self.entry().glGetMaterialfv)(_face, _pname, _params)
    }
    unsafe fn glPopName(&self) {
        (self.entry().glPopName)()
    }
    unsafe fn glLightModeliv(&self, _pname: GLenum, _params: *const GLint) {
        (self.entry().glLightModeliv)(_pname, _params)
    }
    unsafe fn glIndexd(&self, _c: GLdouble) {
        (self.entry().glIndexd)(_c)
    }
    unsafe fn glAlphaFunc(&self, _func: GLenum, _ref: GLfloat) {
        (self.entry().glAlphaFunc)(_func, _ref)
    }
    unsafe fn glPixelZoom(&self, _xfactor: GLfloat, _yfactor: GLfloat) {
        (self.entry().glPixelZoom)(_xfactor, _yfactor)
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
    unsafe fn glNormal3s(&self, _nx: GLshort, _ny: GLshort, _nz: GLshort) {
        (self.entry().glNormal3s)(_nx, _ny, _nz)
    }
    unsafe fn glRasterPos4f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat, _w: GLfloat) {
        (self.entry().glRasterPos4f)(_x, _y, _z, _w)
    }
    unsafe fn glIndexMask(&self, _mask: GLuint) {
        (self.entry().glIndexMask)(_mask)
    }
    unsafe fn glPolygonStipple(&self, _mask: *const GLubyte) {
        (self.entry().glPolygonStipple)(_mask)
    }
    unsafe fn glRectf(&self, _x1: GLfloat, _y1: GLfloat, _x2: GLfloat, _y2: GLfloat) {
        (self.entry().glRectf)(_x1, _y1, _x2, _y2)
    }
    unsafe fn glFogi(&self, _pname: GLenum, _param: GLint) {
        (self.entry().glFogi)(_pname, _param)
    }
    unsafe fn glColor3d(&self, _red: GLdouble, _green: GLdouble, _blue: GLdouble) {
        (self.entry().glColor3d)(_red, _green, _blue)
    }
    unsafe fn glGetTexGeniv(&self, _coord: GLenum, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetTexGeniv)(_coord, _pname, _params)
    }
    unsafe fn glLoadName(&self, _name: GLuint) {
        (self.entry().glLoadName)(_name)
    }
    unsafe fn glTexGendv(&self, _coord: GLenum, _pname: GLenum, _params: *const GLdouble) {
        (self.entry().glTexGendv)(_coord, _pname, _params)
    }
    unsafe fn glRectfv(&self, _v1: *const GLfloat, _v2: *const GLfloat) {
        (self.entry().glRectfv)(_v1, _v2)
    }
    unsafe fn glMatrixMode(&self, _mode: GLenum) {
        (self.entry().glMatrixMode)(_mode)
    }
    unsafe fn glIndexiv(&self, _c: *const GLint) {
        (self.entry().glIndexiv)(_c)
    }
    unsafe fn glLightfv(&self, _light: GLenum, _pname: GLenum, _params: *const GLfloat) {
        (self.entry().glLightfv)(_light, _pname, _params)
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
    unsafe fn glFrontFace(&self, _mode: GLenum) {
        (self.entry().glFrontFace)(_mode)
    }
    unsafe fn glPixelStoref(&self, _pname: GLenum, _param: GLfloat) {
        (self.entry().glPixelStoref)(_pname, _param)
    }
    unsafe fn glMultMatrixd(&self, _m: *const GLdouble) {
        (self.entry().glMultMatrixd)(_m)
    }
    unsafe fn glColor4ubv(&self, _v: *const GLubyte) {
        (self.entry().glColor4ubv)(_v)
    }
    unsafe fn glNormal3i(&self, _nx: GLint, _ny: GLint, _nz: GLint) {
        (self.entry().glNormal3i)(_nx, _ny, _nz)
    }
    unsafe fn glRasterPos3fv(&self, _v: *const GLfloat) {
        (self.entry().glRasterPos3fv)(_v)
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
    unsafe fn glMateriali(&self, _face: GLenum, _pname: GLenum, _param: GLint) {
        (self.entry().glMateriali)(_face, _pname, _param)
    }
    unsafe fn glEvalMesh1(&self, _mode: GLenum, _i1: GLint, _i2: GLint) {
        (self.entry().glEvalMesh1)(_mode, _i1, _i2)
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
    unsafe fn glRasterPos2fv(&self, _v: *const GLfloat) {
        (self.entry().glRasterPos2fv)(_v)
    }
    unsafe fn glVertex4iv(&self, _v: *const GLint) {
        (self.entry().glVertex4iv)(_v)
    }
    unsafe fn glMaterialiv(&self, _face: GLenum, _pname: GLenum, _params: *const GLint) {
        (self.entry().glMaterialiv)(_face, _pname, _params)
    }
    unsafe fn glColor3i(&self, _red: GLint, _green: GLint, _blue: GLint) {
        (self.entry().glColor3i)(_red, _green, _blue)
    }
    unsafe fn glRotatef(&self, _angle: GLfloat, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glRotatef)(_angle, _x, _y, _z)
    }
    unsafe fn glTexCoord2dv(&self, _v: *const GLdouble) {
        (self.entry().glTexCoord2dv)(_v)
    }
    unsafe fn glTexCoord2s(&self, _s: GLshort, _t: GLshort) {
        (self.entry().glTexCoord2s)(_s, _t)
    }
    unsafe fn glPixelMapusv(&self, _map: GLenum, _mapsize: GLsizei, _values: *const GLushort) {
        (self.entry().glPixelMapusv)(_map, _mapsize, _values)
    }
    unsafe fn glNormal3f(&self, _nx: GLfloat, _ny: GLfloat, _nz: GLfloat) {
        (self.entry().glNormal3f)(_nx, _ny, _nz)
    }
    unsafe fn glEnable(&self, _cap: GLenum) {
        (self.entry().glEnable)(_cap)
    }
    unsafe fn glGetTexEnvfv(&self, _target: GLenum, _pname: GLenum, _params: *mut GLfloat) {
        (self.entry().glGetTexEnvfv)(_target, _pname, _params)
    }
    unsafe fn glFinish(&self) {
        (self.entry().glFinish)()
    }
    unsafe fn glEvalCoord2fv(&self, _u: *const GLfloat) {
        (self.entry().glEvalCoord2fv)(_u)
    }
    unsafe fn glLoadMatrixf(&self, _m: *const GLfloat) {
        (self.entry().glLoadMatrixf)(_m)
    }
    unsafe fn glTexParameterfv(&self, _target: GLenum, _pname: GLenum, _params: *const GLfloat) {
        (self.entry().glTexParameterfv)(_target, _pname, _params)
    }
    unsafe fn glColor4dv(&self, _v: *const GLdouble) {
        (self.entry().glColor4dv)(_v)
    }
    unsafe fn glColor3ui(&self, _red: GLuint, _green: GLuint, _blue: GLuint) {
        (self.entry().glColor3ui)(_red, _green, _blue)
    }
    unsafe fn glPushAttrib(&self, _mask: GLbitfield) {
        (self.entry().glPushAttrib)(_mask)
    }
    unsafe fn glRasterPos2f(&self, _x: GLfloat, _y: GLfloat) {
        (self.entry().glRasterPos2f)(_x, _y)
    }
    unsafe fn glEndList(&self) {
        (self.entry().glEndList)()
    }
    unsafe fn glHint(&self, _target: GLenum, _mode: GLenum) {
        (self.entry().glHint)(_target, _mode)
    }
    unsafe fn glColor4i(&self, _red: GLint, _green: GLint, _blue: GLint, _alpha: GLint) {
        (self.entry().glColor4i)(_red, _green, _blue, _alpha)
    }
    unsafe fn glClipPlane(&self, _plane: GLenum, _equation: *const GLdouble) {
        (self.entry().glClipPlane)(_plane, _equation)
    }
    unsafe fn glVertex3f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glVertex3f)(_x, _y, _z)
    }
    unsafe fn glNormal3sv(&self, _v: *const GLshort) {
        (self.entry().glNormal3sv)(_v)
    }
    unsafe fn glVertex4i(&self, _x: GLint, _y: GLint, _z: GLint, _w: GLint) {
        (self.entry().glVertex4i)(_x, _y, _z, _w)
    }
    unsafe fn glVertex3sv(&self, _v: *const GLshort) {
        (self.entry().glVertex3sv)(_v)
    }
    unsafe fn glColor3b(&self, _red: GLbyte, _green: GLbyte, _blue: GLbyte) {
        (self.entry().glColor3b)(_red, _green, _blue)
    }
    unsafe fn glTexEnvfv(&self, _target: GLenum, _pname: GLenum, _params: *const GLfloat) {
        (self.entry().glTexEnvfv)(_target, _pname, _params)
    }
    unsafe fn glRasterPos2iv(&self, _v: *const GLint) {
        (self.entry().glRasterPos2iv)(_v)
    }
    unsafe fn glBlendFunc(&self, _sfactor: GLenum, _dfactor: GLenum) {
        (self.entry().glBlendFunc)(_sfactor, _dfactor)
    }
    unsafe fn glColor4usv(&self, _v: *const GLushort) {
        (self.entry().glColor4usv)(_v)
    }
    unsafe fn glVertex3dv(&self, _v: *const GLdouble) {
        (self.entry().glVertex3dv)(_v)
    }
    unsafe fn glTexCoord2f(&self, _s: GLfloat, _t: GLfloat) {
        (self.entry().glTexCoord2f)(_s, _t)
    }
    unsafe fn glColor3dv(&self, _v: *const GLdouble) {
        (self.entry().glColor3dv)(_v)
    }
    unsafe fn glTexCoord2fv(&self, _v: *const GLfloat) {
        (self.entry().glTexCoord2fv)(_v)
    }
    unsafe fn glGetMapdv(&self, _target: GLenum, _query: GLenum, _v: *mut GLdouble) {
        (self.entry().glGetMapdv)(_target, _query, _v)
    }
    unsafe fn glVertex2i(&self, _x: GLint, _y: GLint) {
        (self.entry().glVertex2i)(_x, _y)
    }
    unsafe fn glRasterPos3sv(&self, _v: *const GLshort) {
        (self.entry().glRasterPos3sv)(_v)
    }
    unsafe fn glTexCoord4s(&self, _s: GLshort, _t: GLshort, _r: GLshort, _q: GLshort) {
        (self.entry().glTexCoord4s)(_s, _t, _r, _q)
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
    unsafe fn glFogfv(&self, _pname: GLenum, _params: *const GLfloat) {
        (self.entry().glFogfv)(_pname, _params)
    }
    unsafe fn glColor4fv(&self, _v: *const GLfloat) {
        (self.entry().glColor4fv)(_v)
    }
    unsafe fn glTexCoord2sv(&self, _v: *const GLshort) {
        (self.entry().glTexCoord2sv)(_v)
    }
    unsafe fn glEvalPoint1(&self, _i: GLint) {
        (self.entry().glEvalPoint1)(_i)
    }
    unsafe fn glEvalCoord1d(&self, _u: GLdouble) {
        (self.entry().glEvalCoord1d)(_u)
    }
    unsafe fn glColor3ub(&self, _red: GLubyte, _green: GLubyte, _blue: GLubyte) {
        (self.entry().glColor3ub)(_red, _green, _blue)
    }
    unsafe fn glLightf(&self, _light: GLenum, _pname: GLenum, _param: GLfloat) {
        (self.entry().glLightf)(_light, _pname, _param)
    }
    unsafe fn glPixelMapfv(&self, _map: GLenum, _mapsize: GLsizei, _values: *const GLfloat) {
        (self.entry().glPixelMapfv)(_map, _mapsize, _values)
    }
    unsafe fn glFogiv(&self, _pname: GLenum, _params: *const GLint) {
        (self.entry().glFogiv)(_pname, _params)
    }
    unsafe fn glClearStencil(&self, _s: GLint) {
        (self.entry().glClearStencil)(_s)
    }
    unsafe fn glGetDoublev(&self, _pname: GLenum, _data: *mut GLdouble) {
        (self.entry().glGetDoublev)(_pname, _data)
    }
    unsafe fn glTexParameteriv(&self, _target: GLenum, _pname: GLenum, _params: *const GLint) {
        (self.entry().glTexParameteriv)(_target, _pname, _params)
    }
    unsafe fn glStencilOp(&self, _fail: GLenum, _zfail: GLenum, _zpass: GLenum) {
        (self.entry().glStencilOp)(_fail, _zfail, _zpass)
    }
    unsafe fn glGetString(&self, _name: GLenum) -> *const GLubyte {
        (self.entry().glGetString)(_name)
    }
    unsafe fn glNormal3iv(&self, _v: *const GLint) {
        (self.entry().glNormal3iv)(_v)
    }
    unsafe fn glTranslated(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glTranslated)(_x, _y, _z)
    }
    unsafe fn glIsList(&self, _list: GLuint) -> GLboolean {
        (self.entry().glIsList)(_list)
    }
    unsafe fn glEdgeFlag(&self, _flag: GLboolean) {
        (self.entry().glEdgeFlag)(_flag)
    }
    unsafe fn glStencilFunc(&self, _func: GLenum, _ref: GLint, _mask: GLuint) {
        (self.entry().glStencilFunc)(_func, _ref, _mask)
    }
    unsafe fn glGenLists(&self, _range: GLsizei) -> GLuint {
        (self.entry().glGenLists)(_range)
    }
    unsafe fn glClearDepth(&self, _depth: GLdouble) {
        (self.entry().glClearDepth)(_depth)
    }
    unsafe fn glTexCoord1dv(&self, _v: *const GLdouble) {
        (self.entry().glTexCoord1dv)(_v)
    }
    unsafe fn glTexCoord4f(&self, _s: GLfloat, _t: GLfloat, _r: GLfloat, _q: GLfloat) {
        (self.entry().glTexCoord4f)(_s, _t, _r, _q)
    }
    unsafe fn glFeedbackBuffer(&self, _size: GLsizei, _type: GLenum, _buffer: *mut GLfloat) {
        (self.entry().glFeedbackBuffer)(_size, _type, _buffer)
    }
    unsafe fn glColor4sv(&self, _v: *const GLshort) {
        (self.entry().glColor4sv)(_v)
    }
    unsafe fn glRasterPos3i(&self, _x: GLint, _y: GLint, _z: GLint) {
        (self.entry().glRasterPos3i)(_x, _y, _z)
    }
    unsafe fn glIndexfv(&self, _c: *const GLfloat) {
        (self.entry().glIndexfv)(_c)
    }
    unsafe fn glTexCoord3d(&self, _s: GLdouble, _t: GLdouble, _r: GLdouble) {
        (self.entry().glTexCoord3d)(_s, _t, _r)
    }
    unsafe fn glIndexs(&self, _c: GLshort) {
        (self.entry().glIndexs)(_c)
    }
    unsafe fn glEvalCoord1fv(&self, _u: *const GLfloat) {
        (self.entry().glEvalCoord1fv)(_u)
    }
    unsafe fn glColor4iv(&self, _v: *const GLint) {
        (self.entry().glColor4iv)(_v)
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
    unsafe fn glGetTexLevelParameteriv(
        &self,
        _target: GLenum,
        _level: GLint,
        _pname: GLenum,
        _params: *mut GLint,
    ) {
        (self.entry().glGetTexLevelParameteriv)(_target, _level, _pname, _params)
    }
    unsafe fn glPointSize(&self, _size: GLfloat) {
        (self.entry().glPointSize)(_size)
    }
    unsafe fn glVertex2d(&self, _x: GLdouble, _y: GLdouble) {
        (self.entry().glVertex2d)(_x, _y)
    }
    unsafe fn glNormal3dv(&self, _v: *const GLdouble) {
        (self.entry().glNormal3dv)(_v)
    }
    unsafe fn glVertex4dv(&self, _v: *const GLdouble) {
        (self.entry().glVertex4dv)(_v)
    }
    unsafe fn glLightiv(&self, _light: GLenum, _pname: GLenum, _params: *const GLint) {
        (self.entry().glLightiv)(_light, _pname, _params)
    }
    unsafe fn glColor4f(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat, _alpha: GLfloat) {
        (self.entry().glColor4f)(_red, _green, _blue, _alpha)
    }
    unsafe fn glLineStipple(&self, _factor: GLint, _pattern: GLushort) {
        (self.entry().glLineStipple)(_factor, _pattern)
    }
    unsafe fn glTexCoord3i(&self, _s: GLint, _t: GLint, _r: GLint) {
        (self.entry().glTexCoord3i)(_s, _t, _r)
    }
    unsafe fn glEdgeFlagv(&self, _flag: *const GLboolean) {
        (self.entry().glEdgeFlagv)(_flag)
    }
    unsafe fn glColor3usv(&self, _v: *const GLushort) {
        (self.entry().glColor3usv)(_v)
    }
    unsafe fn glLoadIdentity(&self) {
        (self.entry().glLoadIdentity)()
    }
    unsafe fn glNewList(&self, _list: GLuint, _mode: GLenum) {
        (self.entry().glNewList)(_list, _mode)
    }
    unsafe fn glColorMaterial(&self, _face: GLenum, _mode: GLenum) {
        (self.entry().glColorMaterial)(_face, _mode)
    }
    unsafe fn glPopAttrib(&self) {
        (self.entry().glPopAttrib)()
    }
    unsafe fn glPixelTransferi(&self, _pname: GLenum, _param: GLint) {
        (self.entry().glPixelTransferi)(_pname, _param)
    }
    unsafe fn glRenderMode(&self, _mode: GLenum) -> GLint {
        (self.entry().glRenderMode)(_mode)
    }
    unsafe fn glEvalCoord2dv(&self, _u: *const GLdouble) {
        (self.entry().glEvalCoord2dv)(_u)
    }
    unsafe fn glScalef(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glScalef)(_x, _y, _z)
    }
    unsafe fn glTexEnviv(&self, _target: GLenum, _pname: GLenum, _params: *const GLint) {
        (self.entry().glTexEnviv)(_target, _pname, _params)
    }
    unsafe fn glPixelTransferf(&self, _pname: GLenum, _param: GLfloat) {
        (self.entry().glPixelTransferf)(_pname, _param)
    }
    unsafe fn glLoadMatrixd(&self, _m: *const GLdouble) {
        (self.entry().glLoadMatrixd)(_m)
    }
    unsafe fn glPixelStorei(&self, _pname: GLenum, _param: GLint) {
        (self.entry().glPixelStorei)(_pname, _param)
    }
    unsafe fn glClearIndex(&self, _c: GLfloat) {
        (self.entry().glClearIndex)(_c)
    }
    unsafe fn glGetMaterialiv(&self, _face: GLenum, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetMaterialiv)(_face, _pname, _params)
    }
    unsafe fn glVertex2f(&self, _x: GLfloat, _y: GLfloat) {
        (self.entry().glVertex2f)(_x, _y)
    }
    unsafe fn glCallList(&self, _list: GLuint) {
        (self.entry().glCallList)(_list)
    }
    unsafe fn glRasterPos2s(&self, _x: GLshort, _y: GLshort) {
        (self.entry().glRasterPos2s)(_x, _y)
    }
    unsafe fn glRasterPos4s(&self, _x: GLshort, _y: GLshort, _z: GLshort, _w: GLshort) {
        (self.entry().glRasterPos4s)(_x, _y, _z, _w)
    }
    unsafe fn glRasterPos3s(&self, _x: GLshort, _y: GLshort, _z: GLshort) {
        (self.entry().glRasterPos3s)(_x, _y, _z)
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
    unsafe fn glPixelMapuiv(&self, _map: GLenum, _mapsize: GLsizei, _values: *const GLuint) {
        (self.entry().glPixelMapuiv)(_map, _mapsize, _values)
    }
    unsafe fn glColor3us(&self, _red: GLushort, _green: GLushort, _blue: GLushort) {
        (self.entry().glColor3us)(_red, _green, _blue)
    }
    unsafe fn glScaled(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {
        (self.entry().glScaled)(_x, _y, _z)
    }
    unsafe fn glEvalPoint2(&self, _i: GLint, _j: GLint) {
        (self.entry().glEvalPoint2)(_i, _j)
    }
    unsafe fn glEvalCoord2f(&self, _u: GLfloat, _v: GLfloat) {
        (self.entry().glEvalCoord2f)(_u, _v)
    }
    unsafe fn glRectd(&self, _x1: GLdouble, _y1: GLdouble, _x2: GLdouble, _y2: GLdouble) {
        (self.entry().glRectd)(_x1, _y1, _x2, _y2)
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
    unsafe fn glTexCoord1f(&self, _s: GLfloat) {
        (self.entry().glTexCoord1f)(_s)
    }
    unsafe fn glRasterPos2d(&self, _x: GLdouble, _y: GLdouble) {
        (self.entry().glRasterPos2d)(_x, _y)
    }
    unsafe fn glRasterPos3dv(&self, _v: *const GLdouble) {
        (self.entry().glRasterPos3dv)(_v)
    }
    unsafe fn glMapGrid1d(&self, _un: GLint, _u1: GLdouble, _u2: GLdouble) {
        (self.entry().glMapGrid1d)(_un, _u1, _u2)
    }
    unsafe fn glColor4ub(&self, _red: GLubyte, _green: GLubyte, _blue: GLubyte, _alpha: GLubyte) {
        (self.entry().glColor4ub)(_red, _green, _blue, _alpha)
    }
    unsafe fn glTexCoord1i(&self, _s: GLint) {
        (self.entry().glTexCoord1i)(_s)
    }
    unsafe fn glTexCoord1fv(&self, _v: *const GLfloat) {
        (self.entry().glTexCoord1fv)(_v)
    }
    unsafe fn glVertex3i(&self, _x: GLint, _y: GLint, _z: GLint) {
        (self.entry().glVertex3i)(_x, _y, _z)
    }
    unsafe fn glColor3iv(&self, _v: *const GLint) {
        (self.entry().glColor3iv)(_v)
    }
    unsafe fn glMaterialfv(&self, _face: GLenum, _pname: GLenum, _params: *const GLfloat) {
        (self.entry().glMaterialfv)(_face, _pname, _params)
    }
    unsafe fn glLighti(&self, _light: GLenum, _pname: GLenum, _param: GLint) {
        (self.entry().glLighti)(_light, _pname, _param)
    }
    unsafe fn glGetTexParameteriv(&self, _target: GLenum, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetTexParameteriv)(_target, _pname, _params)
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
    unsafe fn glIndexdv(&self, _c: *const GLdouble) {
        (self.entry().glIndexdv)(_c)
    }
    unsafe fn glVertex4sv(&self, _v: *const GLshort) {
        (self.entry().glVertex4sv)(_v)
    }
    unsafe fn glTexGenfv(&self, _coord: GLenum, _pname: GLenum, _params: *const GLfloat) {
        (self.entry().glTexGenfv)(_coord, _pname, _params)
    }
    unsafe fn glTexCoord3dv(&self, _v: *const GLdouble) {
        (self.entry().glTexCoord3dv)(_v)
    }
    unsafe fn glTexParameteri(&self, _target: GLenum, _pname: GLenum, _param: GLint) {
        (self.entry().glTexParameteri)(_target, _pname, _param)
    }
    unsafe fn glPushName(&self, _name: GLuint) {
        (self.entry().glPushName)(_name)
    }
    unsafe fn glEvalCoord2d(&self, _u: GLdouble, _v: GLdouble) {
        (self.entry().glEvalCoord2d)(_u, _v)
    }
    unsafe fn glEvalMesh2(&self, _mode: GLenum, _i1: GLint, _i2: GLint, _j1: GLint, _j2: GLint) {
        (self.entry().glEvalMesh2)(_mode, _i1, _i2, _j1, _j2)
    }
    unsafe fn glNormal3b(&self, _nx: GLbyte, _ny: GLbyte, _nz: GLbyte) {
        (self.entry().glNormal3b)(_nx, _ny, _nz)
    }
    unsafe fn glGetBooleanv(&self, _pname: GLenum, _data: *mut GLboolean) {
        (self.entry().glGetBooleanv)(_pname, _data)
    }
    unsafe fn glVertex3fv(&self, _v: *const GLfloat) {
        (self.entry().glVertex3fv)(_v)
    }
    unsafe fn glIndexsv(&self, _c: *const GLshort) {
        (self.entry().glIndexsv)(_c)
    }
    unsafe fn glTexCoord3iv(&self, _v: *const GLint) {
        (self.entry().glTexCoord3iv)(_v)
    }
    unsafe fn glTexCoord4sv(&self, _v: *const GLshort) {
        (self.entry().glTexCoord4sv)(_v)
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
    unsafe fn glCullFace(&self, _mode: GLenum) {
        (self.entry().glCullFace)(_mode)
    }
    unsafe fn glCallLists(&self, _n: GLsizei, _type: GLenum, _lists: *const std::os::raw::c_void) {
        (self.entry().glCallLists)(_n, _type, _lists)
    }
    unsafe fn glVertex4d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble, _w: GLdouble) {
        (self.entry().glVertex4d)(_x, _y, _z, _w)
    }
    unsafe fn glEnd(&self) {
        (self.entry().glEnd)()
    }
    unsafe fn glTexGeniv(&self, _coord: GLenum, _pname: GLenum, _params: *const GLint) {
        (self.entry().glTexGeniv)(_coord, _pname, _params)
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
    unsafe fn glTexGend(&self, _coord: GLenum, _pname: GLenum, _param: GLdouble) {
        (self.entry().glTexGend)(_coord, _pname, _param)
    }
    unsafe fn glAccum(&self, _op: GLenum, _value: GLfloat) {
        (self.entry().glAccum)(_op, _value)
    }
    unsafe fn glLightModelfv(&self, _pname: GLenum, _params: *const GLfloat) {
        (self.entry().glLightModelfv)(_pname, _params)
    }
    unsafe fn glTexCoord4d(&self, _s: GLdouble, _t: GLdouble, _r: GLdouble, _q: GLdouble) {
        (self.entry().glTexCoord4d)(_s, _t, _r, _q)
    }
    unsafe fn glTexCoord1iv(&self, _v: *const GLint) {
        (self.entry().glTexCoord1iv)(_v)
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
    unsafe fn glRasterPos2dv(&self, _v: *const GLdouble) {
        (self.entry().glRasterPos2dv)(_v)
    }
    unsafe fn glGetPixelMapfv(&self, _map: GLenum, _values: *mut GLfloat) {
        (self.entry().glGetPixelMapfv)(_map, _values)
    }
    unsafe fn glGetTexGendv(&self, _coord: GLenum, _pname: GLenum, _params: *mut GLdouble) {
        (self.entry().glGetTexGendv)(_coord, _pname, _params)
    }
    unsafe fn glSelectBuffer(&self, _size: GLsizei, _buffer: *mut GLuint) {
        (self.entry().glSelectBuffer)(_size, _buffer)
    }
    unsafe fn glTexCoord4iv(&self, _v: *const GLint) {
        (self.entry().glTexCoord4iv)(_v)
    }
    unsafe fn glColor4ui(&self, _red: GLuint, _green: GLuint, _blue: GLuint, _alpha: GLuint) {
        (self.entry().glColor4ui)(_red, _green, _blue, _alpha)
    }
    unsafe fn glFlush(&self) {
        (self.entry().glFlush)()
    }
    unsafe fn glClear(&self, _mask: GLbitfield) {
        (self.entry().glClear)(_mask)
    }
    unsafe fn glColor4uiv(&self, _v: *const GLuint) {
        (self.entry().glColor4uiv)(_v)
    }
    unsafe fn glTexCoord1d(&self, _s: GLdouble) {
        (self.entry().glTexCoord1d)(_s)
    }
    unsafe fn glRectsv(&self, _v1: *const GLshort, _v2: *const GLshort) {
        (self.entry().glRectsv)(_v1, _v2)
    }
    unsafe fn glRectdv(&self, _v1: *const GLdouble, _v2: *const GLdouble) {
        (self.entry().glRectdv)(_v1, _v2)
    }
    unsafe fn glColor3sv(&self, _v: *const GLshort) {
        (self.entry().glColor3sv)(_v)
    }
    unsafe fn glListBase(&self, _base: GLuint) {
        (self.entry().glListBase)(_base)
    }
    unsafe fn glEvalCoord1dv(&self, _u: *const GLdouble) {
        (self.entry().glEvalCoord1dv)(_u)
    }
    unsafe fn glVertex2fv(&self, _v: *const GLfloat) {
        (self.entry().glVertex2fv)(_v)
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
    unsafe fn glLightModeli(&self, _pname: GLenum, _param: GLint) {
        (self.entry().glLightModeli)(_pname, _param)
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
    unsafe fn glGetTexParameterfv(&self, _target: GLenum, _pname: GLenum, _params: *mut GLfloat) {
        (self.entry().glGetTexParameterfv)(_target, _pname, _params)
    }
    unsafe fn glEvalCoord1f(&self, _u: GLfloat) {
        (self.entry().glEvalCoord1f)(_u)
    }
    unsafe fn glRasterPos2i(&self, _x: GLint, _y: GLint) {
        (self.entry().glRasterPos2i)(_x, _y)
    }
    unsafe fn glInitNames(&self) {
        (self.entry().glInitNames)()
    }
    unsafe fn glTexGeni(&self, _coord: GLenum, _pname: GLenum, _param: GLint) {
        (self.entry().glTexGeni)(_coord, _pname, _param)
    }
    unsafe fn glColor3fv(&self, _v: *const GLfloat) {
        (self.entry().glColor3fv)(_v)
    }
    unsafe fn glMapGrid1f(&self, _un: GLint, _u1: GLfloat, _u2: GLfloat) {
        (self.entry().glMapGrid1f)(_un, _u1, _u2)
    }
    unsafe fn glTexEnvf(&self, _target: GLenum, _pname: GLenum, _param: GLfloat) {
        (self.entry().glTexEnvf)(_target, _pname, _param)
    }
    unsafe fn glTexCoord4fv(&self, _v: *const GLfloat) {
        (self.entry().glTexCoord4fv)(_v)
    }
    unsafe fn glColor3ubv(&self, _v: *const GLubyte) {
        (self.entry().glColor3ubv)(_v)
    }
    unsafe fn glColor4b(&self, _red: GLbyte, _green: GLbyte, _blue: GLbyte, _alpha: GLbyte) {
        (self.entry().glColor4b)(_red, _green, _blue, _alpha)
    }
    unsafe fn glRasterPos4iv(&self, _v: *const GLint) {
        (self.entry().glRasterPos4iv)(_v)
    }
    unsafe fn glTexCoord3fv(&self, _v: *const GLfloat) {
        (self.entry().glTexCoord3fv)(_v)
    }
    unsafe fn glRectiv(&self, _v1: *const GLint, _v2: *const GLint) {
        (self.entry().glRectiv)(_v1, _v2)
    }
    unsafe fn glGetMapfv(&self, _target: GLenum, _query: GLenum, _v: *mut GLfloat) {
        (self.entry().glGetMapfv)(_target, _query, _v)
    }
    unsafe fn glGetTexGenfv(&self, _coord: GLenum, _pname: GLenum, _params: *mut GLfloat) {
        (self.entry().glGetTexGenfv)(_coord, _pname, _params)
    }
    unsafe fn glReadBuffer(&self, _src: GLenum) {
        (self.entry().glReadBuffer)(_src)
    }
    unsafe fn glRasterPos4i(&self, _x: GLint, _y: GLint, _z: GLint, _w: GLint) {
        (self.entry().glRasterPos4i)(_x, _y, _z, _w)
    }
    unsafe fn glTexEnvi(&self, _target: GLenum, _pname: GLenum, _param: GLint) {
        (self.entry().glTexEnvi)(_target, _pname, _param)
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
    unsafe fn glFogf(&self, _pname: GLenum, _param: GLfloat) {
        (self.entry().glFogf)(_pname, _param)
    }
    unsafe fn glVertex2sv(&self, _v: *const GLshort) {
        (self.entry().glVertex2sv)(_v)
    }
    unsafe fn glStencilMask(&self, _mask: GLuint) {
        (self.entry().glStencilMask)(_mask)
    }
    unsafe fn glDepthMask(&self, _flag: GLboolean) {
        (self.entry().glDepthMask)(_flag)
    }
    unsafe fn glDepthRange(&self, _n: GLdouble, _f: GLdouble) {
        (self.entry().glDepthRange)(_n, _f)
    }
    unsafe fn glRasterPos4fv(&self, _v: *const GLfloat) {
        (self.entry().glRasterPos4fv)(_v)
    }
    unsafe fn glDrawBuffer(&self, _buf: GLenum) {
        (self.entry().glDrawBuffer)(_buf)
    }
}
