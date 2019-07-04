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

    pub(crate) fn as_ptr(&self) -> *const ffi::ass_track {
        self.handle.as_ptr()
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
}

impl<'library> Drop for Track<'library> {
    fn drop(&mut self) {
        unsafe { ffi::ass_free_track(self.handle.as_ptr()) }
    }
}
