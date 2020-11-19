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
        write!(f, "Libass Error")
    }
}
impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! err_if_null {
    ($e:expr) => {
        if $e.is_null() {
            return Err(crate::Error);
        }
    };
}
