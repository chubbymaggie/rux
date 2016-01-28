use common::*;
use alloc::boxed::Box;
use paging::table::PageTable;
use paging::table::{PageTableLevel,
                    PageTableLevel4, PageTableLevel3,
                    PageTableLevel2, PageTableLevel1};
use core::mem::size_of;

use self::pool::CapabilityPool;
use self::untyped::UntypedMemoryCapability;

pub mod pool;
pub mod untyped;

pub trait MemoryBlockCapability {
    fn block_start_addr(&self) -> PhysicalAddress {
        self.block_end_addr() - self.block_size() + 1
    }

    fn block_size(&self) -> usize {
        self.block_end_addr() - self.block_start_addr() + 1
    }

    fn block_end_addr(&self) -> PhysicalAddress {
        self.block_start_addr() + self.block_size() - 1
    }
}

pub trait PageBlockCapability {
    fn page_start_addr(&self) -> PhysicalAddress {
        self.page_end_addr() - self.page_size() + 1
    }

    fn page_size(&self) -> usize {
        PAGE_SIZE
    }

    fn page_end_addr(&self) -> PhysicalAddress {
        self.block_start_addr() + self.block_size() - 1
    }

    unsafe fn mapped_p4_table_addr(&self) -> Option<PhysicalAddress>;
    unsafe fn set_mapped_p4_table_addr(&self, Option<PhysicalAddress>);
    unsafe fn set_mapped_ptr(&self, Option<usize>);

    fn active_mapped(&self) -> bool {
        unsafe { controlregs::cr3() as PhysicalAddress == self.mapped_p4_table_addr() }
    }

    fn map_to(&self, VirtualAddress, &PageTableCapability) {
        unimplemented!();
    }

    fn identity_map_to(&self, &PageTableCapability) {
        unimplemented!();
    }

    fn active_map(&self) {
        unimplemented!();
    }

    fn active_identity_map(&self) {
        unimplemented!();
    }
}

pub trait Capability { }
