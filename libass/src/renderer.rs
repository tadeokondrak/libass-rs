use std::ffi::CString;
use std::os::raw::c_int;
use std::ptr;

use crate::image::Image;
use crate::library::DefaultFontProvider;
use crate::track::Track;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ShapingLevel {
    Simple,
    Complex,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Hinting {
    None,
    Light,
    Normal,
    Native,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Change {
    None,
    Position,
    Content,
}

pub struct Renderer(*mut libass_sys::ass_renderer);

impl Renderer {
    pub(crate) fn new(ptr: *mut libass_sys::ass_renderer) -> Self {
        Renderer(ptr)
    }

    pub fn render_frame(&self, track: Track, now: i64) -> (Option<Image>, Change) {
        let mut change = 0;
        let change_ptr: *mut _ = &mut change;

        let image = unsafe {
            libass_sys::ass_render_frame(self.0, track.as_ptr() as *mut _, now, change_ptr)
        };

        let change = match change {
            0 => Change::None,
            1 => Change::Position,
            2 => Change::Content,
            _ => unreachable!(),
        };

        if image.is_null() {
            return (None, change);
        }

        (Some(Image::new(image)), change)
    }

    /// default_font, default_family, and fontconfig_config_path can't have null characters
    pub fn set_fonts(
        &self,
        default_font: Option<&str>,
        default_family: Option<&str>,
        default_font_provider: DefaultFontProvider,
        fontconfig_config_path: Option<&str>,
        update_fontconfig_cache: bool,
    ) {
        macro_rules! optional_cstring {
            ($i:ident, $p:ident) => {
                let $i: Option<CString> = match $i {
                    Some(name) => Some(cstring!(name)),
                    None => None,
                };
                let $p = match $i {
                    Some(name) => name.as_ptr(),
                    None => ptr::null(),
                };
            };
        }

        optional_cstring!(default_font, default_font_ptr);
        optional_cstring!(default_family, default_family_ptr);
        optional_cstring!(fontconfig_config_path, fontconfig_config_path_ptr);

        use libass_sys::ASS_DefaultFontProvider::*;
        let default_font_provider = match default_font_provider {
            DefaultFontProvider::None => ASS_FONTPROVIDER_NONE,
            DefaultFontProvider::Autodetect => ASS_FONTPROVIDER_AUTODETECT,
            DefaultFontProvider::CoreText => ASS_FONTPROVIDER_CORETEXT,
            DefaultFontProvider::Fontconfig => ASS_FONTPROVIDER_FONTCONFIG,
            DefaultFontProvider::DirectWrite => ASS_FONTPROVIDER_DIRECTWRITE,
        };

        unsafe {
            libass_sys::ass_set_fonts(
                self.0,
                default_font_ptr,
                default_family_ptr,
                default_font_provider as c_int,
                fontconfig_config_path_ptr,
                update_fontconfig_cache as c_int,
            )
        };
    }

    pub fn set_frame_size(&self, width: i32, height: i32) {
        unsafe { libass_sys::ass_set_frame_size(self.0, width, height) }
    }

    pub fn set_storage_size(&self, width: i32, height: i32) {
        unsafe { libass_sys::ass_set_storage_size(self.0, width, height) }
    }

    pub fn set_shaper(&self, level: ShapingLevel) {
        unsafe {
            use crate::renderer::ShapingLevel::*;
            use libass_sys::ASS_ShapingLevel::*;
            libass_sys::ass_set_shaper(self.0, {
                match level {
                    Simple => ASS_SHAPING_SIMPLE,
                    Complex => ASS_SHAPING_COMPLEX,
                }
            })
        }
    }

    pub fn set_margins(&self, top: i32, bottom: i32, left: i32, right: i32) {
        unsafe { libass_sys::ass_set_margins(self.0, top, bottom, left, right) }
    }

    pub fn use_margins(&self, use_: bool) {
        unsafe { libass_sys::ass_set_use_margins(self.0, use_ as c_int) }
    }

    pub fn set_pixel_aspect_ratio(&self, par: f64) {
        unsafe { libass_sys::ass_set_pixel_aspect(self.0, par) }
    }

    pub fn set_aspect_ratio(&self, dar: f64, sar: f64) {
        unsafe { libass_sys::ass_set_aspect_ratio(self.0, dar, sar) }
    }

    pub fn set_font_scale(&self, font_scale: f64) {
        unsafe { libass_sys::ass_set_font_scale(self.0, font_scale) }
    }

    pub fn set_hinting(&self, font_hinting: Hinting) {
        unsafe {
            use crate::Hinting::*;
            use libass_sys::ASS_Hinting::*;
            libass_sys::ass_set_hinting(self.0, {
                match font_hinting {
                    None => ASS_HINTING_NONE,
                    Light => ASS_HINTING_LIGHT,
                    Normal => ASS_HINTING_NORMAL,
                    Native => ASS_HINTING_NATIVE,
                }
            })
        }
    }

    pub fn set_line_spacing(&self, line_spacing: f64) {
        unsafe { libass_sys::ass_set_line_spacing(self.0, line_spacing) }
    }

    pub fn set_line_position(&self, line_position: f64) {
        unsafe { libass_sys::ass_set_line_position(self.0, line_position) }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe { libass_sys::ass_renderer_done(self.0) }
    }
}
