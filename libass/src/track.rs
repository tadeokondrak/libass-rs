pub struct Track(*mut libass_sys::ass_track);

impl Track {
    pub(crate) fn new(ptr: *mut libass_sys::ass_track) -> Self {
        Track(ptr)
    }

    pub(crate) fn as_ptr(&self) -> *const libass_sys::ass_track {
        self.0
    }

    pub fn process_force_style(&self) {
        unsafe { libass_sys::ass_process_force_style(self.0) }
    }

    pub fn step_sub(&self, now: i64, movement: i32) -> i64 {
        unsafe { libass_sys::ass_step_sub(self.0, now, movement) }
    }
}

impl Drop for Track {
    fn drop(&mut self) {
        unsafe { libass_sys::ass_free_track(self.0) }
    }
}
