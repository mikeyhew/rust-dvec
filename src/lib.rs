use std::mem;
use std::ptr;
mod buffer;
use buffer::Buffer;

#[doc(hidden)]
pub use std::vec;

macro_rules! dvec {
    ($elem: expr, $n: expr) => (
        $crate::DVec::from_vec($crate::vec::from_elem($elem, $n))
    );

    ($($x:expr),*) => (
        $crate::DVec::from_vec(vec![$($x),*]);
    );
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

    pub fn from_vec(mut v: Vec<T>) -> DVec<T> {
        unsafe {
            let dv = DVec {
                buf: Buffer::from_raw_parts(v.as_mut_ptr(), v.capacity()),
                offset: 0,
                length: v.len(),
            };
            mem::forget(v);
            dv
        }
    }

    pub fn capacity(&self) -> usize {
        self.buf.capacity
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push_front(&mut self, value: T) {

        if self.capacity() == 0 {
            self.resize(1);
            self.offset = 1;
        } else if self.offset == 0 {
            let new_capacity = self.capacity() * 2;
            self.resize(new_capacity);
        }

        self.offset -= 1;
        self.length += 1;
        unsafe {
            ptr::write(
                self.buf.ptr.offset(self.offset as isize),
                value
            );
        }
    }

    pub fn push_back(&mut self, value: T) {

        if self.capacity() == 0 {
            self.resize(1);
            self.offset = 0;
        }

        if self.buf.capacity == self.offset + self.length {
            let new_capacity = self.buf.capacity * 2;
            self.resize(new_capacity);
        }

        self.length += 1;
        unsafe {
            ptr::write(
                self.buf.ptr.offset((self.offset + self.length - 1) as isize),
                value
            );
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

impl<A, B> PartialEq<DVec<B>> for DVec<A> where A: PartialEq<B> {
    fn eq(&self, other: &DVec<B>) -> bool {
        unimplemented!()
    }
}

#[test]
fn test_resize() {
    let mut v = DVec::<i32>::with_capacity(2);
    v.push_front(1);
    v.push_front(2);
    v.resize(4);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_bad_resize() {
    let mut v = DVec::<i32>::with_capacity(2);
    v.push_front(1);
    v.push_front(2);
    v.resize(1);
}

#[cfg(test)]
mod tests {
    use super::DVec;

    #[test]
    fn it_works() {
        DVec::<usize>::with_capacity(20);
        let mut v = DVec::<usize>::new();
        v.push_front(1);
        v.push_back(2);
    }

    #[test]
    fn test_macro() {
        dvec![String::from("abc"), 10];
        dvec![String::from("foo"), String::from("bar"), String::from("baz")];
        let x: DVec<()> = dvec![];
    }

    // fn test_equality() {
    //     let mut v1 = DVec::<usize>::new();
    //     v1.push_back(1);
    //     v1.push_back(2);
    //     v1.push_back(3);
    //
    //     let mut v2 = DVec::<usize>::new();
    //     v2.push_back(3);
    //     v2.push_back(2);
    //     v2.push_back(1);
    //
    //     assert!(v1 == v2)
    // }
}
