use crate::gles::feature::EntryGLESFn;
use crate::types::*;
use std::mem;
pub trait GLES10 {
    unsafe fn entry(&self) -> &EntryGLESFn;
    unsafe fn glActiveTexture(&self, _texture: GLenum) {
        (self.entry().glActiveTexture)(_texture)
    }
    unsafe fn glAlphaFunc(&self, _func: GLenum, _ref: GLfloat) {
        (self.entry().glAlphaFunc)(_func, _ref)
    }
    unsafe fn glAlphaFuncx(&self, _func: GLenum, _ref: GLfixed) {
        (self.entry().glAlphaFuncx)(_func, _ref)
    }
    unsafe fn glBindBuffer(&self, _target: GLenum, _buffer: GLuint) {
        (self.entry().glBindBuffer)(_target, _buffer)
    }
    unsafe fn glBindTexture(&self, _target: GLenum, _texture: GLuint) {
        (self.entry().glBindTexture)(_target, _texture)
    }
    unsafe fn glBlendFunc(&self, _sfactor: GLenum, _dfactor: GLenum) {
        (self.entry().glBlendFunc)(_sfactor, _dfactor)
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
    unsafe fn glClear(&self, _mask: GLbitfield) {
        (self.entry().glClear)(_mask)
    }
    unsafe fn glClearColor(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat, _alpha: GLfloat) {
        (self.entry().glClearColor)(_red, _green, _blue, _alpha)
    }
    unsafe fn glClearColorx(
        &self,
        _red: GLfixed,
        _green: GLfixed,
        _blue: GLfixed,
        _alpha: GLfixed,
    ) {
        (self.entry().glClearColorx)(_red, _green, _blue, _alpha)
    }
    unsafe fn glClearDepthf(&self, _d: GLfloat) {
        (self.entry().glClearDepthf)(_d)
    }
    unsafe fn glClearDepthx(&self, _depth: GLfixed) {
        (self.entry().glClearDepthx)(_depth)
    }
    unsafe fn glClearStencil(&self, _s: GLint) {
        (self.entry().glClearStencil)(_s)
    }
    unsafe fn glClientActiveTexture(&self, _texture: GLenum) {
        (self.entry().glClientActiveTexture)(_texture)
    }
    unsafe fn glClipPlanef(&self, _p: GLenum, _eqn: *const GLfloat) {
        (self.entry().glClipPlanef)(_p, _eqn)
    }
    unsafe fn glClipPlanex(&self, _plane: GLenum, _equation: *const GLfixed) {
        (self.entry().glClipPlanex)(_plane, _equation)
    }
    unsafe fn glColor4f(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat, _alpha: GLfloat) {
        (self.entry().glColor4f)(_red, _green, _blue, _alpha)
    }
    unsafe fn glColor4ub(&self, _red: GLubyte, _green: GLubyte, _blue: GLubyte, _alpha: GLubyte) {
        (self.entry().glColor4ub)(_red, _green, _blue, _alpha)
    }
    unsafe fn glColor4x(&self, _red: GLfixed, _green: GLfixed, _blue: GLfixed, _alpha: GLfixed) {
        (self.entry().glColor4x)(_red, _green, _blue, _alpha)
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
    unsafe fn glColorPointer(
        &self,
        _size: GLint,
        _type: GLenum,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glColorPointer)(_size, _type, _stride, _pointer)
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
    unsafe fn glCullFace(&self, _mode: GLenum) {
        (self.entry().glCullFace)(_mode)
    }
    unsafe fn glDeleteBuffers(&self, array: &[GLuint]) {
        (self.entry().glDeleteBuffers)(array.len() as GLsizei, array.as_ptr())
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
    unsafe fn glDepthRangef(&self, _n: GLfloat, _f: GLfloat) {
        (self.entry().glDepthRangef)(_n, _f)
    }
    unsafe fn glDepthRangex(&self, _n: GLfixed, _f: GLfixed) {
        (self.entry().glDepthRangex)(_n, _f)
    }
    unsafe fn glDisable(&self, _cap: GLenum) {
        (self.entry().glDisable)(_cap)
    }
    unsafe fn glDisableClientState(&self, _array: GLenum) {
        (self.entry().glDisableClientState)(_array)
    }
    unsafe fn glDrawArrays(&self, _mode: GLenum, _first: GLint, _count: GLsizei) {
        (self.entry().glDrawArrays)(_mode, _first, _count)
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
    unsafe fn glEnable(&self, _cap: GLenum) {
        (self.entry().glEnable)(_cap)
    }
    unsafe fn glEnableClientState(&self, _array: GLenum) {
        (self.entry().glEnableClientState)(_array)
    }
    unsafe fn glFinish(&self) {
        (self.entry().glFinish)()
    }
    unsafe fn glFlush(&self) {
        (self.entry().glFlush)()
    }
    unsafe fn glFogf(&self, _pname: GLenum, _param: GLfloat) {
        (self.entry().glFogf)(_pname, _param)
    }
    unsafe fn glFogfv(&self, _pname: GLenum, _params: *const GLfloat) {
        (self.entry().glFogfv)(_pname, _params)
    }
    unsafe fn glFogx(&self, _pname: GLenum, _param: GLfixed) {
        (self.entry().glFogx)(_pname, _param)
    }
    unsafe fn glFogxv(&self, _pname: GLenum, _param: *const GLfixed) {
        (self.entry().glFogxv)(_pname, _param)
    }
    unsafe fn glFrontFace(&self, _mode: GLenum) {
        (self.entry().glFrontFace)(_mode)
    }
    unsafe fn glFrustumf(
        &self,
        _l: GLfloat,
        _r: GLfloat,
        _b: GLfloat,
        _t: GLfloat,
        _n: GLfloat,
        _f: GLfloat,
    ) {
        (self.entry().glFrustumf)(_l, _r, _b, _t, _n, _f)
    }
    unsafe fn glFrustumx(
        &self,
        _l: GLfixed,
        _r: GLfixed,
        _b: GLfixed,
        _t: GLfixed,
        _n: GLfixed,
        _f: GLfixed,
    ) {
        (self.entry().glFrustumx)(_l, _r, _b, _t, _n, _f)
    }
    unsafe fn glGenBuffers(&self, _n: GLsizei, _buffers: *mut GLuint) {
        (self.entry().glGenBuffers)(_n, _buffers)
    }
    unsafe fn glGenTextures(&self, array: &mut [GLuint]) {
        (self.entry().glGenTextures)(array.len() as GLsizei, array.as_mut_ptr())
    }
    unsafe fn glGetBooleanv(&self, _pname: GLenum, _data: *mut GLboolean) {
        (self.entry().glGetBooleanv)(_pname, _data)
    }
    unsafe fn glGetBufferParameteriv(&self, _target: GLenum, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetBufferParameteriv)(_target, _pname, _params)
    }
    unsafe fn glGetClipPlanef(&self, _plane: GLenum, _equation: *mut GLfloat) {
        (self.entry().glGetClipPlanef)(_plane, _equation)
    }
    unsafe fn glGetClipPlanex(&self, _plane: GLenum, _equation: *mut GLfixed) {
        (self.entry().glGetClipPlanex)(_plane, _equation)
    }
    unsafe fn glGetError(&self) -> GLenum {
        (self.entry().glGetError)()
    }
    unsafe fn glGetFixedv(&self, _pname: GLenum, _params: *mut GLfixed) {
        (self.entry().glGetFixedv)(_pname, _params)
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
    unsafe fn glGetLightxv(&self, _light: GLenum, _pname: GLenum, _params: *mut GLfixed) {
        (self.entry().glGetLightxv)(_light, _pname, _params)
    }
    unsafe fn glGetMaterialfv(&self, _face: GLenum, _pname: GLenum, _params: *mut GLfloat) {
        (self.entry().glGetMaterialfv)(_face, _pname, _params)
    }
    unsafe fn glGetMaterialxv(&self, _face: GLenum, _pname: GLenum, _params: *mut GLfixed) {
        (self.entry().glGetMaterialxv)(_face, _pname, _params)
    }
    unsafe fn glGetPointerv(&self, _pname: GLenum, _params: *mut *mut std::os::raw::c_void) {
        (self.entry().glGetPointerv)(_pname, _params)
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
    unsafe fn glGetTexEnvxv(&self, _target: GLenum, _pname: GLenum, _params: *mut GLfixed) {
        (self.entry().glGetTexEnvxv)(_target, _pname, _params)
    }
    unsafe fn glGetTexParameterfv(&self, _target: GLenum, _pname: GLenum, _params: *mut GLfloat) {
        (self.entry().glGetTexParameterfv)(_target, _pname, _params)
    }
    unsafe fn glGetTexParameteriv(&self, _target: GLenum, _pname: GLenum, _params: *mut GLint) {
        (self.entry().glGetTexParameteriv)(_target, _pname, _params)
    }
    unsafe fn glGetTexParameterxv(&self, _target: GLenum, _pname: GLenum, _params: *mut GLfixed) {
        (self.entry().glGetTexParameterxv)(_target, _pname, _params)
    }
    unsafe fn glHint(&self, _target: GLenum, _mode: GLenum) {
        (self.entry().glHint)(_target, _mode)
    }
    unsafe fn glIsBuffer(&self, _buffer: GLuint) -> GLboolean {
        (self.entry().glIsBuffer)(_buffer)
    }
    unsafe fn glIsEnabled(&self, _cap: GLenum) -> GLboolean {
        (self.entry().glIsEnabled)(_cap)
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
    unsafe fn glLightModelx(&self, _pname: GLenum, _param: GLfixed) {
        (self.entry().glLightModelx)(_pname, _param)
    }
    unsafe fn glLightModelxv(&self, _pname: GLenum, _param: *const GLfixed) {
        (self.entry().glLightModelxv)(_pname, _param)
    }
    unsafe fn glLightf(&self, _light: GLenum, _pname: GLenum, _param: GLfloat) {
        (self.entry().glLightf)(_light, _pname, _param)
    }
    unsafe fn glLightfv(&self, _light: GLenum, _pname: GLenum, _params: *const GLfloat) {
        (self.entry().glLightfv)(_light, _pname, _params)
    }
    unsafe fn glLightx(&self, _light: GLenum, _pname: GLenum, _param: GLfixed) {
        (self.entry().glLightx)(_light, _pname, _param)
    }
    unsafe fn glLightxv(&self, _light: GLenum, _pname: GLenum, _params: *const GLfixed) {
        (self.entry().glLightxv)(_light, _pname, _params)
    }
    unsafe fn glLineWidth(&self, _width: GLfloat) {
        (self.entry().glLineWidth)(_width)
    }
    unsafe fn glLineWidthx(&self, _width: GLfixed) {
        (self.entry().glLineWidthx)(_width)
    }
    unsafe fn glLoadIdentity(&self) {
        (self.entry().glLoadIdentity)()
    }
    unsafe fn glLoadMatrixf(&self, _m: *const GLfloat) {
        (self.entry().glLoadMatrixf)(_m)
    }
    unsafe fn glLoadMatrixx(&self, _m: *const GLfixed) {
        (self.entry().glLoadMatrixx)(_m)
    }
    unsafe fn glLogicOp(&self, _opcode: GLenum) {
        (self.entry().glLogicOp)(_opcode)
    }
    unsafe fn glMaterialf(&self, _face: GLenum, _pname: GLenum, _param: GLfloat) {
        (self.entry().glMaterialf)(_face, _pname, _param)
    }
    unsafe fn glMaterialfv(&self, _face: GLenum, _pname: GLenum, _params: *const GLfloat) {
        (self.entry().glMaterialfv)(_face, _pname, _params)
    }
    unsafe fn glMaterialx(&self, _face: GLenum, _pname: GLenum, _param: GLfixed) {
        (self.entry().glMaterialx)(_face, _pname, _param)
    }
    unsafe fn glMaterialxv(&self, _face: GLenum, _pname: GLenum, _param: *const GLfixed) {
        (self.entry().glMaterialxv)(_face, _pname, _param)
    }
    unsafe fn glMatrixMode(&self, _mode: GLenum) {
        (self.entry().glMatrixMode)(_mode)
    }
    unsafe fn glMultMatrixf(&self, _m: *const GLfloat) {
        (self.entry().glMultMatrixf)(_m)
    }
    unsafe fn glMultMatrixx(&self, _m: *const GLfixed) {
        (self.entry().glMultMatrixx)(_m)
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
    unsafe fn glMultiTexCoord4x(
        &self,
        _texture: GLenum,
        _s: GLfixed,
        _t: GLfixed,
        _r: GLfixed,
        _q: GLfixed,
    ) {
        (self.entry().glMultiTexCoord4x)(_texture, _s, _t, _r, _q)
    }
    unsafe fn glNormal3f(&self, _nx: GLfloat, _ny: GLfloat, _nz: GLfloat) {
        (self.entry().glNormal3f)(_nx, _ny, _nz)
    }
    unsafe fn glNormal3x(&self, _nx: GLfixed, _ny: GLfixed, _nz: GLfixed) {
        (self.entry().glNormal3x)(_nx, _ny, _nz)
    }
    unsafe fn glNormalPointer(
        &self,
        _type: GLenum,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
        (self.entry().glNormalPointer)(_type, _stride, _pointer)
    }
    unsafe fn glOrthof(
        &self,
        _l: GLfloat,
        _r: GLfloat,
        _b: GLfloat,
        _t: GLfloat,
        _n: GLfloat,
        _f: GLfloat,
    ) {
        (self.entry().glOrthof)(_l, _r, _b, _t, _n, _f)
    }
    unsafe fn glOrthox(
        &self,
        _l: GLfixed,
        _r: GLfixed,
        _b: GLfixed,
        _t: GLfixed,
        _n: GLfixed,
        _f: GLfixed,
    ) {
        (self.entry().glOrthox)(_l, _r, _b, _t, _n, _f)
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
    unsafe fn glPointParameterx(&self, _pname: GLenum, _param: GLfixed) {
        (self.entry().glPointParameterx)(_pname, _param)
    }
    unsafe fn glPointParameterxv(&self, _pname: GLenum, _params: *const GLfixed) {
        (self.entry().glPointParameterxv)(_pname, _params)
    }
    unsafe fn glPointSize(&self, _size: GLfloat) {
        (self.entry().glPointSize)(_size)
    }
    unsafe fn glPointSizex(&self, _size: GLfixed) {
        (self.entry().glPointSizex)(_size)
    }
    unsafe fn glPolygonOffset(&self, _factor: GLfloat, _units: GLfloat) {
        (self.entry().glPolygonOffset)(_factor, _units)
    }
    unsafe fn glPolygonOffsetx(&self, _factor: GLfixed, _units: GLfixed) {
        (self.entry().glPolygonOffsetx)(_factor, _units)
    }
    unsafe fn glPopMatrix(&self) {
        (self.entry().glPopMatrix)()
    }
    unsafe fn glPushMatrix(&self) {
        (self.entry().glPushMatrix)()
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
    unsafe fn glRotatef(&self, _angle: GLfloat, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glRotatef)(_angle, _x, _y, _z)
    }
    unsafe fn glRotatex(&self, _angle: GLfixed, _x: GLfixed, _y: GLfixed, _z: GLfixed) {
        (self.entry().glRotatex)(_angle, _x, _y, _z)
    }
    unsafe fn glSampleCoverage(&self, _value: GLfloat, _invert: GLboolean) {
        (self.entry().glSampleCoverage)(_value, _invert)
    }
    unsafe fn glSampleCoveragex(&self, _value: GLclampx, _invert: GLboolean) {
        (self.entry().glSampleCoveragex)(_value, _invert)
    }
    unsafe fn glScalef(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glScalef)(_x, _y, _z)
    }
    unsafe fn glScalex(&self, _x: GLfixed, _y: GLfixed, _z: GLfixed) {
        (self.entry().glScalex)(_x, _y, _z)
    }
    unsafe fn glScissor(&self, _x: GLint, _y: GLint, _width: GLsizei, _height: GLsizei) {
        (self.entry().glScissor)(_x, _y, _width, _height)
    }
    unsafe fn glShadeModel(&self, _mode: GLenum) {
        (self.entry().glShadeModel)(_mode)
    }
    unsafe fn glStencilFunc(&self, _func: GLenum, _ref: GLint, _mask: GLuint) {
        (self.entry().glStencilFunc)(_func, _ref, _mask)
    }
    unsafe fn glStencilMask(&self, _mask: GLuint) {
        (self.entry().glStencilMask)(_mask)
    }
    unsafe fn glStencilOp(&self, _fail: GLenum, _zfail: GLenum, _zpass: GLenum) {
        (self.entry().glStencilOp)(_fail, _zfail, _zpass)
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
    unsafe fn glTexEnvx(&self, _target: GLenum, _pname: GLenum, _param: GLfixed) {
        (self.entry().glTexEnvx)(_target, _pname, _param)
    }
    unsafe fn glTexEnvxv(&self, _target: GLenum, _pname: GLenum, _params: *const GLfixed) {
        (self.entry().glTexEnvxv)(_target, _pname, _params)
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
    unsafe fn glTexParameterx(&self, _target: GLenum, _pname: GLenum, _param: GLfixed) {
        (self.entry().glTexParameterx)(_target, _pname, _param)
    }
    unsafe fn glTexParameterxv(&self, _target: GLenum, _pname: GLenum, _params: *const GLfixed) {
        (self.entry().glTexParameterxv)(_target, _pname, _params)
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
    unsafe fn glTranslatef(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {
        (self.entry().glTranslatef)(_x, _y, _z)
    }
    unsafe fn glTranslatex(&self, _x: GLfixed, _y: GLfixed, _z: GLfixed) {
        (self.entry().glTranslatex)(_x, _y, _z)
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
}
