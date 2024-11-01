use std::{
    alloc::Layout,
    ffi::c_void,
    ptr::{copy_nonoverlapping, from_mut, from_ref, null_mut},
};

// https://github.com/manenko/cassander/blob/9c5b0f1fa1deee31759a1e424496411dce802e49/src/allocator.rs

/// Stores the layout used during the allocation of the memory block to the
/// block itself and returns a pointer to the first byte after the layout data.
fn store_layout(layout: Layout, start: *mut u8) -> *mut u8 {
    let size_start = start as *mut usize;
    unsafe { copy_nonoverlapping(from_ref(&layout.size()), size_start, 1) };

    let align_start = unsafe { size_start.add(1) };
    unsafe { copy_nonoverlapping(from_ref(&DEFAULT_ALIGNMENT), align_start, 1) };

    unsafe { start.add(LAYOUT_DATA_SIZE) }
}

/// Restores the layout from the memory block and returns the layout and the
/// real start of the allocated memory block, i.e. the start of the allocation
/// data.
fn restore_layout(ptr: *const u8) -> (*mut u8, Layout) {
    let layout_start = unsafe { ptr.sub(LAYOUT_DATA_SIZE) };
    let size_start = layout_start as *const usize;
    let mut size = 0usize;
    unsafe { copy_nonoverlapping(size_start, from_mut(&mut size), 1) };

    let align_start = unsafe { size_start.add(1) };
    let mut align = 0usize;
    unsafe { copy_nonoverlapping(align_start, from_mut(&mut align), 1) };

    let layout = Layout::from_size_align(size, align).expect("invalid memory layout");

    (layout_start as *mut u8, layout)
}

/// The default alignment for memory allocation requests coming from C.
const DEFAULT_ALIGNMENT: usize = 1;
/// The size of the additional memory we use to store internal allocation
/// data which consists of the block size and alignment.
const LAYOUT_DATA_SIZE: usize = size_of::<usize>() + size_of::<usize>();

unsafe extern "C" fn rust_global_allocator_alloc(size: usize) -> *mut c_void {
    // TODO: handle zero-sized allocations
    let layout = Layout::from_size_align(size + LAYOUT_DATA_SIZE, DEFAULT_ALIGNMENT)
        .expect("invalid memory layout");

    let block_start = std::alloc::alloc(layout);

    if block_start.is_null() {
        return null_mut::<c_void>();
    }

    store_layout(layout, block_start) as *mut c_void
}

unsafe extern "C" fn rust_global_allocator_realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
    if ptr.is_null() {
        return rust_global_allocator_alloc(size);
    }

    // TODO: handle zero-sized allocations

    let new_size = size + LAYOUT_DATA_SIZE;

    let (block_start, layout) = restore_layout(ptr as *const u8);
    let new_layout =
        Layout::from_size_align(new_size, layout.align()).expect("invalid memory layout");

    let new_block_start = std::alloc::realloc(block_start, layout, new_size);

    if new_block_start.is_null() {
        return null_mut::<c_void>();
    }

    store_layout(new_layout, new_block_start) as *mut c_void
}

unsafe extern "C" fn rust_global_allocator_free(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }

    let (block_start, layout) = restore_layout(ptr as *const u8);
    std::alloc::dealloc(block_start, layout);
}

#[no_mangle]
pub unsafe fn alloc(size: usize) -> *mut c_void {
    rust_global_allocator_alloc(size)
}

#[no_mangle]
pub unsafe fn free(ptr: *mut c_void) {
    rust_global_allocator_free(ptr);
}

#[no_mangle]
pub unsafe fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
    rust_global_allocator_realloc(ptr, size)
}
