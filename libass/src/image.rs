use std::slice;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ImageKind {
    Character,
    Outline,
    Shadow,
}

pub struct Image<'renderer>(Option<&'renderer mut libass_sys::ass_image>);

impl<'renderer> Image<'renderer> {
    pub(crate) fn new(image: &'renderer mut libass_sys::ass_image) -> Self {
        Image(Some(image))
    }
}

impl<'renderer> Iterator for Image<'renderer> {
    type Item = Layer;
    fn next(&mut self) -> Option<Layer> {
        if self.0.is_none() {
            return None
        }

        let c_layer = self.0.as_ref().unwrap();

        let width = c_layer.w;
        let height = c_layer.h;

        use crate::ImageKind::*;
        use libass_sys::ass_image__bindgen_ty_1::*;

        let layer = Some(Layer {
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
        });

        if c_layer.next.is_null() {
            self.0 = None
        } else {
            self.0 = unsafe { Some(&mut *c_layer.next) };
        }

        layer
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
