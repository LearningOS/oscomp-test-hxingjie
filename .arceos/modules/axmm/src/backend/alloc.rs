use axalloc::global_allocator;
use axhal::mem::{phys_to_virt, virt_to_phys};
use axhal::paging::{MappingFlags, PageSize, PageTable};
use memory_addr::{PAGE_SIZE_4K, PageIter4K, PhysAddr, VirtAddr};

use super::Backend;

// 4.28 my code
use memory_addr::PageIter;
use memory_addr::is_aligned;
pub const PAGE_SIZE_2M: usize = 0x200000; // 2*2^20 / 2^12 = 2^9
pub type PageIter2M<A> = PageIter<PAGE_SIZE_2M, A>;
// 4.28 my code

fn alloc_frame(zeroed: bool) -> Option<PhysAddr> {
    let vaddr = VirtAddr::from(global_allocator().alloc_pages(1, PAGE_SIZE_4K).ok()?);
    if zeroed {
        unsafe { core::ptr::write_bytes(vaddr.as_mut_ptr(), 0, PAGE_SIZE_4K) };
    }
    let paddr = virt_to_phys(vaddr);
    Some(paddr)
}

fn dealloc_frame(frame: PhysAddr) {
    let vaddr = phys_to_virt(frame);
    global_allocator().dealloc_pages(vaddr.as_usize(), 1);
}

// 4.28 my code
fn alloc_frame_2m(zeroed: bool) -> Option<PhysAddr> {
    let vaddr = VirtAddr::from(global_allocator().alloc_pages(PAGE_SIZE_2M/PAGE_SIZE_4K, PAGE_SIZE_2M).ok()?);
    if zeroed {
        unsafe { core::ptr::write_bytes(vaddr.as_mut_ptr(), 0, PAGE_SIZE_2M) };
    }
    let paddr = virt_to_phys(vaddr);
    Some(paddr)
}

fn dealloc_frame_2m(frame: PhysAddr) {
    let vaddr = phys_to_virt(frame);
    global_allocator().dealloc_pages(vaddr.as_usize(), PAGE_SIZE_2M/PAGE_SIZE_4K);
}
// 4.28 my code

impl Backend {
    /// Creates a new allocation mapping backend.
    pub const fn new_alloc(populate: bool) -> Self {
        Self::Alloc { populate }
    }

    pub(crate) fn map_alloc(
        start: VirtAddr,
        size: usize,
        flags: MappingFlags,
        pt: &mut PageTable,
        populate: bool,
    ) -> bool {
        debug!(
            "map_alloc: [{:#x}, {:#x}) {:?} (populate={})",
            start,
            start + size,
            flags,
            populate
        );
        if populate {
            // allocate all possible physical frames for populated mapping.
            for addr in PageIter4K::new(start, start + size).unwrap() {
                if let Some(frame) = alloc_frame(true) {
                    if let Ok(tlb) = pt.map(addr, frame, PageSize::Size4K, flags) {
                        tlb.ignore(); // TLB flush on map is unnecessary, as there are no outdated mappings.
                    } else {
                        return false;
                    }
                }
            }
        } else {
            // create mapping entries on demand later in `handle_page_fault_alloc`.
        }
        true
    }

    pub(crate) fn unmap_alloc(
        start: VirtAddr,
        size: usize,
        pt: &mut PageTable,
        _populate: bool,
    ) -> bool {
        debug!("unmap_alloc: [{:#x}, {:#x})", start, start + size);
        for addr in PageIter4K::new(start, start + size).unwrap() {
            if let Ok((frame, page_size, tlb)) = pt.unmap(addr) {
                // Deallocate the physical frame if there is a mapping in the
                // page table.
                if page_size.is_huge() {
                    return false;
                }
                tlb.flush();
                dealloc_frame(frame);
            } else {
                // Deallocation is needn't if the page is not mapped.
            }
        }
        true
    }

    pub(crate) fn map_alloc_2m(
        start: VirtAddr,
        size: usize,
        flags: MappingFlags,
        pt: &mut PageTable,
        populate: bool,
    ) -> bool {
        error!("map_alloc_2m: [{:#x}, {:#x}), size: {}", start, start + size, size);
        // allocate all possible physical frames for populated mapping.
        for addr in PageIter2M::new(start, start + size).unwrap() {
            if let Some(frame) = alloc_frame_2m(true) {
                if let Ok(tlb) = pt.map(addr, frame, PageSize::Size2M, flags) {
                    tlb.ignore(); // TLB flush on map is unnecessary, as there are no outdated mappings.
                } else {
                    return false;
                }
            }
        }
        true
    }

    pub(crate) fn unmap_alloc_2m(
        start: VirtAddr,
        size: usize,
        pt: &mut PageTable,
        _populate: bool,
    ) -> bool {
        error!("unmap_alloc_2m: [{:#x}, {:#x}), size: {}", start, start + size, size);
        for addr in PageIter2M::new(start, start + size).unwrap() {
            if let Ok((frame, page_size, tlb)) = pt.unmap(addr) {
                tlb.flush();
                dealloc_frame_2m(frame);
            } else {
                // Deallocation is needn't if the page is not mapped.
            }
        }
        true
    }

    pub(crate) fn handle_page_fault_alloc(
        vaddr: VirtAddr,
        orig_flags: MappingFlags,
        pt: &mut PageTable,
        populate: bool,
    ) -> bool {
        if populate {
            false // Populated mappings should not trigger page faults.
        } else if let Some(frame) = alloc_frame(true) {
            // Allocate a physical frame lazily and map it to the fault address.
            // `vaddr` does not need to be aligned. It will be automatically
            // aligned during `pt.map` regardless of the page size.
            pt.map(vaddr, frame, PageSize::Size4K, orig_flags)
                .map(|tlb| tlb.flush())
                .is_ok()
        } else {
            false
        }
    }
}
