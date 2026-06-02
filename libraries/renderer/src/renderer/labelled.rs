pub trait Labelled {

    fn set_label(&mut self, label: &str) {
        unsafe { gl::sys::ObjectLabel(self.identifier(), self.name(), label.len() as _, label.as_ptr().cast()); }
    }

    fn identifier(&self) -> gl::sys::types::GLenum;

    fn name(&self) -> gl::sys::types::GLuint;
}
