use std::ffi::CString;
use std::os::raw::c_int;
use std::ptr;
use std::slice;
use std::mem;

use crate::renderer::Renderer;
use crate::track::Track;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DefaultFontProvider {
    None,
    Autodetect,
    CoreText,
    Fontconfig,
    DirectWrite,
}

pub struct Library<'a>(&'a mut libass_sys::ass_library);

impl<'a> Library<'a> {
    pub fn version() -> i32 {
        unsafe { libass_sys::ass_library_version() }
    }

    pub fn new() -> Option<Self> {
        let lib = unsafe { libass_sys::ass_library_init() };
        if lib.is_null() {
            return None;
        }
        unsafe { Some(Library(&mut *lib)) }
    }

    /// fonts_dir can't have null bytes
    pub fn set_fonts_dir(&mut self, fonts_dir: &str) {
        let fonts_dir = cstring!(fonts_dir);
        unsafe { libass_sys::ass_set_fonts_dir(self.0, fonts_dir.as_ptr()) }
    }

    pub fn set_extract_fonts(&mut self, extract: bool) {
        unsafe { libass_sys::ass_set_extract_fonts(self.0, extract as c_int) }
    }

    /// list can't have null bytes
    pub fn set_style_overrides(&mut self, list: &[&str]) {
        let mut c_list: Vec<CString> = Vec::with_capacity(list.len());
        for item in list {
            c_list.push(cstring!(*item))
        }
        unsafe {
            libass_sys::ass_set_style_overrides(
                self.0,
                c_list
                    .iter()
                    .map(|x| x.as_ptr())
                    .collect::<Vec<_>>()
                    .as_slice()
                    .as_ptr() as *mut *mut _,
            )
        };
    }

    /// name can't have null bytes
    pub fn add_font(&mut self, name: &str, data: &[u8]) {
        let name = cstring!(name);
        unsafe {
            libass_sys::ass_add_font(
                self.0,
                name.as_ptr() as *mut _,
                data.as_ptr() as *mut _,
                data.len() as c_int,
            )
        }
    }

    pub fn clear_fonts(&mut self) {
        unsafe { libass_sys::ass_clear_fonts(self.0) }
    }

    pub fn get_available_font_providers(&mut self) -> Vec<DefaultFontProvider> {
        let mut providers: *mut libass_sys::ASS_DefaultFontProvider = ptr::null_mut();
        let providers_ptr = &mut providers as *mut *mut libass_sys::ASS_DefaultFontProvider;

        let mut size: usize = 0;
        let size_ptr = &mut size as *mut usize;

        unsafe { libass_sys::ass_get_available_font_providers(self.0, providers_ptr, size_ptr) };

        let providers_slice = unsafe { slice::from_raw_parts(providers, size) };

        let mut vec: Vec<DefaultFontProvider> = Vec::with_capacity(size);

        for provider in providers_slice {
            use crate::library::DefaultFontProvider::*;
            use libass_sys::ASS_DefaultFontProvider::*;
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

    pub fn new_renderer(&self) -> Option<Renderer> {
        let renderer = unsafe { libass_sys::ass_renderer_init(mem::transmute::<*const _, *mut _>(self.0)) };
        if renderer.is_null() {
            return None;
        }
        unsafe { Some(Renderer::new(&mut *renderer)) }
    }

    pub fn new_track(&self) -> Option<Track> {
        let track = unsafe { libass_sys::ass_new_track(mem::transmute::<*const _, *mut _>(self.0)) };
        if track.is_null() {
            return None;
        }
        unsafe { Some(Track::new(&mut *track)) }
    }

    /// filename and codepage can't have null bytes
    pub fn new_track_from_file(&self, filename: &str, codepage: &str) -> Option<Track> {
        let filename = cstring!(filename);
        let codepage = cstring!(codepage);
        let track = unsafe {
            libass_sys::ass_read_file(
                mem::transmute::<*const _, *mut _>(self.0),
                filename.as_ptr() as *mut _,
                codepage.as_ptr() as *mut _,
            )
        };
        if track.is_null() {
            return None;
        }
        unsafe { Some(Track::new(&mut *track))}
    }

    /// codepage can't have null bytes
    pub fn new_track_from_memory(&self, data: &[u8], codepage: &str) -> Option<Track> {
        let codepage = cstring!(codepage);
        let track = unsafe {
            libass_sys::ass_read_memory(
                mem::transmute::<*const _, *mut _>(self.0),
                data.as_ptr() as *mut _,
                data.len(),
                codepage.as_ptr() as *mut _,
            )
        };
        if track.is_null() {
            return None;
        }
        unsafe { Some(Track::new(&mut *track)) }
    }
}

impl<'a> Drop for Library<'a> {
    fn drop(&mut self) {
        unsafe { libass_sys::ass_library_done(self.0) }
    }
}
