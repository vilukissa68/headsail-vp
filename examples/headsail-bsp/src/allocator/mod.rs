use good_memory_allocator::SpinLockedAllocator;

const HEAP_START: usize = 0x30000000;
const HEAP_SIZE: usize = 0x10000000;

#[global_allocator]
static ALLOCATOR: SpinLockedAllocator = SpinLockedAllocator::empty();

#[inline]
pub unsafe fn init_heap() {
    ALLOCATOR.init(HEAP_START, HEAP_SIZE);
}
