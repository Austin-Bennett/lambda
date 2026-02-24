use crate::lvm::lenv::LPTR;
use std::{alloc::Layout, sync::Arc, sync::Mutex};

pub const PAGE_BYTE_INDEX_MASK: usize = 0xFFFFFFFFFFFF;

struct LAllocH {
    prev: Option<Arc<Mutex<LAllocH>>>,
    next: Option<Arc<Mutex<LAllocH>>>,
    i: usize,
    len: usize,
}

impl LAllocH {
    pub fn new(prev: Option<Arc<Mutex<LAllocH>>>, i: usize, len: usize) -> Arc<Mutex<Self>> {
        let res = Self {
            prev: prev.clone(),
            next: None,
            i,
            len,
        };

        let res = Arc::new(Mutex::new(res));

        if let Some(n) = prev {
            let mut lock = n.lock().unwrap();
            lock.next = Some(res.clone());
        }

        res
    }
}

struct LPage {
    data: Box<[u8]>,
    allocations: Option<Arc<Mutex<LAllocH>>>,
}

impl LPage {
    pub fn new(size: usize) -> Self {
        Self {
            allocations: None,
            data: vec![0; size].into_boxed_slice(),
        }
    }

    //returns the index on success
    pub fn alloc(&mut self, layout: Layout) -> Option<usize> {
        //find an appropriate chunk of memory that is properly aligned (loc % layout.alignment())
        //== 0
        //store there

        let align = layout.align();
        let size = layout.size();

        if align > 16 {
            panic!(
                "Cannot allocate memory for an alignment: {}, because it is greater than 16",
                layout.align()
            );
        }
        let mut cur = self.allocations.clone();

        if cur.is_none() {
            let alloc_pos = 0;
            let len = self.data.len();

            if len >= size {
                let node = LAllocH::new(None, alloc_pos, len);
                self.allocations = Some(node);
            }

            return Some(alloc_pos);
        } else {
            while let Some(node) = cur {
                let alloc_pos;
                let next;
                {
                    let lock = node.lock().unwrap();
                    next = lock.next.clone();
                    let mut raw_pos = lock.i + lock.len;
                    raw_pos += raw_pos & 1; //make even
                    alloc_pos = align * (raw_pos.div_ceil(align));
                }

                let end = if let Some(node) = &next {
                    node.lock().unwrap().i
                } else {
                    //if this is the last node
                    self.data.len()
                };

                let len = end - alloc_pos;

                if len >= size {
                    //insert node here

                    let node = LAllocH::new(Some(node), alloc_pos, len);
                    if let Some(next) = &next {
                        let mut lock = next.lock().unwrap();

                        lock.prev = Some(node.clone());
                    }
                    {
                        let mut lock = node.lock().unwrap();

                        lock.next = next.clone();
                    }

                    return Some(alloc_pos);
                }

                cur = next.clone();
            }
        }

        None
    }
}

//when accessing via LPTR,
//first 16 bytes -> page index (1 - indexed)
//last 48 bytes -> page byte index
pub struct LHeap {
    pages: Vec<LPage>,
}

impl LHeap {
    pub fn new() -> Self {
        Self { pages: Vec::new() }
    }

    pub fn to_ptr(&mut self, ptr: LPTR) -> *const u8 {}

    pub fn alloc(&mut self, layout: Layout) -> LPTR {
        for (i, p) in self.pages.iter_mut().enumerate() {
            if let Some(ind) = p.alloc(layout) {
                return ((i + 1) << 48) | (ind & PAGE_BYTE_INDEX_MASK);
            }
        }

        //min size is 1000 * 1024 bytes
        let mut page = LPage::new(layout.size().max(1_024_000));
        let i = self.pages.len();
        let Some(ind) = page.alloc(layout) else {
            panic!("Failed to allocate layout: {:?}", layout)
        };

        (i << 48) | (ind & PAGE_BYTE_INDEX_MASK)
    }
}
