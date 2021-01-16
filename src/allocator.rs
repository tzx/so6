use spin::Mutex;
use bitflags::bitflags;

pub static FRAME_ALLOCATOR: LLAllocator = LLAllocator::new(ListNode::new());

extern "C" {
    // Starting bytes for each section in the linker script
    // We can get the address by & and then cast (as *const _ as usize)
    static _HEAP_START: u8;
    static _PHYSTOP: u8;
}

/// Align address down to PAGE_SIZE
fn align_down(addr: usize) -> usize {
    addr & !(PAGE_SIZE - 1)
}

/// Align address up to PAGE_SIZE
fn align_up(addr: usize) -> usize {
    align_down(addr + PAGE_SIZE - 1)
}

trait Allocator {
    fn alloc(&self) -> *mut u8;
    fn dealloc(&self, ptr: *mut u8);
}


struct ListNode {
    next: Option<&'static ListNode>,
}

pub struct LLAllocator {
    inner: Mutex<ListNode>,
}

impl ListNode {
    const fn new() -> Self {
        ListNode {
            next: None,
        }
    }

    fn start_addr(&self) -> usize {
        self as *const Self as usize
    }
}

impl LLAllocator {
    const fn new(inner: ListNode) -> Self {
        LLAllocator {
            inner: Mutex::new(inner),
        }
    }

    fn lock(&self) -> spin::MutexGuard<ListNode> {
        self.inner.lock()
    }

    pub fn init(&self) {
        // Assumes that the areas are not being used (which it shouldn't)
        unsafe {
            let head_ptr = self.lock();
            assert!(head_ptr.next.is_none());
            drop(head_ptr);

            let start = & _HEAP_START as *const _ as usize;
            let end = & _PHYSTOP as *const _ as usize;

            let page_addr_start = align_up(start);
            for page_addr in (page_addr_start..end).step_by(PAGE_SIZE) {
                let ptr = page_addr as *mut u8;
                self.dealloc(ptr);
            }
        }
    }
}

impl Allocator for LLAllocator {
    fn alloc(&self) -> *mut u8 {
        let mut head_ptr = self.lock();
        if let Some(free) = head_ptr.next {
            head_ptr.next = free.next;
            // XXX: Should we fill with junk?
            free.start_addr() as *mut u8
        } else {
            core::ptr::null_mut()
        }
    }
    
    fn dealloc(&self, ptr: *mut u8) {
        let mut head_ptr = self.lock();
        // XXX: Should we fill with junk?
        let mut node = unsafe {&mut *(ptr as *mut ListNode)};
        node.next = head_ptr.next;
        head_ptr.next = Some(node);
    }
}

// MMU: Table and Entries
bitflags! {
    struct PageTableFlags: usize {
        const NONE = 0;
        const VALID = 1 << 0;
        const READ = 1 << 1;
        const WRITE = 1 << 2;
        const EXECUTE = 1 << 3;
        const USER = 1 << 4;
        const GLOBAL = 1 << 5;
        const ACCESS = 1 << 6;
        const DIRTY = 1 << 7;
    }
}

const NUM_PAGE_ENTRIES: usize = 512;
const PAGE_SIZE: usize = 4096;

struct PageTableEntry {
    entry: usize,
}

impl PageTableEntry {
    const fn new() -> Self {
        PageTableEntry {
            entry: 0,
        }
    }

    fn get_flags(&self) -> PageTableFlags {
        PageTableFlags::from_bits_truncate(self.entry)
    }

    fn set_flags(&mut self, flags: PageTableFlags) {
        self.entry = self.entry | flags.bits();
    }

    fn get_phy_addr(&self) -> usize {
        self.get_ppn() << 12
    }

    fn get_ppn(&self) -> usize {
        self.entry >> 10
    }

    fn set_ppn(&mut self, paddr: usize) {
        assert_eq!(paddr % PAGE_SIZE, 0, "paddr must be page-aligned");
        self.entry = paddr | self.get_flags().bits();
    }
}

struct PageTable {
    entries: [PageTableEntry; NUM_PAGE_ENTRIES],
}

impl PageTable {
    fn new() -> Self {
        // const so it's inline for the array
        const EMPTY: PageTableEntry = PageTableEntry::new();
        PageTable {
            entries: [EMPTY; NUM_PAGE_ENTRIES],
        }
    }
}
