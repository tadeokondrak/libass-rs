use std::marker::PhantomData;
use std::ptr::NonNull;
use std::slice;

use libass_sys as ffi;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ImageKind {
    Character,
    Outline,
    Shadow,
}

pub struct Image<'renderer> {
    handle: Option<NonNull<ffi::ass_image>>,
    phantom: PhantomData<&'renderer mut ffi::ass_image>,
}

impl<'renderer> Image<'renderer> {
    pub(crate) unsafe fn new_unchecked(image: *mut ffi::ass_image) -> Self {
        Image {
            handle: Some(NonNull::new_unchecked(image)),
            phantom: PhantomData,
        }
    }
}

impl<'renderer> Iterator for Image<'renderer> {
    type Item = Layer;
    fn next(&mut self) -> Option<Layer> {
        let handle = self.handle?;
        let c_layer = unsafe { handle.as_ref() };

        use crate::ImageKind::*;
        use ffi::ass_image__bindgen_ty_1::*;

        let layer = Some(Layer {
            width: c_layer.w,
            height: c_layer.h,
            bitmap: {
                let mut vec = Vec::with_capacity((c_layer.w * c_layer.h) as usize);
                let mut ptr = c_layer.bitmap;
                for _ in 0..c_layer.h {
                    unsafe {
                        vec.extend_from_slice(slice::from_raw_parts(ptr, c_layer.w as usize))
                    };
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

        self.handle = if c_layer.next.is_null() {
            None
        } else {
            unsafe { Some(NonNull::new_unchecked(c_layer.next)) }
        };

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
