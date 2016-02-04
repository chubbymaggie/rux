pub mod table;
pub mod entry;

use common::*;
use core::marker::PhantomData;

use super::PageBlockCapability;

pub trait PageTableStatus { }
pub enum ActivePageTableStatus { }
pub enum InactivePageTableStatus { }

impl PageTableStatus for ActivePageTableStatus { }
impl PageTableStatus for InactivePageTableStatus { }

pub struct PageTableCapability<L: PageTableStatus> {
    block_start_addr: PhysicalAddress,
    block_size: usize,
    table_start_addr: PhysicalAddress,
    active: PhantomData<L>,
}

pub type ActivePageTableCapability = PageTableCapability<ActivePageTableStatus>;
pub type InactivePageTableCapability = PageTableCapability<InactivePageTableStatus>;

impl<L> PageTableCapability<L> where L: PageTableStatus {
    pub fn map<T, U>(&self, page: &T) -> VirtualAddress<U> where T: PageBlockCapability<U> {
        unimplemented!();
    }
}

impl ActivePageTableCapability {
    pub fn switch(new: InactivePageTableCapability, current: ActivePageTableCapability)
                  -> (ActivePageTableCapability, InactivePageTableCapability) {
        unimplemented!();
    }

    pub fn borrow<'r, U>(&'r self, virt: &VirtualAddress<U>) -> &'r U {
        assert!(virt.table_start_addr == self.table_start_addr);
        unsafe { &*(virt.addr as *mut _) }
    }

    pub fn borrow_mut<'r, U>(&'r self, virt: &mut VirtualAddress<U>) -> &'r U {
        assert!(virt.table_start_addr == self.table_start_addr);
        unsafe { &mut *(virt.addr as *mut _) }
    }
}

// WARNING: Currently it is unsafe to map one page block in one page table
// multiple times. It is indeed safe if that is not violated.
// TODO: Implement this.

pub struct VirtualAddress<T> {
    table_start_addr: PhysicalAddress,
    addr: usize,
    _marker: PhantomData<T>,
}

impl<T> Drop for VirtualAddress<T> {
    fn drop(&mut self) {
        unimplemented!()
    }
}
