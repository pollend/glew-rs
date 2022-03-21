use crate::gl::bitflags::{
    BufferStorageMask, ClearBufferMask, MapBufferAccessMask, MemoryBarrierMask,
};
use crate::gl::context::LoadEntryPoint;
use crate::gl::enums::*;
use crate::types::*;
use crate::gl::feature::EntryFnGL45;

use libloading::Library;
use std::ffi::c_void;
use std::ptr;
use std::sync::Arc;

pub trait GL45 {
    fn entry_gl45(&self) -> &crate::gl::feature::EntryFnGL45;

    unsafe fn glGetnConvolutionFilter(
        &self,
        target: ConvolutionTarget,
        format: PixelFormat,
        pixel_type: PixelType,
        data: &[u8],
    ) {
        (self.entry_gl45().glGetnConvolutionFilter)(
            target,
            format,
            pixel_type,
            data.len() as GLsizei,
            data.as_ptr() as _,
        );
    }
    unsafe fn glGetnPixelMapuiv(&self, map: PixelMap, values: &mut [GLuint]) {
        (self.entry_gl45().glGetnPixelMapuiv)(map, values.len() as GLsizei, values.as_mut_ptr());
    }

    unsafe fn glVertexArrayAttribFormat(
        &self,
        va_obj: GLuint,
        attrib_index: GLuint,
        size: GLint,
        attrib_type: VertexAttribType,
        normalized: bool,
        relative_offset: GLuint,
    ) {
        (self.entry_gl45().glVertexArrayAttribFormat)(
            va_obj,
            attrib_index,
            size,
            attrib_type,
            normalized as GLboolean,
            relative_offset,
        );
    }
    unsafe fn glGetCompressedTextureSubImage(
        &self,
        texture: Texture,
        level: GLint,
        x_offset: GLint,
        y_offset: GLint,
        z_offset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        buffers: &mut [u8],
    ) {
        (self.entry_gl45().glGetCompressedTextureSubImage)(
            texture.0,
            level,
            x_offset,
            y_offset,
            z_offset,
            width,
            height,
            depth,
            buffers.len() as GLsizei,
            buffers.as_mut_ptr() as _,
        );
    }
    unsafe fn glGetNamedBufferSubData(&self, buffer: GLuint, offset: GLintptr, data: &mut [u8]) {
        (self.entry_gl45().glGetNamedBufferSubData)(
            buffer,
            offset,
            data.len() as GLsizeiptr,
            data.as_mut_ptr() as _,
        );
    }
    unsafe fn glGetNamedFramebufferParameteriv(
        &self,
        target: GLuint,
        pname: GetFramebufferParameter,
        params: &mut GLint,
    ) {
        (self.entry_gl45().glGetNamedFramebufferParameteriv)(target, pname, params);
    }
    unsafe fn glVertexArrayElementBuffer(&self, va_obj: GLuint, buffer: GLuint) {
        (self.entry_gl45().glVertexArrayElementBuffer)(va_obj, buffer);
    }
    unsafe fn glGetQueryBufferObjecti64v(
        &self,
        id: GLuint,
        buffer: GLuint,
        pname: QueryObjectParameterName,
        offset: GLintptr,
    ) {
        (self.entry_gl45().glGetQueryBufferObjecti64v)(id, buffer, pname, offset);
    }
    unsafe fn glCopyNamedBufferSubData(
        &self,
        read_buffer: GLuint,
        write_buffer: GLuint,
        read_offset: GLintptr,
        write_offset: GLintptr,
        size: GLsizeiptr,
    ) {
        (self.entry_gl45().glCopyNamedBufferSubData)(
            read_buffer,
            write_buffer,
            read_offset,
            write_offset,
            size,
        );
    }
    unsafe fn glNamedFramebufferTextureLayer(
        &self,
        framebuffer: GLuint,
        attachment: FramebufferAttachment,
        texture: GLuint,
        level: GLint,
        layer: GLint,
    ) {
        (self.entry_gl45().glNamedFramebufferTextureLayer)(
            framebuffer,
            attachment,
            texture,
            level,
            layer,
        );
    }
    unsafe fn glCreateProgramPipelines(&self, pipeline: &mut [GLuint]) {
        (self.entry_gl45().glCreateProgramPipelines)(
            pipeline.len() as GLsizei,
            pipeline.as_mut_ptr(),
        );
    }
    unsafe fn glGetNamedBufferPointerv(
        &self,
        buffer: GLuint,
        pname: BufferPointerNameARB,
        params: *mut *mut std::os::raw::c_void,
    ) {
        (self.entry_gl45().glGetNamedBufferPointerv)(buffer, pname, params);
    }

