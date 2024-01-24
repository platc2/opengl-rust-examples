use nalgebra_glm as glm;

pub use orthographic_camera::OrthographicCamera;
pub use perspective_camera::PerspectiveCamera;

mod orthographic_camera;
mod perspective_camera;

pub trait Camera {
    #[must_use]
    fn projection(&self) -> &glm::Mat4;

    #[must_use]
    fn view(&self) -> &glm::Mat4;
}
