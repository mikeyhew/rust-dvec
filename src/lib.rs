use std::marker::PhantomData;
use std::mem;

pub struct DVec<T> {
    data: *mut T,
    length: usize,
    offset: usize,
    capacity: usize,
}

impl<T> DVec<T> {
    pub fn with_capacity(capacity: usize) -> DVec<T> {
        let p: *mut T = unsafe {
            let mut v = Vec::<T>::with_capacity(capacity);
            v.set_len(capacity);
            let mut b = v.into_boxed_slice();
            let p = b.as_mut().as_mut_ptr();
            mem::forget(b);
            p
        };

        DVec {
            data: p,
            length: 0,
            offset: capacity/2,
            capacity: capacity,
        }
    }
}

impl<T> Drop for DVec<T> {
    fn drop(&mut self) {
        let mut v = unsafe {
            Vec::from_raw_parts(self.data, self.capacity, self.capacity)
        };
        let mut i: usize = 0;
        for x in v {
            if i < self.offset {
                mem::forget(x);
            } else if i < self.offset + self.length {
                mem::drop(x);
            } else if i < self.capacity {
                mem::forget(x);
            } else {
                unreachable!();
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DVec;
    #[test]
    fn it_works() {
        DVec::<usize>::with_capacity(0);
        DVec::<usize>::with_capacity(20);
    }
}
