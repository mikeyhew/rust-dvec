use std::mem;

/// An uninitialized buffer of `capacity * mem::size_of<T>()` bytes.
pub struct Buffer<T> {
    pub ptr: *mut T,
    pub capacity: usize,
}

impl<T> Buffer<T> {
    pub fn new(capacity: usize) -> Buffer<T> {
        let mut v = Vec::<T>::with_capacity(capacity);
        let ptr = v.as_mut_ptr();
        mem::forget(v);
        Buffer {
            ptr: ptr,
            capacity: capacity,
        }
    }

    pub unsafe fn from_raw_parts(ptr: *mut T, capacity: usize) -> Buffer<T> {
        Buffer {
            ptr: ptr,
            capacity: capacity,
        }
    }
}

impl<T> Drop for Buffer<T> {
    fn drop(&mut self) {
        unsafe {
            mem::drop(Vec::from_raw_parts(self.ptr, 0, self.capacity));
        };
    }
}
