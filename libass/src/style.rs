use bitflags::bitflags;
use std::ffi::CString;
use std::os::raw::c_int;

use libass_sys as ffi;

pub struct Style {
    pub name: CString,
    pub font_name: CString,
    pub font_size: f64,
    pub primary_color: u32,
    pub secondary_color: u32,
    pub outline_color: u32,
    pub back_color: u32,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikeout: bool,
    pub scale_x: f64,
    pub scale_y: f64,
    pub spacing: f64,
    pub angle: f64,
    pub border_style: i32,
    pub outline: f64,
    pub shadow: f64,
    pub alignment: i32,
    pub margin_l: i32,
    pub margin_r: i32,
    pub margin_v: i32,
    pub encoding: i32,
    pub treat_fontname_as_pattern: bool,
    pub blur: f64,
    pub justify: i32,
}

bitflags! {
    pub struct OverrideBits: u32 {
        const DEFAULT = 0;
        const BIT_STYLE = 1;
        const BIT_SELECTIVE_FONT_SCALE = 1 << 1;
        const BIT_FONT_SIZE = 1 << 1;
        const BIT_FONT_SIZE_FIELDS = 1 << 2;
        const BIT_FONT_NAME = 1 << 3;
        const BIT_COLORS = 1 << 4;
        const BIT_ATTRIBUTES = 1 << 5;
        const BIT_BORDER = 1 << 6;
        const BIT_ALIGNMENT = 1 << 7;
        const BIT_MARGINS = 1 << 8;
        const FULL_STYLE = 1 << 9;
        const BIT_JUSTIFY = 1 << 10;
    }
}

impl Style {
    // the result is only valid as long as the parent is
    pub(crate) unsafe fn as_ass_style(&self) -> ffi::ass_style {
        ffi::ass_style {
            Name: self.name.as_ptr() as *mut _,
            FontName: self.font_name.as_ptr() as *mut _,
            FontSize: self.font_size,
            PrimaryColour: self.primary_color,
            SecondaryColour: self.secondary_color,
            OutlineColour: self.outline_color,
            BackColour: self.back_color,
            Bold: self.bold as c_int,
            Italic: self.italic as c_int,
            Underline: self.underline as c_int,
            StrikeOut: self.strikeout as c_int,
            ScaleX: self.scale_x,
            ScaleY: self.scale_y,
            Spacing: self.spacing,
            Angle: self.angle,
            BorderStyle: self.border_style,
            Outline: self.outline,
            Shadow: self.shadow,
            Alignment: self.alignment,
            MarginL: self.margin_l,
            MarginR: self.margin_r,
            MarginV: self.margin_v,
            Encoding: self.encoding,
            treat_fontname_as_pattern: self.treat_fontname_as_pattern as c_int,
            Blur: self.blur,
            Justify: self.justify,
        }
    }
}
