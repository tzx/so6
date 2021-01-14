use spin::Mutex;

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
