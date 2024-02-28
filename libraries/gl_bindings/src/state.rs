use gl_bindings_raw_handle_derive::RawHandle;

use crate::{define_gl_constants, gl, gl::RawHandle};

pub fn viewport(pos: (usize, usize), size: (usize, usize)) {
    unsafe { gl::Viewport(pos.0 as _, pos.1 as _, size.0 as _, size.1 as _); }
}

pub fn scissor(pos: (usize, usize), size: (usize, usize)) {
    unsafe { gl::Scissor(pos.0 as _, pos.1 as _, size.0 as _, size.1 as _); }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct PolygonFace(gl::GLenum);

impl PolygonFace {
    define_gl_constants!(PolygonFace ::
        FRONT,
        BACK,
        FRONT_AND_BACK
    );
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct PolygonMode(gl::GLenum);

impl PolygonMode {
    define_gl_constants!(PolygonMode ::
        POINT,
        LINE,
        FILL
    );
}

pub fn polygon_mode(polygon_face: PolygonFace, polygon_mode: PolygonMode) {
    unsafe { gl::PolygonMode(polygon_face.raw_handle(), polygon_mode.raw_handle()); }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct BlendEquation(gl::GLenum);

impl BlendEquation {
    define_gl_constants!(BlendEquation ::
        FUNC_ADD,
        FUNC_SUBTRACT,
        FUNC_REVERSE_SUBTRACT
    );
}

pub fn blend_equation(blend_equation: BlendEquation) {
    unsafe { gl::BlendEquation(blend_equation.raw_handle()); }
}

pub fn blend_equation_separate(blend_equation_rgb: BlendEquation, blend_equation_alpha: BlendEquation) {
    unsafe { gl::BlendEquationSeparate(blend_equation_rgb.raw_handle(), blend_equation_alpha.raw_handle()); }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct BlendSourceFuncFactor(gl::GLenum);

impl BlendSourceFuncFactor {
    define_gl_constants!(BlendSourceFuncFactor ::
        ZERO,
        ONE,
        SRC_COLOR,
        ONE_MINUS_SRC_COLOR,
        DST_COLOR,
        ONE_MINUS_DST_COLOR,
        SRC_ALPHA,
        ONE_MINUS_SRC_ALPHA,
        DST_ALPHA,
        ONE_MINUS_DST_ALPHA,
        CONSTANT_COLOR,
        ONE_MINUS_CONSTANT_COLOR,
        CONSTANT_ALPHA,
        ONE_MINUS_CONSTANT_ALPHA
    );
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct BlendDestinationFuncFactor(gl::GLenum);

impl BlendDestinationFuncFactor {
    define_gl_constants!(BlendDestinationFuncFactor ::
        ZERO,
        ONE,
        SRC_COLOR,
        ONE_MINUS_SRC_COLOR,
        DST_COLOR,
        ONE_MINUS_DST_COLOR,
        SRC_ALPHA,
        ONE_MINUS_SRC_ALPHA,
        DST_ALPHA,
        ONE_MINUS_DST_ALPHA,
        CONSTANT_COLOR,
        ONE_MINUS_CONSTANT_COLOR,
        CONSTANT_ALPHA,
        ONE_MINUS_CONSTANT_ALPHA,
        SRC_ALPHA_SATURATE
    );
}

pub fn blend_func(blend_func_factor_source: BlendSourceFuncFactor,
                  blend_func_factor_destination: BlendDestinationFuncFactor) {
    unsafe { gl::BlendFunc(blend_func_factor_source.raw_handle(), blend_func_factor_destination.raw_handle()); }
}

pub fn blend_func_separate(
    blend_func_factor_source_rgb: BlendSourceFuncFactor,
    blend_func_factor_source_alpha: BlendSourceFuncFactor,
    blend_func_factor_destination_rgb: BlendDestinationFuncFactor,
    blend_func_factor_destination_alpha: BlendDestinationFuncFactor)
{
    unsafe {
        gl::BlendFuncSeparate(
            blend_func_factor_source_rgb.raw_handle(),
            blend_func_factor_source_alpha.raw_handle(),
            blend_func_factor_destination_rgb.raw_handle(),
            blend_func_factor_destination_alpha.raw_handle());
    }
}
