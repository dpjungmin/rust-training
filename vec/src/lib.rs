use std::alloc::{self, Layout};
use std::ptr::NonNull;

pub struct MyVec<T> {
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        assert_ne!(std::mem::size_of::<T>(), 0, "No zero sized types");

        if self.capacity == 0 {
            let layout = Layout::array::<T>(4).expect("Failed to allocate memory");
            // SAFETY: the layout is hard coded to be 4 * size_of(T) and size_of(T) > 0
            let ptr = unsafe { alloc::alloc(layout) } as *mut T;
            let ptr = NonNull::new(ptr).expect("Failed to allocate memory");
            // SAFETY: ptr is non-null and we have enough space to write one item
            unsafe { ptr.as_ptr().write(item) };
            self.ptr = ptr;
            self.len = 1;
            self.capacity = 4;
        } else {
            todo!()
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut vec = MyVec::<usize>::new();
        vec.push(1usize);
        // vec.push(2);
        // vec.push(3);
        // vec.push(4);
        // vec.push(5);

        assert_eq!(vec.len(), 1);
        assert_eq!(vec.capacity(), 4);
    }
}
