use crate::Allocator;
use core::ptr;

pub struct System {
    _priv: (),
}

impl System {
    pub const fn new() -> System {
        System { _priv: () }
    }
}

unsafe impl Allocator for System {
    /// Allocate an additional `size` bytes on the heap, and return a new
    /// chunk of memory, as well as the size of the allocation and some
    /// flags. Since flags are unused on this platform, they will always
    /// be `0`.
    fn alloc(&self, size: usize) -> (*mut u8, usize, u32) {
        let size = if size == 0 {
            0x1000
        } else {
            size.next_multiple_of(0x1000)
        };

        if let Ok(range) = xous::map_memory(None, None, size, xous::MemoryFlags::W) {
            (range.as_mut_ptr(), range.len(), 0)
        } else {
            (ptr::null_mut(), 0, 0)
        }
    }

    fn remap(&self, _ptr: *mut u8, _oldsize: usize, _newsize: usize, _can_move: bool) -> *mut u8 {
        // TODO
        ptr::null_mut()
    }

    fn free_part(&self, ptr: *mut u8, oldsize: usize, newsize: usize) -> bool {
        let range = unsafe { xous::MemoryRange::new(ptr as usize + newsize, oldsize - newsize) };
        if let Ok(range) = range {
            xous::unmap_memory(range).is_ok()
        } else {
            false
        }
    }

    fn free(&self, ptr: *mut u8, size: usize) -> bool {
        let range = unsafe { xous::MemoryRange::new(ptr as _, size) };
        if let Ok(range) = range {
            xous::unmap_memory(range).is_ok()
        } else {
            false
        }
    }

    fn can_release_part(&self, _flags: u32) -> bool {
        true
    }

    fn allocates_zeros(&self) -> bool {
        true
    }

    fn page_size(&self) -> usize {
        0x1000
    }
}

#[cfg(feature = "global")]
pub fn acquire_global_lock() {
    // global feature should not be enabled
    unimplemented!()
}

#[cfg(feature = "global")]
pub fn release_global_lock() {
    // global feature should not be enabled
    unimplemented!()
}

#[cfg(feature = "global")]
pub unsafe fn enable_alloc_after_fork() {
    // platform does not support `fork()` call
}
