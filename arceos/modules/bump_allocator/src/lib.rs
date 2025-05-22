#![no_std]

use core::{alloc::Layout, ptr::NonNull};

use allocator::{AllocError, AllocResult, BaseAllocator, ByteAllocator, PageAllocator};

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
// const PAGE_SIZE: usize = 0x1000;

pub struct EarlyAllocator<const PAGE_SIZE: usize> {
    start: usize,
    end: usize,
    forward_current: usize,
    backward_current: usize,
}

impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE> {
    pub const fn new() -> Self {
        EarlyAllocator { 
            start: 0, 
            end: 0, 
            forward_current: 0,
            backward_current: 0 
        }
    }
}

impl<const PAGE_SIZE: usize> BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        // todo!()
        self.start = start;
        let end = (start + size) / PAGE_SIZE * PAGE_SIZE;
        self.end = end;
        self.forward_current = self.start;
        self.backward_current = self.end;
    }

    fn add_memory(&mut self, _: usize, _: usize) -> allocator::AllocResult {
        todo!()
    }
}

impl<const PAGE_SIZE: usize> ByteAllocator for EarlyAllocator<PAGE_SIZE> {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        if self.forward_current as usize + layout.size() <= self.backward_current as usize {
            let ptr = unsafe{NonNull::new_unchecked(self.forward_current as *mut u8)};
            self.forward_current = self.forward_current as usize + layout.size();
            Ok(ptr)
        }
        else {
            Err(allocator::AllocError::NoMemory)
        }
    }

    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        
    }

    fn total_bytes(&self) -> usize {
        self.backward_current as usize - self.start as usize
    }

    fn used_bytes(&self) -> usize {
        self.forward_current as usize  - self.start as usize 
    }

    fn available_bytes(&self) -> usize {
        self.backward_current as usize  - self.forward_current as usize
    }
}

impl<const PAGE_SIZE: usize> PageAllocator for EarlyAllocator<PAGE_SIZE> {
    const PAGE_SIZE: usize = PAGE_SIZE;
    fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> AllocResult<usize> {
        if self.backward_current as usize - num_pages * PAGE_SIZE >= self.forward_current as usize {
            self.backward_current = self.backward_current as usize - num_pages * PAGE_SIZE;
            Ok(self.backward_current as usize)
        }
        else {
            Err(AllocError::NoMemory)
        }
    }

    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {
        
    }

    fn total_pages(&self) -> usize {
        (self.backward_current as usize - self.forward_current as usize) / PAGE_SIZE
    }

    fn used_pages(&self) -> usize {
        (self.end as usize - self.backward_current as usize) / PAGE_SIZE
    }

    fn available_pages(&self) -> usize {
        self.total_pages() - self.used_pages()
    }
}
