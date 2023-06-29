mod library;
pub use crate::library::*;

mod track;
pub use crate::track::*;

mod renderer;
pub use crate::renderer::*;

mod image;
pub use crate::image::*;

mod style;
pub use crate::style::*;

#[derive(Debug)]
pub struct Error;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("libass error")
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