    unsafe fn glCreateFramebuffers(&self, frame_buffers: &mut [GLuint]) {
        (self.entry_gl45().glCreateFramebuffers)(
            frame_buffers.len() as GLsizei,
            frame_buffers.as_mut_ptr(),
        );
    }
    unsafe fn glGetnCompressedTexImage(&self, target: TextureTarget, lod: GLint, data: &mut [u8]) {
        (self.entry_gl45().glGetnCompressedTexImage)(
            target,
            lod,
            data.len() as GLsizei,
            data.as_mut_ptr() as _,
        );
    }
    unsafe fn glGetQueryBufferObjectuiv(
        &self,
        id: GLuint,
        buffer: GLuint,
        pname: QueryObjectParameterName,
        offset: GLintptr,
    ) {
        (self.entry_gl45().glGetQueryBufferObjectuiv)(id, buffer, pname, offset);
    }
    // need validity checks
    unsafe fn glTextureParameterfv(
        &self,
        texture: GLuint,
        pname: TextureParameterName,
        param: &mut [GLfloat],
    ) {
        (self.entry_gl45().glTextureParameterfv)(texture, pname, param.as_mut_ptr());
    }
    unsafe fn glGetnUniformfv(&self, program: GLuint, location: GLint, params: &mut [GLfloat]) {
        (self.entry_gl45().glGetnUniformfv)(
            program,
            location,
            params.len() as GLsizei,
            params.as_mut_ptr(),
        );
    }
    unsafe fn glCreateVertexArrays(&self, arrays: &mut [GLuint]) {
        (self.entry_gl45().glCreateVertexArrays)(arrays.len() as GLsizei, arrays.as_mut_ptr());
    }
    unsafe fn glTransformFeedbackBufferBase(&self, xfb: GLuint, index: GLuint, buffer: GLuint) {
        (self.entry_gl45().glTransformFeedbackBufferBase)(xfb, index, buffer);
    }
    unsafe fn glCompressedTextureSubImage3D(
        &self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: InternalFormat,
        data: &[u8],
    ) {
        (self.entry_gl45().glCompressedTextureSubImage3D)(
            texture,
            level,
            xoffset,
            yoffset,
            zoffset,
            width,
            height,
            depth,
            format,
            data.len() as GLsizei,
            data.as_ptr() as _,
        );
    }
    unsafe fn glNamedFramebufferDrawBuffer(&self, framebuffer: GLuint, buf: ColorBuffer) {
        (self.entry_gl45().glNamedFramebufferDrawBuffer)(framebuffer, buf);
    }
    unsafe fn glTextureStorage3DMultisample(
        &self,
        texture: GLuint,
        samples: GLsizei,
        internalformat: SizedInternalFormat,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        fixedsamplelocations: GLboolean,
    ) {
        (self.entry_gl45().glTextureStorage3DMultisample)(
            texture,
            samples,
            internalformat,
            width,
            height,
            depth,
            fixedsamplelocations,
        );
    }
    unsafe fn glGetVertexArrayIndexediv(
        &self,
        vaobj: GLuint,
        index: GLuint,
        pname: VertexArrayPName,
        param: &mut GLint,
    ) {
        (self.entry_gl45().glGetVertexArrayIndexediv)(vaobj, index, pname, param);
    }
    unsafe fn glGenerateTextureMipmap(&self, texture: GLuint) {
        (self.entry_gl45().glGenerateTextureMipmap)(texture);
    }
    unsafe fn glGetnMapdv(&self, target: MapTarget, query: MapQuery, v: &mut [GLdouble]) {
        (self.entry_gl45().glGetnMapdv)(target, query, v.len() as GLsizei, v.as_mut_ptr());
    }
    unsafe fn glGetNamedFramebufferAttachmentParameteriv(
        &self,
        framebuffer: GLuint,
        attachment: FramebufferAttachment,
        pname: FramebufferAttachmentParameterName,
        params: &mut GLint,
    ) {
        (self.entry_gl45().glGetNamedFramebufferAttachmentParameteriv)(
            framebuffer,
            attachment,
            pname,
            params,
        );
    }
    unsafe fn glCreateBuffers(&self, buffers: &mut [GLuint]) {
        (self.entry_gl45().glCreateBuffers)(buffers.len() as GLsizei, buffers.as_mut_ptr());
    }
    unsafe fn glClearNamedFramebufferiv(
        &self,
        framebuffer: GLuint,
        buffer: Buffer,
        drawbuffer: GLint,
        value: &[GLint],
    ) {
        (self.entry_gl45().glClearNamedFramebufferiv)(
            framebuffer,
            buffer,
            drawbuffer,
            value.as_ptr(),
        );
    }
    unsafe fn glClearNamedFramebufferuiv(
        &self,
        framebuffer: GLuint,
        buffer: Buffer,
        drawbuffer: GLint,
        value: &[GLuint],
    ) {
        (self.entry_gl45().glClearNamedFramebufferuiv)(
            framebuffer,
            buffer,
            drawbuffer,
            value.as_ptr(),
        );
    }
    unsafe fn glTextureBufferRange(
        &self,
        texture: GLuint,
        internalformat: SizedInternalFormat,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr,
    ) {
        (self.entry_gl45().glTextureBufferRange)(texture, internalformat, buffer, offset, size);
    }
    unsafe fn glCreateRenderbuffers(&self, renderbuffers: &mut [GLuint]) {
        (self.entry_gl45().glCreateRenderbuffers)(
            renderbuffers.len() as GLsizei,
            renderbuffers.as_mut_ptr(),
        );
    }

