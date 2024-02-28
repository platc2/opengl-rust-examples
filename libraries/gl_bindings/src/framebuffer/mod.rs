pub use attachment::*;
pub use framebuffer_id::*;
pub use framebuffer_status::*;
pub use framebuffer_target::*;
pub use renderbuffer_id::*;
pub use renderbuffer_target::*;

use crate::{gl, gl::RawHandle};
use crate::image_format::ImageFormat;
use crate::texture::{TextureId, TextureTarget};

mod framebuffer_id;
mod framebuffer_target;
mod renderbuffer_id;
mod renderbuffer_target;
mod framebuffer_status;
mod attachment;

pub fn bind_framebuffer(target: FramebufferTarget, framebuffer: FramebufferId) {
    unsafe { gl::BindFramebuffer(target.raw_handle(), framebuffer.raw_handle()); }
}

pub fn bind_renderbuffer(target: RenderbufferTarget, renderbuffer: RenderbufferId) {
    unsafe { gl::BindRenderbuffer(target.raw_handle(), renderbuffer.raw_handle()); }
}

/*
pub fn blit_framebuffer(...)
*/

/*
#[cfg(feature = "GL45")]
pub fn blit_named_framebuffer(...)
*/

#[must_use]
pub fn check_framebuffer_status(target: FramebufferTarget) -> FramebufferStatus {
    FramebufferStatus(unsafe { gl::CheckFramebufferStatus(target.raw_handle()) })
}

#[must_use]
#[cfg(feature = "GL45")]
pub fn check_named_framebuffer_status(framebuffer: FramebufferId,
                                      target: FramebufferTarget) -> FramebufferStatus {
    let status = unsafe {
        gl::CheckNamedFramebufferStatus(framebuffer.raw_handle(), target.raw_handle())
    };
    FramebufferStatus(status)
}

#[must_use]
#[cfg(feature = "GL45")]
pub fn create_framebuffers(count: usize) -> Vec<FramebufferId> {
    let mut framebuffer_ids = Vec::with_capacity(count);

    unsafe {
        let ptr = framebuffer_ids.spare_capacity_mut().as_mut_ptr().cast();
        gl::CreateFramebuffers(count as _, ptr);
        framebuffer_ids.set_len(count);
    }

    framebuffer_ids
}

#[must_use]
#[cfg(feature = "GL45")]
pub fn create_framebuffer() -> FramebufferId {
    let mut id = 0;
    unsafe { gl::CreateFramebuffers(1, &mut id); }
    FramebufferId(id)
}

#[must_use]
#[cfg(feature = "GL45")]
pub fn create_renderbuffers(count: usize) -> Vec<RenderbufferId> {
    let mut renderbuffer_ids = Vec::with_capacity(count);

    unsafe {
        let ptr = renderbuffer_ids.spare_capacity_mut().as_mut_ptr().cast();
        gl::CreateRenderbuffers(count as _, ptr);
        renderbuffer_ids.set_len(count);
    }

    renderbuffer_ids
}

#[must_use]
#[cfg(feature = "GL45")]
pub fn create_renderbuffer() -> RenderbufferId {
    let mut id = 0;
    unsafe { gl::CreateRenderbuffers(1, &mut id); }
    RenderbufferId(id)
}

pub fn delete_framebuffers(framebuffers: &mut [FramebufferId]) {
    unsafe { gl::DeleteFramebuffers(framebuffers.len() as _, framebuffers.as_ptr().cast()); }
    for framebuffer in framebuffers {
        *framebuffer = FramebufferId::DEFAULT_FRAMEBUFFER;
    }
}

pub fn delete_framebuffer(framebuffer: &mut FramebufferId) {
    delete_framebuffers(core::slice::from_mut(framebuffer));
}

pub fn delete_renderbuffers(renderbuffers: &mut [RenderbufferId]) {
    unsafe { gl::DeleteRenderbuffers(renderbuffers.len() as _, renderbuffers.as_ptr().cast()); }
    for renderbuffer in renderbuffers {
        *renderbuffer = RenderbufferId::NO_RENDERBUFFER;
    }
}

