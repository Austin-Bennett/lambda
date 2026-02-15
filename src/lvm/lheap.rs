use std::ptr;
use std::ptr::slice_from_raw_parts;
use crate::lvm::lenv::LPTR;

pub struct HeapAllocHeader {
    pub size: usize,
    pub next: u64, //index into the page
    pub prev: u64, //always valid (but it may point to a page header if 0
}

pub struct HeapPageHeader {
    first: u64, //index into the page
}

pub struct HeapPage {
    pub mem: Box<[u8]>
}



impl HeapPage {
    pub fn new(size: usize) -> Self {
        //needs to be big enough to store both a
        //heap page header and a heap alloc header
        let size = size + size_of::<HeapPageHeader>() + size_of::<HeapAllocHeader>();
        let mut res = Self{
            mem: vec![0; size].into_boxed_slice()
        };

        let header: *mut HeapPageHeader = res.mem.as_mut_ptr() as *mut HeapPageHeader;

        unsafe {
            (*header).first = 0; //null
        }

        res
    }

    //0 on failure
    pub fn try_alloc(&mut self, size: usize) -> u64 {
        //early exit if this.mem.len() - sizeof(page_header) - sizeof(alloc_header) is < size
        if self.mem.len() - size_of::<HeapPageHeader>() - size_of::<HeapAllocHeader>() < size {
            return 0;
        }
        let header = self.get_header();

        let node = header.first;

        if node == 0 {
            header.first = size_of::<HeapPageHeader>() as u64;

            let node = unsafe{
                self.get_alloc_header(header.first as usize)
            };

            node.size = size;
            node.next = 0;
            node.prev = 0;
            header.first + size_of::<HeapAllocHeader>() as u64
        } else {
            //try to insert between nodes or right after the last node if possible

            let mut start = size_of::<HeapPageHeader>();
            let mut i = node;
            let mut node = unsafe{
                self.get_alloc_header(i as usize)
            };
            while node.next != 0 {
                if start + size_of::<HeapAllocHeader>() + size < i as usize {

                    //insert a node here prev -> header -> node
                    let new_alloc = unsafe{
                        self.get_alloc_header(start)
                    };

                    if node.prev != 0 {
                        let prev = unsafe {
                            self.get_alloc_header(node.prev as usize)
                        };

                        prev.next = start as u64;
                    } else {
                        header.first = start as u64;
                    }
                    new_alloc.prev = node.prev;
                    new_alloc.next = i;
                    new_alloc.size = size;
                    node.prev = start as u64;


                    return (start + size_of::<HeapAllocHeader>()) as u64;
                }

                start = i as usize + size_of::<HeapAllocHeader>() + node.size as usize;
                i = node.next;
                node = unsafe{
                    self.get_alloc_header(i as usize)
                };
            }

            //if we get here, we need to try to append a node at start
            if start + size_of::<HeapAllocHeader>() + size < self.mem.len() {
                //insert a node here  node -> header
                let new_alloc = unsafe{
                    self.get_alloc_header(start)
                };


                node.next = start as u64;
                new_alloc.prev = i;
                new_alloc.next = 0;
                new_alloc.size = size;

                (start + size_of::<HeapAllocHeader>()) as u64
            } else {
                0
            }
        }
    }

    fn get_header(&self) -> &mut HeapPageHeader {
        unsafe {
            &mut *(self.mem.as_ptr() as *mut HeapPageHeader)
        }
    }

    unsafe fn get_alloc_header(&self, i: usize) -> &mut HeapAllocHeader {
        unsafe{
            &mut * (self.mem.as_ptr().add(i) as *mut HeapAllocHeader)
        }
    }
}

pub struct LHeap {
    pub pages: Vec<HeapPage>
}

//an address into the heap starts at LENV_STACK_SIZE + 8, the first 16 bits are the page index,
//and the last 48 are the index into the page (heap index)
//EX: PPPPPPPPPPPPPPPPHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH
impl LHeap {
    pub fn new() -> Self {
        Self{
            pages: Vec::new()
        }
    }

    pub fn alloc(&mut self, size: usize) -> LPTR {
        let mut page_ind = 0u64;
        let mut alloc_ind = 0u64;
        let mut success = false;
        for (i, page) in self.pages.iter_mut().enumerate() {
            let ind = page.try_alloc(size);
            if ind != 0 {
                page_ind = i as u64;
                alloc_ind = ind;
                success = true;
                break;
            }
        }

        if !success {
            //alloc a page with a minimum 1_000_000 bytes
            let mut page = HeapPage::new(size.max(1_000_000));
            page_ind = self.pages.len() as u64;
            alloc_ind = page.try_alloc(size);
            self.pages.push(page);
        }

        page_ind << 48 | alloc_ind
    }

    #[inline(always)]
    pub fn separate_indices(ptr: LPTR) -> (u16, u64) {
        ((ptr >> 48) as u16, ptr & 0xFFFFFFFFFFFF )
    }

    pub fn get_ptr(&self, ptr: LPTR) -> *const u8 {
        let (page, page_index) = Self::separate_indices(ptr);

        if page >= self.pages.len() as u16 {
            return ptr::null();
        }
        let page = &self.pages[page_index as usize];
        if page_index >= page.mem.len() as u64 {
            ptr::null()
        } else {
            & page.mem[page_index as usize]
        }
    }

    pub fn get_ptr_mut(&mut self, ptr: LPTR) -> *mut u8 {
        self.get_ptr(ptr) as *mut u8
    }

    pub unsafe fn free_assume_good(&mut self, ptr: LPTR) {

    }

    pub fn free(&mut self, ptr: LPTR) -> Result<(), String> {
        //go to the page and find the ptr in said page,
        //if it doesn't exist, return an error

    }
    
}