use gl_bindings_raw_handle_derive::RawHandle;

use crate::{gl, gl::RawHandle};
use crate::shader::ShaderId;

#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct ProgramId(gl::GLuint);

impl ProgramId {
    pub const NO_PROGRAM: ProgramId = ProgramId(0);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct UniformLocation(gl::GLint);

impl UniformLocation {
    pub fn fixed(location: usize) -> Self {
        Self(location as _)
    }
}

#[must_use]
pub fn create_program() -> ProgramId {
    let id = unsafe { gl::CreateProgram() };
    ProgramId(id)
}

pub fn delete_program(program_id: &mut ProgramId) {
    unsafe { gl::DeleteProgram(program_id.raw_handle()); }
    program_id.0 = ProgramId::NO_PROGRAM.0;
}

pub fn attach_shader(program_id: ProgramId, shader_id: ShaderId) {
    unsafe { gl::AttachShader(program_id.raw_handle(), shader_id.raw_handle()); }
}

pub fn detach_shader(program_id: ProgramId, shader_id: ShaderId) {
    unsafe { gl::DetachShader(program_id.raw_handle(), shader_id.raw_handle()); }
}

pub fn link_program(program_id: ProgramId) {
    unsafe { gl::LinkProgram(program_id.raw_handle()); }
}

#[must_use]
pub fn program_link_status(program_id: ProgramId) -> bool {
    let mut success: gl::GLint = 0;
    unsafe { gl::GetProgramiv(program_id.raw_handle(), gl::LINK_STATUS, &mut success); }
    success != 0
}

#[must_use]
pub fn program_info_log(program_id: ProgramId) -> Option<String> {
    let mut log_len: gl::GLint = 0;
    unsafe { gl::GetProgramiv(program_id.raw_handle(), gl::INFO_LOG_LENGTH, &mut log_len); }

    if log_len == 0 {
        None
    } else {
        // log_len includes null termination character, which we do not require
        let mut info_log_buffer = Vec::with_capacity(log_len as _);
        let mut written_length: gl::GLsizei = 0;
        let info_log_buffer_ptr = info_log_buffer.spare_capacity_mut().as_ptr() as _;
        let info_log = unsafe {
            gl::GetProgramInfoLog(program_id.raw_handle(), log_len, &mut written_length, info_log_buffer_ptr);
            info_log_buffer.set_len(std::cmp::max(written_length, log_len) as _);
            let written_length = written_length as _;
            String::from_raw_parts(info_log_buffer.as_mut_ptr(), written_length, written_length)
        };

        Some(info_log)
    }
}

pub fn use_program(program_id: ProgramId) {
    unsafe { gl::UseProgram(program_id.raw_handle()); }
}

#[must_use]
pub fn uniform_location<T: Into<String>>(program_id: ProgramId, name: T) -> UniformLocation {
    let name = std::ffi::CString::new(name.into())
        .expect("Null character found in uniform name!");
    let id = unsafe { gl::GetUniformLocation(program_id.raw_handle(), name.as_ptr()) };
    UniformLocation(id)
}

pub fn uniform_1f(uniform_location: UniformLocation, a: f32) {
    unsafe { gl::Uniform1f(uniform_location.raw_handle(), a); }
}

pub fn uniform_2f(uniform_location: UniformLocation, a: f32, b: f32) {
    unsafe { gl::Uniform2f(uniform_location.raw_handle(), a, b); }
}

pub fn uniform_3f(uniform_location: UniformLocation, a: f32, b: f32, c: f32) {
    unsafe { gl::Uniform3f(uniform_location.raw_handle(), a, b, c); }
}

pub fn uniform_4f(uniform_location: UniformLocation, a: f32, b: f32, c: f32, d: f32) {
    unsafe { gl::Uniform4f(uniform_location.raw_handle(), a, b, c, d); }
}

pub fn uniform_1i(uniform_location: UniformLocation, a: i32) {
    unsafe { gl::Uniform1i(uniform_location.raw_handle(), a); }
}

pub fn uniform_2i(uniform_location: UniformLocation, a: i32, b: i32) {
    unsafe { gl::Uniform2i(uniform_location.raw_handle(), a, b); }
}

pub fn uniform_3i(uniform_location: UniformLocation, a: i32, b: i32, c: i32) {
    unsafe { gl::Uniform3i(uniform_location.raw_handle(), a, b, c); }
}

pub fn uniform_4i(uniform_location: UniformLocation, a: i32, b: i32, c: i32, d: i32) {
    unsafe { gl::Uniform4i(uniform_location.raw_handle(), a, b, c, d); }
}

pub fn uniform_1ui(uniform_location: UniformLocation, a: u32) {
    unsafe { gl::Uniform1ui(uniform_location.raw_handle(), a); }
}

pub fn uniform_2ui(uniform_location: UniformLocation, a: u32, b: u32) {
    unsafe { gl::Uniform2ui(uniform_location.raw_handle(), a, b); }
}

pub fn uniform_3ui(uniform_location: UniformLocation, a: u32, b: u32, c: u32) {
    unsafe { gl::Uniform3ui(uniform_location.raw_handle(), a, b, c); }
}

pub fn uniform_4ui(uniform_location: UniformLocation, a: u32, b: u32, c: u32, d: u32) {
    unsafe { gl::Uniform4ui(uniform_location.raw_handle(), a, b, c, d); }
}

pub fn uniform_1fv(uniform_location: UniformLocation, values: &[f32]) {
    let length = values.len();
    unsafe { gl::Uniform1fv(uniform_location.raw_handle(), length as _, values.as_ptr()); }
}

pub fn uniform_2fv(uniform_location: UniformLocation, values: &[f32]) {
    let length = values.len() / 2;
    unsafe { gl::Uniform2fv(uniform_location.raw_handle(), length as _, values.as_ptr()); }
}

pub fn uniform_3fv(uniform_location: UniformLocation, values: &[f32]) {
    let length = values.len() / 3;
    unsafe { gl::Uniform3fv(uniform_location.raw_handle(), length as _, values.as_ptr()); }
}

pub fn uniform_4fv(uniform_location: UniformLocation, values: &[f32]) {
    let length = values.len() / 4;
    unsafe { gl::Uniform4fv(uniform_location.raw_handle(), length as _, values.as_ptr()); }
}

pub fn uniform_1iv(uniform_location: UniformLocation, values: &[i32]) {
    let length = values.len();
    unsafe { gl::Uniform1iv(uniform_location.raw_handle(), length as _, values.as_ptr()); }
}

pub fn uniform_2iv(uniform_location: UniformLocation, values: &[i32]) {
    let length = values.len() / 2;
    unsafe { gl::Uniform2iv(uniform_location.raw_handle(), length as _, values.as_ptr()); }
}

pub fn uniform_3iv(uniform_location: UniformLocation, values: &[i32]) {
    let length = values.len() / 3;
    unsafe { gl::Uniform3iv(uniform_location.raw_handle(), length as _, values.as_ptr()); }
}

pub fn uniform_4iv(uniform_location: UniformLocation, values: &[i32]) {
    let length = values.len() / 4;
    unsafe { gl::Uniform4iv(uniform_location.raw_handle(), length as _, values.as_ptr()); }
}

pub fn uniform_1uiv(uniform_location: UniformLocation, values: &[u32]) {
    let length = values.len();
    unsafe { gl::Uniform1uiv(uniform_location.raw_handle(), length as _, values.as_ptr()); }
}

pub fn uniform_2uiv(uniform_location: UniformLocation, values: &[u32]) {
    let length = values.len() / 2;
    unsafe { gl::Uniform2uiv(uniform_location.raw_handle(), length as _, values.as_ptr()); }
}

pub fn uniform_3uiv(uniform_location: UniformLocation, values: &[u32]) {
    let length = values.len() / 3;
    unsafe { gl::Uniform3uiv(uniform_location.raw_handle(), length as _, values.as_ptr()); }
}

pub fn uniform_4uiv(uniform_location: UniformLocation, values: &[u32]) {
    let length = values.len() / 4;
    unsafe { gl::Uniform4uiv(uniform_location.raw_handle(), length as _, values.as_ptr()); }
}

pub fn uniform_matrix_2fv(uniform_location: UniformLocation, transpose: bool, values: &[f32]) {
    let length = values.len() / (2 * 2);
    unsafe { gl::UniformMatrix2fv(uniform_location.raw_handle(), length as _, transpose as _, values.as_ptr()); }
}

pub fn uniform_matrix_3fv(uniform_location: UniformLocation, transpose: bool, values: &[f32]) {
    let length = values.len() / (3 * 3);
    unsafe { gl::UniformMatrix3fv(uniform_location.raw_handle(), length as _, transpose as _, values.as_ptr()); }
}

pub fn uniform_matrix_4fv(uniform_location: UniformLocation, transpose: bool, values: &[f32]) {
    let length = values.len() / (4 * 4);
    unsafe { gl::UniformMatrix4fv(uniform_location.raw_handle(), length as _, transpose as _, values.as_ptr()); }
}

pub fn uniform_matrix_2x3(uniform_location: UniformLocation, transpose: bool, values: &[f32]) {
    let length = values.len() / (2 * 3);
    unsafe { gl::UniformMatrix2x3fv(uniform_location.raw_handle(), length as _, transpose as _, values.as_ptr()); }
}

pub fn uniform_matrix_3x2(uniform_location: UniformLocation, transpose: bool, values: &[f32]) {
    let length = values.len() / (3 * 2);
    unsafe { gl::UniformMatrix3x2fv(uniform_location.raw_handle(), length as _, transpose as _, values.as_ptr()); }
}

pub fn uniform_matrix_2x4(uniform_location: UniformLocation, transpose: bool, values: &[f32]) {
    let length = values.len() / (2 * 4);
    unsafe { gl::UniformMatrix2x4fv(uniform_location.raw_handle(), length as _, transpose as _, values.as_ptr()); }
}

pub fn uniform_matrix_4x2(uniform_location: UniformLocation, transpose: bool, values: &[f32]) {
    let length = values.len() / (4 * 2);
    unsafe { gl::UniformMatrix4x2fv(uniform_location.raw_handle(), length as _, transpose as _, values.as_ptr()); }
}

pub fn uniform_matrix_3x4(uniform_location: UniformLocation, transpose: bool, values: &[f32]) {
    let length = values.len() / (3 * 4);
    unsafe { gl::UniformMatrix3x4fv(uniform_location.raw_handle(), length as _, transpose as _, values.as_ptr()); }
}

pub fn uniform_matrix_4x3(uniform_location: UniformLocation, transpose: bool, values: &[f32]) {
    let length = values.len() / (4 * 3);
    unsafe { gl::UniformMatrix4x3fv(uniform_location.raw_handle(), length as _, transpose as _, values.as_ptr()); }
}
