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
            unsafe { ptr.as_ptr().write(item) }
            self.ptr = ptr;
            self.len = 1;
            self.capacity = 4;
        } else if self.len < self.capacity {
            let offset = self
                .len
                .checked_mul(std::mem::size_of::<T>())
                .expect("Failed to reach memory location");
            assert!(offset < isize::MAX as usize, "Wrapped isize");
            // SAFETY: writing to an offset at self.len is valid
            unsafe { self.ptr.as_ptr().add(self.len).write(item) }
            self.len += 1;
        } else {
            debug_assert!(self.len == self.capacity);

            let size = self.capacity.checked_mul(2).expect("Capacity wrapped");
            let align = std::mem::align_of::<T>();
            let new_capacity = self.capacity * 2;
            size.checked_add(size % align).expect("Cannot allocate");

            let ptr = unsafe {
                let layout = Layout::from_size_align_unchecked(size, align);
                let new_size = std::mem::size_of::<T>() * new_capacity;
                let ptr = alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, new_size);
                let ptr = NonNull::new(ptr as *mut T).expect("Failed to reallocate memory");
                ptr.as_ptr().add(self.len).write(item);
                ptr
            };

            self.ptr = ptr;
            self.len += 1;
            self.capacity = new_capacity;
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        Some(unsafe { &*self.ptr.as_ptr().add(index) })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len));
            let layout = Layout::from_size_align_unchecked(
                std::mem::size_of::<T>() * self.capacity,
                std::mem::align_of::<T>(),
            );
            alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut vec = MyVec::<usize>::new();
        vec.push(1usize);

        assert_eq!(vec.len(), 1);
        assert_eq!(vec.capacity(), 4);
        assert_eq!(vec.get(0), Some(&1));
        assert_eq!(vec.get(1), None);

        vec.push(2);
        vec.push(3);
        vec.push(4);
        vec.push(5);

        assert_eq!(vec.len(), 5);
        assert_eq!(vec.capacity(), 8);
        assert_eq!(vec.get(4), Some(&5));
        assert_eq!(vec.get(5), None);

        vec.push(6);
        vec.push(7);
        vec.push(8);
        vec.push(9);

        assert_eq!(vec.len(), 9);
        assert_eq!(vec.capacity(), 16);
        assert_eq!(vec.get(8), Some(&9));
        assert_eq!(vec.get(9), None);
    }
}
