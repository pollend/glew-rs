use crate::gl;
use crate::types::*;
use gl::bitflags::*;
use gl::enums::*;
use std::ffi::c_void;
use std::fmt;
impl GL13 {
    pub unsafe fn glLightfv(
        &self,
        _light: LightName,
        _pname: LightParameter,
        _params: *const GLfloat,
    ) {
    }
    pub unsafe fn glTexEnvf(
        &self,
        _target: TextureEnvTarget,
        _pname: TextureEnvParameter,
        _param: GLfloat,
    ) {
    }
    pub unsafe fn glAreTexturesResident(
        &self,
        _n: GLsizei,
        _textures: *const GLuint,
        _residences: *mut GLboolean,
    ) {
    }
    pub unsafe fn glRasterPos4iv(&self, _v: *const GLint) {}
    pub unsafe fn glGetLightfv(
        &self,
        _light: LightName,
        _pname: LightParameter,
        _params: *mut GLfloat,
    ) {
    }
    pub unsafe fn glColor3b(&self, _red: GLbyte, _green: GLbyte, _blue: GLbyte) {}
    pub unsafe fn glRasterPos2i(&self, _x: GLint, _y: GLint) {}
    pub unsafe fn glMultiTexCoord1d(&self, _target: TextureUnit, _s: GLdouble) {}
    pub unsafe fn glRasterPos4d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble, _w: GLdouble) {}
    pub unsafe fn glFlush(&self) {}
    pub unsafe fn glMap1d(
        &self,
        _target: MapTarget,
        _u1: GLdouble,
        _u2: GLdouble,
        _stride: GLint,
        _order: GLint,
        _points: *const GLdouble,
    ) {
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
    }
    pub unsafe fn glGetLightiv(
        &self,
        _light: LightName,
        _pname: LightParameter,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glGetMapdv(&self, _target: MapTarget, _query: GetMapQuery, _v: *mut GLdouble) {}
    pub unsafe fn glGetPixelMapuiv(&self, _map: PixelMap, _values: *mut GLuint) {}
    pub unsafe fn glRectiv(&self, _v1: *const GLint, _v2: *const GLint) {}
    pub unsafe fn glHint(&self, _target: HintTarget, _mode: HintMode) {}
    pub unsafe fn glIndexubv(&self, _c: *const GLubyte) {}
    pub unsafe fn glCullFace(&self, _mode: CullFaceMode) {}
    pub unsafe fn glRasterPos4s(&self, _x: GLshort, _y: GLshort, _z: GLshort, _w: GLshort) {}
    pub unsafe fn glStencilMask(&self, _mask: GLuint) {}
    pub unsafe fn glEvalCoord2fv(&self, _u: *const GLfloat) {}
    pub unsafe fn glLoadIdentity(&self) {}
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
    pub unsafe fn glGetCompressedTexImage(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _img: *mut std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glMultiTexCoord2iv(&self, _target: TextureUnit, _v: *const GLint) {}
    pub unsafe fn glMultiTexCoord4iv(&self, _target: TextureUnit, _v: *const GLint) {}
    pub unsafe fn glVertex2fv(&self, _v: *const GLfloat) {}
    pub unsafe fn glPixelZoom(&self, _xfactor: GLfloat, _yfactor: GLfloat) {}
    pub unsafe fn glFrustum(
        &self,
        _left: GLdouble,
        _right: GLdouble,
        _bottom: GLdouble,
        _top: GLdouble,
        _zNear: GLdouble,
        _zFar: GLdouble,
    ) {
    }
    pub unsafe fn glColor3dv(&self, _v: *const GLdouble) {}
    pub unsafe fn glVertex3d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {}
    pub unsafe fn glCopyPixels(
        &self,
        _x: GLint,
        _y: GLint,
        _width: GLsizei,
        _height: GLsizei,
        _type: PixelCopyType,
    ) {
    }
    pub unsafe fn glBindTexture(&self, _target: TextureTarget, _texture: GLuint) {}
    pub unsafe fn glPopName(&self) {}
    pub unsafe fn glMultMatrixd(&self, _m: *const GLdouble) {}
    pub unsafe fn glLoadMatrixd(&self, _m: *const GLdouble) {}
    pub unsafe fn glPushMatrix(&self) {}
    pub unsafe fn glInterleavedArrays(
        &self,
        _format: InterleavedArrayFormat,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
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
    }
    pub unsafe fn glMultiTexCoord3sv(&self, _target: TextureUnit, _v: *const GLshort) {}
    pub unsafe fn glMultiTexCoord4i(
        &self,
        _target: TextureUnit,
        _s: GLint,
        _t: GLint,
        _r: GLint,
        _q: GLint,
    ) {
    }
    pub unsafe fn glRasterPos3i(&self, _x: GLint, _y: GLint, _z: GLint) {}
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
    pub unsafe fn glColor4b(&self, _red: GLbyte, _green: GLbyte, _blue: GLbyte, _alpha: GLbyte) {}
    pub unsafe fn glMultiTexCoord4f(
        &self,
        _target: TextureUnit,
        _s: GLfloat,
        _t: GLfloat,
        _r: GLfloat,
        _q: GLfloat,
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
    pub unsafe fn glVertex3fv(&self, _v: *const GLfloat) {}
    pub unsafe fn glMultiTexCoord1i(&self, _target: TextureUnit, _s: GLint) {}
    pub unsafe fn glLoadTransposeMatrixf(&self, _m: *const GLfloat) {}
    pub unsafe fn glColor4uiv(&self, _v: *const GLuint) {}
    pub unsafe fn glIndexi(&self, _c: GLint) {}
    pub unsafe fn glTexCoord2i(&self, _s: GLint, _t: GLint) {}
    pub unsafe fn glColor4iv(&self, _v: *const GLint) {}
    pub unsafe fn glTexCoord3i(&self, _s: GLint, _t: GLint, _r: GLint) {}
    pub unsafe fn glStencilFunc(&self, _func: StencilFunction, _ref: GLint, _mask: GLuint) {}
    pub unsafe fn glTexGenf(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _param: GLfloat,
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
    pub unsafe fn glColor3fv(&self, _v: *const GLfloat) {}
    pub unsafe fn glPixelTransferi(&self, _pname: PixelTransferParameter, _param: GLint) {}
    pub unsafe fn glTranslatef(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {}
    pub unsafe fn glMaterialf(
        &self,
        _face: MaterialFace,
        _pname: MaterialParameter,
        _param: GLfloat,
    ) {
    }
    pub unsafe fn glMultiTexCoord4d(
        &self,
        _target: TextureUnit,
        _s: GLdouble,
        _t: GLdouble,
        _r: GLdouble,
        _q: GLdouble,
    ) {
    }
    pub unsafe fn glLoadName(&self, _name: GLuint) {}
    pub unsafe fn glEnable(&self, _cap: EnableCap) {}
    pub unsafe fn glMap1f(
        &self,
        _target: MapTarget,
        _u1: GLfloat,
        _u2: GLfloat,
        _stride: GLint,
        _order: GLint,
        _points: *const GLfloat,
    ) {
    }
    pub unsafe fn glTexCoord2f(&self, _s: GLfloat, _t: GLfloat) {}
    pub unsafe fn glTranslated(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {}
    pub unsafe fn glMultiTexCoord3i(&self, _target: TextureUnit, _s: GLint, _t: GLint, _r: GLint) {}
    pub unsafe fn glFogfv(&self, _pname: FogParameter, _params: *const GLfloat) {}
    pub unsafe fn glMultiTexCoord3fv(&self, _target: TextureUnit, _v: *const GLfloat) {}
    pub unsafe fn glRectsv(&self, _v1: *const GLshort, _v2: *const GLshort) {}
    pub unsafe fn glGetTexEnviv(
        &self,
        _target: TextureEnvTarget,
        _pname: TextureEnvParameter,
        _params: *mut GLint,
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
    pub unsafe fn glScissor(&self, _x: GLint, _y: GLint, _width: GLsizei, _height: GLsizei) {}
    pub unsafe fn glBlendFunc(&self, _sfactor: BlendingFactor, _dfactor: BlendingFactor) {}
    pub unsafe fn glNormal3sv(&self, _v: *const GLshort) {}
    pub unsafe fn glTexCoord3f(&self, _s: GLfloat, _t: GLfloat, _r: GLfloat) {}
    pub unsafe fn glViewport(&self, _x: GLint, _y: GLint, _width: GLsizei, _height: GLsizei) {}
    pub unsafe fn glColor4usv(&self, _v: *const GLushort) {}
    pub unsafe fn glDrawArrays(&self, _mode: PrimitiveType, _first: GLint, _count: GLsizei) {}
    pub unsafe fn glRects(&self, _x1: GLshort, _y1: GLshort, _x2: GLshort, _y2: GLshort) {}
    pub unsafe fn glDeleteLists(&self, _list: GLuint, _range: GLsizei) {}
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
    pub unsafe fn glPointSize(&self, _size: GLfloat) {}
    pub unsafe fn glTexCoord4dv(&self, _v: *const GLdouble) {}
    pub unsafe fn glNormalPointer(
        &self,
        _type: NormalPointerType,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glTexCoord2dv(&self, _v: *const GLdouble) {}
    pub unsafe fn glLightModelfv(&self, _pname: LightModelParameter, _params: *const GLfloat) {}
    pub unsafe fn glScaled(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {}
    pub unsafe fn glVertex2d(&self, _x: GLdouble, _y: GLdouble) {}
    pub unsafe fn glLightiv(
        &self,
        _light: LightName,
        _pname: LightParameter,
        _params: *const GLint,
    ) {
    }
    pub unsafe fn glMultMatrixf(&self, _m: *const GLfloat) {}
    pub unsafe fn glColor4us(
        &self,
        _red: GLushort,
        _green: GLushort,
        _blue: GLushort,
        _alpha: GLushort,
    ) {
    }
    pub unsafe fn glDeleteTextures(&self, _n: GLsizei, _textures: *const GLuint) {}
    pub unsafe fn glColor3iv(&self, _v: *const GLint) {}
    pub unsafe fn glTexCoord3iv(&self, _v: *const GLint) {}
    pub unsafe fn glTexCoord1f(&self, _s: GLfloat) {}
    pub unsafe fn glVertex4dv(&self, _v: *const GLdouble) {}
    pub unsafe fn glDrawPixels(
        &self,
        _width: GLsizei,
        _height: GLsizei,
        _format: PixelFormat,
        _type: PixelType,
        _pixels: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glRasterPos2s(&self, _x: GLshort, _y: GLshort) {}
    pub unsafe fn glColor3ub(&self, _red: GLubyte, _green: GLubyte, _blue: GLubyte) {}
    pub unsafe fn glEvalMesh2(
        &self,
        _mode: MeshMode2,
        _i1: GLint,
        _i2: GLint,
        _j1: GLint,
        _j2: GLint,
    ) {
    }
    pub unsafe fn glIndexdv(&self, _c: *const GLdouble) {}
    pub unsafe fn glTexParameteri(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _param: GLint,
    ) {
    }
    pub unsafe fn glIndexd(&self, _c: GLdouble) {}
    pub unsafe fn glColor3usv(&self, _v: *const GLushort) {}
    pub unsafe fn glVertex2i(&self, _x: GLint, _y: GLint) {}
    pub unsafe fn glMultiTexCoord1s(&self, _target: TextureUnit, _s: GLshort) {}
    pub unsafe fn glRectd(&self, _x1: GLdouble, _y1: GLdouble, _x2: GLdouble, _y2: GLdouble) {}
    pub unsafe fn glTexParameteriv(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _params: *const GLint,
    ) {
    }
    pub unsafe fn glTexCoord3fv(&self, _v: *const GLfloat) {}
    pub unsafe fn glIndexsv(&self, _c: *const GLshort) {}
    pub unsafe fn glDrawBuffer(&self, _buf: DrawBufferMode) {}
    pub unsafe fn glRasterPos2fv(&self, _v: *const GLfloat) {}
    pub unsafe fn glVertex3iv(&self, _v: *const GLint) {}
    pub unsafe fn glLightModeliv(&self, _pname: LightModelParameter, _params: *const GLint) {}
    pub unsafe fn glRenderMode(&self, _mode: RenderingMode) {}
    pub unsafe fn glPolygonMode(&self, _face: MaterialFace, _mode: PolygonMode) {}
    pub unsafe fn glIndexfv(&self, _c: *const GLfloat) {}
    pub unsafe fn glTexCoord1fv(&self, _v: *const GLfloat) {}
    pub unsafe fn glPassThrough(&self, _token: GLfloat) {}
    pub unsafe fn glTexGeniv(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _params: *const GLint,
    ) {
    }
    pub unsafe fn glGetMapiv(&self, _target: MapTarget, _query: GetMapQuery, _v: *mut GLint) {}
    pub unsafe fn glPixelMapuiv(&self, _map: PixelMap, _mapsize: GLsizei, _values: *const GLuint) {}
    pub unsafe fn glTexGend(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _param: GLdouble,
    ) {
    }
    pub unsafe fn glGenTextures(&self, _n: GLsizei, _textures: *mut GLuint) {}
    pub unsafe fn glPixelMapfv(&self, _map: PixelMap, _mapsize: GLsizei, _values: *const GLfloat) {}
    pub unsafe fn glShadeModel(&self, _mode: ShadingModel) {}
    pub unsafe fn glNormal3b(&self, _nx: GLbyte, _ny: GLbyte, _nz: GLbyte) {}
    pub unsafe fn glRectf(&self, _x1: GLfloat, _y1: GLfloat, _x2: GLfloat, _y2: GLfloat) {}
    pub unsafe fn glStencilOp(&self, _fail: StencilOp, _zfail: StencilOp, _zpass: StencilOp) {}
    pub unsafe fn glColor4i(&self, _red: GLint, _green: GLint, _blue: GLint, _alpha: GLint) {}
    pub unsafe fn glTexCoord4d(&self, _s: GLdouble, _t: GLdouble, _r: GLdouble, _q: GLdouble) {}
    pub unsafe fn glVertex3i(&self, _x: GLint, _y: GLint, _z: GLint) {}
    pub unsafe fn glColor3ui(&self, _red: GLuint, _green: GLuint, _blue: GLuint) {}
    pub unsafe fn glClearAccum(
        &self,
        _red: GLfloat,
        _green: GLfloat,
        _blue: GLfloat,
        _alpha: GLfloat,
    ) {
    }
    pub unsafe fn glGetMaterialiv(
        &self,
        _face: MaterialFace,
        _pname: MaterialParameter,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glNormal3d(&self, _nx: GLdouble, _ny: GLdouble, _nz: GLdouble) {}
    pub unsafe fn glVertex2sv(&self, _v: *const GLshort) {}
    pub unsafe fn glGetBooleanv(&self, _pname: GetPName, _data: *mut GLboolean) {}
    pub unsafe fn glClearIndex(&self, _c: GLfloat) {}
    pub unsafe fn glEvalCoord2dv(&self, _u: *const GLdouble) {}
    pub unsafe fn glGetTexImage(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _format: PixelFormat,
        _type: PixelType,
        _pixels: *mut std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glVertex3dv(&self, _v: *const GLdouble) {}
    pub unsafe fn glRectdv(&self, _v1: *const GLdouble, _v2: *const GLdouble) {}
    pub unsafe fn glPixelStoref(&self, _pname: PixelStoreParameter, _param: GLfloat) {}
    pub unsafe fn glMapGrid2f(
        &self,
        _un: GLint,
        _u1: GLfloat,
        _u2: GLfloat,
        _vn: GLint,
        _v1: GLfloat,
        _v2: GLfloat,
    ) {
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
    }
    pub unsafe fn glVertex3f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {}
    pub unsafe fn glMultiTexCoord3d(
        &self,
        _target: TextureUnit,
        _s: GLdouble,
        _t: GLdouble,
        _r: GLdouble,
    ) {
    }
    pub unsafe fn glLoadTransposeMatrixd(&self, _m: *const GLdouble) {}
    pub unsafe fn glNormal3fv(&self, _v: *const GLfloat) {}
    pub unsafe fn glIndexf(&self, _c: GLfloat) {}
    pub unsafe fn glGetTexLevelParameterfv(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _pname: GetTextureParameter,
        _params: *mut GLfloat,
    ) {
    }
    pub unsafe fn glMultiTexCoord1iv(&self, _target: TextureUnit, _v: *const GLint) {}
    pub unsafe fn glFogf(&self, _pname: FogParameter, _param: GLfloat) {}
    pub unsafe fn glGetPixelMapfv(&self, _map: PixelMap, _values: *mut GLfloat) {}
    pub unsafe fn glPolygonStipple(&self, _mask: *const GLubyte) {}
    pub unsafe fn glRotatef(&self, _angle: GLfloat, _x: GLfloat, _y: GLfloat, _z: GLfloat) {}
    pub unsafe fn glMultiTexCoord2i(&self, _target: TextureUnit, _s: GLint, _t: GLint) {}
    pub unsafe fn glPushAttrib(&self, _mask: AttribMask) {}
    pub unsafe fn glGetMaterialfv(
        &self,
        _face: MaterialFace,
        _pname: MaterialParameter,
        _params: *mut GLfloat,
    ) {
    }
    pub unsafe fn glTexGendv(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _params: *const GLdouble,
    ) {
    }
    pub unsafe fn glTexCoord1sv(&self, _v: *const GLshort) {}
    pub unsafe fn glGetMapfv(&self, _target: MapTarget, _query: GetMapQuery, _v: *mut GLfloat) {}
    pub unsafe fn glTexCoord3d(&self, _s: GLdouble, _t: GLdouble, _r: GLdouble) {}
    pub unsafe fn glClipPlane(&self, _plane: ClipPlaneName, _equation: *const GLdouble) {}
    pub unsafe fn glMaterialiv(
        &self,
        _face: MaterialFace,
        _pname: MaterialParameter,
        _params: *const GLint,
    ) {
    }
    pub unsafe fn glLogicOp(&self, _opcode: LogicOp) {}
    pub unsafe fn glGetPixelMapusv(&self, _map: PixelMap, _values: *mut GLushort) {}
    pub unsafe fn glLoadMatrixf(&self, _m: *const GLfloat) {}
    pub unsafe fn glMultiTexCoord4fv(&self, _target: TextureUnit, _v: *const GLfloat) {}
    pub unsafe fn glMultiTexCoord3dv(&self, _target: TextureUnit, _v: *const GLdouble) {}
    pub unsafe fn glFeedbackBuffer(
        &self,
        _size: GLsizei,
        _type: FeedbackType,
        _buffer: *mut GLfloat,
    ) {
    }
    pub unsafe fn glRasterPos2dv(&self, _v: *const GLdouble) {}
    pub unsafe fn glGetFloatv(&self, _pname: GetPName, _data: *mut GLfloat) {}
    pub unsafe fn glColor3s(&self, _red: GLshort, _green: GLshort, _blue: GLshort) {}
    pub unsafe fn glVertex2s(&self, _x: GLshort, _y: GLshort) {}
    pub unsafe fn glRectfv(&self, _v1: *const GLfloat, _v2: *const GLfloat) {}
    pub unsafe fn glMapGrid2d(
        &self,
        _un: GLint,
        _u1: GLdouble,
        _u2: GLdouble,
        _vn: GLint,
        _v1: GLdouble,
        _v2: GLdouble,
    ) {
    }
    pub unsafe fn glTexCoord4s(&self, _s: GLshort, _t: GLshort, _r: GLshort, _q: GLshort) {}
    pub unsafe fn glVertex3sv(&self, _v: *const GLshort) {}
    pub unsafe fn glDisable(&self, _cap: EnableCap) {}
    pub unsafe fn glRecti(&self, _x1: GLint, _y1: GLint, _x2: GLint, _y2: GLint) {}
    pub unsafe fn glColor4dv(&self, _v: *const GLdouble) {}
    pub unsafe fn glRasterPos3dv(&self, _v: *const GLdouble) {}
    pub unsafe fn glTexCoord3sv(&self, _v: *const GLshort) {}
    pub unsafe fn glTexParameterfv(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _params: *const GLfloat,
    ) {
    }
    pub unsafe fn glIndexMask(&self, _mask: GLuint) {}
    pub unsafe fn glNormal3i(&self, _nx: GLint, _ny: GLint, _nz: GLint) {}
    pub unsafe fn glEndList(&self) {}
    pub unsafe fn glRasterPos2f(&self, _x: GLfloat, _y: GLfloat) {}
    pub unsafe fn glGetClipPlane(&self, _plane: ClipPlaneName, _equation: *mut GLdouble) {}
    pub unsafe fn glPopMatrix(&self) {}
    pub unsafe fn glPrioritizeTextures(
        &self,
        _n: GLsizei,
        _textures: *const GLuint,
        _priorities: *const GLfloat,
    ) {
    }
    pub unsafe fn glGetTexGenfv(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _params: *mut GLfloat,
    ) {
    }
    pub unsafe fn glSampleCoverage(&self, _value: GLfloat, _invert: GLboolean) {}
    pub unsafe fn glColor4s(
        &self,
        _red: GLshort,
        _green: GLshort,
        _blue: GLshort,
        _alpha: GLshort,
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
    pub unsafe fn glMapGrid1d(&self, _un: GLint, _u1: GLdouble, _u2: GLdouble) {}
    pub unsafe fn glTexCoord2d(&self, _s: GLdouble, _t: GLdouble) {}
    pub unsafe fn glRasterPos3sv(&self, _v: *const GLshort) {}
    pub unsafe fn glTexCoord1d(&self, _s: GLdouble) {}
    pub unsafe fn glInitNames(&self) {}
    pub unsafe fn glEnd(&self) {}
    pub unsafe fn glEvalCoord1dv(&self, _u: *const GLdouble) {}
    pub unsafe fn glEnableClientState(&self, _array: EnableCap) {}
    pub unsafe fn glMultiTexCoord4dv(&self, _target: TextureUnit, _v: *const GLdouble) {}
    pub unsafe fn glTexCoord1iv(&self, _v: *const GLint) {}
    pub unsafe fn glColor3us(&self, _red: GLushort, _green: GLushort, _blue: GLushort) {}
    pub unsafe fn glNormal3s(&self, _nx: GLshort, _ny: GLshort, _nz: GLshort) {}
    pub unsafe fn glEvalCoord1f(&self, _u: GLfloat) {}
    pub unsafe fn glGetTexParameterfv(
        &self,
        _target: TextureTarget,
        _pname: GetTextureParameter,
        _params: *mut GLfloat,
    ) {
    }
    pub unsafe fn glRasterPos2d(&self, _x: GLdouble, _y: GLdouble) {}
    pub unsafe fn glRotated(&self, _angle: GLdouble, _x: GLdouble, _y: GLdouble, _z: GLdouble) {}
    pub unsafe fn glTexParameterf(
        &self,
        _target: TextureTarget,
        _pname: TextureParameterName,
        _param: GLfloat,
    ) {
    }
    pub unsafe fn glPixelStorei(&self, _pname: PixelStoreParameter, _param: GLint) {}
    pub unsafe fn glColor3uiv(&self, _v: *const GLuint) {}
    pub unsafe fn glPushName(&self, _name: GLuint) {}
    pub unsafe fn glGetPointerv(
        &self,
        _pname: GetPointervPName,
        _params: *mut *mut std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glSelectBuffer(&self, _size: GLsizei, _buffer: *mut GLuint) {}
    pub unsafe fn glEvalPoint1(&self, _i: GLint) {}
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
    pub unsafe fn glMultiTexCoord2s(&self, _target: TextureUnit, _s: GLshort, _t: GLshort) {}
    pub unsafe fn glMultiTexCoord3iv(&self, _target: TextureUnit, _v: *const GLint) {}
    pub unsafe fn glActiveTexture(&self, _texture: TextureUnit) {}
    pub unsafe fn glClearColor(
        &self,
        _red: GLfloat,
        _green: GLfloat,
        _blue: GLfloat,
        _alpha: GLfloat,
    ) {
    }
    pub unsafe fn glTexEnviv(
        &self,
        _target: TextureEnvTarget,
        _pname: TextureEnvParameter,
        _params: *const GLint,
    ) {
    }
    pub unsafe fn glGetIntegerv(&self, _pname: GetPName, _data: *mut GLint) {}
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
    pub unsafe fn glFogiv(&self, _pname: FogParameter, _params: *const GLint) {}
    pub unsafe fn glGetTexLevelParameteriv(
        &self,
        _target: TextureTarget,
        _level: GLint,
        _pname: GetTextureParameter,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glEvalCoord2f(&self, _u: GLfloat, _v: GLfloat) {}
    pub unsafe fn glVertex4s(&self, _x: GLshort, _y: GLshort, _z: GLshort, _w: GLshort) {}
    pub unsafe fn glPixelTransferf(&self, _pname: PixelTransferParameter, _param: GLfloat) {}
    pub unsafe fn glGetTexEnvfv(
        &self,
        _target: TextureEnvTarget,
        _pname: TextureEnvParameter,
        _params: *mut GLfloat,
    ) {
    }
    pub unsafe fn glDepthRange(&self, _n: GLdouble, _f: GLdouble) {}
    pub unsafe fn glEvalCoord2d(&self, _u: GLdouble, _v: GLdouble) {}
    pub unsafe fn glTexEnvfv(
        &self,
        _target: TextureEnvTarget,
        _pname: TextureEnvParameter,
        _params: *const GLfloat,
    ) {
    }
    pub unsafe fn glRasterPos4dv(&self, _v: *const GLdouble) {}
    pub unsafe fn glColor3sv(&self, _v: *const GLshort) {}
    pub unsafe fn glMaterialfv(
        &self,
        _face: MaterialFace,
        _pname: MaterialParameter,
        _params: *const GLfloat,
    ) {
    }
    pub unsafe fn glIsTexture(&self, _texture: GLuint) {}
    pub unsafe fn glTexCoord4sv(&self, _v: *const GLshort) {}
    pub unsafe fn glFinish(&self) {}
    pub unsafe fn glGetTexParameteriv(
        &self,
        _target: TextureTarget,
        _pname: GetTextureParameter,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glColor3f(&self, _red: GLfloat, _green: GLfloat, _blue: GLfloat) {}
    pub unsafe fn glClientActiveTexture(&self, _texture: TextureUnit) {}
    pub unsafe fn glPopAttrib(&self) {}
    pub unsafe fn glMultiTexCoord4s(
        &self,
        _target: TextureUnit,
        _s: GLshort,
        _t: GLshort,
        _r: GLshort,
        _q: GLshort,
    ) {
    }
    pub unsafe fn glTexEnvi(
        &self,
        _target: TextureEnvTarget,
        _pname: TextureEnvParameter,
        _param: GLint,
    ) {
    }
    pub unsafe fn glRasterPos3s(&self, _x: GLshort, _y: GLshort, _z: GLshort) {}
    pub unsafe fn glEdgeFlag(&self, _flag: GLboolean) {}
    pub unsafe fn glBegin(&self, _mode: PrimitiveType) {}
    pub unsafe fn glTexCoord2fv(&self, _v: *const GLfloat) {}
    pub unsafe fn glRasterPos4f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat, _w: GLfloat) {}
    pub unsafe fn glTexCoord2iv(&self, _v: *const GLint) {}
    pub unsafe fn glMultiTexCoord1dv(&self, _target: TextureUnit, _v: *const GLdouble) {}
    pub unsafe fn glLineWidth(&self, _width: GLfloat) {}
    pub unsafe fn glLineStipple(&self, _factor: GLint, _pattern: GLushort) {}
    pub unsafe fn glVertex4sv(&self, _v: *const GLshort) {}
    pub unsafe fn glFrontFace(&self, _mode: FrontFaceDirection) {}
    pub unsafe fn glGetError(&self) {}
    pub unsafe fn glNormal3iv(&self, _v: *const GLint) {}
    pub unsafe fn glTexCoord2s(&self, _s: GLshort, _t: GLshort) {}
    pub unsafe fn glAlphaFunc(&self, _func: AlphaFunction, _ref: GLfloat) {}
    pub unsafe fn glMultiTexCoord3s(
        &self,
        _target: TextureUnit,
        _s: GLshort,
        _t: GLshort,
        _r: GLshort,
    ) {
    }
    pub unsafe fn glLightf(&self, _light: LightName, _pname: LightParameter, _param: GLfloat) {}
    pub unsafe fn glMatrixMode(&self, _mode: MatrixMode) {}
    pub unsafe fn glVertex3s(&self, _x: GLshort, _y: GLshort, _z: GLshort) {}
    pub unsafe fn glIsList(&self, _list: GLuint) {}
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
    pub unsafe fn glArrayElement(&self, _i: GLint) {}
    pub unsafe fn glMultiTexCoord1fv(&self, _target: TextureUnit, _v: *const GLfloat) {}
    pub unsafe fn glColor4fv(&self, _v: *const GLfloat) {}
    pub unsafe fn glGetTexGendv(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _params: *mut GLdouble,
    ) {
    }
    pub unsafe fn glVertex4i(&self, _x: GLint, _y: GLint, _z: GLint, _w: GLint) {}
    pub unsafe fn glPushClientAttrib(&self, _mask: ClientAttribMask) {}
    pub unsafe fn glDepthMask(&self, _flag: GLboolean) {}
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
    pub unsafe fn glColor4sv(&self, _v: *const GLshort) {}
    pub unsafe fn glPixelMapusv(
        &self,
        _map: PixelMap,
        _mapsize: GLsizei,
        _values: *const GLushort,
    ) {
    }
    pub unsafe fn glNewList(&self, _list: GLuint, _mode: ListMode) {}
    pub unsafe fn glVertexPointer(
        &self,
        _size: GLint,
        _type: VertexPointerType,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glGetTexGeniv(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _params: *mut GLint,
    ) {
    }
    pub unsafe fn glRasterPos2sv(&self, _v: *const GLshort) {}
    pub unsafe fn glMultiTexCoord1sv(&self, _target: TextureUnit, _v: *const GLshort) {}
    pub unsafe fn glMapGrid1f(&self, _un: GLint, _u1: GLfloat, _u2: GLfloat) {}
    pub unsafe fn glEvalPoint2(&self, _i: GLint, _j: GLint) {}
    pub unsafe fn glMultiTexCoord2d(&self, _target: TextureUnit, _s: GLdouble, _t: GLdouble) {}
    pub unsafe fn glColor4ui(&self, _red: GLuint, _green: GLuint, _blue: GLuint, _alpha: GLuint) {}
    pub unsafe fn glMultiTexCoord2f(&self, _target: TextureUnit, _s: GLfloat, _t: GLfloat) {}
    pub unsafe fn glVertex4fv(&self, _v: *const GLfloat) {}
    pub unsafe fn glLightModeli(&self, _pname: LightModelParameter, _param: GLint) {}
    pub unsafe fn glVertex2dv(&self, _v: *const GLdouble) {}
    pub unsafe fn glCallLists(
        &self,
        _n: GLsizei,
        _type: ListNameType,
        _lists: *const std::os::raw::c_void,
    ) {
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
    }
    pub unsafe fn glIndexPointer(
        &self,
        _type: IndexPointerType,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glGenLists(&self, _range: GLsizei) {}
    pub unsafe fn glMateriali(
        &self,
        _face: MaterialFace,
        _pname: MaterialParameter,
        _param: GLint,
    ) {
    }
    pub unsafe fn glMultTransposeMatrixd(&self, _m: *const GLdouble) {}
    pub unsafe fn glColor4bv(&self, _v: *const GLbyte) {}
    pub unsafe fn glNormal3dv(&self, _v: *const GLdouble) {}
    pub unsafe fn glGetString(&self, _name: StringName) {}
    pub unsafe fn glRasterPos3iv(&self, _v: *const GLint) {}
    pub unsafe fn glLightModelf(&self, _pname: LightModelParameter, _param: GLfloat) {}
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
    }
    pub unsafe fn glColorPointer(
        &self,
        _size: GLint,
        _type: ColorPointerType,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glMultiTexCoord4sv(&self, _target: TextureUnit, _v: *const GLshort) {}
    pub unsafe fn glTexCoord4i(&self, _s: GLint, _t: GLint, _r: GLint, _q: GLint) {}
    pub unsafe fn glTexCoord2sv(&self, _v: *const GLshort) {}
    pub unsafe fn glTexCoord4f(&self, _s: GLfloat, _t: GLfloat, _r: GLfloat, _q: GLfloat) {}
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
    pub unsafe fn glTexGenfv(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _params: *const GLfloat,
    ) {
    }
    pub unsafe fn glMultiTexCoord1f(&self, _target: TextureUnit, _s: GLfloat) {}
    pub unsafe fn glColor3i(&self, _red: GLint, _green: GLint, _blue: GLint) {}
    pub unsafe fn glRasterPos3d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble) {}
    pub unsafe fn glNormal3bv(&self, _v: *const GLbyte) {}
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
    pub unsafe fn glMultiTexCoord2dv(&self, _target: TextureUnit, _v: *const GLdouble) {}
    pub unsafe fn glColor3bv(&self, _v: *const GLbyte) {}
    pub unsafe fn glReadBuffer(&self, _src: ReadBufferMode) {}
    pub unsafe fn glRasterPos4sv(&self, _v: *const GLshort) {}
    pub unsafe fn glMultiTexCoord2fv(&self, _target: TextureUnit, _v: *const GLfloat) {}
    pub unsafe fn glVertex2iv(&self, _v: *const GLint) {}
    pub unsafe fn glIndexs(&self, _c: GLshort) {}
    pub unsafe fn glIsEnabled(&self, _cap: EnableCap) {}
    pub unsafe fn glColor4f(
        &self,
        _red: GLfloat,
        _green: GLfloat,
        _blue: GLfloat,
        _alpha: GLfloat,
    ) {
    }
    pub unsafe fn glCallList(&self, _list: GLuint) {}
    pub unsafe fn glVertex4iv(&self, _v: *const GLint) {}
    pub unsafe fn glClearStencil(&self, _s: GLint) {}
    pub unsafe fn glDisableClientState(&self, _array: EnableCap) {}
    pub unsafe fn glScalef(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {}
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
    pub unsafe fn glEdgeFlagv(&self, _flag: *const GLboolean) {}
    pub unsafe fn glRasterPos3fv(&self, _v: *const GLfloat) {}
    pub unsafe fn glColor4ub(
        &self,
        _red: GLubyte,
        _green: GLubyte,
        _blue: GLubyte,
        _alpha: GLubyte,
    ) {
    }
    pub unsafe fn glColor4ubv(&self, _v: *const GLubyte) {}
    pub unsafe fn glTexCoord3dv(&self, _v: *const GLdouble) {}
    pub unsafe fn glIndexub(&self, _c: GLubyte) {}
    pub unsafe fn glMultTransposeMatrixf(&self, _m: *const GLfloat) {}
    pub unsafe fn glNormal3f(&self, _nx: GLfloat, _ny: GLfloat, _nz: GLfloat) {}
    pub unsafe fn glIndexiv(&self, _c: *const GLint) {}
    pub unsafe fn glGetPolygonStipple(&self, _mask: *mut GLubyte) {}
    pub unsafe fn glColorMaterial(&self, _face: MaterialFace, _mode: ColorMaterialParameter) {}
    pub unsafe fn glDepthFunc(&self, _func: DepthFunction) {}
    pub unsafe fn glVertex4f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat, _w: GLfloat) {}
    pub unsafe fn glFogi(&self, _pname: FogParameter, _param: GLint) {}
    pub unsafe fn glEvalMesh1(&self, _mode: MeshMode1, _i1: GLint, _i2: GLint) {}
    pub unsafe fn glListBase(&self, _base: GLuint) {}
    pub unsafe fn glTexCoord4fv(&self, _v: *const GLfloat) {}
    pub unsafe fn glEvalCoord1fv(&self, _u: *const GLfloat) {}
    pub unsafe fn glPolygonOffset(&self, _factor: GLfloat, _units: GLfloat) {}
    pub unsafe fn glRasterPos2iv(&self, _v: *const GLint) {}
    pub unsafe fn glColor3d(&self, _red: GLdouble, _green: GLdouble, _blue: GLdouble) {}
    pub unsafe fn glLighti(&self, _light: LightName, _pname: LightParameter, _param: GLint) {}
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
    pub unsafe fn glRasterPos3f(&self, _x: GLfloat, _y: GLfloat, _z: GLfloat) {}
    pub unsafe fn glTexCoord1i(&self, _s: GLint) {}
    pub unsafe fn glDrawElements(
        &self,
        _mode: PrimitiveType,
        _count: GLsizei,
        _type: DrawElementsType,
        _indices: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glColor3ubv(&self, _v: *const GLubyte) {}
    pub unsafe fn glEdgeFlagPointer(
        &self,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glAccum(&self, _op: AccumOp, _value: GLfloat) {}
    pub unsafe fn glTexCoord4iv(&self, _v: *const GLint) {}
    pub unsafe fn glVertex2f(&self, _x: GLfloat, _y: GLfloat) {}
    pub unsafe fn glPopClientAttrib(&self) {}
    pub unsafe fn glColor4d(
        &self,
        _red: GLdouble,
        _green: GLdouble,
        _blue: GLdouble,
        _alpha: GLdouble,
    ) {
    }
    pub unsafe fn glTexCoordPointer(
        &self,
        _size: GLint,
        _type: TexCoordPointerType,
        _stride: GLsizei,
        _pointer: *const std::os::raw::c_void,
    ) {
    }
    pub unsafe fn glTexGeni(
        &self,
        _coord: TextureCoordName,
        _pname: TextureGenParameter,
        _param: GLint,
    ) {
    }
    pub unsafe fn glEvalCoord1d(&self, _u: GLdouble) {}
    pub unsafe fn glGetDoublev(&self, _pname: GetPName, _data: *mut GLdouble) {}
    pub unsafe fn glRasterPos4fv(&self, _v: *const GLfloat) {}
    pub unsafe fn glTexCoord1dv(&self, _v: *const GLdouble) {}
    pub unsafe fn glRasterPos4i(&self, _x: GLint, _y: GLint, _z: GLint, _w: GLint) {}
    pub unsafe fn glTexCoord1s(&self, _s: GLshort) {}
    pub unsafe fn glMultiTexCoord2sv(&self, _target: TextureUnit, _v: *const GLshort) {}
    pub unsafe fn glMultiTexCoord3f(
        &self,
        _target: TextureUnit,
        _s: GLfloat,
        _t: GLfloat,
        _r: GLfloat,
    ) {
    }
    pub unsafe fn glClearDepth(&self, _depth: GLdouble) {}
    pub unsafe fn glVertex4d(&self, _x: GLdouble, _y: GLdouble, _z: GLdouble, _w: GLdouble) {}
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
    }
    pub unsafe fn glTexCoord3s(&self, _s: GLshort, _t: GLshort, _r: GLshort) {}
}
