use std::ffi::CStr;
use std::marker::PhantomData;
use std::os::raw::c_int;
use std::ptr::NonNull;

use libass_sys as ffi;

pub struct Track<'library> {
    handle: NonNull<ffi::ass_track>,
    phantom: PhantomData<&'library mut ffi::ass_track>,
}

impl<'library> Track<'library> {
    pub(crate) unsafe fn new_unchecked(track: *mut ffi::ass_track) -> Self {
        Track {
            handle: NonNull::new_unchecked(track),
            phantom: PhantomData,
        }
    }

    pub(crate) fn as_ptr(&mut self) -> *mut ffi::ass_track {
        self.handle.as_ptr()
    }

    pub fn new_style(&self) -> Style {
        Style {
            id: unsafe { ffi::ass_alloc_style(self.handle.as_ptr()) },
            parent: &self,
        }
    }

    pub fn new_event(&self) -> Event {
        Event {
            id: unsafe { ffi::ass_alloc_event(self.handle.as_ptr()) },
            parent: &self,
        }
    }

    pub fn step_sub(&self, now: i64, movement: i32) -> i64 {
        unsafe { ffi::ass_step_sub(self.handle.as_ptr() as *mut _, now, movement) }
    }

    pub fn process_force_style(&mut self) {
        unsafe { ffi::ass_process_force_style(self.handle.as_ptr()) }
    }

    pub fn read_styles(&mut self, filename: &CStr, codepage: &CStr) {
        unsafe {
            ffi::ass_read_styles(
                self.handle.as_ptr(),
                filename.as_ptr() as *mut _,
                codepage.as_ptr() as *mut _,
            );
        }
    }

    pub fn set_check_readorder(&mut self, check_readorder: bool) {
        unsafe { ffi::ass_set_check_readorder(self.handle.as_ptr(), check_readorder as c_int) }
    }

    pub fn flush_events(&mut self) {
        unsafe { ffi::ass_flush_events(self.handle.as_ptr()) }
    }

    pub fn process_data(&mut self, data: &mut [u8]) {
        unsafe {
            ffi::ass_process_data(
                self.handle.as_ptr(),
                data.as_ptr() as *mut _,
                data.len() as c_int,
            )
        }
    }

    pub fn process_codec_private(&mut self, data: &mut [u8]) {
        unsafe {
            ffi::ass_process_codec_private(
                self.handle.as_ptr(),
                data.as_ptr() as *mut _,
                data.len() as c_int,
            )
        }
    }

    pub fn process_chunk(&mut self, data: &mut [u8], timecode: i64, duration: i64) {
        unsafe {
            ffi::ass_process_chunk(
                self.handle.as_ptr(),
                data.as_ptr() as *mut _,
                data.len() as c_int,
                timecode,
                duration,
            )
        }
    }
}

impl<'library> Drop for Track<'library> {
    fn drop(&mut self) {
        unsafe { ffi::ass_free_track(self.handle.as_ptr()) }
    }
}

pub struct Style<'track> {
    pub id: i32,
    parent: &'track Track<'track>,
}

impl<'track> Drop for Style<'track> {
    fn drop(&mut self) {
        unsafe { ffi::ass_free_style(self.parent.handle.as_ptr(), self.id) }
    }
}

pub struct Event<'track> {
    pub id: i32,
    parent: &'track Track<'track>,
}

impl<'track> Drop for Event<'track> {
    fn drop(&mut self) {
        unsafe { ffi::ass_free_event(self.parent.handle.as_ptr(), self.id) }
    }
}
