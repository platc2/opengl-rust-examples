use thiserror::Error;
use crate::window_options::WindowOptions;

#[derive(Debug, Error)]
pub enum PlatformError {
    #[error("Failed to initialise platform: {0}")]
    InitialisationError(String),

    #[error("Failed to create platform window: {0}")]
    WindowCreationError(String),
}

pub type Result<T> = ::std::result::Result<T, PlatformError>;

pub trait Platform {
    type Window;

    fn create_window(&mut self, window_options: WindowOptions) -> Result<Self::Window>;
}
