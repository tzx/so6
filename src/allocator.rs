use spin::Mutex;
use bitflags::bitflags;

static FRAME_ALLOCATOR: LLAllocator = LLAllocator::new(ListNode::new());

trait Allocator {
    fn alloc(&self) -> *mut u8;
    fn dealloc(&self, ptr: *mut u8);
}


struct ListNode {
    next: Option<&'static ListNode>,
}

struct LLAllocator {
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

    fn init(&self, start: usize, end: usize) {
        todo!();
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
    struct PageTableFlags: u64 {
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
const PAGE_SIZE: u64 = 4096;

#[derive(Clone, Copy)]
struct PageTableEntry {
    entry: u64,
}

impl PageTableEntry {
    fn new() -> Self {
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

    fn get_phy_addr(&self) -> u64 {
        self.get_ppn() << 12
    }

    fn get_ppn(&self) -> u64 {
        self.entry >> 10
    }

    fn set_ppn(&mut self, paddr: u64) {
        assert_eq!(paddr % PAGE_SIZE, 0, "paddr must be page-aligned");
        self.entry = paddr | self.get_flags().bits();
    }
}

struct PageTable {
    entries: [PageTableEntry; NUM_PAGE_ENTRIES],
}

impl PageTable {
    fn new() -> Self {
        let empty = PageTableEntry::new();
        PageTable {
            entries: [empty; NUM_PAGE_ENTRIES],
        }
    }
}
