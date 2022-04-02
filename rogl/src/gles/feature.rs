use crate::types::*;
use std::ffi::c_void;
#[derive(Clone)]
pub struct EntryGLESFn {
    #[cfg(any(
        feature = "gles32",
        feature = "gles30",
        feature = "gles20",
        feature = "gles31",
    ))]
    pub glRenderbufferStorage: crate::command::PFN_glRenderbufferStorage,
    #[cfg(any(
        feature = "gles20",
        feature = "gles31",
        feature = "gles10",
        feature = "gles30",
        feature = "gles32",
    ))]
    pub glHint: crate::command::PFN_glHint,
    #[cfg(any(
        feature = "gles31",
        feature = "gles30",
        feature = "gles32",
        feature = "gles20",
    ))]
    pub glUniform4fv: crate::command::PFN_glUniform4fv,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glGetFramebufferParameteriv: crate::command::PFN_glGetFramebufferParameteriv,
    #[cfg(any(
        feature = "gles31",
        feature = "gles32",
        feature = "gles10",
        feature = "gles20",
        feature = "gles30",
    ))]
    pub glScissor: crate::command::PFN_glScissor,
    #[cfg(any(
        feature = "gles31",
        feature = "gles32",
        feature = "gles30",
        feature = "gles20",
        feature = "gles10",
    ))]
    pub glIsTexture: crate::command::PFN_glIsTexture,
    #[cfg(any(
        feature = "gles32",
        feature = "gles30",
        feature = "gles20",
        feature = "gles31",
    ))]
    pub glUniform4iv: crate::command::PFN_glUniform4iv,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glBlitFramebuffer: crate::command::PFN_glBlitFramebuffer,
    #[cfg(any(feature = "gles10",))]
    pub glTexEnvi: crate::command::PFN_glTexEnvi,
    #[cfg(any(
        feature = "gles31",
        feature = "gles30",
        feature = "gles32",
        feature = "gles20",
    ))]
    pub glCompileShader: crate::command::PFN_glCompileShader,
    #[cfg(any(feature = "gles10",))]
    pub glLightModelf: crate::command::PFN_glLightModelf,
    #[cfg(any(
        feature = "gles31",
        feature = "gles30",
        feature = "gles32",
        feature = "gles20",
    ))]
    pub glUseProgram: crate::command::PFN_glUseProgram,
    #[cfg(any(
        feature = "gles30",
        feature = "gles32",
        feature = "gles20",
        feature = "gles31",
    ))]
    pub glUniformMatrix4fv: crate::command::PFN_glUniformMatrix4fv,
    #[cfg(any(
        feature = "gles30",
        feature = "gles31",
        feature = "gles20",
        feature = "gles32",
    ))]
    pub glValidateProgram: crate::command::PFN_glValidateProgram,
    #[cfg(any(
        feature = "gles20",
        feature = "gles31",
        feature = "gles30",
        feature = "gles32",
    ))]
    pub glGetShaderInfoLog: crate::command::PFN_glGetShaderInfoLog,
    #[cfg(any(
        feature = "gles20",
        feature = "gles31",
        feature = "gles32",
        feature = "gles30",
    ))]
    pub glUniform1f: crate::command::PFN_glUniform1f,
    #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
    pub glProgramParameteri: crate::command::PFN_glProgramParameteri,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glProgramUniform1fv: crate::command::PFN_glProgramUniform1fv,
    #[cfg(any(
        feature = "gles31",
        feature = "gles30",
        feature = "gles32",
        feature = "gles20",
    ))]
    pub glDeleteProgram: crate::command::PFN_glDeleteProgram,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glProgramUniform2ui: crate::command::PFN_glProgramUniform2ui,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glProgramUniform4fv: crate::command::PFN_glProgramUniform4fv,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glProgramUniform1iv: crate::command::PFN_glProgramUniform1iv,
    #[cfg(any(feature = "gles10",))]
    pub glMultMatrixx: crate::command::PFN_glMultMatrixx,
    #[cfg(any(feature = "gles10",))]
    pub glFogf: crate::command::PFN_glFogf,
    #[cfg(any(feature = "gles10",))]
    pub glLineWidthx: crate::command::PFN_glLineWidthx,
    #[cfg(any(feature = "gles10",))]
    pub glTexEnvx: crate::command::PFN_glTexEnvx,
    #[cfg(any(feature = "gles30", feature = "gles32", feature = "gles31",))]
    pub glGetBufferPointerv: crate::command::PFN_glGetBufferPointerv,
    #[cfg(any(feature = "gles32", feature = "gles31", feature = "gles30",))]
    pub glClearBufferfi: crate::command::PFN_glClearBufferfi,
    #[cfg(any(
        feature = "gles30",
        feature = "gles32",
        feature = "gles31",
        feature = "gles20",
    ))]
    pub glVertexAttrib2f: crate::command::PFN_glVertexAttrib2f,
    #[cfg(any(
        feature = "gles20",
        feature = "gles31",
        feature = "gles30",
        feature = "gles32",
    ))]
    pub glUniform1iv: crate::command::PFN_glUniform1iv,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glProgramUniformMatrix3x4fv: crate::command::PFN_glProgramUniformMatrix3x4fv,
    #[cfg(any(feature = "gles10",))]
    pub glMaterialxv: crate::command::PFN_glMaterialxv,
    #[cfg(any(
        feature = "gles10",
        feature = "gles20",
        feature = "gles30",
        feature = "gles31",
        feature = "gles32",
    ))]
    pub glCompressedTexSubImage2D: crate::command::PFN_glCompressedTexSubImage2D,
    #[cfg(any(feature = "gles32", feature = "gles31", feature = "gles30",))]
    pub glCopyBufferSubData: crate::command::PFN_glCopyBufferSubData,
    #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
    pub glGetSamplerParameteriv: crate::command::PFN_glGetSamplerParameteriv,
    #[cfg(any(feature = "gles31", feature = "gles30", feature = "gles32",))]
    pub glVertexAttribI4i: crate::command::PFN_glVertexAttribI4i,
    #[cfg(any(feature = "gles10",))]
    pub glLoadMatrixx: crate::command::PFN_glLoadMatrixx,
    #[cfg(any(feature = "gles32",))]
    pub glTexParameterIuiv: crate::command::PFN_glTexParameterIuiv,
    #[cfg(any(feature = "gles32",))]
    pub glDrawElementsBaseVertex: crate::command::PFN_glDrawElementsBaseVertex,
    #[cfg(any(
        feature = "gles31",
        feature = "gles30",
        feature = "gles32",
        feature = "gles20",
    ))]
    pub glFramebufferRenderbuffer: crate::command::PFN_glFramebufferRenderbuffer,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glBindProgramPipeline: crate::command::PFN_glBindProgramPipeline,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glProgramUniform4ui: crate::command::PFN_glProgramUniform4ui,
    #[cfg(any(feature = "gles10",))]
    pub glPointSize: crate::command::PFN_glPointSize,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glBeginQuery: crate::command::PFN_glBeginQuery,
    #[cfg(any(feature = "gles10",))]
    pub glScalef: crate::command::PFN_glScalef,
    #[cfg(any(
        feature = "gles30",
        feature = "gles20",
        feature = "gles10",
        feature = "gles31",
        feature = "gles32",
    ))]
    pub glClearStencil: crate::command::PFN_glClearStencil,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glGetProgramResourceiv: crate::command::PFN_glGetProgramResourceiv,
    #[cfg(any(feature = "gles32",))]
    pub glPatchParameteri: crate::command::PFN_glPatchParameteri,
    #[cfg(any(feature = "gles32",))]
    pub glTexParameterIiv: crate::command::PFN_glTexParameterIiv,
    #[cfg(any(feature = "gles32",))]
    pub glBlendEquationi: crate::command::PFN_glBlendEquationi,
    #[cfg(any(
        feature = "gles30",
        feature = "gles20",
        feature = "gles31",
        feature = "gles32",
    ))]
    pub glBindAttribLocation: crate::command::PFN_glBindAttribLocation,
    #[cfg(any(feature = "gles10",))]
    pub glTranslatex: crate::command::PFN_glTranslatex,
    #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
    pub glGenSamplers: crate::command::PFN_glGenSamplers,
    #[cfg(any(
        feature = "gles20",
        feature = "gles31",
        feature = "gles30",
        feature = "gles32",
    ))]
    pub glVertexAttrib3f: crate::command::PFN_glVertexAttrib3f,
    #[cfg(any(
        feature = "gles20",
        feature = "gles31",
        feature = "gles32",
        feature = "gles30",
    ))]
    pub glGetVertexAttribiv: crate::command::PFN_glGetVertexAttribiv,
    #[cfg(any(
        feature = "gles20",
        feature = "gles32",
        feature = "gles31",
        feature = "gles30",
    ))]
    pub glDisableVertexAttribArray: crate::command::PFN_glDisableVertexAttribArray,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glGetBufferParameteri64v: crate::command::PFN_glGetBufferParameteri64v,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glFramebufferTextureLayer: crate::command::PFN_glFramebufferTextureLayer,
    #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
    pub glProgramBinary: crate::command::PFN_glProgramBinary,
    #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
    pub glRenderbufferStorageMultisample: crate::command::PFN_glRenderbufferStorageMultisample,
    #[cfg(any(feature = "gles10",))]
    pub glDisableClientState: crate::command::PFN_glDisableClientState,
    #[cfg(any(feature = "gles30", feature = "gles32", feature = "gles31",))]
    pub glGetIntegeri_v: crate::command::PFN_glGetIntegeri_v,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glProgramUniformMatrix2x4fv: crate::command::PFN_glProgramUniformMatrix2x4fv,
    #[cfg(any(feature = "gles10",))]
    pub glClipPlanef: crate::command::PFN_glClipPlanef,
    #[cfg(any(feature = "gles10",))]
    pub glTexEnvf: crate::command::PFN_glTexEnvf,
    #[cfg(any(feature = "gles10",))]
    pub glTexEnvxv: crate::command::PFN_glTexEnvxv,
    #[cfg(any(
        feature = "gles31",
        feature = "gles30",
        feature = "gles32",
        feature = "gles20",
    ))]
    pub glBlendEquation: crate::command::PFN_glBlendEquation,
    #[cfg(any(feature = "gles32", feature = "gles31", feature = "gles30",))]
    pub glGetActiveUniformBlockName: crate::command::PFN_glGetActiveUniformBlockName,
    #[cfg(any(feature = "gles10",))]
    pub glGetTexParameterxv: crate::command::PFN_glGetTexParameterxv,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glFlushMappedBufferRange: crate::command::PFN_glFlushMappedBufferRange,
    #[cfg(any(feature = "gles10",))]
    pub glFrustumf: crate::command::PFN_glFrustumf,
    #[cfg(any(feature = "gles10",))]
    pub glLightf: crate::command::PFN_glLightf,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glVertexAttribDivisor: crate::command::PFN_glVertexAttribDivisor,
    #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
    pub glBindVertexArray: crate::command::PFN_glBindVertexArray,
    #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
    pub glGenTransformFeedbacks: crate::command::PFN_glGenTransformFeedbacks,
    #[cfg(any(feature = "gles32",))]
    pub glTexBufferRange: crate::command::PFN_glTexBufferRange,
    #[cfg(any(
        feature = "gles32",
        feature = "gles30",
        feature = "gles31",
        feature = "gles20",
    ))]
    pub glUniform4i: crate::command::PFN_glUniform4i,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glDrawElementsIndirect: crate::command::PFN_glDrawElementsIndirect,
    #[cfg(any(feature = "gles10",))]
    pub glPointParameterxv: crate::command::PFN_glPointParameterxv,
    #[cfg(any(feature = "gles10",))]
    pub glAlphaFuncx: crate::command::PFN_glAlphaFuncx,
    #[cfg(any(
        feature = "gles20",
        feature = "gles31",
        feature = "gles30",
        feature = "gles32",
    ))]
    pub glStencilMaskSeparate: crate::command::PFN_glStencilMaskSeparate,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glVertexAttribFormat: crate::command::PFN_glVertexAttribFormat,
    #[cfg(any(feature = "gles10",))]
    pub glSampleCoveragex: crate::command::PFN_glSampleCoveragex,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glReadBuffer: crate::command::PFN_glReadBuffer,
    #[cfg(any(feature = "gles32", feature = "gles31", feature = "gles30",))]
    pub glVertexAttribI4iv: crate::command::PFN_glVertexAttribI4iv,
    #[cfg(any(feature = "gles10",))]
    pub glClientActiveTexture: crate::command::PFN_glClientActiveTexture,
    #[cfg(any(
        feature = "gles20",
        feature = "gles30",
        feature = "gles31",
        feature = "gles32",
    ))]
    pub glEnableVertexAttribArray: crate::command::PFN_glEnableVertexAttribArray,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glDispatchComputeIndirect: crate::command::PFN_glDispatchComputeIndirect,
    #[cfg(any(
        feature = "gles30",
        feature = "gles32",
        feature = "gles20",
        feature = "gles31",
    ))]
    pub glIsFramebuffer: crate::command::PFN_glIsFramebuffer,
    #[cfg(any(
        feature = "gles20",
        feature = "gles31",
        feature = "gles30",
        feature = "gles32",
    ))]
    pub glGetAttachedShaders: crate::command::PFN_glGetAttachedShaders,
    #[cfg(any(feature = "gles32",))]
    pub glGetSamplerParameterIiv: crate::command::PFN_glGetSamplerParameterIiv,
    #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
    pub glDeleteSync: crate::command::PFN_glDeleteSync,
    #[cfg(any(feature = "gles32",))]
    pub glDebugMessageCallback: crate::command::PFN_glDebugMessageCallback,
    #[cfg(any(
        feature = "gles20",
        feature = "gles30",
        feature = "gles32",
        feature = "gles31",
    ))]
    pub glGetProgramInfoLog: crate::command::PFN_glGetProgramInfoLog,
    #[cfg(any(feature = "gles10",))]
    pub glVertexPointer: crate::command::PFN_glVertexPointer,
    #[cfg(any(feature = "gles10",))]
    pub glLightfv: crate::command::PFN_glLightfv,
    #[cfg(any(
        feature = "gles31",
        feature = "gles32",
        feature = "gles20",
        feature = "gles30",
    ))]
    pub glVertexAttrib4f: crate::command::PFN_glVertexAttrib4f,
    #[cfg(any(
        feature = "gles20",
        feature = "gles32",
        feature = "gles31",
        feature = "gles30",
        feature = "gles10",
    ))]
    pub glFinish: crate::command::PFN_glFinish,
    #[cfg(any(
        feature = "gles20",
        feature = "gles30",
        feature = "gles31",
        feature = "gles32",
    ))]
    pub glGenFramebuffers: crate::command::PFN_glGenFramebuffers,
    #[cfg(any(feature = "gles30", feature = "gles32", feature = "gles31",))]
    pub glTexStorage2D: crate::command::PFN_glTexStorage2D,
    #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
    pub glUniformMatrix4x2fv: crate::command::PFN_glUniformMatrix4x2fv,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glProgramUniform2i: crate::command::PFN_glProgramUniform2i,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glUseProgramStages: crate::command::PFN_glUseProgramStages,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glProgramUniform4i: crate::command::PFN_glProgramUniform4i,
    #[cfg(any(feature = "gles32",))]
    pub glGetDebugMessageLog: crate::command::PFN_glGetDebugMessageLog,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glBindSampler: crate::command::PFN_glBindSampler,
    #[cfg(any(
        feature = "gles30",
        feature = "gles32",
        feature = "gles10",
        feature = "gles31",
        feature = "gles20",
    ))]
    pub glDepthMask: crate::command::PFN_glDepthMask,
    #[cfg(any(feature = "gles10",))]
    pub glLightModelx: crate::command::PFN_glLightModelx,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glGetVertexAttribIuiv: crate::command::PFN_glGetVertexAttribIuiv,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glProgramUniform3f: crate::command::PFN_glProgramUniform3f,
    #[cfg(any(feature = "gles10",))]
    pub glPolygonOffsetx: crate::command::PFN_glPolygonOffsetx,
    #[cfg(any(feature = "gles10",))]
    pub glEnableClientState: crate::command::PFN_glEnableClientState,
    #[cfg(any(
        feature = "gles32",
        feature = "gles31",
        feature = "gles20",
        feature = "gles30",
    ))]
    pub glUniform1fv: crate::command::PFN_glUniform1fv,
    #[cfg(any(
        feature = "gles20",
        feature = "gles30",
        feature = "gles32",
        feature = "gles10",
        feature = "gles31",
    ))]
    pub glBindTexture: crate::command::PFN_glBindTexture,
    #[cfg(any(
        feature = "gles32",
        feature = "gles31",
        feature = "gles20",
        feature = "gles30",
    ))]
    pub glGetShaderSource: crate::command::PFN_glGetShaderSource,
    #[cfg(any(
        feature = "gles10",
        feature = "gles32",
        feature = "gles20",
        feature = "gles31",
        feature = "gles30",
    ))]
    pub glActiveTexture: crate::command::PFN_glActiveTexture,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glGetSynciv: crate::command::PFN_glGetSynciv,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glGetMultisamplefv: crate::command::PFN_glGetMultisamplefv,
    #[cfg(any(feature = "gles10",))]
    pub glTexParameterxv: crate::command::PFN_glTexParameterxv,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glDeleteProgramPipelines: crate::command::PFN_glDeleteProgramPipelines,
    #[cfg(any(feature = "gles32",))]
    pub glEnablei: crate::command::PFN_glEnablei,
    #[cfg(any(feature = "gles32",))]
    pub glFramebufferTexture: crate::command::PFN_glFramebufferTexture,
    #[cfg(any(feature = "gles10",))]
    pub glPushMatrix: crate::command::PFN_glPushMatrix,
    #[cfg(any(feature = "gles32",))]
    pub glGetTexParameterIiv: crate::command::PFN_glGetTexParameterIiv,
    #[cfg(any(feature = "gles32",))]
    pub glDrawRangeElementsBaseVertex: crate::command::PFN_glDrawRangeElementsBaseVertex,
    #[cfg(any(feature = "gles32",))]
    pub glDebugMessageControl: crate::command::PFN_glDebugMessageControl,
    #[cfg(any(
        feature = "gles20",
        feature = "gles31",
        feature = "gles30",
        feature = "gles32",
    ))]
    pub glUniform3iv: crate::command::PFN_glUniform3iv,
    #[cfg(any(
        feature = "gles31",
        feature = "gles30",
        feature = "gles10",
        feature = "gles20",
        feature = "gles32",
    ))]
    pub glBufferSubData: crate::command::PFN_glBufferSubData,
    #[cfg(any(feature = "gles10",))]
    pub glRotatex: crate::command::PFN_glRotatex,
    #[cfg(any(feature = "gles10",))]
    pub glPointParameterx: crate::command::PFN_glPointParameterx,
    #[cfg(any(
        feature = "gles20",
        feature = "gles32",
        feature = "gles30",
        feature = "gles31",
    ))]
    pub glLinkProgram: crate::command::PFN_glLinkProgram,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glBindBufferRange: crate::command::PFN_glBindBufferRange,
    #[cfg(any(
        feature = "gles20",
        feature = "gles30",
        feature = "gles32",
        feature = "gles10",
        feature = "gles31",
    ))]
    pub glGetTexParameterfv: crate::command::PFN_glGetTexParameterfv,
    #[cfg(any(
        feature = "gles30",
        feature = "gles20",
        feature = "gles31",
        feature = "gles32",
    ))]
    pub glGetActiveUniform: crate::command::PFN_glGetActiveUniform,
    #[cfg(any(feature = "gles10",))]
    pub glGetLightxv: crate::command::PFN_glGetLightxv,
    #[cfg(any(
        feature = "gles10",
        feature = "gles20",
        feature = "gles31",
        feature = "gles32",
        feature = "gles30",
    ))]
    pub glStencilMask: crate::command::PFN_glStencilMask,
    #[cfg(any(
        feature = "gles20",
        feature = "gles32",
        feature = "gles30",
        feature = "gles31",
    ))]
    pub glGetUniformLocation: crate::command::PFN_glGetUniformLocation,
    #[cfg(any(feature = "gles10",))]
    pub glNormalPointer: crate::command::PFN_glNormalPointer,
    #[cfg(any(feature = "gles10",))]
    pub glGetTexEnvxv: crate::command::PFN_glGetTexEnvxv,
    #[cfg(any(
        feature = "gles30",
        feature = "gles20",
        feature = "gles10",
        feature = "gles32",
        feature = "gles31",
    ))]
    pub glBlendFunc: crate::command::PFN_glBlendFunc,
    #[cfg(any(feature = "gles31", feature = "gles30", feature = "gles32",))]
    pub glUniform2uiv: crate::command::PFN_glUniform2uiv,
    #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
    pub glUniform3uiv: crate::command::PFN_glUniform3uiv,
    #[cfg(any(feature = "gles10",))]
    pub glMaterialfv: crate::command::PFN_glMaterialfv,
    #[cfg(any(feature = "gles32", feature = "gles31", feature = "gles30",))]
    pub glUnmapBuffer: crate::command::PFN_glUnmapBuffer,
    #[cfg(any(feature = "gles31", feature = "gles30", feature = "gles32",))]
    pub glEndQuery: crate::command::PFN_glEndQuery,
    #[cfg(any(
        feature = "gles32",
        feature = "gles31",
        feature = "gles30",
        feature = "gles20",
    ))]
    pub glGetUniformiv: crate::command::PFN_glGetUniformiv,
    #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
    pub glGetSamplerParameterfv: crate::command::PFN_glGetSamplerParameterfv,
    #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
    pub glUniformMatrix2x3fv: crate::command::PFN_glUniformMatrix2x3fv,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glProgramUniformMatrix4x2fv: crate::command::PFN_glProgramUniformMatrix4x2fv,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glValidateProgramPipeline: crate::command::PFN_glValidateProgramPipeline,
    #[cfg(any(feature = "gles10",))]
    pub glRotatef: crate::command::PFN_glRotatef,
    #[cfg(any(feature = "gles10",))]
    pub glColor4ub: crate::command::PFN_glColor4ub,
    #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
    pub glCompressedTexImage3D: crate::command::PFN_glCompressedTexImage3D,
    #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
    pub glEndTransformFeedback: crate::command::PFN_glEndTransformFeedback,
    #[cfg(any(
        feature = "gles30",
        feature = "gles10",
        feature = "gles20",
        feature = "gles31",
        feature = "gles32",
    ))]
    pub glCopyTexSubImage2D: crate::command::PFN_glCopyTexSubImage2D,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glGetProgramResourceIndex: crate::command::PFN_glGetProgramResourceIndex,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glIsProgramPipeline: crate::command::PFN_glIsProgramPipeline,
    #[cfg(any(feature = "gles32",))]
    pub glReadnPixels: crate::command::PFN_glReadnPixels,
    #[cfg(any(feature = "gles10",))]
    pub glPopMatrix: crate::command::PFN_glPopMatrix,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glDeleteSamplers: crate::command::PFN_glDeleteSamplers,
    #[cfg(any(feature = "gles32", feature = "gles31", feature = "gles30",))]
    pub glGetQueryObjectuiv: crate::command::PFN_glGetQueryObjectuiv,
    #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
    pub glUniform2ui: crate::command::PFN_glUniform2ui,
    #[cfg(any(feature = "gles30", feature = "gles32", feature = "gles31",))]
    pub glGetStringi: crate::command::PFN_glGetStringi,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glMemoryBarrier: crate::command::PFN_glMemoryBarrier,
    #[cfg(any(feature = "gles32",))]
    pub glColorMaski: crate::command::PFN_glColorMaski,
    #[cfg(any(feature = "gles10",))]
    pub glTexCoordPointer: crate::command::PFN_glTexCoordPointer,
    #[cfg(any(feature = "gles30", feature = "gles32", feature = "gles31",))]
    pub glGetTransformFeedbackVarying: crate::command::PFN_glGetTransformFeedbackVarying,
    #[cfg(any(feature = "gles10",))]
    pub glGetFixedv: crate::command::PFN_glGetFixedv,
    #[cfg(any(
        feature = "gles32",
        feature = "gles30",
        feature = "gles10",
        feature = "gles20",
        feature = "gles31",
    ))]
    pub glCompressedTexImage2D: crate::command::PFN_glCompressedTexImage2D,
    #[cfg(any(
        feature = "gles10",
        feature = "gles30",
        feature = "gles20",
        feature = "gles31",
        feature = "gles32",
    ))]
    pub glBindBuffer: crate::command::PFN_glBindBuffer,
    #[cfg(any(
        feature = "gles20",
        feature = "gles32",
        feature = "gles31",
        feature = "gles30",
        feature = "gles10",
    ))]
    pub glPolygonOffset: crate::command::PFN_glPolygonOffset,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glMemoryBarrierByRegion: crate::command::PFN_glMemoryBarrierByRegion,
    #[cfg(any(feature = "gles10",))]
    pub glColorPointer: crate::command::PFN_glColorPointer,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glProgramUniform1ui: crate::command::PFN_glProgramUniform1ui,
    #[cfg(any(feature = "gles32",))]
    pub glGetnUniformuiv: crate::command::PFN_glGetnUniformuiv,
    #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
    pub glTexStorage3D: crate::command::PFN_glTexStorage3D,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glProgramUniformMatrix4x3fv: crate::command::PFN_glProgramUniformMatrix4x3fv,
    #[cfg(any(feature = "gles32",))]
    pub glCopyImageSubData: crate::command::PFN_glCopyImageSubData,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glProgramUniform3iv: crate::command::PFN_glProgramUniform3iv,
    #[cfg(any(feature = "gles10", feature = "gles32",))]
    pub glGetPointerv: crate::command::PFN_glGetPointerv,
    #[cfg(any(
        feature = "gles20",
        feature = "gles32",
        feature = "gles30",
        feature = "gles10",
        feature = "gles31",
    ))]
    pub glViewport: crate::command::PFN_glViewport,
    #[cfg(any(
        feature = "gles10",
        feature = "gles30",
        feature = "gles20",
        feature = "gles31",
        feature = "gles32",
    ))]
    pub glGenTextures: crate::command::PFN_glGenTextures,
    #[cfg(any(
        feature = "gles30",
        feature = "gles31",
        feature = "gles32",
        feature = "gles20",
    ))]
    pub glGenerateMipmap: crate::command::PFN_glGenerateMipmap,
    #[cfg(any(
        feature = "gles30",
        feature = "gles31",
        feature = "gles20",
        feature = "gles32",
    ))]
    pub glCreateShader: crate::command::PFN_glCreateShader,
    #[cfg(any(feature = "gles31", feature = "gles30", feature = "gles32",))]
    pub glDeleteTransformFeedbacks: crate::command::PFN_glDeleteTransformFeedbacks,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glProgramUniformMatrix2x3fv: crate::command::PFN_glProgramUniformMatrix2x3fv,
    #[cfg(any(feature = "gles32",))]
    pub glIsEnabledi: crate::command::PFN_glIsEnabledi,
    #[cfg(any(feature = "gles10",))]
    pub glGetMaterialfv: crate::command::PFN_glGetMaterialfv,
    #[cfg(any(
        feature = "gles32",
        feature = "gles10",
        feature = "gles20",
        feature = "gles30",
        feature = "gles31",
    ))]
    pub glTexParameterfv: crate::command::PFN_glTexParameterfv,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glSampleMaski: crate::command::PFN_glSampleMaski,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glTexStorage2DMultisample: crate::command::PFN_glTexStorage2DMultisample,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glProgramUniform4iv: crate::command::PFN_glProgramUniform4iv,
    #[cfg(any(feature = "gles10",))]
    pub glMaterialf: crate::command::PFN_glMaterialf,
    #[cfg(any(
        feature = "gles20",
        feature = "gles30",
        feature = "gles32",
        feature = "gles31",
    ))]
    pub glVertexAttrib1fv: crate::command::PFN_glVertexAttrib1fv,
    #[cfg(any(
        feature = "gles20",
        feature = "gles30",
        feature = "gles32",
        feature = "gles31",
    ))]
    pub glStencilOpSeparate: crate::command::PFN_glStencilOpSeparate,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glProgramUniform1uiv: crate::command::PFN_glProgramUniform1uiv,
    #[cfg(any(feature = "gles10",))]
    pub glNormal3x: crate::command::PFN_glNormal3x,
    #[cfg(any(
        feature = "gles10",
        feature = "gles30",
        feature = "gles32",
        feature = "gles31",
        feature = "gles20",
    ))]
    pub glClearDepthf: crate::command::PFN_glClearDepthf,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glClearBufferuiv: crate::command::PFN_glClearBufferuiv,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glGetBooleani_v: crate::command::PFN_glGetBooleani_v,
    #[cfg(any(feature = "gles32",))]
    pub glGetObjectPtrLabel: crate::command::PFN_glGetObjectPtrLabel,
    #[cfg(any(feature = "gles10",))]
    pub glOrthox: crate::command::PFN_glOrthox,
    #[cfg(any(feature = "gles10",))]
    pub glLogicOp: crate::command::PFN_glLogicOp,
    #[cfg(any(
        feature = "gles32",
        feature = "gles30",
        feature = "gles20",
        feature = "gles31",
    ))]
    pub glGetAttribLocation: crate::command::PFN_glGetAttribLocation,
    #[cfg(any(
        feature = "gles20",
        feature = "gles31",
        feature = "gles30",
        feature = "gles32",
    ))]
    pub glGetProgramiv: crate::command::PFN_glGetProgramiv,
    #[cfg(any(feature = "gles30", feature = "gles32", feature = "gles31",))]
    pub glWaitSync: crate::command::PFN_glWaitSync,
    #[cfg(any(feature = "gles30", feature = "gles32", feature = "gles31",))]
    pub glInvalidateFramebuffer: crate::command::PFN_glInvalidateFramebuffer,
    #[cfg(any(feature = "gles10",))]
    pub glClearColorx: crate::command::PFN_glClearColorx,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glPauseTransformFeedback: crate::command::PFN_glPauseTransformFeedback,
    #[cfg(any(feature = "gles32",))]
    pub glGetnUniformiv: crate::command::PFN_glGetnUniformiv,
    #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
    pub glGetFragDataLocation: crate::command::PFN_glGetFragDataLocation,
    #[cfg(any(
        feature = "gles10",
        feature = "gles20",
        feature = "gles31",
        feature = "gles32",
        feature = "gles30",
    ))]
    pub glStencilFunc: crate::command::PFN_glStencilFunc,
    #[cfg(any(feature = "gles10",))]
    pub glShadeModel: crate::command::PFN_glShadeModel,
    #[cfg(any(feature = "gles10",))]
    pub glFrustumx: crate::command::PFN_glFrustumx,
    #[cfg(any(
        feature = "gles32",
        feature = "gles20",
        feature = "gles30",
        feature = "gles10",
        feature = "gles31",
    ))]
    pub glDisable: crate::command::PFN_glDisable,
    #[cfg(any(
        feature = "gles20",
        feature = "gles10",
        feature = "gles31",
        feature = "gles32",
        feature = "gles30",
    ))]
    pub glSampleCoverage: crate::command::PFN_glSampleCoverage,
    #[cfg(any(feature = "gles10",))]
    pub glFogx: crate::command::PFN_glFogx,
    #[cfg(any(
        feature = "gles31",
        feature = "gles30",
        feature = "gles10",
        feature = "gles32",
        feature = "gles20",
    ))]
    pub glGetBooleanv: crate::command::PFN_glGetBooleanv,
    #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
    pub glUniform1ui: crate::command::PFN_glUniform1ui,
    #[cfg(any(feature = "gles10",))]
    pub glMatrixMode: crate::command::PFN_glMatrixMode,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glIsSync: crate::command::PFN_glIsSync,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glTexImage3D: crate::command::PFN_glTexImage3D,
    #[cfg(any(feature = "gles32", feature = "gles31", feature = "gles30",))]
    pub glDeleteVertexArrays: crate::command::PFN_glDeleteVertexArrays,
    #[cfg(any(
        feature = "gles32",
        feature = "gles31",
        feature = "gles30",
        feature = "gles20",
    ))]
    pub glVertexAttrib4fv: crate::command::PFN_glVertexAttrib4fv,
    #[cfg(any(feature = "gles32",))]
    pub glObjectLabel: crate::command::PFN_glObjectLabel,
    #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
    pub glDeleteQueries: crate::command::PFN_glDeleteQueries,
    #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
    pub glSamplerParameteriv: crate::command::PFN_glSamplerParameteriv,
    #[cfg(any(
        feature = "gles10",
        feature = "gles32",
        feature = "gles30",
        feature = "gles20",
        feature = "gles31",
    ))]
    pub glClearColor: crate::command::PFN_glClearColor,
    #[cfg(any(feature = "gles31", feature = "gles30", feature = "gles32",))]
    pub glUniformBlockBinding: crate::command::PFN_glUniformBlockBinding,
    #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
    pub glCopyTexSubImage3D: crate::command::PFN_glCopyTexSubImage3D,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glVertexBindingDivisor: crate::command::PFN_glVertexBindingDivisor,
    #[cfg(any(feature = "gles32",))]
    pub glGetObjectLabel: crate::command::PFN_glGetObjectLabel,
    #[cfg(any(
        feature = "gles30",
        feature = "gles31",
        feature = "gles20",
        feature = "gles32",
    ))]
    pub glUniform4f: crate::command::PFN_glUniform4f,
    #[cfg(any(
        feature = "gles30",
        feature = "gles32",
        feature = "gles20",
        feature = "gles31",
    ))]
    pub glVertexAttrib2fv: crate::command::PFN_glVertexAttrib2fv,
    #[cfg(any(
        feature = "gles32",
        feature = "gles20",
        feature = "gles31",
        feature = "gles30",
    ))]
    pub glGetFramebufferAttachmentParameteriv:
        crate::command::PFN_glGetFramebufferAttachmentParameteriv,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glProgramUniform4f: crate::command::PFN_glProgramUniform4f,
    #[cfg(any(feature = "gles10",))]
    pub glGetTexEnviv: crate::command::PFN_glGetTexEnviv,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glSamplerParameterfv: crate::command::PFN_glSamplerParameterfv,
    #[cfg(any(feature = "gles32",))]
    pub glMinSampleShading: crate::command::PFN_glMinSampleShading,
    #[cfg(any(
        feature = "gles31",
        feature = "gles32",
        feature = "gles30",
        feature = "gles20",
    ))]
    pub glUniform2fv: crate::command::PFN_glUniform2fv,
    #[cfg(any(
        feature = "gles20",
        feature = "gles32",
        feature = "gles31",
        feature = "gles10",
        feature = "gles30",
    ))]
    pub glDepthRangef: crate::command::PFN_glDepthRangef,
    #[cfg(any(feature = "gles10",))]
    pub glColor4x: crate::command::PFN_glColor4x,
    #[cfg(any(feature = "gles10",))]
    pub glPointParameterf: crate::command::PFN_glPointParameterf,
    #[cfg(any(feature = "gles10",))]
    pub glTranslatef: crate::command::PFN_glTranslatef,
    #[cfg(any(
        feature = "gles31",
        feature = "gles32",
        feature = "gles10",
        feature = "gles20",
        feature = "gles30",
    ))]
    pub glIsBuffer: crate::command::PFN_glIsBuffer,
    #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
    pub glTexSubImage3D: crate::command::PFN_glTexSubImage3D,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glBindTransformFeedback: crate::command::PFN_glBindTransformFeedback,
    #[cfg(any(feature = "gles10",))]
    pub glMultiTexCoord4f: crate::command::PFN_glMultiTexCoord4f,
    #[cfg(any(feature = "gles10",))]
    pub glAlphaFunc: crate::command::PFN_glAlphaFunc,
    #[cfg(any(feature = "gles10",))]
    pub glLoadIdentity: crate::command::PFN_glLoadIdentity,
    #[cfg(any(feature = "gles10",))]
    pub glTexParameterx: crate::command::PFN_glTexParameterx,
    #[cfg(any(
        feature = "gles31",
        feature = "gles20",
        feature = "gles30",
        feature = "gles32",
    ))]
    pub glIsRenderbuffer: crate::command::PFN_glIsRenderbuffer,
    #[cfg(any(
        feature = "gles20",
        feature = "gles31",
        feature = "gles10",
        feature = "gles30",
        feature = "gles32",
    ))]
    pub glStencilOp: crate::command::PFN_glStencilOp,
    #[cfg(any(
        feature = "gles30",
        feature = "gles31",
        feature = "gles20",
        feature = "gles32",
    ))]
    pub glUniform3i: crate::command::PFN_glUniform3i,
    #[cfg(any(
        feature = "gles30",
        feature = "gles31",
        feature = "gles32",
        feature = "gles20",
    ))]
    pub glFramebufferTexture2D: crate::command::PFN_glFramebufferTexture2D,
    #[cfg(any(
        feature = "gles31",
        feature = "gles32",
        feature = "gles20",
        feature = "gles30",
    ))]
    pub glUniform3f: crate::command::PFN_glUniform3f,
    #[cfg(any(
        feature = "gles32",
        feature = "gles31",
        feature = "gles20",
        feature = "gles30",
    ))]
    pub glBindFramebuffer: crate::command::PFN_glBindFramebuffer,
    #[cfg(any(feature = "gles31", feature = "gles30", feature = "gles32",))]
    pub glSamplerParameteri: crate::command::PFN_glSamplerParameteri,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glDispatchCompute: crate::command::PFN_glDispatchCompute,
    #[cfg(any(
        feature = "gles30",
        feature = "gles20",
        feature = "gles31",
        feature = "gles32",
    ))]
    pub glVertexAttribPointer: crate::command::PFN_glVertexAttribPointer,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glGetProgramResourceLocation: crate::command::PFN_glGetProgramResourceLocation,
    #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
    pub glClientWaitSync: crate::command::PFN_glClientWaitSync,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glProgramUniformMatrix3fv: crate::command::PFN_glProgramUniformMatrix3fv,
    #[cfg(any(feature = "gles10",))]
    pub glMaterialx: crate::command::PFN_glMaterialx,
    #[cfg(any(feature = "gles10",))]
    pub glFogxv: crate::command::PFN_glFogxv,
    #[cfg(any(
        feature = "gles30",
        feature = "gles31",
        feature = "gles20",
        feature = "gles32",
    ))]
    pub glDetachShader: crate::command::PFN_glDetachShader,
    #[cfg(any(
        feature = "gles10",
        feature = "gles30",
        feature = "gles31",
        feature = "gles32",
        feature = "gles20",
    ))]
    pub glCullFace: crate::command::PFN_glCullFace,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glBindBufferBase: crate::command::PFN_glBindBufferBase,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glActiveShaderProgram: crate::command::PFN_glActiveShaderProgram,
    #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
    pub glGetActiveUniformBlockiv: crate::command::PFN_glGetActiveUniformBlockiv,
    #[cfg(any(feature = "gles10",))]
    pub glMultMatrixf: crate::command::PFN_glMultMatrixf,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glProgramUniform2f: crate::command::PFN_glProgramUniform2f,
    #[cfg(any(
        feature = "gles30",
        feature = "gles31",
        feature = "gles20",
        feature = "gles32",
        feature = "gles10",
    ))]
    pub glTexParameteri: crate::command::PFN_glTexParameteri,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glProgramUniform2uiv: crate::command::PFN_glProgramUniform2uiv,
    #[cfg(any(feature = "gles10",))]
    pub glNormal3f: crate::command::PFN_glNormal3f,
    #[cfg(any(
        feature = "gles32",
        feature = "gles30",
        feature = "gles31",
        feature = "gles20",
    ))]
    pub glUniform2iv: crate::command::PFN_glUniform2iv,
    #[cfg(any(feature = "gles10",))]
    pub glClearDepthx: crate::command::PFN_glClearDepthx,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glProgramUniform3ui: crate::command::PFN_glProgramUniform3ui,
    #[cfg(any(
        feature = "gles32",
        feature = "gles20",
        feature = "gles30",
        feature = "gles31",
    ))]
    pub glShaderBinary: crate::command::PFN_glShaderBinary,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glProgramUniformMatrix2fv: crate::command::PFN_glProgramUniformMatrix2fv,
    #[cfg(any(feature = "gles32",))]
    pub glDisablei: crate::command::PFN_glDisablei,
    #[cfg(any(
        feature = "gles32",
        feature = "gles20",
        feature = "gles30",
        feature = "gles31",
    ))]
    pub glUniform3fv: crate::command::PFN_glUniform3fv,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glUniformMatrix3x4fv: crate::command::PFN_glUniformMatrix3x4fv,
    #[cfg(any(feature = "gles10",))]
    pub glTexEnvfv: crate::command::PFN_glTexEnvfv,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glProgramUniformMatrix4fv: crate::command::PFN_glProgramUniformMatrix4fv,
    #[cfg(any(
        feature = "gles20",
        feature = "gles32",
        feature = "gles31",
        feature = "gles30",
    ))]
    pub glIsShader: crate::command::PFN_glIsShader,
    #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
    pub glSamplerParameterf: crate::command::PFN_glSamplerParameterf,
    #[cfg(any(
        feature = "gles20",
        feature = "gles32",
        feature = "gles30",
        feature = "gles31",
    ))]
    pub glGetRenderbufferParameteriv: crate::command::PFN_glGetRenderbufferParameteriv,
    #[cfg(any(
        feature = "gles30",
        feature = "gles20",
        feature = "gles10",
        feature = "gles32",
        feature = "gles31",
    ))]
    pub glDeleteTextures: crate::command::PFN_glDeleteTextures,
    #[cfg(any(
        feature = "gles30",
        feature = "gles31",
        feature = "gles32",
        feature = "gles10",
        feature = "gles20",
    ))]
    pub glDeleteBuffers: crate::command::PFN_glDeleteBuffers,
    #[cfg(any(feature = "gles10",))]
    pub glLightxv: crate::command::PFN_glLightxv,
    #[cfg(any(
        feature = "gles30",
        feature = "gles32",
        feature = "gles20",
        feature = "gles10",
        feature = "gles31",
    ))]
    pub glClear: crate::command::PFN_glClear,
    #[cfg(any(feature = "gles10",))]
    pub glMultiTexCoord4x: crate::command::PFN_glMultiTexCoord4x,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glGetTexLevelParameterfv: crate::command::PFN_glGetTexLevelParameterfv,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glGetTexLevelParameteriv: crate::command::PFN_glGetTexLevelParameteriv,
    #[cfg(any(feature = "gles32",))]
    pub glGetGraphicsResetStatus: crate::command::PFN_glGetGraphicsResetStatus,
    #[cfg(any(feature = "gles32",))]
    pub glDrawElementsInstancedBaseVertex: crate::command::PFN_glDrawElementsInstancedBaseVertex,
    #[cfg(any(feature = "gles31", feature = "gles30", feature = "gles32",))]
    pub glDrawElementsInstanced: crate::command::PFN_glDrawElementsInstanced,
    #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
    pub glUniformMatrix3x2fv: crate::command::PFN_glUniformMatrix3x2fv,
    #[cfg(any(
        feature = "gles32",
        feature = "gles31",
        feature = "gles20",
        feature = "gles30",
    ))]
    pub glDeleteShader: crate::command::PFN_glDeleteShader,
    #[cfg(any(
        feature = "gles31",
        feature = "gles20",
        feature = "gles32",
        feature = "gles10",
        feature = "gles30",
    ))]
    pub glGetTexParameteriv: crate::command::PFN_glGetTexParameteriv,
    #[cfg(any(feature = "gles32",))]
    pub glSamplerParameterIuiv: crate::command::PFN_glSamplerParameterIuiv,
    #[cfg(any(feature = "gles32",))]
    pub glPopDebugGroup: crate::command::PFN_glPopDebugGroup,
    #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
    pub glBeginTransformFeedback: crate::command::PFN_glBeginTransformFeedback,
    #[cfg(any(
        feature = "gles20",
        feature = "gles10",
        feature = "gles32",
        feature = "gles30",
        feature = "gles31",
    ))]
    pub glIsEnabled: crate::command::PFN_glIsEnabled,
    #[cfg(any(
        feature = "gles32",
        feature = "gles31",
        feature = "gles20",
        feature = "gles30",
    ))]
    pub glDeleteRenderbuffers: crate::command::PFN_glDeleteRenderbuffers,
    #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
    pub glGetInteger64v: crate::command::PFN_glGetInteger64v,
    #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
    pub glGetActiveUniformsiv: crate::command::PFN_glGetActiveUniformsiv,
    #[cfg(any(feature = "gles32",))]
    pub glDebugMessageInsert: crate::command::PFN_glDebugMessageInsert,
    #[cfg(any(feature = "gles10",))]
    pub glLightModelfv: crate::command::PFN_glLightModelfv,
    #[cfg(any(
        feature = "gles20",
        feature = "gles31",
        feature = "gles30",
        feature = "gles32",
    ))]
    pub glUniform2i: crate::command::PFN_glUniform2i,
    #[cfg(any(feature = "gles10",))]
    pub glPointParameterfv: crate::command::PFN_glPointParameterfv,
    #[cfg(any(
        feature = "gles30",
        feature = "gles20",
        feature = "gles31",
        feature = "gles32",
    ))]
    pub glShaderSource: crate::command::PFN_glShaderSource,
    #[cfg(any(feature = "gles32",))]
    pub glBlendEquationSeparatei: crate::command::PFN_glBlendEquationSeparatei,
    #[cfg(any(
        feature = "gles32",
        feature = "gles20",
        feature = "gles30",
        feature = "gles31",
    ))]
    pub glAttachShader: crate::command::PFN_glAttachShader,
    #[cfg(any(
        feature = "gles31",
        feature = "gles32",
        feature = "gles30",
        feature = "gles20",
    ))]
    pub glGetUniformfv: crate::command::PFN_glGetUniformfv,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glFramebufferParameteri: crate::command::PFN_glFramebufferParameteri,
    #[cfg(any(feature = "gles10",))]
    pub glOrthof: crate::command::PFN_glOrthof,
    #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
    pub glClearBufferiv: crate::command::PFN_glClearBufferiv,
    #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
    pub glIsVertexArray: crate::command::PFN_glIsVertexArray,
    #[cfg(any(feature = "gles30", feature = "gles32", feature = "gles31",))]
    pub glGetVertexAttribIiv: crate::command::PFN_glGetVertexAttribIiv,
    #[cfg(any(feature = "gles32", feature = "gles31", feature = "gles30",))]
    pub glGetUniformBlockIndex: crate::command::PFN_glGetUniformBlockIndex,
    #[cfg(any(
        feature = "gles10",
        feature = "gles20",
        feature = "gles30",
        feature = "gles31",
        feature = "gles32",
    ))]
    pub glPixelStorei: crate::command::PFN_glPixelStorei,
    #[cfg(any(
        feature = "gles32",
        feature = "gles30",
        feature = "gles20",
        feature = "gles31",
    ))]
    pub glGetVertexAttribfv: crate::command::PFN_glGetVertexAttribfv,
    #[cfg(any(
        feature = "gles32",
        feature = "gles20",
        feature = "gles31",
        feature = "gles30",
    ))]
    pub glDeleteFramebuffers: crate::command::PFN_glDeleteFramebuffers,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glIsQuery: crate::command::PFN_glIsQuery,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glProgramUniform3uiv: crate::command::PFN_glProgramUniform3uiv,
    #[cfg(any(
        feature = "gles30",
        feature = "gles20",
        feature = "gles31",
        feature = "gles32",
        feature = "gles10",
    ))]
    pub glGetFloatv: crate::command::PFN_glGetFloatv,
    #[cfg(any(
        feature = "gles10",
        feature = "gles32",
        feature = "gles30",
        feature = "gles31",
        feature = "gles20",
    ))]
    pub glDrawElements: crate::command::PFN_glDrawElements,
    #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
    pub glGenVertexArrays: crate::command::PFN_glGenVertexArrays,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glGetQueryiv: crate::command::PFN_glGetQueryiv,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glGetProgramResourceName: crate::command::PFN_glGetProgramResourceName,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glGetProgramInterfaceiv: crate::command::PFN_glGetProgramInterfaceiv,
    #[cfg(any(
        feature = "gles20",
        feature = "gles10",
        feature = "gles31",
        feature = "gles32",
        feature = "gles30",
    ))]
    pub glBufferData: crate::command::PFN_glBufferData,
    #[cfg(any(feature = "gles32",))]
    pub glBlendBarrier: crate::command::PFN_glBlendBarrier,
    #[cfg(any(
        feature = "gles31",
        feature = "gles32",
        feature = "gles20",
        feature = "gles30",
        feature = "gles10",
    ))]
    pub glTexParameteriv: crate::command::PFN_glTexParameteriv,
    #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
    pub glDrawRangeElements: crate::command::PFN_glDrawRangeElements,
    #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
    pub glUniform4uiv: crate::command::PFN_glUniform4uiv,
    #[cfg(any(feature = "gles30", feature = "gles32", feature = "gles31",))]
    pub glVertexAttribI4ui: crate::command::PFN_glVertexAttribI4ui,
    #[cfg(any(
        feature = "gles30",
        feature = "gles31",
        feature = "gles10",
        feature = "gles20",
        feature = "gles32",
    ))]
    pub glFrontFace: crate::command::PFN_glFrontFace,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glMapBufferRange: crate::command::PFN_glMapBufferRange,
    #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
    pub glResumeTransformFeedback: crate::command::PFN_glResumeTransformFeedback,
    #[cfg(any(feature = "gles32", feature = "gles31", feature = "gles30",))]
    pub glTransformFeedbackVaryings: crate::command::PFN_glTransformFeedbackVaryings,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glGenProgramPipelines: crate::command::PFN_glGenProgramPipelines,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glProgramUniformMatrix3x2fv: crate::command::PFN_glProgramUniformMatrix3x2fv,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glDrawArraysIndirect: crate::command::PFN_glDrawArraysIndirect,
    #[cfg(any(feature = "gles10",))]
    pub glLightModelxv: crate::command::PFN_glLightModelxv,
    #[cfg(any(
        feature = "gles10",
        feature = "gles31",
        feature = "gles20",
        feature = "gles30",
        feature = "gles32",
    ))]
    pub glLineWidth: crate::command::PFN_glLineWidth,
    #[cfg(any(
        feature = "gles20",
        feature = "gles30",
        feature = "gles31",
        feature = "gles32",
    ))]
    pub glBlendColor: crate::command::PFN_glBlendColor,
    #[cfg(any(feature = "gles32", feature = "gles31", feature = "gles30",))]
    pub glGetInteger64i_v: crate::command::PFN_glGetInteger64i_v,
    #[cfg(any(feature = "gles10",))]
    pub glGetClipPlanex: crate::command::PFN_glGetClipPlanex,
    #[cfg(any(
        feature = "gles31",
        feature = "gles20",
        feature = "gles32",
        feature = "gles30",
        feature = "gles10",
    ))]
    pub glDepthFunc: crate::command::PFN_glDepthFunc,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glVertexAttribI4uiv: crate::command::PFN_glVertexAttribI4uiv,
    #[cfg(any(feature = "gles10",))]
    pub glLoadMatrixf: crate::command::PFN_glLoadMatrixf,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glUniformMatrix2x4fv: crate::command::PFN_glUniformMatrix2x4fv,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glCreateShaderProgramv: crate::command::PFN_glCreateShaderProgramv,
    #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
    pub glInvalidateSubFramebuffer: crate::command::PFN_glInvalidateSubFramebuffer,
    #[cfg(any(
        feature = "gles31",
        feature = "gles10",
        feature = "gles20",
        feature = "gles30",
        feature = "gles32",
    ))]
    pub glColorMask: crate::command::PFN_glColorMask,
    #[cfg(any(feature = "gles10",))]
    pub glGetMaterialxv: crate::command::PFN_glGetMaterialxv,
    #[cfg(any(feature = "gles10",))]
    pub glGetClipPlanef: crate::command::PFN_glGetClipPlanef,
    #[cfg(any(feature = "gles10",))]
    pub glTexEnviv: crate::command::PFN_glTexEnviv,
    #[cfg(any(
        feature = "gles20",
        feature = "gles30",
        feature = "gles10",
        feature = "gles31",
        feature = "gles32",
    ))]
    pub glGetBufferParameteriv: crate::command::PFN_glGetBufferParameteriv,
    #[cfg(any(feature = "gles10",))]
    pub glColor4f: crate::command::PFN_glColor4f,
    #[cfg(any(
        feature = "gles20",
        feature = "gles30",
        feature = "gles10",
        feature = "gles31",
        feature = "gles32",
    ))]
    pub glGetIntegerv: crate::command::PFN_glGetIntegerv,
    #[cfg(any(
        feature = "gles32",
        feature = "gles30",
        feature = "gles31",
        feature = "gles20",
    ))]
    pub glGetShaderiv: crate::command::PFN_glGetShaderiv,
    #[cfg(any(
        feature = "gles30",
        feature = "gles20",
        feature = "gles32",
        feature = "gles31",
    ))]
    pub glIsProgram: crate::command::PFN_glIsProgram,
    #[cfg(any(
        feature = "gles32",
        feature = "gles20",
        feature = "gles31",
        feature = "gles30",
    ))]
    pub glBlendEquationSeparate: crate::command::PFN_glBlendEquationSeparate,
    #[cfg(any(feature = "gles32", feature = "gles31", feature = "gles30",))]
    pub glCompressedTexSubImage3D: crate::command::PFN_glCompressedTexSubImage3D,
    #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
    pub glVertexAttribIPointer: crate::command::PFN_glVertexAttribIPointer,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glGenQueries: crate::command::PFN_glGenQueries,
    #[cfg(any(
        feature = "gles30",
        feature = "gles31",
        feature = "gles32",
        feature = "gles20",
    ))]
    pub glVertexAttrib1f: crate::command::PFN_glVertexAttrib1f,
    #[cfg(any(
        feature = "gles31",
        feature = "gles30",
        feature = "gles32",
        feature = "gles20",
        feature = "gles10",
    ))]
    pub glFlush: crate::command::PFN_glFlush,
    #[cfg(any(
        feature = "gles31",
        feature = "gles32",
        feature = "gles20",
        feature = "gles30",
    ))]
    pub glGetActiveAttrib: crate::command::PFN_glGetActiveAttrib,
    #[cfg(any(
        feature = "gles20",
        feature = "gles30",
        feature = "gles32",
        feature = "gles31",
    ))]
    pub glCreateProgram: crate::command::PFN_glCreateProgram,
    #[cfg(any(
        feature = "gles30",
        feature = "gles31",
        feature = "gles32",
        feature = "gles10",
        feature = "gles20",
    ))]
    pub glTexSubImage2D: crate::command::PFN_glTexSubImage2D,
    #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
    pub glIsSampler: crate::command::PFN_glIsSampler,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glVertexAttribBinding: crate::command::PFN_glVertexAttribBinding,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glProgramUniform3i: crate::command::PFN_glProgramUniform3i,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glUniformMatrix4x3fv: crate::command::PFN_glUniformMatrix4x3fv,
    #[cfg(any(
        feature = "gles31",
        feature = "gles32",
        feature = "gles30",
        feature = "gles20",
    ))]
    pub glUniformMatrix3fv: crate::command::PFN_glUniformMatrix3fv,
    #[cfg(any(
        feature = "gles32",
        feature = "gles20",
        feature = "gles30",
        feature = "gles10",
        feature = "gles31",
    ))]
    pub glGenBuffers: crate::command::PFN_glGenBuffers,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glBindImageTexture: crate::command::PFN_glBindImageTexture,
    #[cfg(any(feature = "gles10",))]
    pub glGetLightfv: crate::command::PFN_glGetLightfv,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glBindVertexBuffer: crate::command::PFN_glBindVertexBuffer,
    #[cfg(any(
        feature = "gles10",
        feature = "gles30",
        feature = "gles32",
        feature = "gles20",
        feature = "gles31",
    ))]
    pub glGetString: crate::command::PFN_glGetString,
    #[cfg(any(feature = "gles32", feature = "gles31", feature = "gles30",))]
    pub glFenceSync: crate::command::PFN_glFenceSync,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glGetProgramPipelineInfoLog: crate::command::PFN_glGetProgramPipelineInfoLog,
    #[cfg(any(feature = "gles32", feature = "gles31",))]
    pub glGetProgramPipelineiv: crate::command::PFN_glGetProgramPipelineiv,
    #[cfg(any(feature = "gles32",))]
    pub glSamplerParameterIiv: crate::command::PFN_glSamplerParameterIiv,
    #[cfg(any(feature = "gles32",))]
    pub glGetnUniformfv: crate::command::PFN_glGetnUniformfv,
    #[cfg(any(
        feature = "gles30",
        feature = "gles31",
        feature = "gles32",
        feature = "gles20",
    ))]
    pub glUniform2f: crate::command::PFN_glUniform2f,
    #[cfg(any(
        feature = "gles30",
        feature = "gles32",
        feature = "gles31",
        feature = "gles20",
        feature = "gles10",
    ))]
    pub glGetError: crate::command::PFN_glGetError,
    #[cfg(any(feature = "gles32",))]
    pub glPushDebugGroup: crate::command::PFN_glPushDebugGroup,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glProgramUniform2fv: crate::command::PFN_glProgramUniform2fv,
    #[cfg(any(feature = "gles32",))]
    pub glTexBuffer: crate::command::PFN_glTexBuffer,
    #[cfg(any(
        feature = "gles32",
        feature = "gles20",
        feature = "gles31",
        feature = "gles30",
    ))]
    pub glGetShaderPrecisionFormat: crate::command::PFN_glGetShaderPrecisionFormat,
    #[cfg(any(feature = "gles31", feature = "gles30", feature = "gles32",))]
    pub glUniform1uiv: crate::command::PFN_glUniform1uiv,
    #[cfg(any(
        feature = "gles10",
        feature = "gles31",
        feature = "gles32",
        feature = "gles30",
        feature = "gles20",
    ))]
    pub glTexImage2D: crate::command::PFN_glTexImage2D,
    #[cfg(any(feature = "gles10",))]
    pub glFogfv: crate::command::PFN_glFogfv,
    #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
    pub glUniform3ui: crate::command::PFN_glUniform3ui,
    #[cfg(any(feature = "gles31", feature = "gles30", feature = "gles32",))]
    pub glDrawArraysInstanced: crate::command::PFN_glDrawArraysInstanced,
    #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
    pub glGetUniformuiv: crate::command::PFN_glGetUniformuiv,
    #[cfg(any(feature = "gles32",))]
    pub glTexStorage3DMultisample: crate::command::PFN_glTexStorage3DMultisample,
    #[cfg(any(feature = "gles10",))]
    pub glScalex: crate::command::PFN_glScalex,
    #[cfg(any(
        feature = "gles32",
        feature = "gles20",
        feature = "gles30",
        feature = "gles31",
    ))]
    pub glStencilFuncSeparate: crate::command::PFN_glStencilFuncSeparate,
    #[cfg(any(
        feature = "gles20",
        feature = "gles30",
        feature = "gles32",
        feature = "gles31",
    ))]
    pub glUniform1i: crate::command::PFN_glUniform1i,
    #[cfg(any(
        feature = "gles31",
        feature = "gles30",
        feature = "gles20",
        feature = "gles32",
    ))]
    pub glBindRenderbuffer: crate::command::PFN_glBindRenderbuffer,
    #[cfg(any(feature = "gles10",))]
    pub glGetTexEnvfv: crate::command::PFN_glGetTexEnvfv,
    #[cfg(any(
        feature = "gles10",
        feature = "gles30",
        feature = "gles31",
        feature = "gles32",
        feature = "gles20",
    ))]
    pub glDrawArrays: crate::command::PFN_glDrawArrays,
    #[cfg(any(feature = "gles10",))]
    pub glDepthRangex: crate::command::PFN_glDepthRangex,
    #[cfg(any(
        feature = "gles32",
        feature = "gles20",
        feature = "gles31",
        feature = "gles30",
    ))]
    pub glUniformMatrix2fv: crate::command::PFN_glUniformMatrix2fv,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glGetUniformIndices: crate::command::PFN_glGetUniformIndices,
    #[cfg(any(feature = "gles10",))]
    pub glPointSizex: crate::command::PFN_glPointSizex,
    #[cfg(any(feature = "gles10",))]
    pub glClipPlanex: crate::command::PFN_glClipPlanex,
    #[cfg(any(
        feature = "gles32",
        feature = "gles10",
        feature = "gles20",
        feature = "gles30",
        feature = "gles31",
    ))]
    pub glReadPixels: crate::command::PFN_glReadPixels,
    #[cfg(any(
        feature = "gles32",
        feature = "gles20",
        feature = "gles10",
        feature = "gles30",
        feature = "gles31",
    ))]
    pub glEnable: crate::command::PFN_glEnable,
    #[cfg(any(
        feature = "gles32",
        feature = "gles31",
        feature = "gles20",
        feature = "gles30",
    ))]
    pub glReleaseShaderCompiler: crate::command::PFN_glReleaseShaderCompiler,
    #[cfg(any(
        feature = "gles30",
        feature = "gles31",
        feature = "gles20",
        feature = "gles32",
    ))]
    pub glGenRenderbuffers: crate::command::PFN_glGenRenderbuffers,
    #[cfg(any(
        feature = "gles31",
        feature = "gles20",
        feature = "gles30",
        feature = "gles32",
    ))]
    pub glVertexAttrib3fv: crate::command::PFN_glVertexAttrib3fv,
    #[cfg(any(
        feature = "gles30",
        feature = "gles10",
        feature = "gles32",
        feature = "gles31",
        feature = "gles20",
    ))]
    pub glTexParameterf: crate::command::PFN_glTexParameterf,
    #[cfg(any(
        feature = "gles31",
        feature = "gles32",
        feature = "gles30",
        feature = "gles20",
    ))]
    pub glBlendFuncSeparate: crate::command::PFN_glBlendFuncSeparate,
    #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
    pub glUniform4ui: crate::command::PFN_glUniform4ui,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glVertexAttribIFormat: crate::command::PFN_glVertexAttribIFormat,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glClearBufferfv: crate::command::PFN_glClearBufferfv,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glProgramUniform1i: crate::command::PFN_glProgramUniform1i,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glProgramUniform1f: crate::command::PFN_glProgramUniform1f,
    #[cfg(any(feature = "gles32",))]
    pub glBlendFuncSeparatei: crate::command::PFN_glBlendFuncSeparatei,
    #[cfg(any(feature = "gles32",))]
    pub glGetTexParameterIuiv: crate::command::PFN_glGetTexParameterIuiv,
    #[cfg(any(feature = "gles10",))]
    pub glLightx: crate::command::PFN_glLightx,
    #[cfg(any(feature = "gles32",))]
    pub glObjectPtrLabel: crate::command::PFN_glObjectPtrLabel,
    #[cfg(any(
        feature = "gles10",
        feature = "gles31",
        feature = "gles20",
        feature = "gles30",
        feature = "gles32",
    ))]
    pub glCopyTexImage2D: crate::command::PFN_glCopyTexImage2D,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glProgramUniform2iv: crate::command::PFN_glProgramUniform2iv,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glIsTransformFeedback: crate::command::PFN_glIsTransformFeedback,
    #[cfg(any(
        feature = "gles30",
        feature = "gles32",
        feature = "gles20",
        feature = "gles31",
    ))]
    pub glCheckFramebufferStatus: crate::command::PFN_glCheckFramebufferStatus,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glDrawBuffers: crate::command::PFN_glDrawBuffers,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glProgramUniform4uiv: crate::command::PFN_glProgramUniform4uiv,
    #[cfg(any(
        feature = "gles20",
        feature = "gles31",
        feature = "gles30",
        feature = "gles32",
    ))]
    pub glGetVertexAttribPointerv: crate::command::PFN_glGetVertexAttribPointerv,
    #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
    pub glGetProgramBinary: crate::command::PFN_glGetProgramBinary,
    #[cfg(any(feature = "gles31", feature = "gles32",))]
    pub glProgramUniform3fv: crate::command::PFN_glProgramUniform3fv,
    #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
    pub glGetInternalformativ: crate::command::PFN_glGetInternalformativ,
    #[cfg(any(feature = "gles32",))]
    pub glBlendFunci: crate::command::PFN_glBlendFunci,
    #[cfg(any(feature = "gles32",))]
    pub glGetSamplerParameterIuiv: crate::command::PFN_glGetSamplerParameterIuiv,
    #[cfg(any(feature = "gles32",))]
    pub glPrimitiveBoundingBox: crate::command::PFN_glPrimitiveBoundingBox,
}
impl EntryGLESFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            #[cfg(any(
                feature = "gles32",
                feature = "gles30",
                feature = "gles20",
                feature = "gles31",
            ))]
            glRenderbufferStorage: unsafe {
                unsafe extern "system" fn __glRenderbufferStorage(
                    _target: GLenum,
                    _internalformat: GLenum,
                    _width: GLsizei,
                    _height: GLsizei,
                ) {
                    panic!("Unable to load glRenderbufferStorage")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRenderbufferStorage\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRenderbufferStorage
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles31",
                feature = "gles10",
                feature = "gles30",
                feature = "gles32",
            ))]
            glHint: unsafe {
                unsafe extern "system" fn __glHint(_target: GLenum, _mode: GLenum) {
                    panic!("Unable to load glHint")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glHint\0");
                let val = _f(cname);
                if val.is_null() {
                    __glHint
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles31",
                feature = "gles30",
                feature = "gles32",
                feature = "gles20",
            ))]
            glUniform4fv: unsafe {
                unsafe extern "system" fn __glUniform4fv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load glUniform4fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform4fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform4fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glGetFramebufferParameteriv: unsafe {
                unsafe extern "system" fn __glGetFramebufferParameteriv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetFramebufferParameteriv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetFramebufferParameteriv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetFramebufferParameteriv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles31",
                feature = "gles32",
                feature = "gles10",
                feature = "gles20",
                feature = "gles30",
            ))]
            glScissor: unsafe {
                unsafe extern "system" fn __glScissor(
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                ) {
                    panic!("Unable to load glScissor")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glScissor\0");
                let val = _f(cname);
                if val.is_null() {
                    __glScissor
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles31",
                feature = "gles32",
                feature = "gles30",
                feature = "gles20",
                feature = "gles10",
            ))]
            glIsTexture: unsafe {
                unsafe extern "system" fn __glIsTexture(_texture: GLuint) -> GLboolean {
                    panic!("Unable to load glIsTexture")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIsTexture\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIsTexture
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles32",
                feature = "gles30",
                feature = "gles20",
                feature = "gles31",
            ))]
            glUniform4iv: unsafe {
                unsafe extern "system" fn __glUniform4iv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLint,
                ) {
                    panic!("Unable to load glUniform4iv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform4iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform4iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glBlitFramebuffer: unsafe {
                unsafe extern "system" fn __glBlitFramebuffer(
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
                    panic!("Unable to load glBlitFramebuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBlitFramebuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBlitFramebuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glTexEnvi: unsafe {
                unsafe extern "system" fn __glTexEnvi(
                    _target: GLenum,
                    _pname: GLenum,
                    _param: GLint,
                ) {
                    panic!("Unable to load glTexEnvi")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexEnvi\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexEnvi
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles31",
                feature = "gles30",
                feature = "gles32",
                feature = "gles20",
            ))]
            glCompileShader: unsafe {
                unsafe extern "system" fn __glCompileShader(_shader: GLuint) {
                    panic!("Unable to load glCompileShader")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCompileShader\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCompileShader
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glLightModelf: unsafe {
                unsafe extern "system" fn __glLightModelf(_pname: GLenum, _param: GLfloat) {
                    panic!("Unable to load glLightModelf")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLightModelf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLightModelf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles31",
                feature = "gles30",
                feature = "gles32",
                feature = "gles20",
            ))]
            glUseProgram: unsafe {
                unsafe extern "system" fn __glUseProgram(_program: GLuint) {
                    panic!("Unable to load glUseProgram")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUseProgram\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUseProgram
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles32",
                feature = "gles20",
                feature = "gles31",
            ))]
            glUniformMatrix4fv: unsafe {
                unsafe extern "system" fn __glUniformMatrix4fv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: GLboolean,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load glUniformMatrix4fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix4fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformMatrix4fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles31",
                feature = "gles20",
                feature = "gles32",
            ))]
            glValidateProgram: unsafe {
                unsafe extern "system" fn __glValidateProgram(_program: GLuint) {
                    panic!("Unable to load glValidateProgram")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glValidateProgram\0");
                let val = _f(cname);
                if val.is_null() {
                    __glValidateProgram
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles31",
                feature = "gles30",
                feature = "gles32",
            ))]
            glGetShaderInfoLog: unsafe {
                unsafe extern "system" fn __glGetShaderInfoLog(
                    _shader: GLuint,
                    _bufSize: GLsizei,
                    _length: *mut GLsizei,
                    _infoLog: *mut GLchar,
                ) {
                    panic!("Unable to load glGetShaderInfoLog")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetShaderInfoLog\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetShaderInfoLog
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles31",
                feature = "gles32",
                feature = "gles30",
            ))]
            glUniform1f: unsafe {
                unsafe extern "system" fn __glUniform1f(_location: GLint, _v0: GLfloat) {
                    panic!("Unable to load glUniform1f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform1f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform1f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
            glProgramParameteri: unsafe {
                unsafe extern "system" fn __glProgramParameteri(
                    _program: GLuint,
                    _pname: GLenum,
                    _value: GLint,
                ) {
                    panic!("Unable to load glProgramParameteri")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramParameteri\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramParameteri
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glProgramUniform1fv: unsafe {
                unsafe extern "system" fn __glProgramUniform1fv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load glProgramUniform1fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform1fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform1fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles31",
                feature = "gles30",
                feature = "gles32",
                feature = "gles20",
            ))]
            glDeleteProgram: unsafe {
                unsafe extern "system" fn __glDeleteProgram(_program: GLuint) {
                    panic!("Unable to load glDeleteProgram")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDeleteProgram\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDeleteProgram
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glProgramUniform2ui: unsafe {
                unsafe extern "system" fn __glProgramUniform2ui(
                    _program: GLuint,
                    _location: GLint,
                    _v0: GLuint,
                    _v1: GLuint,
                ) {
                    panic!("Unable to load glProgramUniform2ui")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform2ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform2ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glProgramUniform4fv: unsafe {
                unsafe extern "system" fn __glProgramUniform4fv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load glProgramUniform4fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform4fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform4fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glProgramUniform1iv: unsafe {
                unsafe extern "system" fn __glProgramUniform1iv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLint,
                ) {
                    panic!("Unable to load glProgramUniform1iv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform1iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform1iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glMultMatrixx: unsafe {
                unsafe extern "system" fn __glMultMatrixx(_m: *const GLfixed) {
                    panic!("Unable to load glMultMatrixx")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultMatrixx\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultMatrixx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glFogf: unsafe {
                unsafe extern "system" fn __glFogf(_pname: GLenum, _param: GLfloat) {
                    panic!("Unable to load glFogf")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFogf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFogf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glLineWidthx: unsafe {
                unsafe extern "system" fn __glLineWidthx(_width: GLfixed) {
                    panic!("Unable to load glLineWidthx")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLineWidthx\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLineWidthx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glTexEnvx: unsafe {
                unsafe extern "system" fn __glTexEnvx(
                    _target: GLenum,
                    _pname: GLenum,
                    _param: GLfixed,
                ) {
                    panic!("Unable to load glTexEnvx")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexEnvx\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexEnvx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles32", feature = "gles31",))]
            glGetBufferPointerv: unsafe {
                unsafe extern "system" fn __glGetBufferPointerv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *mut *mut std::os::raw::c_void,
                ) {
                    panic!("Unable to load glGetBufferPointerv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetBufferPointerv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetBufferPointerv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31", feature = "gles30",))]
            glClearBufferfi: unsafe {
                unsafe extern "system" fn __glClearBufferfi(
                    _buffer: GLenum,
                    _drawbuffer: GLint,
                    _depth: GLfloat,
                    _stencil: GLint,
                ) {
                    panic!("Unable to load glClearBufferfi")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClearBufferfi\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClearBufferfi
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles32",
                feature = "gles31",
                feature = "gles20",
            ))]
            glVertexAttrib2f: unsafe {
                unsafe extern "system" fn __glVertexAttrib2f(
                    _index: GLuint,
                    _x: GLfloat,
                    _y: GLfloat,
                ) {
                    panic!("Unable to load glVertexAttrib2f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib2f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib2f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles31",
                feature = "gles30",
                feature = "gles32",
            ))]
            glUniform1iv: unsafe {
                unsafe extern "system" fn __glUniform1iv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLint,
                ) {
                    panic!("Unable to load glUniform1iv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform1iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform1iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glProgramUniformMatrix3x4fv: unsafe {
                unsafe extern "system" fn __glProgramUniformMatrix3x4fv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: GLboolean,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load glProgramUniformMatrix3x4fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glProgramUniformMatrix3x4fv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniformMatrix3x4fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glMaterialxv: unsafe {
                unsafe extern "system" fn __glMaterialxv(
                    _face: GLenum,
                    _pname: GLenum,
                    _param: *const GLfixed,
                ) {
                    panic!("Unable to load glMaterialxv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMaterialxv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMaterialxv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles10",
                feature = "gles20",
                feature = "gles30",
                feature = "gles31",
                feature = "gles32",
            ))]
            glCompressedTexSubImage2D: unsafe {
                unsafe extern "system" fn __glCompressedTexSubImage2D(
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
                    panic!("Unable to load glCompressedTexSubImage2D")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCompressedTexSubImage2D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCompressedTexSubImage2D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31", feature = "gles30",))]
            glCopyBufferSubData: unsafe {
                unsafe extern "system" fn __glCopyBufferSubData(
                    _readTarget: GLenum,
                    _writeTarget: GLenum,
                    _readOffset: GLintptr,
                    _writeOffset: GLintptr,
                    _size: GLsizeiptr,
                ) {
                    panic!("Unable to load glCopyBufferSubData")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCopyBufferSubData\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCopyBufferSubData
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
            glGetSamplerParameteriv: unsafe {
                unsafe extern "system" fn __glGetSamplerParameteriv(
                    _sampler: GLuint,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetSamplerParameteriv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetSamplerParameteriv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetSamplerParameteriv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles30", feature = "gles32",))]
            glVertexAttribI4i: unsafe {
                unsafe extern "system" fn __glVertexAttribI4i(
                    _index: GLuint,
                    _x: GLint,
                    _y: GLint,
                    _z: GLint,
                    _w: GLint,
                ) {
                    panic!("Unable to load glVertexAttribI4i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI4i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribI4i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glLoadMatrixx: unsafe {
                unsafe extern "system" fn __glLoadMatrixx(_m: *const GLfixed) {
                    panic!("Unable to load glLoadMatrixx")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLoadMatrixx\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLoadMatrixx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glTexParameterIuiv: unsafe {
                unsafe extern "system" fn __glTexParameterIuiv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *const GLuint,
                ) {
                    panic!("Unable to load glTexParameterIuiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexParameterIuiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexParameterIuiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glDrawElementsBaseVertex: unsafe {
                unsafe extern "system" fn __glDrawElementsBaseVertex(
                    _mode: GLenum,
                    _count: GLsizei,
                    _type: GLenum,
                    _indices: *const std::os::raw::c_void,
                    _basevertex: GLint,
                ) {
                    panic!("Unable to load glDrawElementsBaseVertex")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDrawElementsBaseVertex\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDrawElementsBaseVertex
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles31",
                feature = "gles30",
                feature = "gles32",
                feature = "gles20",
            ))]
            glFramebufferRenderbuffer: unsafe {
                unsafe extern "system" fn __glFramebufferRenderbuffer(
                    _target: GLenum,
                    _attachment: GLenum,
                    _renderbuffertarget: GLenum,
                    _renderbuffer: GLuint,
                ) {
                    panic!("Unable to load glFramebufferRenderbuffer")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFramebufferRenderbuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFramebufferRenderbuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glBindProgramPipeline: unsafe {
                unsafe extern "system" fn __glBindProgramPipeline(_pipeline: GLuint) {
                    panic!("Unable to load glBindProgramPipeline")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindProgramPipeline\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindProgramPipeline
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glProgramUniform4ui: unsafe {
                unsafe extern "system" fn __glProgramUniform4ui(
                    _program: GLuint,
                    _location: GLint,
                    _v0: GLuint,
                    _v1: GLuint,
                    _v2: GLuint,
                    _v3: GLuint,
                ) {
                    panic!("Unable to load glProgramUniform4ui")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform4ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform4ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glPointSize: unsafe {
                unsafe extern "system" fn __glPointSize(_size: GLfloat) {
                    panic!("Unable to load glPointSize")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPointSize\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPointSize
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glBeginQuery: unsafe {
                unsafe extern "system" fn __glBeginQuery(_target: GLenum, _id: GLuint) {
                    panic!("Unable to load glBeginQuery")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBeginQuery\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBeginQuery
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glScalef: unsafe {
                unsafe extern "system" fn __glScalef(_x: GLfloat, _y: GLfloat, _z: GLfloat) {
                    panic!("Unable to load glScalef")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glScalef\0");
                let val = _f(cname);
                if val.is_null() {
                    __glScalef
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles20",
                feature = "gles10",
                feature = "gles31",
                feature = "gles32",
            ))]
            glClearStencil: unsafe {
                unsafe extern "system" fn __glClearStencil(_s: GLint) {
                    panic!("Unable to load glClearStencil")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClearStencil\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClearStencil
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glGetProgramResourceiv: unsafe {
                unsafe extern "system" fn __glGetProgramResourceiv(
                    _program: GLuint,
                    _programInterface: GLenum,
                    _index: GLuint,
                    _propCount: GLsizei,
                    _props: *const GLenum,
                    _count: GLsizei,
                    _length: *mut GLsizei,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetProgramResourceiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetProgramResourceiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetProgramResourceiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glPatchParameteri: unsafe {
                unsafe extern "system" fn __glPatchParameteri(_pname: GLenum, _value: GLint) {
                    panic!("Unable to load glPatchParameteri")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPatchParameteri\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPatchParameteri
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glTexParameterIiv: unsafe {
                unsafe extern "system" fn __glTexParameterIiv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *const GLint,
                ) {
                    panic!("Unable to load glTexParameterIiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexParameterIiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexParameterIiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glBlendEquationi: unsafe {
                unsafe extern "system" fn __glBlendEquationi(_buf: GLuint, _mode: GLenum) {
                    panic!("Unable to load glBlendEquationi")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBlendEquationi\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBlendEquationi
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles20",
                feature = "gles31",
                feature = "gles32",
            ))]
            glBindAttribLocation: unsafe {
                unsafe extern "system" fn __glBindAttribLocation(
                    _program: GLuint,
                    _index: GLuint,
                    _name: *const GLchar,
                ) {
                    panic!("Unable to load glBindAttribLocation")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindAttribLocation\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindAttribLocation
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glTranslatex: unsafe {
                unsafe extern "system" fn __glTranslatex(_x: GLfixed, _y: GLfixed, _z: GLfixed) {
                    panic!("Unable to load glTranslatex")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTranslatex\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTranslatex
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
            glGenSamplers: unsafe {
                unsafe extern "system" fn __glGenSamplers(_count: GLsizei, _samplers: *mut GLuint) {
                    panic!("Unable to load glGenSamplers")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGenSamplers\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGenSamplers
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles31",
                feature = "gles30",
                feature = "gles32",
            ))]
            glVertexAttrib3f: unsafe {
                unsafe extern "system" fn __glVertexAttrib3f(
                    _index: GLuint,
                    _x: GLfloat,
                    _y: GLfloat,
                    _z: GLfloat,
                ) {
                    panic!("Unable to load glVertexAttrib3f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib3f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib3f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles31",
                feature = "gles32",
                feature = "gles30",
            ))]
            glGetVertexAttribiv: unsafe {
                unsafe extern "system" fn __glGetVertexAttribiv(
                    _index: GLuint,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetVertexAttribiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetVertexAttribiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetVertexAttribiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles32",
                feature = "gles31",
                feature = "gles30",
            ))]
            glDisableVertexAttribArray: unsafe {
                unsafe extern "system" fn __glDisableVertexAttribArray(_index: GLuint) {
                    panic!("Unable to load glDisableVertexAttribArray")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glDisableVertexAttribArray\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glDisableVertexAttribArray
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glGetBufferParameteri64v: unsafe {
                unsafe extern "system" fn __glGetBufferParameteri64v(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *mut GLint64,
                ) {
                    panic!("Unable to load glGetBufferParameteri64v")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetBufferParameteri64v\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetBufferParameteri64v
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glFramebufferTextureLayer: unsafe {
                unsafe extern "system" fn __glFramebufferTextureLayer(
                    _target: GLenum,
                    _attachment: GLenum,
                    _texture: GLuint,
                    _level: GLint,
                    _layer: GLint,
                ) {
                    panic!("Unable to load glFramebufferTextureLayer")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFramebufferTextureLayer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFramebufferTextureLayer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
            glProgramBinary: unsafe {
                unsafe extern "system" fn __glProgramBinary(
                    _program: GLuint,
                    _binaryFormat: GLenum,
                    _binary: *const std::os::raw::c_void,
                    _length: GLsizei,
                ) {
                    panic!("Unable to load glProgramBinary")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramBinary\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramBinary
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
            glRenderbufferStorageMultisample: unsafe {
                unsafe extern "system" fn __glRenderbufferStorageMultisample(
                    _target: GLenum,
                    _samples: GLsizei,
                    _internalformat: GLenum,
                    _width: GLsizei,
                    _height: GLsizei,
                ) {
                    panic!("Unable to load glRenderbufferStorageMultisample")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glRenderbufferStorageMultisample\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glRenderbufferStorageMultisample
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glDisableClientState: unsafe {
                unsafe extern "system" fn __glDisableClientState(_array: GLenum) {
                    panic!("Unable to load glDisableClientState")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDisableClientState\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDisableClientState
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles32", feature = "gles31",))]
            glGetIntegeri_v: unsafe {
                unsafe extern "system" fn __glGetIntegeri_v(
                    _target: GLenum,
                    _index: GLuint,
                    _data: *mut GLint,
                ) {
                    panic!("Unable to load glGetIntegeri_v")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetIntegeri_v\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetIntegeri_v
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glProgramUniformMatrix2x4fv: unsafe {
                unsafe extern "system" fn __glProgramUniformMatrix2x4fv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: GLboolean,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load glProgramUniformMatrix2x4fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glProgramUniformMatrix2x4fv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniformMatrix2x4fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glClipPlanef: unsafe {
                unsafe extern "system" fn __glClipPlanef(_p: GLenum, _eqn: *const GLfloat) {
                    panic!("Unable to load glClipPlanef")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClipPlanef\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClipPlanef
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glTexEnvf: unsafe {
                unsafe extern "system" fn __glTexEnvf(
                    _target: GLenum,
                    _pname: GLenum,
                    _param: GLfloat,
                ) {
                    panic!("Unable to load glTexEnvf")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexEnvf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexEnvf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glTexEnvxv: unsafe {
                unsafe extern "system" fn __glTexEnvxv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *const GLfixed,
                ) {
                    panic!("Unable to load glTexEnvxv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexEnvxv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexEnvxv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles31",
                feature = "gles30",
                feature = "gles32",
                feature = "gles20",
            ))]
            glBlendEquation: unsafe {
                unsafe extern "system" fn __glBlendEquation(_mode: GLenum) {
                    panic!("Unable to load glBlendEquation")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBlendEquation\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBlendEquation
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31", feature = "gles30",))]
            glGetActiveUniformBlockName: unsafe {
                unsafe extern "system" fn __glGetActiveUniformBlockName(
                    _program: GLuint,
                    _uniformBlockIndex: GLuint,
                    _bufSize: GLsizei,
                    _length: *mut GLsizei,
                    _uniformBlockName: *mut GLchar,
                ) {
                    panic!("Unable to load glGetActiveUniformBlockName")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetActiveUniformBlockName\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetActiveUniformBlockName
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glGetTexParameterxv: unsafe {
                unsafe extern "system" fn __glGetTexParameterxv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *mut GLfixed,
                ) {
                    panic!("Unable to load glGetTexParameterxv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTexParameterxv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTexParameterxv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glFlushMappedBufferRange: unsafe {
                unsafe extern "system" fn __glFlushMappedBufferRange(
                    _target: GLenum,
                    _offset: GLintptr,
                    _length: GLsizeiptr,
                ) {
                    panic!("Unable to load glFlushMappedBufferRange")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFlushMappedBufferRange\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFlushMappedBufferRange
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glFrustumf: unsafe {
                unsafe extern "system" fn __glFrustumf(
                    _l: GLfloat,
                    _r: GLfloat,
                    _b: GLfloat,
                    _t: GLfloat,
                    _n: GLfloat,
                    _f: GLfloat,
                ) {
                    panic!("Unable to load glFrustumf")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFrustumf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFrustumf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glLightf: unsafe {
                unsafe extern "system" fn __glLightf(
                    _light: GLenum,
                    _pname: GLenum,
                    _param: GLfloat,
                ) {
                    panic!("Unable to load glLightf")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLightf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLightf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glVertexAttribDivisor: unsafe {
                unsafe extern "system" fn __glVertexAttribDivisor(
                    _index: GLuint,
                    _divisor: GLuint,
                ) {
                    panic!("Unable to load glVertexAttribDivisor")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribDivisor\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribDivisor
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
            glBindVertexArray: unsafe {
                unsafe extern "system" fn __glBindVertexArray(_array: GLuint) {
                    panic!("Unable to load glBindVertexArray")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindVertexArray\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindVertexArray
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
            glGenTransformFeedbacks: unsafe {
                unsafe extern "system" fn __glGenTransformFeedbacks(
                    _n: GLsizei,
                    _ids: *mut GLuint,
                ) {
                    panic!("Unable to load glGenTransformFeedbacks")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGenTransformFeedbacks\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGenTransformFeedbacks
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glTexBufferRange: unsafe {
                unsafe extern "system" fn __glTexBufferRange(
                    _target: GLenum,
                    _internalformat: GLenum,
                    _buffer: GLuint,
                    _offset: GLintptr,
                    _size: GLsizeiptr,
                ) {
                    panic!("Unable to load glTexBufferRange")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexBufferRange\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexBufferRange
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles32",
                feature = "gles30",
                feature = "gles31",
                feature = "gles20",
            ))]
            glUniform4i: unsafe {
                unsafe extern "system" fn __glUniform4i(
                    _location: GLint,
                    _v0: GLint,
                    _v1: GLint,
                    _v2: GLint,
                    _v3: GLint,
                ) {
                    panic!("Unable to load glUniform4i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform4i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform4i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glDrawElementsIndirect: unsafe {
                unsafe extern "system" fn __glDrawElementsIndirect(
                    _mode: GLenum,
                    _type: GLenum,
                    _indirect: *const std::os::raw::c_void,
                ) {
                    panic!("Unable to load glDrawElementsIndirect")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDrawElementsIndirect\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDrawElementsIndirect
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glPointParameterxv: unsafe {
                unsafe extern "system" fn __glPointParameterxv(
                    _pname: GLenum,
                    _params: *const GLfixed,
                ) {
                    panic!("Unable to load glPointParameterxv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPointParameterxv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPointParameterxv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glAlphaFuncx: unsafe {
                unsafe extern "system" fn __glAlphaFuncx(_func: GLenum, _ref: GLfixed) {
                    panic!("Unable to load glAlphaFuncx")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glAlphaFuncx\0");
                let val = _f(cname);
                if val.is_null() {
                    __glAlphaFuncx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles31",
                feature = "gles30",
                feature = "gles32",
            ))]
            glStencilMaskSeparate: unsafe {
                unsafe extern "system" fn __glStencilMaskSeparate(_face: GLenum, _mask: GLuint) {
                    panic!("Unable to load glStencilMaskSeparate")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glStencilMaskSeparate\0");
                let val = _f(cname);
                if val.is_null() {
                    __glStencilMaskSeparate
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glVertexAttribFormat: unsafe {
                unsafe extern "system" fn __glVertexAttribFormat(
                    _attribindex: GLuint,
                    _size: GLint,
                    _type: GLenum,
                    _normalized: GLboolean,
                    _relativeoffset: GLuint,
                ) {
                    panic!("Unable to load glVertexAttribFormat")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribFormat\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribFormat
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glSampleCoveragex: unsafe {
                unsafe extern "system" fn __glSampleCoveragex(
                    _value: GLclampx,
                    _invert: GLboolean,
                ) {
                    panic!("Unable to load glSampleCoveragex")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSampleCoveragex\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSampleCoveragex
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glReadBuffer: unsafe {
                unsafe extern "system" fn __glReadBuffer(_src: GLenum) {
                    panic!("Unable to load glReadBuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glReadBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glReadBuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31", feature = "gles30",))]
            glVertexAttribI4iv: unsafe {
                unsafe extern "system" fn __glVertexAttribI4iv(_index: GLuint, _v: *const GLint) {
                    panic!("Unable to load glVertexAttribI4iv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI4iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribI4iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glClientActiveTexture: unsafe {
                unsafe extern "system" fn __glClientActiveTexture(_texture: GLenum) {
                    panic!("Unable to load glClientActiveTexture")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClientActiveTexture\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClientActiveTexture
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles30",
                feature = "gles31",
                feature = "gles32",
            ))]
            glEnableVertexAttribArray: unsafe {
                unsafe extern "system" fn __glEnableVertexAttribArray(_index: GLuint) {
                    panic!("Unable to load glEnableVertexAttribArray")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEnableVertexAttribArray\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEnableVertexAttribArray
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glDispatchComputeIndirect: unsafe {
                unsafe extern "system" fn __glDispatchComputeIndirect(_indirect: GLintptr) {
                    panic!("Unable to load glDispatchComputeIndirect")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDispatchComputeIndirect\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDispatchComputeIndirect
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles32",
                feature = "gles20",
                feature = "gles31",
            ))]
            glIsFramebuffer: unsafe {
                unsafe extern "system" fn __glIsFramebuffer(_framebuffer: GLuint) -> GLboolean {
                    panic!("Unable to load glIsFramebuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIsFramebuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIsFramebuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles31",
                feature = "gles30",
                feature = "gles32",
            ))]
            glGetAttachedShaders: unsafe {
                unsafe extern "system" fn __glGetAttachedShaders(
                    _program: GLuint,
                    _maxCount: GLsizei,
                    _count: *mut GLsizei,
                    _shaders: *mut GLuint,
                ) {
                    panic!("Unable to load glGetAttachedShaders")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetAttachedShaders\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetAttachedShaders
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glGetSamplerParameterIiv: unsafe {
                unsafe extern "system" fn __glGetSamplerParameterIiv(
                    _sampler: GLuint,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetSamplerParameterIiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetSamplerParameterIiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetSamplerParameterIiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
            glDeleteSync: unsafe {
                unsafe extern "system" fn __glDeleteSync(_sync: GLsync) {
                    panic!("Unable to load glDeleteSync")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDeleteSync\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDeleteSync
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glDebugMessageCallback: unsafe {
                unsafe extern "system" fn __glDebugMessageCallback(
                    _callback: GLDEBUGPROC,
                    _userParam: *const std::os::raw::c_void,
                ) {
                    panic!("Unable to load glDebugMessageCallback")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDebugMessageCallback\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDebugMessageCallback
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles30",
                feature = "gles32",
                feature = "gles31",
            ))]
            glGetProgramInfoLog: unsafe {
                unsafe extern "system" fn __glGetProgramInfoLog(
                    _program: GLuint,
                    _bufSize: GLsizei,
                    _length: *mut GLsizei,
                    _infoLog: *mut GLchar,
                ) {
                    panic!("Unable to load glGetProgramInfoLog")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetProgramInfoLog\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetProgramInfoLog
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glVertexPointer: unsafe {
                unsafe extern "system" fn __glVertexPointer(
                    _size: GLint,
                    _type: GLenum,
                    _stride: GLsizei,
                    _pointer: *const std::os::raw::c_void,
                ) {
                    panic!("Unable to load glVertexPointer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexPointer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexPointer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glLightfv: unsafe {
                unsafe extern "system" fn __glLightfv(
                    _light: GLenum,
                    _pname: GLenum,
                    _params: *const GLfloat,
                ) {
                    panic!("Unable to load glLightfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLightfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLightfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles31",
                feature = "gles32",
                feature = "gles20",
                feature = "gles30",
            ))]
            glVertexAttrib4f: unsafe {
                unsafe extern "system" fn __glVertexAttrib4f(
                    _index: GLuint,
                    _x: GLfloat,
                    _y: GLfloat,
                    _z: GLfloat,
                    _w: GLfloat,
                ) {
                    panic!("Unable to load glVertexAttrib4f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib4f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib4f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles32",
                feature = "gles31",
                feature = "gles30",
                feature = "gles10",
            ))]
            glFinish: unsafe {
                unsafe extern "system" fn __glFinish() {
                    panic!("Unable to load glFinish")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFinish\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFinish
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles30",
                feature = "gles31",
                feature = "gles32",
            ))]
            glGenFramebuffers: unsafe {
                unsafe extern "system" fn __glGenFramebuffers(
                    _n: GLsizei,
                    _framebuffers: *mut GLuint,
                ) {
                    panic!("Unable to load glGenFramebuffers")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGenFramebuffers\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGenFramebuffers
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles32", feature = "gles31",))]
            glTexStorage2D: unsafe {
                unsafe extern "system" fn __glTexStorage2D(
                    _target: GLenum,
                    _levels: GLsizei,
                    _internalformat: GLenum,
                    _width: GLsizei,
                    _height: GLsizei,
                ) {
                    panic!("Unable to load glTexStorage2D")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexStorage2D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexStorage2D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
            glUniformMatrix4x2fv: unsafe {
                unsafe extern "system" fn __glUniformMatrix4x2fv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: GLboolean,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load glUniformMatrix4x2fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix4x2fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformMatrix4x2fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glProgramUniform2i: unsafe {
                unsafe extern "system" fn __glProgramUniform2i(
                    _program: GLuint,
                    _location: GLint,
                    _v0: GLint,
                    _v1: GLint,
                ) {
                    panic!("Unable to load glProgramUniform2i")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform2i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform2i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glUseProgramStages: unsafe {
                unsafe extern "system" fn __glUseProgramStages(
                    _pipeline: GLuint,
                    _stages: GLbitfield,
                    _program: GLuint,
                ) {
                    panic!("Unable to load glUseProgramStages")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUseProgramStages\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUseProgramStages
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glProgramUniform4i: unsafe {
                unsafe extern "system" fn __glProgramUniform4i(
                    _program: GLuint,
                    _location: GLint,
                    _v0: GLint,
                    _v1: GLint,
                    _v2: GLint,
                    _v3: GLint,
                ) {
                    panic!("Unable to load glProgramUniform4i")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform4i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform4i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glGetDebugMessageLog: unsafe {
                unsafe extern "system" fn __glGetDebugMessageLog(
                    _count: GLuint,
                    _bufSize: GLsizei,
                    _sources: *mut GLenum,
                    _types: *mut GLenum,
                    _ids: *mut GLuint,
                    _severities: *mut GLenum,
                    _lengths: *mut GLsizei,
                    _messageLog: *mut GLchar,
                ) -> GLuint {
                    panic!("Unable to load glGetDebugMessageLog")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetDebugMessageLog\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetDebugMessageLog
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glBindSampler: unsafe {
                unsafe extern "system" fn __glBindSampler(_unit: GLuint, _sampler: GLuint) {
                    panic!("Unable to load glBindSampler")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindSampler\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindSampler
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles32",
                feature = "gles10",
                feature = "gles31",
                feature = "gles20",
            ))]
            glDepthMask: unsafe {
                unsafe extern "system" fn __glDepthMask(_flag: GLboolean) {
                    panic!("Unable to load glDepthMask")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDepthMask\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDepthMask
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glLightModelx: unsafe {
                unsafe extern "system" fn __glLightModelx(_pname: GLenum, _param: GLfixed) {
                    panic!("Unable to load glLightModelx")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLightModelx\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLightModelx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glGetVertexAttribIuiv: unsafe {
                unsafe extern "system" fn __glGetVertexAttribIuiv(
                    _index: GLuint,
                    _pname: GLenum,
                    _params: *mut GLuint,
                ) {
                    panic!("Unable to load glGetVertexAttribIuiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetVertexAttribIuiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetVertexAttribIuiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glProgramUniform3f: unsafe {
                unsafe extern "system" fn __glProgramUniform3f(
                    _program: GLuint,
                    _location: GLint,
                    _v0: GLfloat,
                    _v1: GLfloat,
                    _v2: GLfloat,
                ) {
                    panic!("Unable to load glProgramUniform3f")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform3f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform3f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glPolygonOffsetx: unsafe {
                unsafe extern "system" fn __glPolygonOffsetx(_factor: GLfixed, _units: GLfixed) {
                    panic!("Unable to load glPolygonOffsetx")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPolygonOffsetx\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPolygonOffsetx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glEnableClientState: unsafe {
                unsafe extern "system" fn __glEnableClientState(_array: GLenum) {
                    panic!("Unable to load glEnableClientState")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEnableClientState\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEnableClientState
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles32",
                feature = "gles31",
                feature = "gles20",
                feature = "gles30",
            ))]
            glUniform1fv: unsafe {
                unsafe extern "system" fn __glUniform1fv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load glUniform1fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform1fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform1fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles30",
                feature = "gles32",
                feature = "gles10",
                feature = "gles31",
            ))]
            glBindTexture: unsafe {
                unsafe extern "system" fn __glBindTexture(_target: GLenum, _texture: GLuint) {
                    panic!("Unable to load glBindTexture")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindTexture\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindTexture
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles32",
                feature = "gles31",
                feature = "gles20",
                feature = "gles30",
            ))]
            glGetShaderSource: unsafe {
                unsafe extern "system" fn __glGetShaderSource(
                    _shader: GLuint,
                    _bufSize: GLsizei,
                    _length: *mut GLsizei,
                    _source: *mut GLchar,
                ) {
                    panic!("Unable to load glGetShaderSource")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetShaderSource\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetShaderSource
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles10",
                feature = "gles32",
                feature = "gles20",
                feature = "gles31",
                feature = "gles30",
            ))]
            glActiveTexture: unsafe {
                unsafe extern "system" fn __glActiveTexture(_texture: GLenum) {
                    panic!("Unable to load glActiveTexture")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glActiveTexture\0");
                let val = _f(cname);
                if val.is_null() {
                    __glActiveTexture
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glGetSynciv: unsafe {
                unsafe extern "system" fn __glGetSynciv(
                    _sync: GLsync,
                    _pname: GLenum,
                    _count: GLsizei,
                    _length: *mut GLsizei,
                    _values: *mut GLint,
                ) {
                    panic!("Unable to load glGetSynciv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetSynciv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetSynciv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glGetMultisamplefv: unsafe {
                unsafe extern "system" fn __glGetMultisamplefv(
                    _pname: GLenum,
                    _index: GLuint,
                    _val: *mut GLfloat,
                ) {
                    panic!("Unable to load glGetMultisamplefv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetMultisamplefv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetMultisamplefv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glTexParameterxv: unsafe {
                unsafe extern "system" fn __glTexParameterxv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *const GLfixed,
                ) {
                    panic!("Unable to load glTexParameterxv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexParameterxv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexParameterxv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glDeleteProgramPipelines: unsafe {
                unsafe extern "system" fn __glDeleteProgramPipelines(
                    _n: GLsizei,
                    _pipelines: *const GLuint,
                ) {
                    panic!("Unable to load glDeleteProgramPipelines")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDeleteProgramPipelines\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDeleteProgramPipelines
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glEnablei: unsafe {
                unsafe extern "system" fn __glEnablei(_target: GLenum, _index: GLuint) {
                    panic!("Unable to load glEnablei")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEnablei\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEnablei
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glFramebufferTexture: unsafe {
                unsafe extern "system" fn __glFramebufferTexture(
                    _target: GLenum,
                    _attachment: GLenum,
                    _texture: GLuint,
                    _level: GLint,
                ) {
                    panic!("Unable to load glFramebufferTexture")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFramebufferTexture\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFramebufferTexture
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glPushMatrix: unsafe {
                unsafe extern "system" fn __glPushMatrix() {
                    panic!("Unable to load glPushMatrix")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPushMatrix\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPushMatrix
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glGetTexParameterIiv: unsafe {
                unsafe extern "system" fn __glGetTexParameterIiv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetTexParameterIiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTexParameterIiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTexParameterIiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glDrawRangeElementsBaseVertex: unsafe {
                unsafe extern "system" fn __glDrawRangeElementsBaseVertex(
                    _mode: GLenum,
                    _start: GLuint,
                    _end: GLuint,
                    _count: GLsizei,
                    _type: GLenum,
                    _indices: *const std::os::raw::c_void,
                    _basevertex: GLint,
                ) {
                    panic!("Unable to load glDrawRangeElementsBaseVertex")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glDrawRangeElementsBaseVertex\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glDrawRangeElementsBaseVertex
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glDebugMessageControl: unsafe {
                unsafe extern "system" fn __glDebugMessageControl(
                    _source: GLenum,
                    _type: GLenum,
                    _severity: GLenum,
                    _count: GLsizei,
                    _ids: *const GLuint,
                    _enabled: GLboolean,
                ) {
                    panic!("Unable to load glDebugMessageControl")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDebugMessageControl\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDebugMessageControl
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles31",
                feature = "gles30",
                feature = "gles32",
            ))]
            glUniform3iv: unsafe {
                unsafe extern "system" fn __glUniform3iv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLint,
                ) {
                    panic!("Unable to load glUniform3iv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform3iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform3iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles31",
                feature = "gles30",
                feature = "gles10",
                feature = "gles20",
                feature = "gles32",
            ))]
            glBufferSubData: unsafe {
                unsafe extern "system" fn __glBufferSubData(
                    _target: GLenum,
                    _offset: GLintptr,
                    _size: GLsizeiptr,
                    _data: *const std::os::raw::c_void,
                ) {
                    panic!("Unable to load glBufferSubData")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBufferSubData\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBufferSubData
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glRotatex: unsafe {
                unsafe extern "system" fn __glRotatex(
                    _angle: GLfixed,
                    _x: GLfixed,
                    _y: GLfixed,
                    _z: GLfixed,
                ) {
                    panic!("Unable to load glRotatex")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRotatex\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRotatex
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glPointParameterx: unsafe {
                unsafe extern "system" fn __glPointParameterx(_pname: GLenum, _param: GLfixed) {
                    panic!("Unable to load glPointParameterx")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPointParameterx\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPointParameterx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles32",
                feature = "gles30",
                feature = "gles31",
            ))]
            glLinkProgram: unsafe {
                unsafe extern "system" fn __glLinkProgram(_program: GLuint) {
                    panic!("Unable to load glLinkProgram")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLinkProgram\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLinkProgram
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glBindBufferRange: unsafe {
                unsafe extern "system" fn __glBindBufferRange(
                    _target: GLenum,
                    _index: GLuint,
                    _buffer: GLuint,
                    _offset: GLintptr,
                    _size: GLsizeiptr,
                ) {
                    panic!("Unable to load glBindBufferRange")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindBufferRange\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindBufferRange
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles30",
                feature = "gles32",
                feature = "gles10",
                feature = "gles31",
            ))]
            glGetTexParameterfv: unsafe {
                unsafe extern "system" fn __glGetTexParameterfv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load glGetTexParameterfv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTexParameterfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTexParameterfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles20",
                feature = "gles31",
                feature = "gles32",
            ))]
            glGetActiveUniform: unsafe {
                unsafe extern "system" fn __glGetActiveUniform(
                    _program: GLuint,
                    _index: GLuint,
                    _bufSize: GLsizei,
                    _length: *mut GLsizei,
                    _size: *mut GLint,
                    _type: *mut GLenum,
                    _name: *mut GLchar,
                ) {
                    panic!("Unable to load glGetActiveUniform")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetActiveUniform\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetActiveUniform
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glGetLightxv: unsafe {
                unsafe extern "system" fn __glGetLightxv(
                    _light: GLenum,
                    _pname: GLenum,
                    _params: *mut GLfixed,
                ) {
                    panic!("Unable to load glGetLightxv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetLightxv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetLightxv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles10",
                feature = "gles20",
                feature = "gles31",
                feature = "gles32",
                feature = "gles30",
            ))]
            glStencilMask: unsafe {
                unsafe extern "system" fn __glStencilMask(_mask: GLuint) {
                    panic!("Unable to load glStencilMask")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glStencilMask\0");
                let val = _f(cname);
                if val.is_null() {
                    __glStencilMask
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles32",
                feature = "gles30",
                feature = "gles31",
            ))]
            glGetUniformLocation: unsafe {
                unsafe extern "system" fn __glGetUniformLocation(
                    _program: GLuint,
                    _name: *const GLchar,
                ) -> GLint {
                    panic!("Unable to load glGetUniformLocation")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetUniformLocation\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetUniformLocation
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glNormalPointer: unsafe {
                unsafe extern "system" fn __glNormalPointer(
                    _type: GLenum,
                    _stride: GLsizei,
                    _pointer: *const std::os::raw::c_void,
                ) {
                    panic!("Unable to load glNormalPointer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glNormalPointer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glNormalPointer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glGetTexEnvxv: unsafe {
                unsafe extern "system" fn __glGetTexEnvxv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *mut GLfixed,
                ) {
                    panic!("Unable to load glGetTexEnvxv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTexEnvxv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTexEnvxv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles20",
                feature = "gles10",
                feature = "gles32",
                feature = "gles31",
            ))]
            glBlendFunc: unsafe {
                unsafe extern "system" fn __glBlendFunc(_sfactor: GLenum, _dfactor: GLenum) {
                    panic!("Unable to load glBlendFunc")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBlendFunc\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBlendFunc
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles30", feature = "gles32",))]
            glUniform2uiv: unsafe {
                unsafe extern "system" fn __glUniform2uiv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLuint,
                ) {
                    panic!("Unable to load glUniform2uiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform2uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform2uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
            glUniform3uiv: unsafe {
                unsafe extern "system" fn __glUniform3uiv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLuint,
                ) {
                    panic!("Unable to load glUniform3uiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform3uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform3uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glMaterialfv: unsafe {
                unsafe extern "system" fn __glMaterialfv(
                    _face: GLenum,
                    _pname: GLenum,
                    _params: *const GLfloat,
                ) {
                    panic!("Unable to load glMaterialfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMaterialfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMaterialfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31", feature = "gles30",))]
            glUnmapBuffer: unsafe {
                unsafe extern "system" fn __glUnmapBuffer(_target: GLenum) -> GLboolean {
                    panic!("Unable to load glUnmapBuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUnmapBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUnmapBuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles30", feature = "gles32",))]
            glEndQuery: unsafe {
                unsafe extern "system" fn __glEndQuery(_target: GLenum) {
                    panic!("Unable to load glEndQuery")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEndQuery\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEndQuery
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles32",
                feature = "gles31",
                feature = "gles30",
                feature = "gles20",
            ))]
            glGetUniformiv: unsafe {
                unsafe extern "system" fn __glGetUniformiv(
                    _program: GLuint,
                    _location: GLint,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetUniformiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetUniformiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetUniformiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
            glGetSamplerParameterfv: unsafe {
                unsafe extern "system" fn __glGetSamplerParameterfv(
                    _sampler: GLuint,
                    _pname: GLenum,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load glGetSamplerParameterfv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetSamplerParameterfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetSamplerParameterfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
            glUniformMatrix2x3fv: unsafe {
                unsafe extern "system" fn __glUniformMatrix2x3fv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: GLboolean,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load glUniformMatrix2x3fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix2x3fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformMatrix2x3fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glProgramUniformMatrix4x2fv: unsafe {
                unsafe extern "system" fn __glProgramUniformMatrix4x2fv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: GLboolean,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load glProgramUniformMatrix4x2fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glProgramUniformMatrix4x2fv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniformMatrix4x2fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glValidateProgramPipeline: unsafe {
                unsafe extern "system" fn __glValidateProgramPipeline(_pipeline: GLuint) {
                    panic!("Unable to load glValidateProgramPipeline")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glValidateProgramPipeline\0");
                let val = _f(cname);
                if val.is_null() {
                    __glValidateProgramPipeline
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glRotatef: unsafe {
                unsafe extern "system" fn __glRotatef(
                    _angle: GLfloat,
                    _x: GLfloat,
                    _y: GLfloat,
                    _z: GLfloat,
                ) {
                    panic!("Unable to load glRotatef")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glRotatef\0");
                let val = _f(cname);
                if val.is_null() {
                    __glRotatef
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glColor4ub: unsafe {
                unsafe extern "system" fn __glColor4ub(
                    _red: GLubyte,
                    _green: GLubyte,
                    _blue: GLubyte,
                    _alpha: GLubyte,
                ) {
                    panic!("Unable to load glColor4ub")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor4ub\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor4ub
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
            glCompressedTexImage3D: unsafe {
                unsafe extern "system" fn __glCompressedTexImage3D(
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
                    panic!("Unable to load glCompressedTexImage3D")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCompressedTexImage3D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCompressedTexImage3D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
            glEndTransformFeedback: unsafe {
                unsafe extern "system" fn __glEndTransformFeedback() {
                    panic!("Unable to load glEndTransformFeedback")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEndTransformFeedback\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEndTransformFeedback
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles10",
                feature = "gles20",
                feature = "gles31",
                feature = "gles32",
            ))]
            glCopyTexSubImage2D: unsafe {
                unsafe extern "system" fn __glCopyTexSubImage2D(
                    _target: GLenum,
                    _level: GLint,
                    _xoffset: GLint,
                    _yoffset: GLint,
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                ) {
                    panic!("Unable to load glCopyTexSubImage2D")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCopyTexSubImage2D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCopyTexSubImage2D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glGetProgramResourceIndex: unsafe {
                unsafe extern "system" fn __glGetProgramResourceIndex(
                    _program: GLuint,
                    _programInterface: GLenum,
                    _name: *const GLchar,
                ) -> GLuint {
                    panic!("Unable to load glGetProgramResourceIndex")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetProgramResourceIndex\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetProgramResourceIndex
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glIsProgramPipeline: unsafe {
                unsafe extern "system" fn __glIsProgramPipeline(_pipeline: GLuint) -> GLboolean {
                    panic!("Unable to load glIsProgramPipeline")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIsProgramPipeline\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIsProgramPipeline
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glReadnPixels: unsafe {
                unsafe extern "system" fn __glReadnPixels(
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                    _format: GLenum,
                    _type: GLenum,
                    _bufSize: GLsizei,
                    _data: *mut std::os::raw::c_void,
                ) {
                    panic!("Unable to load glReadnPixels")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glReadnPixels\0");
                let val = _f(cname);
                if val.is_null() {
                    __glReadnPixels
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glPopMatrix: unsafe {
                unsafe extern "system" fn __glPopMatrix() {
                    panic!("Unable to load glPopMatrix")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPopMatrix\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPopMatrix
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glDeleteSamplers: unsafe {
                unsafe extern "system" fn __glDeleteSamplers(
                    _count: GLsizei,
                    _samplers: *const GLuint,
                ) {
                    panic!("Unable to load glDeleteSamplers")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDeleteSamplers\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDeleteSamplers
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31", feature = "gles30",))]
            glGetQueryObjectuiv: unsafe {
                unsafe extern "system" fn __glGetQueryObjectuiv(
                    _id: GLuint,
                    _pname: GLenum,
                    _params: *mut GLuint,
                ) {
                    panic!("Unable to load glGetQueryObjectuiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetQueryObjectuiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetQueryObjectuiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
            glUniform2ui: unsafe {
                unsafe extern "system" fn __glUniform2ui(
                    _location: GLint,
                    _v0: GLuint,
                    _v1: GLuint,
                ) {
                    panic!("Unable to load glUniform2ui")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform2ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform2ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles32", feature = "gles31",))]
            glGetStringi: unsafe {
                unsafe extern "system" fn __glGetStringi(
                    _name: GLenum,
                    _index: GLuint,
                ) -> *const GLubyte {
                    panic!("Unable to load glGetStringi")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetStringi\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetStringi
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glMemoryBarrier: unsafe {
                unsafe extern "system" fn __glMemoryBarrier(_barriers: GLbitfield) {
                    panic!("Unable to load glMemoryBarrier")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMemoryBarrier\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMemoryBarrier
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glColorMaski: unsafe {
                unsafe extern "system" fn __glColorMaski(
                    _index: GLuint,
                    _r: GLboolean,
                    _g: GLboolean,
                    _b: GLboolean,
                    _a: GLboolean,
                ) {
                    panic!("Unable to load glColorMaski")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColorMaski\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColorMaski
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glTexCoordPointer: unsafe {
                unsafe extern "system" fn __glTexCoordPointer(
                    _size: GLint,
                    _type: GLenum,
                    _stride: GLsizei,
                    _pointer: *const std::os::raw::c_void,
                ) {
                    panic!("Unable to load glTexCoordPointer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexCoordPointer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexCoordPointer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles32", feature = "gles31",))]
            glGetTransformFeedbackVarying: unsafe {
                unsafe extern "system" fn __glGetTransformFeedbackVarying(
                    _program: GLuint,
                    _index: GLuint,
                    _bufSize: GLsizei,
                    _length: *mut GLsizei,
                    _size: *mut GLsizei,
                    _type: *mut GLenum,
                    _name: *mut GLchar,
                ) {
                    panic!("Unable to load glGetTransformFeedbackVarying")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetTransformFeedbackVarying\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetTransformFeedbackVarying
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glGetFixedv: unsafe {
                unsafe extern "system" fn __glGetFixedv(_pname: GLenum, _params: *mut GLfixed) {
                    panic!("Unable to load glGetFixedv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetFixedv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetFixedv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles32",
                feature = "gles30",
                feature = "gles10",
                feature = "gles20",
                feature = "gles31",
            ))]
            glCompressedTexImage2D: unsafe {
                unsafe extern "system" fn __glCompressedTexImage2D(
                    _target: GLenum,
                    _level: GLint,
                    _internalformat: GLenum,
                    _width: GLsizei,
                    _height: GLsizei,
                    _border: GLint,
                    _imageSize: GLsizei,
                    _data: *const std::os::raw::c_void,
                ) {
                    panic!("Unable to load glCompressedTexImage2D")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCompressedTexImage2D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCompressedTexImage2D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles10",
                feature = "gles30",
                feature = "gles20",
                feature = "gles31",
                feature = "gles32",
            ))]
            glBindBuffer: unsafe {
                unsafe extern "system" fn __glBindBuffer(_target: GLenum, _buffer: GLuint) {
                    panic!("Unable to load glBindBuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindBuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles32",
                feature = "gles31",
                feature = "gles30",
                feature = "gles10",
            ))]
            glPolygonOffset: unsafe {
                unsafe extern "system" fn __glPolygonOffset(_factor: GLfloat, _units: GLfloat) {
                    panic!("Unable to load glPolygonOffset")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPolygonOffset\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPolygonOffset
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glMemoryBarrierByRegion: unsafe {
                unsafe extern "system" fn __glMemoryBarrierByRegion(_barriers: GLbitfield) {
                    panic!("Unable to load glMemoryBarrierByRegion")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMemoryBarrierByRegion\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMemoryBarrierByRegion
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glColorPointer: unsafe {
                unsafe extern "system" fn __glColorPointer(
                    _size: GLint,
                    _type: GLenum,
                    _stride: GLsizei,
                    _pointer: *const std::os::raw::c_void,
                ) {
                    panic!("Unable to load glColorPointer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColorPointer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColorPointer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glProgramUniform1ui: unsafe {
                unsafe extern "system" fn __glProgramUniform1ui(
                    _program: GLuint,
                    _location: GLint,
                    _v0: GLuint,
                ) {
                    panic!("Unable to load glProgramUniform1ui")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform1ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform1ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glGetnUniformuiv: unsafe {
                unsafe extern "system" fn __glGetnUniformuiv(
                    _program: GLuint,
                    _location: GLint,
                    _bufSize: GLsizei,
                    _params: *mut GLuint,
                ) {
                    panic!("Unable to load glGetnUniformuiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetnUniformuiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetnUniformuiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
            glTexStorage3D: unsafe {
                unsafe extern "system" fn __glTexStorage3D(
                    _target: GLenum,
                    _levels: GLsizei,
                    _internalformat: GLenum,
                    _width: GLsizei,
                    _height: GLsizei,
                    _depth: GLsizei,
                ) {
                    panic!("Unable to load glTexStorage3D")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexStorage3D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexStorage3D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glProgramUniformMatrix4x3fv: unsafe {
                unsafe extern "system" fn __glProgramUniformMatrix4x3fv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: GLboolean,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load glProgramUniformMatrix4x3fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glProgramUniformMatrix4x3fv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniformMatrix4x3fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glCopyImageSubData: unsafe {
                unsafe extern "system" fn __glCopyImageSubData(
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
                    panic!("Unable to load glCopyImageSubData")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCopyImageSubData\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCopyImageSubData
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glProgramUniform3iv: unsafe {
                unsafe extern "system" fn __glProgramUniform3iv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLint,
                ) {
                    panic!("Unable to load glProgramUniform3iv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform3iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform3iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10", feature = "gles32",))]
            glGetPointerv: unsafe {
                unsafe extern "system" fn __glGetPointerv(
                    _pname: GLenum,
                    _params: *mut *mut std::os::raw::c_void,
                ) {
                    panic!("Unable to load glGetPointerv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetPointerv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetPointerv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles32",
                feature = "gles30",
                feature = "gles10",
                feature = "gles31",
            ))]
            glViewport: unsafe {
                unsafe extern "system" fn __glViewport(
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                ) {
                    panic!("Unable to load glViewport")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glViewport\0");
                let val = _f(cname);
                if val.is_null() {
                    __glViewport
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles10",
                feature = "gles30",
                feature = "gles20",
                feature = "gles31",
                feature = "gles32",
            ))]
            glGenTextures: unsafe {
                unsafe extern "system" fn __glGenTextures(_n: GLsizei, _textures: *mut GLuint) {
                    panic!("Unable to load glGenTextures")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGenTextures\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGenTextures
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles31",
                feature = "gles32",
                feature = "gles20",
            ))]
            glGenerateMipmap: unsafe {
                unsafe extern "system" fn __glGenerateMipmap(_target: GLenum) {
                    panic!("Unable to load glGenerateMipmap")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGenerateMipmap\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGenerateMipmap
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles31",
                feature = "gles20",
                feature = "gles32",
            ))]
            glCreateShader: unsafe {
                unsafe extern "system" fn __glCreateShader(_type: GLenum) -> GLuint {
                    panic!("Unable to load glCreateShader")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCreateShader\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCreateShader
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles30", feature = "gles32",))]
            glDeleteTransformFeedbacks: unsafe {
                unsafe extern "system" fn __glDeleteTransformFeedbacks(
                    _n: GLsizei,
                    _ids: *const GLuint,
                ) {
                    panic!("Unable to load glDeleteTransformFeedbacks")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glDeleteTransformFeedbacks\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glDeleteTransformFeedbacks
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glProgramUniformMatrix2x3fv: unsafe {
                unsafe extern "system" fn __glProgramUniformMatrix2x3fv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: GLboolean,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load glProgramUniformMatrix2x3fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glProgramUniformMatrix2x3fv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniformMatrix2x3fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glIsEnabledi: unsafe {
                unsafe extern "system" fn __glIsEnabledi(
                    _target: GLenum,
                    _index: GLuint,
                ) -> GLboolean {
                    panic!("Unable to load glIsEnabledi")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIsEnabledi\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIsEnabledi
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glGetMaterialfv: unsafe {
                unsafe extern "system" fn __glGetMaterialfv(
                    _face: GLenum,
                    _pname: GLenum,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load glGetMaterialfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetMaterialfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetMaterialfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles32",
                feature = "gles10",
                feature = "gles20",
                feature = "gles30",
                feature = "gles31",
            ))]
            glTexParameterfv: unsafe {
                unsafe extern "system" fn __glTexParameterfv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *const GLfloat,
                ) {
                    panic!("Unable to load glTexParameterfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexParameterfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexParameterfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glSampleMaski: unsafe {
                unsafe extern "system" fn __glSampleMaski(_maskNumber: GLuint, _mask: GLbitfield) {
                    panic!("Unable to load glSampleMaski")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSampleMaski\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSampleMaski
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glTexStorage2DMultisample: unsafe {
                unsafe extern "system" fn __glTexStorage2DMultisample(
                    _target: GLenum,
                    _samples: GLsizei,
                    _internalformat: GLenum,
                    _width: GLsizei,
                    _height: GLsizei,
                    _fixedsamplelocations: GLboolean,
                ) {
                    panic!("Unable to load glTexStorage2DMultisample")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexStorage2DMultisample\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexStorage2DMultisample
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glProgramUniform4iv: unsafe {
                unsafe extern "system" fn __glProgramUniform4iv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLint,
                ) {
                    panic!("Unable to load glProgramUniform4iv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform4iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform4iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glMaterialf: unsafe {
                unsafe extern "system" fn __glMaterialf(
                    _face: GLenum,
                    _pname: GLenum,
                    _param: GLfloat,
                ) {
                    panic!("Unable to load glMaterialf")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMaterialf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMaterialf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles30",
                feature = "gles32",
                feature = "gles31",
            ))]
            glVertexAttrib1fv: unsafe {
                unsafe extern "system" fn __glVertexAttrib1fv(_index: GLuint, _v: *const GLfloat) {
                    panic!("Unable to load glVertexAttrib1fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib1fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib1fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles30",
                feature = "gles32",
                feature = "gles31",
            ))]
            glStencilOpSeparate: unsafe {
                unsafe extern "system" fn __glStencilOpSeparate(
                    _face: GLenum,
                    _sfail: GLenum,
                    _dpfail: GLenum,
                    _dppass: GLenum,
                ) {
                    panic!("Unable to load glStencilOpSeparate")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glStencilOpSeparate\0");
                let val = _f(cname);
                if val.is_null() {
                    __glStencilOpSeparate
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glProgramUniform1uiv: unsafe {
                unsafe extern "system" fn __glProgramUniform1uiv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLuint,
                ) {
                    panic!("Unable to load glProgramUniform1uiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform1uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform1uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glNormal3x: unsafe {
                unsafe extern "system" fn __glNormal3x(_nx: GLfixed, _ny: GLfixed, _nz: GLfixed) {
                    panic!("Unable to load glNormal3x")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glNormal3x\0");
                let val = _f(cname);
                if val.is_null() {
                    __glNormal3x
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles10",
                feature = "gles30",
                feature = "gles32",
                feature = "gles31",
                feature = "gles20",
            ))]
            glClearDepthf: unsafe {
                unsafe extern "system" fn __glClearDepthf(_d: GLfloat) {
                    panic!("Unable to load glClearDepthf")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClearDepthf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClearDepthf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glClearBufferuiv: unsafe {
                unsafe extern "system" fn __glClearBufferuiv(
                    _buffer: GLenum,
                    _drawbuffer: GLint,
                    _value: *const GLuint,
                ) {
                    panic!("Unable to load glClearBufferuiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClearBufferuiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClearBufferuiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glGetBooleani_v: unsafe {
                unsafe extern "system" fn __glGetBooleani_v(
                    _target: GLenum,
                    _index: GLuint,
                    _data: *mut GLboolean,
                ) {
                    panic!("Unable to load glGetBooleani_v")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetBooleani_v\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetBooleani_v
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glGetObjectPtrLabel: unsafe {
                unsafe extern "system" fn __glGetObjectPtrLabel(
                    _ptr: *const std::os::raw::c_void,
                    _bufSize: GLsizei,
                    _length: *mut GLsizei,
                    _label: *mut GLchar,
                ) {
                    panic!("Unable to load glGetObjectPtrLabel")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetObjectPtrLabel\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetObjectPtrLabel
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glOrthox: unsafe {
                unsafe extern "system" fn __glOrthox(
                    _l: GLfixed,
                    _r: GLfixed,
                    _b: GLfixed,
                    _t: GLfixed,
                    _n: GLfixed,
                    _f: GLfixed,
                ) {
                    panic!("Unable to load glOrthox")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glOrthox\0");
                let val = _f(cname);
                if val.is_null() {
                    __glOrthox
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glLogicOp: unsafe {
                unsafe extern "system" fn __glLogicOp(_opcode: GLenum) {
                    panic!("Unable to load glLogicOp")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLogicOp\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLogicOp
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles32",
                feature = "gles30",
                feature = "gles20",
                feature = "gles31",
            ))]
            glGetAttribLocation: unsafe {
                unsafe extern "system" fn __glGetAttribLocation(
                    _program: GLuint,
                    _name: *const GLchar,
                ) -> GLint {
                    panic!("Unable to load glGetAttribLocation")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetAttribLocation\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetAttribLocation
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles31",
                feature = "gles30",
                feature = "gles32",
            ))]
            glGetProgramiv: unsafe {
                unsafe extern "system" fn __glGetProgramiv(
                    _program: GLuint,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetProgramiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetProgramiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetProgramiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles32", feature = "gles31",))]
            glWaitSync: unsafe {
                unsafe extern "system" fn __glWaitSync(
                    _sync: GLsync,
                    _flags: GLbitfield,
                    _timeout: GLuint64,
                ) {
                    panic!("Unable to load glWaitSync")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glWaitSync\0");
                let val = _f(cname);
                if val.is_null() {
                    __glWaitSync
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles32", feature = "gles31",))]
            glInvalidateFramebuffer: unsafe {
                unsafe extern "system" fn __glInvalidateFramebuffer(
                    _target: GLenum,
                    _numAttachments: GLsizei,
                    _attachments: *const GLenum,
                ) {
                    panic!("Unable to load glInvalidateFramebuffer")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glInvalidateFramebuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glInvalidateFramebuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glClearColorx: unsafe {
                unsafe extern "system" fn __glClearColorx(
                    _red: GLfixed,
                    _green: GLfixed,
                    _blue: GLfixed,
                    _alpha: GLfixed,
                ) {
                    panic!("Unable to load glClearColorx")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClearColorx\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClearColorx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glPauseTransformFeedback: unsafe {
                unsafe extern "system" fn __glPauseTransformFeedback() {
                    panic!("Unable to load glPauseTransformFeedback")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPauseTransformFeedback\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPauseTransformFeedback
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glGetnUniformiv: unsafe {
                unsafe extern "system" fn __glGetnUniformiv(
                    _program: GLuint,
                    _location: GLint,
                    _bufSize: GLsizei,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetnUniformiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetnUniformiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetnUniformiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
            glGetFragDataLocation: unsafe {
                unsafe extern "system" fn __glGetFragDataLocation(
                    _program: GLuint,
                    _name: *const GLchar,
                ) -> GLint {
                    panic!("Unable to load glGetFragDataLocation")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetFragDataLocation\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetFragDataLocation
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles10",
                feature = "gles20",
                feature = "gles31",
                feature = "gles32",
                feature = "gles30",
            ))]
            glStencilFunc: unsafe {
                unsafe extern "system" fn __glStencilFunc(
                    _func: GLenum,
                    _ref: GLint,
                    _mask: GLuint,
                ) {
                    panic!("Unable to load glStencilFunc")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glStencilFunc\0");
                let val = _f(cname);
                if val.is_null() {
                    __glStencilFunc
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glShadeModel: unsafe {
                unsafe extern "system" fn __glShadeModel(_mode: GLenum) {
                    panic!("Unable to load glShadeModel")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glShadeModel\0");
                let val = _f(cname);
                if val.is_null() {
                    __glShadeModel
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glFrustumx: unsafe {
                unsafe extern "system" fn __glFrustumx(
                    _l: GLfixed,
                    _r: GLfixed,
                    _b: GLfixed,
                    _t: GLfixed,
                    _n: GLfixed,
                    _f: GLfixed,
                ) {
                    panic!("Unable to load glFrustumx")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFrustumx\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFrustumx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles32",
                feature = "gles20",
                feature = "gles30",
                feature = "gles10",
                feature = "gles31",
            ))]
            glDisable: unsafe {
                unsafe extern "system" fn __glDisable(_cap: GLenum) {
                    panic!("Unable to load glDisable")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDisable\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDisable
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles10",
                feature = "gles31",
                feature = "gles32",
                feature = "gles30",
            ))]
            glSampleCoverage: unsafe {
                unsafe extern "system" fn __glSampleCoverage(_value: GLfloat, _invert: GLboolean) {
                    panic!("Unable to load glSampleCoverage")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSampleCoverage\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSampleCoverage
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glFogx: unsafe {
                unsafe extern "system" fn __glFogx(_pname: GLenum, _param: GLfixed) {
                    panic!("Unable to load glFogx")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFogx\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFogx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles31",
                feature = "gles30",
                feature = "gles10",
                feature = "gles32",
                feature = "gles20",
            ))]
            glGetBooleanv: unsafe {
                unsafe extern "system" fn __glGetBooleanv(_pname: GLenum, _data: *mut GLboolean) {
                    panic!("Unable to load glGetBooleanv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetBooleanv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetBooleanv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
            glUniform1ui: unsafe {
                unsafe extern "system" fn __glUniform1ui(_location: GLint, _v0: GLuint) {
                    panic!("Unable to load glUniform1ui")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform1ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform1ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glMatrixMode: unsafe {
                unsafe extern "system" fn __glMatrixMode(_mode: GLenum) {
                    panic!("Unable to load glMatrixMode")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMatrixMode\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMatrixMode
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glIsSync: unsafe {
                unsafe extern "system" fn __glIsSync(_sync: GLsync) -> GLboolean {
                    panic!("Unable to load glIsSync")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIsSync\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIsSync
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glTexImage3D: unsafe {
                unsafe extern "system" fn __glTexImage3D(
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
                    panic!("Unable to load glTexImage3D")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexImage3D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexImage3D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31", feature = "gles30",))]
            glDeleteVertexArrays: unsafe {
                unsafe extern "system" fn __glDeleteVertexArrays(
                    _n: GLsizei,
                    _arrays: *const GLuint,
                ) {
                    panic!("Unable to load glDeleteVertexArrays")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDeleteVertexArrays\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDeleteVertexArrays
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles32",
                feature = "gles31",
                feature = "gles30",
                feature = "gles20",
            ))]
            glVertexAttrib4fv: unsafe {
                unsafe extern "system" fn __glVertexAttrib4fv(_index: GLuint, _v: *const GLfloat) {
                    panic!("Unable to load glVertexAttrib4fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib4fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib4fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glObjectLabel: unsafe {
                unsafe extern "system" fn __glObjectLabel(
                    _identifier: GLenum,
                    _name: GLuint,
                    _length: GLsizei,
                    _label: *const GLchar,
                ) {
                    panic!("Unable to load glObjectLabel")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glObjectLabel\0");
                let val = _f(cname);
                if val.is_null() {
                    __glObjectLabel
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
            glDeleteQueries: unsafe {
                unsafe extern "system" fn __glDeleteQueries(_n: GLsizei, _ids: *const GLuint) {
                    panic!("Unable to load glDeleteQueries")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDeleteQueries\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDeleteQueries
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
            glSamplerParameteriv: unsafe {
                unsafe extern "system" fn __glSamplerParameteriv(
                    _sampler: GLuint,
                    _pname: GLenum,
                    _param: *const GLint,
                ) {
                    panic!("Unable to load glSamplerParameteriv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSamplerParameteriv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSamplerParameteriv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles10",
                feature = "gles32",
                feature = "gles30",
                feature = "gles20",
                feature = "gles31",
            ))]
            glClearColor: unsafe {
                unsafe extern "system" fn __glClearColor(
                    _red: GLfloat,
                    _green: GLfloat,
                    _blue: GLfloat,
                    _alpha: GLfloat,
                ) {
                    panic!("Unable to load glClearColor")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClearColor\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClearColor
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles30", feature = "gles32",))]
            glUniformBlockBinding: unsafe {
                unsafe extern "system" fn __glUniformBlockBinding(
                    _program: GLuint,
                    _uniformBlockIndex: GLuint,
                    _uniformBlockBinding: GLuint,
                ) {
                    panic!("Unable to load glUniformBlockBinding")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformBlockBinding\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformBlockBinding
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
            glCopyTexSubImage3D: unsafe {
                unsafe extern "system" fn __glCopyTexSubImage3D(
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
                    panic!("Unable to load glCopyTexSubImage3D")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCopyTexSubImage3D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCopyTexSubImage3D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glVertexBindingDivisor: unsafe {
                unsafe extern "system" fn __glVertexBindingDivisor(
                    _bindingindex: GLuint,
                    _divisor: GLuint,
                ) {
                    panic!("Unable to load glVertexBindingDivisor")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexBindingDivisor\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexBindingDivisor
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glGetObjectLabel: unsafe {
                unsafe extern "system" fn __glGetObjectLabel(
                    _identifier: GLenum,
                    _name: GLuint,
                    _bufSize: GLsizei,
                    _length: *mut GLsizei,
                    _label: *mut GLchar,
                ) {
                    panic!("Unable to load glGetObjectLabel")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetObjectLabel\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetObjectLabel
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles31",
                feature = "gles20",
                feature = "gles32",
            ))]
            glUniform4f: unsafe {
                unsafe extern "system" fn __glUniform4f(
                    _location: GLint,
                    _v0: GLfloat,
                    _v1: GLfloat,
                    _v2: GLfloat,
                    _v3: GLfloat,
                ) {
                    panic!("Unable to load glUniform4f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform4f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform4f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles32",
                feature = "gles20",
                feature = "gles31",
            ))]
            glVertexAttrib2fv: unsafe {
                unsafe extern "system" fn __glVertexAttrib2fv(_index: GLuint, _v: *const GLfloat) {
                    panic!("Unable to load glVertexAttrib2fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib2fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib2fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles32",
                feature = "gles20",
                feature = "gles31",
                feature = "gles30",
            ))]
            glGetFramebufferAttachmentParameteriv: unsafe {
                unsafe extern "system" fn __glGetFramebufferAttachmentParameteriv(
                    _target: GLenum,
                    _attachment: GLenum,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetFramebufferAttachmentParameteriv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetFramebufferAttachmentParameteriv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetFramebufferAttachmentParameteriv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glProgramUniform4f: unsafe {
                unsafe extern "system" fn __glProgramUniform4f(
                    _program: GLuint,
                    _location: GLint,
                    _v0: GLfloat,
                    _v1: GLfloat,
                    _v2: GLfloat,
                    _v3: GLfloat,
                ) {
                    panic!("Unable to load glProgramUniform4f")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform4f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform4f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glGetTexEnviv: unsafe {
                unsafe extern "system" fn __glGetTexEnviv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetTexEnviv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTexEnviv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTexEnviv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glSamplerParameterfv: unsafe {
                unsafe extern "system" fn __glSamplerParameterfv(
                    _sampler: GLuint,
                    _pname: GLenum,
                    _param: *const GLfloat,
                ) {
                    panic!("Unable to load glSamplerParameterfv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSamplerParameterfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSamplerParameterfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glMinSampleShading: unsafe {
                unsafe extern "system" fn __glMinSampleShading(_value: GLfloat) {
                    panic!("Unable to load glMinSampleShading")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMinSampleShading\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMinSampleShading
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles31",
                feature = "gles32",
                feature = "gles30",
                feature = "gles20",
            ))]
            glUniform2fv: unsafe {
                unsafe extern "system" fn __glUniform2fv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load glUniform2fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform2fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform2fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles32",
                feature = "gles31",
                feature = "gles10",
                feature = "gles30",
            ))]
            glDepthRangef: unsafe {
                unsafe extern "system" fn __glDepthRangef(_n: GLfloat, _f: GLfloat) {
                    panic!("Unable to load glDepthRangef")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDepthRangef\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDepthRangef
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glColor4x: unsafe {
                unsafe extern "system" fn __glColor4x(
                    _red: GLfixed,
                    _green: GLfixed,
                    _blue: GLfixed,
                    _alpha: GLfixed,
                ) {
                    panic!("Unable to load glColor4x")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor4x\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor4x
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glPointParameterf: unsafe {
                unsafe extern "system" fn __glPointParameterf(_pname: GLenum, _param: GLfloat) {
                    panic!("Unable to load glPointParameterf")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPointParameterf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPointParameterf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glTranslatef: unsafe {
                unsafe extern "system" fn __glTranslatef(_x: GLfloat, _y: GLfloat, _z: GLfloat) {
                    panic!("Unable to load glTranslatef")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTranslatef\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTranslatef
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles31",
                feature = "gles32",
                feature = "gles10",
                feature = "gles20",
                feature = "gles30",
            ))]
            glIsBuffer: unsafe {
                unsafe extern "system" fn __glIsBuffer(_buffer: GLuint) -> GLboolean {
                    panic!("Unable to load glIsBuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIsBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIsBuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
            glTexSubImage3D: unsafe {
                unsafe extern "system" fn __glTexSubImage3D(
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
                    panic!("Unable to load glTexSubImage3D")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexSubImage3D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexSubImage3D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glBindTransformFeedback: unsafe {
                unsafe extern "system" fn __glBindTransformFeedback(_target: GLenum, _id: GLuint) {
                    panic!("Unable to load glBindTransformFeedback")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindTransformFeedback\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindTransformFeedback
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glMultiTexCoord4f: unsafe {
                unsafe extern "system" fn __glMultiTexCoord4f(
                    _target: GLenum,
                    _s: GLfloat,
                    _t: GLfloat,
                    _r: GLfloat,
                    _q: GLfloat,
                ) {
                    panic!("Unable to load glMultiTexCoord4f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord4f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord4f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glAlphaFunc: unsafe {
                unsafe extern "system" fn __glAlphaFunc(_func: GLenum, _ref: GLfloat) {
                    panic!("Unable to load glAlphaFunc")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glAlphaFunc\0");
                let val = _f(cname);
                if val.is_null() {
                    __glAlphaFunc
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glLoadIdentity: unsafe {
                unsafe extern "system" fn __glLoadIdentity() {
                    panic!("Unable to load glLoadIdentity")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLoadIdentity\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLoadIdentity
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glTexParameterx: unsafe {
                unsafe extern "system" fn __glTexParameterx(
                    _target: GLenum,
                    _pname: GLenum,
                    _param: GLfixed,
                ) {
                    panic!("Unable to load glTexParameterx")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexParameterx\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexParameterx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles31",
                feature = "gles20",
                feature = "gles30",
                feature = "gles32",
            ))]
            glIsRenderbuffer: unsafe {
                unsafe extern "system" fn __glIsRenderbuffer(_renderbuffer: GLuint) -> GLboolean {
                    panic!("Unable to load glIsRenderbuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIsRenderbuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIsRenderbuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles31",
                feature = "gles10",
                feature = "gles30",
                feature = "gles32",
            ))]
            glStencilOp: unsafe {
                unsafe extern "system" fn __glStencilOp(
                    _fail: GLenum,
                    _zfail: GLenum,
                    _zpass: GLenum,
                ) {
                    panic!("Unable to load glStencilOp")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glStencilOp\0");
                let val = _f(cname);
                if val.is_null() {
                    __glStencilOp
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles31",
                feature = "gles20",
                feature = "gles32",
            ))]
            glUniform3i: unsafe {
                unsafe extern "system" fn __glUniform3i(
                    _location: GLint,
                    _v0: GLint,
                    _v1: GLint,
                    _v2: GLint,
                ) {
                    panic!("Unable to load glUniform3i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform3i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform3i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles31",
                feature = "gles32",
                feature = "gles20",
            ))]
            glFramebufferTexture2D: unsafe {
                unsafe extern "system" fn __glFramebufferTexture2D(
                    _target: GLenum,
                    _attachment: GLenum,
                    _textarget: GLenum,
                    _texture: GLuint,
                    _level: GLint,
                ) {
                    panic!("Unable to load glFramebufferTexture2D")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFramebufferTexture2D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFramebufferTexture2D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles31",
                feature = "gles32",
                feature = "gles20",
                feature = "gles30",
            ))]
            glUniform3f: unsafe {
                unsafe extern "system" fn __glUniform3f(
                    _location: GLint,
                    _v0: GLfloat,
                    _v1: GLfloat,
                    _v2: GLfloat,
                ) {
                    panic!("Unable to load glUniform3f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform3f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform3f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles32",
                feature = "gles31",
                feature = "gles20",
                feature = "gles30",
            ))]
            glBindFramebuffer: unsafe {
                unsafe extern "system" fn __glBindFramebuffer(
                    _target: GLenum,
                    _framebuffer: GLuint,
                ) {
                    panic!("Unable to load glBindFramebuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindFramebuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindFramebuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles30", feature = "gles32",))]
            glSamplerParameteri: unsafe {
                unsafe extern "system" fn __glSamplerParameteri(
                    _sampler: GLuint,
                    _pname: GLenum,
                    _param: GLint,
                ) {
                    panic!("Unable to load glSamplerParameteri")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSamplerParameteri\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSamplerParameteri
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glDispatchCompute: unsafe {
                unsafe extern "system" fn __glDispatchCompute(
                    _num_groups_x: GLuint,
                    _num_groups_y: GLuint,
                    _num_groups_z: GLuint,
                ) {
                    panic!("Unable to load glDispatchCompute")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDispatchCompute\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDispatchCompute
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles20",
                feature = "gles31",
                feature = "gles32",
            ))]
            glVertexAttribPointer: unsafe {
                unsafe extern "system" fn __glVertexAttribPointer(
                    _index: GLuint,
                    _size: GLint,
                    _type: GLenum,
                    _normalized: GLboolean,
                    _stride: GLsizei,
                    _pointer: *const std::os::raw::c_void,
                ) {
                    panic!("Unable to load glVertexAttribPointer")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribPointer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribPointer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glGetProgramResourceLocation: unsafe {
                unsafe extern "system" fn __glGetProgramResourceLocation(
                    _program: GLuint,
                    _programInterface: GLenum,
                    _name: *const GLchar,
                ) -> GLint {
                    panic!("Unable to load glGetProgramResourceLocation")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetProgramResourceLocation\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetProgramResourceLocation
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
            glClientWaitSync: unsafe {
                unsafe extern "system" fn __glClientWaitSync(
                    _sync: GLsync,
                    _flags: GLbitfield,
                    _timeout: GLuint64,
                ) -> GLenum {
                    panic!("Unable to load glClientWaitSync")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClientWaitSync\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClientWaitSync
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glProgramUniformMatrix3fv: unsafe {
                unsafe extern "system" fn __glProgramUniformMatrix3fv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: GLboolean,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load glProgramUniformMatrix3fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniformMatrix3fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniformMatrix3fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glMaterialx: unsafe {
                unsafe extern "system" fn __glMaterialx(
                    _face: GLenum,
                    _pname: GLenum,
                    _param: GLfixed,
                ) {
                    panic!("Unable to load glMaterialx")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMaterialx\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMaterialx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glFogxv: unsafe {
                unsafe extern "system" fn __glFogxv(_pname: GLenum, _param: *const GLfixed) {
                    panic!("Unable to load glFogxv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFogxv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFogxv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles31",
                feature = "gles20",
                feature = "gles32",
            ))]
            glDetachShader: unsafe {
                unsafe extern "system" fn __glDetachShader(_program: GLuint, _shader: GLuint) {
                    panic!("Unable to load glDetachShader")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDetachShader\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDetachShader
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles10",
                feature = "gles30",
                feature = "gles31",
                feature = "gles32",
                feature = "gles20",
            ))]
            glCullFace: unsafe {
                unsafe extern "system" fn __glCullFace(_mode: GLenum) {
                    panic!("Unable to load glCullFace")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCullFace\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCullFace
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glBindBufferBase: unsafe {
                unsafe extern "system" fn __glBindBufferBase(
                    _target: GLenum,
                    _index: GLuint,
                    _buffer: GLuint,
                ) {
                    panic!("Unable to load glBindBufferBase")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindBufferBase\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindBufferBase
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glActiveShaderProgram: unsafe {
                unsafe extern "system" fn __glActiveShaderProgram(
                    _pipeline: GLuint,
                    _program: GLuint,
                ) {
                    panic!("Unable to load glActiveShaderProgram")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glActiveShaderProgram\0");
                let val = _f(cname);
                if val.is_null() {
                    __glActiveShaderProgram
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
            glGetActiveUniformBlockiv: unsafe {
                unsafe extern "system" fn __glGetActiveUniformBlockiv(
                    _program: GLuint,
                    _uniformBlockIndex: GLuint,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetActiveUniformBlockiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetActiveUniformBlockiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetActiveUniformBlockiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glMultMatrixf: unsafe {
                unsafe extern "system" fn __glMultMatrixf(_m: *const GLfloat) {
                    panic!("Unable to load glMultMatrixf")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultMatrixf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultMatrixf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glProgramUniform2f: unsafe {
                unsafe extern "system" fn __glProgramUniform2f(
                    _program: GLuint,
                    _location: GLint,
                    _v0: GLfloat,
                    _v1: GLfloat,
                ) {
                    panic!("Unable to load glProgramUniform2f")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform2f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform2f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles31",
                feature = "gles20",
                feature = "gles32",
                feature = "gles10",
            ))]
            glTexParameteri: unsafe {
                unsafe extern "system" fn __glTexParameteri(
                    _target: GLenum,
                    _pname: GLenum,
                    _param: GLint,
                ) {
                    panic!("Unable to load glTexParameteri")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexParameteri\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexParameteri
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glProgramUniform2uiv: unsafe {
                unsafe extern "system" fn __glProgramUniform2uiv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLuint,
                ) {
                    panic!("Unable to load glProgramUniform2uiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform2uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform2uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glNormal3f: unsafe {
                unsafe extern "system" fn __glNormal3f(_nx: GLfloat, _ny: GLfloat, _nz: GLfloat) {
                    panic!("Unable to load glNormal3f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glNormal3f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glNormal3f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles32",
                feature = "gles30",
                feature = "gles31",
                feature = "gles20",
            ))]
            glUniform2iv: unsafe {
                unsafe extern "system" fn __glUniform2iv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLint,
                ) {
                    panic!("Unable to load glUniform2iv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform2iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform2iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glClearDepthx: unsafe {
                unsafe extern "system" fn __glClearDepthx(_depth: GLfixed) {
                    panic!("Unable to load glClearDepthx")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClearDepthx\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClearDepthx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glProgramUniform3ui: unsafe {
                unsafe extern "system" fn __glProgramUniform3ui(
                    _program: GLuint,
                    _location: GLint,
                    _v0: GLuint,
                    _v1: GLuint,
                    _v2: GLuint,
                ) {
                    panic!("Unable to load glProgramUniform3ui")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform3ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform3ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles32",
                feature = "gles20",
                feature = "gles30",
                feature = "gles31",
            ))]
            glShaderBinary: unsafe {
                unsafe extern "system" fn __glShaderBinary(
                    _count: GLsizei,
                    _shaders: *const GLuint,
                    _binaryFormat: GLenum,
                    _binary: *const std::os::raw::c_void,
                    _length: GLsizei,
                ) {
                    panic!("Unable to load glShaderBinary")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glShaderBinary\0");
                let val = _f(cname);
                if val.is_null() {
                    __glShaderBinary
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glProgramUniformMatrix2fv: unsafe {
                unsafe extern "system" fn __glProgramUniformMatrix2fv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: GLboolean,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load glProgramUniformMatrix2fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniformMatrix2fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniformMatrix2fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glDisablei: unsafe {
                unsafe extern "system" fn __glDisablei(_target: GLenum, _index: GLuint) {
                    panic!("Unable to load glDisablei")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDisablei\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDisablei
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles32",
                feature = "gles20",
                feature = "gles30",
                feature = "gles31",
            ))]
            glUniform3fv: unsafe {
                unsafe extern "system" fn __glUniform3fv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load glUniform3fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform3fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform3fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glUniformMatrix3x4fv: unsafe {
                unsafe extern "system" fn __glUniformMatrix3x4fv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: GLboolean,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load glUniformMatrix3x4fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix3x4fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformMatrix3x4fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glTexEnvfv: unsafe {
                unsafe extern "system" fn __glTexEnvfv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *const GLfloat,
                ) {
                    panic!("Unable to load glTexEnvfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexEnvfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexEnvfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glProgramUniformMatrix4fv: unsafe {
                unsafe extern "system" fn __glProgramUniformMatrix4fv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: GLboolean,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load glProgramUniformMatrix4fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniformMatrix4fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniformMatrix4fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles32",
                feature = "gles31",
                feature = "gles30",
            ))]
            glIsShader: unsafe {
                unsafe extern "system" fn __glIsShader(_shader: GLuint) -> GLboolean {
                    panic!("Unable to load glIsShader")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIsShader\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIsShader
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
            glSamplerParameterf: unsafe {
                unsafe extern "system" fn __glSamplerParameterf(
                    _sampler: GLuint,
                    _pname: GLenum,
                    _param: GLfloat,
                ) {
                    panic!("Unable to load glSamplerParameterf")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSamplerParameterf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSamplerParameterf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles32",
                feature = "gles30",
                feature = "gles31",
            ))]
            glGetRenderbufferParameteriv: unsafe {
                unsafe extern "system" fn __glGetRenderbufferParameteriv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetRenderbufferParameteriv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetRenderbufferParameteriv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetRenderbufferParameteriv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles20",
                feature = "gles10",
                feature = "gles32",
                feature = "gles31",
            ))]
            glDeleteTextures: unsafe {
                unsafe extern "system" fn __glDeleteTextures(
                    _n: GLsizei,
                    _textures: *const GLuint,
                ) {
                    panic!("Unable to load glDeleteTextures")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDeleteTextures\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDeleteTextures
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles31",
                feature = "gles32",
                feature = "gles10",
                feature = "gles20",
            ))]
            glDeleteBuffers: unsafe {
                unsafe extern "system" fn __glDeleteBuffers(_n: GLsizei, _buffers: *const GLuint) {
                    panic!("Unable to load glDeleteBuffers")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDeleteBuffers\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDeleteBuffers
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glLightxv: unsafe {
                unsafe extern "system" fn __glLightxv(
                    _light: GLenum,
                    _pname: GLenum,
                    _params: *const GLfixed,
                ) {
                    panic!("Unable to load glLightxv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLightxv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLightxv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles32",
                feature = "gles20",
                feature = "gles10",
                feature = "gles31",
            ))]
            glClear: unsafe {
                unsafe extern "system" fn __glClear(_mask: GLbitfield) {
                    panic!("Unable to load glClear")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClear\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClear
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glMultiTexCoord4x: unsafe {
                unsafe extern "system" fn __glMultiTexCoord4x(
                    _texture: GLenum,
                    _s: GLfixed,
                    _t: GLfixed,
                    _r: GLfixed,
                    _q: GLfixed,
                ) {
                    panic!("Unable to load glMultiTexCoord4x")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMultiTexCoord4x\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMultiTexCoord4x
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glGetTexLevelParameterfv: unsafe {
                unsafe extern "system" fn __glGetTexLevelParameterfv(
                    _target: GLenum,
                    _level: GLint,
                    _pname: GLenum,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load glGetTexLevelParameterfv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTexLevelParameterfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTexLevelParameterfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glGetTexLevelParameteriv: unsafe {
                unsafe extern "system" fn __glGetTexLevelParameteriv(
                    _target: GLenum,
                    _level: GLint,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetTexLevelParameteriv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTexLevelParameteriv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTexLevelParameteriv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glGetGraphicsResetStatus: unsafe {
                unsafe extern "system" fn __glGetGraphicsResetStatus() -> GLenum {
                    panic!("Unable to load glGetGraphicsResetStatus")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetGraphicsResetStatus\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetGraphicsResetStatus
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glDrawElementsInstancedBaseVertex: unsafe {
                unsafe extern "system" fn __glDrawElementsInstancedBaseVertex(
                    _mode: GLenum,
                    _count: GLsizei,
                    _type: GLenum,
                    _indices: *const std::os::raw::c_void,
                    _instancecount: GLsizei,
                    _basevertex: GLint,
                ) {
                    panic!("Unable to load glDrawElementsInstancedBaseVertex")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glDrawElementsInstancedBaseVertex\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glDrawElementsInstancedBaseVertex
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles30", feature = "gles32",))]
            glDrawElementsInstanced: unsafe {
                unsafe extern "system" fn __glDrawElementsInstanced(
                    _mode: GLenum,
                    _count: GLsizei,
                    _type: GLenum,
                    _indices: *const std::os::raw::c_void,
                    _instancecount: GLsizei,
                ) {
                    panic!("Unable to load glDrawElementsInstanced")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDrawElementsInstanced\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDrawElementsInstanced
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
            glUniformMatrix3x2fv: unsafe {
                unsafe extern "system" fn __glUniformMatrix3x2fv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: GLboolean,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load glUniformMatrix3x2fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix3x2fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformMatrix3x2fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles32",
                feature = "gles31",
                feature = "gles20",
                feature = "gles30",
            ))]
            glDeleteShader: unsafe {
                unsafe extern "system" fn __glDeleteShader(_shader: GLuint) {
                    panic!("Unable to load glDeleteShader")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDeleteShader\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDeleteShader
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles31",
                feature = "gles20",
                feature = "gles32",
                feature = "gles10",
                feature = "gles30",
            ))]
            glGetTexParameteriv: unsafe {
                unsafe extern "system" fn __glGetTexParameteriv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetTexParameteriv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTexParameteriv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTexParameteriv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glSamplerParameterIuiv: unsafe {
                unsafe extern "system" fn __glSamplerParameterIuiv(
                    _sampler: GLuint,
                    _pname: GLenum,
                    _param: *const GLuint,
                ) {
                    panic!("Unable to load glSamplerParameterIuiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSamplerParameterIuiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSamplerParameterIuiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glPopDebugGroup: unsafe {
                unsafe extern "system" fn __glPopDebugGroup() {
                    panic!("Unable to load glPopDebugGroup")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPopDebugGroup\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPopDebugGroup
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
            glBeginTransformFeedback: unsafe {
                unsafe extern "system" fn __glBeginTransformFeedback(_primitiveMode: GLenum) {
                    panic!("Unable to load glBeginTransformFeedback")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBeginTransformFeedback\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBeginTransformFeedback
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles10",
                feature = "gles32",
                feature = "gles30",
                feature = "gles31",
            ))]
            glIsEnabled: unsafe {
                unsafe extern "system" fn __glIsEnabled(_cap: GLenum) -> GLboolean {
                    panic!("Unable to load glIsEnabled")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIsEnabled\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIsEnabled
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles32",
                feature = "gles31",
                feature = "gles20",
                feature = "gles30",
            ))]
            glDeleteRenderbuffers: unsafe {
                unsafe extern "system" fn __glDeleteRenderbuffers(
                    _n: GLsizei,
                    _renderbuffers: *const GLuint,
                ) {
                    panic!("Unable to load glDeleteRenderbuffers")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDeleteRenderbuffers\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDeleteRenderbuffers
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
            glGetInteger64v: unsafe {
                unsafe extern "system" fn __glGetInteger64v(_pname: GLenum, _data: *mut GLint64) {
                    panic!("Unable to load glGetInteger64v")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetInteger64v\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetInteger64v
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
            glGetActiveUniformsiv: unsafe {
                unsafe extern "system" fn __glGetActiveUniformsiv(
                    _program: GLuint,
                    _uniformCount: GLsizei,
                    _uniformIndices: *const GLuint,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetActiveUniformsiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetActiveUniformsiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetActiveUniformsiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glDebugMessageInsert: unsafe {
                unsafe extern "system" fn __glDebugMessageInsert(
                    _source: GLenum,
                    _type: GLenum,
                    _id: GLuint,
                    _severity: GLenum,
                    _length: GLsizei,
                    _buf: *const GLchar,
                ) {
                    panic!("Unable to load glDebugMessageInsert")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDebugMessageInsert\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDebugMessageInsert
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glLightModelfv: unsafe {
                unsafe extern "system" fn __glLightModelfv(
                    _pname: GLenum,
                    _params: *const GLfloat,
                ) {
                    panic!("Unable to load glLightModelfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLightModelfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLightModelfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles31",
                feature = "gles30",
                feature = "gles32",
            ))]
            glUniform2i: unsafe {
                unsafe extern "system" fn __glUniform2i(_location: GLint, _v0: GLint, _v1: GLint) {
                    panic!("Unable to load glUniform2i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform2i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform2i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glPointParameterfv: unsafe {
                unsafe extern "system" fn __glPointParameterfv(
                    _pname: GLenum,
                    _params: *const GLfloat,
                ) {
                    panic!("Unable to load glPointParameterfv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPointParameterfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPointParameterfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles20",
                feature = "gles31",
                feature = "gles32",
            ))]
            glShaderSource: unsafe {
                unsafe extern "system" fn __glShaderSource(
                    _shader: GLuint,
                    _count: GLsizei,
                    _string: *const *const GLchar,
                    _length: *const GLint,
                ) {
                    panic!("Unable to load glShaderSource")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glShaderSource\0");
                let val = _f(cname);
                if val.is_null() {
                    __glShaderSource
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glBlendEquationSeparatei: unsafe {
                unsafe extern "system" fn __glBlendEquationSeparatei(
                    _buf: GLuint,
                    _modeRGB: GLenum,
                    _modeAlpha: GLenum,
                ) {
                    panic!("Unable to load glBlendEquationSeparatei")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBlendEquationSeparatei\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBlendEquationSeparatei
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles32",
                feature = "gles20",
                feature = "gles30",
                feature = "gles31",
            ))]
            glAttachShader: unsafe {
                unsafe extern "system" fn __glAttachShader(_program: GLuint, _shader: GLuint) {
                    panic!("Unable to load glAttachShader")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glAttachShader\0");
                let val = _f(cname);
                if val.is_null() {
                    __glAttachShader
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles31",
                feature = "gles32",
                feature = "gles30",
                feature = "gles20",
            ))]
            glGetUniformfv: unsafe {
                unsafe extern "system" fn __glGetUniformfv(
                    _program: GLuint,
                    _location: GLint,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load glGetUniformfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetUniformfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetUniformfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glFramebufferParameteri: unsafe {
                unsafe extern "system" fn __glFramebufferParameteri(
                    _target: GLenum,
                    _pname: GLenum,
                    _param: GLint,
                ) {
                    panic!("Unable to load glFramebufferParameteri")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFramebufferParameteri\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFramebufferParameteri
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glOrthof: unsafe {
                unsafe extern "system" fn __glOrthof(
                    _l: GLfloat,
                    _r: GLfloat,
                    _b: GLfloat,
                    _t: GLfloat,
                    _n: GLfloat,
                    _f: GLfloat,
                ) {
                    panic!("Unable to load glOrthof")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glOrthof\0");
                let val = _f(cname);
                if val.is_null() {
                    __glOrthof
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
            glClearBufferiv: unsafe {
                unsafe extern "system" fn __glClearBufferiv(
                    _buffer: GLenum,
                    _drawbuffer: GLint,
                    _value: *const GLint,
                ) {
                    panic!("Unable to load glClearBufferiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClearBufferiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClearBufferiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
            glIsVertexArray: unsafe {
                unsafe extern "system" fn __glIsVertexArray(_array: GLuint) -> GLboolean {
                    panic!("Unable to load glIsVertexArray")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIsVertexArray\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIsVertexArray
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles32", feature = "gles31",))]
            glGetVertexAttribIiv: unsafe {
                unsafe extern "system" fn __glGetVertexAttribIiv(
                    _index: GLuint,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetVertexAttribIiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetVertexAttribIiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetVertexAttribIiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31", feature = "gles30",))]
            glGetUniformBlockIndex: unsafe {
                unsafe extern "system" fn __glGetUniformBlockIndex(
                    _program: GLuint,
                    _uniformBlockName: *const GLchar,
                ) -> GLuint {
                    panic!("Unable to load glGetUniformBlockIndex")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetUniformBlockIndex\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetUniformBlockIndex
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles10",
                feature = "gles20",
                feature = "gles30",
                feature = "gles31",
                feature = "gles32",
            ))]
            glPixelStorei: unsafe {
                unsafe extern "system" fn __glPixelStorei(_pname: GLenum, _param: GLint) {
                    panic!("Unable to load glPixelStorei")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPixelStorei\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPixelStorei
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles32",
                feature = "gles30",
                feature = "gles20",
                feature = "gles31",
            ))]
            glGetVertexAttribfv: unsafe {
                unsafe extern "system" fn __glGetVertexAttribfv(
                    _index: GLuint,
                    _pname: GLenum,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load glGetVertexAttribfv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetVertexAttribfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetVertexAttribfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles32",
                feature = "gles20",
                feature = "gles31",
                feature = "gles30",
            ))]
            glDeleteFramebuffers: unsafe {
                unsafe extern "system" fn __glDeleteFramebuffers(
                    _n: GLsizei,
                    _framebuffers: *const GLuint,
                ) {
                    panic!("Unable to load glDeleteFramebuffers")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDeleteFramebuffers\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDeleteFramebuffers
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glIsQuery: unsafe {
                unsafe extern "system" fn __glIsQuery(_id: GLuint) -> GLboolean {
                    panic!("Unable to load glIsQuery")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIsQuery\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIsQuery
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glProgramUniform3uiv: unsafe {
                unsafe extern "system" fn __glProgramUniform3uiv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLuint,
                ) {
                    panic!("Unable to load glProgramUniform3uiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform3uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform3uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles20",
                feature = "gles31",
                feature = "gles32",
                feature = "gles10",
            ))]
            glGetFloatv: unsafe {
                unsafe extern "system" fn __glGetFloatv(_pname: GLenum, _data: *mut GLfloat) {
                    panic!("Unable to load glGetFloatv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetFloatv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetFloatv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles10",
                feature = "gles32",
                feature = "gles30",
                feature = "gles31",
                feature = "gles20",
            ))]
            glDrawElements: unsafe {
                unsafe extern "system" fn __glDrawElements(
                    _mode: GLenum,
                    _count: GLsizei,
                    _type: GLenum,
                    _indices: *const std::os::raw::c_void,
                ) {
                    panic!("Unable to load glDrawElements")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDrawElements\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDrawElements
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
            glGenVertexArrays: unsafe {
                unsafe extern "system" fn __glGenVertexArrays(_n: GLsizei, _arrays: *mut GLuint) {
                    panic!("Unable to load glGenVertexArrays")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGenVertexArrays\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGenVertexArrays
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glGetQueryiv: unsafe {
                unsafe extern "system" fn __glGetQueryiv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetQueryiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetQueryiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetQueryiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glGetProgramResourceName: unsafe {
                unsafe extern "system" fn __glGetProgramResourceName(
                    _program: GLuint,
                    _programInterface: GLenum,
                    _index: GLuint,
                    _bufSize: GLsizei,
                    _length: *mut GLsizei,
                    _name: *mut GLchar,
                ) {
                    panic!("Unable to load glGetProgramResourceName")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetProgramResourceName\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetProgramResourceName
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glGetProgramInterfaceiv: unsafe {
                unsafe extern "system" fn __glGetProgramInterfaceiv(
                    _program: GLuint,
                    _programInterface: GLenum,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetProgramInterfaceiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetProgramInterfaceiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetProgramInterfaceiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles10",
                feature = "gles31",
                feature = "gles32",
                feature = "gles30",
            ))]
            glBufferData: unsafe {
                unsafe extern "system" fn __glBufferData(
                    _target: GLenum,
                    _size: GLsizeiptr,
                    _data: *const std::os::raw::c_void,
                    _usage: GLenum,
                ) {
                    panic!("Unable to load glBufferData")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBufferData\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBufferData
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glBlendBarrier: unsafe {
                unsafe extern "system" fn __glBlendBarrier() {
                    panic!("Unable to load glBlendBarrier")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBlendBarrier\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBlendBarrier
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles31",
                feature = "gles32",
                feature = "gles20",
                feature = "gles30",
                feature = "gles10",
            ))]
            glTexParameteriv: unsafe {
                unsafe extern "system" fn __glTexParameteriv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *const GLint,
                ) {
                    panic!("Unable to load glTexParameteriv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexParameteriv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexParameteriv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
            glDrawRangeElements: unsafe {
                unsafe extern "system" fn __glDrawRangeElements(
                    _mode: GLenum,
                    _start: GLuint,
                    _end: GLuint,
                    _count: GLsizei,
                    _type: GLenum,
                    _indices: *const std::os::raw::c_void,
                ) {
                    panic!("Unable to load glDrawRangeElements")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDrawRangeElements\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDrawRangeElements
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
            glUniform4uiv: unsafe {
                unsafe extern "system" fn __glUniform4uiv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLuint,
                ) {
                    panic!("Unable to load glUniform4uiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform4uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform4uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles32", feature = "gles31",))]
            glVertexAttribI4ui: unsafe {
                unsafe extern "system" fn __glVertexAttribI4ui(
                    _index: GLuint,
                    _x: GLuint,
                    _y: GLuint,
                    _z: GLuint,
                    _w: GLuint,
                ) {
                    panic!("Unable to load glVertexAttribI4ui")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI4ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribI4ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles31",
                feature = "gles10",
                feature = "gles20",
                feature = "gles32",
            ))]
            glFrontFace: unsafe {
                unsafe extern "system" fn __glFrontFace(_mode: GLenum) {
                    panic!("Unable to load glFrontFace")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFrontFace\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFrontFace
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glMapBufferRange: unsafe {
                unsafe extern "system" fn __glMapBufferRange(
                    _target: GLenum,
                    _offset: GLintptr,
                    _length: GLsizeiptr,
                    _access: GLbitfield,
                ) -> *mut std::os::raw::c_void {
                    panic!("Unable to load glMapBufferRange")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glMapBufferRange\0");
                let val = _f(cname);
                if val.is_null() {
                    __glMapBufferRange
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
            glResumeTransformFeedback: unsafe {
                unsafe extern "system" fn __glResumeTransformFeedback() {
                    panic!("Unable to load glResumeTransformFeedback")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glResumeTransformFeedback\0");
                let val = _f(cname);
                if val.is_null() {
                    __glResumeTransformFeedback
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31", feature = "gles30",))]
            glTransformFeedbackVaryings: unsafe {
                unsafe extern "system" fn __glTransformFeedbackVaryings(
                    _program: GLuint,
                    _count: GLsizei,
                    _varyings: *const *const GLchar,
                    _bufferMode: GLenum,
                ) {
                    panic!("Unable to load glTransformFeedbackVaryings")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glTransformFeedbackVaryings\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glTransformFeedbackVaryings
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glGenProgramPipelines: unsafe {
                unsafe extern "system" fn __glGenProgramPipelines(
                    _n: GLsizei,
                    _pipelines: *mut GLuint,
                ) {
                    panic!("Unable to load glGenProgramPipelines")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGenProgramPipelines\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGenProgramPipelines
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glProgramUniformMatrix3x2fv: unsafe {
                unsafe extern "system" fn __glProgramUniformMatrix3x2fv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: GLboolean,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load glProgramUniformMatrix3x2fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glProgramUniformMatrix3x2fv\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniformMatrix3x2fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glDrawArraysIndirect: unsafe {
                unsafe extern "system" fn __glDrawArraysIndirect(
                    _mode: GLenum,
                    _indirect: *const std::os::raw::c_void,
                ) {
                    panic!("Unable to load glDrawArraysIndirect")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDrawArraysIndirect\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDrawArraysIndirect
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glLightModelxv: unsafe {
                unsafe extern "system" fn __glLightModelxv(_pname: GLenum, _param: *const GLfixed) {
                    panic!("Unable to load glLightModelxv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLightModelxv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLightModelxv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles10",
                feature = "gles31",
                feature = "gles20",
                feature = "gles30",
                feature = "gles32",
            ))]
            glLineWidth: unsafe {
                unsafe extern "system" fn __glLineWidth(_width: GLfloat) {
                    panic!("Unable to load glLineWidth")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLineWidth\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLineWidth
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles30",
                feature = "gles31",
                feature = "gles32",
            ))]
            glBlendColor: unsafe {
                unsafe extern "system" fn __glBlendColor(
                    _red: GLfloat,
                    _green: GLfloat,
                    _blue: GLfloat,
                    _alpha: GLfloat,
                ) {
                    panic!("Unable to load glBlendColor")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBlendColor\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBlendColor
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31", feature = "gles30",))]
            glGetInteger64i_v: unsafe {
                unsafe extern "system" fn __glGetInteger64i_v(
                    _target: GLenum,
                    _index: GLuint,
                    _data: *mut GLint64,
                ) {
                    panic!("Unable to load glGetInteger64i_v")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetInteger64i_v\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetInteger64i_v
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glGetClipPlanex: unsafe {
                unsafe extern "system" fn __glGetClipPlanex(
                    _plane: GLenum,
                    _equation: *mut GLfixed,
                ) {
                    panic!("Unable to load glGetClipPlanex")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetClipPlanex\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetClipPlanex
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles31",
                feature = "gles20",
                feature = "gles32",
                feature = "gles30",
                feature = "gles10",
            ))]
            glDepthFunc: unsafe {
                unsafe extern "system" fn __glDepthFunc(_func: GLenum) {
                    panic!("Unable to load glDepthFunc")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDepthFunc\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDepthFunc
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glVertexAttribI4uiv: unsafe {
                unsafe extern "system" fn __glVertexAttribI4uiv(_index: GLuint, _v: *const GLuint) {
                    panic!("Unable to load glVertexAttribI4uiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI4uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribI4uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glLoadMatrixf: unsafe {
                unsafe extern "system" fn __glLoadMatrixf(_m: *const GLfloat) {
                    panic!("Unable to load glLoadMatrixf")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLoadMatrixf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLoadMatrixf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glUniformMatrix2x4fv: unsafe {
                unsafe extern "system" fn __glUniformMatrix2x4fv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: GLboolean,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load glUniformMatrix2x4fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix2x4fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformMatrix2x4fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glCreateShaderProgramv: unsafe {
                unsafe extern "system" fn __glCreateShaderProgramv(
                    _type: GLenum,
                    _count: GLsizei,
                    _strings: *const *const GLchar,
                ) -> GLuint {
                    panic!("Unable to load glCreateShaderProgramv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCreateShaderProgramv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCreateShaderProgramv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
            glInvalidateSubFramebuffer: unsafe {
                unsafe extern "system" fn __glInvalidateSubFramebuffer(
                    _target: GLenum,
                    _numAttachments: GLsizei,
                    _attachments: *const GLenum,
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                ) {
                    panic!("Unable to load glInvalidateSubFramebuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glInvalidateSubFramebuffer\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glInvalidateSubFramebuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles31",
                feature = "gles10",
                feature = "gles20",
                feature = "gles30",
                feature = "gles32",
            ))]
            glColorMask: unsafe {
                unsafe extern "system" fn __glColorMask(
                    _red: GLboolean,
                    _green: GLboolean,
                    _blue: GLboolean,
                    _alpha: GLboolean,
                ) {
                    panic!("Unable to load glColorMask")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColorMask\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColorMask
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glGetMaterialxv: unsafe {
                unsafe extern "system" fn __glGetMaterialxv(
                    _face: GLenum,
                    _pname: GLenum,
                    _params: *mut GLfixed,
                ) {
                    panic!("Unable to load glGetMaterialxv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetMaterialxv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetMaterialxv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glGetClipPlanef: unsafe {
                unsafe extern "system" fn __glGetClipPlanef(
                    _plane: GLenum,
                    _equation: *mut GLfloat,
                ) {
                    panic!("Unable to load glGetClipPlanef")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetClipPlanef\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetClipPlanef
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glTexEnviv: unsafe {
                unsafe extern "system" fn __glTexEnviv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *const GLint,
                ) {
                    panic!("Unable to load glTexEnviv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexEnviv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexEnviv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles30",
                feature = "gles10",
                feature = "gles31",
                feature = "gles32",
            ))]
            glGetBufferParameteriv: unsafe {
                unsafe extern "system" fn __glGetBufferParameteriv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetBufferParameteriv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetBufferParameteriv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetBufferParameteriv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glColor4f: unsafe {
                unsafe extern "system" fn __glColor4f(
                    _red: GLfloat,
                    _green: GLfloat,
                    _blue: GLfloat,
                    _alpha: GLfloat,
                ) {
                    panic!("Unable to load glColor4f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glColor4f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glColor4f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles30",
                feature = "gles10",
                feature = "gles31",
                feature = "gles32",
            ))]
            glGetIntegerv: unsafe {
                unsafe extern "system" fn __glGetIntegerv(_pname: GLenum, _data: *mut GLint) {
                    panic!("Unable to load glGetIntegerv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetIntegerv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetIntegerv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles32",
                feature = "gles30",
                feature = "gles31",
                feature = "gles20",
            ))]
            glGetShaderiv: unsafe {
                unsafe extern "system" fn __glGetShaderiv(
                    _shader: GLuint,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetShaderiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetShaderiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetShaderiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles20",
                feature = "gles32",
                feature = "gles31",
            ))]
            glIsProgram: unsafe {
                unsafe extern "system" fn __glIsProgram(_program: GLuint) -> GLboolean {
                    panic!("Unable to load glIsProgram")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIsProgram\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIsProgram
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles32",
                feature = "gles20",
                feature = "gles31",
                feature = "gles30",
            ))]
            glBlendEquationSeparate: unsafe {
                unsafe extern "system" fn __glBlendEquationSeparate(
                    _modeRGB: GLenum,
                    _modeAlpha: GLenum,
                ) {
                    panic!("Unable to load glBlendEquationSeparate")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBlendEquationSeparate\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBlendEquationSeparate
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31", feature = "gles30",))]
            glCompressedTexSubImage3D: unsafe {
                unsafe extern "system" fn __glCompressedTexSubImage3D(
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
                    panic!("Unable to load glCompressedTexSubImage3D")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCompressedTexSubImage3D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCompressedTexSubImage3D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
            glVertexAttribIPointer: unsafe {
                unsafe extern "system" fn __glVertexAttribIPointer(
                    _index: GLuint,
                    _size: GLint,
                    _type: GLenum,
                    _stride: GLsizei,
                    _pointer: *const std::os::raw::c_void,
                ) {
                    panic!("Unable to load glVertexAttribIPointer")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribIPointer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribIPointer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glGenQueries: unsafe {
                unsafe extern "system" fn __glGenQueries(_n: GLsizei, _ids: *mut GLuint) {
                    panic!("Unable to load glGenQueries")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGenQueries\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGenQueries
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles31",
                feature = "gles32",
                feature = "gles20",
            ))]
            glVertexAttrib1f: unsafe {
                unsafe extern "system" fn __glVertexAttrib1f(_index: GLuint, _x: GLfloat) {
                    panic!("Unable to load glVertexAttrib1f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib1f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib1f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles31",
                feature = "gles30",
                feature = "gles32",
                feature = "gles20",
                feature = "gles10",
            ))]
            glFlush: unsafe {
                unsafe extern "system" fn __glFlush() {
                    panic!("Unable to load glFlush")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFlush\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFlush
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles31",
                feature = "gles32",
                feature = "gles20",
                feature = "gles30",
            ))]
            glGetActiveAttrib: unsafe {
                unsafe extern "system" fn __glGetActiveAttrib(
                    _program: GLuint,
                    _index: GLuint,
                    _bufSize: GLsizei,
                    _length: *mut GLsizei,
                    _size: *mut GLint,
                    _type: *mut GLenum,
                    _name: *mut GLchar,
                ) {
                    panic!("Unable to load glGetActiveAttrib")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetActiveAttrib\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetActiveAttrib
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles30",
                feature = "gles32",
                feature = "gles31",
            ))]
            glCreateProgram: unsafe {
                unsafe extern "system" fn __glCreateProgram() -> GLuint {
                    panic!("Unable to load glCreateProgram")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCreateProgram\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCreateProgram
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles31",
                feature = "gles32",
                feature = "gles10",
                feature = "gles20",
            ))]
            glTexSubImage2D: unsafe {
                unsafe extern "system" fn __glTexSubImage2D(
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
                    panic!("Unable to load glTexSubImage2D")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexSubImage2D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexSubImage2D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
            glIsSampler: unsafe {
                unsafe extern "system" fn __glIsSampler(_sampler: GLuint) -> GLboolean {
                    panic!("Unable to load glIsSampler")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIsSampler\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIsSampler
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glVertexAttribBinding: unsafe {
                unsafe extern "system" fn __glVertexAttribBinding(
                    _attribindex: GLuint,
                    _bindingindex: GLuint,
                ) {
                    panic!("Unable to load glVertexAttribBinding")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribBinding\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribBinding
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glProgramUniform3i: unsafe {
                unsafe extern "system" fn __glProgramUniform3i(
                    _program: GLuint,
                    _location: GLint,
                    _v0: GLint,
                    _v1: GLint,
                    _v2: GLint,
                ) {
                    panic!("Unable to load glProgramUniform3i")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform3i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform3i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glUniformMatrix4x3fv: unsafe {
                unsafe extern "system" fn __glUniformMatrix4x3fv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: GLboolean,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load glUniformMatrix4x3fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix4x3fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformMatrix4x3fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles31",
                feature = "gles32",
                feature = "gles30",
                feature = "gles20",
            ))]
            glUniformMatrix3fv: unsafe {
                unsafe extern "system" fn __glUniformMatrix3fv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: GLboolean,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load glUniformMatrix3fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix3fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformMatrix3fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles32",
                feature = "gles20",
                feature = "gles30",
                feature = "gles10",
                feature = "gles31",
            ))]
            glGenBuffers: unsafe {
                unsafe extern "system" fn __glGenBuffers(_n: GLsizei, _buffers: *mut GLuint) {
                    panic!("Unable to load glGenBuffers")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGenBuffers\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGenBuffers
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glBindImageTexture: unsafe {
                unsafe extern "system" fn __glBindImageTexture(
                    _unit: GLuint,
                    _texture: GLuint,
                    _level: GLint,
                    _layered: GLboolean,
                    _layer: GLint,
                    _access: GLenum,
                    _format: GLenum,
                ) {
                    panic!("Unable to load glBindImageTexture")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindImageTexture\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindImageTexture
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glGetLightfv: unsafe {
                unsafe extern "system" fn __glGetLightfv(
                    _light: GLenum,
                    _pname: GLenum,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load glGetLightfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetLightfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetLightfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glBindVertexBuffer: unsafe {
                unsafe extern "system" fn __glBindVertexBuffer(
                    _bindingindex: GLuint,
                    _buffer: GLuint,
                    _offset: GLintptr,
                    _stride: GLsizei,
                ) {
                    panic!("Unable to load glBindVertexBuffer")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindVertexBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindVertexBuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles10",
                feature = "gles30",
                feature = "gles32",
                feature = "gles20",
                feature = "gles31",
            ))]
            glGetString: unsafe {
                unsafe extern "system" fn __glGetString(_name: GLenum) -> *const GLubyte {
                    panic!("Unable to load glGetString")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetString\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetString
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31", feature = "gles30",))]
            glFenceSync: unsafe {
                unsafe extern "system" fn __glFenceSync(
                    _condition: GLenum,
                    _flags: GLbitfield,
                ) -> GLsync {
                    panic!("Unable to load glFenceSync")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFenceSync\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFenceSync
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glGetProgramPipelineInfoLog: unsafe {
                unsafe extern "system" fn __glGetProgramPipelineInfoLog(
                    _pipeline: GLuint,
                    _bufSize: GLsizei,
                    _length: *mut GLsizei,
                    _infoLog: *mut GLchar,
                ) {
                    panic!("Unable to load glGetProgramPipelineInfoLog")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetProgramPipelineInfoLog\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetProgramPipelineInfoLog
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles31",))]
            glGetProgramPipelineiv: unsafe {
                unsafe extern "system" fn __glGetProgramPipelineiv(
                    _pipeline: GLuint,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetProgramPipelineiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetProgramPipelineiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetProgramPipelineiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glSamplerParameterIiv: unsafe {
                unsafe extern "system" fn __glSamplerParameterIiv(
                    _sampler: GLuint,
                    _pname: GLenum,
                    _param: *const GLint,
                ) {
                    panic!("Unable to load glSamplerParameterIiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glSamplerParameterIiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glSamplerParameterIiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glGetnUniformfv: unsafe {
                unsafe extern "system" fn __glGetnUniformfv(
                    _program: GLuint,
                    _location: GLint,
                    _bufSize: GLsizei,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load glGetnUniformfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetnUniformfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetnUniformfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles31",
                feature = "gles32",
                feature = "gles20",
            ))]
            glUniform2f: unsafe {
                unsafe extern "system" fn __glUniform2f(
                    _location: GLint,
                    _v0: GLfloat,
                    _v1: GLfloat,
                ) {
                    panic!("Unable to load glUniform2f")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform2f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform2f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles32",
                feature = "gles31",
                feature = "gles20",
                feature = "gles10",
            ))]
            glGetError: unsafe {
                unsafe extern "system" fn __glGetError() -> GLenum {
                    panic!("Unable to load glGetError")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetError\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetError
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glPushDebugGroup: unsafe {
                unsafe extern "system" fn __glPushDebugGroup(
                    _source: GLenum,
                    _id: GLuint,
                    _length: GLsizei,
                    _message: *const GLchar,
                ) {
                    panic!("Unable to load glPushDebugGroup")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPushDebugGroup\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPushDebugGroup
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glProgramUniform2fv: unsafe {
                unsafe extern "system" fn __glProgramUniform2fv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load glProgramUniform2fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform2fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform2fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glTexBuffer: unsafe {
                unsafe extern "system" fn __glTexBuffer(
                    _target: GLenum,
                    _internalformat: GLenum,
                    _buffer: GLuint,
                ) {
                    panic!("Unable to load glTexBuffer")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexBuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles32",
                feature = "gles20",
                feature = "gles31",
                feature = "gles30",
            ))]
            glGetShaderPrecisionFormat: unsafe {
                unsafe extern "system" fn __glGetShaderPrecisionFormat(
                    _shadertype: GLenum,
                    _precisiontype: GLenum,
                    _range: *mut GLint,
                    _precision: *mut GLint,
                ) {
                    panic!("Unable to load glGetShaderPrecisionFormat")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"glGetShaderPrecisionFormat\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    __glGetShaderPrecisionFormat
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles30", feature = "gles32",))]
            glUniform1uiv: unsafe {
                unsafe extern "system" fn __glUniform1uiv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLuint,
                ) {
                    panic!("Unable to load glUniform1uiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform1uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform1uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles10",
                feature = "gles31",
                feature = "gles32",
                feature = "gles30",
                feature = "gles20",
            ))]
            glTexImage2D: unsafe {
                unsafe extern "system" fn __glTexImage2D(
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
                    panic!("Unable to load glTexImage2D")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexImage2D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexImage2D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glFogfv: unsafe {
                unsafe extern "system" fn __glFogfv(_pname: GLenum, _params: *const GLfloat) {
                    panic!("Unable to load glFogfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glFogfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glFogfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
            glUniform3ui: unsafe {
                unsafe extern "system" fn __glUniform3ui(
                    _location: GLint,
                    _v0: GLuint,
                    _v1: GLuint,
                    _v2: GLuint,
                ) {
                    panic!("Unable to load glUniform3ui")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform3ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform3ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles30", feature = "gles32",))]
            glDrawArraysInstanced: unsafe {
                unsafe extern "system" fn __glDrawArraysInstanced(
                    _mode: GLenum,
                    _first: GLint,
                    _count: GLsizei,
                    _instancecount: GLsizei,
                ) {
                    panic!("Unable to load glDrawArraysInstanced")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDrawArraysInstanced\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDrawArraysInstanced
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
            glGetUniformuiv: unsafe {
                unsafe extern "system" fn __glGetUniformuiv(
                    _program: GLuint,
                    _location: GLint,
                    _params: *mut GLuint,
                ) {
                    panic!("Unable to load glGetUniformuiv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetUniformuiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetUniformuiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glTexStorage3DMultisample: unsafe {
                unsafe extern "system" fn __glTexStorage3DMultisample(
                    _target: GLenum,
                    _samples: GLsizei,
                    _internalformat: GLenum,
                    _width: GLsizei,
                    _height: GLsizei,
                    _depth: GLsizei,
                    _fixedsamplelocations: GLboolean,
                ) {
                    panic!("Unable to load glTexStorage3DMultisample")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexStorage3DMultisample\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexStorage3DMultisample
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glScalex: unsafe {
                unsafe extern "system" fn __glScalex(_x: GLfixed, _y: GLfixed, _z: GLfixed) {
                    panic!("Unable to load glScalex")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glScalex\0");
                let val = _f(cname);
                if val.is_null() {
                    __glScalex
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles32",
                feature = "gles20",
                feature = "gles30",
                feature = "gles31",
            ))]
            glStencilFuncSeparate: unsafe {
                unsafe extern "system" fn __glStencilFuncSeparate(
                    _face: GLenum,
                    _func: GLenum,
                    _ref: GLint,
                    _mask: GLuint,
                ) {
                    panic!("Unable to load glStencilFuncSeparate")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glStencilFuncSeparate\0");
                let val = _f(cname);
                if val.is_null() {
                    __glStencilFuncSeparate
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles30",
                feature = "gles32",
                feature = "gles31",
            ))]
            glUniform1i: unsafe {
                unsafe extern "system" fn __glUniform1i(_location: GLint, _v0: GLint) {
                    panic!("Unable to load glUniform1i")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform1i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform1i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles31",
                feature = "gles30",
                feature = "gles20",
                feature = "gles32",
            ))]
            glBindRenderbuffer: unsafe {
                unsafe extern "system" fn __glBindRenderbuffer(
                    _target: GLenum,
                    _renderbuffer: GLuint,
                ) {
                    panic!("Unable to load glBindRenderbuffer")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBindRenderbuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBindRenderbuffer
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glGetTexEnvfv: unsafe {
                unsafe extern "system" fn __glGetTexEnvfv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load glGetTexEnvfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTexEnvfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTexEnvfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles10",
                feature = "gles30",
                feature = "gles31",
                feature = "gles32",
                feature = "gles20",
            ))]
            glDrawArrays: unsafe {
                unsafe extern "system" fn __glDrawArrays(
                    _mode: GLenum,
                    _first: GLint,
                    _count: GLsizei,
                ) {
                    panic!("Unable to load glDrawArrays")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDrawArrays\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDrawArrays
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glDepthRangex: unsafe {
                unsafe extern "system" fn __glDepthRangex(_n: GLfixed, _f: GLfixed) {
                    panic!("Unable to load glDepthRangex")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDepthRangex\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDepthRangex
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles32",
                feature = "gles20",
                feature = "gles31",
                feature = "gles30",
            ))]
            glUniformMatrix2fv: unsafe {
                unsafe extern "system" fn __glUniformMatrix2fv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: GLboolean,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load glUniformMatrix2fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix2fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniformMatrix2fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glGetUniformIndices: unsafe {
                unsafe extern "system" fn __glGetUniformIndices(
                    _program: GLuint,
                    _uniformCount: GLsizei,
                    _uniformNames: *const *const GLchar,
                    _uniformIndices: *mut GLuint,
                ) {
                    panic!("Unable to load glGetUniformIndices")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetUniformIndices\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetUniformIndices
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glPointSizex: unsafe {
                unsafe extern "system" fn __glPointSizex(_size: GLfixed) {
                    panic!("Unable to load glPointSizex")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPointSizex\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPointSizex
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glClipPlanex: unsafe {
                unsafe extern "system" fn __glClipPlanex(
                    _plane: GLenum,
                    _equation: *const GLfixed,
                ) {
                    panic!("Unable to load glClipPlanex")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClipPlanex\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClipPlanex
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles32",
                feature = "gles10",
                feature = "gles20",
                feature = "gles30",
                feature = "gles31",
            ))]
            glReadPixels: unsafe {
                unsafe extern "system" fn __glReadPixels(
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                    _format: GLenum,
                    _type: GLenum,
                    _pixels: *mut std::os::raw::c_void,
                ) {
                    panic!("Unable to load glReadPixels")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glReadPixels\0");
                let val = _f(cname);
                if val.is_null() {
                    __glReadPixels
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles32",
                feature = "gles20",
                feature = "gles10",
                feature = "gles30",
                feature = "gles31",
            ))]
            glEnable: unsafe {
                unsafe extern "system" fn __glEnable(_cap: GLenum) {
                    panic!("Unable to load glEnable")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glEnable\0");
                let val = _f(cname);
                if val.is_null() {
                    __glEnable
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles32",
                feature = "gles31",
                feature = "gles20",
                feature = "gles30",
            ))]
            glReleaseShaderCompiler: unsafe {
                unsafe extern "system" fn __glReleaseShaderCompiler() {
                    panic!("Unable to load glReleaseShaderCompiler")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glReleaseShaderCompiler\0");
                let val = _f(cname);
                if val.is_null() {
                    __glReleaseShaderCompiler
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles31",
                feature = "gles20",
                feature = "gles32",
            ))]
            glGenRenderbuffers: unsafe {
                unsafe extern "system" fn __glGenRenderbuffers(
                    _n: GLsizei,
                    _renderbuffers: *mut GLuint,
                ) {
                    panic!("Unable to load glGenRenderbuffers")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGenRenderbuffers\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGenRenderbuffers
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles31",
                feature = "gles20",
                feature = "gles30",
                feature = "gles32",
            ))]
            glVertexAttrib3fv: unsafe {
                unsafe extern "system" fn __glVertexAttrib3fv(_index: GLuint, _v: *const GLfloat) {
                    panic!("Unable to load glVertexAttrib3fv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib3fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttrib3fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles10",
                feature = "gles32",
                feature = "gles31",
                feature = "gles20",
            ))]
            glTexParameterf: unsafe {
                unsafe extern "system" fn __glTexParameterf(
                    _target: GLenum,
                    _pname: GLenum,
                    _param: GLfloat,
                ) {
                    panic!("Unable to load glTexParameterf")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glTexParameterf\0");
                let val = _f(cname);
                if val.is_null() {
                    __glTexParameterf
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles31",
                feature = "gles32",
                feature = "gles30",
                feature = "gles20",
            ))]
            glBlendFuncSeparate: unsafe {
                unsafe extern "system" fn __glBlendFuncSeparate(
                    _sfactorRGB: GLenum,
                    _dfactorRGB: GLenum,
                    _sfactorAlpha: GLenum,
                    _dfactorAlpha: GLenum,
                ) {
                    panic!("Unable to load glBlendFuncSeparate")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBlendFuncSeparate\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBlendFuncSeparate
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32", feature = "gles30", feature = "gles31",))]
            glUniform4ui: unsafe {
                unsafe extern "system" fn __glUniform4ui(
                    _location: GLint,
                    _v0: GLuint,
                    _v1: GLuint,
                    _v2: GLuint,
                    _v3: GLuint,
                ) {
                    panic!("Unable to load glUniform4ui")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glUniform4ui\0");
                let val = _f(cname);
                if val.is_null() {
                    __glUniform4ui
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glVertexAttribIFormat: unsafe {
                unsafe extern "system" fn __glVertexAttribIFormat(
                    _attribindex: GLuint,
                    _size: GLint,
                    _type: GLenum,
                    _relativeoffset: GLuint,
                ) {
                    panic!("Unable to load glVertexAttribIFormat")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glVertexAttribIFormat\0");
                let val = _f(cname);
                if val.is_null() {
                    __glVertexAttribIFormat
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glClearBufferfv: unsafe {
                unsafe extern "system" fn __glClearBufferfv(
                    _buffer: GLenum,
                    _drawbuffer: GLint,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load glClearBufferfv")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glClearBufferfv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glClearBufferfv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glProgramUniform1i: unsafe {
                unsafe extern "system" fn __glProgramUniform1i(
                    _program: GLuint,
                    _location: GLint,
                    _v0: GLint,
                ) {
                    panic!("Unable to load glProgramUniform1i")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform1i\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform1i
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glProgramUniform1f: unsafe {
                unsafe extern "system" fn __glProgramUniform1f(
                    _program: GLuint,
                    _location: GLint,
                    _v0: GLfloat,
                ) {
                    panic!("Unable to load glProgramUniform1f")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform1f\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform1f
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glBlendFuncSeparatei: unsafe {
                unsafe extern "system" fn __glBlendFuncSeparatei(
                    _buf: GLuint,
                    _srcRGB: GLenum,
                    _dstRGB: GLenum,
                    _srcAlpha: GLenum,
                    _dstAlpha: GLenum,
                ) {
                    panic!("Unable to load glBlendFuncSeparatei")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBlendFuncSeparatei\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBlendFuncSeparatei
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glGetTexParameterIuiv: unsafe {
                unsafe extern "system" fn __glGetTexParameterIuiv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *mut GLuint,
                ) {
                    panic!("Unable to load glGetTexParameterIuiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetTexParameterIuiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetTexParameterIuiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles10",))]
            glLightx: unsafe {
                unsafe extern "system" fn __glLightx(
                    _light: GLenum,
                    _pname: GLenum,
                    _param: GLfixed,
                ) {
                    panic!("Unable to load glLightx")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glLightx\0");
                let val = _f(cname);
                if val.is_null() {
                    __glLightx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glObjectPtrLabel: unsafe {
                unsafe extern "system" fn __glObjectPtrLabel(
                    _ptr: *const std::os::raw::c_void,
                    _length: GLsizei,
                    _label: *const GLchar,
                ) {
                    panic!("Unable to load glObjectPtrLabel")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glObjectPtrLabel\0");
                let val = _f(cname);
                if val.is_null() {
                    __glObjectPtrLabel
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles10",
                feature = "gles31",
                feature = "gles20",
                feature = "gles30",
                feature = "gles32",
            ))]
            glCopyTexImage2D: unsafe {
                unsafe extern "system" fn __glCopyTexImage2D(
                    _target: GLenum,
                    _level: GLint,
                    _internalformat: GLenum,
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                    _border: GLint,
                ) {
                    panic!("Unable to load glCopyTexImage2D")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCopyTexImage2D\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCopyTexImage2D
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glProgramUniform2iv: unsafe {
                unsafe extern "system" fn __glProgramUniform2iv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLint,
                ) {
                    panic!("Unable to load glProgramUniform2iv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform2iv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform2iv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glIsTransformFeedback: unsafe {
                unsafe extern "system" fn __glIsTransformFeedback(_id: GLuint) -> GLboolean {
                    panic!("Unable to load glIsTransformFeedback")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glIsTransformFeedback\0");
                let val = _f(cname);
                if val.is_null() {
                    __glIsTransformFeedback
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles30",
                feature = "gles32",
                feature = "gles20",
                feature = "gles31",
            ))]
            glCheckFramebufferStatus: unsafe {
                unsafe extern "system" fn __glCheckFramebufferStatus(_target: GLenum) -> GLenum {
                    panic!("Unable to load glCheckFramebufferStatus")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glCheckFramebufferStatus\0");
                let val = _f(cname);
                if val.is_null() {
                    __glCheckFramebufferStatus
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glDrawBuffers: unsafe {
                unsafe extern "system" fn __glDrawBuffers(_n: GLsizei, _bufs: *const GLenum) {
                    panic!("Unable to load glDrawBuffers")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glDrawBuffers\0");
                let val = _f(cname);
                if val.is_null() {
                    __glDrawBuffers
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glProgramUniform4uiv: unsafe {
                unsafe extern "system" fn __glProgramUniform4uiv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLuint,
                ) {
                    panic!("Unable to load glProgramUniform4uiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform4uiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform4uiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(
                feature = "gles20",
                feature = "gles31",
                feature = "gles30",
                feature = "gles32",
            ))]
            glGetVertexAttribPointerv: unsafe {
                unsafe extern "system" fn __glGetVertexAttribPointerv(
                    _index: GLuint,
                    _pname: GLenum,
                    _pointer: *mut *mut std::os::raw::c_void,
                ) {
                    panic!("Unable to load glGetVertexAttribPointerv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetVertexAttribPointerv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetVertexAttribPointerv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32", feature = "gles30",))]
            glGetProgramBinary: unsafe {
                unsafe extern "system" fn __glGetProgramBinary(
                    _program: GLuint,
                    _bufSize: GLsizei,
                    _length: *mut GLsizei,
                    _binaryFormat: *mut GLenum,
                    _binary: *mut std::os::raw::c_void,
                ) {
                    panic!("Unable to load glGetProgramBinary")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetProgramBinary\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetProgramBinary
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles31", feature = "gles32",))]
            glProgramUniform3fv: unsafe {
                unsafe extern "system" fn __glProgramUniform3fv(
                    _program: GLuint,
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load glProgramUniform3fv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glProgramUniform3fv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glProgramUniform3fv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles30", feature = "gles31", feature = "gles32",))]
            glGetInternalformativ: unsafe {
                unsafe extern "system" fn __glGetInternalformativ(
                    _target: GLenum,
                    _internalformat: GLenum,
                    _pname: GLenum,
                    _count: GLsizei,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load glGetInternalformativ")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetInternalformativ\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetInternalformativ
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glBlendFunci: unsafe {
                unsafe extern "system" fn __glBlendFunci(_buf: GLuint, _src: GLenum, _dst: GLenum) {
                    panic!("Unable to load glBlendFunci")
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glBlendFunci\0");
                let val = _f(cname);
                if val.is_null() {
                    __glBlendFunci
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glGetSamplerParameterIuiv: unsafe {
                unsafe extern "system" fn __glGetSamplerParameterIuiv(
                    _sampler: GLuint,
                    _pname: GLenum,
                    _params: *mut GLuint,
                ) {
                    panic!("Unable to load glGetSamplerParameterIuiv")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glGetSamplerParameterIuiv\0");
                let val = _f(cname);
                if val.is_null() {
                    __glGetSamplerParameterIuiv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            #[cfg(any(feature = "gles32",))]
            glPrimitiveBoundingBox: unsafe {
                unsafe extern "system" fn __glPrimitiveBoundingBox(
                    _minX: GLfloat,
                    _minY: GLfloat,
                    _minZ: GLfloat,
                    _minW: GLfloat,
                    _maxX: GLfloat,
                    _maxY: GLfloat,
                    _maxZ: GLfloat,
                    _maxW: GLfloat,
                ) {
                    panic!("Unable to load glPrimitiveBoundingBox")
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"glPrimitiveBoundingBox\0");
                let val = _f(cname);
                if val.is_null() {
                    __glPrimitiveBoundingBox
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
