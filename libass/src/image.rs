use std::slice;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ImageKind {
    Character,
    Outline,
    Shadow,
}

#[derive(Debug, Copy, Clone)]
pub struct Image(*mut libass_sys::ass_image);

impl Image {
    pub(crate) fn new(ptr: *mut libass_sys::ass_image) -> Self {
        Image(ptr)
    }
}

impl Iterator for Image {
    type Item = Layer;
    fn next(&mut self) -> Option<Layer> {
        if self.0.is_null() {
            return None;
        }

        let c_layer = unsafe { &*self.0 };

        self.0 = c_layer.next;

        let width = c_layer.w;
        let height = c_layer.h;

        use crate::ImageKind::*;
        use libass_sys::ass_image__bindgen_ty_1::*;

        Some(Layer {
            width,
            height,
            bitmap: {
                let mut vec = Vec::with_capacity((width * height) as usize);
                let mut ptr = c_layer.bitmap;
                for _ in 0..height {
                    unsafe { vec.extend_from_slice(slice::from_raw_parts(ptr, width as usize)) };
                    ptr = unsafe { ptr.offset(c_layer.stride as isize) };
                }
                vec
            },
            color: c_layer.color,
            x: c_layer.dst_x,
            y: c_layer.dst_y,
            kind: match c_layer.type_ {
                IMAGE_TYPE_CHARACTER => Character,
                IMAGE_TYPE_OUTLINE => Outline,
                IMAGE_TYPE_SHADOW => Shadow,
            },
        })
    }
}

#[derive(Debug, Clone)]
pub struct Layer {
    pub width: i32,
    pub height: i32,
    pub bitmap: Vec<u8>,
    pub color: u32,
    pub x: i32,
    pub y: i32,
    pub kind: ImageKind,
}
