use std::ffi::CStr;
use std::marker::PhantomData;
use std::os::raw::c_int;
use std::ptr::NonNull;

use crate::image::Image;
use crate::library::DefaultFontProvider;
use crate::track::Track;

use libass_sys as ffi;

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

pub struct Renderer<'library> {
    handle: NonNull<ffi::ass_renderer>,
    phantom: PhantomData<&'library mut ffi::ass_renderer>,
}

impl<'library> Renderer<'library> {
    pub(crate) unsafe fn new_unchecked(renderer: *mut ffi::ass_renderer) -> Self {
        Renderer {
            handle: NonNull::new_unchecked(renderer),
            phantom: PhantomData,
        }
    }

    pub fn render_frame(&mut self, track: Track, now: i64) -> (Option<Image>, Change) {
        let mut change = 0;
        let change_ptr: *mut _ = &mut change;

        let image = unsafe {
            ffi::ass_render_frame(
                self.handle.as_ptr(),
                track.as_ptr() as *mut _,
                now,
                change_ptr,
            )
        };

        let change = match change {
            0 => Change::None,
            1 => Change::Position,
            2 => Change::Content,
            _ => unreachable!(),
        };

        if image.is_null() {
            (None, change)
        } else {
            unsafe { (Some(Image::new_unchecked(image)), change) }
        }
    }

    pub fn set_fonts(
        &mut self,
        default_font: Option<&CStr>,
        default_family: Option<&CStr>,
        default_font_provider: DefaultFontProvider,
        fontconfig_config_path: Option<&CStr>,
        update_fontconfig_cache: bool,
    ) {
        macro_rules! unwrap_or_null {
            ($x:expr) => {
                match $x {
                    Some(s) => s.as_ptr(),
                    None => ::std::ptr::null(),
                }
            };
        }

        use ffi::ASS_DefaultFontProvider::*;
        let default_font_provider = match default_font_provider {
            DefaultFontProvider::None => ASS_FONTPROVIDER_NONE,
            DefaultFontProvider::Autodetect => ASS_FONTPROVIDER_AUTODETECT,
            DefaultFontProvider::CoreText => ASS_FONTPROVIDER_CORETEXT,
            DefaultFontProvider::Fontconfig => ASS_FONTPROVIDER_FONTCONFIG,
            DefaultFontProvider::DirectWrite => ASS_FONTPROVIDER_DIRECTWRITE,
        };

        unsafe {
            ffi::ass_set_fonts(
                self.handle.as_ptr(),
                unwrap_or_null!(default_font),
                unwrap_or_null!(default_family),
                default_font_provider as c_int,
                unwrap_or_null!(fontconfig_config_path),
                update_fontconfig_cache as c_int,
            )
        };
    }

    pub fn set_frame_size(&mut self, width: i32, height: i32) {
        unsafe { ffi::ass_set_frame_size(self.handle.as_ptr(), width, height) }
    }

    pub fn set_storage_size(&mut self, width: i32, height: i32) {
        unsafe { ffi::ass_set_storage_size(self.handle.as_ptr(), width, height) }
    }

    pub fn set_shaper(&mut self, level: ShapingLevel) {
        unsafe {
            use crate::renderer::ShapingLevel::*;
            use ffi::ASS_ShapingLevel::*;
            ffi::ass_set_shaper(self.handle.as_ptr(), {
                match level {
                    Simple => ASS_SHAPING_SIMPLE,
                    Complex => ASS_SHAPING_COMPLEX,
                }
            })
        }
    }

    pub fn set_margins(&mut self, top: i32, bottom: i32, left: i32, right: i32) {
        unsafe { ffi::ass_set_margins(self.handle.as_ptr(), top, bottom, left, right) }
    }

    pub fn use_margins(&mut self, use_: bool) {
        unsafe { ffi::ass_set_use_margins(self.handle.as_ptr(), use_ as c_int) }
    }

    pub fn set_pixel_aspect_ratio(&mut self, par: f64) {
        unsafe { ffi::ass_set_pixel_aspect(self.handle.as_ptr(), par) }
    }

    pub fn set_aspect_ratio(&mut self, dar: f64, sar: f64) {
        unsafe { ffi::ass_set_aspect_ratio(self.handle.as_ptr(), dar, sar) }
    }

    pub fn set_font_scale(&mut self, font_scale: f64) {
        unsafe { ffi::ass_set_font_scale(self.handle.as_ptr(), font_scale) }
    }

    pub fn set_hinting(&mut self, font_hinting: Hinting) {
        unsafe {
            use crate::Hinting::*;
            use ffi::ASS_Hinting::*;
            ffi::ass_set_hinting(self.handle.as_ptr(), {
                match font_hinting {
                    None => ASS_HINTING_NONE,
                    Light => ASS_HINTING_LIGHT,
                    Normal => ASS_HINTING_NORMAL,
                    Native => ASS_HINTING_NATIVE,
                }
            })
        }
    }

    pub fn set_line_spacing(&mut self, line_spacing: f64) {
        unsafe { ffi::ass_set_line_spacing(self.handle.as_ptr(), line_spacing) }
    }

    pub fn set_line_position(&mut self, line_position: f64) {
        unsafe { ffi::ass_set_line_position(self.handle.as_ptr(), line_position) }
    }

    #[doc(hidden)]
    pub fn update_fonts(&mut self) -> Result<(), i32> {
        let ret = unsafe { ffi::ass_fonts_update(self.handle.as_ptr()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(ret)
        }
    }
}

impl<'library> Drop for Renderer<'library> {
    fn drop(&mut self) {
        unsafe { ffi::ass_renderer_done(self.handle.as_ptr()) }
    }
}
