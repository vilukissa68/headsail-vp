use good_memory_allocator::SpinLockedAllocator;

#[global_allocator]
static ALLOCATOR: SpinLockedAllocator = SpinLockedAllocator::empty();

#[inline]
pub unsafe fn init_heap(heap_start: usize, heap_size: usize) {
    ALLOCATOR.init(heap_start, heap_size);
}