pub fn delete_renderbuffer(renderbuffer: &mut RenderbufferId) {
    delete_renderbuffers(core::slice::from_mut(renderbuffer));
}

/*
pub fn draw_buffers(count: usize, buffers: &[unimplemented!()]) {
}
*/

/*
#[cfg(feature = "GL45")]
pub fn named_framebuffer_draw_buffers(framebuffer: FramebufferId, count: usize, ...) {
}
*/

/*
#[cfg(feature = "GL43")]
pub fn framebuffer_parameteri()
*/

/*
#[cfg(feature = "GL45")]
pub fn named_framebuffer_parameteri()
*/

pub fn framebuffer_renderbuffer(framebuffer_target: FramebufferTarget,
                                attachment: Attachment,
                                renderbuffer_target: RenderbufferTarget,
                                renderbuffer: RenderbufferId) {
    unsafe {
        gl::FramebufferRenderbuffer(
            framebuffer_target.raw_handle(),
            attachment.raw_handle(),
            renderbuffer_target.raw_handle(),
            renderbuffer.raw_handle());
    }
}

#[cfg(feature = "GL45")]
pub fn named_framebuffer_renderbuffer(framebuffer: FramebufferId,
                                      attachment: Attachment,
                                      renderbuffer_target: RenderbufferTarget,
                                      renderbuffer: RenderbufferId) {
    unsafe {
        gl::NamedFramebufferRenderbuffer(
            framebuffer.raw_handle(),
            attachment.raw_handle(),
            renderbuffer_target.raw_handle(),
            renderbuffer.raw_handle(),
        );
    }
}

pub fn framebuffer_texture(target: FramebufferTarget,
                           attachment: Attachment,
                           texture: TextureId, level: usize) {
    unsafe {
        gl::FramebufferTexture(
            target.raw_handle(),
            attachment.raw_handle(),
            texture.raw_handle(),
            level as _);
    }
}

pub fn framebuffer_texture_1d(target: FramebufferTarget,
                              attachment: Attachment,
                              texture_target: TextureTarget,
                              texture: TextureId,
                              level: usize) {
    unsafe {
        gl::FramebufferTexture1D(
            target.raw_handle(),
            attachment.raw_handle(),
            texture_target.raw_handle(),
            texture.raw_handle(),
            level as _,
        );
    }
}

pub fn framebuffer_texture_2d(target: FramebufferTarget,
                              attachment: Attachment,
                              texture_target: TextureTarget,
                              texture: TextureId,
                              level: usize) {
    unsafe {
        gl::FramebufferTexture2D(
            target.raw_handle(),
            attachment.raw_handle(),
            texture_target.raw_handle(),
            texture.raw_handle(),
            level as _,
        );
    }
}

pub fn framebuffer_texture_3d(target: FramebufferTarget,
                              attachment: Attachment,
                              texture_target: TextureTarget,
                              texture: TextureId,
                              level: usize,
                              layer: usize) {
    unsafe {
        gl::FramebufferTexture3D(
            target.raw_handle(),
            attachment.raw_handle(),
            texture_target.raw_handle(),
            texture.raw_handle(),
            level as _,
            layer as _,
        );
    }
}

#[cfg(feature = "GL45")]
pub fn named_framebuffer_texture(framebuffer: FramebufferId,
                                 attachment: Attachment,
                                 texture: TextureId,
                                 level: usize) {
    unsafe {
        gl::NamedFramebufferTexture(
            framebuffer.raw_handle(),
            attachment.raw_handle(),
            texture.raw_handle(),
            level as _,
        );
    }
}

pub fn generate_mipmap(target: TextureTarget) {
    unsafe { gl::GenerateMipmap(target.raw_handle()); }
}

#[cfg(feature = "GL45")]
pub fn generate_texture_mipmap(texture: TextureId) {
    unsafe { gl::GenerateTextureMipmap(texture.raw_handle()); }
}






pub fn renderbuffer_storage(target: RenderbufferTarget, internal_format: ImageFormat, width: usize, height: usize) {
    unsafe { gl::RenderbufferStorage(target.raw_handle(), internal_format.raw_handle(), width as _, height as _); }
}