    unsafe fn glTextureBuffer(
        &self,
        texture: GLuint,
        internalformat: SizedInternalFormat,
        buffer: GLuint,
    ) {
        (self.entry_gl45().glTextureBuffer)(texture, internalformat, buffer);
    }
    unsafe fn glGetnHistogram(
        &self,
        target: HistogramTarget,
        reset: GLboolean,
        format: PixelFormat,
        ptype: PixelType,
        bufSize: GLsizei,
        values: *mut std::os::raw::c_void,
    ) {
        (self.entry_gl45().glGetnHistogram)(target, reset, format, ptype, bufSize, values);
    }
    unsafe fn glInvalidateNamedFramebufferData(
        &self,
        framebuffer: GLuint,
        attachments: &[FramebufferAttachment],
    ) {
        (self.entry_gl45().glInvalidateNamedFramebufferData)(
            framebuffer,
            attachments.len() as GLsizei,
            attachments.as_ptr(),
        );
    }
    unsafe fn glCopyTextureSubImage2D(
        &self,
        target: TextureTarget,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        (self.entry_gl45().glCopyTextureSubImage2D)(
            target.0, level, xoffset, yoffset, x, y, width, height,
        );
    }
    unsafe fn glVertexArrayAttribLFormat(
        &self,
        vaobj: GLuint,
        attribindex: GLuint,
        size: GLint,
        attrib_type: VertexAttribLType,
        relativeoffset: GLuint,
    ) {
        (self.entry_gl45().glVertexArrayAttribLFormat)(
            vaobj,
            attribindex,
            size,
            attrib_type,
            relativeoffset,
        );
    }
    unsafe fn glCompressedTextureSubImage2D(
        &self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: InternalFormat,
        data: &mut [u8],
    ) {
        (self.entry_gl45().glCompressedTextureSubImage2D)(
            texture,
            level,
            xoffset,
            yoffset,
            width,
            height,
            format,
            data.len() as GLsizei,
            data.as_ptr() as _,
        );
    }
    unsafe fn glMemoryBarrierByRegion(&self, barriers: MemoryBarrierMask) {
        (self.entry_gl45().glMemoryBarrierByRegion)(barriers);
    }
    unsafe fn glVertexArrayAttribBinding(
        &self,
        vaobj: GLuint,
        attribindex: GLuint,
        bindingindex: GLuint,
    ) {
        (self.entry_gl45().glVertexArrayAttribBinding)(vaobj, attribindex, bindingindex);
    }
    unsafe fn glUnmapNamedBuffer(&self, buffer: GLuint) -> GLboolean {
        (self.entry_gl45().glUnmapNamedBuffer)(buffer)
    }
    unsafe fn glInvalidateNamedFramebufferSubData(
        &self,
        framebuffer: GLuint,
        // _numAttachments: GLsizei,
        attachments: &[FramebufferAttachment],
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        (self.entry_gl45().glInvalidateNamedFramebufferSubData)(
            framebuffer,
            attachments.len() as GLsizei,
            attachments.as_ptr(),
            x,
            y,
            width,
            height,
        );
    }
    unsafe fn glGetCompressedTextureImage(&self, texture: GLuint, level: GLint, pixels: &mut [u8]) {
        (self.entry_gl45().glGetCompressedTextureImage)(
            texture,
            level,
            pixels.len() as GLsizei,
            pixels.as_mut_ptr() as _,
        );
    }
    unsafe fn glClearNamedBufferData(
        &self,
        buffer: GLuint,
        internalformat: SizedInternalFormat,
        format: PixelFormat,
        p_type: PixelType,
        data: &[u8],
    ) {
        (self.entry_gl45().glClearNamedBufferData)(
            buffer,
            internalformat,
            format,
            p_type,
            data.as_ptr() as _,
        );
    }
    unsafe fn glGetTextureParameteriv(
        &self,
        texture: &TextureTarget,
        pname: GetTextureParameter,
        params: &mut [GLint],
    ) {
        (self.entry_gl45().glGetTextureParameteriv)(texture.0, pname, params.as_mut_ptr());
    }
    unsafe fn glGetTextureParameterIuiv(
        &self,
        texture: TextureTarget,
        pname: GetTextureParameter,
        params: &mut [GLuint],
    ) {
        (self.entry_gl45().glGetTextureParameterIuiv)(texture.0, pname, params.as_mut_ptr());
    }

