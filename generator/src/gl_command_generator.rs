use crate::context::{APICommand, APIName};
use crate::generator::build_function_block;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

fn generate_default_method_block<'a>(
    api: &'a APIName,
    cmd: &'a str,
    command: &'a APICommand,
) -> TokenStream {
    let api_name = format_ident!("{}", cmd);
    let (arg_block, return_block) =
        build_function_block(&command.proto, command.arguments.as_slice());
    let args: Vec<TokenStream> = command
        .arguments
        .iter()
        .map(|p| {
            let name = format_ident!("_{}", p.name.as_str());
            quote! { #name }
        })
        .collect();
    match return_block {
        None => {
            quote! {
                unsafe fn #api_name(&self,#arg_block) {
                     (self.entry().#api_name)(#(#args,)*)
                }
            }
        }
        Some(return_block) => {
            quote! {
                 unsafe fn #api_name(&self,#arg_block) -> #return_block{
                    (self.entry().#api_name)(#(#args,)*)
                }
            }
        }
    }
}

pub fn construct_method_block<'a>(
    api: &'a APIName,
    cmds: impl Iterator<Item = &'a str>,
    mut get_api_command: impl FnMut(&'a str) -> &'a APICommand,
) -> Vec<TokenStream> {
    let mut method_block: Vec<TokenStream> = vec![];
    for cmd in cmds {
        match cmd {
            "glUniform1fv" => method_block.push(quote! {
                unsafe fn glUniform1fv(&self, location: GLint, count: GLsizei, value: &[GLfloat]) {
                    assert!(count <= value.len() as GLsizei);
                    (self.entry().glUniform1fv)(location, count, value.as_ptr())
                }
            }),
            "glUniform2fv" => method_block.push(quote! {
                unsafe fn glUniform2fv(&self, location: GLint, count: GLsizei, value: &[GLfloat]) {
                    assert!((count * 2) <= value.len() as GLsizei);
                    (self.entry().glUniform2fv)(location, count, value.as_ptr())
                }
            }),
            "glUniform3fv" => method_block.push(quote! {
                unsafe fn glUniform3fv(&self, location: GLint, count: GLsizei, value: &[GLfloat]) {
                    assert!((count * 3) <= value.len() as GLsizei);
                    (self.entry().glUniform3fv)(location, count, value.as_ptr())
                }
            }),
            "glUniform4fv" => method_block.push(quote! {
                unsafe fn glUniform4fv(&self, location: GLint, count: GLsizei, value: &[GLfloat]) {
                    assert!((count * 4) <= value.len() as GLsizei);
                    (self.entry().glUniform4fv)(location, count, value.as_ptr())
                }
            }),
            "glUniform1iv" => method_block.push(quote! {
                unsafe fn glUniform1iv(&self, location: GLint, count: GLsizei, value: &[GLint]) {
                    assert!(count <= value.len() as GLsizei);
                    (self.entry().glUniform1iv)(location, count, value.as_ptr())
                }
            }),
            "glUniform2iv" => method_block.push(quote! {
                unsafe fn glUniform2iv(&self, location: GLint, count: GLsizei, value: &[GLint]) {
                    assert!((count * 2) <= value.len() as GLsizei);
                    (self.entry().glUniform2iv)(location, count, value.as_ptr())
                }
            }),
            "glUniform3iv" => method_block.push(quote! {
                unsafe fn glUniform3iv(&self, location: GLint, count: GLsizei, value: &[GLint]) {
                    assert!((count * 3) <= value.len() as GLsizei);
                    (self.entry().glUniform3iv)(location, count, value.as_ptr())
                }
            }),
            "glUniform4iv" => method_block.push(quote! {
                unsafe fn glUniform4iv(&self, location: GLint, count: GLsizei, value: &[GLint]) {
                    assert!((count * 4) <= value.len() as GLsizei);
                    (self.entry().glUniform4iv)(location, count, value.as_ptr())
                }
            }),
            "glUniform4uiv" => method_block.push(quote! {
                unsafe fn glUniform4uiv(&self, location: GLint, count: GLsizei, value: &[GLuint]) {
                    assert!((count * 4) <= value.len() as GLsizei);
                    (self.entry().glUniform4uiv)(location, count, value.as_ptr())
                }
            }),
            "glUniform3uiv" => method_block.push(quote! {
                unsafe fn glUniform3uiv(&self, location: GLint, count: GLsizei, value: &[GLuint]) {
                    assert!((count * 3) <= value.len() as GLsizei);
                    (self.entry().glUniform3uiv)(location, count, value.as_ptr())
                }
            }),
            "glUniform2uiv" => method_block.push(quote! {
                unsafe fn glUniform2uiv(&self, location: GLint, count: GLsizei, value: &[GLuint]) {
                    assert!((count * 2) <= value.len() as GLsizei);
                    (self.entry().glUniform2uiv)(location, count, value.as_ptr())
                }
            }),
            "glUniform1uiv" => method_block.push(quote! {
                unsafe fn glUniform1uiv(&self, location: GLint, count: GLsizei, value: &[GLuint]) {
                    assert!(count <= value.len() as GLsizei);
                    (self.entry().glUniform1uiv)(location, count, value.as_ptr())
                }
            }),
            "glUniformMatrix2fv" => method_block.push(quote! {
                unsafe fn glUniformMatrix2fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: &[GLfloat]) {
                    assert!((count * (2 * 2))  <= value.len() as GLsizei);
                    (self.entry().glUniformMatrix2fv)(location, count, transpose, value.as_ptr())
                }
            }),
            "glUniformMatrix3fv" => method_block.push(quote! {
                  unsafe fn glUniformMatrix3fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: &[GLfloat]) {
                    assert!((count * (3 * 3))  <= value.len() as GLsizei);
                    (self.entry().glUniformMatrix3fv)(location, count, transpose, value.as_ptr())
                }
            }),
            "glUniformMatrix4fv" => method_block.push(quote! {
                unsafe fn glUniformMatrix4fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: &[GLfloat]) {
                    assert!((count * (4 * 4))  <= value.len() as GLsizei);
                    (self.entry().glUniformMatrix4fv)(location, count, transpose, value.as_ptr())
                }
            }),
            "glUniformMatrix2x3fv" => method_block.push(quote! {
                unsafe fn glUniformMatrix2x3fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: &[GLfloat]) {
                    assert!((count * (2 * 3))  <= value.len() as GLsizei);
                    (self.entry().glUniformMatrix2x3fv)(location, count, transpose, value.as_ptr())
                }
            }),
            "glUniformMatrix3x2fv" => method_block.push(quote! {
                unsafe fn glUniformMatrix3x2fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: &[GLfloat]) {
                    assert!((count * (3 * 2))  <= value.len() as GLsizei);
                    (self.entry().glUniformMatrix3x2fv)(location, count, transpose, value.as_ptr())
                }
            }),
            "glUniformMatrix2x4fv" => method_block.push(quote! {
                unsafe fn glUniformMatrix2x4fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: &[GLfloat]) {
                    assert!((count * (2 * 4))  <= value.len() as GLsizei);
                    (self.entry().glUniformMatrix2x4fv)(location, count, transpose, value.as_ptr())
                }
            }),
            "glUniformMatrix4x2fv" => method_block.push(quote! {
                unsafe fn glUniformMatrix4x2fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: &[GLfloat]) {
                    assert!((count * (2 * 4))  <= value.len() as GLsizei);
                    (self.entry().glUniformMatrix4x2fv)(location, count, transpose, value.as_ptr())
                }
            }),
            "glUniformMatrix3x4fv" => method_block.push(quote! {
                unsafe fn glUniformMatrix3x4fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: &[GLfloat]) {
                    assert!((count * (3 * 4))  <= value.len() as GLsizei);
                    (self.entry().glUniformMatrix3x4fv)(location, count, transpose, value.as_ptr())
                }
            }),
            "glUniformMatrix4x3fv" => method_block.push(quote! {
                unsafe fn glUniformMatrix4x3fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: &[GLfloat]) {
                    assert!((count * (4 * 3))  <= value.len() as GLsizei);
                    (self.entry().glUniformMatrix4x3fv)(location, count, transpose, value.as_ptr())
                }
            }),
            "glProgramUniform1fv" => method_block.push(quote! {
                 unsafe fn glProgramUniform1fv(&self, program: GLuint, location: GLint, count: GLsizei, value: &[GLfloat]) {
                    assert!(count  <= value.len() as GLsizei);
                    (self.entry().glProgramUniform1fv)(program, location, count, value.as_ptr())
                }
            }),
            "glProgramUniform2fv" => method_block.push(quote! {
                 unsafe fn glProgramUniform2fv(&self, program: GLuint, location: GLint, count: GLsizei, value: &[GLfloat]) {
                    assert!((count * 2)  <= value.len() as GLsizei);
                    (self.entry().glProgramUniform2fv)(program, location, count, value.as_ptr())
                }
            }),
            "glProgramUniform3fv" => method_block.push(quote! {
                 unsafe fn glProgramUniform3fv(&self, program: GLuint, location: GLint, count: GLsizei, value: &[GLfloat]) {
                    assert!((count * 3)  <= value.len() as GLsizei);
                    (self.entry().glProgramUniform3fv)(program, location, count, value.as_ptr())
                }
            }),
            "glProgramUniform4fv" => method_block.push(quote! {
                 unsafe fn glProgramUniform4fv(&self, program: GLuint, location: GLint, count: GLsizei, value: &[GLfloat]) {
                    assert!((count * 4)  <= value.len() as GLsizei);
                    (self.entry().glProgramUniform4fv)(program, location, count, value.as_ptr())
                }
            }),
            "glProgramUniform1iv" => method_block.push(quote! {
                 unsafe fn glProgramUniform1iv(&self, program: GLuint, location: GLint, count: GLsizei, value: &[GLint]) {
                    assert!(count <= value.len() as GLsizei);
                    (self.entry().glProgramUniform1iv)(program, location, count, value.as_ptr())
                }
            }),
            "glProgramUniform2iv" => method_block.push(quote! {
                 unsafe fn glProgramUniform2iv(&self, program: GLuint, location: GLint, count: GLsizei, value: &[GLint]) {
                    assert!((count * 2)  <= value.len() as GLsizei);
                    (self.entry().glProgramUniform2iv)(program, location, count, value.as_ptr())
                }
            }),
            "glProgramUniform3iv" => method_block.push(quote! {
                 unsafe fn glProgramUniform3iv(&self, program: GLuint, location: GLint, count: GLsizei, value: &[GLint]) {
                    assert!((count * 3)  <= value.len() as GLsizei);
                    (self.entry().glProgramUniform3iv)(program, location, count, value.as_ptr())
                }
            }),
            "glProgramUniform4iv" => method_block.push(quote! {
                 unsafe fn glProgramUniform4iv(&self, program: GLuint, location: GLint, count: GLsizei, value: &[GLint]) {
                    assert!((count * 4)  <= value.len() as GLsizei);
                    (self.entry().glProgramUniform4iv)(program, location, count, value.as_ptr())
                }
            }),
            "glProgramUniform1uiv" => method_block.push(quote! {
                 unsafe fn glProgramUniform1uiv(&self, program: GLuint, location: GLint, count: GLsizei, value: &[GLuint]) {
                    assert!(count <= value.len() as GLsizei);
                    (self.entry().glProgramUniform1uiv)(program, location, count, value.as_ptr())
                }
            }),
            "glProgramUniform2uiv" => method_block.push(quote! {
                 unsafe fn glProgramUniform2uiv(&self, program: GLuint, location: GLint, count: GLsizei, value: &[GLuint]) {
                    assert!((count * 2)  <= value.len() as GLsizei);
                    (self.entry().glProgramUniform2uiv)(program, location, count, value.as_ptr())
                }
            }),
            "glProgramUniform3uiv" => method_block.push(quote! {
                 unsafe fn glProgramUniform3uiv(&self, program: GLuint, location: GLint, count: GLsizei, value: &[GLuint]) {
                    assert!((count * 3)  <= value.len() as GLsizei);
                    (self.entry().glProgramUniform3uiv)(program, location, count, value.as_ptr())
                }
            }),
            "glProgramUniform4uiv" => method_block.push(quote! {
                 unsafe fn glProgramUniform4uiv(&self, program: GLuint, location: GLint, count: GLsizei, value: &[GLuint]) {
                    assert!((count * 4)  <= value.len() as GLsizei);
                    (self.entry().glProgramUniform4uiv)(program, location, count, value.as_ptr())
                }
            }),
            "glProgramUniformMatrix2fv" => method_block.push(quote! {
                unsafe fn glProgramUniformMatrix2fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: &[GLfloat]) {
                    assert!((count * (2 * 2))  <= value.len() as GLsizei);
                    (self.entry().glProgramUniformMatrix2fv)(program, location, count, transpose, value.as_ptr())
                }
            }),
            "glProgramUniformMatrix3fv" =>  method_block.push(quote! {
                unsafe fn glProgramUniformMatrix3fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: &[GLfloat]) {
                    assert!((count * (3 * 3))  <= value.len() as GLsizei);
                    (self.entry().glProgramUniformMatrix3fv)(program, location, count, transpose, value.as_ptr())
                }
            }),
            "glProgramUniformMatrix4fv" => method_block.push(quote! {
                unsafe fn glProgramUniformMatrix4fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: &[GLfloat]) {
                    assert!((count * (4 * 4))  <= value.len() as GLsizei);
                    (self.entry().glProgramUniformMatrix4fv)(program, location, count, transpose, value.as_ptr())
                }
            }),
            "glProgramUniformMatrix2x3fv" => method_block.push(quote! {
                unsafe fn glProgramUniformMatrix2x3fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: &[GLfloat]) {
                    assert!((count * (2 * 3))  <= value.len() as GLsizei);
                    (self.entry().glProgramUniformMatrix2x3fv)(program, location, count, transpose, value.as_ptr())
                }
            }),
            "glProgramUniformMatrix3x2fv" => method_block.push(quote! {
                unsafe fn glProgramUniformMatrix3x2fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: &[GLfloat]) {
                    assert!((count * (3 * 2))  <= value.len() as GLsizei);
                    (self.entry().glProgramUniformMatrix3x2fv)(program, location, count, transpose, value.as_ptr())
                }
            }),
            "glProgramUniformMatrix2x4fv" => method_block.push(quote! {
                unsafe fn glProgramUniformMatrix2x4fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: &[GLfloat]) {
                    assert!((count * (2 * 4))  <= value.len() as GLsizei);
                    (self.entry().glProgramUniformMatrix2x4fv)(program, location, count, transpose, value.as_ptr())
                }
            }),
            "glProgramUniformMatrix4x2fv" => method_block.push(quote! {
                unsafe fn glProgramUniformMatrix4x2fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: &[GLfloat]) {
                    assert!((count * (4 * 2))  <= value.len() as GLsizei);
                    (self.entry().glProgramUniformMatrix4x2fv)(program, location, count, transpose, value.as_ptr())
                }
            }),
            "glProgramUniformMatrix3x4fv" => method_block.push(quote! {
                unsafe fn glProgramUniformMatrix3x4fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: &[GLfloat]) {
                    assert!((count * (3 * 4))  <= value.len() as GLsizei);
                    (self.entry().glProgramUniformMatrix3x4fv)(program, location, count, transpose, value.as_ptr())
                }
            }),
            "glProgramUniformMatrix4x3fv" => method_block.push(quote! {
                unsafe fn glProgramUniformMatrix4x3fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: &[GLfloat]) {
                    assert!((count * (4 * 3))  <= value.len() as GLsizei);
                    (self.entry().glProgramUniformMatrix4x3fv)(program, location, count, transpose, value.as_ptr())
                }
            }),
            "glCreateVertexArrays" => method_block.push(quote! {
                unsafe fn glCreateVertexArrays(&self, array: &mut [GLuint]) {
                    (self.entry().glCreateVertexArrays)(array.len() as GLsizei, array.as_mut_ptr())
                }
            }),
            "glCreateFramebuffers" => method_block.push(quote! {
                unsafe fn glCreateFramebuffers(&self, array: &mut [GLuint]) {
                    (self.entry().glCreateFramebuffers)(array.len() as GLsizei, array.as_mut_ptr())
                }
            }),
            "glCreateBuffers" => method_block.push(quote! {
                unsafe fn glCreateBuffers(&self, buffers: &mut [GLuint]) {
                    (self.entry().glCreateBuffers)(buffers.len() as GLsizei, buffers.as_mut_ptr())
                }
            }),
            "glCreateQueries" => method_block.push(quote! {
                unsafe fn glCreateQueries(&self, target: GLenum, array: &mut [GLuint]) {
                    (self.entry().glCreateQueries)(target, array.len() as GLsizei, array.as_mut_ptr())
                }
            }),
            "glGenTextures" => method_block.push(quote! {
                unsafe fn glGenTextures(&self, array: &mut [GLuint]) {
                    (self.entry().glGenTextures)(array.len() as GLsizei, array.as_mut_ptr())
                }
            }),
            "glGenFramebuffers" => method_block.push(quote! {
                unsafe fn glGenFramebuffers(&self, array: &mut [GLuint]) {
                    (self.entry().glGenFramebuffers)(array.len() as GLsizei, array.as_mut_ptr())
                }
            }),
            "glGenTransformFeedbacks" => method_block.push(quote! {
                unsafe fn glGenTransformFeedbacks(&self, array: &mut [GLuint]) {
                    (self.entry().glGenTransformFeedbacks)(array.len() as GLsizei, array.as_mut_ptr())
                }
            }),
            "glGenQueries" => method_block.push(quote! {
                unsafe fn glGenQueries(&self, array: &mut [GLuint]) {
                    (self.entry().glGenQueries)(array.len() as GLsizei, array.as_mut_ptr())
                }
            }),
            "glGenVertexArrays" => method_block.push(quote! {
                unsafe fn glGenVertexArrays(&self, array: &mut [GLuint]) {
                    (self.entry().glGenVertexArrays)(array.len() as GLsizei, array.as_mut_ptr())
                }
            }),
            "glGenProgramPipelines" => method_block.push(quote! {
                unsafe fn glGenProgramPipelines(&self, array: &mut [GLuint]) {
                    (self.entry().glGenProgramPipelines)(array.len() as GLsizei, array.as_mut_ptr())
                }
            }),
            "glGenRenderbuffers" => method_block.push(quote! {
                unsafe fn glGenRenderbuffers(&self, renderbuffers: &mut [GLuint]) {
                    (self.entry().glGenRenderbuffers)(renderbuffers.len() as GLsizei, renderbuffers.as_mut_ptr())
                }
            }),
            "glGenSamplers" => method_block.push(quote! {
                unsafe fn glGenSamplers(&self, samplers: &mut [GLuint]) {
                    (self.entry().glGenSamplers)(samplers.len() as GLsizei, samplers.as_mut_ptr())
                }
            }),
            "glDeleteFramebuffers" => method_block.push(quote! {
                unsafe fn glDeleteFramebuffers(&self, array: &[GLuint]) {
                    (self.entry().glDeleteFramebuffers)(array.len() as GLsizei, array.as_ptr())
                }
            }),
            "glDeleteQueries" => method_block.push(quote! {
                unsafe fn glDeleteQueries(&self, array: &[GLuint]) {
                    (self.entry().glDeleteQueries)(array.len() as GLsizei, array.as_ptr())
                }
            }),
            "glDeleteBuffers" => method_block.push(quote! {
                unsafe fn glDeleteBuffers(&self, array: &[GLuint]) {
                    (self.entry().glDeleteBuffers)(array.len() as GLsizei, array.as_ptr())
                }
            }),
            "glDeleteRenderbuffers" => method_block.push(quote! {
                unsafe fn glDeleteRenderbuffers(&self, array: &[GLuint]) {
                    (self.entry().glDeleteRenderbuffers)(array.len() as GLsizei, array.as_ptr())
                }
            }),
            "glGetUniformfv" => method_block.push(quote! {
                unsafe fn glGetUniformfv(&self, program: GLuint, location: GLint, params: &mut [GLfloat]) {
                    (self.entry().glGetUniformfv)(program,  location, params.as_mut_ptr())
                }
            }),
            "glGetUniformiv" =>  method_block.push(quote! {
                unsafe fn glGetUniformiv(&self, program: GLuint, location: GLint, params: &mut [GLint]) {
                    (self.entry().glGetUniformiv)(program,  location, params.as_mut_ptr())
                }
            }),
            "glGetUniformuiv" =>  method_block.push(quote! {
                unsafe fn glGetUniformuiv(&self, program: GLuint, location: GLint, params: &mut [GLuint]) {
                    (self.entry().glGetUniformuiv)(program,  location, params.as_mut_ptr())
                }
            }),
            "glGetUniformdv" =>  method_block.push(quote! {
                unsafe fn glGetUniformdv(&self, program: GLuint, location: GLint, params: &mut [GLdouble]) {
                    (self.entry().glGetUniformdv)(program,  location, params.as_mut_ptr())
                }
            }),
            "glGetnUniformfv" =>  method_block.push(quote! {
                unsafe fn glGetnUniformfv(&self, program: GLuint, location: GLint, params: &mut [GLfloat]) {
                    (self.entry().glGetnUniformfv)(program,  location, ((params.len() * mem::size_of::<GLfloat>()) as GLsizei), params.as_mut_ptr())
                }
            }),
            "glGetnUniformiv" =>  method_block.push(quote! {
                unsafe fn glGetnUniformiv(&self, program: GLuint, location: GLint, params: &mut [GLint]) {
                    (self.entry().glGetnUniformiv)(program,  location, ((params.len() * mem::size_of::<GLint>()) as GLsizei), params.as_mut_ptr())
                }
            }),
            "glGetnUniformuiv" =>  method_block.push(quote! {
                unsafe fn glGetnUniformuiv(&self, program: GLuint, location: GLint, params: &mut [GLuint]) {
                    (self.entry().glGetnUniformuiv)(program,  location, ((params.len() * mem::size_of::<GLuint>()) as GLsizei), params.as_mut_ptr())
                }
            }),
            "glGetnUniformdv" =>  method_block.push(quote! {
                unsafe fn glGetnUniformdv(&self, program: GLuint, location: GLint, params: &mut [GLdouble]) {
                    (self.entry().glGetnUniformdv)(program,  location, ((params.len() * mem::size_of::<GLdouble>()) as GLsizei), params.as_mut_ptr())
                }
            }),
            _ => {
                method_block.push(generate_default_method_block(api, cmd, get_api_command(cmd)))
            }
        }
    }

    method_block
}
