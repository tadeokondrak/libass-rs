use std::ffi::{c_char, c_void, CString};
use std::os::raw::c_int;
use std::ptr;
use std::ptr::NonNull;
use std::slice;

use libass_sys as ffi;

use crate::renderer::Renderer;
use crate::track::Track;
use crate::Result;
use std::rc::Rc;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DefaultFontProvider {
    None,
    Autodetect,
    CoreText,
    Fontconfig,
    DirectWrite,
}

pub fn version() -> i32 {
    unsafe { ffi::ass_library_version() }
}

struct RawLibrary {
    handle: NonNull<ffi::ass_library>,
}

impl RawLibrary {
    pub fn new() -> Result<Self> {
        let lib = unsafe { ffi::ass_library_init() };

        Ok(RawLibrary {
            handle: NonNull::new(lib).ok_or(crate::Error)?,
        })
    }

    pub fn as_ptr(&self) -> *mut ffi::ass_library {
        self.handle.as_ptr()
    }
}

impl Drop for RawLibrary {
    fn drop(&mut self) {
        unsafe { ffi::ass_library_done(self.handle.as_ptr()) }
    }
}

pub struct Library {
    raw: Rc<RawLibrary>,
}

impl Library {
    pub fn new() -> Result<Self> {
        RawLibrary::new().map(|raw| Self { raw: Rc::new(raw) })
    }

    #[doc(alias = "ass_set_fonts_dir")]
    pub fn set_fonts_dir(&self, fonts_dir: Option<&str>) {
        match fonts_dir {
            Some(fonts_dir) => {
                let fonts_dir = CString::new(fonts_dir).unwrap();
                unsafe {
                    ffi::ass_set_fonts_dir(self.raw.as_ptr(), fonts_dir.as_ptr());
                }
            }
            None => unsafe {
                ffi::ass_set_fonts_dir(self.raw.as_ptr(), ptr::null());
            },
        }
    }

    pub fn set_extract_fonts(&self, extract: bool) {
        unsafe { ffi::ass_set_extract_fonts(self.raw.as_ptr(), extract as c_int) }
    }

    #[doc(alias = "ass_set_style_overrides")]
    pub fn set_style_overrides<'a, I>(&self, list: I)
    where
        I: IntoIterator<Item = &'a str>,
    {
        let c_strings = list
            .into_iter()
            .map(|s| CString::new(s).unwrap())
            .collect::<Vec<CString>>();
        let c_strs = c_strings
            .iter()
            .map(|c_string| c_string.as_ptr().cast_mut())
            .collect::<Vec<*mut c_char>>();
        unsafe {
            ffi::ass_set_style_overrides(self.raw.as_ptr(), c_strs.as_slice().as_ptr().cast_mut())
        };
    }

    pub fn add_font(&self, name: &str, data: &[u8]) {
        let name = CString::new(name).unwrap();
        unsafe {
            ffi::ass_add_font(
                self.raw.as_ptr(),
                name.as_ptr() as *mut _,
                data.as_ptr() as *mut _,
                data.len() as c_int,
            )
        }
    }

    pub fn clear_fonts(&self) {
        unsafe { ffi::ass_clear_fonts(self.raw.as_ptr()) }
    }

    #[doc(alias = "ass_get_available_font_providers")]
    pub fn available_font_providers(&self) -> Vec<DefaultFontProvider> {
        let mut ptr: *mut ffi::ASS_DefaultFontProvider = ptr::null_mut();
        let mut size: usize = 0;

        unsafe {
            ffi::ass_get_available_font_providers(self.raw.as_ptr(), &mut ptr, &mut size);
        }

        let providers = unsafe { slice::from_raw_parts(ptr, size) }
            .iter()
            .map(|provider| {
                use crate::library::DefaultFontProvider::*;
                use ffi::ASS_DefaultFontProvider::*;
                match provider {
                    ASS_FONTPROVIDER_NONE => None,
                    ASS_FONTPROVIDER_AUTODETECT => Autodetect,
                    ASS_FONTPROVIDER_CORETEXT => CoreText,
                    ASS_FONTPROVIDER_FONTCONFIG => Fontconfig,
                    ASS_FONTPROVIDER_DIRECTWRITE => DirectWrite,
                }
            })
            .collect();

        unsafe {
            libc::free(ptr.cast::<c_void>());
        }

        providers
    }

    pub fn new_renderer(&self) -> Result<Renderer> {
        let renderer = unsafe { ffi::ass_renderer_init(self.raw.as_ptr() as *mut _) };

        if renderer.is_null() {
            return Err(crate::Error);
        }

        unsafe { Ok(Renderer::new_unchecked(renderer)) }
    }

    pub fn new_track(&self) -> Result<Track> {
        let track = unsafe { ffi::ass_new_track(self.raw.as_ptr() as *mut _) };

        if track.is_null() {
            return Err(crate::Error);
        }

        unsafe { Ok(Track::new_unchecked(track)) }
    }

    pub fn new_track_from_file(&self, filename: &str, codepage: &str) -> Result<Track> {
        let filename = CString::new(filename).unwrap();
        let cp = CString::new(codepage).unwrap();
        let track = unsafe {
            ffi::ass_read_file(
                self.raw.as_ptr() as *mut _,
                filename.as_ptr() as *mut _,
                cp.as_ptr() as *mut _,
            )
        };

        if track.is_null() {
            return Err(crate::Error);
        }

        unsafe { Ok(Track::new_unchecked(track)) }
    }

    pub fn new_track_from_memory(&self, data: &[u8], codepage: &str) -> Result<Track> {
        let cp = CString::new(codepage).unwrap();
        let track = unsafe {
            ffi::ass_read_memory(
                self.raw.as_ptr() as *mut _,
                data.as_ptr() as *mut _,
                data.len(),
                cp.as_ptr() as *mut _,
            )
        };

        if track.is_null() {
            return Err(crate::Error);
        }

        unsafe { Ok(Track::new_unchecked(track)) }
    }
}
