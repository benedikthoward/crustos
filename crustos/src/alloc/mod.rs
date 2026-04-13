pub trait Allocator {
    fn init(heap: &'static mut [u8]);
    unsafe fn allocate(size: usize, align: usize) -> *mut u8;
    unsafe fn deallocate(ptr: *mut u8, size: usize);
    fn free_remaining() -> usize;
}
