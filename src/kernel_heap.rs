/// The number of bytes in a single register on the
/// target architechure.
const REGISTER_BYTES: usize = core::mem::size_of::<usize>();

/// A simple heap structure
pub struct SimpleHeap<const SIZE: usize> {
    data: [u8; SIZE],
    in_use: [u8; SIZE / 8]
}

impl<const SIZE: usize> SimpleHeap<const SIZE: usize> {
    /// Set whether a byte is in use
    fn set_used(&mut self, id: usize, value: bool) {
        let bit = id % 8;
        let mut byte = self.in_use[id / 8];
        byte = byte & !(1 << bit);
        if value {
            byte += 1 << bit;
        }
        self.in_use[id / 8] = byte;
    }
    /// Check if a byte is in use
    pub fn is_used(&self, id: usize) -> bool {
        (self.in_use[id / 8] >> (id % 8)) > 0
    }
    /// Convert a byte index to a pointer
    pub fn get_pointer(&self, id: usize) -> *const () {
        &raw self.data[id]
    }
    /// Convert a byte index to a mutable pointer
    pub fn get_pointer_mut(&mut self, id: usize) -> *mut () {
        &raw mut self.data[id]
    }
    /// Convert a pointer to a byte index
    pub fn get_index(&self, ptr: *const ()) -> usize {
        (ptr as usize) - (&raw self.data[0] as usize)
    }
    /// Convert a pointer to a byte index
    pub fn get_index_mut(&self, ptr: *mut ()) -> usize {
        (ptr as usize) - (&raw self.data[0] as usize)
    }
    /// Allocate some memory
    pub unsafe fn allocate(
        &mut self,
        bytes: usize
    ) -> Option<*mut ()> {
        let mut run_length: usize = 0;
        let mut run_start: usize = 0;
        for i in 0..self.data.len() {
            if run_length == 0 {
                if (self.get_pointer(i) as usize).is_multiple_of(REGISTER_BYTES) && !self.is_used(i) {
                    run_length += 1;
                    run_start = i;
                }
                continue;
            }
            // Run length is not zero, we are doing a run
            if self.is_used(i) {
                run_length = 0;
            } else {
                run_length += 1;
            }
            if run_length == (bytes + REGISTER_BYTES) {
                for i in run_start..(run_start + bytes + REGISTER_BYTES) {
                    self.set_used(i, true);
                }
                unsafe { *(self.get_pointer_mut(run_start) as *mut usize) = bytes; }
                return Some(self.get_pointer_mut(run_start + REGISTER_BYTES));
            }
        }
        return None;
    }
    /// Deallocate a pointer
    pub unsafe fn deallocate(
        &mut self,
        ptr: *mut ()
    ) {
        let length = unsafe {*(ptr.sub(REGISTER_BYTES) as *mut usize)};
        for i in 0..(length + REGISTER_BYTES) {
            self.set_used(i + self.get_index_mut(ptr) - REGISTER_BYTES, false);
        }
    }
}
