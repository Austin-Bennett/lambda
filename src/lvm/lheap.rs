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
    pub fn alloc(&mut self, layout: Layout) -> Option<(usize, *const u8)> {
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

            return Some((alloc_pos,
                         unsafe{ self.data.as_ptr().add(alloc_pos) }
            ));
        } else {
            while let Some(node) = cur {
                let alloc_pos;
                let next;
                {
                    let lock = node.lock().unwrap();
                    next = lock.next.clone();
                    let mut raw_pos = lock.i + lock.len;
                    raw_pos += raw_pos & 1; //make even
                    alloc_pos = (raw_pos + align - 1) & !(align - 1);
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

                    return Some((alloc_pos, unsafe{ self.data.as_ptr().add(alloc_pos) }));
                }

                cur = next.clone();
            }
        }

        None
    }

    pub fn free(&mut self, byte: usize) -> bool {
        let mut alloc = self.allocations.clone();

        while let Some(node) = alloc {
            let lock = node.lock().unwrap();

            if lock.i == byte {
                //remove this link
                if let Some(prev) = lock.prev.clone() {
                    let mut prev_lock = prev.lock().unwrap();
                    prev_lock.next = lock.next.clone();
                } else {
                    //must be the first node
                    self.allocations = lock.next.clone()
                }

                if let Some(next) = lock.next.clone() {
                    let mut next_lock = next.lock().unwrap();
                    next_lock.prev = lock.prev.clone();
                }

                return true;
            }

            alloc = lock.next.clone()
        }
        false
    }
}

//when accessing via LPTR,
//first 16 bytes -> page index (1 - indexed)
//last 48 bytes -> page byte index
pub struct LHeap {
    pages: Vec<LPage>,
}

impl LHeap {
    
    pub const MIN_HEAP_ADDR: LPTR = 0x0001FFFFFFFFFFFF;
    
    pub fn new() -> Self {
        Self { pages: Vec::new() }
    }

    fn get_indices(ptr: LPTR) -> (usize, usize) {
        ((ptr as usize >> 48) - 1, ptr as usize & PAGE_BYTE_INDEX_MASK)
    }

    pub fn get_ptr(&mut self, ptr: LPTR) -> Option<*mut u8> {
        let (page_ind, byte_ind) = Self::get_indices(ptr);

        if self.pages.len() <= page_ind {
            return None;
        }

        let page = &mut self.pages[page_ind];

        if page.data.len() <= byte_ind {
            return None;
        }

        Some(&mut page.data[byte_ind] as *mut u8)
    }

    pub fn alloc(&mut self, layout: Layout) -> (LPTR, *const u8) {
        for (i, p) in self.pages.iter_mut().enumerate() {
            if let Some((ind, ptr)) = p.alloc(layout) {
                return ((((i + 1) << 48) | (ind & PAGE_BYTE_INDEX_MASK)) as LPTR, ptr);
            }
        }

        //min size is 1000 * 1024 bytes
        let mut page = LPage::new(layout.size().max(1_024_000));
        let i = self.pages.len();
        let Some((ind, ptr)) = page.alloc(layout) else {
            panic!("Failed to allocate layout: {:?}", layout)
        };

        self.pages.push(page);

        ((((i + 1) << 48) | (ind & PAGE_BYTE_INDEX_MASK)) as LPTR, ptr)
    }

    pub fn free(&mut self, ptr: LPTR) -> bool {
        let (page, byte) = Self::get_indices(ptr);

        if let Some(page) = self.pages.get_mut(page) {


            page.free(byte)
        } else {
            false
        }
    }
}
