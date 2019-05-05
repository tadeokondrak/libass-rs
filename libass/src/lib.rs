#[macro_use]
mod macros {
    macro_rules! cstring {
        ($i:expr) => {
            CString::new($i).expect("no null characters are allowed")
        };
    }
}

mod library;
pub use crate::library::*;

mod track;
pub use crate::track::*;

mod renderer;
pub use crate::renderer::*;

mod image;
pub use crate::image::*;
