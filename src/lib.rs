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

        let old_buf = mem::replace(&mut self.buf, Buffer::new(new_capacity));

        let old_offset = self.offset;
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
                ptr::drop_in_place(
                    self.buf.ptr.offset((self.offset + i) as isize)
                );
            }
        }
    }
}

#[test]
fn test_resize() {
    let mut v = DVec::<i32>::with_capacity(2);
    v.resize(4);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_bad_resize() {
    unimplemented!()
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
