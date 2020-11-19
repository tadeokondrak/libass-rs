use std::marker::PhantomData;
use std::os::raw::c_int;
use std::ptr;
use std::ptr::null_mut;
use std::ptr::NonNull;
use std::slice;

use libass_sys as ffi;

use crate::renderer::Renderer;
use crate::track::Track;
use crate::{err_if_null, Result};

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

pub struct Library<'a> {
    handle: NonNull<ffi::ass_library>,
    phantom: PhantomData<&'a mut ffi::ass_library>,
}

impl<'a> Library<'a> {
    pub fn new() -> Result<Self> {
        let lib = unsafe { ffi::ass_library_init() };
        err_if_null!(lib);
        Ok(Library {
            handle: unsafe { NonNull::new_unchecked(lib) },
            phantom: PhantomData,
        })
    }

    pub fn set_fonts_dir(&mut self, fonts_dir: &str) {
        unsafe { ffi::ass_set_fonts_dir(self.handle.as_ptr(), fonts_dir.as_ptr() as *const i8) }
    }

    pub fn set_extract_fonts(&mut self, extract: bool) {
        unsafe { ffi::ass_set_extract_fonts(self.handle.as_ptr(), extract as c_int) }
    }

    pub fn set_style_overrides(&mut self, list: &[&str]) {
        unsafe {
            ffi::ass_set_style_overrides(
                self.handle.as_ptr(),
                list.iter()
                    .map(|x| x.as_ptr())
                    .collect::<Vec<_>>()
                    .as_slice()
                    .as_ptr() as *mut *mut _,
            )
        };
    }

    pub fn add_font(&mut self, name: &str, data: &[u8]) {
        unsafe {
            ffi::ass_add_font(
                self.handle.as_ptr(),
                name.as_ptr() as *mut _,
                data.as_ptr() as *mut _,
                data.len() as c_int,
            )
        }
    }

    pub fn clear_fonts(&mut self) {
        unsafe { ffi::ass_clear_fonts(self.handle.as_ptr()) }
    }

    pub fn get_available_font_providers(&mut self) -> Vec<DefaultFontProvider> {
        let mut providers: *mut ffi::ASS_DefaultFontProvider = ptr::null_mut();
        let providers_ptr = &mut providers as *mut *mut ffi::ASS_DefaultFontProvider;

        let mut size: usize = 0;
        let size_ptr = &mut size as *mut usize;

        unsafe {
            ffi::ass_get_available_font_providers(self.handle.as_ptr(), providers_ptr, size_ptr)
        };

        let providers_slice = unsafe { slice::from_raw_parts(providers, size) };

        let mut vec: Vec<DefaultFontProvider> = Vec::with_capacity(size);

        for provider in providers_slice {
            use crate::library::DefaultFontProvider::*;
            use ffi::ASS_DefaultFontProvider::*;
            vec.push(match provider {
                ASS_FONTPROVIDER_NONE => None,
                ASS_FONTPROVIDER_AUTODETECT => Autodetect,
                ASS_FONTPROVIDER_CORETEXT => CoreText,
                ASS_FONTPROVIDER_FONTCONFIG => Fontconfig,
                ASS_FONTPROVIDER_DIRECTWRITE => DirectWrite,
            })
        }

        unsafe { libc::free(providers as *mut libc::c_void) };

        vec
    }

    pub fn new_renderer(&self) -> Result<Renderer> {
        let renderer = unsafe { ffi::ass_renderer_init(self.handle.as_ptr() as *mut _) };
        err_if_null!(renderer);
        unsafe { Ok(Renderer::new_unchecked(renderer)) }
    }

    pub fn new_track(&self) -> Result<Track> {
        let track = unsafe { ffi::ass_new_track(self.handle.as_ptr() as *mut _) };
        err_if_null!(track);
        unsafe { Ok(Track::new_unchecked(track)) }
    }

    pub fn new_track_from_file(&self, filename: &str) -> Result<Track> {
        let track = unsafe {
            ffi::ass_read_file(
                self.handle.as_ptr() as *mut _,
                filename.as_ptr() as *mut _,
                null_mut(),
            )
        };

        err_if_null!(track);
        unsafe { Ok(Track::new_unchecked(track)) }
    }

    pub fn new_track_from_memory(&self, data: &[u8], codepage: &str) -> Result<Track> {
        let track = unsafe {
            ffi::ass_read_memory(
                self.handle.as_ptr() as *mut _,
                data.as_ptr() as *mut _,
                data.len(),
                codepage.as_ptr() as *mut _,
            )
        };

        err_if_null!(track);
        unsafe { Ok(Track::new_unchecked(track)) }
    }
}

impl<'a> Drop for Library<'a> {
    fn drop(&mut self) {
        unsafe { ffi::ass_library_done(self.handle.as_ptr()) }
    }
}
