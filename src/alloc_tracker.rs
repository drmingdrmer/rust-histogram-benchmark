use std::alloc::GlobalAlloc;
use std::alloc::Layout;
use std::alloc::System;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

static LIVE: AtomicUsize = AtomicUsize::new(0);

pub struct TrackingAllocator;

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc(layout) };
        if !ptr.is_null() {
            LIVE.fetch_add(layout.size(), Ordering::Relaxed);
        }
        ptr
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc_zeroed(layout) };
        if !ptr.is_null() {
            LIVE.fetch_add(layout.size(), Ordering::Relaxed);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        LIVE.fetch_sub(layout.size(), Ordering::Relaxed);
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let new_ptr = unsafe { System.realloc(ptr, layout, new_size) };
        if !new_ptr.is_null() {
            let old_size = layout.size();
            match new_size.cmp(&old_size) {
                std::cmp::Ordering::Greater => {
                    LIVE.fetch_add(new_size - old_size, Ordering::Relaxed);
                }
                std::cmp::Ordering::Less => {
                    LIVE.fetch_sub(old_size - new_size, Ordering::Relaxed);
                }
                std::cmp::Ordering::Equal => {}
            }
        }
        new_ptr
    }
}

/// Returns current live heap bytes tracked by the benchmark process.
pub fn live_bytes() -> usize {
    LIVE.load(Ordering::Relaxed)
}