    // TODO: struct to wrap pixeltype
    unsafe fn glGetnTexImage(
        &self,
        target: TextureTarget,
        level: GLint,
        format: PixelFormat,
        p_type: PixelType,
        bufSize: GLsizei,
        pixels: *mut std::os::raw::c_void,
    ) {
        (self.entry_gl45().glGetnTexImage)(target, level, format, p_type, bufSize, pixels);
    }
    unsafe fn glGetTextureImage(
        &self,
        texture: GLuint,
        level: GLint,
        format: PixelFormat,
        ptype: PixelType,
        buf_size: GLsizei,
        pixels: *mut std::os::raw::c_void,
    ) {
        (self.entry_gl45().glGetTextureImage)(texture, level, format, ptype, buf_size, pixels);
    }

    unsafe fn glGetnUniformdv(&self, program: GLuint, location: GLint, params: &mut [GLdouble]) {
        (self.entry_gl45().glGetnUniformdv)(
            program,
            location,
            params.len() as GLsizei,
            params.as_mut_ptr(),
        );
    }
    unsafe fn glCreateSamplers(&self, samplers: &mut [GLuint]) {
        (self.entry_gl45().glCreateSamplers)(samplers.len() as GLsizei, samplers.as_mut_ptr());
    }
    unsafe fn glGetNamedBufferParameteri64v(
        &self,
        buffer: GLuint,
        pname: BufferPNameARB,
        params: &mut [GLint64],
    ) {
        (self.entry_gl45().glGetNamedBufferParameteri64v)(buffer, pname, params.as_mut_ptr());
    }
    unsafe fn glNamedBufferSubData(&self, buffer: GLuint, offset: GLintptr, data: &[u8]) {
        (self.entry_gl45().glNamedBufferSubData)(
            buffer,
            offset,
            data.len() as GLsizeiptr,
            data.as_ptr() as _,
        );
    }
    unsafe fn glCopyTextureSubImage1D(
        &self,
        texture: TextureTarget,
        level: GLint,
        xoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
    ) {
        (self.entry_gl45().glCopyTextureSubImage1D)(texture.0, level, xoffset, x, y, width);
    }
    unsafe fn glNamedBufferData(
        &self,
        buffer: GLuint,
        size: GLsizeiptr,
        data: *const std::os::raw::c_void,
        usage: VertexBufferObjectUsage,
    ) {
        (self.entry_gl45().glNamedBufferData)(buffer, size, data, usage);
    }
    unsafe fn glTextureParameteri(
        &self,
        texture: GLuint,
        pname: TextureParameterName,
        param: GLint,
    ) {
        (self.entry_gl45().glTextureParameteri)(texture, pname, param);
    }
    unsafe fn glNamedRenderbufferStorageMultisample(
        &self,
        renderbuffer: GLuint,
        samples: GLsizei,
        internalformat: InternalFormat,
        width: GLsizei,
        height: GLsizei,
    ) {
        (self.entry_gl45().glNamedRenderbufferStorageMultisample)(
            renderbuffer,
            samples,
            internalformat,
            width,
            height,
        );
    }
    unsafe fn glNamedFramebufferTexture(
        &self,
        framebuffer: GLuint,
        attachment: FramebufferAttachment,
        texture: GLuint,
        level: GLint,
    ) {
        (self.entry_gl45().glNamedFramebufferTexture)(framebuffer, attachment, texture, level);
    }
    unsafe fn glCreateQueries(&self, target: QueryTarget, ids: &mut [GLuint]) {
        (self.entry_gl45().glCreateQueries)(target, ids.len() as GLsizei, ids.as_mut_ptr());
    }
    unsafe fn glClearNamedFramebufferfv(
        &self,
        framebuffer: GLuint,
        buffer: Buffer,
        drawbuffer: GLint,
        value: *const GLfloat,
    ) {
        (self.entry_gl45().glClearNamedFramebufferfv)(framebuffer, buffer, drawbuffer, value);
    }
    unsafe fn glEnableVertexArrayAttrib(&self, va_obj: GLuint, index: GLuint) {
        (self.entry_gl45().glEnableVertexArrayAttrib)(va_obj, index);
    }
    unsafe fn glGetVertexArrayIndexed64iv(
        &self,
        vaobj: GLuint,
        index: GLuint,
        pname: VertexArrayPName,
        param: *mut GLint64,
    ) {
        (self.entry_gl45().glGetVertexArrayIndexed64iv)(vaobj, index, pname, param);
    }
    unsafe fn glGetNamedRenderbufferParameteriv(
        &self,
        renderbuffer: GLuint,
        pname: RenderbufferParameterName,
        params: *mut GLint,
    ) {
        (self.entry_gl45().glGetNamedRenderbufferParameteriv)(renderbuffer, pname, params);
    }
    unsafe fn glTextureSubImage1D(
        &self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        width: GLsizei,
        format: PixelFormat,
        ptype: PixelType,
        pixels: *const std::os::raw::c_void,
    ) {
        (self.entry_gl45().glTextureSubImage1D)(
            texture, level, xoffset, width, format, ptype, pixels,
        );
    }
    unsafe fn glGetGraphicsResetStatus(&self) -> GraphicsResetStatus {
        (self.entry_gl45().glGetGraphicsResetStatus)()
    }
    unsafe fn glCopyTextureSubImage3D(
        &self,
        texture: TextureTarget,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        (self.entry_gl45().glCopyTextureSubImage3D)(
            texture.0, level, xoffset, yoffset, zoffset, x, y, width, height,
        );
    }
    unsafe fn glTextureParameterIiv(
        &self,
        texture: TextureTarget,
        pname: TextureParameterName,
        params: *const GLint,
    ) {
        (self.entry_gl45().glTextureParameterIiv)(texture.0, pname, params);
    }
    unsafe fn glGetTextureLevelParameteriv(
        &self,
        texture: TextureTarget,
        level: GLint,
        pname: GetTextureParameter,
        params: *mut GLint,
    ) {
        (self.entry_gl45().glGetTextureLevelParameteriv)(texture.0, level, pname, params);
    }
    unsafe fn glGetQueryBufferObjectiv(
        &self,
        id: GLuint,
        buffer: GLuint,
        pname: QueryObjectParameterName,
        offset: GLintptr,
    ) {
        (self.entry_gl45().glGetQueryBufferObjectiv)(id, buffer, pname, offset);
    }
    unsafe fn glTextureStorage2DMultisample(
        &self,
        texture: TextureTarget,
        samples: GLsizei,
        internalformat: SizedInternalFormat,
        width: GLsizei,
        height: GLsizei,
        fixedsamplelocations: GLboolean,
    ) {
        (self.entry_gl45().glTextureStorage2DMultisample)(
            texture.0,
            samples,
            internalformat,
            width,
            height,
            fixedsamplelocations,
        );
    }
    unsafe fn glTextureParameterf(
        &self,
        texture: TextureTarget,
        pname: TextureParameterName,
        param: GLfloat,
    ) {
        (self.entry_gl45().glTextureParameterf)(texture.0, pname, param);
    }
    unsafe fn glGetQueryBufferObjectui64v(
        &self,
        id: GLuint,
        buffer: GLuint,
        pname: QueryObjectParameterName,
        offset: GLintptr,
    ) {
        (self.entry_gl45().glGetQueryBufferObjectui64v)(id, buffer, pname, offset);
    }
    unsafe fn glGetnUniformiv(
        &self,
        program: GLuint,
        location: GLint,
        bufSize: GLsizei,
        params: *mut GLint,
    ) {
        (self.entry_gl45().glGetnUniformiv)(program, location, bufSize, params);
    }
    unsafe fn glNamedFramebufferRenderbuffer(
        &self,
        framebuffer: GLuint,
        attachment: FramebufferAttachment,
        renderbuffertarget: RenderbufferTarget,
        renderbuffer: GLuint,
    ) {
        (self.entry_gl45().glNamedFramebufferRenderbuffer)(
            framebuffer,
            attachment,
            renderbuffertarget,
            renderbuffer,
        );
    }
    unsafe fn glGetTransformFeedbackiv(
        &self,
        xfb: GLuint,
        pname: TransformFeedbackPName,
        param: *mut GLint,
    ) {
        (self.entry_gl45().glGetTransformFeedbackiv)(xfb, pname, param);
    }
    unsafe fn glCreateTextures(&self, target: TextureTarget, textures: &mut [GLuint]) {
        (self.entry_gl45().glCreateTextures)(
            target,
            textures.len() as GLsizei,
            textures.as_mut_ptr(),
        );
    }
    unsafe fn glGetnMapfv(
        &self,
        target: MapTarget,
        query: MapQuery,
        bufSize: GLsizei,
        v: *mut GLfloat,
    ) {
        (self.entry_gl45().glGetnMapfv)(target, query, bufSize, v);
    }
    unsafe fn glGetTextureLevelParameterfv(
        &self,
        texture: GLuint,
        level: GLint,
        pname: GetTextureParameter,
        params: *mut GLfloat,
    ) {
        (self.entry_gl45().glGetTextureLevelParameterfv)(texture, level, pname, params);
    }
    unsafe fn glGetnPixelMapfv(&self, map: PixelMap, bufSize: GLsizei, values: *mut GLfloat) {
        (self.entry_gl45().glGetnPixelMapfv)(map, bufSize, values);
    }
    unsafe fn glGetnPolygonStipple(&self, bufSize: GLsizei, pattern: *mut GLubyte) {
        (self.entry_gl45().glGetnPolygonStipple)(bufSize, pattern);
    }
    unsafe fn glGetnSeparableFilter(
        &self,
        target: SeparableTarget,
        format: PixelFormat,
        ptype: PixelType,
        rowBufSize: GLsizei,
        row: *mut std::os::raw::c_void,
        columnBufSize: GLsizei,
        column: *mut std::os::raw::c_void,
        span: *mut std::os::raw::c_void,
    ) {
        (self.entry_gl45().glGetnSeparableFilter)(
            target,
            format,
            ptype,
            rowBufSize,
            row,
            columnBufSize,
            column,
            span,
        );
    }
    unsafe fn glNamedBufferStorage(
        &self,
        buffer: GLuint,
        size: GLsizeiptr,
        data: *const std::os::raw::c_void,
        flags: BufferStorageMask,
    ) {
        (self.entry_gl45().glNamedBufferStorage)(buffer, size, data, flags);
    }
    unsafe fn glTextureStorage3D(
        &self,
        texture: TextureTarget,
        levels: GLsizei,
        internalformat: SizedInternalFormat,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
    ) {
        (self.entry_gl45().glTextureStorage3D)(
            texture.0,
            levels,
            internalformat,
            width,
            height,
            depth,
        );
    }
    unsafe fn glVertexArrayBindingDivisor(
        &self,
        vaobj: GLuint,
        bindingindex: GLuint,
        divisor: GLuint,
    ) {
        (self.entry_gl45().glVertexArrayBindingDivisor)(vaobj, bindingindex, divisor);
    }
    unsafe fn glCheckNamedFramebufferStatus(&self, framebuffer: GLuint, target: FramebufferTarget) {
        (self.entry_gl45().glCheckNamedFramebufferStatus)(framebuffer, target);
    }
    unsafe fn glCreateTransformFeedbacks(&self, ids: &mut [GLuint]) {
        (self.entry_gl45().glCreateTransformFeedbacks)(ids.len() as GLsizei, ids.as_mut_ptr());
    }
    unsafe fn glGetTextureParameterfv(
        &self,
        texture: GLuint,
        pname: GetTextureParameter,
        params: *mut GLfloat,
    ) {
        (self.entry_gl45().glGetTextureParameterfv)(texture, pname, params);
    }
    unsafe fn glGetTransformFeedbacki64_v(
        &self,
        xfb: GLuint,
        pname: TransformFeedbackPName,
        index: GLuint,
        param: *mut GLint64,
    ) {
        (self.entry_gl45().glGetTransformFeedbacki64_v)(xfb, pname, index, param);
    }
    unsafe fn glNamedFramebufferReadBuffer(&self, framebuffer: GLuint, src: ColorBuffer) {
        (self.entry_gl45().glNamedFramebufferReadBuffer)(framebuffer, src);
    }
    unsafe fn glTextureSubImage2D(
        &self,
        texture: TextureTarget,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: PixelFormat,
        ptype: PixelType,
        pixels: *const std::os::raw::c_void,
    ) {
        (self.entry_gl45().glTextureSubImage2D)(
            texture.0, level, xoffset, yoffset, width, height, format, ptype, pixels,
        );
    }
    unsafe fn glGetTransformFeedbacki_v(
        &self,
        xfb: GLuint,
        pname: TransformFeedbackPName,
        index: GLuint,
        param: *mut GLint,
    ) {
        (self.entry_gl45().glGetTransformFeedbacki_v)(xfb, pname, index, param);
    }
    unsafe fn glTextureParameteriv(
        &self,
        texture: GLuint,
        pname: TextureParameterName,
        param: *const GLint,
    ) {
        (self.entry_gl45().glTextureParameteriv)(texture, pname, param);
    }
    unsafe fn glVertexArrayVertexBuffer(
        &self,
        vaobj: GLuint,
        bindingindex: GLuint,
        buffer: GLuint,
        offset: GLintptr,
        stride: GLsizei,
    ) {
        (self.entry_gl45().glVertexArrayVertexBuffer)(vaobj, bindingindex, buffer, offset, stride);
    }
    unsafe fn glVertexArrayAttribIFormat(
        &self,
        vaobj: GLuint,
        attribindex: GLuint,
        size: GLint,
        vtype: VertexAttribIType,
        relativeoffset: GLuint,
    ) {
        (self.entry_gl45().glVertexArrayAttribIFormat)(
            vaobj,
            attribindex,
            size,
            vtype,
            relativeoffset,
        );
    }
    unsafe fn glNamedFramebufferDrawBuffers(
        &self,
        framebuffer: GLuint,
        n: GLsizei,
        bufs: *const ColorBuffer,
    ) {
        (self.entry_gl45().glNamedFramebufferDrawBuffers)(framebuffer, n, bufs);
    }
    unsafe fn glVertexArrayVertexBuffers(
        &self,
        vaobj: GLuint,
        first: GLuint,
        count: GLsizei,
        buffers: *const GLuint,
        offsets: *const GLintptr,
        strides: *const GLsizei,
    ) {
        (self.entry_gl45().glVertexArrayVertexBuffers)(
            vaobj, first, count, buffers, offsets, strides,
        );
    }
    unsafe fn glCompressedTextureSubImage1D(
        &self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        width: GLsizei,
        format: InternalFormat,
        imageSize: GLsizei,
        data: *const std::os::raw::c_void,
    ) {
        (self.entry_gl45().glCompressedTextureSubImage1D)(
            texture, level, xoffset, width, format, imageSize, data,
        );
    }
    unsafe fn glBlitNamedFramebuffer(
        &self,
        readFramebuffer: GLuint,
        drawFramebuffer: GLuint,
        srcX0: GLint,
        srcY0: GLint,
        srcX1: GLint,
        srcY1: GLint,
        dstX0: GLint,
        dstY0: GLint,
        dstX1: GLint,
        dstY1: GLint,
        mask: ClearBufferMask,
        filter: BlitFramebufferFilter,
    ) {
        (self.entry_gl45().glBlitNamedFramebuffer)(
            readFramebuffer,
            drawFramebuffer,
            srcX0,
            srcY0,
            srcX1,
            srcY1,
            dstX0,
            dstY0,
            dstX1,
            dstY1,
            mask,
            filter,
        );
    }
    unsafe fn glGetnMapiv(
        &self,
        target: MapTarget,
        query: MapQuery,
        bufSize: GLsizei,
        v: *mut GLint,
    ) {
        (self.entry_gl45().glGetnMapiv)(target, query, bufSize, v);
    }
    unsafe fn glClearNamedFramebufferfi(
        &self,
        framebuffer: GLuint,
        buffer: Buffer,
        drawbuffer: GLint,
        depth: GLfloat,
        stencil: GLint,
    ) {
        (self.entry_gl45().glClearNamedFramebufferfi)(
            framebuffer,
            buffer,
            drawbuffer,
            depth,
            stencil,
        );
    }
    unsafe fn glTextureStorage2D(
        &self,
        texture: GLuint,
        levels: GLsizei,
        internalformat: SizedInternalFormat,
        width: GLsizei,
        height: GLsizei,
    ) {
        (self.entry_gl45().glTextureStorage2D)(texture, levels, internalformat, width, height);
    }
    unsafe fn glNamedFramebufferParameteri(
        &self,
        framebuffer: GLuint,
        pname: FramebufferParameterName,
        param: GLint,
    ) {
        (self.entry_gl45().glNamedFramebufferParameteri)(framebuffer, pname, param);
    }
    unsafe fn glTextureParameterIuiv(
        &self,
        texture: GLuint,
        pname: TextureParameterName,
        params: *const GLuint,
    ) {
        (self.entry_gl45().glTextureParameterIuiv)(texture, pname, params);
    }
    unsafe fn glMapNamedBufferRange(
        &self,
        buffer: GLuint,
        offset: GLintptr,
        length: GLsizeiptr,
        access: MapBufferAccessMask,
    ) -> *mut std::os::raw::c_void {
        (self.entry_gl45().glMapNamedBufferRange)(buffer, offset, length, access)
    }
    unsafe fn glGetnUniformuiv(
        &self,
        program: GLuint,
        location: GLint,
        bufSize: GLsizei,
        params: *mut GLuint,
    ) {
        (self.entry_gl45().glGetnUniformuiv)(program, location, bufSize, params)
    }
    unsafe fn glBindTextureUnit(&self, unit: GLuint, texture: GLuint) {
        (self.entry_gl45().glBindTextureUnit)(unit, texture)
    }
    unsafe fn glClearNamedBufferSubData(
        &self,
        buffer: GLuint,
        internalformat: SizedInternalFormat,
        offset: GLintptr,
        size: GLsizeiptr,
        format: PixelFormat,
        ptype: PixelType,
        data: *const std::os::raw::c_void,
    ) {
        (self.entry_gl45().glClearNamedBufferSubData)(
            buffer,
            internalformat,
            offset,
            size,
            format,
            ptype,
            data,
        )
    }
    unsafe fn glGetnColorTable(
        &self,
        target: ColorTableTarget,
        format: PixelFormat,
        ptype: PixelType,
        bufSize: GLsizei,
        table: *mut std::os::raw::c_void,
    ) {
        (self.entry_gl45().glGetnColorTable)(target, format, ptype, bufSize, table)
    }
    unsafe fn glDisableVertexArrayAttrib(&self, vaobj: GLuint, index: GLuint) {
        (self.entry_gl45().glDisableVertexArrayAttrib)(vaobj, index)
    }
    unsafe fn glClipControl(&self, origin: ClipControlOrigin, depth: ClipControlDepth) {
        (self.entry_gl45().glClipControl)(origin, depth)
    }
    unsafe fn glNamedRenderbufferStorage(
        &self,
        renderbuffer: GLuint,
        internalformat: InternalFormat,
        width: GLsizei,
        height: GLsizei,
    ) {
        (self.entry_gl45().glNamedRenderbufferStorage)(renderbuffer, internalformat, width, height)
    }
    unsafe fn glFlushMappedNamedBufferRange(
        &self,
        buffer: GLuint,
        offset: GLintptr,
        length: GLsizeiptr,
    ) {
        (self.entry_gl45().glFlushMappedNamedBufferRange)(buffer, offset, length)
    }
    unsafe fn glTextureBarrier(&self) {
        (self.entry_gl45().glTextureBarrier)()
    }
    unsafe fn glTextureStorage1D(
        &self,
        texture: GLuint,
        levels: GLsizei,
        internalformat: SizedInternalFormat,
        width: GLsizei,
    ) {
        (self.entry_gl45().glTextureStorage1D)(texture, levels, internalformat, width)
    }
    unsafe fn glGetNamedBufferParameteriv(
        &self,
        buffer: GLuint,
        pname: BufferPNameARB,
        params: *mut GLint,
    ) {
        (self.entry_gl45().glGetNamedBufferParameteriv)(buffer, pname, params)
    }
    unsafe fn glGetTextureSubImage(
        &self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: PixelFormat,
        ptype: PixelType,
        bufSize: GLsizei,
        pixels: *mut std::os::raw::c_void,
    ) {
        (self.entry_gl45().glGetTextureSubImage)(
            texture, level, xoffset, yoffset, zoffset, width, height, depth, format, ptype,
            bufSize, pixels,
        )
    }
    unsafe fn glReadnPixels(
        &self,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: PixelFormat,
        ptype: PixelType,
        bufSize: GLsizei,
        data: *mut std::os::raw::c_void,
    ) {
        (self.entry_gl45().glReadnPixels)(x, y, width, height, format, ptype, bufSize, data)
    }
    unsafe fn glGetnMinmax(
        &self,
        target: MinmaxTarget,
        reset: GLboolean,
        format: PixelFormat,
        ptype: PixelType,
        bufSize: GLsizei,
        values: *mut std::os::raw::c_void,
    ) {
        (self.entry_gl45().glGetnMinmax)(target, reset, format, ptype, bufSize, values)
    }
    unsafe fn glGetTextureParameterIiv(
        &self,
        texture: GLuint,
        pname: GetTextureParameter,
        params: *mut GLint,
    ) {
        (self.entry_gl45().glGetTextureParameterIiv)(texture, pname, params)
    }
    unsafe fn glTransformFeedbackBufferRange(
        &self,
        xfb: GLuint,
        index: GLuint,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr,
    ) {
        (self.entry_gl45().glTransformFeedbackBufferRange)(xfb, index, buffer, offset, size)
    }
    unsafe fn glGetnPixelMapusv(&self, map: PixelMap, bufSize: GLsizei, values: *mut GLushort) {
        (self.entry_gl45().glGetnPixelMapusv)(map, bufSize, values)
    }
    unsafe fn glTextureSubImage3D(
        &self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: PixelFormat,
        ptype: PixelType,
        pixels: *const std::os::raw::c_void,
    ) {
        (self.entry_gl45().glTextureSubImage3D)(
            texture, level, xoffset, yoffset, zoffset, width, height, depth, format, ptype, pixels,
        )
    }
    unsafe fn glMapNamedBuffer(
        &self,
        buffer: GLuint,
        access: BufferAccessARB,
    ) -> *mut std::os::raw::c_void {
        (self.entry_gl45().glMapNamedBuffer)(buffer, access)
    }
    unsafe fn glGetVertexArrayiv(&self, vaobj: GLuint, pname: VertexArrayPName, param: *mut GLint) {
        (self.entry_gl45().glGetVertexArrayiv)(vaobj, pname, param)
    }
}
