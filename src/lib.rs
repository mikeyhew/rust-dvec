use std::marker::PhantomData;
use std::mem;
use std::ptr;

/// An uninitialized buffer of `capacity * mem::size_of<T>()` bytes.
struct Buffer<T> {
    ptr: *mut T,
    capacity: usize,
}

impl<T> Buffer<T> {
    fn new(capacity: usize) -> Buffer<T> {
        let mut v = Vec::<T>::with_capacity(capacity);
        let ptr = v.as_mut_ptr();
        mem::forget(v);
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

struct DVec<T> {
    buf: Buffer<T>,
    offset: usize,
    length: usize,
}

impl<T> DVec<T> {
    pub fn new() -> DVec<T> {
        DVec::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> DVec<T> {
        DVec {
            buf: Buffer::<T>::new(capacity),
            length: 0,
            offset: capacity/2,
        }
    }

    fn resize(&mut self, new_capacity: usize) {
        assert!(new_capacity >= self.length);

        // This *could* panic, and I don't know much about panic safety,
        // but it would probably be better if it panics before we change
        // things inside `self`.
        let new_buf = Buffer::new(new_capacity);

        let old_buf = self.buf;
        let old_offset = self.offset;

        self.buf = new_buf;
        self.offset = (new_capacity - self.length) / 2;

        unsafe {
            ptr::copy(
                old_buf.ptr.offset(old_offset as isize),
                self.buf.ptr.offset(self.offset as isize),
                self.length
            );
        }
    }
}

impl<T> Drop for DVec<T> {
    fn drop(&mut self) {
        for i in 0..self.length {
            unsafe {
                mem::drop(*self.buf.ptr.offset((self.offset + i) as isize));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DVec;
    #[test]
    fn it_works() {
        DVec::<usize>::new();
        DVec::<usize>::with_capacity(20);
    }
}
