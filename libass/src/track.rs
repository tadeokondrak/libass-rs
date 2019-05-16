pub struct Track<'library>(&'library mut libass_sys::ass_track);

impl<'library> Track<'library> {
    pub(crate) fn new(track: &'library mut libass_sys::ass_track) -> Self {
        Track(track)
    }

    pub(crate) fn as_ptr(&self) -> *const libass_sys::ass_track {
        self.0
    }

    pub fn process_force_style(&mut self) {
        unsafe { libass_sys::ass_process_force_style(self.0) }
    }

    pub fn step_sub(&self, now: i64, movement: i32) -> i64 {
        unsafe { libass_sys::ass_step_sub(self.0 as *const _ as *mut _, now, movement) }
    }
}

impl<'library> Drop for Track<'library> {
    fn drop(&mut self) {
        unsafe { libass_sys::ass_free_track(self.0) }
    }
}
