/// The number of bytes in a single register on the
/// target architechure.
const REGISTER_BYTES: usize = core::mem::size_of::<usize>();

/// The alignment of usize
const ALIGN: usize = core::mem::align_of::<usize>();

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
                if (self.get_pointer(i) as usize).is_multiple_of(ALIGN) && !self.is_used(i) {
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

pub struct Vec<T: Copy, const HEAP_SIZE: usize> {
    ptr: *mut T,
    length: usize,
    capacity: usize
}

impl<T: Copy, const HEAP_SIZE: usize> Vec<T: Copy, const HEAP_SIZE: usize> {
    /// Create a `Vec<T>`
    pub fn new() -> Vec<T> {
        Vec<T> {
            ptr: core::ptr::null_mut(),
            length: 0,
            capacity: 0
        }
    }
    /// Get an item
    pub fn get(&self, index: usize) -> Option<T> {
        if index < self.length {
            return Ok(unsafe { *self.ptr.add(index) });
        }
        None
    }
    /// Set an item
    pub fn set(&self, index: usize, value: T) {
        if index < self.length {
            unsafe { *self.ptr.add(index) } = value;
        }
    }
    /// Change capacity
    pub fn resize(&mut self, new_capacity: usize, heap: &mut SimpleHeap<HEAP_SIZE>) -> Result<(), ()> {
        if new_capacity == self.capacity { return Ok(()); }
        if self.capacity == 0 {
            self.ptr = match unsafe { heap.allocate(core::mem::size_of::<T>() * new_capacity) } {
                Some(ptr) => ptr as *mut T,
                None => return Err(())
            };
            return Ok(());
        }
        // I know this is inefficient.
        // I will improve this later.
        // It's fine for now though.
        let new_ptr: *mut T = match unsafe { heap.allocate(core::mem::size_of::<T>() * new_capacity) } {
            Some(ptr) => ptr as *mut T,
            None => return Err(())
        };
        // Copy data
        for i in 0..core::cmp::min(self.capacity, new_capacity) {
            let old_item_ptr = unsafe { self.ptr.add(i) };
            let new_item_ptr = unsafe { new_ptr.add(i) };
            unsafe { *new_item_ptr } = unsafe { *old_item_ptr };
        }
        unsafe { heap.deallocate(self.ptr as *mut ()); }
        return Ok(())
    }
    /// Add an item
    pub fn push(&mut self, item: T, heap: &mut SimpleHeap<HEAP_SIZE>) -> Result<(), ()> {
        if self.length < self.capacity {
            self.length += 1;
            self.set(self.length - 1, item);
        } else {
            // We'd rather just grow more often
            // than use unnecesary memory for
            // the purpose of a kernel for devices
            // with very low RAM, especially after
            // I fix the inefficiency of the `resize`
            // method above.
            self.resize(self.capacity + 4);
        }
    }
}
